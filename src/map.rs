use std::collections::HashMap;

use crate::model;
use crate::gameplay_modes;
//this is the current map data loaded in memory
pub struct Map{
	modes:gameplay_modes::Modes,
	models:model::Models,
	//RenderPattern
	textures:HashMap<u32,Vec<u8>>,
}
