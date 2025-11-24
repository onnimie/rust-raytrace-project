use crate::math::vector::Vector3;
use crate::raytracing::{Ray, RayHit};
use crate::scene::Scene;

// Phong model
#[derive(Debug)]
pub struct Material {
    diffuse: Vector3<f64>,
    specular: Vector3<f64>,
    ambient: Vector3<f64>,
    shininess: f64,
}

impl Material {
    pub fn test_material() -> Self {
        Self {
            diffuse: Vector3::new(0.8, 0.2, 0.1),
            specular: Vector3::new(0.8, 0.2, 0.1),
            ambient: Vector3::new(0.8, 0.2, 0.1),
            shininess: 1.0,
        }
    }
}

pub struct DirectionalLight {
    dir: Vector3<f64>,
    intensity: Vector3<f64>,
}

pub struct PointLight {
    pos: Vector3<f64>,
    intensity: Vector3<f64>,
    constant_loss: f64,
    linear_loss: f64,
    quadratic_loss: f64,
}


impl DirectionalLight {
    pub fn new(dir: Vector3<f64>, intensity: Vector3<f64>) -> Self {
        Self {
            dir,
            intensity,
        }
    }
}

impl PointLight {
    pub fn new(pos: Vector3<f64>,
        intensity: Vector3<f64>,
        constant_loss: f64,
        linear_loss: f64,
        quadratic_loss: f64) -> Self {
            Self {
                pos, intensity, constant_loss, linear_loss, quadratic_loss
            }
    }

    pub fn move_to_pos(&mut self, new_pos: Vector3<f64>) {
        self.pos = new_pos;
    }

    pub fn move_by(&mut self, deltas: &Vector3<f64>) {
        self.pos.add(&deltas);
    }
}


pub fn compute_phong_illumination(rayhit: &RayHit, ray: &Ray, scene: &Scene) -> Vector3<f64> {
    let m: &Material = &rayhit.material;
    let ks: Vector3<f64> = m.specular;
    let kd: Vector3<f64> = m.diffuse;
    let ka: Vector3<f64> = m.ambient;
    let alpha: f64 = m.shininess;

    let ia: Vector3<f64> = scene.ambient_light_intensity;
    let n: Vector3<f64> = rayhit.normal;
    let v: Vector3<f64> = ray.dir.scaled(-1.0);

    let mut illumination: Vector3<f64> = ka.componentwise_prod(&ia);

    let mut light_sources_dir_intensity: Vec<(Vector3<f64>, Vector3<f64>)> = Vec::new();

    for ls in &scene.directional_lights {
        light_sources_dir_intensity.push((ls.dir.scaled(-1.0), ls.intensity))
    }
    for ls in &scene.point_lights {
        let dv: Vector3<f64> = ls.pos.subtracted(&rayhit.pos);
        let dir_to_light: Vector3<f64> = dv.normalized();
        let distance: f64 = dv.len();
        let attenuation = 1_f64/(ls.constant_loss + ls.linear_loss*distance + ls.quadratic_loss*distance*distance);
        let intensity = ls.intensity.scaled(attenuation);
        light_sources_dir_intensity.push((dir_to_light, intensity));
    }

    for (dir_to_light, intensity) in light_sources_dir_intensity {

        //check if in shadow (cast ray towards light, and check if intersected by another object)
        let shadow_ray: Ray = Ray::new(rayhit.pos.added(&n.scaled(0.001)), dir_to_light, 0.001);
        let shadow_hit: Option<Vector3<f64>> = shadow_ray.trace(scene, true);

        if shadow_hit == None {

            let ldotn = dir_to_light.dot(&n);
            if ldotn > 0.0 {
                let first: Vector3<f64> = kd.scaled(ldotn).componentwise_prod(&intensity);
                let r: Vector3<f64> = n.scaled(2_f64 * dir_to_light.dot(&n)).subtracted(&dir_to_light);
                illumination.add(&first);

                let rdotv = r.dot(&v);
                if rdotv > 0.0 {
                    let second: Vector3<f64> = ks.scaled(rdotv.powf(alpha)).componentwise_prod(&intensity);
                    illumination.add(&second);
                }
            }
        }
    }

    illumination.x = illumination.x.min(1.0).max(0.0);
    illumination.y = illumination.y.min(1.0).max(0.0);
    illumination.z = illumination.z.min(1.0).max(0.0);
    illumination
}