use core::any::Any;

use id::Id;
use scene::Scene;


pub trait Plugin: Any {

    fn get_id(&self) -> Id;

    fn get_scene(&self) -> Option<Scene>;
    fn set_scene(&mut self, scene: Option<Scene>);

    fn get_order(&self) -> usize;

    fn clear(&mut self);
    fn init(&mut self);
    fn before(&mut self);
    fn after(&mut self);
}

impl Plugin {
    impl_any!();
}
