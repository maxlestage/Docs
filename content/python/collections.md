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

Ligne par ligne :

- `courses` reçoit une liste de trois textes, rangés dans cet ordre ;
- `vide = []` crée une liste sans rien dedans, prête à être remplie plus tard ;
- `print(courses)` affiche la liste entière, avec ses crochets et ses guillemets ;
- `len(notes)` compte les éléments de `notes` : il y en a 4.

Une liste peut contenir des valeurs de types différents. En pratique, on y range
souvent des valeurs de même nature : des notes, des noms, des prix.

## Lire un élément

Une liste ne sert à rien si vous ne pouvez pas retrouver ce qu'elle contient. Comme
on lit « la troisième ligne » d'une liste de courses, vous demandez à Python l'élément
rangé à une position donnée.

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

Ligne par ligne :

- `couleurs[0]` donne le premier élément, `"rouge"` : on compte à partir de 0 ;
- `couleurs[2]` donne donc le troisième, `"bleu"` ;
- `couleurs[-1]` donne le dernier, `"jaune"`, quelle que soit la longueur de la liste.

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

Parfois, vous ne voulez pas un seul élément, mais un morceau de la liste : les trois
premiers, les deux derniers. C'est comme couper une tranche de pain sans toucher au reste
de la miche.

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

Ligne par ligne :

- `nombres[1:4]` prend les indices 1, 2 et 3 : la fin, 4, est exclue ;
- `nombres[:3]` part du début, et `nombres[-2:]` va jusqu'à la fin ;
- `nombres[::2]` prend un élément sur deux ;
- `nombres[::-1]` parcourt la liste à l'envers.

**À vous :** écrivez un programme qui range les jours de la semaine dans une liste
`jours`, puis affiche le premier jour, le dernier, et la tranche des cinq premiers.

> **Solution**
>
> ```python
> jours = ["lun", "mar", "mer", "jeu", "ven", "sam", "dim"]
> print(jours[0])
> print(jours[-1])
> print(jours[:5])
> ```
>
> ```sortie
> lun
> dim
> ['lun', 'mar', 'mer', 'jeu', 'ven']
> ```

## Ajouter et retirer des éléments

Une liste change souvent pendant la vie d'un programme : vous ajoutez un article aux
courses, vous rayez celui que vous avez acheté. Ces méthodes font ce travail.

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

Ligne par ligne :

- `file.append("Chloé")` ajoute Chloé à la fin : la file contient trois prénoms ;
- `file.insert(0, "Zoé")` place Zoé à l'indice 0, tout devant ; les autres reculent ;
- `file.remove("Bob")` cherche Bob et le retire ;
- `file.pop()` retire le dernier prénom, Chloé, et le range dans `dernier` ;
- `file.extend(["David", "Emma"])` ajoute deux prénoms d'un coup, à la fin.

Ces méthodes modifient la liste **sur place** : elles changent la liste elle-même.

**À vous :** écrivez un programme qui part de `courses = ["pain", "lait"]`, ajoute
« beurre » à la fin, retire « pain », puis affiche la liste.

> **Solution**
>
> ```python
> courses = ["pain", "lait"]
> courses.append("beurre")
> courses.remove("pain")
> print(courses)
> ```
>
> ```sortie
> ['lait', 'beurre']
> ```

## Chercher et mesurer

Une fois la liste remplie, vous voulez lui poser des questions : « le lait est-il sur
la liste ? », « combien y a-t-il de notes ? », « quelle est la meilleure ? ».

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

Trier, c'est mettre de l'ordre : classer des notes de la plus basse à la plus haute,
ranger des prénoms dans l'ordre alphabétique, comme dans un annuaire.

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

Ligne par ligne :

- `sorted(notes)` fabrique une nouvelle liste triée, rangée dans `triees` ;
- `print(notes)` montre que `notes` n'a pas bougé ;
- `notes.sort()` trie cette fois `notes` elle-même ;
- `notes.sort(reverse=True)` la trie à nouveau, dans l'autre sens.

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Souvent, vous voulez faire la même chose avec chaque élément : additionner tous les prix
d'un ticket de caisse, afficher chaque nom d'une classe.

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

Ligne par ligne :

- `total = 0` prépare une variable qui va accumuler la somme ;
- `for p in prix:` donne à `p` la valeur 4.5, puis 12.0, puis 3.25 ;
- à chaque tour, `total += p` ajoute le prix à `total` : 4.5, puis 16.5, puis 19.75 ;
- après la boucle, le `print` affiche le total avec deux chiffres après la virgule.

**À vous :** écrivez un programme qui parcourt `notes = [12, 15, 9]` et affiche chaque
note sur sa propre ligne, précédée de « Note : ».

> **Solution**
>
> ```python
> notes = [12, 15, 9]
> for n in notes:
>     print("Note :", n)
> ```
>
> ```sortie
> Note : 12
> Note : 15
> Note : 9
> ```

## Les listes imbriquées

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Vous voulez parfois garder une liste intacte et travailler sur un double, comme on
photocopie un document avant d'écrire dessus. Encore faut-il avoir vraiment deux listes.

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

Ligne par ligne :

- `a = [1, 2, 3]` crée une liste et lui donne le nom `a` ;
- `b = a` ne crée aucune liste : `b` devient un deuxième nom pour la même ;
- `b.append(4)` ajoute 4 à cette liste unique ;
- les deux `print` montrent donc la même liste, avec le 4 ; `a is b` vaut `True`.

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

Ligne par ligne :

- `point = (3, 4)` crée un tuple de deux nombres ;
- `seul = (42,)` crée un tuple d'un seul élément, grâce à la virgule ;
- `point[0]` et `point[1]` se lisent comme dans une liste ;
- `type(seul)` confirme qu'il s'agit bien d'un tuple.

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

**À vous :** écrivez un programme qui découpe le tuple `date = (14, 7, 1789)` en trois
variables `jour`, `mois` et `annee`, puis affiche « Année : 1789 ».

> **Solution**
>
> ```python
> date = (14, 7, 1789)
> jour, mois, annee = date
> print("Année :", annee)
> ```
>
> ```sortie
> Année : 1789
> ```

## range et les listes

Il faut parfois une liste de nombres qui se suivent : de 1 à 10, ou tous les nombres
pairs jusqu'à 20. Les taper à la main serait long, et on risquerait d'en oublier un.

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

Ligne par ligne :

- `range(5)` donne 0, 1, 2, 3, 4 : il commence à 0 et s'arrête avant 5 ;
- `range(2, 11, 2)` part de 2, avance de 2 en 2 et s'arrête avant 11 ;
- `list(...)` transforme ces nombres en vraie liste, que `print` peut afficher.

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
