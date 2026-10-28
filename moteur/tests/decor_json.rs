use moteur::decor::etat::Decor;

#[test]
fn json_de_decor_analyse() {
    let source = r#"{"vise":[1.5,-2],"distance":18,"elevation":0.9,"elements":[{"cle":"cahier","x":-3,"z":1,"l":7,"h":5,"feuille":1,"flotte":0.1},{"cle":"defaut:rouge-7-a"},{"cle":"craie","feuille":1,"flotte":0,"y":0.005}]}"#;
    let decor = Decor::depuis_json(source).unwrap();
    assert_eq!(decor.vise, [1.5, -2.0]);
    assert_eq!(decor.elements.len(), 3);
    assert!(decor.elements[0].feuille);
    assert!(decor.elements[0].est_mobile());
    assert_eq!(decor.elements[0].taille, [7.0, 5.0]);
    assert!(!decor.elements[1].feuille);
    assert!(decor.elements[1].est_mobile());
    assert_eq!(decor.elements[1].taille, [1.5, 2.25]);
    assert!(decor.elements[2].feuille);
    assert!(!decor.elements[2].est_mobile());
    assert!(decor.elements[2].pos[1] >= 0.015);
}

#[test]
fn decor_sans_elements_ou_invalide() {
    assert!(Decor::depuis_json(r#"{"elements":[]}"#).is_some());
    assert!(Decor::depuis_json(r#"{"vise":[0,0]}"#).is_none());
    assert!(Decor::depuis_json("{").is_none());
}
