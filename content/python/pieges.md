# Les pièges du débutant en Python

Cette page rassemble les douze pièges qui bloquent le plus souvent les débutants en Python, avec pour chacun ce qui se passe, pourquoi, et comment l'éviter.

> **Objectifs :**
>
> - vous saurez reconnaître les douze pièges les plus courants de Python ;
> - vous comprendrez pourquoi Python se comporte ainsi ;
> - vous saurez corriger chaque piège en une ligne ou deux.

## Comment lire cette page

Chaque piège suit le même chemin :

1. ce que vous **pensez** qu'il va se passer ;
2. le code, et ce qui se passe **vraiment** ;
3. **pourquoi** ;
4. **la correction**.

Ne lisez pas tout d'un coup. Gardez cette page sous la main. Quand un programme vous
surprend, revenez ici et cherchez le piège qui ressemble à votre problème.

> **À retenir :** un piège n'est pas une faute de votre part. Tout le monde tombe dedans
> au début. Ce qui compte, c'est de le reconnaître la deuxième fois.

## Les pièges de syntaxe

La [syntaxe](../debuter/glossaire.md#syntaxe), ce sont les règles d'écriture du langage,
comme la grammaire pour le français.

### Écrire = au lieu de ==

**Vous pensez :** « je teste si l'âge vaut 18 ».

```python norun
age = 18
if age = 18:
    print("majeur")
```

```text
  File "main.py", line 2
    if age = 18:
       ^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

**Pourquoi :** en Python, les deux signes ne veulent pas dire la même chose.

| Signe | Se lit | Rôle |
|---|---|---|
| `=` | « reçoit » | range une valeur dans une variable |
| `==` | « est égal à ? » | pose une question, répond `True` ou `False` |

Pensez à une boîte avec une étiquette. `age = 18` met 18 **dans** la boîte. `age == 18`
**regarde** dans la boîte et répond oui ou non.

**La correction :**

```python
age = 18
if age == 18:
    print("majeur")
```

```sortie
majeur
```

Python vous aide : le message dit `Maybe you meant '=='` (« vouliez-vous dire `==` ? »).

### Oublier l'indentation

**Vous pensez :** « le `print` fait partie du `if` ».

```python norun
age = 18
if age >= 18:
print("majeur")
```

```text
  File "main.py", line 3
    print("majeur")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

**Pourquoi :** en Python, l'[indentation](../debuter/glossaire.md#indentation) (les
espaces au début de la ligne) n'est pas décorative. C'est elle qui dit quelles lignes
sont **dans** le `if`.

```text
if age >= 18:          ← la question
    print("majeur")    ← 4 espaces : cette ligne est DANS le if
print("fin")           ← 0 espace : cette ligne est APRÈS le if
```

**La correction :** ajoutez 4 espaces devant chaque ligne du bloc.

```python
age = 18
if age >= 18:
    print("majeur")
print("fin")
```

```sortie
majeur
fin
```

Le piège inverse existe aussi : trop d'espaces, sans raison. Python répond alors
`IndentationError: unexpected indent` (« indentation inattendue »).

> **Attention :** ne mélangez jamais tabulations et espaces. Réglez votre éditeur pour
> qu'il mette 4 espaces quand vous appuyez sur Tab.

## Les pièges de types

### Additionner du texte et un nombre

**Vous pensez :** `"12" + 1` donne `13`.

