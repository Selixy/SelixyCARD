use moteur::table::souffle::Souffles;

#[test]
fn un_joker_laisse_un_depot_qui_reste() {
    let mut joker = Souffles::default();
    joker.impact_joker([1.4, 0.1, 0.0], [0.23, 0.39, 0.72], &[3.0, 4.0]);
    let pendant = joker.instances(0.1).len();
    let apres = (0..30).map(|_| joker.instances(0.1).len()).last().unwrap();
    assert!(apres > 0 && apres < pendant);
    let mut normale = Souffles::default();
    normale.impact([1.4, 0.1, 0.0], 1.0, [0.85, 0.28, 0.18]);
    assert_eq!((0..30).map(|_| normale.instances(0.1).len()).last().unwrap(), 0);
}
