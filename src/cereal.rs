use serde::{Deserialize};
use serde_json::{Value};

#[derive(Deserialize)]
pub struct Asset {
	pub asset_key: String,
	pub x: String,
	pub y: String,
	pub width: String,
	pub height: String,
	pub z: i16,
	pub angle: f64,
}

#[derive(Deserialize)]
pub struct Act {
	pub act_key: String,
	pub asset_keys: Vec<String>,
	pub start: f64,
	pub end: f64,
	pub r#type: String,
	pub data: Value, // unstructured data, varies by type
}

#[derive(Deserialize)]
pub struct Episode {
	pub acts: Vec<Act>,
	pub assets: Vec<Asset>,
}
