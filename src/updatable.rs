pub trait Updatable<Updater>{
	fn update(&mut self,update:Updater);
}
#[derive(Clone,Copy,Hash,Eq,PartialEq)]
struct InnerId(u32);
#[derive(Clone)]
struct Inner{
	id:InnerId,
	enabled:bool,
}
#[derive(Clone,Copy,Hash,Eq,PartialEq)]
struct OuterId(u32);
struct Outer{
	id:OuterId,
	inners:std::collections::HashMap<InnerId,Inner>,
}

enum Update<I,U>{
	Insert(I),
	Update(U),
	Remove
}

struct InnerUpdate{
	//#[updatable(Update)]
	enabled:Option<bool>,
}
struct OuterUpdate{
	//#[updatable(Insert,Update,Remove)]
	inners:std::collections::HashMap<InnerId,Update<Inner,InnerUpdate>>,
	//#[updatable(Update)]
	//inners:std::collections::HashMap<InnerId,InnerUpdate>,
}
impl Updatable<InnerUpdate> for Inner{
	fn update(&mut self,update:InnerUpdate){
		if let Some(enabled)=update.enabled{
			self.enabled=enabled;
		}
	}
}
impl Updatable<OuterUpdate> for Outer{
	fn update(&mut self,update:OuterUpdate){
		for (id,up) in update.inners{
			match up{
				Update::Insert(new_inner)=>self.inners.insert(id,new_inner),
				Update::Update(inner_update)=>self.inners.get_mut(&id).map(|inner|{
					let old=inner.clone();
					inner.update(inner_update);
					old
				}),
				Update::Remove=>self.inners.remove(&id),
			};
		}
	}
}
//*/