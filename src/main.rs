#![feature(collections, alloc)]


extern crate core;
extern crate alloc;
extern crate collections;

extern crate scene_graph;
use scene_graph::{Scene, Entity};


fn main() {
    let scene = Scene::new();
    let grandparent = Entity::new();
    let parent = Entity::new();
    let child = Entity::new();

    parent.add(child);
    grandparent.add(parent);

    scene
        .add(grandparent);

    println!("{:?}", scene);
}
