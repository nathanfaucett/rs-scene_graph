#![feature(alloc)]
#![feature(core_intrinsics)]
#![feature(collections)]
#![feature(get_type_id)]
#![feature(raw)]
#![feature(reflect_marker)]
#![no_std]


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;
extern crate shared;



mod scene;
mod entity;
mod component;
mod component_manager;
mod id;

pub use scene::Scene;
pub use entity::Entity;
pub use component::Component;
pub use component_manager::ComponentManager;
pub use id::Id;
