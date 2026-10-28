use crate::scene::geometrie::Pose;

pub struct Trajet {
    pub depart: Pose,
    pub arrivee: Pose,
    t: f32,
    duree: f32,
    arc: f32,
    impact: bool,
}

impl Trajet {
    pub fn nouveau(depart: Pose, arrivee: Pose, rapide: bool) -> Self {
        let d = distance(&depart, &arrivee);
        if rapide {
            return Self { depart, arrivee, t: 0.0, duree: (0.2 + 0.03 * d).min(0.38), arc: (0.15 * d).min(1.0), impact: false };
        }
        Self { depart, arrivee, t: 0.0, duree: (0.26 + 0.035 * d).min(0.5), arc: (0.3 * d).min(2.4), impact: true }
    }

    pub fn avancer(&mut self, dt: f32) -> (Pose, bool) {
        self.t = (self.t + dt / self.duree).min(1.0);
        if self.t >= 1.0 {
            return (self.arrivee, true);
        }
        let (e, bosse) = courbes(self.t, self.impact);
        let m = |a: f32, b: f32, k: f32| a + (b - a) * k;
        let (a, b) = (&self.depart, &self.arrivee);
        let pose = Pose {
            pos: [
                m(a.pos[0], b.pos[0], e),
                m(a.pos[1], b.pos[1], e) + self.arc * bosse,
                m(a.pos[2], b.pos[2], e),
            ],
            lacet: m(a.lacet, b.lacet, e) + 0.35 * bosse,
            tangage: m(a.tangage, b.tangage, e),
        };
        (pose, false)
    }
}

fn courbes(p: f32, impact: bool) -> (f32, f32) {
    if !impact {
        return (p * p * (3.0 - 2.0 * p), (std::f32::consts::PI * p).sin());
    }
    if p < 0.5 {
        let monte = 1.0 - (1.0 - 2.0 * p).powi(2);
        return (0.35 * monte, monte);
    }
    let chute = (2.0 * p - 1.0).powi(3);
    (0.35 + 0.65 * chute, 1.0 - chute)
}

pub fn distance(a: &Pose, b: &Pose) -> f32 {
    (0..3).map(|i| (a.pos[i] - b.pos[i]).powi(2)).sum::<f32>().sqrt()
}
