use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;
use core::any::Any;

use id::Id;
use entity::Entity;
use component_manager::ComponentManager;


struct SceneData {
    entities: Vec<Entity>,
    component_managers: BTreeMap<Id, Box<Any>>,
}

#[derive(Clone)]
pub struct Scene {
    data: Arc<RefCell<SceneData>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene {
            data: Arc::new(RefCell::new(SceneData {
                entities: Vec::new(),
                component_managers: BTreeMap::new(),
            }))
        }
    }

    pub fn add_entity(&self, entity: Entity) -> &Self {
        let scene = entity.scene();

        if scene != None {
            let scene = scene.unwrap();

            if scene != *self {
                scene.remove_entity(entity.clone());
            } else {
                return self;
            }
        }

        self.data.borrow_mut().entities.push(entity.clone());
        entity.__set_scene(self.clone());

        self
    }
    pub fn has_entity(&self, entity: Entity) -> bool {
        let ref entities = self.data.borrow().entities;

        match entities.iter().position(|e| *e == entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_entity(&self, entity: Entity) -> &Self {
        let ref mut entities = self.data.borrow_mut().entities;

        match entities.iter().position(|e| *e == entity) {
            Some(i) => {
                entities.remove(i);
                entity.__remove_scene(self.clone());
                self
            },
            None => self,
        }
    }

    fn add_component_manager<T: ComponentManager>(&self, component_manager: T) -> &Self {
        let ref mut component_managers = self.data.borrow_mut().component_managers;
        let id = Id::of::<T>();

        if !component_managers.contains_key(&id) {
            component_managers.insert(id, Box::new(component_manager));
        }
        self
    }
    fn has_component_manager<T: ComponentManager>(&self) -> bool {
        self.data.borrow().component_managers.contains_key(&Id::of::<T>())
    }
    fn remove_component_manager<T: ComponentManager>(&self) -> &Self {
        let ref mut component_managers = self.data.borrow_mut().component_managers;
        let id = Id::of::<T>();

        if component_managers.contains_key(&id) {
            component_managers.remove(&id);
        }
        self
    }
    pub fn get_component_manager<T: ComponentManager>(&self) -> Option<T> {
        let ref component_managers = self.data.borrow().component_managers;
        let id = Id::of::<T>();

        if component_managers.contains_key(&id) {
            let component_manager = component_managers.get(&id).unwrap().downcast_ref::<T>().unwrap();
            Some(component_manager.clone())
        } else {
            None
        }
    }
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Scene) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Scene) -> bool {
        !self.eq(other)
    }
}
