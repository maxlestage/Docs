# Les annotations de type

Cette page montre comment indiquer les types attendus dans votre code, comment les dataclasses écrivent vos classes à votre place, et comment l'outil mypy repère les erreurs avant l'exécution.

> **Objectifs :**
>
> - vous saurez annoter des variables et des fonctions (`int`, `list[int]`, `dict[str, int]`) ;
> - vous saurez exprimer « une valeur ou rien » avec `X | None` ;
> - vous saurez écrire une classe de données avec `@dataclass` ;
> - vous saurez décrire un dictionnaire avec `TypedDict` et un comportement avec `Protocol` ;
> - vous saurez vérifier votre code avec mypy.

## Pourquoi annoter ?

Python ne demande jamais le [type](../debuter/glossaire.md#type) d'une variable : il le
découvre pendant l'exécution. C'est souple, mais une erreur de type n'apparaît qu'au
moment où la ligne s'exécute, parfois des semaines plus tard.

Une **annotation de type** (*type hint*) est une étiquette qui dit quel type vous
attendez. Elle sert de documentation, aide votre éditeur à compléter le code, et permet à
un outil de vérification de trouver des erreurs **sans lancer le programme**.

> **À retenir :** Python lui-même **ignore** les annotations à l'exécution. Elles ne
> changent rien au comportement du programme. Ce sont des outils comme mypy qui les
> vérifient.

```python
age: int = "dix"
print(age)
```

```sortie
dix
```

Aucune erreur : Python n'a pas regardé l'annotation. Nous verrons plus bas comment mypy,
lui, la signale.

## Annoter variables et fonctions

Pour une variable, on écrit `: type` après le nom. Pour une fonction, on annote chaque
paramètre, puis le type renvoyé après une flèche `->`.

```python
nom: str = "Alice"
age: int = 30
taille: float = 1.68
majeur: bool = True

def saluer(prenom: str, fois: int = 1) -> str:
    return ("Bonjour " + prenom + " ! ") * fois

def afficher(message: str) -> None:
    print(message)

afficher(saluer(nom, 2))
```

```sortie
Bonjour Alice ! Bonjour Alice ! 
```

`-> None` veut dire : « cette fonction ne renvoie rien d'utile ».

> **Astuce :** inutile d'annoter chaque variable. Pour `age = 30`, l'outil devine tout
> seul `int`. Annotez surtout les **paramètres** et les **retours** des fonctions.

## Les collections

Pour une collection, on indique aussi le type de ce qu'elle contient, entre crochets.

| Annotation | Signifie |
|------------|----------|
| `list[int]` | une liste d'entiers |
| `dict[str, int]` | un dictionnaire de clés `str` et de valeurs `int` |
| `set[str]` | un ensemble de chaînes |
| `tuple[str, int]` | un tuple de deux éléments : un `str` puis un `int` |
| `tuple[int, ...]` | un tuple d'entiers, de n'importe quelle longueur |

```python
def moyenne(notes: list[float]) -> float:
    return sum(notes) / len(notes)

def compter_mots(texte: str) -> dict[str, int]:
    compte: dict[str, int] = {}
    for mot in texte.split():
        compte[mot] = compte.get(mot, 0) + 1
    return compte

def min_max(nombres: list[int]) -> tuple[int, int]:
    return min(nombres), max(nombres)

print(moyenne([12, 15.5, 9]))
print(compter_mots("le chat et le chien"))
print(min_max([4, 1, 9]))
```

```sortie
12.166666666666666
{'le': 2, 'chat': 1, 'et': 1, 'chien': 1}
(1, 9)
```

## Une valeur ou rien : X | None

Une fonction qui cherche quelque chose peut ne rien trouver et renvoyer `None`. On
l'écrit `str | None` : « un `str`, ou bien `None` ».

```python
def trouver(noms: list[str], debut: str) -> str | None:
    for nom in noms:
        if nom.startswith(debut):
            return nom
    return None

resultat = trouver(["Alice", "Bob"], "B")
if resultat is not None:
    print(resultat.upper())
print(trouver(["Alice", "Bob"], "Z"))
```

```sortie
BOB
None
```

Le `if resultat is not None` est important : à l'intérieur, l'outil sait que `resultat`
est un `str`, et autorise `.upper()`.

### Optional et Union

Dans du code plus ancien, vous verrez les mêmes idées écrites avec le module `typing` :

| Ancienne écriture | Écriture moderne |
|-------------------|------------------|
| `Optional[str]` | `str \| None` |
| `Union[int, str]` | `int \| str` |

```python
from typing import Optional, Union

def doubler(x: Union[int, str]) -> Union[int, str]:
    return x * 2

def initiale(nom: Optional[str]) -> str:
    return nom[0] if nom else "?"

print(doubler(4), doubler("ab"))
print(initiale("Zoé"), initiale(None))
```

```sortie
8 abab
Z ?
```

Préférez l'écriture avec `|` : elle est plus courte et ne demande pas d'import.

## Les dataclasses

Écrire `__init__`, `__repr__` et `__eq__` pour chaque petite classe est répétitif (voir
[classes et objets](types.md)). Le décorateur `@dataclass` les **génère pour vous** à
partir des annotations.

```python
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

p = Point(1, 2)
print(p)
print(p.x + p.y)
print(p == Point(1, 2))
```

```sortie
Point(x=1, y=2)
3
True
```

Trois lignes suffisent : chaque annotation devient un **champ**, c'est-à-dire un
paramètre de `__init__` et un attribut.

### Valeurs par défaut

Un champ peut avoir une valeur par défaut. Pour une liste ou un dictionnaire, utilisez
`field(default_factory=list)` : chaque objet reçoit ainsi **sa propre** liste.

```python
from dataclasses import dataclass, field

@dataclass
class Eleve:
    nom: str
    classe: str = "6e A"
    notes: list[int] = field(default_factory=list)

    def moyenne(self) -> float:
        return sum(self.notes) / len(self.notes) if self.notes else 0.0

alice = Eleve("Alice")
alice.notes.append(15)
alice.notes.append(12)
bob = Eleve("Bob", "5e B")
print(alice, alice.moyenne())
print(bob)
```

```sortie
Eleve(nom='Alice', classe='6e A', notes=[15, 12]) 13.5
Eleve(nom='Bob', classe='5e B', notes=[])
```

Une dataclass reste une classe normale : vous pouvez y ajouter des méthodes.

### frozen et order

Deux options utiles :

- `frozen=True` rend l'objet **immuable** : impossible de modifier ses champs ;
- `order=True` génère `<`, `>`, `<=`, `>=` : les objets se comparent champ par champ,
  dans l'ordre où ils sont écrits.

```python
from dataclasses import dataclass

@dataclass(frozen=True, order=True)
class Version:
    majeure: int
    mineure: int

versions = [Version(2, 1), Version(1, 9), Version(2, 0)]
print(sorted(versions))
print(max(versions))
print(Version(1, 0) < Version(1, 2))
```

```sortie
[Version(majeure=1, mineure=9), Version(majeure=2, mineure=0), Version(majeure=2, mineure=1)]
Version(majeure=2, mineure=1)
True
```

> **Note :** un objet `frozen` peut servir de clé de dictionnaire ou entrer dans un
> ensemble, car sa valeur ne changera jamais.

## TypedDict : décrire un dictionnaire

Parfois, vos données restent des dictionnaires (par exemple lues depuis du
[JSON](../debuter/glossaire.md#json)). `TypedDict` décrit les clés attendues et le type
de chaque valeur. À l'exécution, c'est un dictionnaire ordinaire.

```python
from typing import TypedDict

class Film(TypedDict):
    titre: str
    annee: int

def resumer(film: Film) -> str:
    return f"{film['titre']} ({film['annee']})"

alien: Film = {"titre": "Alien", "annee": 1979}
print(resumer(alien))
print(type(alien))
```

```sortie
Alien (1979)
<class 'dict'>
```

## Protocol : décrire un comportement

Au chapitre [héritage et polymorphisme](heritage.md), le *duck typing* acceptait tout
objet qui possède la bonne méthode. `Protocol` écrit cette règle sous forme de type :
« n'importe quel objet qui a une méthode `voler()` qui renvoie un `str` ».

```python
from typing import Protocol

class Volant(Protocol):
    def voler(self) -> str: ...

class Oiseau:
    def voler(self) -> str:
        return "Je bats des ailes"

class Avion:
    def voler(self) -> str:
        return "Je fais vrombir mes moteurs"

def decoller(v: Volant) -> None:
    print(v.voler())

decoller(Oiseau())
decoller(Avion())
```

```sortie
Je bats des ailes
Je fais vrombir mes moteurs
```

`Oiseau` et `Avion` n'héritent pas de `Volant` : il leur suffit d'avoir la méthode.
`...` (trois points) signifie « corps vide » : un protocole décrit, il ne fait rien.

## Vérifier avec mypy

**mypy** est le vérificateur de types le plus connu. Il ne fait pas partie de Python :
on l'installe avec pip (voir [modules, paquets et pip](modules.md)).

```sh
python3 -m pip install mypy
```

Prenons ce fichier `main.py`, plein d'erreurs de type :

```python norun
def moyenne(notes: list[int]) -> float:
    return sum(notes) / len(notes)

def saluer(nom: str) -> str:
    return "Bonjour " + nom

print(moyenne([12, 15]))
print(moyenne("12, 15"))
age: int = "dix"
saluer(42)
```

On lance mypy sur le fichier :

```sh
mypy main.py
```

```text
main.py:8: error: Argument 1 to "moyenne" has incompatible type "str"; expected "list[int]"  [arg-type]
main.py:9: error: Incompatible types in assignment (expression has type "str", variable has type "int")  [assignment]
main.py:10: error: Argument 1 to "saluer" has incompatible type "int"; expected "str"  [arg-type]
Found 3 errors in 1 file (checked 1 source file)
```

Chaque ligne donne le fichier, le numéro de ligne et le problème. mypy a trouvé les
trois erreurs **sans exécuter** le programme. Python, lui, se serait arrêté à la ligne 8
avec une `TypeError`, sans voir les deux suivantes.

> **Astuce :** la plupart des éditeurs (VS Code avec Pylance, PyCharm) font la même
> vérification en direct et soulignent les erreurs pendant que vous tapez.

## Erreurs fréquentes

### Oublier le cas None

```python norun
def trouver(noms: list[str], debut: str) -> str | None:
    for nom in noms:
        if nom.startswith(debut):
            return nom
    return None

resultat = trouver(["Alice", "Bob"], "B")
print(resultat.upper())
```

```text
main.py:8: error: Item "None" of "str | None" has no attribute "upper"  [union-attr]
Found 1 error in 1 file (checked 1 source file)
```

mypy rappelle que `resultat` peut valoir `None`. Testez-le d'abord :
`if resultat is not None:`. Sans ce test, le programme planterait le jour où rien n'est
trouvé.

### Une liste comme valeur par défaut d'une dataclass

```python norun
from dataclasses import dataclass

@dataclass
class Eleve:
    nom: str
    notes: list[int] = []
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    @dataclass
     ^^^^^^^^^
  ...
ValueError: mutable default <class 'list'> for field notes is not allowed: use default_factory
```

(La trace complète passe par plusieurs lignes du module `dataclasses`, raccourcies ici.)
Une seule liste serait partagée par tous les élèves. Écrivez
`notes: list[int] = field(default_factory=list)`.

### Modifier une dataclass gelée

```python norun
from dataclasses import dataclass

@dataclass(frozen=True)
class Point:
    x: int
    y: int

p = Point(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 9, in <module>
    p.x = 5
    ^^^
  File "<string>", line 16, in __setattr__
dataclasses.FrozenInstanceError: cannot assign to field 'x'
```

Un objet `frozen` ne se modifie pas : créez-en un nouveau. La fonction `replace` du
module `dataclasses` le fait pour vous : `p = replace(p, x=5)`.

## Testez-vous

**Question 1 :** quelle annotation pour un dictionnaire qui associe un nom de ville à une
liste de températures décimales ?

> **Réponse**
>
> `dict[str, list[float]]`.

**Question 2 :** ce programme lève-t-il une erreur quand on le lance avec `python3` ?

```python norun
def double(x: int) -> int:
    return x * 2

print(double("ha"))
```

> **Réponse**
>
> Non : il affiche `haha`. Python ignore les annotations. C'est mypy qui signalerait que
> `"ha"` n'est pas un `int`.

**Question 3 :** écrivez une dataclass `Livre` avec un titre, un auteur et un nombre de
pages (par défaut 100).

> **Réponse**
>
> ```python
> from dataclasses import dataclass
>
> @dataclass
> class Livre:
>     titre: str
>     auteur: str
>     pages: int = 100
>
> print(Livre("Le Petit Prince", "Saint-Exupéry"))
> ```
>
> ```sortie
> Livre(titre='Le Petit Prince', auteur='Saint-Exupéry', pages=100)
> ```

## En résumé

- `x: int` et `def f(a: str) -> bool:` annotent les types ; Python les ignore à
  l'exécution.
- `list[int]`, `dict[str, int]`, `tuple[str, int]` précisent le contenu des collections.
- `X | None` (ou `Optional[X]`) : une valeur qui peut manquer ; testez `is not None`.
- `@dataclass` génère `__init__`, `__repr__` et `__eq__` ; `frozen` et `order` ajoutent
  l'immuabilité et le tri.
- `TypedDict` décrit un dictionnaire, `Protocol` un comportement.
- `mypy main.py` trouve les erreurs de type sans lancer le programme.

Suite : [itérateurs et générateurs](iterateurs.md)
