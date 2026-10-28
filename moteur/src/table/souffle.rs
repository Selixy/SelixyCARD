use super::depot;
use crate::scene::{self, Instance, geometrie::{PLAT, Pose}};

const DUREE: f32 = 1.0;
const GRAINS: f32 = 40.0;
const DEPOTS_MAX: usize = 800;
pub const CRAIE: [f32; 3] = [0.93, 0.93, 0.89];

#[derive(Default)]
pub struct Souffles {
    actifs: Vec<([f32; 3], f32, f32, [f32; 3], u32)>,
    depots: Vec<Instance>,
    graine: u32,
}

impl Souffles {
    pub fn impact(&mut self, pos: [f32; 3], force: f32, teinte: [f32; 3]) {
        self.graine = self.graine.wrapping_add(1);
        let graine = self.graine.wrapping_mul(2_654_435_761) ^ (pos[0].to_bits() >> 7);
        self.actifs.push((pos, 0.0, force, teinte, graine));
    }

    pub fn impact_joker(&mut self, pos: [f32; 3], teinte: [f32; 3], couches: &[f32]) {
        self.impact(pos, 1.7, teinte);
        let graine = self.actifs.last().map_or(0, |s| s.4);
        self.depots.extend(depot::traces_sol(pos, teinte, graine, couches));
        let trop = self.depots.len().saturating_sub(DEPOTS_MAX);
        self.depots.drain(..trop);
    }

    pub fn instances(&mut self, dt: f32) -> Vec<Instance> {
        self.actifs.iter_mut().for_each(|s| s.1 += dt / DUREE);
        self.actifs.retain(|s| s.1 < 1.0);
        let nuages = self.actifs.iter().flat_map(|&(pos, age, force, teinte, graine)| nuage(pos, age, force, teinte, graine));
        self.depots.iter().copied().chain(nuages).collect()
    }
}

pub fn alea(graine: u32, i: u32, k: u32) -> f32 {
    let mut h = graine ^ i.wrapping_mul(0x9e37_79b9) ^ k.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^= h >> 16;
    h as f32 / u32::MAX as f32
}

fn nuage(pos: [f32; 3], age: f32, force: f32, teinte: [f32; 3], graine: u32) -> Vec<Instance> {
    let flash = 3.0 * force;
    let mut sortie = vec![scene::halo(&Pose { pos: [pos[0], 0.006, pos[2]], lacet: 0.0, tangage: PLAT }, [flash, flash], [teinte[0], teinte[1], teinte[2], 0.55 * (1.0 - age).powi(4)])];
    for i in 0..(GRAINS * force.max(1.0)) as u32 {
        let r = |k| alea(graine, i, k);
        let vie = 0.45 + 0.55 * r(0);
        let a = (age / vie).min(1.0);
        if a >= 1.0 {
            continue;
        }
        let frein = 1.0 - (1.0 - a).powi(6);
        let angle = std::f32::consts::TAU * r(1);
        let portee = (0.8 + 1.9 * r(2)) * force * frein;
        let montee = if r(3) < 0.25 { 0.0 } else { (0.05 + 0.3 * r(4)) * force * frein };
        let taille = (0.45 + 0.75 * r(5)) * force * (1.0 + 1.8 * a);
        let eclat = 0.85 + 0.35 * r(6);
        let couleur = teinte.map(|c| (c * eclat).min(1.0));
        let apparition = (a * 12.0).min(1.0);
        let pose = Pose {
            pos: [pos[0] + portee * angle.cos(), 0.05 + montee, pos[2] + portee * angle.sin() * 0.8],
            lacet: 0.0,
            tangage: if montee > 0.0 { PLAT + 0.9 } else { PLAT },
        };
        let alpha = 0.5 * apparition * (1.0 - a).powf(1.5);
        sortie.push(scene::halo(&pose, [taille, taille * (0.8 + 0.4 * r(7))], [couleur[0], couleur[1], couleur[2], alpha]));
    }
    sortie
}
