use crate::table::etat::EtatTable;
use crate::table::pointage::{self, Vue};
use crate::{Moteur, scene};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Moteur {
    pub fn charger_texture(&mut self, cle: &str, image: &JsValue) -> Result<(), JsValue> {
        let couche = self.textures.attribuer(cle).ok_or("trop de textures chargées")?;
        self.rendu.charger_texture(couche, image)
    }

    pub fn definir_table(&mut self, etat_json: &str) -> Result<(), JsValue> {
        let etat = EtatTable::depuis_json(etat_json).ok_or("JSON de table invalide")?;
        self.table.definir(etat);
        Ok(())
    }

    pub fn carte_sous_pointeur(&mut self, x: f32, y: f32) -> i32 {
        let vue_proj = scene::camera(self.aspect, self.rendu.profondeur_zero_un());
        let vue = Vue { vue_proj, largeur: self.largeur, hauteur: self.hauteur };
        let trouvee = pointage::carte_sous_pointeur(self.table.etat(), &vue, x, y, self.survol);
        self.survol = trouvee.max(-1);
        trouvee
    }
}
