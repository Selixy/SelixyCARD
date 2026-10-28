use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeDonnee {
    Flottant,
    Vecteur2,
    Vecteur3,
    Vecteur4,
}

impl TypeDonnee {
    pub fn wgsl(&self) -> &'static str {
        match self {
            Self::Flottant => "f32",
            Self::Vecteur2 => "vec2<f32>",
            Self::Vecteur3 => "vec3<f32>",
            Self::Vecteur4 => "vec4<f32>",
        }
    }

    pub fn glsl(&self) -> &'static str {
        match self {
            Self::Flottant => "float",
            Self::Vecteur2 => "vec2",
            Self::Vecteur3 => "vec3",
            Self::Vecteur4 => "vec4",
        }
    }
}
