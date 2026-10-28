use super::cartes::{Carte, Couleur, TypeCarte};
use super::erreur::Erreur;
use super::etat::{Attente, Joueur, Partie};
use super::melange::Alea;
use super::options::Options;
use super::paquet::paquet;

impl Partie {
    pub fn nouvelle(pseudos: &[String], set: &[Carte], options: Options, graine: u64) -> Result<Partie, Erreur> {
        let distincts = pseudos.iter().enumerate().all(|(i, p)| !p.is_empty() && !pseudos[..i].contains(p));
        if !(2..=10).contains(&pseudos.len()) || !distincts {
            return Err(Erreur::JoueursInvalides);
        }
        let mut alea = Alea::nouvelle(graine);
        let mut pioche = paquet(set);
        alea.melanger(&mut pioche);
        let joueurs = pseudos
            .iter()
            .map(|p| Joueur { pseudo: p.clone(), main: pioche.split_off(pioche.len() - 7), uno: false })
            .collect();
        let mut partie = Partie {
            options,
            joueurs,
            pioche,
            defausse: Vec::new(),
            couleur: Couleur::Rouge,
            sens: 1,
            courant: 0,
            penalite: 0,
            attente: Attente::Aucune,
            signalable: None,
            piochee: None,
            gagnant: None,
            alea,
        };
        partie.retourner_premiere();
        partie.appliquer_premiere();
        Ok(partie)
    }

    fn retourner_premiere(&mut self) {
        loop {
            let carte = self.pioche.pop().expect("la pioche contient au moins une carte");
            if carte.genre == TypeCarte::JokerPlus4 {
                self.pioche.insert(0, carte);
                continue;
            }
            self.couleur = match carte.couleur {
                Some(couleur) => couleur,
                None => Couleur::TOUTES[self.alea.borne(4)],
            };
            self.defausse.push(carte);
            return;
        }
    }

    fn appliquer_premiere(&mut self) {
        let genre = self.haut().genre;
        match genre {
            TypeCarte::Passer => self.courant = 1,
            TypeCarte::Inverser => {
                self.sens = -1;
                if self.joueurs.len() == 2 {
                    self.courant = 1;
                }
            }
            TypeCarte::Plus2 => {
                self.tirer(0, 2);
                self.courant = 1;
            }
            _ => {}
        }
    }
}
