use alloc::boxed::Box;

use core::any::{Any, TypeId};

use scene::Scene;
use component::Component;


pub trait ComponentManager: Any {
    fn type_id(&self) -> TypeId;

    fn scene(&self) -> Option<Scene>;
    fn set_scene(&mut self, scene: Option<Scene>);

    fn is_empty(&self) -> bool;

    fn add_component(&mut self, component: &mut Box<Component>);
    fn remove_component(&mut self, component: &mut Box<Component>);
}

impl ComponentManager {
    impl_any!();
}
