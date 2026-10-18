# Les expressions régulières

Cette page montre comment chercher, vérifier et remplacer des morceaux de texte grâce aux expressions régulières et au module `re`.

> **Objectifs :**
>
> - vous saurez écrire un motif simple avec des classes et des quantificateurs ;
> - vous saurez choisir entre `search`, `match`, `fullmatch` et `findall` ;
> - vous saurez récupérer des morceaux du texte avec des groupes ;
> - vous saurez remplacer et découper du texte avec `sub` et `split` ;
> - vous saurez vérifier un format simple (code postal, date, adresse e-mail).

## Qu'est-ce qu'une expression régulière ?

Une [expression régulière](../debuter/glossaire.md#expression-reguliere) (on dit aussi
*regex*) est un **motif** qui décrit une forme de texte. Par exemple : « trois chiffres,
un tiret, puis deux lettres ». C'est un portrait-robot : il ne décrit pas une personne
précise, mais toutes celles qui lui ressemblent.

En Python, les regex sont dans le module `re`. Première recherche :

```python
import re

texte = "Rendez-vous salle 214 à 14h"
resultat = re.search(r"\d+", texte)
print(resultat.group())
```

```sortie
214
```

`\d` veut dire « un chiffre » et `+` « une ou plusieurs fois ». `re.search` trouve le
premier endroit du texte qui correspond, ici `214`.

## Les chaînes brutes r""

Dans une chaîne Python normale, `\` sert déjà à écrire des caractères spéciaux (`\n`,
`\t`). Les regex utilisent aussi beaucoup de `\`. Pour éviter le conflit, écrivez vos
motifs dans une **chaîne brute** : un `r` devant les guillemets. Python laisse alors
les `\` tels quels.

```python
print(len("\n"), len(r"\n"))
print(r"\d+")
```

```sortie
1 2
\d+
```

> **À retenir :** écrivez **toujours** vos motifs avec `r"..."`.

## Les briques d'un motif

Un motif se construit avec des briques. Voici les plus utiles.

| Brique | Signification | Exemple | Correspond à |
|--------|---------------|---------|--------------|
| `abc` | le texte exact | `chat` | `chat` |
| `.` | n'importe quel caractère (sauf `\n`) | `c.t` | `cat`, `c9t` |
| `\d` | un chiffre | `\d\d` | `42` |
| `\w` | une lettre, un chiffre ou `_` | `\w+` | `Léa_2` |
| `\s` | un espace, une tabulation, un retour à la ligne | `a\sb` | `a b` |
| `[abc]` | un caractère parmi ceux-là | `[aeiou]` | `e` |
| `[a-z]` | un caractère dans l'intervalle | `[0-9]` | `7` |
| `[^abc]` | un caractère **sauf** ceux-là | `[^0-9]` | `x` |
| `*` | 0 fois ou plus | `ab*` | `a`, `abbb` |
| `+` | 1 fois ou plus | `\d+` | `2024` |
| `?` | 0 ou 1 fois (facultatif) | `chats?` | `chat`, `chats` |
| `{n}` | exactement n fois | `\d{5}` | `69001` |
| `{n,m}` | entre n et m fois | `\d{2,4}` | `12`, `2024` |
| `^` | début du texte | `^Bonjour` | `Bonjour` au début |
| `$` | fin du texte | `!$` | `!` à la fin |
| `a\|b` | `a` ou `b` | `oui\|non` | `oui`, `non` |
| `\.` | un vrai point | `\.fr` | `.fr` |

Les caractères `. * + ? ( ) [ ] { } ^ $ | \` ont un sens spécial. Pour chercher le
caractère lui-même, mettez un `\` devant : `\.` cherche un point, `\?` un point
d'interrogation.

## Chercher : search, match et fullmatch

Ces trois fonctions renvoient un objet `Match` si le texte correspond, ou `None` sinon.
Elles diffèrent par l'endroit où elles cherchent.

| Fonction | Cherche… |
|----------|----------|
| `re.search(motif, texte)` | n'importe où dans le texte |
| `re.match(motif, texte)` | seulement au **début** du texte |
| `re.fullmatch(motif, texte)` | le texte **entier** doit correspondre |

```python
import re

texte = "abc 123"
for fonction in [re.search, re.match, re.fullmatch]:
    resultat = fonction(r"\d+", texte)
    print(fonction.__name__, "->", resultat.group() if resultat else None)

print(re.fullmatch(r"\d+", "123") is not None)
```

```sortie
search -> 123
match -> None
fullmatch -> None
True
```

Comme le résultat peut être `None`, testez-le toujours avant d'appeler `.group()`.

```python
import re

for phrase in ["Il fait 21 degrés", "Il fait beau"]:
    trouve = re.search(r"\d+", phrase)
    if trouve:
        print("Température :", trouve.group())
    else:
        print("Pas de nombre.")
