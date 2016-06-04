use entity::Entity;
use component_manager::ComponentManager;


pub trait Component {

    fn get_entity(&self) -> Entity;
    fn set_entity(&self, entity: Entity);

    fn destroy(&self);
    fn clear(&self);
    fn init(&self);
    fn awake(&self);
    fn update(&self);
}
