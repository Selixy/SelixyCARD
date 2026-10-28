pub mod compose;
pub mod depot;
pub mod disposition;
pub mod etat;
pub mod eventail;
pub mod lueurs;
pub mod mouvement;
pub mod origines;
pub mod pointage;
pub mod selections;
pub mod souffle;
pub mod trajet;

use crate::scene::{self, COUCHE_ABSENTE, Instance, Selection};
use crate::textures::Textures;
use disposition::Face;
use etat::EtatTable;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Table {
    etat: EtatTable,
    mouvements: mouvement::Mouvements,
    souffles: souffle::Souffles,
    compteur_defausse: u32,
    defausses: Vec<(u32, String)>,
    definie: bool,
    retrait: f32,
}

impl Table {
    pub fn definir(&mut self, etat: EtatTable) {
        if etat.defausse != self.etat.defausse {
            self.compteur_defausse += 1;
        }
        origines::preparer(&self.etat, &etat, 3000 + self.compteur_defausse, &mut self.mouvements, &mut self.souffles);
        if etat.defausse != self.etat.defausse
            && let Some(cle) = &etat.defausse
        {
            self.defausses.push((self.compteur_defausse, cle.clone()));
            let trop = self.defausses.len().saturating_sub(disposition::PILE_DEFAUSSE);
            self.defausses.drain(..trop);
        }
        self.etat = etat;
        self.definie = true;
    }

    pub fn est_definie(&self) -> bool {
        self.definie
    }

    pub fn etat(&self) -> &EtatTable {
        &self.etat
    }

    pub fn instances(&mut self, dt: f32, survol: i32, textures: &Textures, ouverture: f32) -> (Vec<Instance>, Vec<Selection>) {
        let attendu = if self.etat.courant == 0 { 0.0 } else { 1.0 };
        self.retrait += (attendu - self.retrait) * (1.0 - (-4.0 * dt).exp());
        let cibles = compose::cibles(&self.etat, survol, &self.defausses, self.retrait, ouverture);
        self.mouvements.retenir(|id| cibles.iter().any(|c| c.id == id));
        let mut sortie = Vec::new();
        let mut selections: BTreeMap<u32, Selection> = BTreeMap::new();

        for cible in &cibles {
            let (pose, atterrie) = self.mouvements.suivre(cible.id, &cible.pose, dt, matches!(cible.face, Face::Main(_)));
            if atterrie && cible.face == Face::Defausse(0) {
                let posee = self.defausses.last().map_or("", |(_, c)| c.as_str());
                let teinte = lueurs::teinte_carte(posee, self.etat.couleur.as_deref());
                if posee.contains("joker") {
                    let couches: Vec<f32> = (0..4).filter_map(|k| textures.couche(&format!("poudre:{k}"))).map(|c| c as f32).collect();
                    self.souffles.impact_joker(pose.pos, teinte, &couches);
                } else {
                    self.souffles.impact(pose.pos, 1.0, teinte);
                }
            }
            let pose = &pose;
            if cible.face == Face::Bloc {
                let dos = textures.couche("dos").map_or(COUCHE_ABSENTE, |c| c as f32);
                let paroi = [0.0, 0.0, 0.0, scene::MODE_CARTE];
                sortie.extend([scene::bloc::bloc(pose, paroi), scene::tranche(pose, cible.echelle), scene::carte(pose, cible.echelle, dos)]);
                selections::marquer(&mut selections, &self.etat, cible, pose, survol);
                continue;
            }
            let cle = match cible.face {
                Face::Main(i) => Some(self.etat.main[i].as_str()),
                Face::Defausse(rang) => self.defausses.iter().rev().nth(rang).map(|(_, c)| c.as_str()),
                Face::Dos | Face::Bloc => Some("dos"),
            };
            let couche = cle.and_then(|c| textures.couche(c)).map_or(COUCHE_ABSENTE, |c| c as f32);
            sortie.extend([scene::tranche(pose, cible.echelle), scene::carte(pose, cible.echelle, couche)]);
            selections::marquer(&mut selections, &self.etat, cible, pose, survol);
        }
        sortie.extend(self.souffles.instances(dt));
        (sortie, selections.into_values().collect())
    }
}
