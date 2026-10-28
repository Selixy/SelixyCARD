struct Reglage {
  direction: vec2<f32>,
  reserve: vec2<f32>,
};
@group(0) @binding(0) var<uniform> r: Reglage;
@group(0) @binding(1) var source: texture_2d<f32>;
@group(0) @binding(2) var masque: texture_2d<f32>;
@group(0) @binding(3) var echantillon: sampler;

struct Sortie {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) i: u32) -> Sortie {
  let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
  var o: Sortie;
  o.position = vec4<f32>(p * 2.0 - vec2<f32>(1.0), 0.0, 1.0);
  o.uv = vec2<f32>(p.x, 1.0 - p.y);
  return o;
}

@fragment
fn fs_flou(e: Sortie) -> @location(0) vec4<f32> {
  var somme = vec4<f32>(0.0);
  var poids = 0.0;
  for (var i = -24; i <= 24; i++) {
    let w = exp(-f32(i * i) / 128.0);
    somme += textureSampleLevel(source, echantillon, e.uv + r.direction * f32(i), 0.0) * w;
    poids += w;
  }
  return somme / poids;
}

@fragment
fn fs_composer(e: Sortie) -> @location(0) vec4<f32> {
  let flou = textureSampleLevel(source, echantillon, e.uv, 0.0);
  let dedans = textureSampleLevel(masque, echantillon, e.uv, 0.0).a;
  if (dedans > 0.004 || flou.a < 0.002) { discard; }
  let opacite = pow(clamp(flou.a * 2.4, 0.0, 1.0), 1.6) * 0.9;
  return vec4<f32>(flou.rgb / flou.a, opacite);
}
