use super::modele::Carte;

pub fn emplacements() -> Vec<Carte> {
    commun::uno::emplacements()
        .into_iter()
        .map(|c| {
            let valeur = serde_json::to_value(&c).ok();
            let champ = |nom: &str| valeur.as_ref().and_then(|v| v.get(nom)).cloned().unwrap_or_default();
            Carte {
                id: c.id,
                genre: champ("type").as_str().unwrap_or_default().to_string(),
                couleur: champ("couleur").as_str().map(str::to_string),
                valeur: champ("valeur").as_i64().map(|v| v as i32),
                image: None,
            }
        })
        .collect()
}
