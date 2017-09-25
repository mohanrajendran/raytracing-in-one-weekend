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

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo: albedo }
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
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter> {
        let target = hit_record.normal + Lambertian::random_in_unit_sphere();
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
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let f = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal { albedo: albedo, fuzz: f }
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * (2.0 * v.dot(n))
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
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: Hit) -> Option<Scatter> {
        let reflected = Metal::reflect(ray.direction().normal(), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + Metal::random_in_unit_sphere() * self.fuzz);
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