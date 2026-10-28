struct Globales {
  vue_proj: mat4x4<f32>,
  vue_proj_lumiere: mat4x4<f32>,
  oeil: vec4<f32>,
};
@group(0) @binding(0) var<uniform> g: Globales;
@group(0) @binding(1) var textures: texture_2d_array<f32>;
@group(0) @binding(2) var echantillon: sampler;
@group(0) @binding(3) var carte_ombre: texture_depth_2d;
@group(0) @binding(4) var echantillon_ombre: sampler_comparison;

struct Sortie {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) couleur: vec4<f32>,
  @location(2) @interpolate(flat) couche: f32,
  @location(3) @interpolate(flat) taille: vec2<f32>,
  @location(4) monde: vec3<f32>,
  @location(5) normale: vec3<f32>,
  @location(6) @interpolate(flat) dessus: f32,
};

var<private> COINS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
  vec2<f32>(-0.5, -0.5), vec2<f32>(0.5, -0.5), vec2<f32>(0.5, 0.5),
  vec2<f32>(-0.5, -0.5), vec2<f32>(0.5, 0.5), vec2<f32>(-0.5, 0.5));

@vertex
fn vs(@builtin(vertex_index) sommet: u32, @location(0) pos_lacet: vec4<f32>,
      @location(1) couleur: vec4<f32>, @location(2) forme: vec4<f32>) -> Sortie {
  if (abs(forme.w + 5.0) < 0.5) {
    let s = sommet_bloc(sommet, forme.x, forme.y, forme.z);
    var ob: Sortie;
    let pb = tourner(s.pos, pos_lacet.w) + pos_lacet.xyz;
    ob.position = g.vue_proj * vec4<f32>(pb, 1.0);
    ob.uv = s.uv;
    ob.couleur = couleur;
    ob.couche = forme.w;
    ob.taille = forme.xy;
    ob.monde = pb;
    ob.normale = tourner(s.normale, pos_lacet.w);
    ob.dessus = s.dessus;
    return ob;
  }
  let c = COINS[sommet];
  let ct = cos(forme.z); let st = sin(forme.z);
  let t = vec3<f32>(c.x * forme.x, c.y * forme.y * ct, c.y * forme.y * st);
  let cy = cos(pos_lacet.w); let sy = sin(pos_lacet.w);
  let p = vec3<f32>(cy * t.x + sy * t.z, t.y, -sy * t.x + cy * t.z) + pos_lacet.xyz;
  var o: Sortie;
  o.position = g.vue_proj * vec4<f32>(p, 1.0);
  let est_ombre = abs(forme.w + 3.0) < 0.5;
  let est_decal = forme.w > -0.5 && couleur.a > 1.5;
  let decalage = select(select(0.0, 0.00015 * o.position.w, est_ombre), 0.00035 * o.position.w, est_decal);
  o.position.z -= decalage;
  o.uv = vec2<f32>(c.x + 0.5, 0.5 - c.y);
  o.couleur = couleur;
  o.couche = forme.w;
  o.taille = forme.xy;
  o.monde = p;
  o.normale = vec3<f32>(sy * ct, -st, cy * ct);
  return o;
}

fn hasard(p: vec2<f32>) -> f32 {
  var u = vec2<u32>(vec2<i32>(floor(p * 256.0)) + vec2<i32>(1000000));
  u = u * 1664525u + vec2<u32>(1013904223u, 1013904223u);
  u.x = u.x + u.y * 1664525u;
  u.y = u.y + u.x * 1664525u;
  u.x = u.x ^ (u.x >> 16u);
  u.y = u.y ^ (u.y >> 16u);
  u.x = u.x + u.y * 1664525u;
  u.y = u.y + u.x * 1664525u;
  u.x = u.x ^ (u.x >> 16u);
  return f32(u.x) / 4294967296.0;
}

fn bruit(p: vec2<f32>) -> f32 {
  let i = floor(p);
  var f = fract(p);
  f = f * f * (vec2<f32>(3.0) - 2.0 * f);
  let bas = mix(hasard(i), hasard(i + vec2<f32>(1.0, 0.0)), f.x);
  let haut = mix(hasard(i + vec2<f32>(0.0, 1.0)), hasard(i + vec2<f32>(1.0, 1.0)), f.x);
  return mix(bas, haut, f.y);
}

