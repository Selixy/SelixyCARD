use super::messages::Resultat;
use commun::uno::{Carte, Options, Partie};
use std::time::Instant;

pub const JOUEURS_MAX: usize = 10;

pub struct Membre {
    pub pseudo: String,
    pub conn: Option<u64>,
}

pub struct Salon {
    pub code: String,
    pub hote: String,
    pub membres: Vec<Membre>,
    pub options: Options,
    pub set: String,
    pub cartes: Vec<Carte>,
    pub partie: Option<Partie>,
    pub vide_depuis: Option<Instant>,
}

impl Salon {
    pub fn nouveau(code: String, hote: String, conn: u64, set: String, cartes: Vec<Carte>) -> Salon {
        let membres = vec![Membre { pseudo: hote.clone(), conn: Some(conn) }];
        Salon { code, hote, membres, options: Options::default(), set, cartes, partie: None, vide_depuis: None }
    }

    pub fn connexions(&self) -> Vec<u64> {
        self.membres.iter().filter_map(|m| m.conn).collect()
    }

    pub fn quitter(&mut self, conn: u64) -> bool {
        if self.partie.is_some() {
            self.membres.iter_mut().filter(|m| m.conn == Some(conn)).for_each(|m| m.conn = None);
        } else {
            self.membres.retain(|m| m.conn != Some(conn));
            if let Some(premier) = self.membres.first() {
                if !self.membres.iter().any(|m| m.pseudo == self.hote) {
                    self.hote = premier.pseudo.clone();
                }
            }
        }
        let vide = self.connexions().is_empty();
        self.vide_depuis = vide.then(Instant::now);
        vide && self.partie.is_none()
    }

    pub fn admettre(&mut self, pseudo: String, conn: u64) -> Resultat {
        let plein = self.membres.len() >= JOUEURS_MAX;
        let existant = self.membres.iter_mut().find(|m| m.pseudo.eq_ignore_ascii_case(&pseudo));
        match (existant, self.partie.is_some()) {
            (Some(m), true) => m.conn = Some(conn),
            (Some(m), false) if m.conn.is_none() => m.conn = Some(conn),
            (Some(_), false) => return Err("Ce pseudo est déjà pris dans ce salon."),
            (None, true) => return Err("La partie a déjà commencé."),
            (None, false) if plein => return Err("Le salon est plein."),
            (None, false) => self.membres.push(Membre { pseudo, conn: Some(conn) }),
        }
        self.vide_depuis = None;
        Ok(())
    }
}
