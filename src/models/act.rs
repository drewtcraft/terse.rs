use crate::types::*;
use super::asset::Asset;

pub enum ActType {
	Move,
}

pub struct Act<'a> {
	act_type: ActType,
	range: TimeRange,
	assets: Vec<&'a Asset>,
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