```

```sortie
Température : 21
Pas de nombre.
```

L'objet `Match` donne aussi la position : `trouve.start()` et `trouve.end()`.

## Trouver toutes les occurrences

`re.findall` renvoie la liste de **tous** les morceaux qui correspondent.
`re.finditer` donne les objets `Match` un par un, avec leurs positions.

```python
import re

texte = "Ticket 12 : 3 pommes à 2 € et 15 noix"
print(re.findall(r"\d+", texte))

for m in re.finditer(r"\d+", texte):
    print(m.group(), "à la position", m.start())
```

```sortie
['12', '3', '2', '15']
12 à la position 7
3 à la position 12
2 à la position 23
15 à la position 30
```

## Les groupes

Des parenthèses `( )` forment un **groupe** : une partie du motif que vous voulez
récupérer à part. `group(1)` donne le premier groupe, `group(2)` le deuxième, etc.

```python
import re

m = re.search(r"(\d{2})/(\d{2})/(\d{4})", "Né le 07/03/2015 à Nantes")
print(m.group())
print(m.group(1), m.group(2), m.group(3))
print(m.groups())
```

```sortie
07/03/2015
07 03 2015
('07', '03', '2015')
```

### Les groupes nommés

Les numéros deviennent vite confus. Vous pouvez nommer un groupe avec `(?P<nom>...)`,
puis le lire avec `m["nom"]` ou `m.groupdict()`.

```python
import re

motif = r"(?P<jour>\d{2})/(?P<mois>\d{2})/(?P<annee>\d{4})"
m = re.search(motif, "Né le 07/03/2015 à Nantes")
print(m["annee"])
print(m.groupdict())
```

```sortie
2015
{'jour': '07', 'mois': '03', 'annee': '2015'}
```

### findall avec des groupes

Avec des groupes, `findall` renvoie la liste des groupes, sous forme de tuples.

```python
import re

texte = "Léa=15, Hugo=11, Inès=17"
for nom, note in re.findall(r"(\w+)=(\d+)", texte):
    print(f"{nom} a eu {note}")
```

```sortie
Léa a eu 15
Hugo a eu 11
Inès a eu 17
```

## Remplacer et découper

`re.sub(motif, remplacement, texte)` remplace chaque morceau trouvé. Dans le
remplacement, `\1`, `\2`… réutilisent les groupes.

```python
import re

print(re.sub(r"\s+", " ", "trop    d'espaces   ici"))
print(re.sub(r"(\d{2})/(\d{2})/(\d{4})", r"\3-\2-\1", "le 07/03/2015"))
print(re.sub(r"\d", "#", "Code : 4521"))
```

```sortie
trop d'espaces ici
le 2015-03-07
Code : ####
```

`re.split` découpe un texte partout où le motif correspond. Plus souple que
`str.split`, il accepte plusieurs séparateurs à la fois.

```python
import re

print(re.split(r"[,;]\s*", "pomme, poire;kiwi ;  melon"))
print(re.split(r"\s*[,;]\s*", "pomme, poire;kiwi ;  melon"))
```

```sortie
['pomme', 'poire', 'kiwi ', 'melon']
['pomme', 'poire', 'kiwi', 'melon']
```

## Les options (flags)

Un troisième argument change le comportement de la recherche.

| Option | Effet |
|--------|-------|
| `re.IGNORECASE` (ou `re.I`) | ignore majuscules et minuscules |
| `re.MULTILINE` (ou `re.M`) | `^` et `$` marchent pour chaque ligne |
| `re.DOTALL` (ou `re.S`) | `.` accepte aussi `\n` |
| `re.VERBOSE` (ou `re.X`) | permet espaces et commentaires dans le motif |

```python
import re

print(re.findall(r"python", "Python, PYTHON et python", re.IGNORECASE))

texte = "- lait\n- pain\nnote\n- œufs"
print(re.findall(r"^- (.+)$", texte, re.MULTILINE))
```

```sortie
['Python', 'PYTHON', 'python']
['lait', 'pain', 'œufs']
```

Pour combiner deux options, utilisez `|` : `re.I | re.M`.

## Compiler un motif

Si vous utilisez le même motif plusieurs fois, `re.compile` le prépare une fois pour
toutes. L'objet obtenu a les mêmes méthodes : `search`, `findall`, `sub`…

```python
import re

heure = re.compile(r"(\d{1,2})h(\d{2})?")
for texte in ["Départ 9h30", "Arrivée 14h", "Sans heure"]:
    m = heure.search(texte)
    if m:
        minutes = m.group(2) or "00"
        print(f"{m.group(1)}:{minutes}")
