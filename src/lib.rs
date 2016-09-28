#![feature(collections, core_intrinsics, reflect_marker, raw, get_type_id)]
#![no_std]


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
