pub fn fonctions_procedurales_wgsl() -> &'static str {
    "fn hasard(p: vec2<f32>) -> f32 {\n    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);\n}\n\nfn bruit(p: vec2<f32>) -> f32 {\n    let i = floor(p);\n    var f = fract(p);\n    f = f * f * (vec2<f32>(3.0) - 2.0 * f);\n    let bas = mix(hasard(i), hasard(i + vec2<f32>(1.0, 0.0)), f.x);\n    let haut = mix(hasard(i + vec2<f32>(0.0, 1.0)), hasard(i + vec2<f32>(1.0, 1.0)), f.x);\n    return mix(bas, haut, f.y);\n}\n"
}

pub fn fonctions_procedurales_glsl() -> &'static str {
    "float hasard(vec2 p) {\n    return fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);\n}\n\nfloat bruit(vec2 p) {\n    vec2 i = floor(p);\n    vec2 f = fract(p);\n    f = f * f * (3.0 - 2.0 * f);\n    float bas = mix(hasard(i), hasard(i + vec2(1.0, 0.0)), f.x);\n    float haut = mix(hasard(i + vec2(0.0, 1.0)), hasard(i + vec2(1.0, 1.0)), f.x);\n    return mix(bas, haut, f.y);\n}\n"
}
