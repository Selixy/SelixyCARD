struct GlobalesLumiere {
  vue_proj_lumiere: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> gl: GlobalesLumiere;

var<private> COINS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
  vec2<f32>(-0.5, -0.5), vec2<f32>(0.5, -0.5), vec2<f32>(0.5, 0.5),
  vec2<f32>(-0.5, -0.5), vec2<f32>(0.5, 0.5), vec2<f32>(-0.5, 0.5)
);

struct SortieOmbre {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) couleur: vec4<f32>,
  @location(2) @interpolate(flat) couche: f32,
};

@vertex
fn vs(@builtin(vertex_index) sommet: u32,
      @location(0) pos_lacet: vec4<f32>,
      @location(1) couleur: vec4<f32>,
      @location(2) forme: vec4<f32>) -> SortieOmbre {
  if (abs(forme.w + 5.0) < 0.5) {
    let s = sommet_bloc(sommet, forme.x, forme.y, forme.z);
    var ob: SortieOmbre;
    ob.position = gl.vue_proj_lumiere * vec4<f32>(tourner(s.pos, pos_lacet.w) + pos_lacet.xyz, 1.0);
    ob.uv = s.uv;
    ob.couleur = couleur;
    ob.couche = forme.w;
    return ob;
  }
  let c = COINS[sommet];
  let ct = cos(forme.z);
  let st = sin(forme.z);
  let t = vec3<f32>(c.x * forme.x, c.y * forme.y * ct, c.y * forme.y * st);
  let cy = cos(pos_lacet.w);
  let sy = sin(pos_lacet.w);
  let p = vec3<f32>(cy * t.x + sy * t.z, t.y, -sy * t.x + cy * t.z) + pos_lacet.xyz;
  var o: SortieOmbre;
  o.position = gl.vue_proj_lumiere * vec4<f32>(p, 1.0);
  o.uv = vec2<f32>(c.x + 0.5, 0.5 - c.y);
  o.couleur = couleur;
  o.couche = forme.w;
  return o;
}

@fragment
fn fs(e: SortieOmbre) {
  if (abs(e.couche + 3.0) < 0.5 || abs(e.couche + 1.0) < 0.5 || e.couleur.a < 0.15) {
    discard;
  }
  if (abs(e.couche + 5.0) < 0.5) {
    return;
  }
  let q = abs(e.uv - vec2<f32>(0.5)) * vec2<f32>(2.0, 3.0) - vec2<f32>(1.0, 1.5) + vec2<f32>(0.2);
  let bord = length(max(q, vec2<f32>(0.0))) - 0.2;
  if (bord > 0.0) {
    discard;
  }
}
