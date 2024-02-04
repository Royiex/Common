const VALVE_SCALE:i64=16;

use crate::integer::{Time,Ratio64,Planar64,Planar64Vec3};

pub struct StyleModifiers{
	controls_used:u32,//controls which are allowed to pass into gameplay
	controls_mask:u32,//controls which are masked from control state (e.g. jump in scroll style)
	strafe:Option<StrafeSettings>,
	jump_impulse:JumpImpulse,
	jump_calculation:JumpCalculation,
	static_friction:Planar64,
	kinetic_friction:Planar64,
	walk_speed:Planar64,
	walk_accel:Planar64,
	ladder_speed:Planar64,
	ladder_accel:Planar64,
	ladder_dot:Planar64,
	swim_speed:Planar64,
	mass:Planar64,
	mv:Planar64,
	surf_slope:Option<Planar64>,
	rocket_force:Option<Planar64>,
	gravity:Planar64Vec3,
	hitbox:Hitbox,
	camera_offset:Planar64Vec3,
}
impl std::default::Default for StyleModifiers{
	fn default()->Self{
		Self::roblox_bhop()
	}
}
impl StyleModifiers{
	const CONTROL_MOVEFORWARD:u32=0b00000001;
	const CONTROL_MOVEBACK:u32=0b00000010;
	const CONTROL_MOVERIGHT:u32=0b00000100;
	const CONTROL_MOVELEFT:u32=0b00001000;
	const CONTROL_MOVEUP:u32=0b00010000;
	const CONTROL_MOVEDOWN:u32=0b00100000;
	const CONTROL_JUMP:u32=0b01000000;
	const CONTROL_ZOOM:u32=0b10000000;

	const RIGHT_DIR:Planar64Vec3=Planar64Vec3::X;
	const UP_DIR:Planar64Vec3=Planar64Vec3::Y;
	const FORWARD_DIR:Planar64Vec3=Planar64Vec3::NEG_Z;

