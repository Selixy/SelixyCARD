pub mod matrice;
pub mod vecteurs;

pub use matrice::{identite, inverser, mise_a_echelle, rotation_euler, translation};
use vecteurs::{normaliser, scalaire, sous, vectoriel};

pub type Mat4 = [f32; 16];

pub fn multiplier(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut r = [0.0; 16];
    for c in 0..4 {
        for l in 0..4 {
            r[c * 4 + l] = (0..4).map(|k| a[k * 4 + l] * b[c * 4 + k]).sum();
        }
    }
    r
}

pub fn perspective(fov_y: f32, aspect: f32, pres: f32, loin: f32, zero_un: bool) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    let (a, b) = if zero_un {
        (loin / (pres - loin), pres * loin / (pres - loin))
    } else {
        ((loin + pres) / (pres - loin), 2.0 * pres * loin / (pres - loin))
    };
    [f / aspect, 0.0, 0.0, 0.0, 0.0, f, 0.0, 0.0, 0.0, 0.0, a, -1.0, 0.0, 0.0, b, 0.0]
}

pub fn fenetre(g: f32, d: f32, b: f32, h: f32, pres: f32, loin: f32, zero_un: bool) -> Mat4 {
    let (a, c) = if zero_un {
        (loin / (pres - loin), pres * loin / (pres - loin))
    } else {
        ((loin + pres) / (pres - loin), 2.0 * pres * loin / (pres - loin))
    };
    [
        2.0 * pres / (d - g), 0.0, 0.0, 0.0,
        0.0, 2.0 * pres / (h - b), 0.0, 0.0,
        (d + g) / (d - g), (h + b) / (h - b), a, -1.0,
        0.0, 0.0, c, 0.0,
    ]
}

pub fn transformer(m: &Mat4, p: [f32; 3]) -> [f32; 4] {
    std::array::from_fn(|l| m[l] * p[0] + m[4 + l] * p[1] + m[8 + l] * p[2] + m[12 + l])
}

pub fn regarder(oeil: [f32; 3], cible: [f32; 3], haut: [f32; 3]) -> Mat4 {
    let f = normaliser(sous(cible, oeil));
    let s = normaliser(vectoriel(f, haut));
    let u = vectoriel(s, f);
    [
        s[0], u[0], -f[0], 0.0,
        s[1], u[1], -f[1], 0.0,
        s[2], u[2], -f[2], 0.0,
        -scalaire(s, oeil), -scalaire(u, oeil), scalaire(f, oeil), 1.0,
    ]
}
