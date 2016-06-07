#![feature(collections, alloc)]


extern crate core;
extern crate alloc;
extern crate collections;

extern crate scene_graph;


use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Entity, Component};


struct Transform2DData {
    entity: Option<Entity>,

    position_needs_update: bool,
    local_position: [f32; 2],
    position: [f32; 2],

    scale_needs_update: bool,
    local_scale: [f32; 2],
    scale: [f32; 2],

    rotation_needs_update: bool,
    local_rotation: f32,
    rotation: f32,

    matrix_needs_update: bool,
    local_matrix: [f32; 6],
    matrix: [f32; 6],
}

#[derive(Clone)]
pub struct Transform2D {
    data: Arc<RefCell<Transform2DData>>,
}

impl Transform2D {
    pub fn new() -> Self {
        Transform2D {
            data: Arc::new(RefCell::new(Transform2DData {
                entity: None,

                position_needs_update: false,
                local_position: [0f32, 0f32],
                position: [0f32, 0f32],

                scale_needs_update: false,
                local_scale: [1f32, 1f32],
                scale: [1f32, 1f32],

                rotation_needs_update: false,
                local_rotation: 0f32,
                rotation: 0f32,

                matrix_needs_update: false,
                local_matrix: [
                    1f32, 0f32, 0f32,
                    0f32, 1f32, 0f32
                ],
                matrix: [
                    1f32, 0f32, 0f32,
                    0f32, 1f32, 0f32
                ],
            }))
        }
    }

    pub fn position(&self) -> [f32; 2] {
        self.data.borrow().position
    }
    pub fn local_position(&self) -> [f32; 2] {
        self.data.borrow().local_position
    }

    pub fn scale(&self) -> [f32; 2] {
        self.data.borrow().scale
    }
    pub fn local_scale(&self) -> [f32; 2] {
        self.data.borrow().local_scale
    }

    pub fn rotation(&self) -> f32 {
        self.data.borrow().rotation
    }
    pub fn local_rotation(&self) -> f32 {
        self.data.borrow().local_rotation
    }

    pub fn matrix(&self) -> [f32; 6] {
        self.data.borrow().matrix
    }
    pub fn local_matrix(&self) -> [f32; 6] {
        self.data.borrow().local_matrix
    }
}

impl Component for Transform2D {
    fn entity(&self) -> Option<Entity> {
        self.data.borrow().entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.borrow_mut().entity = entity;
    }

    fn destroy(&self) {}
    fn clear(&self) {}
    fn init(&self) {}
    fn awake(&self) {}
    fn update(&self) {}
}
