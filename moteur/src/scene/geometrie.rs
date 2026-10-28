use crate::maths::{self, Mat4};

pub const PLAT: f32 = -std::f32::consts::FRAC_PI_2;

#[derive(Clone, Copy)]
pub struct Pose {
    pub pos: [f32; 3],
    pub lacet: f32,
    pub tangage: f32,
}

impl Pose {
    pub fn vers(&self, cible: &Pose, k: f32) -> Pose {
        let m = |a: f32, b: f32| a + (b - a) * k;
        Pose {
            pos: std::array::from_fn(|i| m(self.pos[i], cible.pos[i])),
            lacet: m(self.lacet, cible.lacet),
            tangage: m(self.tangage, cible.tangage),
        }
    }

    pub fn decaler(&self, distance: f32) -> Pose {
        let (ct, st) = (self.tangage.cos(), self.tangage.sin());
        let (cy, sy) = (self.lacet.cos(), self.lacet.sin());
        let n = [sy * ct, -st, cy * ct];
        Pose { pos: std::array::from_fn(|i| self.pos[i] + n[i] * distance), ..*self }
    }

    pub fn glisser(&self, distance: f32) -> Pose {
        let (ct, st) = (self.tangage.cos(), self.tangage.sin());
        let (cy, sy) = (self.lacet.cos(), self.lacet.sin());
        let haut = [sy * st, ct, cy * st];
        Pose { pos: std::array::from_fn(|i| self.pos[i] + haut[i] * distance), ..*self }
    }

    pub fn coins(&self, largeur: f32, hauteur: f32) -> [[f32; 3]; 4] {
        let (ct, st) = (self.tangage.cos(), self.tangage.sin());
        let (cy, sy) = (self.lacet.cos(), self.lacet.sin());
        [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].map(|(cx, cv)| {
            let (x, y, z) = (cx * largeur, cv * hauteur * ct, cv * hauteur * st);
            [cy * x + sy * z + self.pos[0], y + self.pos[1], -sy * x + cy * z + self.pos[2]]
        })
    }
}

pub fn vers_ecran(vue_proj: &Mat4, point: [f32; 3], largeur: f32, hauteur: f32) -> [f32; 2] {
    let c = maths::transformer(vue_proj, point);
    [(c[0] / c[3] * 0.5 + 0.5) * largeur, (0.5 - c[1] / c[3] * 0.5) * hauteur]
}
