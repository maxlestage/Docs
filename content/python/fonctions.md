# Les fonctions

Cette page explique comment écrire des fonctions en Python avec `def`, leur donner des paramètres, récupérer leur résultat avec `return` et savoir quelles variables elles voient.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec `def` et l'appeler ;
> - vous saurez renvoyer une valeur avec `return` ;
> - vous saurez utiliser les valeurs par défaut et les arguments nommés ;
> - vous saurez quelles variables une fonction peut voir.

## Définir une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez. Elle commence par `def` (*define*, « définir »),
suivi du nom, de parenthèses et de deux-points. Le corps est indenté.

```python
def saluer():
    print("Bonjour !")

saluer()
saluer()
```

```sortie
Bonjour !
Bonjour !
```

Ligne par ligne :

- `def saluer():` crée une fonction nommée `saluer`. Rien ne s'affiche encore ;
- `print("Bonjour !")` est décalé : il fait partie de la fonction, il attend ;
- le premier `saluer()` lance la fonction : Python exécute son corps et affiche
  « Bonjour ! » ;
- le second `saluer()` recommence : la même recette, suivie une deuxième fois.

Écrire `def` ne lance rien : cela enregistre la recette. Écrire `saluer()` **appelle**
la fonction, c'est-à-dire l'exécute. Les parenthèses sont obligatoires, même vides.

## Les paramètres

