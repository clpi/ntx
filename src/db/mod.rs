use std::cell::RefCell;
use argonautica::utils;
use ntex::util::{Bytes, BytesVec, Buf};
use chrono::expect;
use nanorand::{Rng, WyRand};
use ntex_bytes::{BufMut, BytesMut};
use smallvec::SmallVec;
use tokio_postgres::{Client, Statement};
use tokio_postgres::types::ToSql;
use yarte::{Serialize, ywrite_html};
use crate::util;
use crate::util::reserve;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct World {
	pub id: i32,
	pub rand_num: i32,
}
#[derive(Serialize, Debug)]
pub struct Fortune<'a> {
	pub id: i32,
	pub msg: &'a str,
}
#[derive(Debug)]
pub struct DbConnection {
	client: Client,
	fortune: Statement,
	world: Statement,
	rng: WyRand,
	updates: Vec<Statement>,
	buf: RefCell<BytesMut>
}

impl DbConnection {
	pub async fn conn(db_url: &str) -> DbConnection {
		let (client, conn) = tokio_postgres::connect(&db_url)
			.await.expect("connection error");
		ntex::rt::spawn(async move {
			let _ = conn.await;
		});
		let fort = client.prepare("SELECT * FROM fortune")
			.await.unwrap();
		let mut upd = Vec::new();
		for num in 1..=500u16 {
			let mut pl: u16 = 1;
			let mut q = String::new();
			q.push_str("UPDATE world SET rand_num = CASE id ");
			for _ in 1..=num {
				let _ = write!(&mut q, "when ${} then ${} ", pl, pl + 1);
				pl += 2;
			}
			q.push_str("ELSE rand_num END WHERE id IN (");
			for _ in 1..=num {
				let _ = write!(&mut q, "${},", pl);
				pl += 1;
			}
			q.pop();
			q.push(')');
			upd.push(client.prepare(&q).await.unwrap());
		}
		let world = client
			.prepare("SELECT id, rand_num FROM world WHERE id=$1")
			.await.unwrap();
		Self {
			client,
			fortune,
			world,
			updates,
			rng: WyRand::new(),
			buf: RefCell::new(BytesMut::with_capacity(65535)),
		}
	}
	pub async fn get_world(&self) -> Bytes {
		let random_id = (self.rng.clone().generate::<u32>() % 10_000 + 1) as i32;

		let row = self.client.query_one(&self.world, &[&random_id]).await.unwrap();

		let mut body = self.buf.borrow_mut();
		util::reserve(&mut body);
		World {
			id: row.get(0),
			rand_num: row.get(1),
		}
			.to_bytes_mut(&mut body);
		body.split().freeze()
	}

	pub async fn get_worlds(&self, num: usize) -> Bytes {
		let mut rng = self.rng.clone();
		let mut queries = SmallVec::<[_; 32]>::new();
		(0..num).for_each(|_| {
			let w_id = (rng.generate::<u32>() % 10_000 + 1) as i32;
			queries.push(self.client.query_one(&self.world, &[&w_id]));
		});

		let mut worlds = SmallVec::<[_; 32]>::new();
		for fut in queries {
			let row = fut.await.unwrap();
			worlds.push(World {
				id: row.get(0),
				rand_num: row.get(1),
			})
		}

		let mut body = self.buf.borrow_mut();
		super::util::reserve(&mut body);
		body.put_u8(b'[');
		worlds.iter().for_each(|w| {
			w.to_bytes_mut(&mut body);
			body.put_u8(b',');
		});
		let idx = body.len() - 1;
		body[idx] = b']';
		body.split().freeze()
	}

	pub async fn update(&self, num: usize) -> Bytes {
		let mut rng = nanorand::tls_rng();
		let mut queries = SmallVec::<[_; 32]>::new();
		(0..num).for_each(|_| {
			let w_id = (rng.generate::<u32>() % 10_000 + 1) as i32;
			queries.push(self.client.query_one(&self.world, &[&w_id]));
		});

		let mut worlds = SmallVec::<[_; 32]>::new();
		for fut in queries.into_iter() {
			let row = fut.await.unwrap();
			worlds.push(World {
				id: row.get(0),
				rand_num: (rng.generate::<u32>() % 10_000 + 1) as i32,
			});
		}

		let mut params: Vec<&dyn ToSql> = Vec::with_capacity(num * 3);
		for w in &worlds {
			params.push(&w.id);
			params.push(&w.rand_num);
		}
		for w in &worlds {
			params.push(&w.id);
		}
		let _ = self.client.query(&self.updates[num - 1], &params).await;

		let mut body = self.buf.borrow_mut();
		super::util::reserve(&mut body);
		body.put_u8(b'[');
		worlds.iter().for_each(|w| {
			w.to_bytes_mut(&mut body);
			body.put_u8(b',');
		});
		let idx = body.len() - 1;
		body[idx] = b']';
		body.split().freeze()
	}

	pub async fn tell_fortune(&self) -> Bytes {
		let rows = self.client.query_raw(&self.fortune, &[]).await.unwrap();

		let mut fortunes = Vec::with_capacity(rows.len() + 1);
		fortunes.push(Fortune {
			id: 0,
			msg: "Additional fortune added at request time.",
		});
		fortunes.extend(rows.iter().map(|row| Fortune {
			id: row.get(0),
			msg: row.get(1),
		}));
		fortunes.sort_by(|it, next| it.msg.cmp(next.msg));

		let mut body = std::mem::replace(&mut *self.buf.borrow_mut(), BytesMut::new());
		super::util::reserve(&mut body);
		ywrite_html!(body, "{{> fortune }}");
		let result = body.split().freeze();
		let _ = std::mem::replace(&mut *self.buf.borrow_mut(), body);
		result
	}
}