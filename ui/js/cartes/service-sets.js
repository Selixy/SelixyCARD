export const listerSets = (appeler) => appeler('GET', '/api/sets');
export const creerSet = (appeler, nom) => appeler('POST', '/api/sets', { nom });
export const chargerSet = (appeler, id) => appeler('GET', `/api/sets/${encodeURIComponent(id)}`);
export const sauverSet = (appeler, set) => appeler('PUT', `/api/sets/${encodeURIComponent(set.id)}`, set);
export const supprimerSet = (appeler, id) => appeler('DELETE', `/api/sets/${encodeURIComponent(id)}`);
