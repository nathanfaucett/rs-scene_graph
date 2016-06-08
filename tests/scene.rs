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
                components: Vec::new(),
            }))
        }
    }
}
impl ComponentManager for SomeComponentManager {
    fn order(&self) -> usize { 9999 }
    fn is_empty(&self) -> bool {
        self.data.borrow().components.len() == 0
    }
    fn sort(&self) {}

    fn clear(&self) {}
    fn init(&self) {}
    fn awake(&self) {}
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
    fn component_manager(&self) -> Box<ComponentManager> {
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

    assert!(scene.has_entity(grandparent) == true);
    assert!(scene.has_entity(parent) == true);
    assert!(scene.has_entity(child) == true);
}
