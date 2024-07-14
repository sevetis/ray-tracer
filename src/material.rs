use crate::ray::{Ray, HitRecord};
use crate::vec3::{Vec3, Color};

#[derive(PartialEq, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color),
}

pub fn scatter(mat: &Material, incident: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    match mat {
        Material::Lambertian(albedo) => {
            if let Some(ray) = lambertian_scatter(rec) {
                return Some((ray, *albedo));
            }
        },
        Material::Metal(albedo) => {
            if let Some(ray) = metal_scatter(incident, rec) {
                return Some((ray, *albedo));
            }
        },
    }
    None
}


fn lambertian_scatter(rec: &HitRecord) -> Option<Ray> {
    let mut scatter_direction = *rec.normal() + Vec3::random_unit_vec();
    if scatter_direction.near_zero() {
        scatter_direction = *rec.normal();
    }
    
    Some(
        Ray::new(*rec.pos(), scatter_direction)
    )
}


fn metal_scatter(ray: &Ray, rec: &HitRecord) -> Option<Ray> {
    let scatter_direction = ray.direct().specular_reflect(rec.normal());
    Some (
        Ray::new(*rec.pos(), scatter_direction)
    )
}



