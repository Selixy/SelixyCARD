use crate::maths::Mat4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigurationOmbres {
    pub resolution: u32,
    pub biais_base: f32,
    pub biais_min: f32,
    pub portee_filtre: f32,
    pub zero_un: bool,
}

impl ConfigurationOmbres {
    pub fn par_defaut() -> Self {
        Self {
            resolution: 1024,
            biais_base: 0.005,
            biais_min: 0.001,
            portee_filtre: 1.0,
            zero_un: false,
        }
    }

    pub fn haute_qualite() -> Self {
        Self {
            resolution: 2048,
            biais_base: 0.005,
            biais_min: 0.001,
            portee_filtre: 1.5,
            zero_un: false,
        }
    }
}

pub fn matrice_biais(zero_un: bool) -> Mat4 {
    if zero_un {
        [
            0.5, 0.0, 0.0, 0.0,
            0.0, 0.5, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.5, 0.5, 0.0, 1.0,
        ]
    } else {
        [
            0.5, 0.0, 0.0, 0.0,
            0.0, 0.5, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.5, 0.5, 0.5, 1.0,
        ]
    }
}

pub fn biais_dynamique(
    normale: [f32; 3],
    direction_lumiere: [f32; 3],
    base: f32,
    minimum: f32,
) -> f32 {
    let scal = normale[0] * direction_lumiere[0]
        + normale[1] * direction_lumiere[1]
        + normale[2] * direction_lumiere[2];
    let cos_theta = scal.clamp(0.0, 1.0);
    let calcule = base * (1.0 - cos_theta);
    calcule.max(minimum)
}
