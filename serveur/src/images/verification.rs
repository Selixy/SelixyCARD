use crate::api::erreur::ErreurApi;
use crate::session::jeton::hexa;
use sha2::{Digest, Sha256};

pub const TAILLE_MAX: usize = 2 * 1024 * 1024;

pub fn verifier(octets: &[u8]) -> Result<(), ErreurApi> {
    if octets.len() > TAILLE_MAX {
        return Err(ErreurApi::TropGros);
    }
    if octets.len() < 12 || &octets[0..4] != b"RIFF" || &octets[8..12] != b"WEBP" {
        return Err(ErreurApi::Invalide("L'image doit être au format WebP."));
    }
    Ok(())
}

pub fn sha256(octets: &[u8]) -> String {
    hexa(&Sha256::digest(octets))
}

pub fn est_sha256(texte: &str) -> bool {
    texte.len() == 64 && texte.bytes().all(|o| matches!(o, b'0'..=b'9' | b'a'..=b'f'))
}
