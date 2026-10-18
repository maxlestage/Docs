# Fonctions avancées et décorateurs

Cette page montre que les fonctions Python sont des valeurs comme les autres, ce qui permet d'écrire des closures, des décorateurs et d'utiliser les outils du module `functools`.

> **Objectifs :**
>
> - vous saurez ranger une fonction dans une variable et la passer à une autre fonction ;
> - vous saurez ce qu'est une closure ;
> - vous saurez écrire un décorateur pas à pas, avec `functools.wraps` ;
> - vous saurez écrire un décorateur qui prend des arguments ;
> - vous saurez utiliser `lru_cache`, `partial`, `map`, `filter` et `reduce`.

## Les fonctions sont des objets

En Python, une [fonction](../debuter/glossaire.md#fonction) est une valeur, comme un
nombre ou une chaîne. Sans parenthèses, `saluer` désigne **la fonction elle-même** ;
avec parenthèses, `saluer()` **l'exécute**.

```python
def saluer(nom):
    return f"Bonjour {nom}"

dire = saluer
print(dire("Alice"))
print(saluer.__name__)
print(type(saluer))
```

```sortie
Bonjour Alice
saluer
<class 'function'>
```

### Passer une fonction en argument

Puisqu'une fonction est une valeur, on peut la donner à une autre fonction. Vous
l'avez déjà fait avec `sorted(..., key=len)`.

```python
def appliquer(fonction, valeurs):
    return [fonction(v) for v in valeurs]

def double(x):
    return x * 2

print(appliquer(double, [1, 2, 3]))
print(appliquer(str.upper, ["a", "b"]))
print(appliquer(lambda x: x ** 2, [1, 2, 3]))
```

```sortie
[2, 4, 6]
['A', 'B']
[1, 4, 9]
```

`lambda x: x ** 2` est une [lambda](../debuter/glossaire.md#lambda) : une petite
fonction sans nom, écrite sur une ligne, qui renvoie une seule expression.

### Renvoyer une fonction

Une fonction peut aussi **fabriquer** et renvoyer une autre fonction :

```python
def multiplicateur(facteur):
    def multiplier(x):
        return x * facteur
    return multiplier

triple = multiplicateur(3)
print(triple(5))
print(multiplicateur(10)(4))
```

```sortie
15
40
```

## Les closures

Dans l'exemple précédent, `multiplier` utilise `facteur`, une variable de la fonction
qui l'a créée. Même quand `multiplicateur` a fini, `triple` **se souvient** que
`facteur` vaut 3. Une fonction qui emporte ainsi des variables avec elle s'appelle une
[closure](../debuter/glossaire.md#closure) (fermeture).

C'est un sac à dos : la fonction intérieure part avec les variables dont elle a besoin.

Pour **modifier** une variable emportée, il faut le dire avec `nonlocal` :

```python
def creer_compteur():
    total = 0
    def compter():
        nonlocal total
        total += 1
        return total
    return compter

a = creer_compteur()
b = creer_compteur()
print(a(), a(), a())
print(b())
```

```sortie
1 2 3
1
```

Chaque appel à `creer_compteur` crée un nouveau sac à dos : `a` et `b` ont chacun leur
propre `total`.

## Écrire un décorateur, pas à pas

Un **décorateur** est une fonction qui prend une fonction et renvoie une version
améliorée de celle-ci. C'est un papier cadeau : le cadeau reste le même, mais on ajoute
quelque chose autour.

### Étape 1 : une fonction qui enveloppe

Écrivons un décorateur qui compte les appels. Il reçoit `fonction`, crée une fonction
`enveloppe` qui fait un travail en plus puis appelle `fonction`, et renvoie
`enveloppe`.

```python
def compter_appels(fonction):
    def enveloppe(*args, **kwargs):
        enveloppe.appels += 1
        return fonction(*args, **kwargs)
    enveloppe.appels = 0
    return enveloppe

def addition(a, b):
    return a + b

addition = compter_appels(addition)
print(addition(2, 3))
print(addition(10, 1))
print(addition.appels)
```

```sortie
5
11
2
```

`*args, **kwargs` signifie « tous les arguments, quels qu'ils soient » : l'enveloppe les
transmet tels quels. Elle fonctionne donc avec n'importe quelle fonction.

### Étape 2 : la syntaxe @

`addition = compter_appels(addition)` est si courant que Python a un raccourci : on
écrit `@compter_appels` juste au-dessus de la définition.

```python
def compter_appels(fonction):
    def enveloppe(*args, **kwargs):
        enveloppe.appels += 1
        return fonction(*args, **kwargs)
    enveloppe.appels = 0
    return enveloppe

@compter_appels
def saluer(nom):
    return f"Salut {nom}"

saluer("Alice")
saluer("Bob")
saluer("Chloé")
print(saluer.appels)
```

```sortie
3
```

> **À retenir :** `@decorateur` au-dessus de `def f` veut exactement dire
> `f = decorateur(f)`. Rien de plus.

### Un décorateur de journal

Autre usage typique : afficher chaque appel et son résultat, pour déboguer.

```python
def journaliser(fonction):
    def enveloppe(*args, **kwargs):
        print(f"→ {fonction.__name__}{args}")
        resultat = fonction(*args, **kwargs)
        print(f"← {resultat!r}")
        return resultat
    return enveloppe

@journaliser
def multiplier(a, b):
    return a * b

multiplier(6, 7)
```

```sortie
→ multiplier(6, 7)
← 42
```

### Un décorateur de chronométrage

Le même schéma sert à mesurer la durée d'une fonction avec `time.perf_counter()`. La
durée change à chaque exécution, donc la sortie ci-dessous n'est qu'un exemple :

```python norun
import time

def chronometrer(fonction):
    def enveloppe(*args, **kwargs):
        debut = time.perf_counter()
        resultat = fonction(*args, **kwargs)
        duree = time.perf_counter() - debut
        print(f"{fonction.__name__} : {duree:.3f} s")
        return resultat
    return enveloppe

@chronometrer
def calcul_long():
    return sum(range(10_000_000))

calcul_long()
```

```text
calcul_long : 0.142 s
```

## functools.wraps

Un petit défaut : après décoration, la fonction s'appelle `enveloppe`. Son nom et sa
documentation d'origine sont perdus.

```python
def decorer(fonction):
    def enveloppe(*args, **kwargs):
        return fonction(*args, **kwargs)
    return enveloppe

@decorer
def saluer():
    """Dit bonjour."""

print(saluer.__name__, saluer.__doc__)
```

```sortie
enveloppe None
```

`@functools.wraps(fonction)`, posé sur l'enveloppe, recopie le nom, la documentation et
les autres informations de la fonction d'origine. Utilisez-le **toujours**.

```python
import functools

def decorer(fonction):
    @functools.wraps(fonction)
    def enveloppe(*args, **kwargs):
        return fonction(*args, **kwargs)
    return enveloppe

@decorer
def saluer():
    """Dit bonjour."""

print(saluer.__name__, saluer.__doc__)
```

```sortie
saluer Dit bonjour.
```

## Un décorateur avec des arguments

Pour écrire `@repeter(3)`, il faut un étage de plus : `repeter(3)` doit **renvoyer un
décorateur**. On obtient trois fonctions imbriquées :

1. `repeter(fois)` reçoit l'argument et renvoie le décorateur ;
2. `decorateur(fonction)` reçoit la fonction et renvoie l'enveloppe ;
3. `enveloppe(*args)` fait le travail.

```python
import functools

def repeter(fois):
    def decorateur(fonction):
        @functools.wraps(fonction)
        def enveloppe(*args, **kwargs):
            for _ in range(fois):
                fonction(*args, **kwargs)
        return enveloppe
    return decorateur

@repeter(3)
def crier(mot):
    print(mot.upper())

crier("hip")
```

```sortie
HIP
HIP
HIP
```

`@repeter(3)` équivaut à `crier = repeter(3)(crier)`.

## lru_cache : mémoriser les résultats

`functools.lru_cache` est un décorateur tout prêt : il garde en mémoire les résultats
déjà calculés. Si la fonction est rappelée avec les mêmes arguments, le résultat est
rendu immédiatement, sans recalcul.

C'est très efficace pour une fonction [récursive](../debuter/glossaire.md#recursion)
comme Fibonacci, qui recalcule sans cesse les mêmes valeurs :

```python
from functools import lru_cache

appels = 0

@lru_cache(maxsize=None)
def fib(n):
    global appels
    appels += 1
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(80))
print(appels)
```

```sortie
23416728348467685
81
```

Seulement 81 calculs, un par valeur de `n`. Sans le cache, il en faudrait des milliards
de milliards.

> **Attention :** le cache fonctionne seulement si les arguments sont *hachables* :
> nombres, chaînes, tuples… mais pas des listes ou des dictionnaires.

## partial : fixer des arguments

`functools.partial` crée une nouvelle fonction où certains arguments sont **déjà
remplis**.

```python
from functools import partial

def puissance(base, exposant):
    return base ** exposant

carre = partial(puissance, exposant=2)
cube = partial(puissance, exposant=3)
print(carre(5), cube(2))

binaire_vers_int = partial(int, base=2)
print(binaire_vers_int("1010"))
```

```sortie
25 8
10
```

## map, filter, reduce ou compréhensions ?

Trois fonctions classiques appliquent une fonction à une collection :

- `map(f, valeurs)` applique `f` à chaque valeur ;
- `filter(f, valeurs)` garde les valeurs pour lesquelles `f` renvoie vrai ;
- `reduce(f, valeurs)` (du module `functools`) combine les valeurs deux à deux jusqu'à
  n'en garder qu'une.

```python
from functools import reduce

nombres = [1, 2, 3, 4, 5]
print(list(map(lambda x: x * 10, nombres)))
print(list(filter(lambda x: x % 2 == 0, nombres)))
print(reduce(lambda acc, x: acc * x, nombres))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
120
```

`map` et `filter` renvoient des itérateurs paresseux : on les transforme en liste avec
`list`. En Python, les [compréhensions](comprehensions.md) font souvent la même chose
de façon plus lisible :

```python
nombres = [1, 2, 3, 4, 5]
print([x * 10 for x in nombres])
print([x for x in nombres if x % 2 == 0])
print(sum(nombres), max(nombres))
```

```sortie
[10, 20, 30, 40, 50]
[2, 4]
15 5
```

| Besoin | Le plus lisible |
|--------|-----------------|
| transformer avec une fonction existante | `map(str, nombres)` ou une compréhension |
| transformer avec un calcul | compréhension |
| filtrer | compréhension avec `if` |
| additionner, max, min | `sum`, `max`, `min` |
| autre combinaison | `reduce`, ou une boucle |

## Erreurs fréquentes

### Oublier de renvoyer l'enveloppe

```python norun
def annoncer(fonction):
    def enveloppe():
        print("Appel !")
        fonction()

@annoncer
def saluer():
    print("Bonjour")

saluer()
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    saluer()
    ~~~~~~^^
TypeError: 'NoneType' object is not callable
```

`annoncer` ne renvoie rien, donc `saluer` vaut `None`. Ajoutez `return enveloppe` à la
fin du décorateur.

### Modifier une variable de closure sans nonlocal

```python norun
def creer_compteur():
    total = 0
    def ajouter():
        total += 1
        return total
    return ajouter

compter = creer_compteur()
print(compter())
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    print(compter())
          ~~~~~~~^^
  File "main.py", line 4, in ajouter
    total += 1
    ^^^^^
UnboundLocalError: cannot access local variable 'total' where it is not associated with a value
```

`total += 1` fait de `total` une variable locale de `ajouter`, qui n'a pas encore de
valeur. Ajoutez `nonlocal total` au début de `ajouter`.

### Oublier les parenthèses d'un décorateur à arguments

```python norun
def repeter(fois):
    def decorateur(fonction):
        def enveloppe():
            for _ in range(fois):
                fonction()
        return enveloppe
    return decorateur

@repeter
def saluer():
    print("Bonjour")

saluer()
```

```text
Traceback (most recent call last):
  File "main.py", line 13, in <module>
    saluer()
    ~~~~~~^^
TypeError: repeter.<locals>.decorateur() missing 1 required positional argument: 'fonction'
```

Sans parenthèses, `saluer` a été donnée comme valeur de `fois`, et `saluer` est devenue
le décorateur lui-même. Écrivez `@repeter(2)`.

### Mettre en cache une fonction qui reçoit une liste

```python norun
from functools import lru_cache

@lru_cache
def somme(nombres):
    return sum(nombres)

print(somme([1, 2, 3]))
```

```text
Traceback (most recent call last):
  File "main.py", line 7, in <module>
    print(somme([1, 2, 3]))
          ~~~~~^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

Le cache range les arguments comme clés d'un dictionnaire : une liste ne peut pas en
être une. Passez un tuple : `somme((1, 2, 3))`.

## Testez-vous

**Question 1 :** quelle différence entre `f` et `f()` ?

> **Réponse**
>
> `f` est la fonction elle-même, une valeur qu'on peut ranger ou passer. `f()` l'exécute
> et donne son résultat.

**Question 2 :** que fait ce code ?

```python norun
@journaliser
def carre(x):
    return x * x
```

> **Réponse**
>
> Il définit `carre`, puis la remplace par `journaliser(carre)` : c'est exactement
> `carre = journaliser(carre)`.

**Question 3 :** écrivez un décorateur `majuscules` qui met en majuscules le texte
renvoyé par une fonction.

> **Réponse**
>
> ```python
> import functools
>
> def majuscules(fonction):
>     @functools.wraps(fonction)
>     def enveloppe(*args, **kwargs):
>         return fonction(*args, **kwargs).upper()
>     return enveloppe
>
> @majuscules
> def saluer(nom):
>     return f"bonjour {nom}"
>
> print(saluer("zoé"))
> ```
>
> ```sortie
> BONJOUR ZOÉ
> ```

## En résumé

- Une fonction est une valeur : on peut la ranger, la passer, la renvoyer.
- Une closure se souvient des variables de la fonction qui l'a créée ; `nonlocal` permet
  de les modifier.
- Un décorateur prend une fonction et renvoie une enveloppe ; `@d` veut dire `f = d(f)`.
- Posez toujours `@functools.wraps(fonction)` sur l'enveloppe.
- `lru_cache` mémorise les résultats, `partial` fixe des arguments.
- Préférez les compréhensions à `map` et `filter` quand elles sont plus lisibles.

Suite : [gérer les erreurs](erreurs.md)
