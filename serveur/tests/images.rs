mod aide;

use aide::{inscrire, requete};
use sha2::{Digest, Sha256};

fn webp(taille: usize) -> Vec<u8> {
    let mut octets = b"RIFF\x04\0\0\0WEBPVP8 ".to_vec();
    octets.resize(taille, 7);
    octets
}

#[tokio::test]
async fn depot_puis_lecture_publique_avec_cache() {
    let app = aide::app().await;
    let (_, cookie) = inscrire(&app).await;
    let mut corps = webp(100);
    corps.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
    let attendu: String = Sha256::digest(&corps).iter().map(|o| format!("{o:02x}")).collect();
    let depot = requete(&app, "POST", "/api/images", Some(&cookie), corps.clone(), "image/webp").await;
    assert_eq!(depot.json()["image"], attendu.as_str());
    let lecture = requete(&app, "GET", &format!("/api/images/{attendu}.webp"), None, vec![], "x").await;
    assert_eq!(lecture.statut, 200);
    assert_eq!(lecture.octets, corps);
    let cache = lecture.entetes.get("cache-control").unwrap().to_str().unwrap();
    assert_eq!(cache, "public, max-age=31536000, immutable");
}

#[tokio::test]
async fn depot_sans_session_refuse() {
    let app = aide::app().await;
    let reponse = requete(&app, "POST", "/api/images", None, webp(50), "image/webp").await;
    assert_eq!(reponse.statut, 401);
}

#[tokio::test]
async fn format_et_taille_controles() {
    let app = aide::app().await;
    let (_, cookie) = inscrire(&app).await;
    let png = requete(&app, "POST", "/api/images", Some(&cookie), b"\x89PNG\r\n\x1a\n....".to_vec(), "image/webp").await;
    assert_eq!(png.statut, 400);
    let gros = requete(&app, "POST", "/api/images", Some(&cookie), webp(2 * 1024 * 1024 + 1), "image/webp").await;
    assert_eq!(gros.statut, 413);
    let enorme = requete(&app, "POST", "/api/images", Some(&cookie), webp(3 * 1024 * 1024), "image/webp").await;
    assert_eq!(enorme.statut, 413);
    let limite = requete(&app, "POST", "/api/images", Some(&cookie), webp(2 * 1024 * 1024), "image/webp").await;
    assert_eq!(limite.statut, 201);
}

#[tokio::test]
async fn image_inconnue_ou_mal_nommee() {
    let app = aide::app().await;
    let absente = requete(&app, "GET", &format!("/api/images/{}.webp", "0".repeat(64)), None, vec![], "x").await;
    let mauvais = requete(&app, "GET", "/api/images/pas-un-sha.webp", None, vec![], "x").await;
    assert_eq!((absente.statut, mauvais.statut), (404, 404));
}
