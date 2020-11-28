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
