#![feature(alloc)]
#![feature(core_intrinsics)]
#![feature(get_type_id)]
#![feature(raw)]
#![no_std]


extern crate alloc;

#[macro_use]
extern crate impl_any;
extern crate shared;
extern crate hash_map;
extern crate vector;
extern crate map;
extern crate iterable;
extern crate iterable_mut;
extern crate stack;
extern crate insert;
extern crate remove;


mod component_manager;
mod component;
mod entity;
mod id;
mod plugin;
mod scene;

pub use component_manager::ComponentManager;
pub use component::Component;
pub use entity::Entity;
pub use id::Id;
pub use plugin::Plugin;
pub use scene::Scene;
