struct Globales {
  vue_proj: mat4x4<f32>,
  inv_vue_proj: mat4x4<f32>,
  transfo: mat4x4<f32>,
  inv_transfo: mat4x4<f32>,
  resolution: vec2<f32>,
  couche: f32,
  opacite: f32,
  fondu: f32,
};

@group(0) @binding(0) var<uniform> g: Globales;
@group(0) @binding(1) var textures: texture_2d_array<f32>;
@group(0) @binding(2) var echantillon: sampler;
@group(0) @binding(3) var tampon_profondeur: texture_depth_2d;

var<private> SOMMETS: array<vec3<f32>, 36> = array<vec3<f32>, 36>(
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>( 0.5,  0.5, -0.5), vec3<f32>( 0.5, -0.5, -0.5),
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>(-0.5,  0.5, -0.5), vec3<f32>( 0.5,  0.5, -0.5),
  vec3<f32>(-0.5, -0.5,  0.5), vec3<f32>( 0.5, -0.5,  0.5), vec3<f32>( 0.5,  0.5,  0.5),
  vec3<f32>(-0.5, -0.5,  0.5), vec3<f32>( 0.5,  0.5,  0.5), vec3<f32>(-0.5,  0.5,  0.5),
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>(-0.5, -0.5,  0.5), vec3<f32>(-0.5,  0.5,  0.5),
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>(-0.5,  0.5,  0.5), vec3<f32>(-0.5,  0.5, -0.5),
  vec3<f32>( 0.5, -0.5, -0.5), vec3<f32>( 0.5,  0.5,  0.5), vec3<f32>( 0.5, -0.5,  0.5),
  vec3<f32>( 0.5, -0.5, -0.5), vec3<f32>( 0.5,  0.5, -0.5), vec3<f32>( 0.5,  0.5,  0.5),
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>( 0.5, -0.5, -0.5), vec3<f32>( 0.5, -0.5,  0.5),
  vec3<f32>(-0.5, -0.5, -0.5), vec3<f32>( 0.5, -0.5,  0.5), vec3<f32>(-0.5, -0.5,  0.5),
  vec3<f32>(-0.5,  0.5, -0.5), vec3<f32>( 0.5,  0.5,  0.5), vec3<f32>( 0.5,  0.5, -0.5),
  vec3<f32>(-0.5,  0.5, -0.5), vec3<f32>(-0.5,  0.5,  0.5), vec3<f32>( 0.5,  0.5,  0.5)
);

@vertex
fn vs(@builtin(vertex_index) sommet: u32) -> @builtin(position) vec4<f32> {
  let p = SOMMETS[sommet];
  let monde = g.transfo * vec4<f32>(p, 1.0);
  return g.vue_proj * monde;
}

@fragment
fn fs(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
  let uv_ecran = frag_pos.xy / g.resolution;
  let coords = vec2<i32>(floor(frag_pos.xy));
  let z_tampon = textureLoad(tampon_profondeur, coords, 0);
  let ndc = vec4<f32>(uv_ecran.x * 2.0 - 1.0, (1.0 - uv_ecran.y) * 2.0 - 1.0, z_tampon, 1.0);
  let monde_h = g.inv_vue_proj * ndc;
  let monde = monde_h.xyz / monde_h.w;
  let local_h = g.inv_transfo * vec4<f32>(monde, 1.0);
  let local = local_h.xyz / local_h.w;
  if (abs(local.x) > 0.5 || abs(local.y) > 0.5 || abs(local.z) > 0.5) {
    discard;
  }
  let uv = local.xz + vec2<f32>(0.5);
  let texel = textureSampleLevel(textures, echantillon, uv, i32(max(g.couche, 0.0)), 0.0);
  let dx = 0.5 - abs(local.x);
  let dy = 0.5 - abs(local.y);
  let dz = 0.5 - abs(local.z);
  let dist_bords = min(dx, min(dy, dz));
  let attenuation = select(1.0, clamp(dist_bords / g.fondu, 0.0, 1.0), g.fondu > 0.0);
  let alpha = texel.a * g.opacite * attenuation;
  if (alpha < 0.001) {
    discard;
  }
  return vec4<f32>(texel.rgb, alpha);
}
