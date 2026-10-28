use super::messages::{Client, Resultat};
use super::registre::Registre;
use super::vues;
use commun::uno::{Partie, Pose};

impl Registre {
    pub fn traiter(&mut self, conn: u64, message: Client) {
        let resultat = match message {
            Client::Creer { set, pseudo } => self.creer(conn, set, pseudo),
            Client::Rejoindre { code, pseudo } => self.rejoindre(conn, &code, pseudo),
            Client::Options { options } => self.regler_options(conn, options),
            Client::ChoisirSet { set } => self.choisir_set(conn, set),
            Client::Lancer => self.lancer(conn),
            Client::Jouer { carte, couleur, cible } => {
                self.agir(conn, |partie, pseudo| partie.jouer(pseudo, Pose { carte, couleur, cible }))
            }
            Client::Piocher => self.agir(conn, |partie, pseudo| partie.piocher(pseudo)),
            Client::Uno => self.agir(conn, |partie, pseudo| partie.dire_uno(pseudo)),
            Client::Signaler { pseudo: cible } => self.agir(conn, |partie, pseudo| partie.signaler(pseudo, &cible)),
            Client::Contester => self.agir(conn, |partie, pseudo| partie.contester(pseudo)),
        };
        if let Err(message) = resultat {
            self.envoyer(conn, vues::erreur(message));
        }
    }

    fn agir<F>(&mut self, conn: u64, coup: F) -> Resultat
    where
        F: FnOnce(&mut Partie, &str) -> Result<(), commun::uno::Erreur>,
    {
        let (code, pseudo) = self.contexte(conn)?;
        let salon = self.salons.get_mut(&code).ok_or("Salon introuvable.")?;
        let partie = salon.partie.as_mut().ok_or("La partie n'a pas commencé.")?;
        coup(partie, &pseudo).map_err(|e| e.message())?;
        self.diffuser_partie(&code);
        Ok(())
    }
}
