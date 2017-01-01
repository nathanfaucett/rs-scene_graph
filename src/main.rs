extern crate scene_graph;


use scene_graph::*;


fn main() {
    let mut scene = Scene::new();
    let mut entity0 = Entity::new();
    let entity1 = Entity::new();
    let entity2 = Entity::new();

    entity0.add_child(entity1.clone());
    entity0.add_child(entity2.clone());

    scene.add_entity(entity0.clone());
    scene.remove_entity(&mut entity0);

    assert_eq!(entity0.depth(), 0);
    assert_eq!(entity1.depth(), 1);
    assert_eq!(entity2.depth(), 1);

    assert!(entity1.parent() == Some(&entity0));
    assert!(entity2.parent() == Some(&entity0));

    assert!(entity0.scene() == None);
    assert!(entity1.scene() == None);
    assert!(entity2.scene() == None);
}
