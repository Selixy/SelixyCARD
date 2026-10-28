use super::salle::Salon;
use super::sets::SourceSets;
use super::vues;
use commun::uno::Alea;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub struct Presence {
    pub pseudo: Option<String>,
    pub session: Option<String>,
    pub code: Option<String>,
    pub tx: UnboundedSender<String>,
}

pub struct Registre {
    pub salons: HashMap<String, Salon>,
    pub presences: HashMap<u64, Presence>,
    pub source: Arc<dyn SourceSets>,
    pub alea: Alea,
    pub prochain: u64,
}

impl Registre {
    pub fn nouveau(source: Arc<dyn SourceSets>, graine: u64) -> Registre {
        Registre { salons: HashMap::new(), presences: HashMap::new(), source, alea: Alea::nouvelle(graine), prochain: 1 }
    }

    pub fn connecter(&mut self, tx: UnboundedSender<String>) -> u64 {
        let conn = self.prochain;
        self.prochain += 1;
        self.presences.insert(conn, Presence { pseudo: None, session: None, code: None, tx });
        conn
    }

    pub fn deconnecter(&mut self, conn: u64) {
        let Some(code) = self.presences.remove(&conn).and_then(|p| p.code) else { return };
        let Some(salon) = self.salons.get_mut(&code) else { return };
        if salon.quitter(conn) {
            self.salons.remove(&code);
        } else if salon.partie.is_none() {
            let texte = vues::salon(salon).to_string();
            self.diffuser(&code, texte);
        }
    }

    pub fn envoyer(&self, conn: u64, texte: String) {
        if let Some(presence) = self.presences.get(&conn) {
            let _ = presence.tx.send(texte);
        }
    }

    pub fn attacher(&mut self, conn: u64, code: &str, pseudo: &str) {
        if let Some(presence) = self.presences.get_mut(&conn) {
            presence.code = Some(code.to_string());
            presence.pseudo = Some(pseudo.to_string());
        }
    }
}
