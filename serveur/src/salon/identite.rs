use super::messages::Resultat;
use super::registre::Registre;

pub fn pseudo_de(demande: Option<&str>) -> Result<String, &'static str> {
    let pseudo = demande.unwrap_or("").trim();
    let invalide = pseudo.is_empty() || pseudo.chars().count() > 20 || pseudo.chars().any(char::is_control);
    if invalide { Err("Pseudo invalide : 1 à 20 caractères.") } else { Ok(pseudo.to_string()) }
}

pub fn verifier_hote(hote: &str, pseudo: &str) -> Resultat {
    if hote == pseudo { Ok(()) } else { Err("Seul l'hôte peut faire cela.") }
}

impl Registre {
    pub fn ouvrir_session(&mut self, conn: u64, pseudo: String) {
        if let Some(presence) = self.presences.get_mut(&conn) {
            presence.session = Some(pseudo);
        }
    }

    pub fn pseudo_retenu(&self, conn: u64, demande: Option<&str>) -> Result<String, &'static str> {
        match self.presences.get(&conn).and_then(|p| p.session.clone()) {
            Some(pseudo) => Ok(pseudo),
            None => pseudo_de(demande),
        }
    }
}
