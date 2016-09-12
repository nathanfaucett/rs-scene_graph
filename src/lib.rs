#![feature(collections, alloc, core_intrinsics, reflect_marker, raw, get_type_id)]
#![no_std]


extern crate alloc;
extern crate collections;

#[macro_use]
extern crate impl_any;


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
