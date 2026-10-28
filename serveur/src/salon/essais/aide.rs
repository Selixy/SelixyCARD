use crate::salon::messages::Client;
use crate::salon::registre::Registre;
use crate::salon::sets::{SetParDefaut, SourceSets};
use commun::uno::Carte;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub struct SansSet;

impl SourceSets for SansSet {
    fn charger(&self, _id: &str) -> Option<Vec<Carte>> {
        None
    }
}

pub fn nouveau() -> Registre {
    Registre::nouveau(Arc::new(SetParDefaut), 7)
}

pub fn connecter(registre: &mut Registre) -> (u64, UnboundedReceiver<String>) {
    let (tx, rx) = mpsc::unbounded_channel();
    (registre.connecter(tx), rx)
}

pub fn envoyer(registre: &mut Registre, conn: u64, json: &str) {
    let message: Client = serde_json::from_str(json).expect("message client valide");
    registre.traiter(conn, message);
}

pub fn recus(rx: &mut UnboundedReceiver<String>) -> Vec<Value> {
    let mut messages = Vec::new();
    while let Ok(texte) = rx.try_recv() {
        messages.push(serde_json::from_str(&texte).expect("JSON serveur"));
    }
    messages
}
