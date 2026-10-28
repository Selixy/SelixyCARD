from lexeur_c import effacer, fin_chaine


def fin_caractere(texte, i):
    j = i + 2 if texte[i + 1:i + 2] == '\\' else i + 1
    fin = texte.find("'", j + 1, j + 10)
    return fin + 1 if fin > 0 and (texte[i + 1] == '\\' or fin == i + 2) else -1


def fin_commentaire(texte, i):
    if texte[i + 1] == '/':
        j = texte.find('\n', i)
        return len(texte) if j < 0 else j
    j = texte.find('*/', i + 2)
    return len(texte) if j < 0 else j + 2


def masquer_rust(texte):
    sortie = list(texte)
    i, total = 0, 0
    while i < len(texte):
        k, suivant = texte[i], texte[i + 1:i + 2]
        fin = -1
        if k == '"':
            fin = fin_chaine(texte, i)
        elif k == "'":
            fin = fin_caractere(texte, i)
        elif k == '/' and suivant in ('/', '*'):
            fin = fin_commentaire(texte, i)
            effacer(sortie, i, fin)
            total += 1
            i = fin
            continue
        if fin > 0:
            effacer(sortie, i + 1, fin - 1)
            i = fin
        else:
            i += 1
    return ''.join(sortie), total
