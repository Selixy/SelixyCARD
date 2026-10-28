use crate::maths::{self, Mat4};

#[derive(Clone, Copy)]
pub struct VueDecor {
    pub vise: [f32; 2],
    pub distance: f32,
    pub elevation: f32,
    pub decalage: [f32; 2],
}

const VISE_TABLE: [f32; 3] = [0.0, 0.0, 0.2];

pub fn oeil(aspect: f32) -> [f32; 3] {
    let portrait = (1.0 - aspect).clamp(0.0, 0.6);
    let distance = 15.0_f32.max(10.5 / aspect.max(0.1)) * (1.0 - 0.12 * portrait) * 0.82;
    let elevation = 0.64_f32 + 0.5 * portrait;
    [0.0, distance * elevation.sin(), VISE_TABLE[2] + distance * elevation.cos()]
}

pub fn camera(aspect: f32, zero_un: bool) -> Mat4 {
    let vue = maths::regarder(oeil(aspect), VISE_TABLE, [0.0, 1.0, 0.0]);
    maths::multiplier(&maths::perspective(0.8, aspect, 1.0, 90.0, zero_un), &vue)
}

pub fn oeil_decor(aspect: f32, vue: &VueDecor) -> [f32; 3] {
    let etroit = (1.1 / aspect.max(0.2)).clamp(1.0, 1.45);
    let distance = vue.distance * etroit;
    let [vx, vz] = vue.vise;
    let [dx, dy] = vue.decalage;
    let y = distance * vue.elevation.sin() + dy * 0.45;
    let z = vz + distance * vue.elevation.cos();
    [vx + dx * 0.65, y, z]
}

pub fn camera_decor(aspect: f32, zero_un: bool, vue: &VueDecor) -> Mat4 {
    let etroit = (1.1 / aspect.max(0.2)).clamp(1.0, 1.45);
    let distance = vue.distance * etroit;
    let [dx, dy] = vue.decalage;
    let demi_h = (0.4_f32).tan();
    let demi_l = demi_h * aspect;
    let shift_x = -dx * 0.65 / distance;
    let shift_y = dy * 0.45 / distance;
    let proj = maths::fenetre(-demi_l - shift_x, demi_l - shift_x, -demi_h - shift_y, demi_h - shift_y, 1.0, 120.0, zero_un);
    let regard = maths::regarder(oeil_decor(aspect, vue), [oeil_decor(aspect, vue)[0], 0.0, vue.vise[1]], [0.0, 1.0, 0.0]);
    maths::multiplier(&proj, &regard)
}
