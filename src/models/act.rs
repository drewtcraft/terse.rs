use crate::types::*;
use super::asset::Asset;

pub struct Act<'a> {
	act_data: ActData,
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
