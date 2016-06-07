#![feature(collections, alloc)]


extern crate core;
extern crate alloc;
extern crate collections;

extern crate scene_graph;

mod transform2d;

use scene_graph::{Scene, Entity};
use transform2d::Transform2D;


#[test]
fn test_scene() {
    let scene = Scene::new();
    let grandparent = Entity::new();
    let parent = Entity::new();
    let child = Entity::new();

    grandparent.add_component(Transform2D::new());
    parent.add_component(Transform2D::new());
    child.add_component(Transform2D::new());

    parent.add_child(child.clone());
    grandparent.add_child(parent.clone());

    scene.add_entity(grandparent.clone());

    assert!(grandparent.has_component::<Transform2D>() == true);
    assert!(parent.has_component::<Transform2D>() == true);
    assert!(child.has_component::<Transform2D>() == true);

    let transform = grandparent.get_component::<Transform2D>().unwrap();
    assert!(transform.position() == [0f32, 0f32]);

    assert!(scene.has_entity(grandparent) == true);
    assert!(scene.has_entity(parent) == true);
    assert!(scene.has_entity(child) == true);
}
