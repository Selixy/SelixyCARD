use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash};

pub async fn hacher(mot_de_passe: String) -> Result<String, ()> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(mot_de_passe.as_bytes())
            .map(|empreinte| empreinte.to_string())
            .map_err(|_| ())
    })
    .await
    .map_err(|_| ())?
}

pub async fn verifier(mot_de_passe: String, empreinte: Option<String>) -> bool {
    tokio::task::spawn_blocking(move || match empreinte {
        Some(texte) => PasswordHash::new(&texte)
            .map(|analyse| Argon2::default().verify_password(mot_de_passe.as_bytes(), &analyse).is_ok())
            .unwrap_or(false),
        None => {
            let _ = Argon2::default().hash_password(mot_de_passe.as_bytes());
            false
        }
    })
    .await
    .unwrap_or(false)
}