fn h(j: vec2<f32>, k: f32) -> f32 {
  return hasard(j + k * vec2<f32>(17.1, 31.7));
}

fn coup(p: vec2<f32>, c: vec2<f32>, j: vec2<f32>) -> f32 {
  if (h(j, 1.0) >= 0.5) { return 0.0; }
  let ang = (h(j, 4.0) - 0.5) * 1.2 + select(0.0, 1.4, h(j, 9.0) > 0.7);
  let r = c - (j + vec2<f32>(h(j, 2.0), h(j, 3.0)));
  let lx = r.x * cos(ang) + r.y * sin(ang);
  let ly = -r.x * sin(ang) + r.y * cos(ang) - (h(j, 6.0) - 0.5) * 0.5 * lx * lx;
  let t = clamp(1.0 - abs(lx) / (0.6 + 0.9 * h(j, 5.0)), 0.0, 1.0);
  if (h(j, 11.0) < 0.3) {
    let e = clamp(1.0 - abs(ly) / (0.22 + 0.15 * h(j, 7.0)), 0.0, 1.0);
    return e * sqrt(e) * clamp(t * 2.0, 0.0, 1.0) * (0.6 + 0.4 * bruit(p * 3.0)) * 0.13;
  }
  let larg = 0.012 + 0.022 * h(j, 7.0);
  let k = clamp(floor(ly / (larg * 3.2) + 0.5), 0.0, floor(3.0 * h(j, 12.0)));
  let ly2 = ly - k * larg * 3.2;
  let corps = clamp(1.0 - abs(ly2) / larg, 0.0, 1.0);
  if (corps <= 0.0 || t <= 0.0) { return 0.0; }
  let trainee = bruit(vec2<f32>(lx * 10.0 + h(j, 8.0) * 40.0, ly2 / larg * 1.5 + k * 7.0));
  let pression = 0.45 + 0.55 * bruit(vec2<f32>(lx * 1.5 + h(j, 13.0) * 20.0, k * 3.1));
  let casse = clamp((trainee * 0.7 + bruit(p * 70.0) * 0.3 - 0.3) / 0.35, 0.0, 1.0);
  return pow(corps, 0.6) * clamp(t * 3.0, 0.0, 1.0) * casse * (0.35 + 0.45 * h(j, 10.0)) * pression;
}

fn coups(p: vec2<f32>) -> f32 {
  let c = p / 3.5;
  let i = floor(c);
  var m = 0.0;
  for (var dx = -1; dx <= 1; dx++) {
    for (var dy = -1; dy <= 1; dy++) {
      m = max(m, coup(p, c, i + vec2<f32>(f32(dx), f32(dy))));
    }
  }
  return m;
}

fn projeter_monde(monde: vec3<f32>, normale: vec3<f32>) -> vec2<f32> {
  let n = abs(normale);
  if (n.x > n.y && n.x > n.z) {
    return vec2<f32>(monde.z, monde.y) + vec2<f32>(43.0, 0.0);
  }
  if (n.z > n.y) {
    return vec2<f32>(monde.x, monde.y) + vec2<f32>(65.0, 0.0);
  }
  return vec2<f32>(monde.x, monde.z) + vec2<f32>(65.0, 43.0);
}

fn ardoise(monde: vec3<f32>, normale: vec3<f32>, base: vec3<f32>) -> vec4<f32> {
  let p = projeter_monde(monde, normale);
  let pierre = 0.5 * bruit(p * 0.35) + 0.5 * bruit(p * 1.3) - 0.5;
  let fin = hasard(floor(p * 90.0)) - 0.5;
  let grain = (bruit(p * 22.0) - 0.5) * 0.5 + (bruit(p * 55.0 + vec2<f32>(3.0, 0.0)) - 0.5) * 0.35 + fin * 0.4;
  let paillette = clamp((hasard(floor(p * 40.0) + vec2<f32>(5.0, 0.0)) - 0.985) / 0.015, 0.0, 1.0) * 0.05;
  let tache = clamp((hasard(floor(p * 30.0) + vec2<f32>(0.0, 9.0)) - 0.97) / 0.03, 0.0, 1.0) * 0.04;
  let voile = 0.7 * bruit(p * vec2<f32>(0.08, 0.25) + vec2<f32>(7.0, 3.0)) + 0.3 * bruit(p * vec2<f32>(0.6, 2.0));
  let craie = pow(clamp((voile - 0.5) / 0.35, 0.0, 1.0), 2.0) * 0.03;
  let c = base * (1.0 + 0.1 * pierre + 0.65 * grain) + vec3<f32>(craie + paillette - tache);
  let trace = coups(p);
  return vec4<f32>(mix(c, vec3<f32>(0.88, 0.88, 0.85), trace), trace);
}

