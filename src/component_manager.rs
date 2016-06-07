

pub trait ComponentManager {
    fn order(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn sort(&self);

    fn clear(&self);
    fn init(&self);
    fn awake(&self);
    fn update(&self);
}
