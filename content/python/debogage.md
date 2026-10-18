# Déboguer en Python

Cette page montre comment trouver et corriger un bug en Python : lire une trace d'erreur, afficher les valeurs avec `print` et `repr`, mettre le programme en pause avec `breakpoint()`, et garder une trace avec `logging`.

> **Objectifs :**
>
> - lire une trace d'erreur (*traceback*) en partant du bas ;
> - afficher l'état d'un programme avec `print(f"{x=}")` et `repr` ;
> - mettre le programme en pause avec `breakpoint()` et le débogueur `pdb` ;
> - utiliser le module `logging` à la place des `print` ;
> - suivre une méthode simple pour corriger un bug.

## Qu'est-ce que déboguer ?

Un [bug](../debuter/glossaire.md#bug) est une erreur dans un programme. Le
[débogage](../debuter/glossaire.md#debogage) consiste à le trouver, puis à le corriger.

Déboguer, c'est mener une enquête : observer les indices, faire une hypothèse, la
vérifier. On ne devine pas au hasard. Pour les bases communes à tous les langages, voyez
aussi [déboguer pour débuter](../debuter/deboguer.md).

Il existe deux sortes de bugs :

- le programme **plante** : Python affiche une erreur et s'arrête ;
- le programme **se trompe** : il ne dit rien, mais le résultat est faux. C'est le plus
  difficile.

## Lire une trace d'erreur

Quand une [exception](../debuter/glossaire.md#exception) arrête le programme, Python
affiche une **trace** (*traceback*). Voici un programme qui calcule le total d'un panier ;
un article a une faute de frappe.

```python norun
def prix_ligne(article):
    return article["prix"] * article["quantite"]


def total(panier):
    somme = 0
    for article in panier:
        somme += prix_ligne(article)
    return somme


panier = [
    {"nom": "pain", "prix": 2, "quantite": 3},
    {"nom": "lait", "prix": 1, "quantité": 2},
]
print(total(panier))
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 16, in <module>
    print(total(panier))
          ~~~~~^^^^^^^^
  File "/home/alice/main.py", line 8, in total
    somme += prix_ligne(article)
             ~~~~~~~~~~^^^^^^^^^
  File "/home/alice/main.py", line 2, in prix_ligne
    return article["prix"] * article["quantite"]
                             ~~~~~~~^^^^^^^^^^^^
KeyError: 'quantite'
```

`most recent call last` veut dire « l'appel le plus récent en dernier ». Lisez donc la
trace **de bas en haut** :

1. **La dernière ligne** donne le type d'erreur et un message : `KeyError: 'quantite'`,
   la clé `"quantite"` n'existe pas dans le dictionnaire.
2. **Le bloc juste au-dessus** montre où : `line 2, in prix_ligne`. Les `^^^^` soulignent
   le morceau exact qui a planté.
3. **Les blocs plus haut** racontent le chemin : `<module>` (le programme principal) a
   appelé `total`, qui a appelé `prix_ligne`.

Ici, la ligne 2 est correcte : c'est la **donnée** qui est fausse. Le second article
s'écrit `"quantité"`, avec un accent.

> **À retenir :** l'endroit où l'erreur éclate n'est pas toujours l'endroit du bug.
> La trace vous dit *où regarder* ; c'est à vous de remonter jusqu'à la cause.

## Afficher les valeurs avec print

L'outil le plus simple reste `print` : affichez les valeurs pour voir ce que le
programme fait vraiment.

### f"{x=}" : le nom et la valeur

Dans une f-string, `{x=}` affiche le nom de la variable, un `=` et sa valeur. Plus
besoin d'écrire `print("x =", x)`.

```python
prix = 12
quantite = 3
nom = "Léa"
notes = [12, None, 18]

print(f"{prix=}")
print(f"{nom=}")
print(f"{notes=}")
print(f"{prix * quantite = }")
```

```sortie
prix=12
nom='Léa'
notes=[12, None, 18]
prix * quantite = 36
```

Remarquez les guillemets autour de `'Léa'` : `{x=}` montre la valeur telle qu'on
l'écrirait dans le code.

### repr : voir le vrai type

`print("12")` et `print(12)` affichent la même chose : `12`. Impossible de savoir si
c'est un texte ou un nombre. La fonction `repr` montre la valeur **avec** ses guillemets,
ses espaces et ses caractères invisibles.

```python
valeur = "12"
print(valeur)
print(repr(valeur))
print(type(valeur))

ligne = "Alice\t15 \n"
print(repr(ligne))
```

```sortie
12
'12'
<class 'str'>
'Alice\t15 \n'
```

> **Astuce :** quand un résultat vous surprend, affichez `repr(x)` et `type(x)`. Un
> nombre qui est en fait un texte, ou un espace en trop, sont des bugs très courants.

## Mettre le programme en pause avec breakpoint()

Ajouter des `print` partout devient vite pénible. La fonction `breakpoint()`, intégrée
à Python, **met le programme en pause** et ouvre le débogueur **pdb**. Vous pouvez alors
regarder les variables, puis avancer ligne par ligne.

```python norun
def moyenne(notes):
    total = 0
    for note in notes:
        if note is not None:
            total += note
    breakpoint()
    return total / len(notes)


print(moyenne([12, None, 18]))
```

Le programme s'arrête sur `breakpoint()` et attend vos commandes après `(Pdb)` :

```text
> /home/alice/main.py(6)moyenne()
-> breakpoint()
(Pdb) p total
30
(Pdb) p notes
[12, None, 18]
(Pdb) p len(notes)
3
(Pdb) n
> /home/alice/main.py(7)moyenne()
-> return total / len(notes)
(Pdb) c
10.0
```

Les commandes à connaître :

| Commande | Effet |
| --- | --- |
| `p expr` | afficher la valeur d'une expression |
| `pp expr` | afficher joliment une grosse structure |
| `l` | montrer le code autour de la ligne courante |
| `n` | exécuter la ligne suivante (*next*) |
| `s` | entrer dans la fonction appelée (*step*) |
| `c` | continuer jusqu'à la prochaine pause (*continue*) |
| `w` | afficher la pile d'appels (*where*) |
| `q` | quitter le programme (*quit*) |

> **Note :** sans toucher au code, `python3 -m pdb main.py` lance tout le programme dans
> le débogueur. Les éditeurs comme VS Code offrent la même chose avec des boutons : voir
> [l'éditeur de code](../debuter/editeur.md).

## Garder une trace avec logging

Les `print` de débogage finissent souvent oubliés dans le code. Le module **logging**
fait mieux : chaque message a un **niveau** d'importance, et un seul réglage choisit ce
qui s'affiche.

| Niveau | Pour… |
| --- | --- |
| `DEBUG` | les détails utiles pendant la mise au point |
| `INFO` | les étapes normales du programme |
| `WARNING` | quelque chose d'étrange, mais le programme continue |
| `ERROR` | une opération a échoué |

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("notes")


def moyenne(notes):
    log.debug("notes reçues : %s", notes)
    if not notes:
        log.warning("liste vide, moyenne impossible")
        return None
    return sum(notes) / len(notes)


print(moyenne([12, 18]))
print(moyenne([]))
```

```sortie
DEBUG:notes:notes reçues : [12, 18]
15.0
DEBUG:notes:notes reçues : []
WARNING:notes:liste vide, moyenne impossible
None
```

- `basicConfig` règle l'affichage une seule fois, au début du programme. Par défaut,
  les messages vont sur la sortie d'erreur ; ici, `stream=sys.stdout` les envoie sur la
  sortie normale.
- `level=logging.DEBUG` affiche tout. Passez à `logging.WARNING` : les messages `DEBUG`
  disparaissent, sans effacer une seule ligne de code.
- `%s` dans le message est remplacé par la valeur suivante.

## Les erreurs les plus courantes

Voici les exceptions que vous croiserez le plus souvent, avec leur cause habituelle.

| Exception | Exemple de message | Cause habituelle |
| --- | --- | --- |
| `NameError` | `name 'nom' is not defined` | faute de frappe, variable pas encore créée |
| `TypeError` | `can only concatenate str (not "int") to str` | mélange de types : texte + nombre |
| `ValueError` | `invalid literal for int() with base 10: 'douze'` | bon type, mauvaise valeur |
| `IndexError` | `list index out of range` | indice trop grand dans une liste |
| `KeyError` | `'b'` | clé absente d'un dictionnaire |
| `AttributeError` | `'NoneType' object has no attribute 'upper'` | la variable vaut `None` |
| `ZeroDivisionError` | `division by zero` | diviser par zéro (liste vide…) |
| `FileNotFoundError` | `No such file or directory: 'absent.txt'` | mauvais nom ou mauvais dossier |
| `ModuleNotFoundError` | `No module named 'monmodule'` | module pas installé ou mal nommé |
| `IndentationError` | `expected an indented block` | décalage oublié après `:` |

Pour attraper et gérer ces erreurs, voyez [les erreurs et exceptions](erreurs.md).

## Une enquête pas à pas

Voici une méthode qui marche pour tous les bugs, appliquée à une vraie enquête. Une
fonction `moyenne` doit ignorer les élèves absents (notés `None`).

```python
def moyenne(notes):
    total = 0
    for note in notes:
        if note is not None:
            total += note
    return total / len(notes)


print(moyenne([12, None, 18]))
```

```sortie
10.0
```

Le programme ne plante pas, mais la moyenne de 12 et 18 vaut 15, pas 10.

**1. Reproduire.** On a un appel précis qui donne un résultat faux : `[12, None, 18]`
donne `10.0` au lieu de `15.0`. C'est notre point de départ.

**2. Observer.** On affiche les valeurs juste avant le calcul :

```python
def moyenne(notes):
    total = 0
    for note in notes:
        if note is not None:
            total += note
    print(f"{total=} {len(notes)=}")
    return total / len(notes)


print(moyenne([12, None, 18]))
```

```sortie
total=30 len(notes)=3
10.0
```

**3. Faire une hypothèse.** Le total est bon (30). Mais on divise par 3, alors qu'il
n'y a que 2 notes : `len(notes)` compte aussi l'absent.

**4. Corriger.** On compte seulement les vraies notes. Au passage, on gère le cas où
tous les élèves sont absents, qui ferait une division par zéro.

```python
def moyenne(notes):
    presentes = [note for note in notes if note is not None]
    if not presentes:
        return None
    return sum(presentes) / len(presentes)


print(moyenne([12, None, 18]))
print(moyenne([None, None]))
```

```sortie
15.0
None
```

**5. Protéger.** On ajoute des tests pour que le bug ne revienne jamais (voir
[tester son code](tests.md)).

```python
def moyenne(notes):
    presentes = [note for note in notes if note is not None]
    if not presentes:
        return None
    return sum(presentes) / len(presentes)


assert moyenne([12, None, 18]) == 15.0
assert moyenne([None, None]) is None
assert moyenne([10]) == 10.0
print("Tests OK")
```

```sortie
Tests OK
```

> **Astuce :** expliquez votre code ligne par ligne à quelqu'un, ou même à un canard en
> plastique. En le disant à voix haute, on trouve souvent le bug tout seul.

## Erreurs fréquentes

### Chercher l'erreur en haut de la trace

La première ligne de la trace est l'appel le plus **ancien**, souvent dans votre
programme principal. L'erreur se trouve en bas.

```python norun
def moyenne(notes):
    total = 0
    for note in notes:
        total += note
    return total / len(notes)


print(moyenne([12, 15, "18"]))
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 8, in <module>
    print(moyenne([12, 15, "18"]))
          ~~~~~~~^^^^^^^^^^^^^^^^
  File "/home/alice/main.py", line 4, in moyenne
    total += note
TypeError: unsupported operand type(s) for +=: 'int' and 'str'
```

Partez du bas : ligne 4, on ajoute un texte (`str`) à un nombre (`int`). La cause est
le `"18"` entre guillemets. Écrivez `18`, ou convertissez avec `int(note)`.

### Des messages logging.debug qui ne s'affichent pas

Sans réglage, `logging` n'affiche que les niveaux `WARNING` et au-dessus.

```python norun
import logging

logging.debug("Calcul de la moyenne")
logging.warning("Note manquante")
print("fin")
```

```text
WARNING:root:Note manquante
fin
```

Le message `DEBUG` a disparu. Ajoutez `logging.basicConfig(level=logging.DEBUG)` au
début du programme.

### Oublier un breakpoint() dans le code

Un `breakpoint()` oublié arrête le programme chez tous ceux qui le lancent :

```text
> /home/alice/main.py(6)moyenne()
-> breakpoint()
(Pdb)
```

Tapez `c` pour continuer, puis supprimez la ligne. Avant de partager votre code,
cherchez les `breakpoint()` et les `print` de débogage qui traînent.

## Testez-vous

**1.** Dans une trace d'erreur, où se trouve le type de l'erreur ?

> **Réponse**
>
> Sur la dernière ligne, par exemple `KeyError: 'quantite'`. Le bloc juste au-dessus
> indique le fichier et la ligne où elle a éclaté.

**2.** Qu'affiche `print(f"{age=}")` si `age` vaut `30` ?

> **Réponse**
>
> `age=30` : le nom de la variable, un signe égal et sa valeur.

**3.** Quelle commande de pdb exécute la ligne suivante ? Et laquelle continue jusqu'à la
fin ?

> **Réponse**
>
> `n` (*next*) exécute la ligne suivante. `c` (*continue*) reprend le programme jusqu'à
> la prochaine pause ou jusqu'à la fin.

## En résumé

- Lisez une trace d'erreur de bas en haut : le type d'erreur, puis la ligne, puis le
  chemin des appels.
- `print(f"{x=}")` affiche le nom et la valeur ; `repr(x)` et `type(x)` révèlent les
  textes déguisés en nombres.
- `breakpoint()` met le programme en pause dans pdb : `p`, `n`, `s`, `c`, `q`.
- `logging` remplace les `print` de débogage : un niveau par message, un seul réglage.
- Reproduire, observer, faire une hypothèse, corriger, protéger avec un test.

Suite : [Bien écrire du Python](style.md)
