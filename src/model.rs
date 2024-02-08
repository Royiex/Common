use crate::integer::{Planar64Vec3,Planar64Affine3};
use crate::gameplay_attributes;

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
pub type IndexedVertexList=Vec<VertexId>;
pub trait PolygonIter{
	fn polys(&self)->impl Iterator<Item=&[VertexId]>;
}
pub trait MapVertexId{
	fn map_vertex_id<F:Fn(VertexId)->VertexId>(self,f:F)->Self;
}
pub struct PolygonList(Vec<IndexedVertexList>);
impl PolygonList{
	pub const fn new(list:Vec<IndexedVertexList>)->Self{
		Self(list)
	}
}
impl PolygonIter for PolygonList{
	fn polys(&self)->impl Iterator<Item=&[VertexId]>{
		self.0.iter().map(|poly|poly.as_slice())
	}
}
impl MapVertexId for PolygonList{
	fn map_vertex_id<F:Fn(VertexId)->VertexId>(self,f:F)->Self{
		Self(self.0.into_iter().map(|ivl|ivl.into_iter().map(&f).collect()).collect())
	}
}
// pub struct TriangleStrip(IndexedVertexList);
// impl PolygonIter for TriangleStrip{
// 	fn polys(&self)->impl Iterator<Item=&[VertexId]>{
// 		self.0.vertices.windows(3).enumerate().map(|(i,s)|if i&0!=0{return s.iter().rev()}else{return s.iter()})
// 	}
// }
#[derive(Clone,Copy,Hash,id::Id,PartialEq,Eq)]
pub struct PolygonGroupId(u32);
pub enum PolygonGroup{
	PolygonList(PolygonList),
	//TriangleStrip(TriangleStrip),
}
impl PolygonIter for PolygonGroup{
	fn polys(&self)->impl Iterator<Item=&[VertexId]>{
		match self{
			PolygonGroup::PolygonList(list)=>list.polys(),
			//PolygonGroup::TriangleStrip(strip)=>strip.polys(),
		}
	}
}
impl MapVertexId for PolygonGroup{
	fn map_vertex_id<F:Fn(VertexId)->VertexId>(self,f:F)->Self{
		match self{
			PolygonGroup::PolygonList(polys)=>Self::PolygonList(polys.map_vertex_id(f)),
		}
	}
}
/// Ah yes, a group of things to render at the same time
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct TextureId(u32);
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct RenderConfigId(u32);
#[derive(Default)]
pub struct RenderConfig{
	pub texture:Option<TextureId>,
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
	pub render:RenderConfigId,
	pub groups:Vec<PolygonGroupId>,
}
#[derive(Default)]
pub struct IndexedPhysicsGroup{
	//the polygons in this group are guaranteed to make a closed convex shape
	pub groups:Vec<PolygonGroupId>,
}
//This is a superset of PhysicsModel and GraphicsModel
#[derive(Clone,Copy,Hash,id::Id,Eq,PartialEq)]
pub struct MeshId(u32);
pub struct Mesh{
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
	pub mesh:MeshId,
	pub attributes:gameplay_attributes::CollisionAttributesId,
	pub color:Color4,//transparency is in here
	pub transform:Planar64Affine3,
}
