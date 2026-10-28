struct SommetBloc {
  pos: vec3<f32>,
  normale: vec3<f32>,
  uv: vec2<f32>,
  dessus: f32,
};

const ARETES: u32 = 28u;
const PAR_COIN: u32 = 7u;
const SOMMETS_MUR: u32 = 168u;

var<private> SOMMETS_QUAD: array<vec2<u32>, 6> = array<vec2<u32>, 6>(
  vec2<u32>(0u, 0u), vec2<u32>(1u, 0u), vec2<u32>(1u, 1u),
  vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), vec2<u32>(0u, 1u));

fn tourner(v: vec3<f32>, lacet: f32) -> vec3<f32> {
  let cy = cos(lacet); let sy = sin(lacet);
  return vec3<f32>(cy * v.x + sy * v.z, v.y, -sy * v.x + cy * v.z);
}

fn point_anneau(j: u32, l: f32, p: f32) -> vec4<f32> {
  let coin = f32(j / PAR_COIN);
  let angle = (coin + f32(j % PAR_COIN) / 6.0) * 1.5707963;
  let milieu = (coin + 0.5) * 1.5707963;
  let rayon = 0.1 * l;
  let centre = vec2<f32>(sign(cos(milieu)) * (0.5 * l - rayon), sign(sin(milieu)) * (0.5 * p - rayon));
  let dir = vec2<f32>(cos(angle), sin(angle));
  let xz = centre + rayon * dir;
  return vec4<f32>(xz.x, xz.y, dir.x, dir.y);
}

fn sommet_bloc(v: u32, l: f32, p: f32, h: f32) -> SommetBloc {
  var s: SommetBloc;
  var a = vec4<f32>(0.0);
  var y = 0.0;
  s.normale = vec3<f32>(0.0, 1.0, 0.0);
  s.dessus = 1.0;
  if (v < SOMMETS_MUR) {
    let k = SOMMETS_QUAD[v % 6u];
    a = point_anneau((v / 6u + k.x) % ARETES, l, p);
    y = select(-h, 0.0, k.y == 1u);
    s.normale = normalize(vec3<f32>(a.z, 0.0, a.w));
    s.dessus = 0.0;
  } else {
    let t = (v - SOMMETS_MUR) / 3u;
    let c = (v - SOMMETS_MUR) % 3u;
    if (c == 1u) { a = point_anneau(t, l, p); }
    if (c == 2u) { a = point_anneau((t + 1u) % ARETES, l, p); }
  }
  s.pos = vec3<f32>(a.x, y, a.y);
  s.uv = vec2<f32>(a.x / l + 0.5, a.y / p + 0.5);
  return s;
}
