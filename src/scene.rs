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
use entity::{self, Entity};
use component::Component;
use component_manager::ComponentManager;
use plugin::Plugin;
use _time::Time;


struct SceneData {
    initted: bool,

    entities: Vector<Entity>,

    component_managers: HashMap<Id, Box<ComponentManager>>,
    plugins: HashMap<Id, Box<Plugin>>,

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

                component_managers: HashMap::new(),
                plugins: HashMap::new(),

                time: Time::new(),
            })
        }
    }

    pub fn get_time(&self) -> Time { self.data.read().time.clone() }

    pub fn clear(&mut self) -> &mut Self {
        for entity in self.data.write().entities.iter_mut() {
            entity.clear();
        }
        for (_, plugin) in self.data.write().plugins.iter_mut() {
            plugin.clear();
        }
        {
            let ref mut data = self.data.write();
            data.initted = false;

            data.entities.clear();

            data.component_managers.clear();
            data.plugins.clear();
        }
        self
    }

    pub fn initted(&self) -> bool {
        self.data.read().initted
    }

    pub fn init(&mut self) -> &Self {
        if !self.initted() {
            self.data.write().initted = true;

            for (_, plugin) in self.data.write().plugins.iter_mut() {
                plugin.init();
            }
            for (_, component_manager) in self.data.write().component_managers.iter_mut() {
                component_manager.init();
            }
        }
        self
    }

    pub fn update(&mut self) -> &Self {

        self.data.write().time.update();

        for (_, plugin) in self.data.write().plugins.iter_mut() {
            plugin.before();
        }
        for (_, component_manager) in self.data.write().component_managers.iter_mut() {
            component_manager.update();
        }
        for (_, plugin) in self.data.write().plugins.iter_mut() {
            plugin.after();
        }
        self
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> &mut Self {
        if let Some(ref mut scene) = entity.get_scene() {
            if *scene != *self {
                scene.remove_entity(entity.clone());
            } else {
                return self;
            }
        }

        entity::set_scene(&mut entity, self);
        self.data.write().entities.push(entity);

        self
    }
    pub fn has_entity(&self, entity: &Entity) -> bool {
        match self.data.read().entities.iter().position(|e| *e == *entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_entity(&mut self, mut entity: Entity) -> &mut Self {
        if remove_entity(self, entity.clone()) {
            entity::remove_scene(&mut entity, self);
        }
        self
    }

    pub fn for_each_entity<F>(&mut self, func: F) where F: Fn(&mut Entity) {
        for entity in self.data.write().entities.iter_mut() {
            func(entity);
        }
    }

    pub fn get_component_manager<T: ComponentManager + Clone>(&self) -> Option<T> {
        let ref component_managers = self.data.read().component_managers;
        let id = Id::of::<T>();

        if component_managers.contains_key(&id) {
            let ref_component_manager = component_managers.get(&id).unwrap();
            let component_manager = ref_component_manager.downcast_ref::<T>().unwrap();
            Some(component_manager.clone())
        } else {
            None
        }
    }

    pub fn add_plugin<T: Plugin + Clone>(&mut self, plugin: T) -> &mut Self {
        let mut plugin = Box::new(plugin.clone()) as Box<Plugin>;

        plugin.set_scene(Some(self.clone()));

        {
            let ref mut data = self.data.write();
            data.plugins.insert(plugin.get_id(), plugin);
        }

        self
    }
    pub fn has_plugin<T: Plugin>(&self) -> bool {
        self.data.read().plugins.contains_key(&Id::of::<T>())
    }
    pub fn remove_plugin<T: Plugin + Clone>(&mut self, mut plugin: T) -> &mut Self {
        if plugin.get_scene().is_none() {
            return self;
        }
        let id = plugin.get_id();

        self.data.write().plugins.remove(&id);

        plugin.set_scene(None);
        plugin.clear();

        self
    }

    pub fn get_plugin<T: Plugin + Clone>(&self) -> Option<T> {
        let ref plugins = self.data.read().plugins;
        let id = Id::of::<T>();

        if plugins.contains_key(&id) {
            let ref_plugin = plugins.get(&id).unwrap();
            let plugin = ref_plugin.downcast_ref::<T>().unwrap();
            Some(plugin.clone())
        } else {
            None
        }
    }
}

pub fn remove_entity<'a>(scene: &mut Scene, mut entity: Entity) -> bool {
    let removed;
    {
        let ref mut entities = scene.data.write().entities;

        match entities.iter().position(|e| *e == entity) {
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
        entity::remove_scene_children(&mut entity, scene);
    }

    removed
}

pub fn add_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    let id = component.get_component_manager_id();
    let contains_key = scene.data.write().component_managers.contains_key(&id);

    if !contains_key {
        let mut component_manager = component.new_component_manager();

        component_manager.set_scene(Some(scene.clone()));

        scene.data.write().component_managers.insert(id, component_manager);
    }
    {
        let mut data = scene.data.write();
        let mut component_manager = data.component_managers.get_mut(&id).unwrap();
        component_manager.add_component(component);
    }
    if scene.initted() {
        scene.data.write().component_managers.get_mut(&id).unwrap().init();
    }
}
pub fn remove_component<'a>(scene: &'a mut Scene, component: &'a mut Box<Component>) {
    let id = component.get_component_manager_id();
    let is_empty;

    {
        let ref mut component_managers = scene.data.write().component_managers;
        let component_manager = component_managers.get_mut(&id).unwrap();
        is_empty = component_manager.is_empty();
        component_manager.remove_component(component);
    }

    if is_empty {
        if let Some(component_manager) = scene.data.write().component_managers.get_mut(&id) {
            component_manager.set_scene(None);
            component_manager.clear();
        }
        scene.data.write().component_managers.remove(&id);
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
