use std::ops::Deref;
use ntex::web;
use ntex::web::{get, HttpResponse, Responder};
use ntex::web::types::State;
use crate::state::AppState;

#[get("/echo")]
pub async fn echo(s: State<AppState>) -> impl Responder{
	HttpResponse::Ok().body("Hi world")
}
#[get("/about")]
pub async fn about(s: State<AppState>) -> String {
	format!("Hello {}!", s.app)
}

#[get("/")]
pub async fn index(state: web::types::State<AppState>) -> String {
	let _users = &state.to_owned().deref().users;
	return format!("Hello {}", &state.to_owned().deref().app)
}

#[get("/info")]
pub async fn info(s: State<AppState>) -> String {
	format!("Hello {}!", s.app)
}
