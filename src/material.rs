use crate::ray::{Ray, HitRecord};
use crate::vec3::{Vec3, Color};

#[derive(PartialEq, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

pub fn scatter(mat: &Material, incident: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    match mat {
        Material::Lambertian(albedo) => {
            if let Some(ray) = lambertian_scatter(rec) {
                return Some((ray, *albedo));
            }
        },
        Material::Metal(albedo, fuzz) => {
            if let Some(ray) = metal_scatter(incident, rec, fuzz) {
                return Some((ray, *albedo));
            }
        },
        Material::Dielectric(refractive_index) => {
            if let Some(ray) = dielectrics_scatter(incident, rec, refractive_index) {
                return Some((ray, Color::new([1.0, 1.0, 1.0])));
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

fn metal_scatter(ray: &Ray, rec: &HitRecord, fuzz: &f64) -> Option<Ray> {
    let mut scatter_direction = ray.direct().specular(rec.normal());
    scatter_direction = scatter_direction.unit() + fuzz.min(1.0) * Vec3::random_unit_vec();
    if scatter_direction.dot(rec.normal()) > 0.0 {
        return Some(Ray::new(*rec.pos(), scatter_direction));
    }
    None
}

fn dielectrics_scatter(ray: &Ray, rec: &HitRecord, eta: &f64) -> Option<Ray> {
    let ri: f64;
    if rec.front_face() { ri = 1.0 / eta; } 
    else { ri = *eta; }

    let refracted = ray.direct().unit().refract(rec.normal(), ri);
    Some(Ray::new(*rec.pos(), refracted))
}

