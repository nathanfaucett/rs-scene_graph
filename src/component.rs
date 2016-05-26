use entity::Entity;
use component_manager::ComponentManager;


pub trait Component {

    fn get_component_manager<T: ComponentManager>(&self) -> T;
    fn set_component_manager<T: ComponentManager>(&self, component_manager: T) -> Self;

    fn get_entity(&self) -> Entity;
    fn set_entity(&self, entity: Entity) -> Self;

    fn destroy(&self) -> Self;
    fn clear(&self) -> Self;
    fn init(&self) -> Self;
    fn awake(&self) -> Self;
    fn update(&self) -> Self;
}
