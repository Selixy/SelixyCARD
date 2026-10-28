use graphe_pbr::materiaux::{fabriquer_graphe_ardoise, fabriquer_graphe_carte, fabriquer_graphe_kraft};
use graphe_pbr::{deserialiser_graphe, emettre_fonction_surface_glsl, emettre_fonction_surface_wgsl, serialiser_graphe, GraphePbr, IdNoeud, Operation, SurfacePbr, TypeDonnee};

#[test]
fn graphe_carte_produit_wgsl_et_glsl() {
    let graphe = fabriquer_graphe_carte();
    let wgsl = emettre_fonction_surface_wgsl(&graphe).expect("emission wgsl valide");
    let glsl = emettre_fonction_surface_glsl(&graphe).expect("emission glsl valide");
    assert!(wgsl.contains("fn evaluer_surface"));
    assert!(wgsl.contains("textureSampleLevel"));
    assert!(glsl.contains("ProprietesSurface evaluer_surface"));
    assert!(glsl.contains("textureLod"));
}

#[test]
fn graphe_ardoise_produit_wgsl_et_glsl() {
    let graphe = fabriquer_graphe_ardoise();
    let wgsl = emettre_fonction_surface_wgsl(&graphe).expect("emission wgsl valide");
    let glsl = emettre_fonction_surface_glsl(&graphe).expect("emission glsl valide");
    assert!(wgsl.contains("bruit"));
    assert!(glsl.contains("bruit"));
}

#[test]
fn graphe_kraft_produit_wgsl_et_glsl() {
    let graphe = fabriquer_graphe_kraft();
    let wgsl = emettre_fonction_surface_wgsl(&graphe).expect("emission wgsl valide");
    let glsl = emettre_fonction_surface_glsl(&graphe).expect("emission glsl valide");
    assert!(wgsl.contains("mix"));
    assert!(glsl.contains("mix"));
}

#[test]
fn cycle_dans_graphe_signale_erreur() {
    let mut g = GraphePbr::nouveau(SurfacePbr {
        albedo: IdNoeud(0),
        rugosite: IdNoeud(0),
        metallique: IdNoeud(0),
        normale: None,
        relief: None,
        emission: None,
        alpha: None,
    });
    g.ajouter(Operation::Addition(IdNoeud(1), IdNoeud(0)), TypeDonnee::Flottant);
    g.ajouter(Operation::Addition(IdNoeud(0), IdNoeud(1)), TypeDonnee::Flottant);
    assert!(g.tri_topologique().is_err());
}

#[test]
fn serialisation_deserialisation_json_identique() {
    let graphe = fabriquer_graphe_carte();
    let json = serialiser_graphe(&graphe).expect("serialisation json valide");
    let recons: GraphePbr = deserialiser_graphe(&json).expect("deserialisation json valide");
    let wgsl1 = emettre_fonction_surface_wgsl(&graphe).unwrap();
    let wgsl2 = emettre_fonction_surface_wgsl(&recons).unwrap();
    assert_eq!(wgsl1, wgsl2);
}
