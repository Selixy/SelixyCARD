use super::identite::verifier_hote;
use super::registre::Registre;
use super::vues;

impl Registre {
    pub fn diffuser(&self, code: &str, texte: String) {
        if let Some(salon) = self.salons.get(code) {
            salon.connexions().into_iter().for_each(|conn| self.envoyer(conn, texte.clone()));
        }
    }

    pub fn diffuser_partie(&self, code: &str) {
        let Some(salon) = self.salons.get(code) else { return };
        let Some(partie) = &salon.partie else { return };
        let etat = vues::etat(partie).to_string();
        let fin = vues::fin(partie).map(|v| v.to_string());
        for (i, membre) in salon.membres.iter().enumerate() {
            let Some(conn) = membre.conn else { continue };
            self.envoyer(conn, etat.clone());
            self.envoyer(conn, vues::main(partie, i).to_string());
            if let Some(fin) = &fin {
                self.envoyer(conn, fin.clone());
            }
        }
    }

    pub fn contexte(&self, conn: u64) -> Result<(String, String), &'static str> {
        let presence = self.presences.get(&conn).ok_or("Connexion inconnue.")?;
        match (&presence.code, &presence.pseudo) {
            (Some(code), Some(pseudo)) => Ok((code.clone(), pseudo.clone())),
            _ => Err("Tu n'es dans aucun salon."),
        }
    }

    pub fn code_hote(&self, conn: u64) -> Result<String, &'static str> {
        let (code, pseudo) = self.contexte(conn)?;
        let salon = self.salons.get(&code).ok_or("Salon introuvable.")?;
        verifier_hote(&salon.hote, &pseudo)?;
        if salon.partie.is_some() {
            return Err("La partie a déjà commencé.");
        }
        Ok(code)
    }
}
