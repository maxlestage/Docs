# Itérateurs et générateurs

Cette page explique ce qui se passe vraiment dans une boucle `for`, comment écrire vos propres générateurs avec `yield`, et comment le module `itertools` traite des suites de valeurs, même infinies.

> **Objectifs :**
>
> - vous saurez distinguer un itérable d'un itérateur ;
> - vous saurez utiliser `iter` et `next`, et reconnaître `StopIteration` ;
> - vous saurez écrire un générateur avec `yield` et enchaîner plusieurs générateurs ;
> - vous saurez utiliser les outils principaux d'`itertools` ;
> - vous saurez ce qu'est l'évaluation paresseuse.

## Itérable et itérateur

Deux mots se ressemblent, mais ne veulent pas dire la même chose :

- un **itérable** est tout ce qu'on peut parcourir avec `for` : une liste, une chaîne, un
  dictionnaire, un `range`, un fichier… ;
- un [itérateur](../debuter/glossaire.md#iterateur) est l'objet qui fait le parcours. Il
  se souvient de **où il en est** et donne l'élément suivant quand on le lui demande.

Prenez un livre : le livre est l'itérable ; le **marque-page** est l'itérateur. Plusieurs
lecteurs peuvent lire le même livre, chacun avec son marque-page.

## iter et next

`iter(iterable)` crée un itérateur ; `next(iterateur)` donne l'élément suivant.

```python
fruits = ["pomme", "poire", "kiwi"]
it = iter(fruits)
print(next(it))
print(next(it))
print(next(it))
```

```sortie
pomme
poire
kiwi
```

Quand il n'y a plus rien, `next` lève l'exception `StopIteration`. Vous pouvez aussi
donner une valeur par défaut à `next`, renvoyée à la fin au lieu de l'erreur :

```python
it = iter(["pomme"])
print(next(it))
print(next(it, "fini"))
```

```sortie
pomme
fini
```

### Ce que fait vraiment for

Une boucle `for` n'est qu'un raccourci : elle appelle `iter`, puis `next` en boucle, et
s'arrête proprement à la première `StopIteration`. Ces deux codes font la même chose :

```python
for lettre in "abc":
    print(lettre)

it = iter("abc")
while True:
    try:
        lettre = next(it)
    except StopIteration:
        break
    print(lettre)
```

```sortie
a
b
c
a
b
c
```

### Un itérateur s'use

Un itérateur ne revient jamais en arrière. Une fois au bout, il est vide.

```python
it = iter([1, 2, 3])
print(list(it))
print(list(it))
```

```sortie
[1, 2, 3]
[]
```

La liste, elle, peut être parcourue autant de fois qu'on veut : chaque `for` crée un
nouvel itérateur.

## Écrire un générateur avec yield

Un **générateur** est la façon la plus simple de créer un itérateur. C'est une fonction
qui contient `yield` (« produire ») au lieu de `return`.

```python
def compte_a_rebours(n):
    while n > 0:
        yield n
        n -= 1
    print("Décollage !")

for x in compte_a_rebours(3):
    print(x)
```

```sortie
3
2
1
Décollage !
```

Que se passe-t-il ?

1. Appeler `compte_a_rebours(3)` **n'exécute rien** : cela renvoie un générateur, en
   pause au début de la fonction.
2. Chaque `next` (ici, fait par `for`) fait avancer la fonction jusqu'au prochain
   `yield`. La valeur est donnée, puis la fonction se **met en pause**, en gardant ses
   variables.
3. Quand la fonction se termine, le générateur lève `StopIteration`, et la boucle
   s'arrête.

On le voit bien avec `next` à la main :

```python
def etapes():
    print("début")
    yield 1
    print("milieu")
    yield 2

g = etapes()
print("générateur créé")
print(next(g))
print(next(g))
```

```sortie
générateur créé
début
1
milieu
2
```

> **À retenir :** `return` rend une valeur et **termine** la fonction. `yield` rend une
> valeur et **met en pause** la fonction, qui reprendra au même endroit.

## Les pipelines de générateurs

Des générateurs peuvent se brancher les uns sur les autres, comme des tuyaux : chacun
reçoit les valeurs du précédent, les transforme et les passe au suivant. Les valeurs
circulent **une par une** dans toute la chaîne.

```python
lignes = [
    "alice,15",
    "",
    "bob,8",
    "# commentaire",
    "chloé,12",
]

def non_vides(lignes):
    for ligne in lignes:
        if ligne and not ligne.startswith("#"):
            yield ligne

def decouper(lignes):
    for ligne in lignes:
        nom, note = ligne.split(",")
        yield nom, int(note)

def recus(eleves):
    for nom, note in eleves:
        if note >= 10:
            yield nom

for nom in recus(decouper(non_vides(lignes))):
    print(nom)
```

```sortie
alice
chloé
```

Chaque étape est une petite fonction facile à tester. Et si `lignes` était un fichier
d'un million de lignes, une seule ligne serait en mémoire à la fois.

### yield from

`yield from` donne, une par une, toutes les valeurs d'un autre itérable :

```python
def tout(*listes):
    for liste in listes:
        yield from liste

print(list(tout([1, 2], [3], [4, 5])))
```

```sortie
[1, 2, 3, 4, 5]
```

## L'évaluation paresseuse

Un générateur est **paresseux** : il ne calcule une valeur que lorsqu'on la demande.
Cela permet même d'écrire une suite **infinie** :

```python
def entiers_pairs():
    n = 0
    while True:
        yield n
        n += 2

gen = entiers_pairs()
for _ in range(5):
    print(next(gen), end=" ")
print()
```

```sortie
0 2 4 6 8 
```

`while True` ne bloque pas le programme : la fonction s'arrête à chaque `yield` et
attend qu'on lui demande la suite. Attention en revanche à `list(entiers_pairs())` :
cela tournerait sans fin.

Autre exemple, la suite de Fibonacci, où chaque nombre est la somme des deux
précédents :

```python
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

for n in fibonacci():
    if n > 100:
        break
    print(n, end=" ")
print()
```

```sortie
0 1 1 2 3 5 8 13 21 34 55 89 
```

## Le module itertools

Le module `itertools` de la bibliothèque standard fournit des itérateurs tout prêts. Ils
sont tous paresseux.

### count et islice

`count(debut, pas)` compte à l'infini. `islice(iterable, n)` prend les `n` premiers
éléments, comme `[:n]` sur une liste, mais pour n'importe quel itérateur.

```python
from itertools import count, islice

print(list(islice(count(10, 5), 4)))

def carres():
    for n in count(1):
        yield n * n

print(list(islice(carres(), 6)))
print(list(islice("abcdefgh", 2, 6)))
```

```sortie
[10, 15, 20, 25]
[1, 4, 9, 16, 25, 36]
['c', 'd', 'e', 'f']
```

### chain

`chain` met plusieurs itérables bout à bout, sans créer de nouvelle liste.

```python
from itertools import chain

matin = ["café", "croissant"]
midi = ["salade"]
for plat in chain(matin, midi, ["pomme"]):
    print(plat)
```

```sortie
café
croissant
salade
pomme
```

### groupby

`groupby` regroupe les éléments **consécutifs** qui ont la même clé. Il faut donc
d'abord **trier** selon cette même clé.

```python
from itertools import groupby

mots = ["avion", "bus", "arbre", "bateau", "chat"]
mots_tries = sorted(mots, key=lambda m: m[0])
for lettre, groupe in groupby(mots_tries, key=lambda m: m[0]):
    print(lettre, list(groupe))
```

```sortie
a ['avion', 'arbre']
b ['bus', 'bateau']
c ['chat']
```

### product

`product` donne toutes les combinaisons, comme des boucles `for` imbriquées.

```python
from itertools import product

for taille, couleur in product(["S", "M"], ["rouge", "bleu"]):
    print(taille, couleur)
print(len(list(product("01", repeat=3))))
```

```sortie
S rouge
S bleu
M rouge
M bleu
8
```

`repeat=3` répète le même itérable trois fois : il y a 8 nombres binaires de trois
chiffres.

### accumulate

`accumulate` donne les totaux successifs : chaque valeur est la somme de toutes les
précédentes.

```python
from itertools import accumulate

depenses = [12, 5, 30, 8]
print(list(accumulate(depenses)))
print(list(accumulate(depenses, max)))
```

```sortie
[12, 17, 47, 55]
[12, 12, 30, 30]
```

Avec `max`, on obtient le plus grand nombre vu jusqu'ici.

| Outil | Rôle |
|-------|------|
| `count(debut, pas)` | compte sans fin |
| `islice(it, n)` / `islice(it, debut, fin)` | découpe un itérateur |
| `chain(a, b, …)` | met des itérables bout à bout |
| `groupby(it, key)` | regroupe les éléments consécutifs |
| `product(a, b)` | toutes les combinaisons |
| `accumulate(it)` | totaux cumulés |

## Erreurs fréquentes

### Appeler next sur une liste

```python norun
fruits = ["pomme", "poire"]
print(next(fruits))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(next(fruits))
          ~~~~^^^^^^^^
TypeError: 'list' object is not an iterator
```

Une liste est itérable, mais ce n'est pas un itérateur. Créez d'abord le marque-page :
`next(iter(fruits))`.

### Aller plus loin que la fin

```python norun
it = iter(["pomme"])
print(next(it))
print(next(it))
```

```text
pomme
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(next(it))
          ~~~~^^^^
StopIteration
```

L'itérateur est vide après `pomme`. Utilisez une boucle `for`, qui s'arrête toute seule,
ou une valeur par défaut : `next(it, None)`.

### Demander la longueur d'un générateur

```python norun
def pairs(n):
    for i in range(0, n, 2):
        yield i

print(len(pairs(10)))
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(len(pairs(10)))
          ~~~^^^^^^^^^^^
TypeError: object of type 'generator' has no len()
```

Un générateur ne sait pas combien de valeurs il va produire avant de les avoir
produites. Transformez-le en liste : `len(list(pairs(10)))`, ou comptez avec
`sum(1 for _ in pairs(10))`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
def lettres():
    yield "a"
    yield "b"

g = lettres()
print(list(g), list(g))
```

> **Réponse**
>
> `['a', 'b'] []`. Le premier `list(g)` vide le générateur ; le second ne trouve plus
> rien.

**Question 2 :** pourquoi `groupby` donne-t-il deux groupes `a` pour
`["avion", "bus", "arbre"]` si on ne trie pas ?

> **Réponse**
>
> Parce que `groupby` ne regroupe que les éléments **qui se suivent**. `avion` et
> `arbre` sont séparés par `bus` : ils forment deux groupes. Triez d'abord avec la même
> clé.

**Question 3 :** écrivez un générateur `multiples(k)` infini, puis affichez les 4
premiers multiples de 7.

> **Réponse**
>
> ```python
> from itertools import islice
>
> def multiples(k):
>     n = k
>     while True:
>         yield n
>         n += k
>
> print(list(islice(multiples(7), 4)))
> ```
>
> ```sortie
> [7, 14, 21, 28]
> ```

## En résumé

- Un itérable se parcourt ; un itérateur fait le parcours et se souvient de sa position.
- `iter` crée un itérateur, `next` donne l'élément suivant, `StopIteration` marque la fin.
- Une fonction avec `yield` est un générateur : elle se met en pause à chaque valeur.
- Les générateurs s'enchaînent en pipelines et peuvent être infinis : ils sont paresseux.
- `itertools` fournit `count`, `islice`, `chain`, `groupby`, `product`, `accumulate`.

Suite : [Les collections avancées](collections-avancees.md)
