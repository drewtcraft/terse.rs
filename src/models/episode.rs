use crate::types::*;
use super::asset::Asset;
use super::act::Act;

// we don't need to know anything about the episode
// this is basically just a container for the style
// interpolation functionality
pub struct Episode {
	acts: Vec<Act>>,
	assets: Vec<Asset>,
}

// require shit where we need it!
use crate::cereal;
use serde_json;
impl Episode {
	fn from_json (json: &str) -> Episode {
		let parsed: cereal::Episode = serde_json::from_str(json)
			.expect("failed to parse JSON");

		let assets: Vec<Asset> = Vec::new();
		for asset in parsed.assets.iter() {
			assets.push(Asset {
				asset_key: asset.asset_key,
				origin: Point(asset.x, asset.y),
				size: Rect.new(asset.width, asset.height),
				z: asset.z,
				angle: asset.angle,
				image_index: 0,
			});
		}

		let assets_iter = assets.iter();

		let acts: Vec<Act> = Vec::new();
		for act in parsed.acts.iter() {
			let my_assets: Vec<&Asset> = Vec::new();
			for asset in assets_iter {
				if act.asset_keys.contains(&asset.asset_key) {
					my_assets.push(asset);
				}
			}

			let act_data = match act.r#type.as_str() {
				"move" => {
					ActData::Move(
						Point(
							act.data["x"].as_f64().unwrap(), 
							act.data["y"].as_f64().unwrap(),
						)
					)
				}
				_ => panic!("unknown act type {}", act.r#type),
			};

			acts.push(Act {
				assets: my_assets,
				range: TimeRange(act.start, act.end),
				act_data,
			});
		}

		Episode {
			assets,
			acts,
		}
	}
}
