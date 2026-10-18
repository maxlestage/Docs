# Bien écrire du Python

Cette page présente les règles de style de Python (la PEP 8), l'esprit du langage (le Zen de Python) et les tournures idiomatiques qui rendent un programme court, clair et facile à relire.

> **Objectifs :**
>
> - appliquer les règles essentielles de la PEP 8 et bien nommer les choses ;
> - connaître le Zen de Python ;
> - utiliser les tournures idiomatiques : `enumerate`, `zip`, compréhensions, `with`,
>   déballage, f-strings ;
> - documenter une fonction avec des annotations de type et une docstring ;
> - vérifier et formater son code avec ruff ou black.

## Pourquoi soigner son style ?

Un programme est lu beaucoup plus souvent qu'il n'est écrit : par vos collègues, et
par vous-même dans six mois. Un code bien présenté se comprend vite ; un code brouillon
cache ses bugs.

Python a un guide de style officiel, la **PEP 8** (*Python Enhancement Proposal* n° 8).
Presque tous les projets Python le suivent. Résultat : un programme Python ressemble à
un autre programme Python, et vous vous y retrouvez partout.

## Les noms

Un bon nom dit **ce que contient** la variable ou **ce que fait** la fonction. Chaque
sorte de nom a sa forme :

| Élément | Forme | Exemple |
| --- | --- | --- |
| [Variable](../debuter/glossaire.md#variable), [fonction](../debuter/glossaire.md#fonction), méthode | minuscules et `_` (*snake_case*) | `prix_total`, `calculer_moyenne` |
| [Classe](../debuter/glossaire.md#classe) | une majuscule à chaque mot (*CamelCase*) | `CompteBancaire` |
| [Constante](../debuter/glossaire.md#constante) | majuscules et `_` | `TAUX_TVA = 0.2` |
| [Module](../debuter/glossaire.md#module) (fichier) | minuscules, courtes | `calcul.py` |
| Usage interne | commence par `_` | `_cache` |
| Variable ignorée | un seul `_` | `for _ in range(3)` |

```python
TAUX_TVA = 0.2


class Article:
    def __init__(self, nom, prix_ht):
        self.nom = nom
        self.prix_ht = prix_ht

    def prix_ttc(self):
        return round(self.prix_ht * (1 + TAUX_TVA), 2)


pain = Article("pain", 1.5)
print(pain.nom, pain.prix_ttc())
```

```sortie
pain 1.8
```

> **Astuce :** évitez les noms d'une lettre (`x`, `l`, `d`) sauf pour un compteur très
> court. `notes` dit plus que `l`, et `l` se confond avec `1`.

## La mise en page

Les règles de présentation les plus importantes de la PEP 8 :

- **4 espaces** pour chaque niveau d'indentation, jamais de tabulation ;
- des lignes de **79 caractères** au plus (beaucoup de projets acceptent 88 ou 100) ;
- un espace autour de `=`, `+`, `==`… : `total = prix * 2`, pas `total=prix*2` ;
- pas d'espace autour du `=` d'un argument nommé : `round(x, ndigits=2)` ;
- **deux lignes vides** entre les fonctions et les classes, une entre les méthodes ;
- les `import` en haut du fichier, un par ligne.

```python
import math
import statistics


def aire_cercle(rayon):
    return math.pi * rayon**2


def mediane(valeurs):
    return statistics.median(valeurs)


print(round(aire_cercle(2), 2))
print(mediane([3, 1, 4, 1, 5]))
```

```sortie
12.57
3
```

## Le Zen de Python

Python cache un petit poème qui résume son esprit. Pour le lire, importez le module
`this` :

```python
import this
```

```sortie
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!
```

Quatre lignes à retenir, en français :

- « Le beau vaut mieux que le laid. »
- « L'explicite vaut mieux que l'implicite. » : dites clairement ce que vous faites.
- « La lisibilité compte. »
- « Les erreurs ne doivent jamais passer en silence. » : ne cachez pas une exception.

## Les tournures idiomatiques

Un code **idiomatique** utilise les tournures naturelles du langage, celles qu'un
habitué attend. Chaque exemple montre la version « avant », qui marche mais se lit mal,
puis la version « après ».

### enumerate plutôt que range(len(...))

Pour avoir à la fois l'indice et l'élément, utilisez `enumerate` (voir
[conditions et boucles](conditions-boucles.md)). `start=1` fait commencer le compte à 1.

```python
fruits = ["pomme", "poire", "kiwi"]

# Avant
for i in range(len(fruits)):
    print(i + 1, fruits[i])

# Après
for numero, fruit in enumerate(fruits, start=1):
    print(numero, fruit)
```

```sortie
1 pomme
2 poire
3 kiwi
1 pomme
2 poire
3 kiwi
```

### zip pour parcourir deux listes ensemble

```python
noms = ["Alice", "Bob"]
notes = [15, 12]

for nom, note in zip(noms, notes):
    print(f"{nom} : {note}")

print(dict(zip(noms, notes)))
```

```sortie
Alice : 15
Bob : 12
{'Alice': 15, 'Bob': 12}
```

### Les compréhensions plutôt qu'une boucle qui remplit une liste

Une [compréhension](comprehensions.md) construit une liste en une ligne lisible.

```python
nombres = [1, 2, 3, 4, 5, 6]

# Avant
carres_pairs = []
for n in nombres:
    if n % 2 == 0:
        carres_pairs.append(n * n)
print(carres_pairs)

# Après
print([n * n for n in nombres if n % 2 == 0])
```

```sortie
[4, 16, 36]
[4, 16, 36]
```

> **Attention :** une compréhension doit rester courte. Si elle déborde sur trois
> lignes avec plusieurs `if`, revenez à une boucle `for` : la lisibilité compte.

### with pour les fichiers

`with` ferme le fichier tout seul à la fin du bloc, même en cas d'erreur (voir
[les fichiers](fichiers.md)).

```python
with open("notes.txt", "w", encoding="utf-8") as fichier:
    fichier.write("Alice 15\nBob 12\n")

with open("notes.txt", encoding="utf-8") as fichier:
    for ligne in fichier:
        print(ligne.strip())
```

```sortie
Alice 15
Bob 12
```

### Le déballage

Le **déballage** (*unpacking*) range les éléments d'une séquence dans plusieurs
variables d'un coup. L'étoile `*` ramasse « tout le reste ».

```python
point = (3, 4)
x, y = point
print(x, y)

a, b = 1, 2
a, b = b, a
print(a, b)

nom, *notes = ["Alice", 15, 12, 18]
print(nom, notes)
```

```sortie
3 4
2 1
Alice [15, 12, 18]
```

`a, b = b, a` échange deux variables sans variable temporaire.

### EAFP : essayer plutôt que tout vérifier

Python préfère **essayer, puis gérer l'erreur** (*EAFP : Easier to Ask Forgiveness than
Permission*, « mieux vaut demander pardon que permission ») plutôt que tout vérifier
avant (*LBYL : Look Before You Leap*).

```python
stock = {"pomme": 3}

# Avant : vérifier d'abord
if "kiwi" in stock:
    print(stock["kiwi"])
else:
    print("pas de kiwi")

# Après : essayer, puis gérer l'erreur
try:
    print(stock["kiwi"])
except KeyError:
    print("pas de kiwi")

# Encore plus court pour un dictionnaire
print(stock.get("kiwi", 0))
```

```sortie
pas de kiwi
pas de kiwi
0
```

Voir [les erreurs et exceptions](erreurs.md) et [les dictionnaires](dictionnaires.md).

### Les f-strings pour assembler du texte

```python
nom = "Alice"
moyenne = 14.6667

# Avant
print("Élève : " + nom + ", moyenne : " + str(round(moyenne, 1)))

# Après
print(f"Élève : {nom}, moyenne : {moyenne:.1f}")
```

```sortie
Élève : Alice, moyenne : 14.7
Élève : Alice, moyenne : 14.7
```

## Annotations de type et docstrings

Deux habitudes rendent une fonction facile à utiliser sans lire son code :

- les **annotations de type** disent ce que la fonction reçoit et renvoie (voir
  [le typage](typage.md)) ;
- la **docstring**, une chaîne entre triples guillemets juste sous `def`, explique ce
  que fait la fonction.

```python
def moyenne(notes: list[float]) -> float | None:
    """Renvoie la moyenne des notes, ou None si la liste est vide."""
    if not notes:
        return None
    return sum(notes) / len(notes)


print(moyenne([12, 18]))
print(moyenne([]))
print(moyenne.__doc__)
```

```sortie
15.0
None
Renvoie la moyenne des notes, ou None si la liste est vide.
```

Python ne vérifie pas les annotations pendant l'exécution : ce sont des indications
pour le lecteur, l'éditeur et des outils comme mypy. `help(moyenne)` affiche la
docstring.

## Refactoriser : avant, après

**Refactoriser**, c'est améliorer le code sans changer ce qu'il fait. Voici une fonction
qui marche, mais qui accumule les maladresses :

```python
def Rapport(d):
    s = ""
    for k in d.keys():
        if d[k] >= 10:
            s = s + k + " : " + str(d[k]) + " (reçu)\n"
        else:
            s = s + k + " : " + str(d[k]) + " (refusé)\n"
    return s


print(Rapport({"Alice": 15, "Bob": 8}), end="")
```

```sortie
Alice : 15 (reçu)
Bob : 8 (refusé)
```

La même, réécrite : un nom de fonction en minuscules, des noms clairs, `items()`, une
expression conditionnelle, une f-string et `join` au lieu de concaténer en boucle.

```python
SEUIL_REUSSITE = 10


def rapport(notes: dict[str, int]) -> str:
    """Une ligne par élève, avec reçu ou refusé."""
    lignes = []
    for eleve, note in notes.items():
        verdict = "reçu" if note >= SEUIL_REUSSITE else "refusé"
        lignes.append(f"{eleve} : {note} ({verdict})")
    return "\n".join(lignes)


print(rapport({"Alice": 15, "Bob": 8}))
```

```sortie
Alice : 15 (reçu)
Bob : 8 (refusé)
```

Un autre exemple, avec des **clauses de garde** : on traite les cas particuliers tout
de suite, avec un `return`, au lieu d'imbriquer des `if`.

```python
# Avant
def prix_livraison(poids, pays):
    if poids > 0:
        if pays == "FR":
            return 5
        else:
            return 12
    else:
        return None


# Après
def prix_livraison_clair(poids, pays):
    if poids <= 0:
        return None
    if pays == "FR":
        return 5
    return 12


for args in [(2, "FR"), (2, "BE"), (0, "FR")]:
    print(prix_livraison(*args), prix_livraison_clair(*args))
```

```sortie
5 5
12 12
None None
```

> **À retenir :** « Flat is better than nested » : un code à plat se lit de haut en bas,
> sans compter les niveaux d'indentation.

## Vérifier et formater avec ruff et black

Inutile d'apprendre toutes les règles par cœur : des outils les vérifient pour vous. Ils
s'installent avec pip (voir [organiser un projet](projet.md)).

- **black** reformate le code tout seul : espaces, lignes vides, guillemets.
- **ruff** signale les problèmes (imports inutiles, mauvais noms…) et sait aussi
  formater (`ruff format`), très vite.

```sh
python3 -m pip install ruff black
```

Prenons un fichier mal écrit :

```python norun
# notes.py
import os
def Moyenne(Notes):
    Total=0
    for i in range(len(Notes)):
        Total+=Notes[i]
    if Total == None:
        return 0
    return Total/len(Notes)
print(Moyenne([12,18]))
```

`ruff check` liste les problèmes. Par défaut, il vérifie peu de règles ; `--select`
en active d'autres (`E` : style, `F` : erreurs probables, `N` : noms).

```sh
ruff check --output-format concise --select E,F,N notes.py
```

```text
notes.py:1:8: F401 [*] `os` imported but unused
notes.py:2:5: N802 Function name `Moyenne` should be lowercase
notes.py:2:13: N803 Argument name `Notes` should be lowercase
notes.py:3:5: N806 Variable `Total` in function should be lowercase
notes.py:5:9: N806 Variable `Total` in function should be lowercase
notes.py:6:17: E711 Comparison to `None` should be `cond is None`
Found 6 errors.
[*] 1 fixable with the `--fix` option (1 hidden fix can be enabled with the `--unsafe-fixes` option).
```

`black` corrige la mise en page, mais pas les noms ni la logique :

```sh
black notes.py
```

```text
reformatted notes.py

All done! ✨ 🍰 ✨
1 file reformatted.
```

> **Astuce :** configurez votre éditeur pour lancer le formateur à chaque
> enregistrement. Vous n'aurez plus jamais à penser aux espaces.

## Erreurs fréquentes

### Appeler une variable comme une fonction intégrée

`list`, `dict`, `str`, `sum`, `max`, `input`… sont des fonctions de Python. Si vous
donnez ce nom à une variable, la fonction disparaît.

```python norun
list = [3, 1, 2]
chiffres = list("123")
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 2, in <module>
    chiffres = list("123")
TypeError: 'list' object is not callable
```

Choisissez un nom qui dit ce que contient la variable : `nombres = [3, 1, 2]`.

### Une liste comme valeur par défaut

La valeur par défaut d'un paramètre est créée **une seule fois**, quand Python lit le
`def`. Une liste par défaut est donc partagée entre tous les appels.

```python norun
def ajouter(tache, taches=[]):
    taches.append(tache)
    return taches


print(ajouter("pain"))
print(ajouter("lait"))
```

```text
['pain']
['pain', 'lait']
```

Le second appel retrouve `"pain"` ! Écrivez `taches=None`, puis
`if taches is None: taches = []` au début de la fonction.

### Comparer une valeur avec is

`is` vérifie que deux noms désignent **le même objet**, pas la même valeur. Réservez-le
à `None` : `x is None`.

```python norun
reponse = 42
if reponse is 42:
    print("Bonne réponse")
```

```text
/home/alice/main.py:2: SyntaxWarning: "is" with 'int' literal. Did you mean "=="?
  if reponse is 42:
Bonne réponse
```

Écrivez `if reponse == 42:`.

## Testez-vous

**1.** Comment nommer une classe qui représente un compte bancaire ? Et une fonction qui
calcule un prix total ?

> **Réponse**
>
> La classe : `CompteBancaire` (*CamelCase*). La fonction : `calculer_prix_total`
> (*snake_case*).

**2.** Réécrivez `for i in range(len(noms)): print(i, noms[i])` de façon idiomatique.

> **Réponse**
>
> `for i, nom in enumerate(noms): print(i, nom)`.

**3.** Que fait `premier, *reste = [1, 2, 3]` ?

> **Réponse**
>
> `premier` reçoit `1` et `reste` reçoit la liste des autres éléments : `[2, 3]`.

## En résumé

- La PEP 8 fixe le style : 4 espaces, `snake_case` pour les fonctions et variables,
  `CamelCase` pour les classes, `MAJUSCULES` pour les constantes.
- `import this` affiche le Zen de Python : la lisibilité compte.
- Préférez `enumerate`, `zip`, les compréhensions, `with`, le déballage et les f-strings.
- Annotations de type et docstrings rendent une fonction compréhensible sans lire son
  code.
- ruff signale les problèmes, black (ou `ruff format`) reformate tout seul.

Suite : [Organiser un projet](projet.md)
