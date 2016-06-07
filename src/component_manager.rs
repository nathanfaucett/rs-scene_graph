use core::any::Any;


pub trait ComponentManager: Any + Clone {

    fn new() -> Self;

    fn order(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn sort(&self) -> &Self;

    fn clear(&self) -> &Self;
    fn init(&self) -> &Self;
    fn awake(&self) -> &Self;
    fn update(&self) -> &Self;
}
