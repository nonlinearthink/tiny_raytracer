use crate::core::{Material, Point3, Ray, Vector3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub material: Option<Box<dyn Material>>,
    pub point: Option<Point3>,
    pub normal: Option<Vector3>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            material: None,
            point: None,
            normal: None,
            t: f32::INFINITY,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        let unit_outward_normal = outward_normal.normolize();

        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            Some(unit_outward_normal)
        } else {
            Some(-unit_outward_normal)
        };
    }
}
