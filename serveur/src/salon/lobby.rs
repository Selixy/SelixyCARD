use super::messages::Resultat;
use super::registre::Registre;
use super::salle::Salon;
use commun::uno::{Options, Partie};

impl Registre {
    pub fn creer(&mut self, conn: u64, set: Option<String>, pseudo: Option<String>) -> Resultat {
        let pseudo = self.pseudo_retenu(conn, pseudo.as_deref())?;
        self.verifier_libre(conn)?;
        let set = set.unwrap_or_default();
        let cartes = self.source.charger(&set).ok_or("Set introuvable.")?;
        self.purger();
        let code = self.nouveau_code();
        self.salons.insert(code.clone(), Salon::nouveau(code.clone(), pseudo.clone(), conn, set, cartes));
        self.attacher(conn, &code, &pseudo);
        self.annoncer_salon(&code);
        Ok(())
    }

    pub fn rejoindre(&mut self, conn: u64, code: &str, pseudo: Option<String>) -> Resultat {
        let pseudo = self.pseudo_retenu(conn, pseudo.as_deref())?;
        self.verifier_libre(conn)?;
        let code = code.trim().to_uppercase();
        let salon = self.salons.get_mut(&code).ok_or("Salon introuvable.")?;
        salon.admettre(pseudo.clone(), conn)?;
        self.attacher(conn, &code, &pseudo);
        self.annoncer_salon(&code);
        self.diffuser_partie(&code);
        Ok(())
    }

    pub fn regler_options(&mut self, conn: u64, options: Options) -> Resultat {
        let code = self.code_hote(conn)?;
        if let Some(salon) = self.salons.get_mut(&code) {
            salon.options = options;
        }
        self.annoncer_salon(&code);
        Ok(())
    }

    pub fn choisir_set(&mut self, conn: u64, set: String) -> Resultat {
        let code = self.code_hote(conn)?;
        let cartes = self.source.charger(&set).ok_or("Set introuvable.")?;
        if let Some(salon) = self.salons.get_mut(&code) {
            salon.set = set;
            salon.cartes = cartes;
        }
        self.annoncer_salon(&code);
        Ok(())
    }

    pub fn lancer(&mut self, conn: u64) -> Resultat {
        let code = self.code_hote(conn)?;
        let graine = self.alea.suivant();
        let salon = self.salons.get_mut(&code).ok_or("Salon introuvable.")?;
        let pseudos: Vec<String> = salon.membres.iter().map(|m| m.pseudo.clone()).collect();
        if pseudos.len() < 2 {
            return Err("Il faut au moins 2 joueurs pour lancer.");
        }
        let partie = Partie::nouvelle(&pseudos, &salon.cartes, salon.options, graine).map_err(|e| e.message())?;
        salon.partie = Some(partie);
        self.diffuser_partie(&code);
        Ok(())
    }
}
