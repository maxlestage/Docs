# Les boucles

Cette page explique comment un programme Python répète des actions avec `while` et `for`, et parcourt des listes avec `range`, `enumerate` et `zip`.

> **Objectifs :**
>
> - vous saurez répéter du code avec `while` et `for` ;
> - vous saurez compter avec `range` ;
> - vous saurez arrêter une boucle avec `break` ou sauter un tour avec `continue` ;
> - vous saurez parcourir une liste avec `enumerate` et `zip`.

Ce chapitre suit [les conditions](conditions-boucles.md) : vous y retrouverez `if` et les comparaisons, cette fois à l'intérieur des boucles.

## La boucle while

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100 `print` serait
long et pénible. Une boucle fait ce travail répétitif à votre place.

Une [boucle](../debuter/glossaire.md#boucle) répète du code. `while` (« tant que »)
répète son bloc **tant que** la condition est vraie.

```python
compte = 3
while compte > 0:
    print(compte)
    compte = compte - 1
print("Décollage !")
```

```sortie
3
2
1
Décollage !
```

Ligne par ligne :

- `compte = 3` : la variable vaut 3 au départ ;
- `while compte > 0:` : avant chaque tour, Python pose la question. `3 > 0` est vrai :
  il entre dans le bloc ;
- le bloc affiche `compte`, puis lui retire 1 : il vaut 2, puis 1, puis 0 ;
- quand `compte` vaut 0, `0 > 0` est faux : Python sort de la boucle et passe au
  dernier `print`.

À chaque tour, Python vérifie `compte > 0`. Quand `compte` vaut 0, la condition est
fausse : la boucle s'arrête.

> **Attention :** si la condition ne devient jamais fausse, la boucle tourne pour
> toujours. Vérifiez qu'une variable change à chaque tour. Pour arrêter un programme
> bloqué, appuyez sur Ctrl+C dans le terminal.

**À vous :** écrivez un programme qui affiche les nombres de 1 à 5 avec une boucle
`while`, puis « Fini ! ».

> **Solution**
>
> ```python
> n = 1
> while n <= 5:
>     print(n)
>     n = n + 1
> print("Fini !")
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Fini !
> ```

## La boucle for

Très souvent, vous avez une liste de choses à traiter une par une : chaque article
d'une liste de courses, chaque élève d'une classe. `for` est fait pour cela.

`for` parcourt les éléments d'une collection, un par un. C'est la boucle la plus
utilisée en Python.

```python
fruits = ["pomme", "banane", "cerise"]
for fruit in fruits:
    print(f"J'aime la {fruit}.")
```

```sortie
J'aime la pomme.
J'aime la banane.
J'aime la cerise.
```

Ligne par ligne :

- `fruits` contient une liste de trois textes ;
- `for fruit in fruits:` prend le premier élément, `"pomme"`, et le range dans `fruit` ;
- le bloc affiche la phrase avec `"pomme"`. Puis Python revient au `for`, qui range
  `"banane"` dans `fruit`, et ainsi de suite ;
- après `"cerise"`, il n'y a plus d'élément : la boucle s'arrête.

À chaque tour, la variable `fruit` reçoit l'élément suivant de la liste. Chaque passage
s'appelle une [itération](../debuter/glossaire.md#iteration).

`for` parcourt aussi un texte, lettre par lettre :

```python
for lettre in "Oui":
    print(lettre)
```

```sortie
O
u
i
```

### Compter avec range

`range` produit une suite de nombres. Il est parfait pour répéter une action un nombre
précis de fois.

- `range(5)` : de 0 à 4 (5 nombres, **sans** le 5) ;
- `range(1, 6)` : de 1 à 5 ;
- `range(0, 10, 2)` : de 0 à 8, de 2 en 2.

```python
for i in range(3):
    print("Tour", i)

print(list(range(1, 6)))
print(list(range(0, 10, 2)))
print(list(range(10, 0, -3)))
```

```sortie
Tour 0
Tour 1
Tour 2
[1, 2, 3, 4, 5]
[0, 2, 4, 6, 8]
[10, 7, 4, 1]
```

`list(...)` transforme le `range` en liste, pour voir tous ses nombres d'un coup. Un pas
négatif (`-3`) compte à rebours.

> **À retenir :** la fin d'un `range` est toujours **exclue**. `range(1, 6)` s'arrête
> à 5.

**À vous :** écrivez un programme qui affiche la table de 7, de « 7 x 1 = 7 » à
« 7 x 5 = 35 », une ligne par calcul.

> **Solution**
>
> ```python
> for i in range(1, 6):
>     print("7 x", i, "=", 7 * i)
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## break et continue

Parfois, vous voulez changer le cours d'une boucle en route : arrêter de chercher dès
que vous avez trouvé, ou ignorer certains éléments. C'est le rôle de ces deux mots.

- `break` sort de la boucle immédiatement ;
- `continue` saute la fin du tour et passe au suivant.

```python
for n in range(1, 10):
    if n % 2 == 0:
        continue
    if n > 7:
        break
    print(n)
```

```sortie
1
3
5
7
```

Ligne par ligne :

- `for n in range(1, 10):` donne à `n` les valeurs 1, 2, 3… jusqu'à 9 ;
- si `n` est pair (le reste de la division par 2 vaut 0), `continue` passe tout de
  suite au tour suivant : le `print` n'est pas atteint ;
- si `n` dépasse 7, `break` arrête toute la boucle : 9 n'est jamais affiché ;
- sinon, `print(n)` affiche le nombre.

Les nombres pairs sont sautés (`continue`). Quand `n` dépasse 7, la boucle s'arrête
(`break`).

### else sur une boucle

Une boucle peut avoir un `else`. Il s'exécute si la boucle va **jusqu'au bout**, sans
`break`. C'est utile pour une recherche : « je n'ai rien trouvé ».

```python
nombres = [3, 7, 11, 15]
for n in nombres:
    if n % 2 == 0:
        print(f"Premier nombre pair : {n}")
        break
else:
    print("Aucun nombre pair.")
```

```sortie
Aucun nombre pair.
```

## enumerate : le numéro et l'élément

Vous voulez parfois numéroter ce que vous parcourez, comme un classement ou les étapes
d'une recette.

Pour avoir à la fois la position et l'élément, utilisez `enumerate`. À chaque tour, il
donne deux valeurs : le numéro (l'[indice](../debuter/glossaire.md#indice)) et
l'élément.

```python
podium = ["Alice", "Bob", "Chloé"]
for rang, nom in enumerate(podium, start=1):
    print(f"{rang}. {nom}")
```

```sortie
1. Alice
2. Bob
3. Chloé
```

Ligne par ligne :

- `podium` contient trois prénoms, dans l'ordre d'arrivée ;
- à chaque tour, `enumerate` donne une paire : d'abord `1` et `"Alice"`, puis `2` et
  `"Bob"`, puis `3` et `"Chloé"` ;
- `rang, nom` range le numéro dans `rang` et le prénom dans `nom` ;
- `start=1` fait commencer la numérotation à 1.

Sans `start=1`, la numérotation commence à 0.

## zip : parcourir deux listes ensemble

Il arrive que deux listes aillent ensemble : des prénoms d'un côté, leurs notes de
l'autre, dans le même ordre.

`zip` avance dans plusieurs listes en même temps, comme une fermeture éclair qui
assemble deux côtés, dent par dent.

```python
prenoms = ["Alice", "Bob", "Chloé"]
notes = [15, 12, 18]
for prenom, note in zip(prenoms, notes):
    print(f"{prenom} : {note}/20")
```

```sortie
Alice : 15/20
Bob : 12/20
Chloé : 18/20
```

Ligne par ligne :

- `prenoms` et `notes` sont dans le même ordre : la première note est celle d'Alice ;
- `zip` prend le premier élément de chaque liste, puis le deuxième, puis le troisième ;
- à chaque tour, `prenom` reçoit un prénom et `note` la note qui va avec.

> **Note :** `zip` s'arrête à la fin de la liste la plus courte.

## Erreurs fréquentes

### Parcourir un nombre avec for

```python norun
for i in 5:
    print(i)
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    for i in 5:
             ^
TypeError: 'int' object is not iterable
```

On ne peut pas parcourir un nombre (*not iterable*). Pour répéter 5 fois, écrivez
`for i in range(5):`.

### Oublier enumerate

```python norun
podium = ["Alice", "Bob", "Chloé"]
for rang, nom in podium:
    print(f"{rang}. {nom}")
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    for rang, nom in podium:
        ^^^^^^^^^
ValueError: too many values to unpack (expected 2)
```

Sans `enumerate`, chaque tour donne un seul élément, le texte `"Alice"`. Python essaie
de le découper en deux variables et trouve trop de lettres (*too many values*). Écrivez
`for rang, nom in enumerate(podium, start=1):`.

### break hors de la boucle

```python norun
for n in [3, 8, 11]:
    if n % 2 == 0:
        print("Nombre pair :", n)
break
```

```text
  File "/home/vous/main.py", line 4
    break
    ^^^^^
SyntaxError: 'break' outside loop
```

`break` n'est pas décalé : il n'est plus dans le bloc du `for` (*outside loop*). Placez-le
sous le `print`, avec la même indentation, pour qu'il fasse partie du bloc du `if`.

## Testez-vous

**Question 1 :** combien de fois s'affiche « coucou » avec `for i in range(2, 8):` ?

> **Réponse**
>
> 6 fois : `i` prend les valeurs 2, 3, 4, 5, 6 et 7. La fin (8) est exclue.

**Question 2 :** écrivez une boucle qui affiche les multiples de 3 de 3 à 15, sur une
ligne.

> **Réponse**
>
> ```python
> for n in range(3, 16, 3):
>     print(n, end=" ")
> print("fin")
> ```
>
> ```sortie
> 3 6 9 12 15 fin
> ```

**Question 3 :** qu'affiche ce code ?

```python norun
n = 1
while n < 100:
    n = n * 2
    if n > 10:
        break
print(n)
```

> **Réponse**
>
> `16`. `n` vaut 2, puis 4, puis 8, puis 16. À 16, `n > 10` est vrai : `break` arrête la
> boucle avant que la condition du `while` ne soit fausse.

## En résumé

- `while` répète tant qu'une condition est vraie ; une variable doit changer à chaque
  tour.
- `for` parcourt une collection ou un `range`.
- La fin d'un `range` est exclue : `range(1, 6)` va de 1 à 5.
- `break` sort, `continue` saute un tour.
- `enumerate` et `zip` simplifient les parcours.

Suite : [les fonctions](fonctions.md).