```python norun
age = "12"
print(age + 1)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(age + 1)
          ~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

**Pourquoi :** `"12"` a des guillemets. C'est du **texte** (une
[chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres)), pas un nombre.
Pour le texte, `+` veut dire « coller bout à bout ». Python ne sait pas coller un nombre
à du texte, et il refuse de deviner.

Le même `+` donne des résultats très différents selon le type :

```python
print(12 + 1)
print("12" + "1")
print("12" * 3)
```

```sortie
13
121
121212
```

**La correction :** transformez le texte en nombre avec `int()`.

```python
age = "12"
print(int(age) + 1)
```

```sortie
13
```

Ce piège arrive souvent avec `input()` : ce que tape l'utilisateur est **toujours** du
texte. Écrivez `int(input("Votre âge : "))`.

> **Essayez :** remplacez `int(age)` par `str(1)` et `+ 1` par `+ str(1)`. Vous obtenez
> `121` : cette fois, tout est du texte.

### La division qui donne un nombre à virgule

**Vous pensez :** `10 / 2` donne `5`.

```python
print(10 / 2)
print(7 / 2)
print(7 // 2)
print(7 % 2)
```

```sortie
5.0
3.5
3
1
```

**Pourquoi :** en Python, `/` donne **toujours** un nombre à virgule (un
[flottant](../debuter/glossaire.md#flottant)), même quand la division tombe juste.

| Opérateur | Nom | `7 ? 2` |
|---|---|---|
| `/` | division | `3.5` |
| `//` | division entière | `3` |
| `%` | reste | `1` |

**La correction :** utilisez `//` quand vous voulez un nombre entier, par exemple pour
partager 7 bonbons entre 2 enfants.

### 0.1 + 0.2 ne fait pas 0.3

**Vous pensez :** `0.1 + 0.2 == 0.3` est vrai.

```python
print(0.1 + 0.2)
print(0.1 + 0.2 == 0.3)
```

```sortie
0.30000000000000004
False
```

**Pourquoi :** l'ordinateur range les nombres à virgule en binaire. `0.1` ne tombe pas
juste en binaire, comme `1/3` ne tombe pas juste en décimal (`0.3333…`). Il reste une
toute petite erreur.

**La correction :** arrondissez avant de comparer.

```python
print(round(0.1 + 0.2, 2) == 0.3)
```

```sortie
True
```

Le chapitre [les nombres](nombres.md) montre aussi `math.isclose` et `Decimal`.

> **À retenir :** regardez toujours le **type** d'une valeur. Avec un doute, affichez
> `type(valeur)`.

## Les pièges de listes

### Compter à partir de 0, et range qui s'arrête avant

**Vous pensez :** une liste de 3 éléments a un élément numéro 3.

```python norun
nombres = [10, 20, 30]
print(nombres[3])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(nombres[3])
          ~~~~~~~^^^
IndexError: list index out of range
```

**Pourquoi :** en Python, on compte les places à partir de **0**. Le numéro d'une place
s'appelle un [indice](../debuter/glossaire.md#indice).

```text
nombres = [ 10 , 20 , 30 ]
indice :     0    1    2      ← il n'y a pas de place 3
```

`range` suit la même logique : `range(1, 5)` s'arrête **avant** 5.

```python
print(list(range(1, 5)))
print(list(range(5)))
```

```sortie
[1, 2, 3, 4]
[0, 1, 2, 3, 4]
```

**La correction :** le dernier élément est à l'indice `len(liste) - 1`, ou plus
simplement `liste[-1]`. Pour compter de 1 à 5 compris, écrivez `range(1, 6)`.

```python
nombres = [10, 20, 30]
print(nombres[2])
print(nombres[-1])
```

```sortie
30
30
```

### Deux noms pour la même liste

**Vous pensez :** `copie = notes` fait une copie.

```python
notes = [12, 15]
copie = notes
copie.append(8)
print(notes)
```

```sortie
[12, 15, 8]
```

**Pourquoi :** `copie = notes` ne copie rien. Cela colle une **deuxième étiquette** sur
la **même** liste.

```text
notes ──┐
        ├──► [12, 15, 8]     une seule liste, deux étiquettes
copie ──┘
```

Une liste est [mutable](../debuter/glossaire.md#mutable) : on peut la modifier. Donc ce
que vous faites avec une étiquette se voit avec l'autre.

**La correction :** demandez une vraie copie avec `.copy()`.

```python
notes = [12, 15]
copie = notes.copy()
copie.append(8)
print(notes)
print(copie)
```

```sortie
[12, 15]
[12, 15, 8]
```

> **Essayez :** remplacez `.copy()` par `[:]` (`copie = notes[:]`). Le résultat est le
> même : `[:]` prend une tranche qui contient tout.

### Supprimer des éléments pendant qu'on parcourt la liste

**Vous pensez :** ce code enlève tous les nombres négatifs.

```python
nombres = [-1, -2, 3, 4]
for n in nombres:
    if n < 0:
        nombres.remove(n)
print(nombres)
```

```sortie
[-2, 3, 4]
```

**Pourquoi :** la boucle avance d'une place à chaque tour. Mais `remove` décale tous les
éléments vers la gauche. La boucle saute donc un élément. Suivons-la pas à pas :

| Tour | Place lue | Liste avant | `n` | Action | Liste après |
|---|---|---|---|---|---|
| 1 | 0 | `[-1, -2, 3, 4]` | `-1` | supprimer | `[-2, 3, 4]` |
| 2 | 1 | `[-2, 3, 4]` | `3` | rien | `[-2, 3, 4]` |
| 3 | 2 | `[-2, 3, 4]` | `4` | rien | `[-2, 3, 4]` |

Au tour 2, la boucle lit la place 1. Le `-2` a glissé à la place 0 : il n'est jamais lu.

**La correction :** ne modifiez pas la liste que vous parcourez. Construisez une
**nouvelle** liste avec ce que vous gardez, par exemple avec une
[compréhension](comprehensions.md).

```python
nombres = [-1, -2, 3, 4]
nombres = [n for n in nombres if n >= 0]
print(nombres)
```

```sortie
[3, 4]
```

### sort() ne renvoie rien

**Vous pensez :** `mots = mots.sort()` range la liste dans `mots`.

```python
mots = ["poire", "abricot"]
mots = mots.sort()
print(mots)
```

```sortie
None
```

**Pourquoi :** `.sort()` trie la liste **sur place** : elle modifie la liste elle-même,
puis renvoie `None` (« rien »). Vous avez donc rangé `None` dans `mots`.

**La correction :** deux façons, au choix.

```python
mots = ["poire", "abricot"]
mots.sort()
print(mots)

fruits = ["poire", "abricot"]
ranges = sorted(fruits)
print(ranges)
print(fruits)
```

```sortie
['abricot', 'poire']
['abricot', 'poire']
['poire', 'abricot']
```

`liste.sort()` modifie la liste. `sorted(liste)` laisse la liste tranquille et renvoie
une nouvelle liste triée.

> **À retenir :** une liste peut avoir plusieurs noms. Avant de la modifier, demandez-vous
> « qui d'autre regarde cette liste ? ».

## Les pièges de texte

### Les méthodes de texte ne modifient pas le texte

**Vous pensez :** `nom.upper()` met `nom` en majuscules.

```python
nom = "alice"
nom.upper()
print(nom)
```

```sortie
alice
```

**Pourquoi :** une chaîne de caractères est
[immuable](../debuter/glossaire.md#immuable) : on ne peut jamais la changer. `upper()`
fabrique une **nouvelle** chaîne. Si vous ne la rangez nulle part, elle est perdue.

C'est le contraire du piège de `sort()` : pour le texte, la méthode renvoie le résultat
et ne touche pas à l'original.

**La correction :** rangez le résultat.

```python
nom = "alice"
nom = nom.upper()
print(nom)
```

```sortie
ALICE
```

## Les pièges de fonctions

### Afficher au lieu de renvoyer

**Vous pensez :** cette fonction donne le double d'un nombre.

```python
def double(x):
    print(x * 2)

resultat = double(5)
print(resultat)
```

```sortie
10
None
```

**Pourquoi :** `print` **montre** une valeur à l'écran. `return` **donne** une valeur au
reste du programme. Une fonction sans `return` renvoie `None`.

Pensez à un distributeur de boissons. `print`, c'est la boisson affichée sur l'écran.
`return`, c'est la boisson qui tombe dans le bac : vous pouvez la prendre.

**La correction :**

```python
def double(x):
    return x * 2

resultat = double(5)
print(resultat)
print(resultat + 1)
```

```sortie
10
11
```

### Une liste vide comme valeur par défaut

**Vous pensez :** chaque appel commence avec un panier vide.

```python
def ajouter(article, panier=[]):
    panier.append(article)
    return panier

print(ajouter("pain"))
print(ajouter("lait"))
```

```sortie
['pain']
['pain', 'lait']
```

**Pourquoi :** la valeur par défaut `[]` est créée **une seule fois**, quand Python lit
la ligne `def`. Tous les appels partagent ensuite **la même** liste.

**La correction :** mettez `None` par défaut, et créez la liste dans la fonction.

```python
def ajouter(article, panier=None):
    if panier is None:
        panier = []
    panier.append(article)
    return panier

print(ajouter("pain"))
print(ajouter("lait"))
```

```sortie
['pain']
['lait']
```

### Modifier une variable du dehors

**Vous pensez :** la fonction ajoute 1 au compteur.

```python norun
compteur = 0

def ajouter():
    compteur = compteur + 1

ajouter()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    ajouter()
    ~~~~~~~^^
  File "main.py", line 4, in ajouter
    compteur = compteur + 1
               ^^^^^^^^
UnboundLocalError: cannot access local variable 'compteur' where it is not associated with a value
```

**Pourquoi :** dès qu'une fonction **range** quelque chose dans un nom (`compteur = …`),
Python considère que ce nom appartient à la fonction. C'est une question de
[portée](../debuter/glossaire.md#portee). La fonction cherche donc son propre `compteur`,
qui n'a encore aucune valeur.

**La correction :** passez la valeur en paramètre et renvoyez le résultat.

```python
def ajouter(compteur):
    return compteur + 1

compteur = 0
compteur = ajouter(compteur)
compteur = ajouter(compteur)
print(compteur)
```

```sortie
2
```

Le mot-clé `global` existe aussi, mais il rend les programmes difficiles à suivre.
Évitez-le au début.

### Appeler une variable list ou str

**Vous pensez :** `list` est un bon nom pour une liste.

```python norun
list = [3, 1, 2]
print(list("abc"))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(list("abc"))
          ~~~~^^^^^^^
TypeError: 'list' object is not callable
```

**Pourquoi :** `list`, `str`, `int`, `sum`, `max`… sont des fonctions de Python. Si vous
rangez autre chose sous ce nom, vous **cachez** la fonction. Python trouve votre liste
et essaie de « l'appeler » comme une fonction.

**La correction :** choisissez un nom qui dit ce que contient la variable :
`nombres`, `prenoms`, `texte`, `total`.

> **À retenir :** une fonction **renvoie** avec `return`, travaille avec ses
> **paramètres**, et ne doit pas réutiliser les noms de Python.

## Erreurs fréquentes

Deux erreurs encore, qui ne sont pas des pièges mais qui arrivent tous les jours.

### Oublier les guillemets

```python norun
print(bonjour)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    print(bonjour)
          ^^^^^^^
NameError: name 'bonjour' is not defined
```

Sans guillemets, `bonjour` est un **nom de variable**, et cette variable n'existe pas.
Écrivez `print("bonjour")`. La même erreur arrive avec une faute de frappe dans un nom :
`prnom` au lieu de `prenom`.

### Une clé absente du dictionnaire

```python norun
ages = {"Alice": 30}
print(ages["Bob"])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(ages["Bob"])
          ~~~~^^^^^^^
KeyError: 'Bob'
```

La clé `"Bob"` n'existe pas. Utilisez `.get`, qui renvoie une valeur de secours au lieu
de planter :

```python
ages = {"Alice": 30}
print(ages.get("Bob", "inconnu"))
```

```sortie
inconnu
```

Pour lire calmement un message d'erreur, voyez
[lire une erreur](../debuter/lire-une-erreur.md).

## Testez-vous

**Question 1 :** qu'affiche ce programme, et pourquoi ?

```python norun
a = [1, 2]
b = a
b.append(3)
print(a)
```

> **Réponse**
>
> Il affiche `[1, 2, 3]`. `b = a` ne copie pas la liste : `a` et `b` sont deux
> étiquettes sur la même liste. Pour une vraie copie, écrivez `b = a.copy()`.

**Question 2 :** ce programme affiche `10 / 4 = 2.5`. Comment obtenir `2` ?

```python norun
print("10 / 4 =", 10 / 4)
```

> **Réponse**
>
> Avec la division entière `//` :
>
> ```python
> print("10 // 4 =", 10 // 4)
> ```
>
> ```sortie
> 10 // 4 = 2
> ```

**Question 3 :** pourquoi ce programme affiche-t-il `None` ?

```python norun
def carre(x):
    print(x * x)

print(carre(3))
```

> **Réponse**
>
> La fonction affiche `9`, mais ne renvoie rien : sans `return`, elle renvoie `None`.
> Le deuxième `print` affiche donc `None`. Remplacez `print(x * x)` par
> `return x * x`.

## En résumé

- `=` range une valeur, `==` compare deux valeurs.
- L'indentation fait partie du code : 4 espaces par niveau.
- `"12"` est du texte : convertissez avec `int()` avant de calculer. `/` donne toujours
  un flottant, `//` un entier.
- On compte à partir de 0, et `range(a, b)` s'arrête avant `b`.
- `b = a` ne copie pas une liste ; ne modifiez pas une liste pendant que vous la
  parcourez.
- Une fonction renvoie avec `return` ; pas de liste vide comme valeur par défaut.

Suite : [Exercices pour bien démarrer](exercices-faciles.md)
