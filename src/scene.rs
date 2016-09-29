use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;

use shared::Shared;

use id::Id;
use entity::Entity;
use component::Component;
use component_manager::ComponentManager;


struct SceneData {
    initted: bool,
    entities: Vec<Entity>,
    component_managers_initted: BTreeMap<Id, bool>,
    component_managers_map: BTreeMap<Id, Shared<Box<ComponentManager>>>,
    component_managers: Vec<Shared<Box<ComponentManager>>>,
}

#[derive(Clone)]
pub struct Scene {
    data: Shared<SceneData>,
}

impl Scene {

    pub fn new() -> Self {
        Scene {
            data: Shared::new(SceneData {
                initted: false,
                entities: Vec::new(),
                component_managers_initted: BTreeMap::new(),
                component_managers_map: BTreeMap::new(),
                component_managers: Vec::new(),
            })
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        for entity in self.data.entities.iter_mut() {
            entity.clear();
        }
        {
            let ref mut data = self.data;
            data.initted = false;
            data.entities.clear();
            data.component_managers_initted.clear();
            data.component_managers_map.clear();
            data.component_managers.clear();
        }
        self
    }

    pub fn initted(&self) -> bool {
        self.data.initted
    }

    pub fn init(&mut self) -> &Self {
        if !self.initted() {
            let mut initted = self.data.component_managers_initted.clone();
            {
                for component_manager in self.data.component_managers.iter_mut() {
                    let component_manager = component_manager;
                    let id = component_manager.get_id();

                    if !initted.contains_key(&id) {
                        initted.insert(id, true);
                        component_manager.init();
                    }
                }
            }
            {
                let ref mut component_managers_initted = self.data.component_managers_initted;
                for (id, _) in initted.iter() {
                    component_managers_initted.insert(id.clone(), true);
                }
            }

            self.data.initted = true;
        }
        self
    }

    pub fn update(&mut self) -> &Self {
        for component_manager in self.data.component_managers.iter_mut() {
            component_manager.update();
        }
        self
    }

    pub fn add_entity(&mut self, entity: &mut Entity) -> &mut Self {
        if let Some(ref mut scene) = entity.get_scene() {
            if *scene != *self {
                scene.remove_entity(entity);
            } else {
                return self;
            }
        }

        self.data.entities.push(entity.clone());
        entity.__set_scene(self);

        self
    }
    pub fn has_entity(&self, entity: &Entity) -> bool {
        match self.data.entities.iter().position(|e| *e == *entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_entity(&mut self, entity: &mut Entity) -> &mut Self {
        if self.__remove_entity(entity) {
            entity.__remove_scene(self);
        }
        self
    }

    pub fn for_each_entity<F>(&mut self, func: F) where F: Fn(&mut Entity) {
        for entity in self.data.entities.iter_mut() {
            func(entity);
        }
    }

    pub fn get_component_manager<T: ComponentManager + Clone>(&self) -> Option<T> {
        let ref component_managers_map = self.data.component_managers_map;
        let id = Id::of::<T>();

        if component_managers_map.contains_key(&id) {
            let ref_component_manager = component_managers_map.get(&id).unwrap();
            let component_manager = ref_component_manager.downcast_ref::<T>().unwrap();
            Some(component_manager.clone())
        } else {
            None
        }
    }

    pub fn for_each_component_manager<F>(&mut self, func: F) where F: Fn(&mut Box<ComponentManager>) {
        for component_manager in self.data.component_managers.iter_mut() {
            func(component_manager);
        }
    }

    fn sort_component_managers(&mut self) {
        self.data.component_managers.sort_by(|a, b| {
            a.get_order().cmp(&b.get_order())
        });
    }

    pub fn __remove_entity(&mut self, entity: &mut Entity) -> bool {
        let removed;
        {
            let ref mut entities = self.data.entities;

            match entities.iter().position(|e| *e == *entity) {
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
            entity.__remove_scene_children(self);
        }

        removed
    }

    pub fn __add_component(&mut self, component: &mut Box<Component>) {
        let id = component.get_component_manager_id();
        let contains_key = self.data.component_managers_map.contains_key(&id);
        let mut component_manager_ref;

        if !contains_key {
            let component_manager = component.new_component_manager();

            component_manager_ref = Shared::new(component_manager);

            component_manager_ref.set_scene(Some(self.clone()));

            self.data.component_managers_map.insert(id, component_manager_ref.clone());
            self.data.component_managers.push(component_manager_ref.clone());

            if self.initted() {
                self.sort_component_managers();
            }
        } else {
            component_manager_ref = self.data.component_managers_map.get(&id).unwrap().clone();
        }

        component_manager_ref.add_component(component);

        if self.initted() {
            if !self.data.component_managers_initted.contains_key(&id) {
                self.data.component_managers_initted.insert(id, true);
            }
            component_manager_ref.init();
        }
    }
    pub fn __remove_component(&mut self, component: &mut Box<Component>) {
        let id = component.get_component_manager_id();
        let is_empty;

        component.set_entity(None);

        {
            let ref mut component_managers_map = self.data.component_managers_map;
            let component_manager = component_managers_map.get_mut(&id).unwrap();
            is_empty = component_manager.is_empty();
            component_manager.remove_component(component);
        }

        if is_empty {
            {
                let ref mut component_managers = self.data.component_managers;
                match component_managers.iter().position(|c| c.get_id() == id) {
                    Some(i) => {
                        {
                            let ref mut component_manager = component_managers[i];
                            component_manager.set_scene(None);
                            component_manager.clear();
                        }
                        component_managers.remove(i);
                    },
                    None => {},
                }
            }
            self.data.component_managers_map.remove(&id);

            if self.initted() {
                if self.data.component_managers_initted.contains_key(&id) {
                    self.data.component_managers_initted.remove(&id);
                }
            }
        }
    }
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Scene) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Scene) -> bool {
        !self.eq(other)
    }
}
