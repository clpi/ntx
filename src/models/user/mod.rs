use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct User {
	id: u32,
	name: String,
}
