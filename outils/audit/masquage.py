import lexeur_c
import lexeur_diese
import lexeur_rust


def masquer(texte, type_fichier):
    if type_fichier in ('ts', 'css'):
        return lexeur_c.passe(texte, 'ts' if type_fichier == 'ts' else 'css')
    if type_fichier == 'rs':
        return lexeur_rust.masquer_rust(texte)
    if type_fichier == 'sh':
        return lexeur_diese.masquer_shell(texte)
    return lexeur_diese.masquer_python(texte)
