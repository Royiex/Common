use std::collections::HashMap;
use crate::integer::{Planar64Vec3,Planar64Affine3};
use crate::gameplay_attributes;
use crate::updatable::Updatable;

pub type TextureCoordinate=glam::Vec2;
pub type Color4=glam::Vec4;
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct PositionId(u32);
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct TextureCoordinateId(u32);
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct NormalId(u32);
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct ColorId(u32);
#[derive(Clone,Hash,PartialEq,Eq)]
pub struct IndexedVertex{
	pub pos:PositionId,
	pub tex:TextureCoordinateId,
	pub normal:NormalId,
	pub color:ColorId,
}
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct VertexId(u32);
pub struct IndexedVertexList{
	vertices:Vec<VertexId>,
}
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct PolygonGroupId(u32);
pub enum PolygonGroup{
	PolygonList(Vec<IndexedVertexList>),
	//TriangleStrip(Vec<VertexId>),
}
impl PolygonGroup{
	pub fn polys(&self)->impl Iterator<Item=&[VertexId]>{
		match self{
			PolygonGroup::PolygonList(polys)=>return polys.iter().map(|poly|poly.vertices.as_slice()),
			//PolygonGroup::TriangleStrip(strip)=>return strip.windows(3).enumerate().map(|(i,s)|if i&0!=0{return s.iter().rev()}else{return s.iter()}),
		}
	}
	pub fn map_vertex_id<F:Fn(VertexId)->VertexId>(self,f:F)->Self{
		match self{
			PolygonGroup::PolygonList(polys)=>Self::PolygonList(polys.into_iter().map(|ivl|IndexedVertexList{vertices:ivl.vertices.into_iter().map(&f).collect()}).collect()),
		}
	}
}
/// Ah yes, a group of things to render at the same time
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct RenderGroupId(u32);
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct TextureId(u32);
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
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct IndexedModelId(u32);
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

#[derive(Debug,Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct ModelId(u32);
pub struct Model{
	pub model:IndexedModelId,
	pub attributes:gameplay_attributes::CollisionAttributesId,
	pub color:Color4,//transparency is in here
	pub transform:Planar64Affine3,
}
