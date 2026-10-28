use serveur::sets::emplacements::emplacements;
use std::collections::HashSet;

#[test]
fn cent_huit_identifiants_uniques() {
    let cartes = emplacements();
    let ids: HashSet<&String> = cartes.iter().map(|c| &c.id).collect();
    assert_eq!(cartes.len(), 108);
    assert_eq!(ids.len(), 108);
}

#[test]
fn vingt_cinq_cartes_par_couleur_et_huit_jokers() {
    let cartes = emplacements();
    let par = |couleur: &str| cartes.iter().filter(|c| c.couleur.as_deref() == Some(couleur)).count();
    assert_eq!([par("rouge"), par("jaune"), par("vert"), par("bleu")], [25; 4]);
    assert_eq!(cartes.iter().filter(|c| c.couleur.is_none()).count(), 8);
}

#[test]
fn identifiants_du_contrat_presents() {
    let ids: HashSet<String> = emplacements().into_iter().map(|c| c.id).collect();
    for attendu in ["bleu-0", "vert-9-b", "jaune-passer-a", "rouge-inverser-b", "bleu-plus2-a", "joker-3", "joker_plus4-1"] {
        assert!(ids.contains(attendu), "{attendu} manquant");
    }
}
