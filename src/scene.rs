use alloc::boxed::Box;
use alloc::arc::Arc;

use core::sync::atomic::{AtomicBool, Ordering};
use core::any::TypeId;
use core::mem;

use spin::RwLock;

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


struct SceneInner {
    initted: AtomicBool,
    entities: Arc<RwLock<Vector<Entity>>>,
    component_managers: Arc<RwLock<HashMap<TypeId, Arc<RwLock<Box<ComponentManager>>>>>>,
}

#[derive(Clone)]
pub struct Scene {
    inner: Shared<SceneInner>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            inner: unsafe {
                Shared::new(Box::into_raw(Box::new(SceneInner {
                    initted: AtomicBool::new(false),
                    entities: Arc::new(RwLock::new(Vector::new())),
                    component_managers: Arc::new(RwLock::new(HashMap::new())),
                })))
            }
        }
    }

    pub fn initted(&mut self) -> bool {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            inner.initted.load(Ordering::Relaxed)
        } else {
            false
        }
    }

    pub fn init(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if inner.initted.load(Ordering::Relaxed) {
                inner.initted.store(true, Ordering::Relaxed)
            }
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            for entity in inner.entities.write().iter_mut() {
                entity.clear();
            }

            inner.component_managers.write().clear();
            inner.entities.write().clear();
        }
        self
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> &mut Self {

        entity::set_scene(&mut entity, self);

        if let Some(inner) = unsafe {self.inner.as_mut()} {
            inner.entities.write().push(entity);
        }

        self
    }
    pub fn remove_entity(&mut self, entity: &mut Entity) -> &mut Self {

        entity::remove_scene(entity, self);

        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let mut entities = inner.entities.write();

            if let Some(index) = entities.iter().position(|e| e == entity) {
                entities.remove(&index);
            }
        }

        self
    }

    pub fn has_component_manager<T: ComponentManager>(&self) -> bool {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            inner.component_managers.read().contains_key(&TypeId::of::<T>())
        } else {
            false
        }
    }
    pub fn component_manager<T: ComponentManager>(&self) -> Option<Arc<RwLock<T>>> {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            if let Some(component_manager) = inner.component_managers.read().get(&TypeId::of::<T>()) {
                Some(unsafe {mem::transmute(component_manager.clone())})
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn add_component_manager<'a>(scene: &'a mut Scene, _component_manager: &'a mut Arc<RwLock<Box<ComponentManager>>>) {
    if let Some(_inner) = unsafe {scene.inner.as_mut()} {

    }
}

pub fn remove_component_manager<'a>(scene: &'a mut Scene, _component_manager: &'a mut Arc<RwLock<Box<ComponentManager>>>) {
    if let Some(_inner) = unsafe {scene.inner.as_mut()} {

    }
}

pub fn add_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        let component_manager_type_id = component.component_manager_type_id();

        if !inner.component_managers.read().contains_key(&component_manager_type_id) {
            let mut component_manager = component.new_component_manager();
            component_manager.set_scene(Some(scene.clone()));
            inner.component_managers.write().insert(
                component_manager_type_id,
                Arc::new(RwLock::new(component_manager))
            );
        }
        let mut component_managers = inner.component_managers.write();
        let mut component_manager = component_managers.get_mut(&component_manager_type_id).unwrap();

        component_manager.write().add_component(component);

        if inner.initted.load(Ordering::Relaxed) {
            add_component_manager(scene, component_manager);
        }
    }
}

pub fn remove_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    if let Some(inner) = unsafe {scene.inner.as_mut()} {
        let component_manager_type_id = component.component_manager_type_id();
        let mut is_empty = false;

        if let Some(component_manager) = inner.component_managers.write().get_mut(&component_manager_type_id) {
            component_manager.write().remove_component(component);
            is_empty = component_manager.read().is_empty();

            if is_empty {
                remove_component_manager(scene, component_manager);
            }
        }

        if is_empty {
            inner.component_managers.write().remove(&component_manager_type_id);
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
