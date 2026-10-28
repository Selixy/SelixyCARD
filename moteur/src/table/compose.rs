use super::disposition::{CARTES_PAR_PILE, Cible, Face, GLISSE_JOUABLE, GLISSE_SURVOL, pose_defausse, pose_main, pose_pioche};
use super::etat::EtatTable;
use super::eventail::{CARTES_VISIBLES_ADVERSAIRE, ECHELLE_ADVERSAIRE, pose_adversaire, pose_premier_plan, serrage};

pub fn cibles(etat: &EtatTable, survol: i32, defausses: &[(u32, String)], retrait: f32, ouverture: f32) -> Vec<Cible> {
    let total = etat.main.len();
    let mut sortie: Vec<Cible> = (0..total)
        .map(|i| {
            if etat.est_au_premier_plan(i) {
                return Cible { id: identifiant(&etat.main[i]), face: Face::Main(i), pose: pose_premier_plan(), echelle: 1.45 };
            }
            let mut pose = pose_main(i, total, ouverture / (1.0 - 0.25 * retrait));
            if etat.courant == 0 && etat.jouables.contains(&i) {
                pose = pose.glisser(if survol == i as i32 { GLISSE_SURVOL } else { GLISSE_JOUABLE });
            }
            let echelle = 1.0 - 0.25 * retrait;
            pose.pos[0] *= echelle;
            pose.pos[1] -= 0.45 * retrait;
            pose.pos[2] += 0.8 * retrait;
            Cible { id: identifiant(&etat.main[i]), face: Face::Main(i), pose, echelle }
        })
        .collect();
    if etat.pioche > 0 {
        sortie.push(Cible { id: 2000, face: Face::Bloc, pose: pose_pioche(etat.pioche.min(CARTES_PAR_PILE).saturating_sub(1)), echelle: 1.0 });
    }
    for (rang, (numero, _)) in defausses.iter().rev().enumerate() {
        let pose = pose_defausse(defausses.len() - 1 - rang, *numero);
        sortie.push(Cible { id: 3000 + numero, face: Face::Defausse(rang), pose, echelle: 1.0 });
    }
    sortie.extend(adversaires(&etat.adversaires, serrage(ouverture)));
    sortie
}

fn adversaires(nombres: &[u32], serre: f32) -> Vec<Cible> {
    let mut sortie = Vec::new();
    for (a, nb) in nombres.iter().enumerate() {
        let visibles = (*nb).min(CARTES_VISIBLES_ADVERSAIRE);
        for j in 0..visibles {
            let pose = pose_adversaire(a, nombres.len(), j, visibles, serre);
            sortie.push(Cible { id: 4000 + a as u32 * 100 + j, face: Face::Dos, pose, echelle: ECHELLE_ADVERSAIRE });
        }
    }
    sortie
}

pub fn identifiant(cle: &str) -> u32 {
    let empreinte = cle.bytes().fold(2_166_136_261_u32, |h, o| (h ^ o as u32).wrapping_mul(16_777_619));
    10_000 + empreinte % 1_000_000_000
}
