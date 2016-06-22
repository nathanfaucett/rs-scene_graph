#![no_std]
#![feature(collections, alloc)]


extern crate alloc;
extern crate collections;

extern crate scene_graph;

use collections::string::String;
use collections::string::ToString;
use collections::vec::Vec;
use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Scene, Entity, Component, ComponentManager, Id};


struct SomeComponentManagerData {
    scene: Option<Scene>,
    components: Vec<SomeComponent>,
}
#[derive(Clone)]
pub struct SomeComponentManager {
    data: Arc<RefCell<SomeComponentManagerData>>,
}
impl SomeComponentManager {
    fn new() -> SomeComponentManager {
        SomeComponentManager {
            data: Arc::new(RefCell::new(SomeComponentManagerData {
                scene: None,
                components: Vec::new(),
            }))
        }
    }
}
impl ComponentManager for SomeComponentManager {

    fn id(&self) -> Id { Id::of::<SomeComponentManager>() }

    fn scene(&self) -> Option<Scene> {
        match self.data.borrow().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&self, scene: Option<Scene>) {
        self.data.borrow_mut().scene = scene;
    }

    fn order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.borrow().components.len() == 0
    }

    fn destroy(&self) {}
    fn init(&self) {}
    fn update(&self) {}

    fn add_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<SomeComponent>().unwrap();
        self.data.borrow_mut().components.push(component.clone());
    }
    fn remove_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<SomeComponent>().unwrap();
        let ref mut components = self.data.borrow_mut().components;

        match components.iter().position(|c| *c == *component) {
            Some(i) => {
                components.remove(i);
            },
            None => (),
        }
    }
}

struct SomeComponentData {
    entity: Option<Entity>,
}
#[derive(Clone)]
pub struct SomeComponent {
    data: Arc<RefCell<SomeComponentData>>,
}
impl SomeComponent {
    pub fn new() -> Self {
        SomeComponent {
            data: Arc::new(RefCell::new(SomeComponentData {
                entity: None,
            }))
        }
    }
    pub fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}
impl Component for SomeComponent {

    fn id(&self) -> Id { Id::of::<SomeComponent>() }

    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(SomeComponentManager::new())
    }
    fn component_manager_id(&self) -> Id {
        Id::of::<SomeComponentManager>()
    }
    fn entity(&self) -> Option<Entity> {
        self.data.borrow().entity.clone()
    }
    fn set_entity(&self, entity: Option<Entity>) {
        self.data.borrow_mut().entity = entity;
    }
}
impl PartialEq<SomeComponent> for SomeComponent {
    fn eq(&self, other: &SomeComponent) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &SomeComponent) -> bool {
        !self.eq(other)
    }
}


#[test]
fn test_scene() {
    let scene = Scene::new();
    let grandparent = Entity::new();
    let parent = Entity::new();
    let child = Entity::new();

    grandparent.add_component(SomeComponent::new());
    parent.add_component(SomeComponent::new());
    child.add_component(SomeComponent::new());

    parent.add_child(child.clone());
    grandparent.add_child(parent.clone());

    scene.add_entity(grandparent.clone());

    assert!(grandparent.has_component::<SomeComponent>() == true);
    assert!(parent.has_component::<SomeComponent>() == true);
    assert!(child.has_component::<SomeComponent>() == true);

    let some_component = grandparent.get_component::<SomeComponent>().unwrap();
    assert!(some_component.hello() == "Hello, world!".to_string());

    assert!(scene.has_entity(grandparent.clone()) == true);
    assert!(scene.has_entity(parent.clone()) == true);
    assert!(scene.has_entity(child.clone()) == true);

    grandparent.remove_component::<SomeComponent>();
    parent.remove_component::<SomeComponent>();
    child.remove_component::<SomeComponent>();

    scene.remove_entity(child.clone());

    assert!(parent.has_child(child.clone()) == false);
    assert!(scene.has_entity(child.clone()) == false);

    scene.remove_entity(grandparent.clone());

    assert!(scene.has_entity(grandparent.clone()) == false);
    assert!(scene.has_entity(parent.clone()) == false);
    assert!(grandparent.has_child(parent.clone()) == true);
}
