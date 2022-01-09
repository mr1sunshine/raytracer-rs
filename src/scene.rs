use crate::{ray::Ray, shapes::{HitRecord, Hittable}, Material, Object, Shape};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn add(&mut self, shape: Shape, material: Material) {
        self.objects.push(Object::new(shape, material));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, Material)> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.shape().hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t();
                result = Some((rec, *object.material()));
            }
        }

        result
    }
}
