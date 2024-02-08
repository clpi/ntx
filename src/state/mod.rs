use serde::{Serialize, Deserialize};
use crate::models::group::Group;
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct AppState {
	pub app: String,
	pub users: Vec<User>,
	pub groups: Vec<Group>,
}
impl Default for AppState {
	fn default() -> Self {
		AppState {
			app: "ntx".into(),
			users: vec![],
			groups: vec![],
		}
	}
}