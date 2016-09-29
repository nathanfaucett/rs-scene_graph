use collections::btree_map::BTreeMap;
use collections::vec::Vec;
use collections::boxed::Box;

use shared::Shared;

use id::Id;
use scene::Scene;
use component::Component;


struct EntityData {
    depth: usize,
    scene: Option<Scene>,
    parent: Option<Entity>,
    children: Vec<Entity>,
    components: BTreeMap<Id, Box<Component>>,
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
                children: Vec::new(),
                components: BTreeMap::new(),
            })
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        {
            let ref mut data = self.data;

            data.depth = 0;
            data.scene = None;
            data.parent = None;

            for child in data.children.iter_mut() {
                child.clear();
            }
        }
        {
            let keys: Vec<Id> = self.data.components.keys().cloned().collect();

            for id in keys {
                self.remove_component_by_id(&id);
            }
        }
        self
    }

    pub fn get_depth(&self) -> usize {
        self.data.depth
    }

    pub fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    pub fn has_scene(&self) -> bool {
        match self.data.scene {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_parent(&self) -> Option<Entity> {
        match self.data.parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }
    pub fn has_parent(&self) -> bool {
        match self.data.parent {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_child(&mut self, child: &mut Entity) -> &Self {
        if *self != *child {
            if let Some(ref mut parent) = child.get_parent() {
                parent.remove_child(child);
            }

            self.data.children.push(child.clone());

            {
                let ref mut child_data = child.data;
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
        match self.data.children.iter().position(|c| *c == *child) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_child(&mut self, child: &mut Entity) -> &mut Self {
        {
            let ref mut children = self.data.children;

            match children.iter().position(|c| *c == *child) {
                Some(i) => {
                    {
                        let ref mut child_data = child.data;
                        child_data.depth = 0;
                        child_data.parent = None;
                    }
                    child.update_children_depth();
                    children.remove(i);
                },
                None => (),
            }
        }
        self
    }

    pub fn add_component<T: Component + Clone>(&mut self, mut component: T) -> &mut Self {
        let id = Id::of::<T>();

        if !self.data.components.contains_key(&id) {
            if let Some(ref mut scene) = self.get_scene() {
                scene.__add_component(&mut (Box::new(component.clone()) as Box<Component>));
            }
            component.set_entity(Some(self.clone()));
            self.data.components.insert(id, Box::new(component));
        }
        self
    }
    pub fn has_component<T: Component + Clone>(&self) -> bool {
        self.data.components.contains_key(&Id::of::<T>())
    }
    pub fn remove_component<T: Component + Clone>(&mut self) -> &mut Self {
        let id = Id::of::<T>();

        if self.data.components.contains_key(&id) {
            {
                let scene = self.get_scene();
                let ref mut data = self.data;
                let ref mut components = data.components;
                let component = components.get_mut(&Id::of::<T>()).unwrap();

                if scene.is_some() {
                    scene.unwrap().__remove_component(component);
                }
            }
            self.data.components.remove(&id);
        }
        self
    }
    pub fn remove_component_by_id<'a>(&mut self, id: &'a Id) -> &mut Self {
        if self.data.components.contains_key(&id) {
            {
                let scene = self.get_scene();
                let ref mut data = self.data;
                let ref mut components = data.components;
                let component = components.get_mut(id).unwrap();

                if scene.is_some() {
                    scene.unwrap().__remove_component(component);
                }
            }
            self.data.components.remove(&id);
        }
        self
    }
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        let ref components = self.data.components;
        let id = Id::of::<T>();

        if components.contains_key(&id) {
            let component = components.get(&id).unwrap().downcast_ref::<T>().unwrap();
            Some(component.clone())
        } else {
            None
        }
    }
    pub fn for_each_component<F>(&mut self, func: F) where F: Fn(&mut Box<Component>) {
        for (_, component) in self.data.components.iter_mut() {
            func(component);
        }
    }

    pub fn for_each_child<F>(&mut self, func: F) where F: Fn(&mut Entity) {
        for child in self.data.children.iter_mut() {
            func(child);
        }
    }
    fn update_children_depth(&mut self) {
        let ref mut entity = self.data;
        let depth = entity.depth + 1;

        for child in entity.children.iter_mut() {
            child.data.depth = depth;
            child.update_children_depth()
        }
    }

    pub fn __set_scene(&mut self, scene: &mut Scene) {
        let ref mut entity_data = self.data;

        for child in entity_data.children.iter_mut() {
            scene.add_entity(child);
        }
        for (_, component) in entity_data.components.iter_mut() {
            scene.__add_component(component);
        }

        entity_data.scene = Some(scene.clone());
    }
    pub fn __remove_scene(&mut self, scene: &mut Scene) {
        if let Some(ref mut parent) = self.get_parent() {
            parent.remove_child(self);
        }

        {
            let ref mut entity_data = self.data;

            for (_, component) in entity_data.components.iter_mut() {
                scene.__remove_component(component);
            }

            entity_data.depth = 0;
            entity_data.scene = None;
        }

        self.__remove_scene_children(scene);
        self.update_children_depth();
    }
    pub fn __remove_scene_children(&mut self, scene: &mut Scene) {
        for child in self.data.children.iter_mut() {
            scene.__remove_entity(child);
        }
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