Une recette de crêpes peut servir pour 4 ou pour 8 personnes : seules les quantités
changent. De même, une fonction est plus utile si elle peut travailler avec des valeurs
différentes à chaque appel.

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la fonction
reçoit. Ce sont des cases vides, remplies au moment de l'appel. Les valeurs données à
l'appel s'appellent les [arguments](../debuter/glossaire.md#argument).

```python
def saluer(prenom):
    print(f"Bonjour, {prenom} !")

saluer("Alice")
saluer("Bob")
```

```sortie
Bonjour, Alice !
Bonjour, Bob !
```

Ligne par ligne :

- `def saluer(prenom):` crée la fonction, avec une case vide nommée `prenom` ;
- `saluer("Alice")` appelle la fonction : `prenom` reçoit `"Alice"`, puis le `print`
  affiche « Bonjour, Alice ! » ;
- `saluer("Bob")` l'appelle de nouveau : cette fois, `prenom` vaut `"Bob"`.

Ici, `prenom` est le paramètre ; `"Alice"` et `"Bob"` sont les arguments. Plusieurs
paramètres se séparent par des virgules.

**À vous :** écrivez une fonction `anniversaire(prenom)` qui affiche « Joyeux
anniversaire, … ! » avec le prénom reçu. Appelez-la pour Léa, puis pour Tom.

> **Solution**
>
> ```python
> def anniversaire(prenom):
>     print(f"Joyeux anniversaire, {prenom} !")
>
> anniversaire("Léa")
> anniversaire("Tom")
> ```
>
> ```sortie
> Joyeux anniversaire, Léa !
> Joyeux anniversaire, Tom !
> ```

## Renvoyer une valeur avec return

Souvent, vous voulez qu'une fonction fasse un calcul et vous **donne** le résultat,
comme une calculatrice. Vous pouvez ensuite l'afficher, le ranger dans une variable ou
le réutiliser dans un autre calcul.

`return` renvoie un résultat au code qui a appelé la fonction, et quitte la fonction.

```python
def aire(largeur, hauteur):
    return largeur * hauteur

print(aire(3, 4))
total = aire(2, 5) + aire(1, 1)
print(total)
```

```sortie
12
11
```

Ligne par ligne :

- `def aire(largeur, hauteur):` crée une fonction qui reçoit deux nombres ;
- `return largeur * hauteur` calcule le produit et le renvoie à l'appelant ;
- `print(aire(3, 4))` : l'appel vaut 12, donc Python affiche 12 ;
- `total = aire(2, 5) + aire(1, 1)` : les deux appels valent 10 et 1, et `total`
  reçoit 11.

La valeur renvoyée **remplace** l'appel : `aire(2, 5) + aire(1, 1)` devient `10 + 1`.

`return` peut aussi servir à sortir plus tôt :

```python
def diviser(a, b):
    if b == 0:
        return "division impossible"
    return a / b

print(diviser(10, 4))
print(diviser(10, 0))
```

```sortie
2.5
division impossible
```

### Afficher ou renvoyer ?

`print` **affiche** une valeur à l'écran. `return` la **donne** au code appelant, qui
peut s'en servir. Une fonction sans `return` renvoie `None`.

```python
def double_affiche(x):
    print(x * 2)

def double(x):
    return x * 2

resultat1 = double_affiche(5)
resultat2 = double(5)
print(resultat1, resultat2)
```

```sortie
10
None 10
```

> **À retenir :** préférez les fonctions qui renvoient une valeur. Vous choisirez
> ensuite de l'afficher, de la stocker ou de la réutiliser.

### Renvoyer plusieurs valeurs

Séparez les valeurs par des virgules. Python les regroupe dans un
[tuple](../debuter/glossaire.md#tuple), que vous pouvez découper en plusieurs
variables.

```python
def min_max(nombres):
    return min(nombres), max(nombres)

petit, grand = min_max([4, 9, 1])
print(petit, grand)
```

```sortie
1 9
```

**À vous :** écrivez une fonction `perimetre(largeur, hauteur)` qui **renvoie** le
périmètre d'un rectangle (deux fois la somme des côtés). Affichez `perimetre(3, 4)`.

> **Solution**
>
> ```python
> def perimetre(largeur, hauteur):
>     return 2 * (largeur + hauteur)
>
> print(perimetre(3, 4))
> ```
>
> ```sortie
> 14
> ```

## Les valeurs par défaut

Au café, on vous sert un café normal, sauf si vous demandez autre chose. Une valeur
par défaut joue ce rôle : elle évite de répéter la valeur la plus courante.

Un paramètre peut avoir une valeur par défaut. Elle sert si l'argument est absent.

```python
def saluer(prenom, salutation="Bonjour"):
    return f"{salutation}, {prenom} !"

print(saluer("Alice"))
print(saluer("Bob", "Salut"))
```

```sortie
Bonjour, Alice !
Salut, Bob !
```

Ligne par ligne :

- `salutation="Bonjour"` donne une valeur de secours au paramètre `salutation` ;
- `saluer("Alice")` ne donne qu'un argument : `salutation` prend donc `"Bonjour"` ;
- `saluer("Bob", "Salut")` donne les deux : `"Salut"` remplace la valeur par défaut.

Les paramètres avec valeur par défaut se placent **après** les autres.

**À vous :** écrivez une fonction `prix(montant, devise="euros")` qui renvoie un texte
comme « 5 euros ». Affichez `prix(5)`, puis `prix(8, "dollars")`.

> **Solution**
>
> ```python
> def prix(montant, devise="euros"):
>     return f"{montant} {devise}"
>
> print(prix(5))
> print(prix(8, "dollars"))
> ```
>
> ```sortie
> 5 euros
> 8 dollars
> ```

## Les arguments nommés

À l'appel, vous pouvez donner un argument par son **nom** : `nom=valeur`. L'ordre n'a
alors plus d'importance, et l'appel se lit tout seul.

```python
def commande(produit, quantite=1, urgent=False):
    texte = f"{quantite} x {produit}"
    if urgent:
        texte += " (urgent)"
    return texte

print(commande("café"))
print(commande("thé", urgent=True))
print(commande(quantite=3, produit="croissant"))
```

```sortie
1 x café
1 x thé (urgent)
3 x croissant
```

Ligne par ligne :

- `commande("café")` : seul `produit` est donné ; `quantite` vaut 1 et `urgent` vaut
  `False` ;
- `commande("thé", urgent=True)` : `urgent` est donné par son nom, sans toucher à
  `quantite` ;
- `commande(quantite=3, produit="croissant")` : les deux arguments sont nommés, donc
  l'ordre n'a pas d'importance.

> **Astuce :** nommez les arguments booléens. `commande("thé", urgent=True)` se
> comprend tout de suite ; `commande("thé", 1, True)` beaucoup moins.

## La portée des variables

Une fonction a son propre brouillon. Ce qu'elle y écrit pendant son travail ne
déborde pas sur le reste du programme. Ainsi, deux fonctions peuvent utiliser le même
nom de variable sans se gêner.

La [portée](../debuter/glossaire.md#portee) d'une variable est la zone du code où elle
existe. Une variable créée **dans** une fonction est **locale** : elle naît à l'appel
et disparaît à la fin.

```python
def calculer():
    resultat = 42
    return resultat

print(calculer())
print("resultat" in globals())
```

```sortie
42
False
```

Ligne par ligne :

- `resultat = 42` crée la variable `resultat` **dans** la fonction ;
- `print(calculer())` appelle la fonction, qui renvoie 42 : Python l'affiche ;
- `globals()` donne les variables du dehors. `"resultat" in globals()` demande si
  `resultat` en fait partie : la réponse est `False`.

Après l'appel, `resultat` n'existe plus en dehors de la fonction.

Une fonction peut **lire** une variable du dehors (une variable **globale**). Mais si
elle lui donne une valeur, Python crée une nouvelle variable locale. Pour vraiment
modifier la variable globale, il faut le mot `global` :

```python
compteur = 0

def incrementer():
    global compteur
    compteur += 1

incrementer()
incrementer()
print(compteur)
```

```sortie
2
```

> **Attention :** `global` rend le code difficile à suivre. Préférez passer la valeur
> en argument et renvoyer le résultat : `compteur = incrementer(compteur)`.

## Erreurs fréquentes

### Oublier un argument

```python norun
def aire(largeur, hauteur):
    return largeur * hauteur

print(aire(3))
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 4, in <module>
    print(aire(3))
          ~~~~^^^
TypeError: aire() missing 1 required positional argument: 'hauteur'
```

Il manque l'argument `hauteur` (*missing 1 required argument*). Donnez-les tous :
`aire(3, 4)`. Ou donnez une valeur par défaut au paramètre.

### Appeler la fonction avant de la définir

```python norun
saluer("Alice")

def saluer(nom):
    print(f"Bonjour, {nom} !")
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    saluer("Alice")
    ^^^^^^
NameError: name 'saluer' is not defined
```

Python lit le fichier de haut en bas. À la ligne 1, `saluer` n'existe pas encore.
Placez le `def` avant l'appel.

### Modifier une variable globale sans global

```python norun
compteur = 0

def incrementer():
    compteur = compteur + 1

incrementer()
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 6, in <module>
    incrementer()
    ~~~~~~~~~~~^^
  File "/home/vous/main.py", line 4, in incrementer
    compteur = compteur + 1
               ^^^^^^^^
UnboundLocalError: cannot access local variable 'compteur' where it is not associated with a value
```

Comme la fonction donne une valeur à `compteur`, Python le considère comme local. Mais
il n'a pas encore de valeur au moment de calculer `compteur + 1`. Passez la valeur en
paramètre et renvoyez le résultat, ou ajoutez `global compteur`.

### Une liste comme valeur par défaut

```python norun
def ajouter(article, panier=[]):
    panier.append(article)
    return panier

print(ajouter("pain"))
print(ajouter("lait"))
```

```text
['pain']
['pain', 'lait']
```

Pas d'erreur, mais une surprise : le deuxième panier contient encore le pain. La
valeur par défaut est créée **une seule fois**, quand Python lit le `def`, puis
partagée entre tous les appels. Utilisez `None` :

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

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
def f(x):
    x + 1

print(f(5))
```

> **Réponse**
>
> `None`. La fonction calcule `x + 1` mais ne le renvoie pas : il manque `return`.

**Question 2 :** écrivez une fonction `moyenne(a, b)` qui renvoie la moyenne de deux
nombres.

> **Réponse**
>
> ```python
> def moyenne(a, b):
>     return (a + b) / 2
>
> print(moyenne(3, 4))
> ```
>
> ```sortie
> 3.5
> ```

**Question 3 :** avec `def saluer(prenom, salutation="Bonjour")`, que vaut
`salutation` lors de l'appel `saluer("Alice")` ?

> **Réponse**
>
> `"Bonjour"`. L'argument est absent, donc Python utilise la valeur par défaut.

## En résumé

- `def nom(paramètres):` définit une fonction ; `nom(arguments)` l'appelle.
- `return` renvoie une valeur et quitte la fonction ; sans `return`, elle renvoie
  `None`.
- Les paramètres peuvent avoir une valeur par défaut ; les arguments peuvent être
  nommés.
- Les variables d'une fonction sont locales ; évitez `global`.

Suite : [aller plus loin avec les fonctions](fonctions-avancees.md). Comparez avec
[les fonctions en Ruby](../ruby/fonctions.md).
