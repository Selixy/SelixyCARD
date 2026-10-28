export function choisirRendu(parametres, webgpuDisponible) {
  const demande = parametres.get('rendu');
  if (demande === 'webgl2' || demande === 'webgpu') return demande;
  return webgpuDisponible ? 'webgpu' : 'webgl2';
}
