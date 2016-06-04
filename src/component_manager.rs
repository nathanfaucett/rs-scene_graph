use component::Component;


pub trait ComponentManager {
    fn get_order(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn sort(&self);

    fn clear(&self);
    fn init(&self);
    fn awake(&self);
    fn update(&self);
}
