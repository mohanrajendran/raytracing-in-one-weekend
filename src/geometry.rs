use vec3::{Vec3, Ray};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut temp_rec: Option<Hit> = None;
        for hitable in self {
            if let Some(hit) = hitable.hit(ray, t_min, t_max) {
                match temp_rec {
                    Some(best) => {
                        if hit.t < best.t {
                            temp_rec = Some(hit)
                        }
                    },
                    None => temp_rec = Some(hit),
                }
            }
        }
        temp_rec
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let temp2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if temp1 < t_max && temp1 > t_min {
                let t = temp1;
                let p = ray.point_at_parameter(t);
                let n = (p - self.center).normal();
                let h = Hit {
                    t: t,
                    p: p,
                    normal: n,
                };
                Some(h)
            } else if temp2 < t_max && temp2 > t_min {
                let t = temp2;
                let p = ray.point_at_parameter(t);
                let n = (p - self.center).normal();
                let h = Hit {
                    t: t,
                    p: p,
                    normal: n,
                };
                Some(h)
            } else {
                None
            }
        } else {
            None
        }
    }
}