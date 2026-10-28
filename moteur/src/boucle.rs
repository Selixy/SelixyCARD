use crate::{Moteur, scene, table};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Moteur {
    pub fn tick(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.1);
        let zero_un = self.rendu.profondeur_zero_un();
        let camera_table = scene::camera(self.aspect, zero_un);
        let ouverture = table::disposition::ouverture(&camera_table);
        let mut instances = self.decor.instances(dt, &self.textures);
        let (table, selections) = self.table.instances(dt, self.survol, &self.textures, ouverture);
        instances.extend(table);
        instances.sort_by(|a, b| a.position_lacet[1].total_cmp(&b.position_lacet[1]));
        instances.insert(0, scene::tapis(self.theme));
        let (vue_proj, oeil) = if self.table.est_definie() {
            (camera_table, scene::oeil(self.aspect))
        } else {
            (scene::camera_decor(self.aspect, zero_un, &self.decor.vue()), scene::oeil_decor(self.aspect, &self.decor.vue()))
        };
        self.rendu.dessiner(&vue_proj, oeil, &instances, &selections);
    }

    pub fn preparer(&mut self) {}

    pub fn rendre(&mut self) {
        self.tick(0.0);
    }

    pub fn mettre_a_jour_decor(&mut self, dt: f32) {
        self.tick(dt);
    }
}
