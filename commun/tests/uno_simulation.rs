use commun::uno::{Attente, Couleur, Options, Partie, Pose};

fn total(p: &Partie) -> usize {
    p.pioche.len() + p.defausse.len() + p.joueurs.iter().map(|j| j.main.len()).sum::<usize>()
}

fn un_coup(p: &mut Partie) {
    let idx = p.courant;
    let pseudo = p.joueurs[idx].pseudo.clone();
    if matches!(p.attente, Attente::Contestation { .. }) && p.alea.borne(2) == 0 {
        p.contester(&pseudo).unwrap();
        return;
    }
    let choix = p.joueurs[idx].main.iter().find(|c| p.peut_poser(idx, c) && p.piochee.as_ref().is_none_or(|id| *id == c.id)).cloned();
    let Some(carte) = choix else {
        p.piocher(&pseudo).unwrap();
        return;
    };
    let couleur = Some(Couleur::TOUTES[p.alea.borne(4)]);
    let cible = Some(p.joueurs[(idx + 1) % p.joueurs.len()].pseudo.clone());
    p.jouer(&pseudo, Pose { carte: carte.id, couleur, cible }).unwrap();
}

#[test]
fn parties_aleatoires_conservent_les_cartes() {
    let tout = Options { cumul_plus: true, sept_zero: true, jump_in: true, piocher_jusqua_jouable: true, penalite_uno: true, contester_plus4: true };
    let rien = Options { penalite_uno: false, contester_plus4: false, ..Options::default() };
    let mut terminees = 0;
    for options in [Options::default(), tout, rien] {
        for graine in 0..40 {
            let pseudos: Vec<String> = (0..2 + graine as usize % 4).map(|i| format!("j{i}")).collect();
            let mut p = Partie::nouvelle(&pseudos, &[], options, graine).unwrap();
            for _ in 0..3000 {
                if p.gagnant.is_some() {
                    terminees += 1;
                    break;
                }
                un_coup(&mut p);
                assert_eq!(total(&p), 108);
            }
        }
    }
    assert!(terminees > 60, "trop peu de parties terminées : {terminees}");
}
