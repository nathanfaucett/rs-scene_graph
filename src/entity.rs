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
use scene::{self, Scene};
use component::Component;


struct EntityData {
    depth: usize,
    scene: Option<Scene>,
    parent: Option<Entity>,
    children: Vector<Entity>,
    components: HashMap<Id, Box<Component>>,
}

#[derive(Clone)]
pub struct Entity {
    data: Shared<EntityData>,
}

impl Entity {

    pub fn new() -> Self {
        Entity {
            data: Shared::new(EntityData {
                depth: 0,
                scene: None,
                parent: None,
                children: Vector::new(),
                components: HashMap::new(),
            })
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        {
            let ref mut data = self.data.write();

            data.depth = 0;
            data.scene = None;
            data.parent = None;

            for child in data.children.iter_mut() {
                child.clear();
            }
        }
        {
            let keys: Vector<Id> = self.data.read().components.keys().cloned().collect();

            for id in keys {
                self.remove_component_by_id(&id);
            }
        }
        self
    }

    pub fn get_depth(&self) -> usize {
        self.data.read().depth
    }

    pub fn get_scene(&self) -> Option<Scene> {
        match self.data.read().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    pub fn has_scene(&self) -> bool {
        match self.data.read().scene {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_parent(&self) -> Option<Entity> {
        match self.data.read().parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }
    pub fn has_parent(&self) -> bool {
        match self.data.read().parent {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_child(&mut self, mut child: Entity) -> &Self {
        if *self != child {
            if let Some(ref mut parent) = child.get_parent() {
                parent.remove_child(child.clone());
            }

            self.data.write().children.push(child.clone());

            {
                let ref mut child_data = child.data.write();
                child_data.depth = self.get_depth() + 1;
                child_data.parent = Some(self.clone());
            }
            child.update_children_depth();

            if let Some(ref mut scene) = self.get_scene() {
                scene.add_entity(child);
            }
        }
        self
    }
    pub fn has_child(&self, child: &Entity) -> bool {
        match self.data.read().children.iter().position(|c| *c == *child) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_child(&mut self, mut child: Entity) -> &mut Self {
        {
            let ref mut children = self.data.write().children;

            match children.iter().position(|c| *c == child) {
                Some(i) => {
                    {
                        let ref mut child_data = child.data.write();
                        child_data.depth = 0;
                        child_data.parent = None;
                    }
                    child.update_children_depth();
                    children.remove(&i);
                },
                None => (),
            }
        }
        self
    }

    pub fn add_component<T: Component + Clone>(&mut self, mut component: T) -> &mut Self {
        let id = Id::of::<T>();

        if !self.data.read().components.contains_key(&id) {
            if let Some(ref mut scene) = self.get_scene() {
                scene::add_component(scene, &mut (Box::new(component.clone()) as Box<Component>));
            }
            component.set_entity(Some(self.clone()));
            self.data.write().components.insert(id, Box::new(component));
        }
        self
    }
    pub fn has_component<T: Component + Clone>(&self) -> bool {
        self.data.read().components.contains_key(&Id::of::<T>())
    }
    pub fn remove_component<T: Component + Clone>(&mut self) -> &mut Self {
        self.remove_component_by_id(&Id::of::<T>())
    }
    pub fn remove_component_by_id(&mut self, id: &Id) -> &mut Self {
        if self.data.read().components.contains_key(&id) {
            {
                let scene = self.get_scene();
                let ref mut data = self.data.write();
                let ref mut components = data.components;
                let mut component = components.get_mut(id).unwrap();

                component.set_entity(None);

                if scene.is_some() {
                    scene::remove_component(&mut scene.unwrap(), component);
                }
            }
            self.data.write().components.remove(&id);
        }
        self
    }
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        let ref components = self.data.read().components;
        let id = Id::of::<T>();

        if components.contains_key(&id) {
            let component = components.get(&id).unwrap().downcast_ref::<T>().unwrap();
            Some(component.clone())
        } else {
            None
        }
    }
    pub fn for_each_component<F>(&mut self, func: F) where F: Fn(&mut Box<Component>) {
        for (_, component) in self.data.write().components.iter_mut() {
            func(component);
        }
    }

    pub fn for_each_child<F>(&mut self, func: F) where F: Fn(&mut Entity) {
        for child in self.data.write().children.iter_mut() {
            func(child);
        }
    }
    fn update_children_depth(&mut self) {
        let ref mut entity = self.data.write();
        let depth = entity.depth + 1;

        for child in entity.children.iter_mut() {
            child.data.write().depth = depth;
            child.update_children_depth()
        }
    }
}

pub fn set_scene<'a>(entity: &'a mut Entity, scene: &'a mut Scene) {

    for child in entity.data.read().children.iter() {
        scene.add_entity(child.clone());
    }
    for (_, component) in entity.data.write().components.iter_mut() {
        scene::add_component(scene, component);
    }

    entity.data.write().scene = Some(scene.clone());
}
pub fn remove_scene<'a>(entity: &'a mut Entity, scene: &'a mut Scene) {
    if let Some(ref mut parent) = entity.get_parent() {
        parent.remove_child(entity.clone());
    }

    {
        let ref mut entity_data = entity.data.write();

        for (_, component) in entity_data.components.iter_mut() {
            scene::remove_component(scene, component);
        }

        entity_data.depth = 0;
        entity_data.scene = None;
    }

    remove_scene_children(entity, scene);
    entity.update_children_depth();
}
pub fn remove_scene_children<'a>(entity: &'a mut Entity, scene: &'a mut Scene) {
    for child in entity.data.write().children.iter_mut() {
        scene::remove_entity(scene, child.clone());
    }
}

impl PartialEq<Entity> for Entity {
    fn eq(&self, other: &Entity) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Entity) -> bool {
        !self.eq(other)
    }
}
