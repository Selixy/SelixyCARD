use serde::{Deserialize, Serialize};
use crate::types::TypeDonnee;
use crate::valeur::Valeur;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Entree {
    Uv,
    PositionMonde,
    Normale,
    DirectionVue,
    CouleurInstance,
    FormeInstance,
    Constante(Valeur),
}

impl Entree {
    pub fn type_donnee(&self) -> TypeDonnee {
        match self {
            Self::Uv => TypeDonnee::Vecteur2,
            Self::PositionMonde | Self::Normale | Self::DirectionVue => TypeDonnee::Vecteur3,
            Self::CouleurInstance | Self::FormeInstance => TypeDonnee::Vecteur4,
            Self::Constante(v) => v.type_donnee(),
        }
    }

    pub fn wgsl(&self) -> String {
        match self {
            Self::Uv => "e.uv".into(),
            Self::PositionMonde => "e.pos_monde".into(),
            Self::Normale => "e.normale".into(),
            Self::DirectionVue => "normalize(uniformes.oeil - e.pos_monde)".into(),
            Self::CouleurInstance => "e.couleur".into(),
            Self::FormeInstance => "e.forme".into(),
            Self::Constante(v) => v.wgsl(),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Self::Uv => "v_uv".into(),
            Self::PositionMonde => "v_pos_monde".into(),
            Self::Normale => "v_normale".into(),
            Self::DirectionVue => "normalize(u_oeil - v_pos_monde)".into(),
            Self::CouleurInstance => "v_couleur".into(),
            Self::FormeInstance => "v_forme".into(),
            Self::Constante(v) => v.glsl(),
        }
    }
}
