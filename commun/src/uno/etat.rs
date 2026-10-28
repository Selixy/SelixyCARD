use super::cartes::{Carte, Couleur};
use super::erreur::Erreur;
use super::melange::Alea;
use super::options::Options;

#[derive(Clone, Debug)]
pub struct Joueur {
    pub pseudo: String,
    pub main: Vec<Carte>,
    pub uno: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attente {
    Aucune,
    Contestation { poseur: usize, avait_couleur: bool },
}

#[derive(Clone, Debug)]
pub struct Partie {
    pub options: Options,
    pub joueurs: Vec<Joueur>,
    pub pioche: Vec<Carte>,
    pub defausse: Vec<Carte>,
    pub couleur: Couleur,
    pub sens: i32,
    pub courant: usize,
    pub penalite: u32,
    pub attente: Attente,
    pub signalable: Option<usize>,
    pub piochee: Option<String>,
    pub gagnant: Option<usize>,
    pub alea: Alea,
}

impl Partie {
    pub fn haut(&self) -> &Carte {
        self.defausse.last().expect("la défausse n'est jamais vide")
    }

    pub fn index(&self, pseudo: &str) -> Result<usize, Erreur> {
        self.joueurs.iter().position(|j| j.pseudo == pseudo).ok_or(Erreur::JoueurInconnu)
    }

    pub fn verifier_en_cours(&self) -> Result<(), Erreur> {
        if self.gagnant.is_some() { Err(Erreur::PartieTerminee) } else { Ok(()) }
    }
}
