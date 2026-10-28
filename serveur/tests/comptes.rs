mod aide;

use aide::{inscrire, json};
use serde_json::json;

#[tokio::test]
async fn inscription_ouvre_la_session() {
    let app = aide::app().await;
    let (pseudo, cookie) = inscrire(&app).await;
    let moi = json(&app, "GET", "/api/moi", Some(&cookie), json!(null)).await;
    assert_eq!(moi.statut, 200);
    assert_eq!(moi.json()["pseudo"], pseudo);
}

#[tokio::test]
async fn pseudo_deja_pris_sans_tenir_compte_de_la_casse() {
    let app = aide::app().await;
    let (pseudo, _) = inscrire(&app).await;
    let corps = json!({"pseudo": pseudo.to_uppercase(), "mot_de_passe": "motdepasse1"});
    let reponse = json(&app, "POST", "/api/comptes", None, corps).await;
    assert_eq!(reponse.statut, 409);
    assert!(reponse.json()["erreur"].as_str().unwrap().contains("pris"));
}

#[tokio::test]
async fn entrees_invalides_refusees() {
    let app = aide::app().await;
    let court = json(&app, "POST", "/api/comptes", None, json!({"pseudo": "ab", "mot_de_passe": "motdepasse1"})).await;
    let faible = json(&app, "POST", "/api/comptes", None, json!({"pseudo": "abcdef", "mot_de_passe": "court"})).await;
    assert_eq!((court.statut, faible.statut), (400, 400));
}

#[tokio::test]
async fn connexion_et_deconnexion() {
    let app = aide::app().await;
    let (pseudo, ancien) = inscrire(&app).await;
    let mauvais = json(&app, "POST", "/api/session", None, json!({"pseudo": pseudo, "mot_de_passe": "faux"})).await;
    assert_eq!(mauvais.statut, 401);
    let bon = json(&app, "POST", "/api/session", None, json!({"pseudo": pseudo, "mot_de_passe": "motdepasse1"})).await;
    let cookie = bon.cookie().unwrap();
    let fin = json(&app, "DELETE", "/api/session", Some(&cookie), json!(null)).await;
    assert_eq!(fin.statut, 204);
    let apres = json(&app, "GET", "/api/moi", Some(&cookie), json!(null)).await;
    let autre = json(&app, "GET", "/api/moi", Some(&ancien), json!(null)).await;
    assert_eq!((apres.statut, autre.statut), (401, 200));
}

#[tokio::test]
async fn moi_sans_session_donne_401() {
    let app = aide::app().await;
    let reponse = json(&app, "GET", "/api/moi", None, json!(null)).await;
    assert_eq!(reponse.statut, 401);
    assert_eq!(reponse.json()["erreur"], "Connexion requise.");
}
