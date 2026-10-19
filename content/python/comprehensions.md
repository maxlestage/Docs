# Les compréhensions

Cette page montre comment construire une liste, un dictionnaire ou un ensemble en une seule ligne grâce aux compréhensions, et quand il vaut mieux s'en passer.

> **Objectifs :**
>
> - vous saurez transformer une boucle `for` en compréhension de liste ;
> - vous saurez filtrer des éléments avec `if` ;
> - vous saurez écrire des compréhensions de dictionnaire et d'ensemble ;
> - vous saurez utiliser une expression génératrice avec `sum`, `any` et `all` ;
> - vous saurez reconnaître une compréhension trop compliquée.

## De la boucle à la compréhension

Vous savez déjà construire une nouvelle [liste](../debuter/glossaire.md#liste) avec une
boucle `for` : on part d'une liste vide, puis on ajoute un élément à chaque tour.

```python
nombres = [1, 2, 3, 4]
carres = []
for n in nombres:
    carres.append(n * n)
print(carres)
```

```sortie
[1, 4, 9, 16]
```

Ce motif est si courant que Python a une écriture plus courte : la **compréhension de
liste**. On la construit en trois étapes, à partir de la boucle :

1. on écrit des crochets `[ ]` : le résultat sera une liste ;
2. on met d'abord **ce qu'on ajoute** : `n * n` ;
3. on recopie ensuite la ligne `for` sans les deux-points : `for n in nombres`.

```python
nombres = [1, 2, 3, 4]
carres = [n * n for n in nombres]
print(carres)
```

```sortie
[1, 4, 9, 16]
```

Lisez-la à voix haute : « la liste des `n * n`, pour chaque `n` dans `nombres` ». C'est
presque une phrase en français.

> **À retenir :** une compréhension crée **toujours une nouvelle liste**. La liste de
> départ n'est pas modifiée.

### N'importe quelle expression

À gauche du `for`, vous pouvez mettre n'importe quelle
[expression](../debuter/glossaire.md#expression) : un calcul, un appel de méthode, une
f-string…

```python
prenoms = ["alice", "bob", "chloé"]
print([p.capitalize() for p in prenoms])
print([len(p) for p in prenoms])
print([f"Bonjour {p} !" for p in prenoms])
```

```sortie
['Alice', 'Bob', 'Chloé']
[5, 3, 5]
['Bonjour alice !', 'Bonjour bob !', 'Bonjour chloé !']
```

## Filtrer avec if

Souvent, on ne garde que certains éléments. Avec une boucle, on écrit un `if` dans le
`for` :

```python
notes = [12, 7, 15, 9, 18]
reussites = []
for note in notes:
    if note >= 10:
        reussites.append(note)
print(reussites)
```

```sortie
[12, 15, 18]
```

Dans une compréhension, le `if` se place **à la fin**, après le `for` :

```python
notes = [12, 7, 15, 9, 18]
reussites = [note for note in notes if note >= 10]
print(reussites)
```

```sortie
[12, 15, 18]
```

C'est un tamis : seuls les éléments qui passent le test arrivent dans la nouvelle liste.

### Transformer selon une condition

Pour **garder tous les éléments** mais les transformer différemment, utilisez
l'expression conditionnelle `a if condition else b`, placée **avant** le `for` :

```python
notes = [12, 7, 15, 9]
mentions = ["reçu" if note >= 10 else "recalé" for note in notes]
print(mentions)
```

```sortie
['reçu', 'recalé', 'reçu', 'recalé']
```

> **Attention :** les deux places ne veulent pas dire la même chose. Un `if` **après** le
> `for` filtre (la liste peut raccourcir). Un `if … else` **avant** le `for` choisit une
> valeur (la liste garde la même longueur).

## Compréhensions de dictionnaire

Le même principe fonctionne pour un
[dictionnaire](../debuter/glossaire.md#dictionnaire). On utilise des accolades `{ }` et
on écrit `clé: valeur` avant le `for`.

```python
prenoms = ["alice", "bob", "chloé"]
longueurs = {p: len(p) for p in prenoms}
print(longueurs)
```

```sortie
{'alice': 5, 'bob': 3, 'chloé': 5}
```

Pour parcourir un dictionnaire existant, utilisez `.items()`, qui donne les paires
`(clé, valeur)` :

```python
prix = {"pomme": 2.0, "poire": 3.0, "kiwi": 0.5}
soldes = {fruit: p * 0.8 for fruit, p in prix.items()}
pas_chers = {fruit: p for fruit, p in prix.items() if p < 2.5}
print(soldes)
print(pas_chers)
```

```sortie
{'pomme': 1.6, 'poire': 2.4000000000000004, 'kiwi': 0.4}
{'pomme': 2.0, 'kiwi': 0.5}
```

> **Note :** `2.4000000000000004` n'est pas un bug de la compréhension : c'est l'arrondi
> des nombres à virgule (voir [les nombres](nombres.md)). Utilisez `round(p * 0.8, 2)`
> pour un affichage propre.

### Inverser un dictionnaire

Un usage classique : échanger les clés et les valeurs.

```python
codes = {"FR": "France", "BE": "Belgique"}
pays_vers_code = {nom: code for code, nom in codes.items()}
print(pays_vers_code)
```

```sortie
{'France': 'FR', 'Belgique': 'BE'}
```

## Compréhensions d'ensemble

Avec des accolades mais **sans** deux-points, on obtient un
[ensemble](../debuter/glossaire.md#ensemble) (`set`) : chaque valeur n'y apparaît
qu'une fois.

```python
mots = ["Chat", "chien", "CHAT", "Poisson", "chien"]
uniques = {m.lower() for m in mots}
print(sorted(uniques))
```

```sortie
['chat', 'chien', 'poisson']
```

Un ensemble n'a pas d'ordre : on le trie avec `sorted` pour un affichage stable.

| Écriture | Résultat |
|----------|----------|
| `[x for x in ...]` | une liste |
| `{x for x in ...}` | un ensemble |
| `{k: v for ... in ...}` | un dictionnaire |
| `(x for x in ...)` | une expression génératrice (voir plus bas) |

## Compréhensions imbriquées

Une compréhension peut contenir **plusieurs `for`**. Ils se lisent dans le même ordre
que des boucles imbriquées : le premier `for` est la boucle extérieure.

```python
couleurs = ["rouge", "bleu"]
tailles = ["S", "M"]
articles = [f"{c}-{t}" for c in couleurs for t in tailles]
print(articles)
```

```sortie
['rouge-S', 'rouge-M', 'bleu-S', 'bleu-M']
```

C'est l'équivalent de :

```python
couleurs = ["rouge", "bleu"]
tailles = ["S", "M"]
articles = []
for c in couleurs:
    for t in tailles:
        articles.append(f"{c}-{t}")
print(articles)
```

```sortie
['rouge-S', 'rouge-M', 'bleu-S', 'bleu-M']
```

### Aplatir une liste de listes

Deux `for` servent souvent à « aplatir » un tableau en une seule liste :

```python
grille = [[1, 2, 3], [4, 5, 6]]
plat = [n for ligne in grille for n in ligne]
print(plat)
```

```sortie
[1, 2, 3, 4, 5, 6]
```

### Une compréhension dans une compréhension

À l'inverse, une compréhension placée **à gauche** d'une autre construit une liste de
listes. Ici, une table de multiplication :

```python
table = [[ligne * col for col in range(1, 4)] for ligne in range(1, 4)]
for ligne in table:
    print(ligne)
```

```sortie
[1, 2, 3]
[2, 4, 6]
[3, 6, 9]
```

## Les expressions génératrices

Avec des **parenthèses** au lieu de crochets, vous obtenez une **expression
génératrice**. Elle ne construit pas la liste en mémoire : elle produit les valeurs une
par une, au moment où on les demande. C'est un robinet plutôt qu'un seau rempli à
l'avance.

Elle est idéale avec les fonctions qui consomment des valeurs : `sum`, `min`, `max`,
`any`, `all`. Quand elle est le seul argument, les parenthèses de la fonction suffisent.

```python
notes = [12, 7, 15, 9, 18]
print(sum(n * 2 for n in notes))
print(max(len(m) for m in ["un", "trois", "quatre"]))
```

```sortie
122
6
```

### any et all

- `any(...)` vaut `True` si **au moins une** valeur est vraie ;
- `all(...)` vaut `True` si **toutes** les valeurs sont vraies.

```python
notes = [12, 7, 15, 9, 18]
print(any(n >= 18 for n in notes))
print(all(n >= 10 for n in notes))
print(all(n >= 0 for n in notes))
```

```sortie
True
False
True
```

Ces deux fonctions s'arrêtent dès qu'elles connaissent la réponse : `any` s'arrête au
premier `True`, `all` au premier `False`.

> **Astuce :** une expression génératrice ne se parcourt **qu'une seule fois**. Une fois
> vidée, elle ne redonne plus rien. Pour réutiliser les valeurs, faites une liste. Le
> chapitre [itérateurs et générateurs](iterateurs.md) explique pourquoi.

```python
carres = (n * n for n in range(4))
print(sum(carres))
print(sum(carres))
```

```sortie
14
0
```

## Savoir s'arrêter

Une compréhension doit rester **lisible d'un coup d'œil**. Celle-ci est correcte, mais
difficile à relire :

```python
resultat = [x * y for x in range(1, 5) if x % 2 == 0 for y in range(1, 5) if y > x]
print(resultat)
```

```sortie
[6, 8]
```

La même chose avec des boucles est plus longue, mais chaque étape est claire :

```python
resultat = []
for x in range(1, 5):
    if x % 2 == 0:
        for y in range(1, 5):
            if y > x:
                resultat.append(x * y)
print(resultat)
```

```sortie
[6, 8]
```

Quelques règles simples :

- **une** boucle et **un** filtre : la compréhension est parfaite ;
- deux `for` : acceptable si c'est court ;
- au-delà, ou si la ligne dépasse l'écran : revenez à une boucle ;
- si vous avez besoin d'un `print` ou d'une action (pas d'une valeur), utilisez une
  boucle : une compréhension sert à **construire** une collection.

Vous pouvez aussi sortir le calcul dans une [fonction](fonctions.md) avec un nom clair :

```python
def est_premier(n):
    return n > 1 and all(n % d != 0 for d in range(2, n))

print([n for n in range(20) if est_premier(n)])
```

```sortie
[2, 3, 5, 7, 11, 13, 17, 19]
```

## Erreurs fréquentes

### Mettre un if … else à la fin

```python norun
nombres = [3, -1, 4]
positifs = [n for n in nombres if n > 0 else 0]
print(positifs)
```

```text
  File "main.py", line 2
    positifs = [n for n in nombres if n > 0 else 0]
                                            ^^^^
SyntaxError: invalid syntax
```

Un `if` placé à la fin est un filtre : il n'accepte pas de `else`. Pour choisir une
valeur, placez l'expression avant le `for` : `[n if n > 0 else 0 for n in nombres]`.

### Oublier les deux-points d'un dictionnaire

```python norun
prix = {"pomme": 2, "poire": 3}
doubles = {nom, p * 2 for nom, p in prix.items()}
print(doubles)
```

```text
  File "main.py", line 2
    doubles = {nom, p * 2 for nom, p in prix.items()}
               ^^^^^^^^^^
SyntaxError: did you forget parentheses around the comprehension target?
```

Une virgule entre la clé et la valeur ne crée pas de dictionnaire. Écrivez
`{nom: p * 2 for nom, p in prix.items()}`.

### Utiliser la variable de boucle après coup

```python norun
carres = [n * n for n in range(4)]
print(n)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(n)
          ^
NameError: name 'n' is not defined
```

Contrairement à une boucle `for`, la variable d'une compréhension n'existe **qu'à
l'intérieur** des crochets. Utilisez le résultat (`carres`), pas la variable.

### Prendre un élément d'une expression génératrice

```python norun
carres = (n * n for n in range(4))
print(carres[0])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(carres[0])
          ~~~~~~^^^
TypeError: 'generator' object is not subscriptable
```

Les parenthèses donnent un générateur, pas une liste : il n'a pas d'indice. Utilisez des
crochets `[n * n for n in range(4)]` si vous avez besoin de `carres[0]`.

## Testez-vous

**Question 1 :** réécrivez cette boucle en compréhension.

```python norun
majuscules = []
for mot in ["oui", "non"]:
    majuscules.append(mot.upper())
```

> **Réponse**
>
> ```python
> majuscules = [mot.upper() for mot in ["oui", "non"]]
> print(majuscules)
> ```
>
> ```sortie
> ['OUI', 'NON']
> ```

**Question 2 :** quel est le type de `{x % 3 for x in range(10)}`, et que contient-il ?

> **Réponse**
>
> C'est un ensemble (`set`), car il y a des accolades sans deux-points. Il contient
> `{0, 1, 2}` : les restes possibles, chacun une seule fois.

**Question 3 :** comment savoir si une liste `ages` contient au moins une personne
mineure, sans construire de nouvelle liste ?

> **Réponse**
>
> Avec `any` et une expression génératrice :
>
> ```python
> ages = [34, 17, 52]
> print(any(age < 18 for age in ages))
> ```
>
> ```sortie
> True
> ```

## En résumé

- `[expression for x in collection]` construit une nouvelle liste à partir d'une autre.
- Un `if` à la fin filtre ; un `a if condition else b` au début choisit une valeur.
- `{k: v for ...}` construit un dictionnaire, `{x for ...}` un ensemble.
- `(x for ...)` est une expression génératrice : parfaite avec `sum`, `any`, `all`, et
  utilisable une seule fois.
- Si la compréhension devient difficile à lire, revenez à une boucle `for`.

Suite : [Les pièges du débutant en Python](pieges.md)
