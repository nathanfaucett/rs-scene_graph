use collections::vec::Vec;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt;

use scene::Scene;


struct EntityData {
    depth: usize,
    scene: Option<Scene>,
    parent: Option<Entity>,
    children: Vec<Entity>,
}

impl fmt::Debug for EntityData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "EntityData: {{ depth: {:?} scene: {:?} parent: {:?} children: {:?} }}",
            self.depth,
            match self.scene { Some(_) => "Scene", None => "None" },
            match self.parent { Some(_) => "Parent", None => "None" },
            self.children.len(),
        )
    }
}

#[derive(Debug, Clone)]
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
            }))
        }
    }

    pub fn depth(&self) -> usize {
        self.data.borrow_mut().depth
    }
    pub fn scene(&self) -> Option<Scene> {
        match self.data.borrow_mut().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    pub fn parent(&self) -> Option<Entity> {
        match self.data.borrow_mut().parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }

    pub fn add(&self, entity: Entity) -> &Self {
        assert!(*self != entity, "entity can not be a child of itself");

        let parent = entity.parent();
        if parent != None {
            parent.unwrap().remove(entity.clone());
        }

        self.data.borrow_mut().children.push(entity.clone());

        {
            let mut entity_data = entity.data.borrow_mut();
            entity_data.depth = self.depth() + 1;
            entity_data.parent = Some(self.clone());
        }
        entity.update_children_depth();

        let scene = self.scene();
        if scene != None {
            scene.unwrap().add(entity.clone());
        }

        self
    }
    pub fn remove(&self, entity: Entity) -> &Self {
        let ref mut children = self.data.borrow_mut().children;

        match children.iter().position(|e| *e == entity) {
            Some(i) => {
                {
                    let mut entity_data = self.data.borrow_mut();
                    entity_data.depth = 0;
                    entity_data.parent = None;
                }
                entity.update_children_depth();
                children.remove(i);
                self
            },
            None => self,
        }
    }

    fn update_children_depth(&self) {
        let entity = self.data.borrow_mut();

        for child in entity.children.iter() {
            child.data.borrow_mut().depth = entity.depth + 1;
            child.update_children_depth()
        }
    }

    pub fn __set_scene(&self, scene: Scene){
        let mut entity = self.data.borrow_mut();

        for child in entity.children.iter() {
            scene.add(child.clone());
        }

        entity.scene = Some(scene);
    }
    pub fn __remove_scene(&self){
        {
            let mut entity_data = self.data.borrow_mut();
            entity_data.scene = None;
            entity_data.depth = 0;
            entity_data.parent = None;
        }
        self.update_children_depth();
    }
}

impl PartialEq<Entity> for Entity {
    fn eq(&self, other: &Entity) -> bool {
        (&*self.data.borrow_mut() as *const _) == (&*other.data.borrow_mut() as *const _)
    }
    fn ne(&self, other: &Entity) -> bool {
        !self.eq(other)
    }
}