```

```sortie
9:30
14:00
```

Le groupe `(\d{2})?` est facultatif : s'il manque, `m.group(2)` vaut `None`.

## Vérifier un format simple

Avec `fullmatch`, une regex devient un contrôle de saisie : le texte entier doit avoir
la bonne forme.

```python
import re

formats = {
    "code postal": r"\d{5}",
    "date": r"\d{2}/\d{2}/\d{4}",
    "e-mail": r"[\w.+-]+@[\w-]+\.[\w.]+",
    "téléphone": r"0\d( ?\d{2}){4}",
}

essais = [
    ("code postal", "69001"),
    ("code postal", "6900"),
    ("date", "31/12/2024"),
    ("e-mail", "lea.martin@exemple.fr"),
    ("e-mail", "lea@exemple"),
    ("téléphone", "06 12 34 56 78"),
]

for nom, valeur in essais:
    ok = re.fullmatch(formats[nom], valeur) is not None
    print(f"{nom:12} {valeur:22} {'valide' if ok else 'invalide'}")
```

```sortie
code postal  69001                  valide
code postal  6900                   invalide
date         31/12/2024             valide
e-mail       lea.martin@exemple.fr  valide
e-mail       lea@exemple            invalide
téléphone    06 12 34 56 78         valide
```

> **Attention :** une regex vérifie la **forme**, pas le sens. `99/99/2024` passe le
> test de date. Pour une vraie date, utilisez `datetime.strptime` (voir
> [les dates](dates.md)). Et une regex d'e-mail parfaite n'existe pas : gardez-la simple.

### Gourmand ou pas ?

`*` et `+` sont **gourmands** : ils prennent le plus de caractères possible. Ajoutez
`?` après (`*?`, `+?`) pour qu'ils en prennent le moins possible.

```python
import re

html = "<b>gras</b> et <i>italique</i>"
print(re.findall(r"<.+>", html))
print(re.findall(r"<.+?>", html))
```

```sortie
['<b>gras</b> et <i>italique</i>']
['<b>', '</b>', '<i>', '</i>']
```

## Erreurs fréquentes

### Oublier le r devant le motif

```python norun
import re
m = re.search("\d+", "abc 42")
print(m.group())
```

```text
main.py:2: SyntaxWarning: invalid escape sequence '\d'
  m = re.search("\d+", "abc 42")
42
```

Le programme marche encore, mais Python avertit : `\d` n'est pas un échappement connu
dans une chaîne normale. Avec d'autres (`\b`, qui veut dire « retour arrière »), le
motif devient faux sans prévenir. Écrivez `r"\d+"`.

### Appeler group() sur None

```python norun
import re
m = re.match(r"\d+", "abc 42")
print(m.group())
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(m.group())
          ^^^^^^^
AttributeError: 'NoneType' object has no attribute 'group'
```

`re.match` ne cherche qu'au début du texte, qui commence par `abc` : il renvoie `None`.
Utilisez `re.search`, et testez toujours `if m:` avant `m.group()`.

### Une parenthèse non fermée

```python norun
import re
re.compile(r"(abc")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    re.compile(r"(abc")
    ~~~~~~~~~~^^^^^^^^^
  ...
re.PatternError: missing ), unterminated subpattern at position 0
```

La parenthèse ouverte en position 0 n'est jamais fermée. Pour chercher une vraie
parenthèse, écrivez `\(`.

## Testez-vous

**Question 1 :** que renvoie `re.findall(r"\d{2}", "1 22 333")` ?

> **Réponse**
>
> `['22', '33']`. `\d{2}` prend deux chiffres exactement : `22`, puis `33` dans `333`
> (le dernier `3` reste seul).

**Question 2 :** quelle fonction utiliser pour vérifier qu'un texte entier est un code
postal ?

> **Réponse**
>
> `re.fullmatch(r"\d{5}", texte)`. Avec `search`, `"abc 69001 xyz"` passerait le test.

**Question 3 :** extrayez tous les mots-clés `#...` de la phrase
`"J'adore #python et #regex !"`.

> **Réponse**
>
> ```python
> import re
>
> print(re.findall(r"#(\w+)", "J'adore #python et #regex !"))
> ```
>
> ```sortie
> ['python', 'regex']
> ```

## En résumé

- Une regex est un motif ; écrivez-la toujours en chaîne brute `r"..."`.
- `search` cherche partout, `match` au début, `fullmatch` sur tout le texte ; tous
  peuvent renvoyer `None`.
- `findall` et `finditer` trouvent toutes les occurrences.
- Les groupes `( )` et `(?P<nom>...)` récupèrent des morceaux du texte.
- `sub` remplace, `split` découpe ; `re.I` ignore la casse.
- `fullmatch` vérifie la forme d'une saisie, pas son sens.

Suite : [dates et heures](dates.md). Comparez avec
[les expressions régulières en Ruby](../ruby/regex.md).
