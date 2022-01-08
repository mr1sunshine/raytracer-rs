use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let range = min..max;
        Self {
            e: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.len_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn len_squared(&self) -> f64 {
        self.e[0].powf(2.0) + self.e[1].powf(2.0) + self.e[2].powf(2.0)
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / (v.len() as f64)
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(v, n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-(*uv), n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let range = -1.0..1.0;

        loop {
            let p = Vec3::new(
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                0.0,
            );
            if p.len_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}
