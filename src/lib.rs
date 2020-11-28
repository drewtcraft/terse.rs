#![allow(dead_code)] // damn this is useful

mod types;
mod models;
mod serialize;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
