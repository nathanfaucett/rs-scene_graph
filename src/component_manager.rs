use collections::boxed::Box;
use core::any::Any;

use id::Id;
use scene::Scene;
use component::Component;


pub trait ComponentManager: Any {

    fn get_id(&self) -> Id;

    fn get_scene(&self) -> Option<Scene>;
    fn set_scene(&self, scene: Option<Scene>);

    fn get_order(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn destroy(&self);
    fn init(&self);
    fn update(&self);

    fn add_component(&self, component: &Box<Component>);
    fn remove_component(&self, component: &Box<Component>);
}

impl ComponentManager {
    impl_any!();
}