fn froissure(p: vec2<f32>) -> vec3<f32> {
  var pente = vec2<f32>(0.0);
  var pli = 0.0;
  for (var i = 0; i < 40; i++) {
    let k = vec2<f32>(f32(i) * 3.17, f32(i) * 1.73);
    let angle = hasard(k) * 3.14159;
    let normale = vec2<f32>(cos(angle), sin(angle));
    let origine = vec2<f32>(65.0, 43.0) + (vec2<f32>(hasard(k + vec2<f32>(1.0, 0.0)), hasard(k + vec2<f32>(0.0, 1.0))) - vec2<f32>(0.5)) * 44.0;
    let portee = select(0.5 + 1.0 * hasard(k + vec2<f32>(2.0, 2.0)), 1.6 + 3.0 * hasard(k + vec2<f32>(2.0, 2.0)), i < 18);
    let sens = select(-1.0, 1.0, hasard(k + vec2<f32>(5.0, 1.0)) > 0.5) * select(0.12, 0.35, i < 18);
    let d = dot(p - origine, normale);
    let dedans = step(abs(d), portee);
    pente += normale * (-sens / portee * sign(d) * dedans);
    pli = max(pli, (1.0 - smoothstep(0.0, 0.05, abs(d))) * dedans * select(0.5, 1.0, i < 18));
  }
  return vec3<f32>(pente, pli);
}

fn relief_kraft(monde: vec3<f32>, normale: vec3<f32>) -> vec3<f32> {
  let p = projeter_monde(monde, normale);
  let f = froissure(p);
  let n = normalize(normale);
  let t = select(normalize(cross(vec3<f32>(0.0, 1.0, 0.0), n)), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.9);
  let b = cross(n, t);
  return normalize(n - (t * f.x + b * f.y));
}

fn kraft(monde: vec3<f32>, normale: vec3<f32>, base: vec3<f32>) -> vec3<f32> {
  let p = projeter_monde(monde, normale);
  let fibres = 0.5 * bruit(p * vec2<f32>(9.0, 14.0)) + 0.5 * bruit(p * vec2<f32>(16.0, 8.0) + vec2<f32>(5.0, 0.0));
  let taches = clamp((hasard(floor(p * 24.0)) - 0.97) / 0.03, 0.0, 1.0);
  let teinte = (0.9 + 0.1 * fibres + 0.1 * (bruit(p * 0.2) - 0.5)) * (1.0 - 0.14 * froissure(p).z);
  return base * teinte - vec3<f32>(taches * 0.03);
}

fn hauteur(p: vec2<f32>) -> f32 {
  return bruit(p * 22.0) * 0.6 + bruit(p * 55.0) * 0.4;
}

fn relief(monde: vec3<f32>, normale: vec3<f32>) -> vec3<f32> {
  let p = projeter_monde(monde, normale);
  let e = vec2<f32>(0.012, 0.0);
  let dx = hauteur(p + e.xy) - hauteur(p - e.xy);
  let dz = hauteur(p + e.yx) - hauteur(p - e.yx);
  let n = normalize(normale);
  let t = select(normalize(cross(vec3<f32>(0.0, 1.0, 0.0), n)), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.9);
  let b = cross(n, t);
  return normalize(n - (t * dx + b * dz) * 0.9);
}

fn lineaire(c: vec3<f32>) -> vec3<f32> {
  return pow(max(c, vec3<f32>(0.0)), vec3<f32>(2.2));
}

fn ecran(c: vec3<f32>) -> vec3<f32> {
  let x = c * 0.88;
  let aces = clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
  return pow(aces, vec3<f32>(1.0 / 2.2));
}

