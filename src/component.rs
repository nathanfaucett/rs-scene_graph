use alloc::boxed::Box;

use core::any::{Any, TypeId};

use entity::Entity;
use component_manager::ComponentManager;


pub trait Component: Any {
    fn type_id(&self) -> TypeId;

    fn entity(&self) -> Option<&Entity>;
    fn entity_mut(&mut self) -> Option<&mut Entity>;
    fn set_entity(&mut self, entity: Option<Entity>);

    fn new_component_manager(&self) -> Box<ComponentManager>;
    fn component_manager_type_id(&self) -> TypeId;
}

impl Component {
    impl_any!();
}
