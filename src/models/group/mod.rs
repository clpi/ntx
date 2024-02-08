
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Group {
	id: u32,
	name: String,
}
