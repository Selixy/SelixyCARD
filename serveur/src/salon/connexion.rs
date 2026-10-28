use super::messages::Client;
use super::global::registre;
use super::vues;
use crate::api::erreur::ErreurApi;
use crate::session::courant::CompteCourant;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use tokio::sync::mpsc;

pub async fn accepter(ws: WebSocketUpgrade) -> Response {
    ws.max_message_size(4096).on_upgrade(|socket| boucle(socket, None))
}

pub async fn accepter_connecte(ws: WebSocketUpgrade, compte: Result<CompteCourant, ErreurApi>) -> Response {
    let pseudo = compte.ok().map(|c| c.pseudo);
    ws.max_message_size(4096).on_upgrade(|socket| boucle(socket, pseudo))
}

async fn boucle(mut socket: WebSocket, session: Option<String>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let conn = {
        let mut reg = registre();
        let conn = reg.connecter(tx);
        if let Some(pseudo) = session {
            reg.ouvrir_session(conn, pseudo);
        }
        conn
    };
    loop {
        tokio::select! {
            recu = socket.recv() => match recu {
                Some(Ok(Message::Text(texte))) => traiter(conn, &texte),
                Some(Ok(Message::Binary(octets))) => if let Ok(texte) = std::str::from_utf8(&octets) {
                    traiter(conn, texte);
                },
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => break,
                Some(Ok(_)) => {}
            },
            sortant = rx.recv() => match sortant {
                Some(texte) => if socket.send(Message::Text(texte.into())).await.is_err() { break },
                None => break,
            },
        }
    }
    registre().deconnecter(conn);
}

fn traiter(conn: u64, texte: &str) {
    let mut reg = registre();
    match serde_json::from_str::<Client>(texte) {
        Ok(message) => reg.traiter(conn, message),
        Err(_) => reg.envoyer(conn, vues::erreur("Message illisible.")),
    }
}
