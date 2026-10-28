use crate::api::erreur::ErreurApi;

pub fn verifier_pseudo(pseudo: &str) -> Result<(), ErreurApi> {
    let longueur = pseudo.chars().count();
    if !(3..=24).contains(&longueur) {
        return Err(ErreurApi::Invalide("Le pseudo doit compter de 3 à 24 caractères."));
    }
    if !pseudo.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.')) {
        return Err(ErreurApi::Invalide("Le pseudo ne peut contenir que des lettres, des chiffres, _, - et ."));
    }
    Ok(())
}

pub fn verifier_mot_de_passe(mot_de_passe: &str) -> Result<(), ErreurApi> {
    if mot_de_passe.chars().count() < 8 {
        return Err(ErreurApi::Invalide("Le mot de passe doit compter au moins 8 caractères."));
    }
    if mot_de_passe.len() > 1024 {
        return Err(ErreurApi::Invalide("Le mot de passe est trop long."));
    }
    Ok(())
}
