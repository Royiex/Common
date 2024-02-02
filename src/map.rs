use crate::model;
use crate::gameplay_modes;
use crate::gameplay_attributes;
//this is the current map data loaded in memory
pub struct Map{
	pub modes:gameplay_modes::Modes,
	pub models:model::Models,
	pub attributes:Vec<gameplay_attributes::CollisionAttributes>,
	//RenderPattern
	pub textures:Vec<Vec<u8>>,
}
