#![feature(collections, alloc)]


extern crate core;
extern crate alloc;
extern crate collections;

extern crate scene_graph;
use scene_graph::{Scene, Entity};


#[test]
fn test_scene() {
    let scene = Scene::new();
    let grandparent = Entity::new();
    let parent = Entity::new();
    let child = Entity::new();

    parent.add_child(child.clone());
    grandparent.add_child(parent.clone());

    scene.add_entity(grandparent.clone());

    assert!(scene.has_entity(grandparent) == true);
    assert!(scene.has_entity(parent) == true);
    assert!(scene.has_entity(child) == true);
}
