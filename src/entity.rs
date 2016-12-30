use alloc::boxed::Box;

use core::any::TypeId;

use hash_map::HashMap;
use insert::Insert;
use map::Map;
use iterable_mut::IterableMut;

use vector::Vector;
use stack::Stack;
use remove::Remove;

use shared::Shared;

use scene::{self, Scene};
use component::Component;


struct EntityInner {
    depth: usize,
    scene: Option<Scene>,
    parent: Option<Entity>,
    children: Vector<Entity>,
    components: HashMap<TypeId, Box<Component>>,
}

#[derive(Clone)]
pub struct Entity {
    inner: Shared<EntityInner>,
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            inner: unsafe {
                Shared::new(Box::into_raw(Box::new(EntityInner {
                    depth: 0usize,
                    scene: None,
                    parent: None,
                    children: Vector::new(),
                    components: HashMap::new(),
                })))
            }
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            inner.depth = 0usize;
            inner.scene = None;
            inner.parent = None;
            inner.children.clear();
            inner.components.clear();
        }
        self
    }

    pub fn depth(&self) -> usize {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            inner.depth
        } else {
            0usize
        }
    }

    pub fn parent(&self) -> Option<&Entity> {
        match unsafe {self.inner.as_ref()} {
            Some(inner) => match inner.parent {
                Some(ref parent) => Some(parent),
                None => None,
            },
            None => None,
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut Entity> {
        match unsafe {self.inner.as_mut()} {
            Some(inner) => match inner.parent {
                Some(ref mut parent) => Some(parent),
                None => None,
            },
            None => None,
        }
    }

    pub fn scene(&self) -> Option<&Scene> {
        match unsafe {self.inner.as_ref()} {
            Some(inner) => match inner.scene {
                Some(ref scene) => Some(scene),
                None => None,
            },
            None => None,
        }
    }
    pub fn scene_mut(&mut self) -> Option<&mut Scene> {
        match unsafe {self.inner.as_mut()} {
            Some(inner) => match inner.scene {
                Some(ref mut scene) => Some(scene),
                None => None,
            },
            None => None,
        }
    }

    fn update_children_depth(&mut self) {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let depth = inner.depth + 1;

            for child in inner.children.iter_mut() {
                if let Some(child_inner) = unsafe {child.inner.as_mut()} {
                    child_inner.depth = depth;
                }
                child.update_children_depth();
            }
        }
    }

    pub fn add_child(&mut self, mut entity: Entity) -> &mut Self {
        assert!(self != &entity);

        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if let Some(child_inner) = unsafe {entity.inner.as_mut()} {
                child_inner.parent = Some(self.clone());
                child_inner.depth = inner.depth + 1;
            }

            entity.update_children_depth();
            inner.children.push(entity.clone());

            if let Some(scene) = self.scene_mut() {
                scene.add_entity(entity);
            }
        }
        self
    }
    pub fn remove_child(&mut self, entity: &mut Entity) -> &mut Self {
        assert!(self != entity);

        entity.detach();

        if let Some(scene) = self.scene_mut() {
            scene.remove_entity(entity);
        }
        self
    }
    pub fn detach(&mut self) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            if let Some(ref mut parent) = inner.parent {
                if let Some(parent_inner) = unsafe {parent.inner.as_mut()} {
                    if let Some(index) = parent_inner.children.iter().position(|e| e == self) {
                        parent_inner.children.remove(&index);
                    }
                }
            }
            inner.depth = 0usize;
            inner.parent = None;
        }
        self.update_children_depth();
        self
    }

    pub fn add_component<T: Component>(&mut self, mut component: T) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let id = TypeId::of::<T>();

            if !inner.components.contains_key(&id) {

                component.set_entity(Some(self.clone()));

                let mut component = Box::new(component) as Box<Component>;

                if let Some(scene) = self.scene_mut() {
                    scene::add_component(scene, &mut component);
                }

                inner.components.insert(id, component);
            }
        }
        self
    }
    pub fn has_component<T: Component>(&self) -> bool {
        if let Some(inner) = unsafe {self.inner.as_ref()} {
            inner.components.contains_key(&TypeId::of::<T>())
        } else {
            false
        }
    }
    pub fn remove_component<T: Component>(&mut self) -> &mut Self {
        self.remove_component_by_type_id(&TypeId::of::<T>())
    }
    pub fn remove_component_by_type_id(&mut self, id: &TypeId) -> &mut Self {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let contains_key = inner.components.contains_key(&id);

            if contains_key {
                {
                    let component = inner.components.get_mut(&id).unwrap();

                    if let Some(scene) = self.scene_mut() {
                        scene::remove_component(scene, component);
                    }

                    component.set_entity(None);
                }
                inner.components.remove(&id);
            }
        }
        self
    }
    pub fn component<T: Component>(&self) -> Option<&T> {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let id = TypeId::of::<T>();

            if let Some(c) = inner.components.get(&id) {
                c.downcast_ref::<T>()
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn component_mut<T: Component>(&self) -> Option<&mut T> {
        if let Some(inner) = unsafe {self.inner.as_mut()} {
            let id = TypeId::of::<T>();

            if let Some(c) = inner.components.get_mut(&id) {
                c.downcast_mut::<T>()
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn set_scene<'a>(entity: &'a mut Entity, scene: &'a mut Scene) {
    if let Some(inner) = unsafe {entity.inner.as_mut()} {
        inner.scene = Some(scene.clone());

        for child in inner.children.iter_mut() {
            scene.add_entity(child.clone());
        }
        for (_, component) in inner.components.iter_mut() {
            scene::add_component(scene, component);
        }
    }
}
pub fn remove_scene<'a>(entity: &'a mut Entity, scene: &'a mut Scene) {
    if let Some(inner) = unsafe {entity.inner.as_mut()} {
        for (_, component) in inner.components.iter_mut() {
            scene::remove_component(scene, component);
        }
        for child in inner.children.iter_mut() {
            scene.remove_entity(child);
        }

        inner.scene = None;
    }
}

impl PartialEq<Entity> for Entity {
    fn eq(&self, other: &Self) -> bool {
        match unsafe {self.inner.as_ref()} {
            Some(a) => match unsafe {other.inner.as_ref()} {
                Some(b) => a as *const _ == b as *const _,
                None => false,
            },
            None => false,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
