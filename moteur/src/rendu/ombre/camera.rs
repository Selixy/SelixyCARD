use crate::maths::{Mat4, multiplier, regarder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraLumiere {
    pub position: [f32; 3],
    pub cible: [f32; 3],
    pub haut: [f32; 3],
    pub portee: f32,
    pub pres: f32,
    pub loin: f32,
    pub zero_un: bool,
}

impl CameraLumiere {
    pub fn nouvelle(
        position: [f32; 3],
        cible: [f32; 3],
        portee: f32,
        pres: f32,
        loin: f32,
        zero_un: bool,
    ) -> Self {
        Self {
            position,
            cible,
            haut: [0.0, 1.0, 0.0],
            portee,
            pres,
            loin,
            zero_un,
        }
    }

    pub fn principale(zero_un: bool) -> Self {
        Self::nouvelle([5.0, 11.0, 4.0], [0.0, 0.0, 0.2], 18.0, 1.0, 40.0, zero_un)
    }

    pub fn direction(&self) -> [f32; 3] {
        let d = [
            self.cible[0] - self.position[0],
            self.cible[1] - self.position[1],
            self.cible[2] - self.position[2],
        ];
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if n < 1e-6 {
            [0.0, -1.0, 0.0]
        } else {
            [d[0] / n, d[1] / n, d[2] / n]
        }
    }

    pub fn matrice_orthographique(
        g: f32,
        d: f32,
        b: f32,
        h: f32,
        pres: f32,
        loin: f32,
        zero_un: bool,
    ) -> Mat4 {
        let (sz, tz) = if zero_un {
            (1.0 / (pres - loin), pres / (pres - loin))
        } else {
            (2.0 / (pres - loin), (loin + pres) / (pres - loin))
        };
        [
            2.0 / (d - g), 0.0, 0.0, 0.0,
            0.0, 2.0 / (h - b), 0.0, 0.0,
            0.0, 0.0, sz, 0.0,
            -(d + g) / (d - g), -(h + b) / (h - b), tz, 1.0,
        ]
    }

    pub fn matrice_vue_projection_lumiere(&self) -> Mat4 {
        let dir = self.direction();
        let scal = dir[0] * self.haut[0] + dir[1] * self.haut[1] + dir[2] * self.haut[2];
        let haut = if scal.abs() > 0.99 {
            [0.0, 0.0, -1.0]
        } else {
            self.haut
        };
        let vue = regarder(self.position, self.cible, haut);
        let proj = Self::matrice_orthographique(
            -self.portee,
            self.portee,
            -self.portee,
            self.portee,
            self.pres,
            self.loin,
            self.zero_un,
        );
        multiplier(&proj, &vue)
    }
}
