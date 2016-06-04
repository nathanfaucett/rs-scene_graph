use collections::vec::Vec;
use alloc::arc::Arc;
use core::cell::RefCell;

use entity::Entity;


struct SceneData {
    entities: Vec<Entity>,
}

#[derive(Clone)]
pub struct Scene {
    data: Arc<RefCell<SceneData>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            data: Arc::new(RefCell::new(SceneData {
                entities: Vec::new(),
            }))
        }
    }

    pub fn add_entity(&self, entity: Entity) -> &Self {
        let scene = entity.get_scene();
        
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
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Scene) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Scene) -> bool {
        !self.eq(other)
    }
}
