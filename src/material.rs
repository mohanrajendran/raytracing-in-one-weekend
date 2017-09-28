use rand;

use geometry::Hit;
use vec3::{Ray, Vec3};

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter>;
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(0.0, 0.0, 0.0);
    loop {
        p = Vec3::new(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        ) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter> {
        let target = hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: albedo,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter> {
        let reflected = reflect(ray.direction().unit(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some(Scatter {
                scattered: scattered,
                attenuation: attenuation,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ri: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Dielectric { ri: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflected = reflect(ray.direction(), hit_record.normal);
        let (outward_normal, ni_over_nt, cosine) =
            if ray.direction().dot(hit_record.normal) > 0.0 {
                (
                    -hit_record.normal,
                    self.ri,
                    self.ri * ray.direction().dot(hit_record.normal) / ray.direction().length(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.ri,
                    -ray.direction().dot(hit_record.normal) / ray.direction().length(),
                )
            }; 
        match refract(ray.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ri);
                let scattered = if (rand::random::<f32>() < reflect_prob) {
                    Ray::new(hit_record.p, reflected)
                } else {
                    Ray::new(hit_record.p, refracted)
                };
                Some(Scatter {
                    attenuation: attenuation,
                    scattered: scattered,
                })
            }
            None => {
                let scattered = Ray::new(hit_record.p, reflected);
                Some(Scatter {
                    attenuation: attenuation,
                    scattered: scattered,
                })
            }
        }
    }
}