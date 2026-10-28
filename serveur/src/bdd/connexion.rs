use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

const ESSAIS: u32 = 15;

pub async fn connecter(url: &str) -> Result<PgPool, String> {
    let mut derniere = String::new();
    for essai in 1..=ESSAIS {
        match ouvrir(url).await {
            Ok(bdd) => {
                migrer(&bdd).await?;
                return Ok(bdd);
            }
            Err(erreur) => {
                derniere = erreur.to_string();
                eprintln!("base injoignable (essai {essai}/{ESSAIS}) : {derniere}");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Err(format!("connexion à la base impossible : {derniere}"))
}

async fn ouvrir(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await
}

pub async fn migrer(bdd: &PgPool) -> Result<(), String> {
    sqlx::migrate!("./migrations")
        .run(bdd)
        .await
        .map_err(|erreur| format!("migrations en échec : {erreur}"))
}
