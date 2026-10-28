mod aide;

use aide::{creer, inscrire, json};
use serde_json::json;

#[tokio::test]
async fn creation_donne_108_cartes_sans_image() {
    let app = aide::app().await;
    let (pseudo, cookie) = inscrire(&app).await;
    let set = creer(&app, &cookie).await;
    let cartes = set["cartes"].as_array().unwrap();
    assert_eq!(cartes.len(), 108);
    assert!(cartes.iter().all(|c| c["image"].is_null()));
    assert_eq!(set["auteur"], pseudo);
    assert_eq!(cartes[0], json!({"id": "rouge-0", "type": "nombre", "couleur": "rouge", "valeur": 0, "image": null}));
    assert_eq!(cartes[107]["id"], "joker_plus4-4");
}

#[tokio::test]
async fn creation_exige_une_session() {
    let app = aide::app().await;
    let reponse = json(&app, "POST", "/api/sets", None, json!({"nom": "x"})).await;
    assert_eq!(reponse.statut, 401);
}

#[tokio::test]
async fn liste_et_lecture_reservees_a_l_auteur() {
    let app = aide::app().await;
    let (_, auteur) = inscrire(&app).await;
    let (_, autre) = inscrire(&app).await;
    let id = creer(&app, &auteur).await["id"].as_str().unwrap().to_string();
    let chemin = format!("/api/sets/{id}");
    assert_eq!(json(&app, "GET", "/api/sets", None, json!(null)).await.statut, 401);
    assert_eq!(json(&app, "GET", &chemin, None, json!(null)).await.statut, 401);
    let liste_auteur = json(&app, "GET", "/api/sets", Some(&auteur), json!(null)).await.json();
    let trouve = liste_auteur.as_array().unwrap().iter().find(|s| s["id"] == id.as_str()).unwrap();
    assert_eq!(trouve["nb_illustrees"], 0);
    let lu = json(&app, "GET", &chemin, Some(&auteur), json!(null)).await;
    assert_eq!(lu.json()["cartes"].as_array().unwrap().len(), 108);
    let liste_autre = json(&app, "GET", "/api/sets", Some(&autre), json!(null)).await.json();
    assert!(liste_autre.as_array().unwrap().is_empty());
    assert_eq!(json(&app, "GET", &chemin, Some(&autre), json!(null)).await.statut, 403);
}

#[tokio::test]
async fn seul_l_auteur_modifie_ou_supprime() {
    let app = aide::app().await;
    let (_, auteur) = inscrire(&app).await;
    let (_, autre) = inscrire(&app).await;
    let set = creer(&app, &auteur).await;
    let chemin = format!("/api/sets/{}", set["id"].as_str().unwrap());
    let modif = json(&app, "PUT", &chemin, Some(&autre), set.clone()).await;
    let suppr = json(&app, "DELETE", &chemin, Some(&autre), json!(null)).await;
    assert_eq!((modif.statut, suppr.statut), (403, 403));
    assert_eq!(json(&app, "DELETE", &chemin, Some(&auteur), json!(null)).await.statut, 204);
    assert_eq!(json(&app, "GET", &chemin, Some(&auteur), json!(null)).await.statut, 404);
}

#[tokio::test]
async fn modification_avec_image_et_validation() {
    let app = aide::app().await;
    let (_, cookie) = inscrire(&app).await;
    let mut set = creer(&app, &cookie).await;
    let chemin = format!("/api/sets/{}", set["id"].as_str().unwrap());
    set["cartes"][0]["image"] = json!("a".repeat(64));
    assert_eq!(json(&app, "PUT", &chemin, Some(&cookie), set.clone()).await.statut, 400);
    let mut webp = b"RIFF\x04\0\0\0WEBPvp8 ".to_vec();
    webp.extend_from_slice(b"donnees");
    let depot = aide::requete(&app, "POST", "/api/images", Some(&cookie), webp, "image/webp").await;
    set["cartes"][0]["image"] = depot.json()["image"].clone();
    set["nom"] = json!("Renommé");
    let ok = json(&app, "PUT", &chemin, Some(&cookie), set.clone()).await;
    assert_eq!(ok.statut, 200);
    assert_eq!(ok.json()["cartes"][0]["image"], set["cartes"][0]["image"]);
    set["cartes"].as_array_mut().unwrap().pop();
    assert_eq!(json(&app, "PUT", &chemin, Some(&cookie), set).await.statut, 400);
}
