use crate::ray::{Ray, HitRecord};
use crate::vec3::{Vec3, Color};
use rand::Rng;


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

    let ray_direct_unit = ray.direct().unit();
    let cos_theta = rec.normal().dot(&ray_direct_unit.reverse()).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    
    let direction;
    let cannot_refract = ri * sin_theta > 1.0;
    let mut rng = rand::thread_rng();
    let schlick = reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0);
    if cannot_refract || schlick {
        direction = ray_direct_unit.specular(rec.normal());
    } else {
        direction = ray_direct_unit.refract(rec.normal(), ri);
    }

    Some(Ray::new(*rec.pos(), direction))
}

fn reflectance(cos: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}