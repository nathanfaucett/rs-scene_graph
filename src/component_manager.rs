use collections::boxed::Box;
use core::any::Any;
use core::any::TypeId;
use core::raw::TraitObject;
use core::mem::transmute;

use id::Id;
use component::Component;


pub trait ComponentManager: Any {

    fn id(&self) -> Id;

    fn order(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn destroy(&self);
    fn init(&self);
    fn update(&self);

    fn add_component(&self, component: &Box<Component>);
    fn remove_component(&self, component: &Box<Component>);
}

impl ComponentManager {
    pub fn is<T: Any>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = self.get_type_id();
        t == boxed
    }
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                let to: TraitObject = transmute(self);
                Some(&*(to.data as *const T))
            }
        } else {
            None
        }
    }
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                let to: TraitObject = transmute(self);
                Some(&mut *(to.data as *const T as *mut T))
            }
        } else {
            None
        }
    }
}
