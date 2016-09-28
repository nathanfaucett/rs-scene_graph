use collections::boxed::Box;
use core::any::Any;

use id::Id;
use entity::Entity;
use component_manager::ComponentManager;


pub trait Component: Any {

    fn get_id(&self) -> Id;

    fn new_component_manager(&self) -> Box<ComponentManager>;
    fn get_component_manager_id(&self) -> Id;

    fn get_entity(&self) -> Option<Entity>;
    fn set_entity(&mut self, entity: Option<Entity>);
}

impl Component {
    impl_any!();
}
