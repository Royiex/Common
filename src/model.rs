use std::collections::HashMap;
use crate::integer::{Planar64Vec3,Planar64Affine3};
use crate::gameplay_attributes;
use crate::updatable::Updatable;

pub type TextureCoordinate=glam::Vec2;
pub type Color4=glam::Vec4;
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct PositionId(u32);
impl PositionId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct TextureCoordinateId(u32);
impl TextureCoordinateId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct NormalId(u32);
impl NormalId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct ColorId(u32);
impl ColorId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Clone,Hash,PartialEq,Eq)]
pub struct IndexedVertex{
	pub pos:PositionId,
	pub tex:TextureCoordinateId,
	pub normal:NormalId,
	pub color:ColorId,
}
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct VertexId(u32);
impl VertexId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
pub struct IndexedVertexList{
	pub vertices:Vec<VertexId>,
}
#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct PolygonGroupId(u32);
impl PolygonGroupId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
pub enum PolygonGroup{
	PolygonList(Vec<IndexedVertexList>),
	//TriangleStrip(Vec<IndexedVertexList>),
}
/// Ah yes, a group of things to render at the same time
#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct RenderGroupId(u32);
impl RenderGroupId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct TextureId(u32);
impl TextureId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
#[derive(Default)]
pub struct RenderConfig{
	texture:Option<TextureId>,
}
impl RenderConfig{
	pub const fn texture(texture:TextureId)->Self{
		Self{
			texture:Some(texture),
		}
	}
}
pub struct IndexedGraphicsGroup{
	//Render pattern material/texture/shader/flat color
	pub render:RenderGroupId,
	pub groups:Vec<PolygonGroupId>,
}
#[derive(Default)]
pub struct IndexedPhysicsGroup{
	//the polygons in this group are guaranteed to make a closed convex shape
	pub groups:Vec<PolygonGroupId>,
}
//This is a superset of PhysicsModel and GraphicsModel
#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct IndexedModelId(u32);
impl IndexedModelId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
pub struct IndexedModel{
	pub unique_pos:Vec<Planar64Vec3>,//Unit32Vec3
	pub unique_normal:Vec<Planar64Vec3>,//Unit32Vec3
	pub unique_tex:Vec<TextureCoordinate>,
	pub unique_color:Vec<Color4>,
	pub unique_vertices:Vec<IndexedVertex>,
	//polygon groups are constant texture AND convexity slices
	pub polygon_groups:Vec<PolygonGroup>,
	//graphics indexed (by texture)
	pub graphics_groups:Vec<IndexedGraphicsGroup>,
	//physics indexed (by convexity)
	pub physics_groups:Vec<IndexedPhysicsGroup>,
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct ModelId(u32);
impl ModelId{
	pub const fn id(id:u32)->Self{
		Self(id)
	}
}
pub struct Model{
	pub model:IndexedModelId,
	pub attributes:gameplay_attributes::CollisionAttributesId,
	pub color:Color4,//transparency is in here
	pub transform:Planar64Affine3,
}

pub struct Models{
	indexed_models:HashMap<IndexedModelId,IndexedModel>,
	models:HashMap<ModelId,Model>,
}
impl Models{
	pub fn new(
		indexed_models:HashMap<IndexedModelId,IndexedModel>,
		models:HashMap<ModelId,Model>,
	)->Self{
		Self{
			indexed_models,
			models,
		}
	}
}
impl Updatable<Models> for Models{
	fn update(&mut self,update:Models){
		self.indexed_models.extend(update.indexed_models);
		self.models.extend(update.models);
	}
}