use serde::{Deserialize, Serialize};
use crate::types::TypeDonnee;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Valeur {
    Flottant(f32),
    Vecteur2([f32; 2]),
    Vecteur3([f32; 3]),
    Vecteur4([f32; 4]),
}

impl Valeur {
    pub fn type_donnee(&self) -> TypeDonnee {
        match self {
            Self::Flottant(_) => TypeDonnee::Flottant,
            Self::Vecteur2(_) => TypeDonnee::Vecteur2,
            Self::Vecteur3(_) => TypeDonnee::Vecteur3,
            Self::Vecteur4(_) => TypeDonnee::Vecteur4,
        }
    }

    pub fn wgsl(&self) -> String {
        match self {
            Self::Flottant(v) => format!("{:.4}", v),
            Self::Vecteur2([x, y]) => format!("vec2<f32>({:.4}, {:.4})", x, y),
            Self::Vecteur3([x, y, z]) => format!("vec3<f32>({:.4}, {:.4}, {:.4})", x, y, z),
            Self::Vecteur4([x, y, z, w]) => format!("vec4<f32>({:.4}, {:.4}, {:.4}, {:.4})", x, y, z, w),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Self::Flottant(v) => format!("{:.4}", v),
            Self::Vecteur2([x, y]) => format!("vec2({:.4}, {:.4})", x, y),
            Self::Vecteur3([x, y, z]) => format!("vec3({:.4}, {:.4}, {:.4})", x, y, z),
            Self::Vecteur4([x, y, z, w]) => format!("vec4({:.4}, {:.4}, {:.4}, {:.4})", x, y, z, w),
        }
    }
}
