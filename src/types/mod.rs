mod rect;
mod algebraic;

pub use self::rect::Rect;
pub use self::algebraic::Algebraic;
use arraystring::{ArrayString, typenum::U10};

#[derive(Clone, Debug)]
pub struct Point(pub f64, pub f64);

#[derive(Clone, Debug)]
pub struct TimeRange(f64, f64); // start -> end

// some useful aliases
pub type Angle = f64;
pub type Z = i16;
pub type ImageIndex = u8;

// basically a nanoid
// need to be smart about these when the episodes get exported
pub type Id = ArrayString<U10>;

// this is the $$, gonna be the main export
#[derive(Clone, Debug)]
pub struct Style {
	pub origin: Point,
	pub size: Rect,
	pub z: Z,
	pub angle: Angle,
	pub image_index: ImageIndex, // track which image we are using
}
