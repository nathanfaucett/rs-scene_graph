extern crate scene_graph;


use std::any::TypeId;

use scene_graph::{Scene, Entity, Component, ComponentManager};


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity0 = Entity::new();
    let entity1 = Entity::new();
    let entity2 = Entity::new();

    entity0.add_child(entity1.clone());
    entity0.add_child(entity2.clone());

    scene.add_entity(entity0.clone());

    assert_eq!(entity0.depth(), 0);
    assert_eq!(entity1.depth(), 1);
    assert_eq!(entity2.depth(), 1);

    assert!(entity0.scene() == Some(&scene));
    assert!(entity1.scene() == Some(&scene));
    assert!(entity2.scene() == Some(&scene));
}
#[test]
fn test_scene_remove() {
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

#[test]
fn test_entity_depth() {
    let mut entity0 = Entity::new();
    let mut entity1 = Entity::new();
    let entity2 = Entity::new();
    let mut entity3 = Entity::new();
    let entity4 = Entity::new();
    let entity5 = Entity::new();

    entity0.add_child(entity1.clone());
    entity0.add_child(entity2.clone());

    entity1.add_child(entity3.clone());
    entity3.add_child(entity4.clone());
    entity3.add_child(entity5.clone());

    assert_eq!(entity0.depth(), 0);
    assert_eq!(entity1.depth(), 1);
    assert_eq!(entity2.depth(), 1);
    assert_eq!(entity3.depth(), 2);
    assert_eq!(entity4.depth(), 3);
    assert_eq!(entity5.depth(), 3);

    assert!(entity3.parent() == Some(&entity1));
    assert!(entity4.parent() == Some(&entity3));
    assert!(entity5.parent() == Some(&entity3));
}
#[test]
fn test_entity_depth_detach() {
    let mut entity0 = Entity::new();
    let mut entity1 = Entity::new();
    let entity2 = Entity::new();
    let mut entity3 = Entity::new();
    let entity4 = Entity::new();
    let entity5 = Entity::new();

    entity0.add_child(entity1.clone());
    entity0.add_child(entity2.clone());

    entity1.add_child(entity3.clone());
    entity3.add_child(entity4.clone());
    entity3.add_child(entity5.clone());

    entity3.detach();

    assert!(entity3.parent() == None);
    assert!(entity4.parent() == Some(&entity3));
    assert!(entity5.parent() == Some(&entity3));

    assert_eq!(entity0.depth(), 0);
    assert_eq!(entity1.depth(), 1);
    assert_eq!(entity2.depth(), 1);
    assert_eq!(entity3.depth(), 0);
    assert_eq!(entity4.depth(), 1);
    assert_eq!(entity5.depth(), 1);
}

pub struct TransformManager {
    scene: Option<Scene>,
    components: usize,
}
impl TransformManager {
    pub fn new() -> Self {
        TransformManager {
            scene: None,
            components: 0usize,
        }
    }
}
impl ComponentManager for TransformManager {
    fn type_id(&self) -> TypeId {
        TypeId::of::<TransformManager>()
    }

    fn scene(&self) -> Option<&Scene> {
        match self.scene {
            Some(ref s) => Some(s),
            None => None,
        }
    }
    fn scene_mut(&mut self) -> Option<&mut Scene> {
        match self.scene {
            Some(ref mut s) => Some(s),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.scene = scene;
    }

    fn is_empty(&self) -> bool {
        self.components == 0usize
    }

    fn add_component(&mut self, _: &mut Box<Component>) {
        self.components += 1;
    }
    fn remove_component(&mut self, _: &mut Box<Component>) {
        self.components -= 1;
    }
}

pub struct Transform {
    entity: Option<Entity>,
    position: [f32; 2]
}
impl Transform {
    pub fn new() -> Self {
        Transform {
            entity: None,
            position: [0f32; 2],
        }
    }
    pub fn position(&self) -> &[f32; 2] {
        &self.position
    }
    pub fn set_position(&mut self, position: [f32; 2]) {
        self.position = position;
    }
}
impl Component for Transform {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Transform>()
    }

    fn entity(&self) -> Option<&Entity> {
        match self.entity {
            Some(ref e) => Some(e),
            None => None,
        }
    }
    fn entity_mut(&mut self) -> Option<&mut Entity> {
        match self.entity {
            Some(ref mut e) => Some(e),
            None => None,
        }
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.entity = entity;
    }

    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(TransformManager::new())
    }
    fn component_manager_type_id(&self) -> TypeId {
        TypeId::of::<TransformManager>()
    }
}

#[test]
fn test_scene_components() {
    let mut scene = Scene::new();

    let mut entity = Entity::new();
    entity.add_component(Transform::new());

    scene.add_entity(entity.clone());

    assert!(entity.has_component::<Transform>());
    assert!(scene.has_component_manager::<TransformManager>());
    {
        let transform_manager = scene.component_manager::<TransformManager>().unwrap();
        assert!(!transform_manager.read().is_empty());
    }
    {
        let mut transform = entity.component_mut::<Transform>().unwrap();
        assert_eq!(transform.position(), &[0f32; 2]);
        transform.set_position([1f32; 2]);
        assert_eq!(transform.position(), &[1f32; 2]);
    }
}
#[test]
fn test_scene_components_remove() {
    let mut scene = Scene::new();

    let mut entity = Entity::new();
    entity.add_component(Transform::new());

    scene.add_entity(entity.clone());
    scene.remove_entity(&mut entity);

    entity.remove_component::<Transform>();

    assert!(!entity.has_component::<Transform>());
    assert!(!scene.has_component_manager::<TransformManager>());
}
#[test]
fn test_scene_init() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    entity.add_component(Transform::new());
    scene.add_entity(entity.clone());
    scene.init();
}
