use crate::types::*;

pub struct Asset {
	pub asset_key: Id,
	pub origin: Point,
	pub size: Rect,
	pub z: Z,
	pub angle: Angle,
	pub image_index: ImageIndex,
}

impl Asset {
	pub fn get_base_style (&self) -> Style {
		Style {
			origin: self.origin.clone(),
			size: self.size.clone(),
			z: self.z,
			angle: self.angle,
			image_index: self.image_index,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
