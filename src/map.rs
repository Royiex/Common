use crate::model;
use crate::gameplay_modes;
use crate::gameplay_attributes;
//this is a temporary struct to try to get the code running again
//TODO: use snf::map::Region to update the data in physics and graphics instead of this
pub struct CompleteMap{
	pub modes:gameplay_modes::Modes,
	pub attributes:Vec<gameplay_attributes::CollisionAttributes>,
	pub meshes:Vec<model::Mesh>,
	pub models:Vec<model::Model>,
	//RenderPattern
	pub textures:Vec<Vec<u8>>,
	pub render_configs:Vec<model::RenderConfig>,
}
