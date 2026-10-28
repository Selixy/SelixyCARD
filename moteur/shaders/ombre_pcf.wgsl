fn calculer_biais_ombre(normale: vec3<f32>, dir_lumiere: vec3<f32>) -> f32 {
  let cos_theta = clamp(dot(normale, dir_lumiere), 0.0, 1.0);
  return max(0.005 * (1.0 - cos_theta), 0.001);
}

fn evaluer_occlusion(prof_actuelle: f32, prof_obstacle: f32, biais: f32) -> f32 {
  if (prof_actuelle - biais > prof_obstacle) {
    return 0.0;
  }
  return 1.0;
}

fn filtrer_ombre_pcf(carte_ombre: texture_depth_2d,
                     echantillon_ombre: sampler_comparison,
                     coord_lumiere: vec4<f32>,
                     normale: vec3<f32>,
                     dir_lumiere: vec3<f32>,
                     texel: vec2<f32>) -> f32 {
  let proj = coord_lumiere.xyz / coord_lumiere.w;
  if (proj.z > 1.0 || proj.x < 0.0 || proj.x > 1.0 || proj.y < 0.0 || proj.y > 1.0) {
    return 1.0;
  }
  let biais = calculer_biais_ombre(normale, dir_lumiere);
  let prof_comparaison = proj.z - biais;
  var visibilite: f32 = 0.0;
  for (var x = -1; x <= 1; x = x + 1) {
    for (var y = -1; y <= 1; y = y + 1) {
      let dec = vec2<f32>(f32(x), f32(y)) * texel;
      let val = textureSampleCompareLevel(carte_ombre, echantillon_ombre, proj.xy + dec, prof_comparaison);
      visibilite = visibilite + val;
    }
  }
  return visibilite / 9.0;
}
