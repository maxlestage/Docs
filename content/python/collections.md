# Listes et tuples

Cette page explique comment ranger plusieurs valeurs ensemble en Python, dans une liste modifiable ou dans un tuple figé.

> **Objectifs :**
>
> - vous saurez créer une liste, lire ses éléments et en extraire une tranche ;
> - vous saurez ajouter, insérer, retirer et trier des éléments ;
> - vous saurez éviter le piège des deux noms pour une même liste ;
> - vous saurez utiliser un tuple et le découper en variables ;
> - vous saurez transformer un `range` en liste.

## Créer une liste

Une [liste](../debuter/glossaire.md#liste) range plusieurs valeurs dans un ordre précis.
Imaginez une liste de courses : une feuille avec des lignes numérotées. On l'écrit entre
crochets `[ ]`, avec des virgules entre les éléments.

```python
courses = ["pain", "lait", "œufs"]
notes = [12, 15, 9, 18]
vide = []
melange = ["Alice", 30, True]

print(courses)
print(len(notes))
print(vide, len(vide))
print(melange)
```

```sortie
['pain', 'lait', 'œufs']
4
[] 0
['Alice', 30, True]
```

Une liste peut contenir des valeurs de types différents. En pratique, on y range
souvent des valeurs de même nature : des notes, des noms, des prix.

## Lire un élément

Comme pour un texte, chaque élément a un [indice](../debuter/glossaire.md#indice), qui
commence à **0**. Les indices négatifs partent de la fin.

```python
couleurs = ["rouge", "vert", "bleu", "jaune"]
print(couleurs[0])
print(couleurs[2])
print(couleurs[-1])
```

```sortie
rouge
bleu
jaune
```

Contrairement à un texte, une liste est [mutable](../debuter/glossaire.md#mutable) :
on peut changer un élément.

```python
couleurs = ["rouge", "vert", "bleu"]
couleurs[1] = "violet"
print(couleurs)
```

```sortie
['rouge', 'violet', 'bleu']
```

## Les tranches

Une tranche `liste[début:fin]` renvoie une **nouvelle** liste avec une partie des
éléments. La fin est exclue, exactement comme pour [le texte](texte.md).

```python
nombres = [10, 20, 30, 40, 50, 60]
print(nombres[1:4])
print(nombres[:3])
print(nombres[-2:])
print(nombres[::2])
print(nombres[::-1])
```

```sortie
[20, 30, 40]
[10, 20, 30]
[50, 60]
[10, 30, 50]
[60, 50, 40, 30, 20, 10]
```

## Ajouter et retirer des éléments

| Méthode | Effet |
| --- | --- |
| `append(x)` | ajoute `x` à la fin |
| `insert(i, x)` | insère `x` à l'indice `i` |
| `extend(autre)` | ajoute tous les éléments d'une autre liste |
| `remove(x)` | retire le premier `x` trouvé |
| `pop()` | retire et renvoie le dernier élément |
| `pop(i)` | retire et renvoie l'élément d'indice `i` |

```python
file = ["Alice", "Bob"]
file.append("Chloé")
print(file)
file.insert(0, "Zoé")
print(file)
file.remove("Bob")
print(file)
dernier = file.pop()
print(dernier, file)
premier = file.pop(0)
print(premier, file)
file.extend(["David", "Emma"])
print(file)
```

```sortie
['Alice', 'Bob', 'Chloé']
['Zoé', 'Alice', 'Bob', 'Chloé']
['Zoé', 'Alice', 'Chloé']
Chloé ['Zoé', 'Alice']
Zoé ['Alice']
['Alice', 'David', 'Emma']
```

Ces méthodes modifient la liste **sur place** : elles changent la liste elle-même.

## Chercher et mesurer

```python
notes = [12, 15, 9, 18, 15]
print(15 in notes)
print(20 in notes)
print(notes.index(18))
print(notes.count(15))
print(len(notes), sum(notes), min(notes), max(notes))
print(sum(notes) / len(notes))
```

```sortie
True
False
3
2
5 69 9 18
13.8
```

- `in` vérifie si une valeur est présente ;
- `index` donne la position de la première occurrence ;
- `count` compte les occurrences ;
- `len`, `sum`, `min` et `max` donnent la taille, la somme, le plus petit et le plus
  grand. La moyenne, c'est `sum / len`.

## Trier : sort et sorted

Il y a deux façons de trier :

- `liste.sort()` trie la liste **sur place** et ne renvoie rien (`None`) ;
- `sorted(liste)` renvoie une **nouvelle** liste triée et laisse l'originale intacte.

```python
notes = [12, 15, 9, 18]
triees = sorted(notes)
print(triees)
print(notes)

notes.sort()
print(notes)

notes.sort(reverse=True)
print(notes)
```

```sortie
[9, 12, 15, 18]
[12, 15, 9, 18]
[9, 12, 15, 18]
[18, 15, 12, 9]
```

`reverse=True` trie du plus grand au plus petit.

### Trier selon un critère avec key

`key` reçoit une fonction. Python l'applique à chaque élément et trie selon le
résultat. C'est comme dire « range les élèves **par taille** » au lieu de « par nom ».

```python
mots = ["kiwi", "Ananas", "fraise", "pomme"]
print(sorted(mots))
print(sorted(mots, key=str.lower))
print(sorted(mots, key=len))

eleves = [("Alice", 15), ("Bob", 12), ("Chloé", 18)]
print(sorted(eleves, key=lambda e: e[1], reverse=True))
```

```sortie
['Ananas', 'fraise', 'kiwi', 'pomme']
['Ananas', 'fraise', 'kiwi', 'pomme']
['kiwi', 'pomme', 'Ananas', 'fraise']
[('Chloé', 18), ('Alice', 15), ('Bob', 12)]
```

Les `lambda` sont expliquées dans [les fonctions](fonctions.md).

> **Attention :** par défaut, les majuscules passent avant les minuscules (`"Z"` avant
> `"a"`). `key=str.lower` trie sans tenir compte de la casse.

### Inverser l'ordre

`reverse()` retourne la liste sur place. `reversed()` ou `[::-1]` donnent une version à
l'envers sans toucher l'originale.

```python
etapes = [1, 2, 3]
print(etapes[::-1])
print(list(reversed(etapes)))
etapes.reverse()
print(etapes)
```

```sortie
[3, 2, 1]
[3, 2, 1]
[3, 2, 1]
```

## Parcourir une liste

La boucle `for` parcourt les éléments un par un. Elle est expliquée dans
[conditions et boucles](conditions-boucles.md).

```python
prix = [4.5, 12.0, 3.25]
total = 0
for p in prix:
    total += p
print(f"Total : {total:.2f} €")
```

```sortie
Total : 19.75 €
```

## Les listes imbriquées

Une liste peut contenir d'autres listes. C'est pratique pour un tableau à deux
dimensions : une grille, un plateau de jeu.

```python
grille = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
]
print(grille[1])
print(grille[1][2])
for ligne in grille:
    print(ligne[0], ligne[1], ligne[2])
```

```sortie
[4, 5, 6]
6
1 2 3
4 5 6
7 8 9
```

`grille[1][2]` se lit en deux temps : la ligne d'indice 1 (`[4, 5, 6]`), puis
l'élément d'indice 2 de cette ligne (`6`).

## Copier ou partager ? Le piège classique

Écrire `b = a` ne copie **pas** la liste. Cela colle une deuxième étiquette sur la
**même** liste. Si vous modifiez par `b`, `a` change aussi : c'est un **alias**.

```python
a = [1, 2, 3]
b = a
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3, 4]
[1, 2, 3, 4]
True
```

`is` vérifie si deux noms désignent le même objet. Ici, oui : il n'y a qu'une liste.

Pour une vraie copie, utilisez `list(a)`, `a.copy()` ou `a[:]` :

```python
a = [1, 2, 3]
b = a.copy()
b.append(4)
print(a)
print(b)
print(a is b)
```

```sortie
[1, 2, 3]
[1, 2, 3, 4]
False
```

> **Attention :** `copy()` fait une copie **superficielle**. Pour une liste de listes,
> les listes intérieures restent partagées. Utilisez alors `copy.deepcopy` (module
> `copy`).

```python
import copy

grille = [[0, 0], [0, 0]]
superficielle = grille.copy()
profonde = copy.deepcopy(grille)
grille[0][0] = 9
print(superficielle)
print(profonde)
```

```sortie
[[9, 0], [0, 0]]
[[0, 0], [0, 0]]
```

## Les tuples

Un [tuple](../debuter/glossaire.md#tuple) ressemble à une liste, mais il est **figé** :
on ne peut ni ajouter, ni retirer, ni changer d'élément. On l'écrit avec des
parenthèses.

```python
point = (3, 4)
couleur = (255, 128, 0)
seul = (42,)

print(point[0], point[1])
print(len(couleur))
print(type(seul))
```

```sortie
3 4
3
<class 'tuple'>
```

Un tuple d'un seul élément a besoin d'une virgule : `(42,)`. Sans elle, `(42)` est
juste le nombre 42 entre parenthèses.

On utilise un tuple pour un groupe de valeurs qui vont ensemble et ne changent pas : des
coordonnées, une date, une couleur RVB.

### Découper un tuple

Le **déballage** (*unpacking*) range chaque élément dans sa propre variable, en une
seule ligne.

```python
point = (3, 4)
x, y = point
print(f"x = {x}, y = {y}")

a, b = 1, 2
a, b = b, a
print(a, b)

premier, *reste = [10, 20, 30, 40]
print(premier, reste)
```

```sortie
x = 3, y = 4
2 1
10 [20, 30, 40]
```

- `a, b = b, a` échange deux variables, sans variable temporaire ;
- `*reste` ramasse tous les éléments restants dans une liste.

## range et les listes

`range` produit une suite de nombres à la demande, sans la stocker. Pour obtenir une
vraie liste, passez-le à `list()`.

```python
print(list(range(5)))
print(list(range(2, 11, 2)))
print(sum(range(1, 101)))
```

```sortie
[0, 1, 2, 3, 4]
[2, 4, 6, 8, 10]
5050
```

`sum(range(1, 101))` additionne les nombres de 1 à 100 sans créer de liste.

## Erreurs fréquentes

### Un indice trop grand

```python norun
fruits = ["pomme", "poire"]
print(fruits[2])
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(fruits[2])
          ~~~~~~^^^
IndexError: list index out of range
```

La liste a 2 éléments, aux indices 0 et 1. Le dernier est `fruits[-1]`.

### Ranger le résultat de sort

```python norun
notes = [12, 8, 15]
triees = notes.sort()
print(triees)
```

```text
None
```

`sort()` trie sur place et renvoie `None`. Écrivez `triees = sorted(notes)`, ou
appelez `notes.sort()` puis utilisez `notes`.

### Retirer un élément absent

```python norun
fruits = ["pomme", "poire"]
fruits.remove("kiwi")
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    fruits.remove("kiwi")
    ~~~~~~~~~~~~~^^^^^^^^
ValueError: list.remove(x): x not in list
```

`"kiwi"` n'est pas dans la liste. Vérifiez d'abord : `if "kiwi" in fruits:`.

### Modifier un tuple

```python norun
point = (3, 4)
point[0] = 10
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    point[0] = 10
    ~~~~~^^^
TypeError: 'tuple' object does not support item assignment
```

Un tuple est figé. Créez-en un nouveau (`point = (10, point[1])`), ou utilisez une
liste si les valeurs doivent changer.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
a = ["x"]
b = a
b.append("y")
print(len(a))
```

> **Réponse**
>
> `2`. `a` et `b` désignent la même liste : ajouter par `b` modifie aussi `a`.

**Question 2 :** quelle est la différence entre `notes.sort()` et `sorted(notes)` ?

> **Réponse**
>
> `notes.sort()` modifie la liste et renvoie `None`. `sorted(notes)` renvoie une
> nouvelle liste triée et laisse `notes` intacte.

**Question 3 :** affichez les trois meilleures notes de `[12, 17, 9, 15, 19, 11]`.

> **Réponse**
>
> ```python
> notes = [12, 17, 9, 15, 19, 11]
> print(sorted(notes, reverse=True)[:3])
> ```
>
> ```sortie
> [19, 17, 15]
> ```

## En résumé

- Une liste `[...]` range des valeurs dans l'ordre ; on la lit avec `liste[i]` et
  `liste[a:b]`.
- `append`, `insert`, `remove` et `pop` la modifient sur place.
- `sorted` renvoie une nouvelle liste triée ; `sort` trie sur place ; `key` choisit le
  critère.
- `b = a` partage la liste ; `a.copy()` la copie.
- Un tuple `(...)` est une liste figée ; `x, y = point` le découpe.
- `list(range(...))` transforme une suite de nombres en liste.

Suite : [dictionnaires et ensembles](dictionnaires.md). Comparez avec
[les collections en Ruby](../ruby/collections.md).
