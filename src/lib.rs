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