fn lobe(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, rugosite: f32) -> f32 {
  let h = normalize(l + v);
  let nl = max(dot(n, l), 0.0);
  let nv = max(dot(n, v), 0.001);
  let nh = max(dot(n, h), 0.0);
  let a2 = pow(rugosite, 4.0);
  let dd = nh * nh * (a2 - 1.0) + 1.0;
  let k = (rugosite + 1.0) * (rugosite + 1.0) / 8.0;
  let geo = nv / (nv * (1.0 - k) + k) * nl / (nl * (1.0 - k) + k);
  let fresnel = 0.04 + 0.96 * pow(1.0 - max(dot(v, h), 0.0), 5.0);
  return a2 / (3.14159 * dd * dd) * geo * fresnel / (4.0 * nv);
}

fn lampe(i: i32, monde: vec3<f32>) -> vec3<f32> {
  if (i == 0) { return normalize(vec3<f32>(5.0, 11.0, 4.0) - monde); }
  if (i == 1) { return normalize(vec3<f32>(-6.0, 5.0, -3.0)); }
  return normalize(vec3<f32>(0.0, 4.0, -9.0));
}

fn teinte_lampe(i: i32) -> vec3<f32> {
  if (i == 0) { return vec3<f32>(1.0, 0.9, 0.78) * 1.5; }
  if (i == 1) { return vec3<f32>(0.5, 0.62, 0.9) * 0.35; }
  return vec3<f32>(0.75, 0.8, 0.95) * 0.28;
}

fn calculer_biais_ombre(normale: vec3<f32>, dir_lumiere: vec3<f32>) -> f32 {
  let cos_theta = clamp(dot(normale, dir_lumiere), 0.0, 1.0);
  return max(0.0025 * (1.0 - cos_theta), 0.0006);
}

fn filtrer_ombre(monde: vec3<f32>, normale: vec3<f32>, dir_lumiere: vec3<f32>) -> f32 {
  let p_lum = g.vue_proj_lumiere * vec4<f32>(monde, 1.0);
  if (p_lum.w <= 0.0) { return 1.0; }
  let proj = vec3<f32>(p_lum.x * 0.5 / p_lum.w + 0.5, 0.5 - 0.5 * p_lum.y / p_lum.w, p_lum.z / p_lum.w);
  if (proj.z > 1.0 || proj.x < 0.0 || proj.x > 1.0 || proj.y < 0.0 || proj.y > 1.0) {
    return 1.0;
  }
  let biais = calculer_biais_ombre(normale, dir_lumiere);
  let ref_prof = proj.z - biais;
  let texel = vec2<f32>(1.0 / 2048.0, 1.0 / 2048.0);
  var vis: f32 = 0.0;
  for (var x = -1; x <= 1; x++) {
    for (var y = -1; y <= 1; y++) {
      let dec = vec2<f32>(f32(x), f32(y)) * texel;
      vis += textureSampleCompare(carte_ombre, echantillon_ombre, proj.xy + dec, ref_prof);
    }
  }
  return mix(0.58, 1.0, vis / 9.0);
}

fn eclairer(albedo: vec3<f32>, rugosite: f32, vernis: f32, normale: vec3<f32>, monde: vec3<f32>) -> vec3<f32> {
  let v = normalize(g.oeil.xyz - monde);
  let n = normalize(normale) * select(1.0, -1.0, dot(normale, v) < 0.0);
  var c = vec3<f32>(0.0);
  let ombre = filtrer_ombre(monde, n, lampe(0, monde));
  for (var i = 0; i < 3; i++) {
    let l = lampe(i, monde);
    let reflet = select(lobe(n, v, l, rugosite) + vernis * lobe(n, v, l, 0.12), 0.0, rugosite >= 0.98);
    let direct = (albedo / 3.14159 * max(dot(n, l), 0.0) + vec3<f32>(reflet)) * teinte_lampe(i);
    c += select(direct, direct * ombre, i == 0);
  }
  let ciel = mix(vec3<f32>(0.02, 0.02, 0.025), vec3<f32>(0.07, 0.08, 0.10), 0.5 + 0.5 * n.y);
  return c + ciel * albedo + ciel * 0.04 * (1.0 - rugosite);
}

fn matiere(base: vec3<f32>, uv: vec2<f32>, normale: vec3<f32>, monde: vec3<f32>, bord: f32, face: bool) -> vec3<f32> {
  let carton = (bruit(uv * vec2<f32>(170.0, 255.0)) - 0.5) * 0.07 + (hasard(floor(uv * vec2<f32>(256.0, 384.0))) - 0.5) * 0.025;
  let biseau = 1.0 - smoothstep(0.0, 0.03, -bord);
  let albedo = lineaire(base * (1.0 + carton) * (1.0 + 0.08 * biseau));
  let rugosite = select(0.85, 0.5 + 2.5 * carton, face);
  return ecran(eclairer(albedo, rugosite, select(0.0, 0.35, face), normale, monde));
}

