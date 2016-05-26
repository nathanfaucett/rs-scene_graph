use collections::vec::Vec;
use alloc::rc::Rc;
use core::cell::RefCell;

use entity::Entity;


#[derive(Debug)]
struct SceneData {
    entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Scene {
    data: Rc<RefCell<SceneData>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            data: Rc::new(RefCell::new(SceneData {
                entities: Vec::new(),
            }))
        }
    }

    pub fn add(&self, entity: Entity) -> &Self {
        let scene = entity.scene();
        if scene != None {
            let scene = scene.unwrap();

            if scene != *self {
                scene.remove(entity.clone());
            } else {
                return self;
            }
        }

        self.data.borrow_mut().entities.push(entity.clone());
        entity.__set_scene(self.clone());

        self
    }
    pub fn has(&self, entity: Entity) -> bool {
        let ref mut entities = self.data.borrow_mut().entities;

        match entities.iter().position(|e| *e == entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove(&self, entity: Entity) -> &Self {
        let ref mut entities = self.data.borrow_mut().entities;

        match entities.iter().position(|e| *e == entity) {
            Some(i) => {
                entities.remove(i);
                entity.__remove_scene();
                self
            },
            None => self,
        }
    }
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Scene) -> bool {
        (&*self.data.borrow_mut() as *const _) == (&*other.data.borrow_mut() as *const _)
    }
    fn ne(&self, other: &Scene) -> bool {
        !self.eq(other)
    }
}
