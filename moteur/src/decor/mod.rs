pub mod apparition;
pub mod etat;

use crate::scene::{Instance, VueDecor};
use crate::textures::Textures;
use etat::Decor;

#[derive(Default)]
pub struct Animation {
    decor: Option<Decor>,
    temps: f32,
    apparition: f32,
    cible: [f32; 2],
    decalage: [f32; 2],
    vise: [f32; 2],
    distance: f32,
    elevation: f32,
}

impl Animation {
    pub fn definir(&mut self, decor: Decor) {
        if self.decor.is_none() {
            self.vise = decor.vise;
            self.distance = decor.distance;
            self.elevation = decor.elevation;
            self.apparition = 0.0;
        }
        self.decor = Some(decor);
    }

    pub fn reinitialiser_apparition(&mut self) {
        self.apparition = 0.0;
    }

    pub fn pointeur(&mut self, x: f32, y: f32) {
        self.cible = [x.clamp(-1.0, 1.0), y.clamp(-1.0, 1.0)];
    }

    pub fn vue(&self) -> VueDecor {
        VueDecor { vise: self.vise, distance: self.distance.max(4.0), elevation: self.elevation, decalage: self.decalage }
    }

    pub fn instances(&mut self, dt: f32, textures: &Textures) -> Vec<Instance> {
        self.temps += dt;
        self.apparition += dt;
        let k = 1.0 - (-4.5 * dt).exp();
        self.decalage = std::array::from_fn(|i| self.decalage[i] + (self.cible[i] - self.decalage[i]) * k);
        let Some(decor) = &self.decor else { return Vec::new() };
        let k_cam = 1.0 - (-4.0 * dt).exp();
        self.vise = std::array::from_fn(|i| self.vise[i] + (decor.vise[i] - self.vise[i]) * k_cam);
        self.distance += (decor.distance - self.distance) * k_cam;
        self.elevation += (decor.elevation - self.elevation) * k_cam;
        let filtre = decor.elements.iter().any(|e| apparition::est_carte(e) && apparition::est_proche(&e.pos, &decor.vise));
        let mut rang_carte = 0;
        let (temps, apparition) = (self.temps, self.apparition);
        let mut sortie = Vec::with_capacity(decor.elements.len() * 2);
        for e in &decor.elements {
            let rang = if apparition::est_carte(e) && (!filtre || apparition::est_proche(&e.pos, &decor.vise)) {
                let r = rang_carte;
                rang_carte += 1;
                r
            } else {
                usize::MAX
            };
            apparition::pousser(e, temps, apparition, rang, textures, &mut sortie);
        }
        sortie
    }
}
