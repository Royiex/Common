use std::collections::{HashSet,HashMap};
use crate::model::ModelId;
use crate::gameplay_style;
use crate::updatable::Updatable;

#[derive(Clone)]
pub struct StageElement{
	stage:StageId,//which stage spawn to send to
	force:bool,//allow setting to lower spawn id i.e. 7->3
	behaviour:StageElementBehaviour
}
impl StageElement{
	pub fn new(stage_id:u32,force:bool,behaviour:StageElementBehaviour)->Self{
		Self{
			stage:StageId(stage_id),
			force,
			behaviour,
		}
	}
}

#[derive(Clone,Hash,Eq,PartialEq)]
pub enum StageElementBehaviour{
	SpawnAt,//must be standing on top to get effect. except cancollide false
	Trigger,
	Teleport,
	Platform,
	//Check(point) acts like a trigger if you haven't hit all the checkpoints on previous stages yet.
	//Note that all stage elements act like this, this is just the isolated behaviour.
	Check,
	Checkpoint,//this is a combined behaviour for Ordered & Unordered in case a model is used multiple times or for both.
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct CheckpointId(usize);
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct StageId(u32);
pub struct Stage{
	spawn:ModelId,
	//open world support lol
	ordered_checkpoints_count:u32,
	//other behaviour models of this stage can have
	ordered_checkpoints:HashMap<CheckpointId,ModelId>,
	unordered_checkpoints:HashSet<ModelId>,
}
#[derive(Default)]
pub struct StageUpdate{
	//other behaviour models of this stage can have
	ordered_checkpoints:HashMap<CheckpointId,ModelId>,
	unordered_checkpoints:HashSet<ModelId>,
}
impl Updatable<&StageUpdate> for Stage{
	fn insert(&mut self,update:&StageUpdate){
		for (&checkpoint,&model) in &update.ordered_checkpoints{
			self.ordered_checkpoints.insert(checkpoint,model);
		}
		for &checkpoint in &update.unordered_checkpoints{
			self.unordered_checkpoints.insert(checkpoint);
		}
	}
	fn remove(&mut self,update:&StageUpdate){
		for (checkpoint,_) in &update.ordered_checkpoints{
			self.ordered_checkpoints.remove(checkpoint);
		}
		for model in &update.unordered_checkpoints{
			self.unordered_checkpoints.remove(model);
		}
	}
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub enum Zone{
	Start,
	Finish,
	Anticheat,
}
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct ModeId(u32);
impl ModeId{
	pub const MAIN:Self=Self(0);
	pub const BONUS:Self=Self(1);
	pub const fn mode(mode_id:u32)->Self{
		Self(mode_id)
	}
}
pub struct Mode{
	style:gameplay_style::StyleModifiers,
	start:ModelId,//when you press reset you go here
	zones:HashMap<ModelId,Zone>,
	stages:Vec<Stage>,//when you load the map you go to stages[0].spawn
	//mutually exlusive stage element behaviour
	elements:HashMap<ModelId,StageElement>,
	jump_limit:HashMap<ModelId,u32>,
}
impl Mode{
	pub fn get_spawn_model_id(&self,stage:StageId)->Option<ModelId>{
		self.stages.get(stage.0 as usize).map(|s|s.spawn)
	}
	//TODO: put this in the SNF
	pub fn denormalize_data(&mut self){
		//expand and index normalized data
		self.zones.insert(self.start,Zone::Start);
		for (stage_id,stage) in self.stages.iter().enumerate(){
			self.elements.insert(stage.spawn,StageElement{
				stage:StageId(stage_id as u32),
				force:false,
				behaviour:StageElementBehaviour::SpawnAt,
			});
			for (_,&model) in &stage.ordered_checkpoints{
				self.elements.insert(model,StageElement{
					stage:StageId(stage_id as u32),
					force:false,
					behaviour:StageElementBehaviour::Checkpoint,
				});
			}
			for &model in &stage.unordered_checkpoints{
				self.elements.insert(model,StageElement{
					stage:StageId(stage_id as u32),
					force:false,
					behaviour:StageElementBehaviour::Checkpoint,
				});
			}
		}
	}
}
//this would be nice as a macro
#[derive(Default)]
pub struct ModeUpdate{
	zones:HashMap<ModelId,Zone>,
	stages:HashMap<StageId,StageUpdate>,
	//mutually exlusive stage element behaviour
	elements:HashMap<ModelId,StageElement>,
	jump_limit:HashMap<ModelId,u32>,
}
impl Updatable<&ModeUpdate> for Mode{
	fn insert(&mut self,update:&ModeUpdate){
		for (&model,&zone) in &update.zones{
			self.zones.insert(model,zone);
		}
		for (stage,stage_update) in &update.stages{
			if let Some(stage)=self.stages.get_mut(stage.0 as usize){
				stage.insert(stage_update);
			}
		}
		for (&model,stage_element) in &update.elements{
			self.elements.insert(model,stage_element.clone());
		}
		for (&model,&limit) in &update.jump_limit{
			self.jump_limit.insert(model,limit);
		}
	}
	fn remove(&mut self,update:&ModeUpdate){
		for (model,_) in &update.zones{
			self.zones.remove(model);
		}
		for (stage,stage_update) in &update.stages{
			if let Some(stage)=self.stages.get_mut(stage.0 as usize){
				stage.remove(stage_update);
			}
		}
		for (model,_) in &update.elements{
			self.elements.remove(model);
		}
		for (model,_) in &update.jump_limit{
			self.jump_limit.remove(model);
		}
	}
}
impl ModeUpdate{
	pub fn zone(model_id:ModelId,zone:Zone)->Self{
		let mut mu=Self::default();
		mu.zones.insert(model_id,zone);
		mu
	}
	pub fn stage(stage_id:StageId,stage_update:StageUpdate)->Self{
		let mut mu=Self::default();
		mu.stages.insert(stage_id,stage_update);
		mu
	}
	pub fn element(model_id:ModelId,element:StageElement)->Self{
		let mut mu=Self::default();
		mu.elements.insert(model_id,element);
		mu
	}
	pub fn jump_limit(model_id:ModelId,jump_limit:u32)->Self{
		let mut mu=Self::default();
		mu.jump_limit.insert(model_id,jump_limit);
		mu
	}
}

#[derive(Default)]
pub struct Modes{
	modes:Vec<Mode>,
}
impl Modes{
	pub fn new(modes:Vec<Mode>)->Self{
		Self{
			modes,
		}
	}
	pub fn get_mode(&self,mode:ModeId)->Option<&Mode>{
		self.modes.get(mode.0 as usize)
	}
}
pub struct ModesUpdate{
	modes:HashMap<ModeId,ModeUpdate>,
}
impl Updatable<&ModesUpdate> for Modes{
	fn insert(&mut self,update:&ModesUpdate){
		for (mode,mode_update) in &update.modes{
			if let Some(mode)=self.modes.get_mut(mode.0 as usize){
				mode.insert(mode_update);
			}
		}
	}
	fn remove(&mut self,update:&ModesUpdate){
		for (mode,mode_update) in &update.modes{
			if let Some(mode)=self.modes.get_mut(mode.0 as usize){
				mode.remove(mode_update);
			}
		}
	}
}