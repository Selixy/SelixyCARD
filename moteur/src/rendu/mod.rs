mod donnees;
pub mod ombre;
pub mod passe_decal;
pub mod passe_ombre;
mod plan;
pub mod wgpu;

pub use wgpu::RenduWgpu;

pub use donnees::{flottants_instances, octets};
pub use ombre::{CameraLumiere, ConfigurationOmbres, TamponProfondeur, biais_dynamique, filtrage_pcf, matrice_biais, occlusion};
pub use passe_decal::{ConfigurationPasseDecal, ParametresProjection, PasseDecal, SOMMETS_BOITE, SOMMETS_PAR_BOITE};
pub use passe_ombre::{ConfigurationPasseOmbre, FormatTextureProfondeur, ParametresOmbre, PasseOmbre};
pub use plan::{Etape, Trace, planifier};

use crate::maths::Mat4;
use crate::scene::{Instance, Selection};
use wasm_bindgen::JsValue;

pub trait Rendu {
    fn profondeur_zero_un(&self) -> bool;
    fn redimensionner(&mut self, largeur: u32, hauteur: u32);
    fn dessiner(&mut self, vue_proj: &Mat4, oeil: [f32; 3], instances: &[Instance], selections: &[Selection]);
    fn charger_texture(&mut self, couche: u32, image: &JsValue) -> Result<(), JsValue>;
}

pub const MAX_INSTANCES: usize = 2048;
pub const FLOTTANTS_PAR_INSTANCE: usize = 12;
pub const TEXTURE_LARGEUR: u32 = 256;
pub const TEXTURE_HAUTEUR: u32 = 384;
pub const SOMMETS_PAR_CARTE: u32 = 6;
pub const SOMMETS_BLOC: u32 = 252;
