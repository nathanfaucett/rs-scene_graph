use collections::boxed::Box;
use core::any::Any;

use id::Id;
use scene::Scene;
use component::Component;


pub trait ComponentManager: Any {

    fn get_id(&self) -> Id;

    fn get_scene(&self) -> Option<Scene>;
    fn set_scene(&mut self, scene: Option<Scene>);

    fn get_order(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn clear(&mut self);
    fn init(&mut self);
    fn update(&mut self);

    fn add_component(&mut self, component: &mut Box<Component>);
    fn remove_component(&mut self, component: &mut Box<Component>);
}

impl ComponentManager {
    impl_any!();
}
