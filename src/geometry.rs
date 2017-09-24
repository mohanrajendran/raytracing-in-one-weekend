use vec3::Vec3;
use ray::Ray;

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