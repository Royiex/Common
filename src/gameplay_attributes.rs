use crate::model;
use crate::integer::{Time,Planar64,Planar64Vec3};

//you have this effect while in contact
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct ContactingLadder{
	pub sticky:bool
}
#[derive(Clone,Hash,Eq,PartialEq)]
pub enum ContactingBehaviour{
	Surf,
	Ladder(ContactingLadder),
	NoJump,
	Cling,//usable as a zipline, or other weird and wonderful things
	Elastic(u32),//[1/2^32,1] 0=None (elasticity+1)/2^32
}
//you have this effect while intersecting
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct IntersectingWater{
	pub viscosity:Planar64,
	pub density:Planar64,
	pub velocity:Planar64Vec3,
}
//All models can be given these attributes
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct Accelerator{
	pub acceleration:Planar64Vec3
}
#[derive(Clone,Hash,Eq,PartialEq)]
pub enum Booster{
	//Affine(crate::integer::Planar64Affine3),//capable of SetVelocity,DotVelocity,normal booster,bouncy part,redirect velocity, and much more
	Velocity(Planar64Vec3),//straight up boost velocity adds to your current velocity
	Energy{direction:Planar64Vec3,energy:Planar64},//increase energy in direction
}
#[derive(Clone,Hash,Eq,PartialEq)]
pub enum TrajectoryChoice{
	HighArcLongDuration,//underhand lob at target: less horizontal speed and more air time
	LowArcShortDuration,//overhand throw at target: more horizontal speed and less air time
}
#[derive(Clone,Hash,Eq,PartialEq)]
pub enum SetTrajectory{
	//Speed-type SetTrajectory
	AirTime(Time),//air time (relative to gravity direction) is invariant across mass and gravity changes
	Height(Planar64),//boost height (relative to gravity direction) is invariant across mass and gravity changes
	DotVelocity{direction:Planar64Vec3,dot:Planar64},//set your velocity in a specific direction without touching other directions
	//Velocity-type SetTrajectory
	TargetPointTime{//launch on a trajectory that will land at a target point in a set amount of time
		target_point:Planar64Vec3,
		time:Time,//short time = fast and direct, long time = launch high in the air, negative time = wrong way
	},
	TargetPointSpeed{//launch at a fixed speed and land at a target point
		target_point:Planar64Vec3,
		speed:Planar64,//if speed is too low this will fail to reach the target.  The closest-passing trajectory will be chosen instead
		trajectory_choice:TrajectoryChoice,
	},
	Velocity(Planar64Vec3),//SetVelocity
}
impl SetTrajectory{
	pub const fn is_velocity(&self)->bool{
		match self{
			SetTrajectory::AirTime(_)
			|SetTrajectory::Height(_)
			|SetTrajectory::DotVelocity{direction:_,dot:_}=>false,
			SetTrajectory::TargetPointTime{target_point:_,time:_}
			|SetTrajectory::TargetPointSpeed{target_point:_,speed:_,trajectory_choice:_}
			|SetTrajectory::Velocity(_)=>true,
		}
	}
}
// enum TrapCondition{
// 	FasterThan(Planar64),
// 	SlowerThan(Planar64),
// 	InRange(Planar64,Planar64),
// 	OutsideRange(Planar64,Planar64),
// }
#[derive(Clone,Hash,Eq,PartialEq)]
pub struct Wormhole{
	//destination does not need to be another wormhole
	//this defines a one way portal to a destination model transform
	//two of these can create a two way wormhole
	pub destination_model:model::ModelId,
	//(position,angles)*=origin.transform.inverse()*destination.transform
}
//attributes listed in order of handling
#[derive(Default,Clone,Hash,Eq,PartialEq)]
pub struct GeneralAttributes{
	pub booster:Option<Booster>,
	pub trajectory:Option<SetTrajectory>,
	pub wormhole:Option<Wormhole>,
	pub accelerator:Option<Accelerator>,
}
impl GeneralAttributes{
	pub const fn any(&self)->bool{
		self.booster.is_some()
		||self.trajectory.is_some()
		||self.wormhole.is_some()
		||self.accelerator.is_some()
	}
	pub fn is_wrcp(&self)->bool{
		self.trajectory.as_ref().map_or(false,|t|t.is_velocity())
		/*
		&&match &self.teleport_behaviour{
			Some(TeleportBehaviour::StageElement(
				StageElement{
					mode_id,
					stage_id:_,
					force:true,
					behaviour:StageElementBehaviour::Trigger|StageElementBehaviour::Teleport
				}
			))=>current_mode_id==*mode_id,
			_=>false,
		}
		*/
	}
}
#[derive(Default,Clone,Hash,Eq,PartialEq)]
pub struct ContactingAttributes{
	//friction?
	pub contact_behaviour:Option<ContactingBehaviour>,
}
impl ContactingAttributes{
	pub const fn any(&self)->bool{
		self.contact_behaviour.is_some()
	}
}
#[derive(Default,Clone,Hash,Eq,PartialEq)]
pub struct IntersectingAttributes{
	pub water:Option<IntersectingWater>,
}
impl IntersectingAttributes{
	pub const fn any(&self)->bool{
		self.water.is_some()
	}
}
#[derive(Clone,Copy,id::Id)]
pub struct CollisionAttributesId(u32);
#[derive(Clone,Hash,Eq,PartialEq)]
pub enum CollisionAttributes{
	Decoration,//visual only
	Contact{//track whether you are contacting the object
		contacting:ContactingAttributes,
		general:GeneralAttributes,
	},
	Intersect{//track whether you are intersecting the object
		intersecting:IntersectingAttributes,
		general:GeneralAttributes,
	},
}
impl std::default::Default for CollisionAttributes{
	fn default()->Self{
		Self::Contact{
			contacting:ContactingAttributes::default(),
			general:GeneralAttributes::default()
		}
	}
}
