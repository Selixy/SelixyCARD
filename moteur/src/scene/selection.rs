use super::bloc::bloc;
use super::geometrie::Pose;
use super::{COUCHE_ABSENTE, HAUTEUR_CARTE, Instance, LARGEUR_CARTE, MODE_SILHOUETTE, instance};

pub const FORCE_MAX: f32 = 1.5;

pub struct Selection {
    pub couleur: [f32; 4],
    pub formes: Vec<Instance>,
}

impl Selection {
    pub fn nouvelle(couleur: [f32; 4]) -> Self {
        Self { couleur, formes: Vec::new() }
    }

    pub fn marque(&self) -> [f32; 4] {
        let force = self.couleur[3].clamp(0.0, FORCE_MAX);
        [self.couleur[0], self.couleur[1], self.couleur[2], MODE_SILHOUETTE + force]
    }

    pub fn ajouter_carte(&mut self, pose: &Pose, echelle: f32) {
        let taille = [LARGEUR_CARTE * echelle, HAUTEUR_CARTE * echelle];
        self.formes.push(instance(&pose.decaler(-0.018 * echelle), self.marque(), taille, COUCHE_ABSENTE));
        self.formes.push(instance(pose, self.marque(), taille, COUCHE_ABSENTE));
    }

    pub fn ajouter_bloc(&mut self, sommet: &Pose) {
        self.formes.push(bloc(sommet, self.marque()));
    }
}
