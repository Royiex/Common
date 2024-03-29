use crate::integer::{Planar64Vec3,Planar64Affine3};
use crate::gameplay_attributes;

pub type TextureCoordinate=glam::Vec2;
pub type Color4=glam::Vec4;
#[derive(Clone,Hash,PartialEq,Eq)]
pub struct IndexedVertex{
	pub pos:u32,
	pub tex:u32,
	pub normal:u32,
	pub color:u32,
}
pub struct VertexId(u32);
pub struct IndexedVertexList{
	pub vertices:Vec<VertexId>,
}
pub struct GroupId(u32);
pub enum IndexedGroup{
	PolygonList(Vec<IndexedVertexList>),
	//TriangleStrip(Vec<IndexedVertexList>),
}
pub struct RenderId(u32);
pub struct IndexedGraphicsGroup{
	//Render pattern material/texture/shader/flat color
	pub render:RenderId,
	pub groups:Vec<GroupId>,
}
pub struct IndexedPhysicsGroup{
	//the polygons in this group are guaranteed to make a closed convex shape
	pub groups:Vec<GroupId>,
}
//This is a superset of PhysicsModel and GraphicsModel
pub struct IndexedModel{
	pub unique_pos:Vec<Planar64Vec3>,//Unit32Vec3
	pub unique_normal:Vec<Planar64Vec3>,//Unit32Vec3
	pub unique_tex:Vec<TextureCoordinate>,
	pub unique_color:Vec<Color4>,
	pub unique_vertices:Vec<IndexedVertex>,
	//groups are constant texture AND convexity slices
	pub groups:Vec<IndexedGroup>,
	//graphics indexed (by texture)
	pub graphics_sets:Vec<IndexedGraphicsGroup>,
	//physics indexed (by convexity)
	pub physics_sets:Vec<IndexedPhysicsGroup>,
}

#[derive(Clone,Copy,Hash,Eq,PartialEq)]
pub struct ModelId(u32);
pub struct Model{
	pub model:ModelId,
	pub attributes:gameplay_attributes::CollisionAttributesId,
	pub color:Color4,//transparency is in here
	pub transform:Planar64Affine3,
}
