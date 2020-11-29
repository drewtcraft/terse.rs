#[derive(Clone, Debug)]
pub struct Rect {
	width: f64,
	height: f64,
}
impl Rect {
	fn new(width: f64, height: f64) -> Rect {
		Rect { width, height }
	}
}

