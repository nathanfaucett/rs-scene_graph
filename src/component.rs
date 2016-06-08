use collections::boxed::Box;
use core::any::Any;
use core::any::TypeId;
use core::raw::TraitObject;
use core::mem::transmute;

use id::Id;
use entity::Entity;
use component_manager::ComponentManager;


pub trait Component: Any {

    fn component_manager(&self) -> Box<ComponentManager>;
    fn component_manager_id(&self) -> Id;

    fn entity(&self) -> Option<Entity>;
    fn set_entity(&self, entity: Option<Entity>);
}

impl Component {
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
