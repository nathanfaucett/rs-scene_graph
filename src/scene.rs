use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use id::Id;
use entity::Entity;
use component::Component;
use component_manager::ComponentManager;


struct SceneData {
    initted: bool,
    entities: Vec<Entity>,
    component_managers: BTreeMap<Id, Box<ComponentManager>>,
}

#[derive(Clone)]
pub struct Scene {
    data: Arc<RefCell<SceneData>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene {
            data: Arc::new(RefCell::new(SceneData {
                initted: false,
                entities: Vec::new(),
                component_managers_initted: BTreeMap::new(),
                component_managers: BTreeMap::new(),
            }))
        }
    }

    pub fn update(&self) -> &Self {
        for (_, component_manager) in self.data.borrow().component_managers.iter() {
            component_manager.update();
        }
        self
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

    pub fn __add_component(&self, component: &Box<Component>) {
        let ref mut component_managers = self.data.borrow_mut().component_managers;
        let id = component.component_manager_id();

        if !component_managers.contains_key(&id) {
            let component_manager = component.component_manager();
            component_manager.init();
            component_managers.insert(id, component_manager);
        }

        let component_manager = component_managers.get(&id).unwrap();
        component_manager.add_component(component);
    }
    pub fn __remove_component(&self, component: &Box<Component>) {
        let ref mut component_managers = self.data.borrow_mut().component_managers;
        let id = component.component_manager_id();
        let is_empty;

        {
            let component_manager = component_managers.get(&id).unwrap();
            is_empty = component_manager.is_empty();
            component_manager.remove_component(component);
        }

        if is_empty {
            component_managers.get(&id).unwrap().destroy();
            component_managers.remove(&id);
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
