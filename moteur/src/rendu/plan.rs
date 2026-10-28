use super::{MAX_INSTANCES, SOMMETS_BLOC, SOMMETS_PAR_CARTE};
use crate::scene::{COUCHE_BLOC, COUCHE_OMBRE, Instance, MODE_POUDRE, Selection};

#[derive(Clone, Copy, PartialEq)]
pub enum Etape {
    Scene,
    Voile,
    Selection,
}

pub struct Trace {
    pub etape: Etape,
    pub debut: u32,
    pub nombre: u32,
    pub sommets: u32,
}

fn est_voile(i: &Instance) -> bool {
    i.forme[3] == COUCHE_OMBRE || (MODE_POUDRE..MODE_POUDRE + 1.0).contains(&i.couleur[3])
}

fn est_bloc(i: &Instance) -> bool {
    i.forme[3] == COUCHE_BLOC
}

fn emettre(tout: &mut Vec<Instance>, traces: &mut Vec<Trace>, lot: Vec<Instance>, etape: Etape, sommets: u32) {
    let debut = tout.len();
    tout.extend(lot.into_iter().take(MAX_INSTANCES - debut));
    let nombre = (tout.len() - debut) as u32;
    if nombre > 0 {
        traces.push(Trace { etape, debut: debut as u32, nombre, sommets });
    }
}

fn pousser(tout: &mut Vec<Instance>, traces: &mut Vec<Trace>, lot: impl IntoIterator<Item = Instance>, etape: Etape) {
    let (blocs, autres): (Vec<Instance>, Vec<Instance>) = lot.into_iter().partition(est_bloc);
    emettre(tout, traces, autres, etape, SOMMETS_PAR_CARTE);
    emettre(tout, traces, blocs, etape, SOMMETS_BLOC);
}

pub fn planifier(instances: &[Instance], selections: &[Selection]) -> (Vec<Instance>, Vec<Trace>) {
    let mut tout = Vec::with_capacity(instances.len() + 64);
    let mut traces = Vec::new();
    pousser(&mut tout, &mut traces, instances.iter().filter(|i| !est_voile(i)).copied(), Etape::Scene);
    pousser(&mut tout, &mut traces, instances.iter().filter(|i| est_voile(i)).copied(), Etape::Voile);
    pousser(&mut tout, &mut traces, selections.iter().flat_map(|s| s.formes.iter().copied()), Etape::Selection);
    (tout, traces)
}
