use super::messages::Resultat;
use super::registre::Registre;
use super::vues;

impl Registre {
    pub fn verifier_libre(&self, conn: u64) -> Resultat {
        match self.presences.get(&conn) {
            Some(presence) if presence.code.is_some() => Err("Tu es déjà dans un salon."),
            Some(_) => Ok(()),
            None => Err("Connexion inconnue."),
        }
    }

    pub fn annoncer_salon(&self, code: &str) {
        if let Some(salon) = self.salons.get(code) {
            self.diffuser(code, vues::salon(salon).to_string());
        }
    }
}
