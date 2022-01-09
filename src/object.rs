use crate::{Material, Shape};

pub struct Object {
    shape: Shape,
    material: Material
}

impl Object {
    pub fn new(shape: Shape, material: Material) -> Self { Self { shape, material } }

    /// Get a reference to the object's shape.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Get a reference to the object's material.
    pub fn material(&self) -> &Material {
        &self.material
    }
}