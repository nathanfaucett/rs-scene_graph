use core::any::Any;

use entity::Entity;


pub trait Component: Any {

    fn entity(&self) -> Option<Entity>;
    fn set_entity(&mut self, entity: Option<Entity>);

    fn destroy(&self);
    fn clear(&self);
    fn init(&self);
    fn awake(&self);
    fn update(&self);
}
