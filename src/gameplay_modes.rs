use std::collections::{HashSet,HashMap};
use crate::model::ModelId;
use crate::gameplay_style;
use crate::gameplay_attributes;

pub struct StageElement{
	stage:StageId,//which stage spawn to send to
	force:bool,//allow setting to lower spawn id i.e. 7->3
	behaviour:StageElementBehaviour
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

pub enum ClipVelocity{
	All,//Set velocity to spawn.velocity
	Normal,//clip all out-of-plane velocity when spawning (always "top" face normal)
	None,//no clipping
}

pub struct SpawnBehaviour{
	set_camera_angles:bool,//set camera angles to spawn direction like source
	//gain access to a ridiculous amount of velocity targetting options
	set_velocity:Option<gameplay_attributes::SetTrajectory>,
	//or just the basics
	//set_velocity:ClipVelocity,
}

pub struct StageId(u32);
pub struct Stage{
	spawn:ModelId,
	spawn_behaviour:SpawnBehaviour,
	//other behaviour models of this stage can have
	ordered_checkpoints:Vec<ModelId>,
	unordered_checkpoints:HashSet<ModelId>,
}

#[derive(Clone,Hash,Eq,PartialEq)]
pub enum ZoneBehaviour{
	Finish,
	Anitcheat,
}
pub struct ModeId(u32);
pub struct Mode{
	style:gameplay_style::StyleModifiers,
	start:ModelId,
	zones:HashMap<ModelId,ZoneBehaviour>,
	stages:Vec<Stage>,
	//mutually exlusive stage element behaviour
	elements:HashMap<ModelId,StageElement>,
	jump_limit:HashMap<ModelId,u32>,
}
impl Mode{
	pub fn get_spawn_model_id(&self,stage:StageId)->Option<ModelId>{
		self.stages.get(stage.0 as usize).map(|s|s.spawn)
	}
	pub fn denormalize_data(&mut self){
		//expand and index normalized data
		for (stage_id,stage) in self.stages.iter().enumerate(){
			self.elements.insert(stage.spawn,StageElement{
				stage:StageId(stage_id as u32),
				force:false,
				behaviour:StageElementBehaviour::SpawnAt,
			});
			for &model_id in &stage.ordered_checkpoints{
				self.elements.insert(model_id,StageElement{
					stage:StageId(stage_id as u32),
					force:false,
					behaviour:StageElementBehaviour::Checkpoint,
				});
			}
			for &model_id in &stage.unordered_checkpoints{
				self.elements.insert(model_id,StageElement{
					stage:StageId(stage_id as u32),
					force:false,
					behaviour:StageElementBehaviour::Checkpoint,
				});
			}
		}
	}
}

#[derive(Default)]
pub struct Modes{
	modes:Vec<Mode>,
}
impl Modes{
	pub fn clear(&mut self){
		self.modes.clear();
	}
	pub fn get_mode(&self,mode:ModeId)->Option<&Mode>{
		self.modes.get(mode.0 as usize)
	}
	pub fn insert(&mut self,mode:Mode){
		self.modes.push(mode);
	}
}