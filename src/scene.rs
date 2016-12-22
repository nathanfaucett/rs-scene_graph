use alloc::boxed::Box;

use hash_map::HashMap;
use vector::Vector;
use iterable::Iterable;
use iterable_mut::IterableMut;
use map::Map;
use stack::Stack;
use insert::Insert;
use remove::Remove;
use shared::Shared;

use id::Id;
use entity::Entity;
use component::Component;
use component_manager::ComponentManager;
use plugin::Plugin;
use _time::Time;


struct SceneData {
    initted: bool,

    entities: Vector<Entity>,

    component_managers_map: HashMap<Id, Shared<Box<ComponentManager>>>,
    component_managers: Vector<Shared<Box<ComponentManager>>>,

    plugins_map: HashMap<Id, Shared<Box<Plugin>>>,
    plugins: Vector<Shared<Box<Plugin>>>,

    time: Time,
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

                entities: Vector::new(),

                component_managers_map: HashMap::new(),
                component_managers: Vector::new(),

                plugins_map: HashMap::new(),
                plugins: Vector::new(),

                time: Time::new(),
            })
        }
    }

    pub fn get_time(&self) -> &Time { &self.data.time }
    pub fn get_time_mut(&mut self) -> &mut Time { &mut self.data.time }

    pub fn clear(&mut self) -> &mut Self {
        for entity in self.data.entities.iter_mut() {
            entity.clear();
        }
        for plugin in self.data.plugins.iter_mut() {
            plugin.clear();
        }
        {
            let ref mut data = self.data;
            data.initted = false;

            data.entities.clear();

            data.component_managers_map.clear();
            data.component_managers.clear();

            data.plugins_map.clear();
            data.plugins.clear();
        }
        self
    }

    pub fn initted(&self) -> bool {
        self.data.initted
    }

    pub fn init(&mut self) -> &Self {
        if !self.initted() {
            self.data.initted = true;

            self.sort_plugins();
            self.sort_component_managers();

            for plugin in self.data.plugins.iter_mut() {
                plugin.init();
            }
            for component_manager in self.data.component_managers.iter_mut() {
                component_manager.init();
            }
        }
        self
    }

    pub fn update(&mut self) -> &Self {

        self.data.time.update();

        for plugin in self.data.plugins.iter_mut() {
            plugin.before();
        }
        for component_manager in self.data.component_managers.iter_mut() {
            component_manager.update();
        }
        for plugin in self.data.plugins.iter_mut() {
            plugin.after();
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

    pub fn add_plugin<T: Plugin + Clone>(&mut self, mut plugin: T) -> &mut Self {
        let shared_plugin = Shared::new(Box::new(plugin.clone()) as Box<Plugin>);

        self.data.plugins_map.insert(plugin.get_id(), shared_plugin.clone());
        self.data.plugins.push(shared_plugin);

        plugin.set_scene(Some(self.clone()));

        if self.data.initted {
            self.sort_plugins();
        }

        self
    }
    pub fn has_plugin<T: Plugin>(&self) -> bool {
        self.data.plugins_map.contains_key(&Id::of::<T>())
    }
    pub fn remove_plugin<T: Plugin + Clone>(&mut self, mut plugin: T) -> &mut Self {
        if plugin.get_scene().is_none() {
            return self;
        }
        let id = plugin.get_id();

        self.data.plugins_map.remove(&id);
        {
            let ref mut plugins = self.data.plugins;
            match plugins.iter().position(|p| p.get_id() == id) {
                Some(i) => {
                    plugins.remove(&i);
                },
                None => {},
            }
        }
        plugin.set_scene(None);
        plugin.clear();

        self
    }

    pub fn get_plugin<T: Plugin + Clone>(&self) -> Option<T> {
        let ref plugins_map = self.data.plugins_map;
        let id = Id::of::<T>();

        if plugins_map.contains_key(&id) {
            let ref_plugin = plugins_map.get(&id).unwrap();
            let plugin = ref_plugin.downcast_ref::<T>().unwrap();
            Some(plugin.clone())
        } else {
            None
        }
    }
    pub fn for_each_plugin<F>(&mut self, func: F) where F: Fn(&mut Box<Plugin>) {
        for plugin in self.data.plugins.iter_mut() {
            func(plugin);
        }
    }
    fn sort_plugins(&mut self) {
        self.data.plugins.sort_by(|a, b| {
            a.get_order().cmp(&b.get_order())
        });
    }

    pub fn __remove_entity(&mut self, entity: &mut Entity) -> bool {
        let removed;
        {
            let ref mut entities = self.data.entities;

            match entities.iter().position(|e| *e == *entity) {
                Some(i) => {
                    entities.remove(&i);
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
                        component_managers.remove(&i);
                    },
                    None => {},
                }
            }
            self.data.component_managers_map.remove(&id);
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
