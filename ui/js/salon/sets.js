export const SET_PAR_DEFAUT = { id: '', nom: 'Set par défaut (sans illustrations)' };

export async function chargerSets(appeler) {
  try {
    const sets = await appeler('GET', '/api/sets');
    return Array.isArray(sets) ? sets : [];
  } catch {
    return [];
  }
}

export function libelleSet(set) {
  if (!set.auteur) return set.nom;
  const illustrees = set.nb_illustrees ?? 0;
  return `${set.nom} — ${set.auteur} (${illustrees} illustrée${illustrees > 1 ? 's' : ''})`;
}

export function remplirSets(select, sets, courant, modifiable) {
  const tous = [SET_PAR_DEFAUT, ...sets];
  if (select.options.length !== tous.length) {
    select.replaceChildren(
      ...tous.map((set) => {
        const option = select.ownerDocument.createElement('option');
        option.value = set.id;
        option.textContent = libelleSet(set);
        return option;
      }),
    );
  }
  select.value = courant ?? '';
  select.disabled = !modifiable;
}
