use std::sync::mpsc::{Sender, Receiver};

use alloc::boxed::Box;

use core::any::{Any, TypeId};

use scene::{SceneMsg, Scene};
use component::Component;


pub trait ComponentManager: Any {
    fn type_id(&self) -> TypeId;

    fn scene(&self) -> Option<&Scene>;
    fn scene_mut(&mut self) -> Option<&mut Scene>;
    fn set_scene(&mut self, scene: Option<Scene>);

    fn set_sender(&mut self, sender: Option<Sender<SceneMsg>>);
    fn set_receiver(&mut self, receiver: Option<Receiver<SceneMsg>>);

    fn is_empty(&self) -> bool;

    fn add_component(&mut self, component: &mut Box<Component>);
    fn remove_component(&mut self, component: &mut Box<Component>);
}

impl ComponentManager {
    impl_any!();
}
