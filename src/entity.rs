use collections::btree_map::BTreeMap;
use collections::vec::Vec;
use collections::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

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
    data: Rc<RefCell<EntityData>>,
}

impl Entity {

    pub fn new() -> Self {
        Entity {
            data: Rc::new(RefCell::new(EntityData {
                depth: 0,
                scene: None,
                parent: None,
                children: Vec::new(),
                components: BTreeMap::new(),
            }))
        }
    }

    pub fn depth(&self) -> usize {
        self.data.borrow().depth
    }

    pub fn scene(&self) -> Option<Scene> {
        match self.data.borrow().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    pub fn has_scene(&self) -> bool {
        match self.data.borrow().scene {
            Some(_) => true,
            None => false,
        }
    }

    pub fn parent(&self) -> Option<Entity> {
        match self.data.borrow().parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }
    pub fn has_parent(&self) -> bool {
        match self.data.borrow().parent {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_child(&self, child: Entity) -> &Self {
        if *self != child {
            if let Some(parent) = child.parent() {
                parent.remove_child(child.clone());
            }

            self.data.borrow_mut().children.push(child.clone());

            {
                let mut child_data = child.data.borrow_mut();
                child_data.depth = self.depth() + 1;
                child_data.parent = Some(self.clone());
            }
            child.update_children_depth();

            if let Some(scene) = self.scene() {
                scene.add_entity(child.clone());
            }
        }
        self
    }
    pub fn has_child(&self, child: Entity) -> bool {
        match self.data.borrow().children.iter().position(|c| *c == child) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_child(&self, child: Entity) -> &Self {
        let ref mut children = self.data.borrow_mut().children;

        match children.iter().position(|c| *c == child) {
            Some(i) => {
                {
                    let mut child_data = child.data.borrow_mut();
                    child_data.depth = 0;
                    child_data.parent = None;
                }
                child.update_children_depth();
                children.remove(i);
                self
            },
            None => self,
        }
    }

    pub fn add_component<T: Component + Clone>(&self, component: T) -> &Self {
        let id = Id::of::<T>();

        if !self.data.borrow().components.contains_key(&id) {
            if let Some(scene) = self.scene() {
                scene.__add_component(&(Box::new(component.clone()) as Box<Component>));
            }
            component.set_entity(Some(self.clone()));
            self.data.borrow_mut().components.insert(id, Box::new(component));
        }
        self
    }
    pub fn has_component<T: Component + Clone>(&self) -> bool {
        self.data.borrow().components.contains_key(&Id::of::<T>())
    }
    pub fn remove_component<T: Component + Clone>(&self) -> &Self {
        let id = Id::of::<T>();

        if self.data.borrow().components.contains_key(&id) {
            {
                let ref components = self.data.borrow().components;
                let component = components.get(&Id::of::<T>()).unwrap();

                if let Some(scene) = self.scene() {
                    scene.__remove_component(component);
                }

                component.set_entity(None);
            }
            self.data.borrow_mut().components.remove(&id);
        }
        self
    }
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        let ref components = self.data.borrow().components;
        let id = Id::of::<T>();

        if components.contains_key(&id) {
            let component = components.get(&id).unwrap().downcast_ref::<T>().unwrap();
            Some(component.clone())
        } else {
            None
        }
    }
    pub fn each_component<F>(&self, func: F) where F: Fn(&Box<Component>) {
        for (_, component) in self.data.borrow().components.iter() {
            func(component);
        }
    }

    pub fn each_child<F>(&self, func: F) where F: Fn(&Entity) {
        for child in self.data.borrow().children.iter() {
            func(child);
        }
    }
    fn update_children_depth(&self) {
        let entity = self.data.borrow_mut();

        for child in entity.children.iter() {
            child.data.borrow_mut().depth = entity.depth + 1;
            child.update_children_depth()
        }
    }

    pub fn __set_scene(&self, scene: Scene) {
        let mut entity_data = self.data.borrow_mut();

        for child in entity_data.children.iter() {
            scene.add_entity(child.clone());
        }
        for (_, component) in entity_data.components.iter() {
            scene.__add_component(component);
        }

        entity_data.scene = Some(scene);
    }
    pub fn __remove_scene(&self, scene: Scene) {
        if let Some(parent) = self.parent() {
            parent.remove_child(self.clone());
        }

        {
            let mut entity_data = self.data.borrow_mut();

            for (_, component) in entity_data.components.iter() {
                scene.__remove_component(component);
            }

            entity_data.depth = 0;
            entity_data.scene = None;
        }

        self.__remove_scene_children(scene);
        self.update_children_depth();
    }
    pub fn __remove_scene_children(&self, scene: Scene) {
        for child in self.data.borrow_mut().children.iter() {
            scene.__remove_entity(child.clone());
        }
    }
}

impl PartialEq<Entity> for Entity {
    fn eq(&self, other: &Entity) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Entity) -> bool {
        !self.eq(other)
    }
}
