use super::{Mat4, multiplier};

pub fn identite() -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn translation(v: [f32; 3]) -> Mat4 {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        v[0], v[1], v[2], 1.0,
    ]
}

pub fn mise_a_echelle(s: [f32; 3]) -> Mat4 {
    [
        s[0], 0.0, 0.0, 0.0,
        0.0, s[1], 0.0, 0.0,
        0.0, 0.0, s[2], 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn rotation_euler(r: [f32; 3]) -> Mat4 {
    let (cx, sx) = (r[0].cos(), r[0].sin());
    let (cy, sy) = (r[1].cos(), r[1].sin());
    let (cz, sz) = (r[2].cos(), r[2].sin());
    let rx = [1.0, 0.0, 0.0, 0.0, 0.0, cx, sx, 0.0, 0.0, -sx, cx, 0.0, 0.0, 0.0, 0.0, 1.0];
    let ry = [cy, 0.0, -sy, 0.0, 0.0, 1.0, 0.0, 0.0, sy, 0.0, cy, 0.0, 0.0, 0.0, 0.0, 1.0];
    let rz = [cz, sz, 0.0, 0.0, -sz, cz, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let rxy = multiplier(&ry, &rx);
    multiplier(&rxy, &rz)
}

pub fn inverser(m: &Mat4) -> Option<Mat4> {
    let mut a = [[0.0f32; 8]; 4];
    for l in 0..4 {
        for c in 0..4 {
            a[l][c] = m[c * 4 + l];
        }
        a[l][4 + l] = 1.0;
    }
    for p in 0..4 {
        let mut pivot = p;
        for l in (p + 1)..4 {
            if a[l][p].abs() > a[pivot][p].abs() {
                pivot = l;
            }
        }
        if a[pivot][p].abs() < 1e-8 {
            return None;
        }
        a.swap(p, pivot);
        let div = a[p][p];
        for c in 0..8 {
            a[p][c] /= div;
        }
        for l in 0..4 {
            if l != p {
                let facteur = a[l][p];
                for c in 0..8 {
                    a[l][c] -= facteur * a[p][c];
                }
            }
        }
    }
    let mut inv = [0.0; 16];
    for c in 0..4 {
        for l in 0..4 {
            inv[c * 4 + l] = a[l][4 + c];
        }
    }
    Some(inv)
}
