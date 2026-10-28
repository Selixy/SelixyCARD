pub fn structure_wgsl() -> &'static str {
    "struct ProprietesSurface {\n    albedo: vec3<f32>,\n    rugosite: f32,\n    metallique: f32,\n    normale: vec3<f32>,\n    relief: f32,\n    emission: vec3<f32>,\n    alpha: f32,\n};\n"
}

pub fn structure_glsl() -> &'static str {
    "struct ProprietesSurface {\n    vec3 albedo;\n    float rugosite;\n    float metallique;\n    vec3 normale;\n    float relief;\n    vec3 emission;\n    float alpha;\n};\n"
}

pub fn fonctions_pbr_wgsl() -> &'static str {
    "fn distribution_ggx(n_dot_h: f32, rugosite: f32) -> f32 {\n    let a = rugosite * rugosite;\n    let a2 = a * a;\n    let d = (n_dot_h * a2 - n_dot_h) * n_dot_h + 1.0;\n    return a2 / (3.14159265 * d * d);\n}\n\nfn geometrie_smith(n_dot_v: f32, n_dot_l: f32, rugosite: f32) -> f32 {\n    let r = rugosite + 1.0;\n    let k = (r * r) / 8.0;\n    let g1 = n_dot_v / (n_dot_v * (1.0 - k) + k);\n    let g2 = n_dot_l / (n_dot_l * (1.0 - k) + k);\n    return g1 * g2;\n}\n\nfn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {\n    return f0 + (vec3<f32>(1.0) - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);\n}\n"
}

pub fn fonctions_pbr_glsl() -> &'static str {
    "float distribution_ggx(float n_dot_h, float rugosite) {\n    float a = rugosite * rugosite;\n    float a2 = a * a;\n    float d = (n_dot_h * a2 - n_dot_h) * n_dot_h + 1.0;\n    return a2 / (3.14159265 * d * d);\n}\n\nfloat geometrie_smith(float n_dot_v, float n_dot_l, float rugosite) {\n    float r = rugosite + 1.0;\n    float k = (r * r) / 8.0;\n    float g1 = n_dot_v / (n_dot_v * (1.0 - k) + k);\n    float g2 = n_dot_l / (n_dot_l * (1.0 - k) + k);\n    return g1 * g2;\n}\n\nvec3 fresnel_schlick(float cos_theta, vec3 f0) {\n    return f0 + (vec3(1.0) - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);\n}\n"
}
