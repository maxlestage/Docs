# Les fonctions

Cette page explique comment écrire des fonctions en Python avec `def`, leur donner des paramètres et récupérer leur résultat avec `return`.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec `def` et l'appeler ;
> - vous saurez renvoyer une valeur avec `return` ;
> - vous saurez utiliser les valeurs par défaut, les arguments nommés, `*args` et
>   `**kwargs` ;
> - vous saurez quelles variables une fonction peut voir ;
> - vous saurez écrire une `lambda` et une fonction récursive.

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

Écrire `def` ne lance rien : cela enregistre la recette. Écrire `saluer()` **appelle**
la fonction, c'est-à-dire l'exécute. Les parenthèses sont obligatoires, même vides.

## Les paramètres

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

Ici, `prenom` est le paramètre ; `"Alice"` et `"Bob"` sont les arguments. Plusieurs
paramètres se séparent par des virgules.

## Renvoyer une valeur avec return

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

## Les valeurs par défaut

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

Les paramètres avec valeur par défaut se placent **après** les autres.

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

> **Astuce :** nommez les arguments booléens. `commande("thé", urgent=True)` se
> comprend tout de suite ; `commande("thé", 1, True)` beaucoup moins.

## Un nombre variable d'arguments : *args et **kwargs

Avec une étoile, `*args` reçoit tous les arguments **positionnels** en trop, rangés dans
un tuple.

```python
def somme(*nombres):
    total = 0
    for n in nombres:
        total += n
    return total

print(somme(1, 2))
print(somme(1, 2, 3, 4))
print(somme())
```

```sortie
3
10
0
```

Avec deux étoiles, `**kwargs` (*keyword arguments*) reçoit tous les arguments **nommés**
en trop, rangés dans un [dictionnaire](../debuter/glossaire.md#dictionnaire).

```python
def fiche(nom, **infos):
    print(f"Fiche de {nom}")
    for cle, valeur in infos.items():
        print(f"  {cle} : {valeur}")

fiche("Alice", age=30, ville="Lyon")
```

```sortie
Fiche de Alice
  age : 30
  ville : Lyon
```

Les noms `args` et `kwargs` sont une convention : seules les étoiles comptent. Les
dictionnaires sont expliqués dans [dictionnaires et ensembles](dictionnaires.md).

## Documenter avec une docstring

Une **docstring** est un texte entre triples guillemets, placé juste sous la ligne
`def`. Elle explique ce que fait la fonction. Les éditeurs l'affichent quand vous
survolez le nom, et `help()` la montre.

```python
def prix_ttc(prix_ht, taux=0.2):
    """Renvoie le prix toutes taxes comprises, arrondi au centime."""
    return round(prix_ht * (1 + taux), 2)

print(prix_ttc(10))
print(prix_ttc.__doc__)
```

```sortie
12.0
Renvoie le prix toutes taxes comprises, arrondi au centime.
```

## La portée des variables

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

## Les fonctions sont des valeurs

En Python, une fonction est une valeur comme une autre. Sans parenthèses, son nom
désigne la fonction elle-même : vous pouvez la ranger dans une variable ou la passer à
une autre fonction.

```python
def crier(texte):
    return texte.upper() + " !"

def chuchoter(texte):
    return texte.lower() + "..."

def parler(texte, style):
    return style(texte)

print(parler("Bonjour", crier))
print(parler("Bonjour", chuchoter))
```

```sortie
BONJOUR !
bonjour...
```

`parler` reçoit une fonction (`style`) et l'appelle. C'est très utile pour trier, par
exemple : `sorted` accepte une fonction qui dit selon quoi trier.

```python
mots = ["kiwi", "ananas", "fraise", "pomme"]
print(sorted(mots, key=len))
```

```sortie
['kiwi', 'pomme', 'ananas', 'fraise']
```

## Les lambdas

Une [lambda](../debuter/glossaire.md#lambda) est une petite fonction sans nom, écrite
sur une ligne : `lambda paramètres: expression`. Elle renvoie automatiquement la valeur
de l'expression.

```python
carre = lambda x: x * x
print(carre(5))

eleves = [("Alice", 15), ("Bob", 12), ("Chloé", 18)]
par_note = sorted(eleves, key=lambda eleve: eleve[1])
print(par_note)
```

```sortie
25
[('Bob', 12), ('Alice', 15), ('Chloé', 18)]
```

`key=lambda eleve: eleve[1]` veut dire « trie selon le deuxième élément ». Le tri est
détaillé dans [listes et tuples](collections.md).

> **Astuce :** une lambda sert surtout comme argument d'une autre fonction. Pour tout
> le reste, un vrai `def` avec un nom est plus clair.

## Une fonction qui s'appelle elle-même

Une fonction peut s'appeler elle-même : c'est la
[récursion](../debuter/glossaire.md#recursion). Il faut un **cas d'arrêt**, sinon elle
ne s'arrête jamais.

```python
def compte_a_rebours(n):
    if n == 0:
        print("Partez !")
        return
    print(n)
    compte_a_rebours(n - 1)

compte_a_rebours(3)
```

```sortie
3
2
1
Partez !
```

Un exemple classique est la factorielle : `5!` vaut `5 × 4 × 3 × 2 × 1`. Autrement dit,
`n!` vaut `n × (n - 1)!`, et `1!` vaut 1.

```python
def factorielle(n):
    if n <= 1:
        return 1
    return n * factorielle(n - 1)

print(factorielle(5))
```

```sortie
120
```

> **Note :** Python limite la profondeur des appels (environ 1000). Pour de très longues
> répétitions, une boucle est préférable.

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

**Question 3 :** que reçoit `args` dans `def f(*args)` appelée avec `f(1, 2, 3)` ?

> **Réponse**
>
> Le tuple `(1, 2, 3)`. L'étoile range tous les arguments positionnels dans un tuple.

## En résumé

- `def nom(paramètres):` définit une fonction ; `nom(arguments)` l'appelle.
- `return` renvoie une valeur et quitte la fonction ; sans `return`, elle renvoie
  `None`.
- Les paramètres peuvent avoir une valeur par défaut ; les arguments peuvent être
  nommés ; `*args` et `**kwargs` acceptent un nombre variable d'arguments.
- Les variables d'une fonction sont locales ; évitez `global`.
- Une fonction est une valeur : on peut la passer à une autre, ou l'écrire en `lambda`.
- Une fonction récursive s'appelle elle-même et a toujours un cas d'arrêt.

Suite : [travailler avec du texte](texte.md). Comparez avec
[les fonctions en Ruby](../ruby/fonctions.md).
