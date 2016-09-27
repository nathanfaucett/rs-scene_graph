use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

use id::Id;
use entity::Entity;
use component::Component;
use component_manager::ComponentManager;


struct SceneData {
    initted: bool,
    entities: Vec<Entity>,
    component_managers_initted: BTreeMap<Id, bool>,
    component_managers_map: BTreeMap<Id, Rc<RefCell<Box<ComponentManager>>>>,
    component_managers: Vec<Rc<RefCell<Box<ComponentManager>>>>,
}

#[derive(Clone)]
pub struct Scene {
    data: Rc<RefCell<SceneData>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene {
            data: Rc::new(RefCell::new(SceneData {
                initted: false,
                entities: Vec::new(),
                component_managers_initted: BTreeMap::new(),
                component_managers_map: BTreeMap::new(),
                component_managers: Vec::new(),
            }))
        }
    }

    pub fn initted(&self) -> bool {
        self.data.borrow().initted
    }

    pub fn init(&self) -> &Self {
        if !self.initted() {
            let mut initted = BTreeMap::new();
            {
                let data = self.data.borrow();
                let ref component_managers = data.component_managers;
                let ref component_managers_initted = data.component_managers_initted;

                for component_manager in component_managers.iter() {
                    let component_manager = component_manager.borrow();
                    let id = component_manager.get_id();

                    if !component_managers_initted.contains_key(&id) {
                        initted.insert(id, true);
                        component_manager.init();
                    }
                }
            }
            {
                let ref mut component_managers_initted = self.data.borrow_mut().component_managers_initted;
                for (id, _) in initted.iter() {
                    component_managers_initted.insert(id.clone(), true);
                }
            }

            self.data.borrow_mut().initted = true;
        }
        self
    }

    pub fn update(&self) -> &Self {
        for component_manager in self.data.borrow().component_managers.iter() {
            component_manager.borrow().update();
        }
        self
    }

    pub fn add_entity(&self, entity: Entity) -> &Self {
        if let Some(scene) = entity.get_scene() {
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
        match self.data.borrow().entities.iter().position(|e| *e == entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_entity(&self, entity: Entity) -> &Self {
        if self.__remove_entity(entity.clone()) {
            entity.__remove_scene(self.clone());
        }
        self
    }

    pub fn for_each_entity<F>(&self, func: F) where F: Fn(&Entity) {
        for entity in self.data.borrow().entities.iter() {
            func(entity);
        }
    }

    pub fn get_component_manager<T: ComponentManager + Clone>(&self) -> Option<T> {
        let ref component_managers_map = self.data.borrow().component_managers_map;
        let id = Id::of::<T>();

        if component_managers_map.contains_key(&id) {
            let ref_component_manager = component_managers_map.get(&id).unwrap().borrow();
            let component_manager = ref_component_manager.downcast_ref::<T>().unwrap();
            Some(component_manager.clone())
        } else {
            None
        }
    }

    pub fn for_each_component_manager<F>(&self, func: F) where F: Fn(&Box<ComponentManager>) {
        for component_manager in self.data.borrow().component_managers.iter() {
            func(&component_manager.borrow());
        }
    }

    fn sort_component_managers(&self) {
        self.data.borrow_mut().component_managers.sort_by(|a, b| {
            a.borrow().get_order().cmp(&b.borrow().get_order())
        });
    }

    pub fn __remove_entity(&self, entity: Entity) -> bool {
        let removed;
        {
            let ref mut entities = self.data.borrow_mut().entities;

            match entities.iter().position(|e| *e == entity) {
                Some(i) => {
                    entities.remove(i);
                    removed = true;
                },
                None => {
                    removed = false
                },
            }
        }

        if removed {
            entity.__remove_scene_children(self.clone());
        }

        removed
    }

    pub fn __add_component(&self, component: &Box<Component>) {
        let id = component.get_component_manager_id();
        let contains_key = self.data.borrow().component_managers_map.contains_key(&id);
        let component_manager_ref;

        if !contains_key {
            let component_manager = component.new_component_manager();
            component_manager.set_scene(Some(self.clone()));

            component_manager_ref = Rc::new(RefCell::new(component_manager));

            self.data.borrow_mut().component_managers_map.insert(id, component_manager_ref.clone());
            self.data.borrow_mut().component_managers.push(component_manager_ref.clone());

            if self.initted() {
                self.sort_component_managers();
            }
        } else {
            component_manager_ref = self.data.borrow().component_managers_map.get(&id).unwrap().clone();
        }

        component_manager_ref.borrow().add_component(component);

        if self.initted() {
            if !self.data.borrow().component_managers_initted.contains_key(&id) {
                self.data.borrow_mut().component_managers_initted.insert(id, true);
            }
            component_manager_ref.borrow().init();
        }
    }
    pub fn __remove_component(&self, component: &Box<Component>) {
        let id = component.get_component_manager_id();
        let is_empty;

        {
            let ref component_managers_map = self.data.borrow().component_managers_map;
            let component_manager = component_managers_map.get(&id).unwrap().borrow();
            is_empty = component_manager.is_empty();
            component_manager.remove_component(component);
        }

        if is_empty {
            {
                let ref mut component_managers = self.data.borrow_mut().component_managers;
                match component_managers.iter().position(|c| c.borrow().get_id() == id) {
                    Some(i) => {
                        {
                            let component_manager = component_managers[i].borrow();
                            component_manager.set_scene(None);
                            component_manager.destroy();
                        }
                        component_managers.remove(i);
                    },
                    None => {},
                }
            }
            self.data.borrow_mut().component_managers_map.remove(&id);

            if self.initted() {
                if self.data.borrow().component_managers_initted.contains_key(&id) {
                    self.data.borrow_mut().component_managers_initted.remove(&id);
                }
            }
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
