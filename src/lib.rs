#![feature(collections, alloc)]
#![no_std]


extern crate alloc;
extern crate collections;


mod scene;
mod entity;
mod component;
mod component_manager;

pub use scene::Scene;
pub use entity::Entity;
pub use component::Component;
pub use component_manager::ComponentManager;
