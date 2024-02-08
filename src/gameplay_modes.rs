use std::collections::{HashSet,HashMap};
use crate::model::ModelId;
use crate::gameplay_style;
use crate::updatable::Updatable;

#[derive(Clone)]
pub struct StageElement{
	pub stage_id:StageId,//which stage spawn to send to
	pub force:bool,//allow setting to lower spawn id i.e. 7->3
	pub behaviour:StageElementBehaviour
}
impl StageElement{
	pub fn new(stage_id:u32,force:bool,behaviour:StageElementBehaviour)->Self{
		Self{
			stage_id:StageId(stage_id),
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

#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct CheckpointId(u32);
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq,Ord,PartialOrd)]
pub struct StageId(u32);
impl StageId{
	pub const FIRST:Self=Self(0);
}
#[derive(Clone)]
pub struct Stage{
	spawn:ModelId,
	//open world support lol
	ordered_checkpoints_count:u32,
	//other behaviour models of this stage can have
	ordered_checkpoints:HashMap<CheckpointId,ModelId>,
	unordered_checkpoints:HashSet<ModelId>,
}
impl Stage{
	pub fn new(spawn:ModelId)->Self{
		Self{
			spawn,
			ordered_checkpoints_count:0,
			ordered_checkpoints:HashMap::new(),
			unordered_checkpoints:HashSet::new(),
		}
	}
	#[inline]
	pub const fn spawn(&self)->ModelId{
		self.spawn
	}
}
#[derive(Default)]
pub struct StageUpdate{
	//other behaviour models of this stage can have
	ordered_checkpoints:HashMap<CheckpointId,ModelId>,
	unordered_checkpoints:HashSet<ModelId>,
}
impl Updatable<StageUpdate> for Stage{
	fn update(&mut self,update:StageUpdate){
		self.ordered_checkpoints.extend(update.ordered_checkpoints);
		self.unordered_checkpoints.extend(update.unordered_checkpoints);
	}
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub enum Zone{
	Start,
	Finish,
	Anticheat,
}
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq,Ord,PartialOrd)]
pub struct ModeId(u32);
impl ModeId{
	pub const MAIN:Self=Self(0);
	pub const BONUS:Self=Self(1);
}
#[derive(Clone)]
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
	pub fn new(style:gameplay_style::StyleModifiers,start:ModelId)->Self{
		Self{
			style,
			start,
			zones:HashMap::new(),
			stages:Vec::new(),
			elements:HashMap::new(),
			jump_limit:HashMap::new(),
		}
	}
	pub fn push_stage(&mut self,stage:Stage){
		self.stages.push(stage)
	}
	pub fn get_stage_mut(&mut self,stage:StageId)->Option<&mut Stage>{
		self.stages.get_mut(stage.0 as usize)
	}
	pub fn get_spawn_model_id(&self,stage:StageId)->Option<ModelId>{
		self.stages.get(stage.0 as usize).map(|s|s.spawn)
	}
	pub fn get_zone(&self,model_id:ModelId)->Option<&Zone>{
		self.zones.get(&model_id)
	}
	pub fn get_stage(&self,stage_id:StageId)->Option<&Stage>{
		self.stages.get(stage_id.0 as usize)
	}
	pub fn get_element(&self,model_id:ModelId)->Option<&StageElement>{
		self.elements.get(&model_id)
	}
	pub fn get_jump_limit(&self,model_id:ModelId)->Option<u32>{
		self.jump_limit.get(&model_id).copied()
	}
	//TODO: put this in the SNF
	pub fn denormalize_data(&mut self){
		//expand and index normalized data
		self.zones.insert(self.start,Zone::Start);
		for (stage_id,stage) in self.stages.iter().enumerate(){
			self.elements.insert(stage.spawn,StageElement{
				stage_id:StageId(stage_id as u32),
				force:false,
				behaviour:StageElementBehaviour::SpawnAt,
			});
			for (_,&model) in &stage.ordered_checkpoints{
				self.elements.insert(model,StageElement{
					stage_id:StageId(stage_id as u32),
					force:false,
					behaviour:StageElementBehaviour::Checkpoint,
				});
			}
			for &model in &stage.unordered_checkpoints{
				self.elements.insert(model,StageElement{
					stage_id:StageId(stage_id as u32),
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
impl Updatable<ModeUpdate> for Mode{
	fn update(&mut self,update:ModeUpdate){
		self.zones.extend(update.zones);
		for (stage,stage_update) in update.stages{
			if let Some(stage)=self.stages.get_mut(stage.0 as usize){
				stage.update(stage_update);
			}
		}
		self.elements.extend(update.elements);
		self.jump_limit.extend(update.jump_limit);
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

#[derive(Default,Clone)]
pub struct Modes{
	modes:Vec<Mode>,
}
impl Modes{
	pub fn new(modes:Vec<Mode>)->Self{
		Self{
			modes,
		}
	}
	pub fn push_mode(&mut self,mode:Mode){
		self.modes.push(mode)
	}
	pub fn get_mode(&self,mode:ModeId)->Option<&Mode>{
		self.modes.get(mode.0 as usize)
	}
}
pub struct ModesUpdate{
	modes:HashMap<ModeId,ModeUpdate>,
}
impl Updatable<ModesUpdate> for Modes{
	fn update(&mut self,update:ModesUpdate){
		for (mode,mode_update) in update.modes{
			if let Some(mode)=self.modes.get_mut(mode.0 as usize){
				mode.update(mode_update);
			}
		}
	}
}