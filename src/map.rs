use std::collections::HashMap;

use crate::model;
use crate::gameplay_modes;
use crate::gameplay_attributes;
//this is the current map data loaded in memory
pub struct Map{
	pub modes:gameplay_modes::Modes,
	pub indexed_models:HashMap<model::IndexedModelId,model::IndexedModel>,
	pub models:HashMap<model::ModelId,model::Model>,
	pub attributes:Vec<gameplay_attributes::CollisionAttributes>,
	//RenderPattern
	pub textures:Vec<Vec<u8>>,
}
