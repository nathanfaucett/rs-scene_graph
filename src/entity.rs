use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;
use alloc::arc::Arc;
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
    data: Arc<RefCell<EntityData>>,
}

impl Entity {

    pub fn new() -> Self {
        Entity {
            data: Arc::new(RefCell::new(EntityData {
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
            let parent = child.parent();
            if parent != None {
                parent.unwrap().remove_child(child.clone());
            }

            self.data.borrow_mut().children.push(child.clone());

            {
                let mut child_data = child.data.borrow_mut();
                child_data.depth = self.depth() + 1;
                child_data.parent = Some(self.clone());
            }
            child.update_children_depth();

            let scene = self.scene();
            if scene != None {
                scene.unwrap().add_entity(child.clone());
            }
        }
        self
    }
    pub fn has_child(&self, child: Entity) -> bool {
        let ref children = self.data.borrow().children;

        match children.iter().position(|c| *c == child) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_child(&self, child: Entity) -> &Self {
        let ref mut children = self.data.borrow_mut().children;

        match children.iter().position(|c| *c == child) {
            Some(i) => {
                {
                    let mut child_data = self.data.borrow_mut();
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
        let ref mut components = self.data.borrow_mut().components;
        let id = Id::of::<T>();

        if !components.contains_key(&id) {
            component.set_entity(Some(self.clone()));
            components.insert(id, Box::new(component));
        }
        self
    }
    pub fn has_component<T: Component + Clone>(&self) -> bool {
        self.data.borrow().components.contains_key(&Id::of::<T>())
    }
    pub fn remove_component<T: Component + Clone>(&self) -> &Self {
        let ref mut components = self.data.borrow_mut().components;
        let id = Id::of::<T>();

        if components.contains_key(&id) {
            {
                let component = components.get(&Id::of::<T>()).unwrap();
                component.set_entity(None);
            }
            components.remove(&id);
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
        {
            let mut entity_data = self.data.borrow_mut();

            for child in entity_data.children.iter() {
                scene.remove_entity(child.clone());
            }
            for (_, component) in entity_data.components.iter() {
                scene.__remove_component(component);
            }

            entity_data.depth = 0;
            entity_data.scene = None;
            entity_data.parent = None;
        }
        self.update_children_depth();
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
