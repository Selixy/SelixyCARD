use crate::{Moteur, scene};
use crate::decor::etat::Decor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Moteur {
    pub fn definir_decor(&mut self, decor_json: &str) -> Result<(), JsValue> {
        let decor = Decor::depuis_json(decor_json).ok_or("JSON de décor invalide")?;
        self.decor.definir(decor);
        Ok(())
    }

    pub fn definir_theme(&mut self, nom: &str) {
        self.theme = if nom == "kraft" { scene::THEME_KRAFT } else { scene::THEME_ARDOISE };
    }

    pub fn animer_apparition_decor(&mut self) {
        self.decor.reinitialiser_apparition();
    }

    pub fn pointeur_decor(&mut self, x: f32, y: f32) {
        self.decor.pointeur(x, y);
    }
}