	fn neo()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,//&!(Self::CONTROL_MOVEUP|Self::CONTROL_MOVEDOWN),
			strafe:Some(StrafeSettings{
				enable:EnableStrafe::Always,
				air_accel_limit:None,
				tick_rate:Ratio64::new(64,Time::ONE_SECOND.nanos() as u64).unwrap(),
			}),
			jump_impulse:JumpImpulse::FromEnergy(Planar64::int(512)),
			jump_calculation:JumpCalculation::Energy,
			gravity:Planar64Vec3::int(0,-80,0),
			static_friction:Planar64::int(2),
			kinetic_friction:Planar64::int(3),//unrealistic: kinetic friction is typically lower than static
			mass:Planar64::int(1),
			mv:Planar64::int(3),
			rocket_force:None,
			walk_speed:Planar64::int(16),
			walk_accel:Planar64::int(80),
			ladder_speed:Planar64::int(16),
			ladder_accel:Planar64::int(160),
			ladder_dot:(Planar64::int(1)/2).sqrt(),
			swim_speed:Planar64::int(12),
			surf_slope:Some(Planar64::raw(7)/8),
			hitbox:Hitbox::roblox(),
			camera_offset:Planar64Vec3::int(0,2,0),//4.5-2.5=2
		}
	}

	fn roblox_bhop()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,//&!(Self::CONTROL_MOVEUP|Self::CONTROL_MOVEDOWN),
			strafe:Some(StrafeSettings{
				enable:EnableStrafe::Always,
				air_accel_limit:None,
				tick_rate:Ratio64::new(100,Time::ONE_SECOND.nanos() as u64).unwrap(),
			}),
			jump_impulse:JumpImpulse::FromTime(Time::from_micros(715_588)),
			jump_calculation:JumpCalculation::Capped,
			gravity:Planar64Vec3::int(0,-100,0),
			static_friction:Planar64::int(2),
			kinetic_friction:Planar64::int(3),//unrealistic: kinetic friction is typically lower than static
			mass:Planar64::int(1),
			mv:Planar64::int(27)/10,
			rocket_force:None,
			walk_speed:Planar64::int(18),
			walk_accel:Planar64::int(90),
			ladder_speed:Planar64::int(18),
			ladder_accel:Planar64::int(180),
			ladder_dot:(Planar64::int(1)/2).sqrt(),
			swim_speed:Planar64::int(12),
			surf_slope:Some(Planar64::raw(3787805118)),// normal.y=0.75
			hitbox:Hitbox::roblox(),
			camera_offset:Planar64Vec3::int(0,2,0),//4.5-2.5=2
		}
	}
	fn roblox_surf()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,//&!(Self::CONTROL_MOVEUP|Self::CONTROL_MOVEDOWN),
			strafe:Some(StrafeSettings{
				enable:EnableStrafe::Always,
				air_accel_limit:None,
				tick_rate:Ratio64::new(100,Time::ONE_SECOND.nanos() as u64).unwrap(),
			}),
			jump_impulse:JumpImpulse::FromTime(Time::from_micros(715_588)),
			jump_calculation:JumpCalculation::Capped,
			gravity:Planar64Vec3::int(0,-50,0),
			static_friction:Planar64::int(2),
			kinetic_friction:Planar64::int(3),//unrealistic: kinetic friction is typically lower than static
			mass:Planar64::int(1),
			mv:Planar64::int(27)/10,
			rocket_force:None,
			walk_speed:Planar64::int(18),
			walk_accel:Planar64::int(90),
			ladder_speed:Planar64::int(18),
			ladder_accel:Planar64::int(180),
			ladder_dot:(Planar64::int(1)/2).sqrt(),
			swim_speed:Planar64::int(12),
			surf_slope:Some(Planar64::raw(3787805118)),// normal.y=0.75
			hitbox:Hitbox::roblox(),
			camera_offset:Planar64Vec3::int(0,2,0),//4.5-2.5=2
		}
	}

	fn source_bhop()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,//&!(Self::CONTROL_MOVEUP|Self::CONTROL_MOVEDOWN),
			strafe:Some(StrafeSettings{
				enable:EnableStrafe::Always,
				air_accel_limit:Some(Planar64::raw(150<<28)*100),
				tick_rate:Ratio64::new(100,Time::ONE_SECOND.nanos() as u64).unwrap(),
			}),
			jump_impulse:JumpImpulse::FromHeight(Planar64::int(52)/VALVE_SCALE),
			jump_calculation:JumpCalculation::Linear,
			gravity:Planar64Vec3::int(0,-800,0)/VALVE_SCALE,
			static_friction:Planar64::int(2),//?
			kinetic_friction:Planar64::int(3),//?
			mass:Planar64::int(1),
			mv:Planar64::raw(30)/VALVE_SCALE,
			rocket_force:None,
			walk_speed:Planar64::int(18),//?
			walk_accel:Planar64::int(90),//?
			ladder_speed:Planar64::int(18),//?
			ladder_accel:Planar64::int(180),//?
			ladder_dot:(Planar64::int(1)/2).sqrt(),//?
			swim_speed:Planar64::int(12),//?
			surf_slope:Some(Planar64::raw(3787805118)),// normal.y=0.75
			hitbox:Hitbox::source(),
			camera_offset:(Planar64Vec3::int(0,64,0)-Planar64Vec3::int(0,73,0)/2)/VALVE_SCALE,
		}
	}
	fn source_surf()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,//&!(Self::CONTROL_MOVEUP|Self::CONTROL_MOVEDOWN),
			strafe:Some(StrafeSettings{
				enable:EnableStrafe::Always,
				air_accel_limit:Some(Planar64::int(150)*66/VALVE_SCALE),
				tick_rate:Ratio64::new(66,Time::ONE_SECOND.nanos() as u64).unwrap(),
			}),
			jump_impulse:JumpImpulse::FromHeight(Planar64::int(52)/VALVE_SCALE),
			jump_calculation:JumpCalculation::Linear,
			gravity:Planar64Vec3::int(0,-800,0)/VALVE_SCALE,
			static_friction:Planar64::int(2),//?
			kinetic_friction:Planar64::int(3),//?
			mass:Planar64::int(1),
			mv:Planar64::int(30)/VALVE_SCALE,
			rocket_force:None,
			walk_speed:Planar64::int(18),//?
			walk_accel:Planar64::int(90),//?
			ladder_speed:Planar64::int(18),//?
			ladder_accel:Planar64::int(180),//?
			ladder_dot:(Planar64::int(1)/2).sqrt(),//?
			swim_speed:Planar64::int(12),//?
			surf_slope:Some(Planar64::raw(3787805118)),// normal.y=0.75
			hitbox:Hitbox::source(),
			camera_offset:(Planar64Vec3::int(0,64,0)-Planar64Vec3::int(0,73,0)/2)/VALVE_SCALE,
		}
	}
	fn roblox_rocket()->Self{
		Self{
			controls_used:!0,
			controls_mask:!0,
			strafe:None,
			jump_impulse:JumpImpulse::FromTime(Time::from_micros(715_588)),
			jump_calculation:JumpCalculation::Capped,
			gravity:Planar64Vec3::int(0,-100,0),
			static_friction:Planar64::int(2),
			kinetic_friction:Planar64::int(3),//unrealistic: kinetic friction is typically lower than static
			mass:Planar64::int(1),
			mv:Planar64::int(27)/10,
			rocket_force:Some(Planar64::int(200)),
			walk_speed:Planar64::int(18),
			walk_accel:Planar64::int(90),
			ladder_speed:Planar64::int(18),
			ladder_accel:Planar64::int(180),
			ladder_dot:(Planar64::int(1)/2).sqrt(),
			swim_speed:Planar64::int(12),
			surf_slope:Some(Planar64::raw(3787805118)),// normal.y=0.75
			hitbox:Hitbox::roblox(),
			camera_offset:Planar64Vec3::int(0,2,0),//4.5-2.5=2
		}
	}
}

enum JumpCalculation{
	Capped,//roblox
	Energy,//new
	Linear,//source
}

enum JumpImpulse{
	FromTime(Time),//jump time is invariant across mass and gravity changes
	FromHeight(Planar64),//jump height is invariant across mass and gravity changes
	FromDeltaV(Planar64),//jump velocity is invariant across mass and gravity changes
	FromEnergy(Planar64),// :)
}
//Jumping acts on dot(walks_state.normal,body.velocity)
//Capped means it increases the dot to the cap
//Energy means it adds energy
//Linear means it linearly adds on

enum EnableStrafe{
	Always,
	MaskAny(u32),//hsw, shsw
	MaskAll(u32),
	//Function(Box<dyn Fn(u32)->bool>),
}

struct StrafeSettings{
	enable:EnableStrafe,
	air_accel_limit:Option<Planar64>,
	tick_rate:Ratio64,
}

enum HitboxMesh{
	Box,//source
	Cylinder,//roblox
	//Sphere,//roblox old physics
	//Point,
	//Line,
	//DualCone,
}

struct Hitbox{
	halfsize:Planar64Vec3,
	mesh:HitboxMesh,
}
impl Hitbox{
	fn roblox()->Self{
		Self{
			halfsize:Planar64Vec3::int(2,5,2)/2,
			mesh:HitboxMesh::Cylinder,
		}
	}
	fn source()->Self{
		Self{
			halfsize:Planar64Vec3::raw(33,73,33)/2/VALVE_SCALE,
			mesh:HitboxMesh::Box,
		}
	}
}
