use std::sync::mpsc::{channel, Sender, Receiver};

use alloc::boxed::Box;

use core::any::TypeId;

use hash_map::HashMap;
use insert::Insert;
use map::Map;
use iterable::Iterable;
use iterable_mut::IterableMut;

use vector::Vector;
use stack::Stack;
use remove::Remove;

use shared::Shared;

use entity::{self, Entity};
use component::Component;
use component_manager::ComponentManager;


#[derive(Debug)]
pub enum SceneMsg {
    Init,
    Update,
    Clear,
    Sort,
}


struct SceneInner {
    initted: bool,
    entities: Vector<Entity>,

    component_manager_sender: Sender<SceneMsg>,
    component_manager_receiver: Receiver<SceneMsg>,
    component_manager_senders: HashMap<TypeId, Sender<SceneMsg>>,

    component_managers: HashMap<TypeId, Box<ComponentManager>>,
}

#[derive(Clone)]
pub struct Scene {
    inner: Shared<SceneInner>,
}

impl Scene {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Scene {
            inner: unsafe {
                Shared::new(Box::into_raw(Box::new(SceneInner {
                    initted: false,
                    entities: Vector::new(),

                    component_manager_sender: sender,
                    component_manager_receiver: receiver,
                    component_manager_senders: HashMap::new(),

                    component_managers: HashMap::new(),
                })))
            }
        }
    }

    pub fn initted(&mut self) -> bool {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            inner.initted
        } else {
            false
        }
    }

    pub fn init(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if !inner.initted {
                let mut waiting = 0usize;

                for _ in inner.component_managers.iter() {
                    match inner.component_manager_sender.send(SceneMsg::Init) {
                        Ok(_) => waiting += 1,
                        Err(_) => (),
                    }
                }

                while waiting != 0 {
                    match inner.component_manager_receiver.recv() {
                        Ok(x) => {
                            waiting -= 1;
                            println!("{:?}", x);
                        },
                        Err(_) => (),
                    }
                }

                inner.initted = true;
            }
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            for entity in inner.entities.iter_mut() {
                entity.clear();
            }

            inner.component_managers.clear();
            inner.entities.clear();
        }
        self
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> &mut Self {

        entity::set_scene(&mut entity, self);

        if let Some(inner) = unsafe {self.inner.as_mut()} {
            inner.entities.push(entity);
        }

        self
    }
    pub fn remove_entity(&mut self, entity: &mut Entity) -> &mut Self {

        entity::remove_scene(entity, self);

        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if let Some(index) = inner.entities.iter().position(|e| e == entity) {
                inner.entities.remove(&index);
            }
        }

        self
    }

    pub fn has_component_manager<T: ComponentManager>(&self) -> bool {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            inner.component_managers.contains_key(&TypeId::of::<T>())
        } else {
            false
        }
    }
    pub fn component_manager<T: ComponentManager>(&self) -> Option<&T> {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            if let Some(component_manager) = inner.component_managers.get(&TypeId::of::<T>()) {
                component_manager.downcast_ref::<T>()
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn component_manager_mut<T: ComponentManager>(&mut self) -> Option<&mut T> {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if let Some(component_manager) = inner.component_managers.get_mut(&TypeId::of::<T>()) {
                component_manager.downcast_mut::<T>()
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn add_component_manager<'a>(scene: &'a mut Scene, component_manager: &'a mut Box<ComponentManager>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        let (sender, receiver) = channel();
        component_manager.set_sender(Some(inner.component_manager_sender.clone()));
        component_manager.set_receiver(Some(receiver));
        inner.component_manager_senders.insert(component_manager.type_id(), sender);
    }
}

pub fn remove_component_manager<'a>(scene: &'a mut Scene, component_manager: &'a mut Box<ComponentManager>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        component_manager.set_sender(None);
        component_manager.set_receiver(None);
        inner.component_manager_senders.remove(&component_manager.type_id());
    }
}

pub fn add_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        let component_manager_type_id = component.component_manager_type_id();

        if !inner.component_managers.contains_key(&component_manager_type_id) {
            let mut component_manager = component.new_component_manager();
            component_manager.set_scene(Some(scene.clone()));
            inner.component_managers.insert(component_manager_type_id, component_manager);
        }

        let mut component_manager = inner.component_managers.get_mut(&component_manager_type_id).unwrap();

        component_manager.add_component(component);

        if inner.initted {
            add_component_manager(scene, component_manager);
        }
    }
}

pub fn remove_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        let component_manager_type_id = component.component_manager_type_id();
        let mut is_empty = false;

        if let Some(component_manager) = inner.component_managers.get_mut(&component_manager_type_id) {
            component_manager.remove_component(component);
            is_empty = component_manager.is_empty();

            if is_empty {
                remove_component_manager(scene, component_manager);
            }
        }

        if is_empty {
            inner.component_managers.remove(&component_manager_type_id);
        }
    }
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Self) -> bool {
        match unsafe {self.inner.as_ref()} {
            Some(a) => match unsafe {other.inner.as_ref()} {
                Some(b) => a as *const _ == b as *const _,
                None => false,
            },
            None => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
