use component::Component;


pub trait ComponentManager {
    fn get_order(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn sort(&self) -> Self;

    fn clear(&self) -> Self;
    fn init(&self) -> Self;
    fn awake(&self) -> Self;
    fn update(&self) -> Self;

    fn has<T: Component>(&self, component: T) -> bool;
    fn add<T: Component>(&self, component: T) -> Self;
    fn remove<T: Component>(&self, component: T) -> Self;
}
