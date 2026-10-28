#![allow(dead_code)]
pub mod reponse;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, header};
use http_body_util::BodyExt;
use reponse::Reponse;
use serde_json::Value;
use tower::ServiceExt;

pub async fn app() -> Router {
    let url = std::env::var("DATABASE_URL_TEST")
        .unwrap_or_else(|_| "postgres://postgres:test@127.0.0.1:55433/postgres".into());
    let bdd = serveur::bdd::connexion::connecter(&url).await.expect("base de test");
    serveur::api::routeur::routeur(bdd)
}

pub async fn requete(app: &Router, methode: &str, chemin: &str, cookie: Option<&str>, corps: Vec<u8>, genre: &str) -> Reponse {
    let mut constructeur = Request::builder().method(methode).uri(chemin).header(header::CONTENT_TYPE, genre);
    if let Some(valeur) = cookie {
        constructeur = constructeur.header(header::COOKIE, valeur);
    }
    let reponse = app.clone().oneshot(constructeur.body(Body::from(corps)).unwrap()).await.unwrap();
    let (parties, corps) = reponse.into_parts();
    let octets = corps.collect().await.unwrap().to_bytes().to_vec();
    Reponse { statut: parties.status.as_u16(), entetes: parties.headers, octets }
}

pub async fn json(app: &Router, methode: &str, chemin: &str, cookie: Option<&str>, corps: Value) -> Reponse {
    requete(app, methode, chemin, cookie, corps.to_string().into_bytes(), "application/json").await
}

pub async fn creer(app: &Router, cookie: &str) -> Value {
    let reponse = json(app, "POST", "/api/sets", Some(cookie), serde_json::json!({"nom": "Mon set"})).await;
    assert_eq!(reponse.statut, 201);
    reponse.json()
}

pub async fn inscrire(app: &Router) -> (String, String) {
    let pseudo = format!("u{}", &uuid::Uuid::new_v4().simple().to_string()[..12]);
    let corps = serde_json::json!({"pseudo": pseudo, "mot_de_passe": "motdepasse1"});
    let reponse = json(app, "POST", "/api/comptes", None, corps).await;
    assert_eq!(reponse.statut, 200);
    (pseudo, reponse.cookie().expect("cookie de session"))
}
