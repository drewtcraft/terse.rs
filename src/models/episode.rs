use crate::types::*;
use super::asset::Asset;
use super::act::Act;

// we don't need to know anything about the episode
// this is basically just a container for the style
// interpolation functionality
pub struct Episode<'a> {
	acts: Vec<&'a Act<'a>>,
	assets: Vec<&'a Asset>,
}

// require shit where we need it!
use crate::cereal;
use serde_json;
impl<'a> Episode<'a> {
	fn from_json (json: &str) -> Episode<'a> {
		let parsed: cereal::Episode = serde_json::from_str(json)
			.unwrap();

		let assets = parsed.assets.map(|asset| {
			Asset {
				origin: Point(asset.x, asset.y),
				size: Rect.new(asset.width, asset.height),
				z: Z(asset.z),
				angle: Angle(asset.angle),
				image_index: ImageIndex(asset.image_index),
			}
		});

		let asset_iter = assets.iter();

		let acts = parsed.acts.map(|act| {
			let my_assets = assets.filter(|asset| {
				
			});

			let t = act.r#type;
			let act_data = match t {
				"move" => ActData::Move(Point()),
				_ => panic!("unknown act type {}", t),
			};

			Act {
				assets: my_assets,
				range: TimeRange(act.start, act.end),
				act_data,
			}
		});

		Episode {
			assets,
			acts,
		}
	}
}
