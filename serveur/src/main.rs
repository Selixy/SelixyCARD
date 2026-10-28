use axum::Router;
use axum::routing::get;
use serveur::{api, bdd, salon};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let dossier_ui = std::env::var("DOSSIER_UI").unwrap_or_else(|_| "ui".into());
    let app = match std::env::var("DATABASE_URL") {
        Ok(url) => {
            let pool = bdd::connexion::connecter(&url).await.expect("base de données injoignable");
            salon::installer_source(Arc::new(salon::SetsPostgres { bdd: pool.clone() }));
            api::routeur::routeur(pool)
        }
        Err(_) => Router::new().route("/ws", get(salon::accepter)).route("/ws/partie", get(salon::accepter)),
    };
    let app = app.route("/sante", get(|| async { "ok" })).fallback_service(ServeDir::new(dossier_ui));
    let adresse = format!("0.0.0.0:{port}");
    let ecoute = tokio::net::TcpListener::bind(&adresse).await.expect("port occupé");
    println!("SelixyCARD sur http://{adresse}");
    axum::serve(ecoute, app).await.expect("serveur arrêté");
}
