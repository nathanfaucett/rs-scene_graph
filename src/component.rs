use core::any::Any;

use id::Id;
use entity::Entity;


pub trait Component: Any + Clone {

    fn component_manager() -> Id;

    fn entity(&self) -> Option<Entity>;
    fn set_entity(&self, entity: Option<Entity>) -> &Self;

    fn destroy(&self) -> &Self;
    fn clear(&self) -> &Self;
    fn init(&self) -> &Self;
    fn awake(&self) -> &Self;
    fn update(&self) -> &Self;
}
