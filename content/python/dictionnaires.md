# Dictionnaires et ensembles

Cette page explique comment associer des clés à des valeurs avec les dictionnaires, et comment manipuler des collections sans doublons avec les ensembles.

> **Objectifs :**
>
> - vous saurez créer un dictionnaire, lire et modifier ses valeurs ;
> - vous saurez parcourir ses clés, ses valeurs ou les deux ;
> - vous saurez compter des mots avec un dictionnaire, puis avec `Counter` ;
> - vous saurez utiliser un ensemble et ses opérations (union, intersection...) ;
> - vous saurez choisir entre liste, tuple, dictionnaire et ensemble.

## Créer un dictionnaire

Un [dictionnaire](../debuter/glossaire.md#dictionnaire) (`dict`) associe des **clés** à
des **valeurs**. Pensez à un vrai dictionnaire : on cherche un mot (la clé) pour
trouver sa définition (la valeur). Ou à un répertoire : un nom donne un numéro.

On l'écrit entre accolades `{ }`, avec `clé: valeur` séparés par des virgules.

```python
ages = {"Alice": 30, "Bob": 25, "Chloé": 35}
capitales = {"France": "Paris", "Italie": "Rome"}
vide = {}

print(ages)
print(len(ages))
print(capitales["Italie"])
print(vide)
```

```sortie
{'Alice': 30, 'Bob': 25, 'Chloé': 35}
3
Rome
{}
```

Ligne par ligne :

- `ages` associe trois prénoms (les clés) à trois âges (les valeurs) ;
- `capitales` associe des pays à leur capitale ; `vide` ne contient rien du tout ;
- `print(ages)` affiche tout le dictionnaire, et `len(ages)` compte ses paires : 3 ;
- `capitales["Italie"]` cherche la clé `"Italie"` et donne sa valeur, `"Rome"`.

- Les clés sont souvent des textes ou des nombres. Chaque clé est **unique**.
- Les valeurs peuvent être n'importe quoi, même des listes ou d'autres dictionnaires.
- Le dictionnaire garde l'ordre dans lequel les clés ont été ajoutées.

On peut aussi le créer avec `dict(...)` et des arguments nommés :

```python
personne = dict(nom="Alice", ville="Lyon")
print(personne)
```

```sortie
{'nom': 'Alice', 'ville': 'Lyon'}
```

## Lire une valeur : [] ou get

Lire une valeur, c'est le geste le plus courant : chercher le prix d'un article, l'âge
d'une personne. Mais il faut aussi prévoir le cas où la clé n'y est pas, comme un mot
absent du dictionnaire.

Il y a deux façons de lire une valeur :

- `d[cle]` : si la clé n'existe pas, c'est une erreur (`KeyError`) ;
- `d.get(cle)` : si la clé n'existe pas, renvoie `None`, ou la valeur par défaut que
  vous donnez.

```python
stock = {"pomme": 12, "poire": 0}
print(stock["pomme"])
print(stock.get("pomme"))
print(stock.get("kiwi"))
print(stock.get("kiwi", 0))
print("kiwi" in stock)
```

```sortie
12
12
None
0
False
```

Ligne par ligne :

- `stock["pomme"]` et `stock.get("pomme")` donnent la même chose : 12 ;
- `stock.get("kiwi")` ne trouve pas la clé `"kiwi"` : il renvoie `None`, sans erreur ;
- `stock.get("kiwi", 0)` renvoie la valeur de secours que vous donnez, 0 ;
- `"kiwi" in stock` pose la question « la clé existe-t-elle ? ». Réponse : `False`.

`in` vérifie si une **clé** existe (pas une valeur).

> **À retenir :** utilisez `d[cle]` quand la clé doit forcément exister (son absence
> serait un bug). Utilisez `d.get(cle, defaut)` quand elle peut manquer.

## Ajouter, modifier, supprimer

Un dictionnaire n'est pas figé. Comme un carnet d'adresses, il change : un ami arrive,
un autre change de numéro, un troisième s'en va.

Pour ajouter ou modifier, on affecte une valeur à une clé. Si la clé existe, sa valeur
est remplacée ; sinon, elle est créée.

```python
ages = {"Alice": 30}
ages["Bob"] = 25
ages["Alice"] = 31
print(ages)

ages.update({"Chloé": 35, "Bob": 26})
print(ages)

del ages["Bob"]
print(ages)

age_chloe = ages.pop("Chloé")
print(age_chloe, ages)
```

```sortie
{'Alice': 31, 'Bob': 25}
{'Alice': 31, 'Bob': 26, 'Chloé': 35}
{'Alice': 31, 'Chloé': 35}
35 {'Alice': 31}
```

Ligne par ligne :

- au départ, `ages` ne contient qu'Alice, avec 30 ;
- `ages["Bob"] = 25` crée la clé `"Bob"` ; `ages["Alice"] = 31` remplace 30 par 31 ;
- `update` change l'âge de Bob (26) et ajoute Chloé (35) ;
- `del` efface Bob ; `pop` efface Chloé et range son âge, 35, dans `age_chloe`.

- `d[cle] = valeur` ajoute ou remplace ;
- `update` ajoute ou remplace plusieurs clés d'un coup ;
- `del d[cle]` supprime ; `pop(cle)` supprime et renvoie la valeur.

**À vous :** écrivez un programme qui crée un dictionnaire `prix` avec `"pain": 1` et
`"lait": 2`, puis ajoute `"beurre": 3`, passe le prix du pain à 2 et affiche le
dictionnaire.

> **Solution**
>
> ```python
> prix = {"pain": 1, "lait": 2}
> prix["beurre"] = 3
> prix["pain"] = 2
> print(prix)
> ```
>
> ```sortie
> {'pain': 2, 'lait': 2, 'beurre': 3}
> ```

## Clés, valeurs et paires

Parfois, vous n'avez besoin que d'une partie : les prénoms pour une feuille d'appel, ou
seulement les notes pour calculer une moyenne.

Trois méthodes donnent une vue sur le contenu :

- `keys()` : les clés ;
- `values()` : les valeurs ;
- `items()` : les paires `(clé, valeur)`.

```python
notes = {"Alice": 15, "Bob": 12, "Chloé": 18}
print(list(notes.keys()))
print(list(notes.values()))
print(list(notes.items()))
print(sum(notes.values()) / len(notes))
```

```sortie
['Alice', 'Bob', 'Chloé']
[15, 12, 18]
[('Alice', 15), ('Bob', 12), ('Chloé', 18)]
15.0
```

Ligne par ligne :

- `keys()` donne les prénoms ; `list(...)` les range dans une liste pour les afficher ;
- `values()` donne les notes, et `items()` les paires prénom-note ;
- la dernière ligne additionne les notes (45) et divise par le nombre d'élèves (3) :
  la moyenne vaut 15.0.

## Parcourir un dictionnaire

Souvent, vous voulez traiter chaque entrée à son tour, comme on lit un bulletin ligne
par ligne : chaque élève avec sa note.

Une boucle `for` sur un dictionnaire parcourt ses **clés**. Pour avoir la clé et la
valeur ensemble, parcourez `items()` et découpez chaque paire.

```python
notes = {"Alice": 15, "Bob": 12, "Chloé": 18}

for nom in notes:
    print(nom)

for nom, note in notes.items():
    print(f"{nom} : {note}/20")
```

```sortie
Alice
Bob
Chloé
Alice : 15/20
Bob : 12/20
Chloé : 18/20
```

Ligne par ligne :

- la première boucle donne à `nom` chaque clé, l'une après l'autre : `"Alice"`, `"Bob"`,
  puis `"Chloé"` ;
- la seconde boucle reçoit à chaque tour une paire, par exemple `("Alice", 15)` ;
- `nom, note` coupe cette paire en deux : `nom` vaut `"Alice"` et `note` vaut 15 ;
- le `f"..."` remplace `{nom}` et `{note}` par leurs valeurs.

Pour parcourir dans un autre ordre, triez. `sorted(notes.items(), key=...)` trie les
paires, par exemple par note décroissante :

```python
notes = {"Alice": 15, "Bob": 12, "Chloé": 18}
for nom, note in sorted(notes.items(), key=lambda paire: paire[1], reverse=True):
    print(nom, note)
```

```sortie
Chloé 18
Alice 15
Bob 12
```

**À vous :** écrivez un programme qui parcourt
`prix = {"pain": 1, "lait": 2, "beurre": 3}` et affiche une ligne par article, sous la
forme « pain : 1 € ».

> **Solution**
>
> ```python
> prix = {"pain": 1, "lait": 2, "beurre": 3}
> for article, montant in prix.items():
>     print(f"{article} : {montant} €")
> ```
>
> ```sortie
> pain : 1 €
> lait : 2 €
> beurre : 3 €
> ```

## Des valeurs complexes

Une valeur peut être une liste ou un autre dictionnaire. On construit ainsi des données
riches, comme une fiche.

```python
eleve = {
    "nom": "Alice",
    "notes": [15, 12, 17],
    "adresse": {"ville": "Lyon", "code": "69001"},
}
print(eleve["notes"][0])
print(eleve["adresse"]["ville"])
eleve["notes"].append(19)
print(eleve["notes"])
```

```sortie
15
Lyon
[15, 12, 17, 19]
```

Ligne par ligne :

- `eleve["notes"]` est une liste ; `[0]` prend son premier élément, 15 ;
- `eleve["adresse"]` est un dictionnaire ; `["ville"]` y cherche la ville, `"Lyon"` ;
- `append(19)` ajoute une note à la fin de la liste rangée dans la fiche.

Lisez `eleve["adresse"]["ville"]` de gauche à droite : la fiche, puis son adresse,
puis la ville de cette adresse.

## Compter des mots

Compter est l'usage le plus classique d'un dictionnaire : la clé est le mot, la valeur
est son nombre d'apparitions.

```python
texte = "le chat et le chien et le lapin"
compteur = {}
for mot in texte.split():
    compteur[mot] = compteur.get(mot, 0) + 1
print(compteur)
```

```sortie
{'le': 3, 'chat': 1, 'et': 2, 'chien': 1, 'lapin': 1}
```

Ligne par ligne :

- `texte.split()` coupe la phrase en mots : `"le"`, `"chat"`, `"et"`… ;
- `compteur` commence vide ;
- au premier `"le"`, `compteur.get("le", 0)` vaut 0 : Python range 0 + 1, soit 1 ;
- au deuxième `"le"`, il vaut 1 : Python range 2. Au troisième, 3.

`compteur.get(mot, 0)` renvoie le compte actuel, ou 0 si le mot est nouveau. On ajoute
1, puis on range le résultat.

**À vous :** écrivez un programme qui compte combien de fois chaque fruit apparaît
dans `achats = ["pomme", "poire", "pomme", "kiwi", "pomme"]`, avec un dictionnaire et
`get`, puis affiche le dictionnaire.

> **Solution**
>
> ```python
> achats = ["pomme", "poire", "pomme", "kiwi", "pomme"]
> compte = {}
> for fruit in achats:
>     compte[fruit] = compte.get(fruit, 0) + 1
> print(compte)
> ```
>
> ```sortie
> {'pomme': 3, 'poire': 1, 'kiwi': 1}
> ```

### Avec collections.Counter

Le module `collections` fournit `Counter`, un dictionnaire spécialisé pour compter. Il
fait tout le travail en une ligne, et sait donner les plus fréquents.

```python
from collections import Counter

texte = "le chat et le chien et le lapin"
compteur = Counter(texte.split())
print(compteur["le"])
print(compteur["tigre"])
print(compteur.most_common(2))
print(Counter("abracadabra").most_common(3))
```

```sortie
3
0
[('le', 3), ('et', 2)]
[('a', 5), ('b', 2), ('r', 2)]
```

Un mot absent vaut 0, sans erreur. `most_common(n)` donne les `n` éléments les plus
fréquents.

### Avec defaultdict

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`defaultdict` crée automatiquement une valeur de départ pour une clé absente. On lui
donne le type de cette valeur : `list` pour une liste vide, `int` pour 0.

```python
from collections import defaultdict

mots = ["avion", "bateau", "ananas", "banane", "cerise"]
par_lettre = defaultdict(list)
for mot in mots:
    par_lettre[mot[0]].append(mot)

print(dict(par_lettre))
```

```sortie
{'a': ['avion', 'ananas'], 'b': ['bateau', 'banane'], 'c': ['cerise']}
```

Sans `defaultdict`, il faudrait vérifier à chaque fois si la clé existe avant
d'ajouter à sa liste.

## Les ensembles

Parfois, seule compte la présence d'un élément, pas son nombre ni sa place. Pensez à la
liste des invités d'une fête : personne ne doit y être compté deux fois.

Un [ensemble](../debuter/glossaire.md#ensemble) (`set`) est une collection **sans
doublons** et **sans ordre**. Pensez à un sac de billes : chaque bille n'y est qu'une
fois, et elles ne sont pas rangées. On l'écrit entre accolades, sans `:`.

```python
couleurs = {"rouge", "vert", "bleu", "rouge"}
print(len(couleurs))
print("vert" in couleurs)

couleurs.add("jaune")
couleurs.discard("rouge")
print(sorted(couleurs))

notes = [12, 15, 12, 18, 15]
print(sorted(set(notes)))
```

```sortie
3
True
['bleu', 'jaune', 'vert']
[12, 15, 18]
```

- le doublon `"rouge"` n'est gardé qu'une fois ;
- `add` ajoute, `discard` retire (sans erreur si l'élément est absent) ;
- `set(liste)` supprime les doublons d'une liste.

Comme un ensemble n'a pas d'ordre, on le trie avec `sorted` avant de l'afficher.

> **Attention :** `{}` crée un dictionnaire vide, pas un ensemble. Pour un ensemble
> vide, écrivez `set()`.

**À vous :** écrivez un programme qui part de la liste
`invites = ["Alice", "Bob", "Alice", "Chloé", "Bob"]`, retire les doublons avec un
ensemble, puis affiche le nombre d'invités et leurs noms triés.

> **Solution**
>
> ```python
> invites = ["Alice", "Bob", "Alice", "Chloé", "Bob"]
> uniques = set(invites)
> print(len(uniques))
> print(sorted(uniques))
> ```
>
> ```sortie
> 3
> ['Alice', 'Bob', 'Chloé']
> ```

### Les opérations d'ensembles

Comparer deux groupes est fréquent : quels loisirs deux amis ont-ils en commun ? Lesquels
n'appartiennent qu'à l'un des deux ?

Les ensembles savent faire les opérations des maths :

| Opération | Méthode | Contient |
| --- | --- | --- |
| union | `a.union(b)` | les éléments de `a` **ou** de `b` |
| intersection | `a.intersection(b)` | les éléments de `a` **et** de `b` |
| différence | `a.difference(b)` | les éléments de `a` qui ne sont pas dans `b` |
| différence symétrique | `a.symmetric_difference(b)` | les éléments d'un seul des deux |

Chaque opération a aussi un signe plus court : `a | b` pour l'union, `a & b` pour
l'intersection, `a - b` pour la différence et `a ^ b` pour la différence symétrique.

```python
alice = {"piano", "tennis", "lecture"}
bob = {"tennis", "cuisine", "lecture", "vélo"}

print(sorted(alice | bob))
print(sorted(alice & bob))
print(sorted(alice - bob))
print(sorted(alice ^ bob))
print({"tennis"} <= alice)
```

```sortie
['cuisine', 'lecture', 'piano', 'tennis', 'vélo']
['lecture', 'tennis']
['piano']
['cuisine', 'piano', 'vélo']
True
```

Ligne par ligne :

- `alice | bob` réunit tous les loisirs des deux, sans doublon ;
- `alice & bob` garde ceux qu'ils ont en commun : la lecture et le tennis ;
- `alice - bob` garde ceux d'Alice que Bob n'a pas : le piano ;
- `alice ^ bob` garde ceux qu'un seul des deux pratique.

`<=` vérifie qu'un ensemble est **inclus** dans un autre.

> **Astuce :** `in` est très rapide sur un ensemble, même avec un million d'éléments.
> Sur une liste, Python doit tout parcourir. Pour de nombreuses recherches, préférez un
> ensemble.

### frozenset : un ensemble figé

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un `frozenset` est un ensemble [immuable](../debuter/glossaire.md#immuable) : on ne
peut plus rien ajouter ni retirer. Il peut donc servir de clé de dictionnaire, ce qui
est interdit pour un `set` ordinaire.

```python
voyelles = frozenset("aeiouy")
print("e" in voyelles)

trajets = {frozenset({"Paris", "Lyon"}): 465}
print(trajets[frozenset({"Lyon", "Paris"})])
```

```sortie
True
465
```

L'ordre ne compte pas dans un ensemble : `{"Paris", "Lyon"}` et `{"Lyon", "Paris"}`
désignent le même trajet.

## Quelle collection choisir ?

Vous connaissez maintenant quatre façons de ranger des données. Comme des boîtes de
rangement, chacune convient à un usage précis.

| Collection | Écriture | Ordonnée | Modifiable | Doublons | Usage typique |
| --- | --- | --- | --- | --- | --- |
| `list` | `[1, 2, 3]` | oui | oui | oui | une suite d'éléments : courses, notes |
| `tuple` | `(1, 2)` | oui | non | oui | un groupe fixe : coordonnées, date |
| `dict` | `{"a": 1}` | oui (ordre d'ajout) | oui | clés uniques | retrouver une valeur par sa clé |
| `set` | `{1, 2}` | non | oui | non | éliminer les doublons, tester l'appartenance |
| `frozenset` | `frozenset({1, 2})` | non | non | non | un ensemble qui sert de clé |

Posez-vous deux questions :

- « Je cherche par **position** ou par **nom** ? » Position : liste ou tuple. Nom :
  dictionnaire.
- « Les doublons ont-ils un sens ? » Non : ensemble.

## Erreurs fréquentes

### Lire une clé absente

```python norun
ages = {"Alice": 30}
print(ages["Bob"])
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    print(ages["Bob"])
          ~~~~^^^^^^^
KeyError: 'Bob'
```

La clé `"Bob"` n'existe pas. Utilisez `ages.get("Bob")`, ou vérifiez avant avec
`if "Bob" in ages:`.

### Une liste comme clé

```python norun
d = {[1, 2]: "a"}
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    d = {[1, 2]: "a"}
        ^^^^^^^^^^^^^
TypeError: unhashable type: 'list'
```

Une clé doit être immuable : une liste peut changer, elle est refusée (*unhashable*).
Utilisez un tuple : `{(1, 2): "a"}`.

### Supprimer pendant le parcours

```python norun
stock = {"pomme": 0, "poire": 3}
for fruit in stock:
    if stock[fruit] == 0:
        del stock[fruit]
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    for fruit in stock:
                 ^^^^^
RuntimeError: dictionary changed size during iteration
```

On ne peut pas changer la taille d'un dictionnaire pendant qu'on le parcourt.
Parcourez une copie des clés : `for fruit in list(stock):`.

### Croire que {} est un ensemble vide

```python norun
s = {}
s.add(1)
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 2, in <module>
    s.add(1)
    ^^^^^
AttributeError: 'dict' object has no attribute 'add'
```

`{}` est un dictionnaire vide. Pour un ensemble vide, écrivez `s = set()`.

## Testez-vous

**Question 1 :** que renvoie `{"a": 1}.get("b", 5)` ?

> **Réponse**
>
> `5`. La clé `"b"` n'existe pas, donc `get` renvoie la valeur par défaut.

**Question 2 :** comment compter les lettres de `"mississippi"` et afficher la plus
fréquente ?

> **Réponse**
>
> ```python
> from collections import Counter
>
> print(Counter("mississippi").most_common(1))
> ```
>
> ```sortie
> [('i', 4)]
> ```
>
> `i` et `s` apparaissent 4 fois chacun ; à égalité, `Counter` garde l'ordre
> d'apparition, donc `i` d'abord.

**Question 3 :** quels élèves sont inscrits aux deux clubs ?

```python norun
echecs = {"Alice", "Bob", "Chloé"}
theatre = {"Bob", "David", "Chloé"}
```

> **Réponse**
>
> L'intersection :
>
> ```python
> echecs = {"Alice", "Bob", "Chloé"}
> theatre = {"Bob", "David", "Chloé"}
> print(sorted(echecs & theatre))
> ```
>
> ```sortie
> ['Bob', 'Chloé']
> ```

## En résumé

- Un dictionnaire `{clé: valeur}` retrouve une valeur par sa clé ; les clés sont
  uniques et immuables.
- `d[cle]` échoue si la clé manque ; `d.get(cle, defaut)` non.
- `keys()`, `values()` et `items()` donnent les clés, les valeurs et les paires ;
  `for cle, valeur in d.items():` parcourt tout.
- `Counter` compte, `defaultdict` crée les valeurs de départ tout seul.
- Un ensemble `{...}` n'a ni ordre ni doublons ; `|`, `&`, `-` et `^` le combinent ;
  `set()` est l'ensemble vide.

Suite : [les compréhensions](comprehensions.md).
