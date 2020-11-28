use super::asset::Asset;

pub enum ActType {
	Move,
}

pub struct TimeRange (f32, f32);

// uh oh lifetimes
pub struct Act<'a> {
	act_type: ActType,
	range: TimeRange,
	assets: Vec<&'a Asset>,
}

impl Act<'_> {
	fn new<'a>(
		act_type: ActType, 
		range: TimeRange, 
		assets: Vec<&'a Asset>) -> Act {
		Act {
			act_type,
			range,
			assets,
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
