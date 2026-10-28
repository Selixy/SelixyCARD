use moteur::table::etat::EtatTable;

#[test]
fn json_de_table_analyse() {
    let source = r#"{"main":["a","defaut:rouge-7-a","c"],"defausse":"d","adversaires":[7,3],"pioche":40}"#;
    let etat = EtatTable::depuis_json(source).unwrap();
    assert_eq!(etat.main, ["a", "defaut:rouge-7-a", "c"]);
    assert_eq!(etat.defausse.as_deref(), Some("d"));
    assert_eq!(etat.adversaires, [7, 3]);
    assert_eq!(etat.pioche, 40);
}

#[test]
fn json_tolerant_et_strict() {
    let vide = r#"{"main":[],"defausse":null,"adversaires":[],"pioche":0}"#;
    assert!(EtatTable::depuis_json(vide).unwrap().defausse.is_none());
    assert!(EtatTable::depuis_json(r#"{"main":["a""#).is_none());
    assert!(EtatTable::depuis_json("pas du json").is_none());
}