fn marque(couleur: vec4<f32>) -> vec4<f32> {
  let poids = (couleur.a - 8.0) / 1.5;
  return vec4<f32>(couleur.rgb * poids, poids);
}

@fragment
fn fs(e: Sortie) -> @location(0) vec4<f32> {
  if (abs(e.couche + 1.0) < 0.5 && e.couleur.a > 1.5) {
    let n = relief_kraft(e.monde, e.normale);
    let teinte = eclairer(lineaire(kraft(e.monde, e.normale, e.couleur.rgb)), 0.88, 0.0, n, e.monde);
    return vec4<f32>(ecran(teinte) * (1.0 - 0.3 * length(e.monde.xz / vec2<f32>(130.0, 86.0))), 1.0);
  }
  if (abs(e.couche + 1.0) < 0.5) {
    let a = ardoise(e.monde, e.normale, e.couleur.rgb);
    let n = relief(e.monde, e.normale);
    let teinte = eclairer(lineaire(a.rgb), mix(0.78, 0.97, a.w), 0.0, n, e.monde);
    return vec4<f32>(ecran(teinte) * (1.0 - 0.3 * length(e.monde.xz / vec2<f32>(130.0, 86.0))), 1.0);
  }
  if (abs(e.couche + 3.0) < 0.5) {
    let q = (e.uv - vec2<f32>(0.5)) * 2.0;
    let r = length(q);
    let bord = 1.0 - smoothstep(0.2, 1.0, r);
    if (bord < 0.01 || e.couleur.a < 0.01) { discard; }
    return vec4<f32>(e.couleur.rgb, e.couleur.a * bord);
  }
  if (abs(e.couche + 5.0) < 0.5) {
    if (e.couleur.a > 7.5) { return marque(e.couleur); }
    let feuillet = select(0.9 + 0.1 * step(0.5, fract(e.monde.y / 0.025)), 1.0, e.dessus > 0.5);
    let albedo = lineaire(vec3<f32>(0.84, 0.83, 0.79) * feuillet);
    return vec4<f32>(ecran(eclairer(albedo, 0.85, 0.0, e.normale, e.monde)), 1.0);
  }
  let uv = e.uv;
  let texel = textureSampleLevel(textures, echantillon, uv, i32(max(e.couche, 0.0)), 0.0);
  if (e.couleur.a > 3.5 && e.couleur.a < 5.0) {
    if (texel.a < 0.02 || e.couche < 0.0) { discard; }
    return vec4<f32>(e.couleur.rgb, texel.a * (e.couleur.a - 4.0));
  }
  if (e.couleur.a > 1.5 && e.couleur.a < 2.5) {
    if (texel.a < 0.08 || e.couche < 0.0) { discard; }
    let papier = lineaire(min(texel.rgb / max(texel.a, 0.001), vec3<f32>(1.0)));
    let vignettage = 1.0 - 0.3 * length(e.monde.xz / vec2<f32>(130.0, 86.0));
    return vec4<f32>(ecran(eclairer(papier, 1.0, 0.0, e.normale, e.monde)) * vignettage, texel.a);
  }
  let q = abs(uv - vec2<f32>(0.5)) * vec2<f32>(2.0, 3.0) - vec2<f32>(1.0, 1.5) + vec2<f32>(0.2);
  let bord = length(max(q, vec2<f32>(0.0))) - 0.2;
  let delta = max(fwidth(bord), 0.001);
  if (e.couleur.a > 7.5) {
    if (bord > -0.5 * delta) { discard; }
    return marque(e.couleur);
  }
  let alpha_bord = clamp(0.5 - bord / delta, 0.0, 1.0);
  if (alpha_bord <= 0.01 || e.couleur.a < 0.02) {
    discard;
  }
  let base = select(e.couleur.rgb, mix(vec3<f32>(0.984, 0.973, 0.945), texel.rgb, texel.a), e.couche >= 0.0);
  let couleur_fin = matiere(base, uv, e.normale, e.monde, bord, e.couche >= 0.0);
  return vec4<f32>(couleur_fin, e.couleur.a * alpha_bord);
}
