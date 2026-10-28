use super::disposition;
use super::trajet::{Trajet, distance};
use crate::scene::geometrie::Pose;
use std::collections::HashMap;

const SEUIL_TRAJET: f32 = 1.2;

#[derive(Default)]
pub struct Mouvements {
    poses: HashMap<u32, (Pose, Option<Trajet>)>,
}

impl Mouvements {
    pub fn apparaitre(&mut self, id: u32, pose: Pose) {
        self.poses.insert(id, (pose, None));
    }

    pub fn pose(&self, id: u32) -> Option<Pose> {
        self.poses.get(&id).map(|(p, _)| *p)
    }

    pub fn retenir(&mut self, garder: impl Fn(u32) -> bool) {
        self.poses.retain(|id, _| garder(*id));
    }

    pub fn suivre(&mut self, id: u32, cible: &Pose, dt: f32, rapide: bool) -> (Pose, bool) {
        let (pose, trajet) = self.poses.entry(id).or_insert_with(|| (disposition::depart(), None));
        match trajet {
            Some(t) => t.arrivee = *cible,
            None if distance(pose, cible) > SEUIL_TRAJET => *trajet = Some(Trajet::nouveau(*pose, *cible, rapide)),
            None => {}
        }
        let Some(t) = trajet else {
            *pose = pose.vers(cible, 1.0 - (-10.0 * dt).exp());
            return (*pose, false);
        };
        let (suivante, fini) = t.avancer(dt);
        *pose = suivante;
        if fini {
            *trajet = None;
        }
        (*pose, fini)
    }
}
