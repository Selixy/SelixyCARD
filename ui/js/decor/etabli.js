import { SCENES } from './scenes/index.js';
import { estCarte } from './objets.js';
import { element } from './scene.js';

export const STATIONS = {
  accueil: { dx: 0, dz: 0, vise: [1, 0], distance: 17, elevation: 1.0, etroit: { dx: 0, dz: 0, vise: [0, 2], distance: 16, elevation: 1.0 } },
  cartes: { dx: -26, dz: -2, vise: [-26, -2], distance: 18, elevation: 1.05, etroit: { dx: -9, dz: 2, vise: [-9, 4], distance: 16, elevation: 1.0 } },
  editeur: { dx: -26, dz: -22, vise: [-26, -22], distance: 18, elevation: 1.05, etroit: { dx: -9, dz: -16, vise: [-9, -14], distance: 16, elevation: 1.0 } },
  salon: { dx: 26, dz: -2, vise: [26, -2], distance: 18, elevation: 1.05, etroit: { dx: 9, dz: 2, vise: [9, 4], distance: 16, elevation: 1.0 } },
  compte: { dx: 26, dz: -22, vise: [26, -22], distance: 17, elevation: 1.0, etroit: { dx: 9, dz: -16, vise: [9, -14], distance: 16, elevation: 1.0 } },
};

function decaler(elem, dx, dz) {
  const [nom, x, z, ...reste] = elem;
  return [nom, x + dx, z + dz, ...reste];
}

function elementsStation(nom, etroit, sansCartes) {
  const scene = SCENES[nom];
  if (!scene) return [];
  const station = STATIONS[nom];
  const config = etroit && station?.etroit ? station.etroit : station;
  const source = etroit && scene.etroit ? scene.etroit : scene;
  const filtres = sansCartes ? source.elements.filter(([type]) => !estCarte(type)) : source.elements;
  const dx = config?.dx ?? 0;
  const dz = config?.dz ?? 0;
  return filtres.map((e) => decaler(e, dx, dz));
}

export function assemblerEtabli(etroit, sansCartes = false) {
  const noms = ['accueil', 'cartes', 'editeur', 'salon', 'compte'];
  return noms.flatMap((nom) => elementsStation(nom, etroit, sansCartes));
}

export function cameraStation(nom, etroit) {
  const station = STATIONS[nom] ?? STATIONS.accueil;
  const config = etroit && station.etroit ? { ...station, ...station.etroit } : station;
  return {
    vise: config.vise,
    distance: config.distance,
    elevation: config.elevation ?? 1.0,
  };
}

export function resoudreEtabli(nom, etroit, sansCartes = false) {
  const camera = cameraStation(nom, etroit);
  const elements = assemblerEtabli(etroit, sansCartes).map(element);
  return {
    vise: camera.vise,
    distance: camera.distance,
    elevation: camera.elevation,
    elements,
  };
}
