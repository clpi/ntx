use argonautica::utils;
use ntex::{Service, ServiceCtx, ServiceFactory, web};
use ntex::http::{HttpService, Request, Response, StatusCode};
use ntex::http::header::{CONTENT_TYPE, SERVER};
use ntex::io::IoStatusUpdate::KeepAlive;
use ntex::web::{App as NApp, HttpResponse, HttpServer, middleware};
use ntex::web::middleware::{Compress, Logger, DefaultHeaders};
use ntex_bytes::PoolId;
use ntex_identity::{CookieIdentityPolicy, IdentityService};
use ntex_util::time::Seconds;
use crate::db;
use crate::route::{about, echo, index, info};
use crate::util::HDR_SERVER;

struct App(crate::db::DbConnection);
impl Service<Request> for App {
	type Response = Response;
	type Error = ntex::web::Error;

	async fn call(&self, req: Request, _: ServiceCtx<'_, Self>) -> Result<Response, ntex::web::Error> {
		match req.path() {
			"/db" => {
				let body = self.0.get_world().await;
				let mut res = HttpResponse::with_body(StatusCode::OK, body.into());
				res.headers_mut().insert(SERVER, super::util::HDR_SERVER);
				res.headers_mut()
					.insert(CONTENT_TYPE, super::util::HDR_JSON_CONTENT_TYPE);
				Ok(res)
			}
			"/fortunes" => {
				let body = self.0.tell_fortune().await;
				let mut res = HttpResponse::with_body(StatusCode::OK, body.into());
				res.headers_mut().insert(SERVER, super::util::HDR_SERVER);
				res.headers_mut()
					.insert(CONTENT_TYPE, super::util::HDR_HTML_CONTENT_TYPE);
				Ok(res)
			}
			"/query" => {
				let worlds = self
					.0
					.get_worlds(super::util::get_query_param(req.uri().query()))
					.await;
				let mut res = HttpResponse::with_body(StatusCode::OK, worlds.into());
				res.headers_mut().insert(SERVER, super::util::HDR_SERVER);
				res.headers_mut()
					.insert(CONTENT_TYPE, super::util::HDR_JSON_CONTENT_TYPE);
				Ok(res)
			}
			"/update" => {
				let worlds = self
					.0
					.update(super::util::get_query_param(req.uri().query()))
					.await;
				let mut res = HttpResponse::with_body(StatusCode::OK, worlds.into());
				res.headers_mut().insert(SERVER, super::util::HDR_SERVER);
				res.headers_mut()
					.insert(CONTENT_TYPE, super::util::HDR_JSON_CONTENT_TYPE);
				Ok(res)
			}
			_ => Ok(Response::new(StatusCode::NOT_FOUND)),
		}
	}
}

struct AppFactory;

impl ServiceFactory<Request> for AppFactory {
	type Response = Response;
	type Error = ntex::web::Error;
	type Service = App;
	type InitError = ();

	async fn create(&self, _: ()) -> Result<Self::Service, Self::InitError> {
		const DB_URL: &str =
			"postgres://benchmarkdbuser:benchmarkdbpass@tfb-database/hello_world";

		Ok(App(db::DbConnection::connect(DB_URL).await))
	}
}





/*
pub async fn run(addr: &'static str, port: u16) -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.wrap(Logger::default())
			.wrap(DefaultHeaders::default())
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&[0; 32])
					.name("auth-example")
					.secure(false)

			))
			.state(super::state::AppState::default())
			.service(index)
			.service(about)
			.service(info)
			.service(echo)
	})
		.client_timeout(Seconds(0))
		.headers_read_rate(Seconds::ZERO, Seconds::ZERO, 0)
		.payload_read_rate(Seconds::ZERO, Seconds::ZERO, 0)
		.backlog(1024)
		.workers(num_cpus::get())
		.bind((addr, port))?
		.run().await
}
*/

pub async fn run() -> std::io::Result<()> {
	println!("Starting http server: 127.0.0.1:8080");

	ntex::server::build()
		.backlog(1024)
		.bind("techempower", "0.0.0.0:8080", |cfg| {
			cfg.memory_pool(PoolId::P1);
			PoolId::P1.set_read_params(65535, 2048);
			PoolId::P1.set_write_params(65535, 2048);

			HttpService::build()
				.keep_alive(KeepAlive::Os)
				.client_timeout(Seconds(0))
				.headers_read_rate(Seconds::ZERO, Seconds::ZERO, 0)
				.payload_read_rate(Seconds::ZERO, Seconds::ZERO, 0)
				.h1(AppFactory)
		})?
		.workers(num_cpus::get())
		.run()
		.await
}