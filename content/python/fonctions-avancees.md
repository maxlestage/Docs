# Aller plus loin avec les fonctions

Cette page montre des usages plus avancés des fonctions Python : un nombre variable d'arguments, les docstrings, les fonctions passées comme valeurs, les `lambda` et la récursion.

> **Objectifs :**
>
> - vous saurez accepter un nombre variable d'arguments avec `*args` et `**kwargs` ;
> - vous saurez écrire une `lambda` et la donner à `sorted` ;
> - vous saurez écrire une fonction récursive avec un cas d'arrêt.

Ce chapitre suit [les fonctions](fonctions.md) : vous y retrouverez `def`, les paramètres et `return`, cette fois pour aller un peu plus loin.

## Un nombre variable d'arguments : *args et **kwargs

Parfois, vous ne savez pas à l'avance combien de valeurs vous allez recevoir. Une
addition au restaurant peut compter deux plats ou dix. Les étoiles permettent à une
fonction d'accepter autant d'arguments que nécessaire.

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

Ligne par ligne :

- `*nombres` range tous les arguments reçus dans un tuple : `(1, 2)`, puis
  `(1, 2, 3, 4)`, puis `()` ;
- `total = 0` part de zéro ;
- la boucle `for` ajoute chaque nombre à `total` ;
- `return total` renvoie la somme. Sans argument, la boucle ne tourne pas : 0.

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

## Les fonctions sont des valeurs

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Parfois, une fonction ne sert qu'une fois, pour une toute petite tâche, comme dire
à `sorted` selon quoi trier. Lui donner un nom avec `def` serait lourd.

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

Ligne par ligne :

- `carre = lambda x: x * x` crée une petite fonction et la range dans `carre` ;
- `carre(5)` calcule `5 * 5` et renvoie 25 ;
- `eleves` est une liste de paires (prénom, note) ;
- `sorted(..., key=lambda eleve: eleve[1])` regarde la note de chaque paire, et
  range les élèves de la plus petite note à la plus grande.

`key=lambda eleve: eleve[1]` veut dire « trie selon le deuxième élément ». Le tri est
détaillé dans [listes et tuples](collections.md).

> **Astuce :** une lambda sert surtout comme argument d'une autre fonction. Pour tout
> le reste, un vrai `def` avec un nom est plus clair.

**À vous :** la liste `amis = [("Léa", 25), ("Tom", 19), ("Inès", 31)]` contient des
prénoms et des âges. Affichez-la triée par âge, avec `sorted` et une lambda.

> **Solution**
>
> ```python
> amis = [("Léa", 25), ("Tom", 19), ("Inès", 31)]
> print(sorted(amis, key=lambda ami: ami[1]))
> ```
>
> ```sortie
> [('Tom', 19), ('Léa', 25), ('Inès', 31)]
> ```

## Une fonction qui s'appelle elle-même

Certains problèmes contiennent une version plus petite d'eux-mêmes. Compter à rebours
depuis 3, c'est dire « 3 », puis compter à rebours depuis 2. Une fonction peut suivre
la même idée.

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

Ligne par ligne :

- `compte_a_rebours(3)` : `n` vaut 3, ce n'est pas 0. La fonction affiche 3, puis
  s'appelle avec 2 ;
- même chose avec 2, puis avec 1 : chaque appel affiche son nombre ;
- avec 0, le `if` est vrai : la fonction affiche « Partez ! » et `return` arrête tout.
  C'est le cas d'arrêt.

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

### Oublier le cas d'arrêt

```python norun
def compte_a_rebours(n):
    print(n)
    compte_a_rebours(n - 1)

compte_a_rebours(3)
```

```text
3
2
1
0
-1
...
Traceback (most recent call last):
  File "/home/vous/main.py", line 5, in <module>
    compte_a_rebours(3)
    ~~~~~~~~~~~~~~~~^^^
  File "/home/vous/main.py", line 3, in compte_a_rebours
    compte_a_rebours(n - 1)
    ~~~~~~~~~~~~~~~~^^^^^^^
  [Previous line repeated 996 more times]
RecursionError: maximum recursion depth exceeded
```

Sans cas d'arrêt, la fonction s'appelle sans fin : 3, 2, 1, 0, -1… Python l'arrête
au bout d'environ 1000 appels (*maximum recursion depth exceeded*, « profondeur
maximale de récursion dépassée »). Ajoutez le cas d'arrêt avant l'appel :
`if n == 0:` suivi de `return`.

### Appeler la fonction au lieu de la passer

```python norun
mots = ["kiwi", "ananas", "fraise", "pomme"]
print(sorted(mots, key=len()))
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(sorted(mots, key=len()))
                           ~~~^^
TypeError: len() takes exactly one argument (0 given)
```

Avec les parenthèses, `len()` **appelle** `len` tout de suite, sans argument. Pour
donner la fonction elle-même, écrivez son nom sans parenthèses : `key=len`.

### Donner des arguments nommés à *args

```python norun
def somme(*nombres):
    total = 0
    for n in nombres:
        total += n
    return total

print(somme(a=1, b=2))
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 7, in <module>
    print(somme(a=1, b=2))
          ~~~~~^^^^^^^^^^
TypeError: somme() got an unexpected keyword argument 'a'
```

`*nombres` ne reçoit que les arguments **positionnels** (*unexpected keyword
argument* : « argument nommé inattendu »). Écrivez `somme(1, 2)`, ou ajoutez un
paramètre `**kwargs` pour accepter les arguments nommés.

## Testez-vous

**Question 1 :** que reçoit `args` dans `def f(*args)` appelée avec `f(1, 2, 3)` ?

> **Réponse**
>
> Le tuple `(1, 2, 3)`. L'étoile range tous les arguments positionnels dans un tuple.

**Question 2 :** réécrivez `def triple(x): return x * 3` avec une lambda.

> **Réponse**
>
> ```python
> triple = lambda x: x * 3
> print(triple(4))
> ```
>
> ```sortie
> 12
> ```

**Question 3 :** pourquoi une fonction récursive a-t-elle besoin d'un cas d'arrêt ?

> **Réponse**
>
> Sans cas d'arrêt, elle s'appelle sans fin. Python finit par l'arrêter avec une
> `RecursionError`.

## En résumé

- `*args` range les arguments positionnels en trop dans un tuple ; `**kwargs` range
  les arguments nommés en trop dans un dictionnaire.
- Une docstring, juste sous la ligne `def`, explique ce que fait la fonction.
- Une fonction est une valeur : on peut la passer à une autre, ou l'écrire en `lambda`.
- Une fonction récursive s'appelle elle-même et a toujours un cas d'arrêt.

Suite : [travailler avec du texte](texte.md).
