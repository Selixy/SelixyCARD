use moteur::table::lueurs::{teinte, teinte_carte};

#[test]
fn le_souffle_prend_la_couleur_de_la_carte_ou_celle_choisie_pour_un_joker() {
    assert_eq!(teinte_carte("defaut:vert-7-a", None), teinte(Some("vert")).unwrap());
    assert_eq!(teinte_carte("abc123:rouge-plus2-b", Some("bleu")), teinte(Some("rouge")).unwrap());
    assert_eq!(teinte_carte("defaut:joker-1", Some("bleu")), teinte(Some("bleu")).unwrap());
}
