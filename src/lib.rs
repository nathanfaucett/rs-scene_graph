#![feature(raw)]
#![feature(alloc)]
#![feature(get_type_id)]
//#![no_std]
extern crate core;


extern crate alloc;

extern crate spin;

#[macro_use]
extern crate impl_any;

extern crate hash_map;
extern crate insert;
extern crate map;
extern crate iterable;
extern crate iterable_mut;

extern crate vector;
extern crate stack;
extern crate remove;

extern crate shared;


mod component_manager;
mod component;
mod scene;
mod entity;


pub use component_manager::ComponentManager;
pub use component::Component;
pub use entity::Entity;
pub use scene::Scene;
