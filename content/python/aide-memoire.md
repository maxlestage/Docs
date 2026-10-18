# Aide-mémoire

Cette page résume en un coup d'œil la syntaxe Python vue dans tout le parcours, avec des tableaux compacts et de courts exemples.

## Bases

| Élément | Exemple |
| --- | --- |
| Variable | `age = 30` |
| Constante (convention) | `TAUX = 0.2` |
| Texte | `"Bonjour"`, `'Bonjour'` |
| f-string | `f"Bonjour {nom}"` |
| Booléens | `True`, `False` |
| Rien | `None` |
| Commentaire | `# texte` |
| Afficher | `print(a, b, sep=", ", end="")` |
| Lire le clavier | `nom = input("Nom : ")` (toujours du texte) |
| Type d'une valeur | `type(x)`, `isinstance(x, int)` |

```python
nom = "Alice"
age = 30
print(f"Bonjour {nom}, {age} ans")
print(type(age), type(3.5), type(nom), type(None))
```

```sortie
Bonjour Alice, 30 ans
<class 'int'> <class 'float'> <class 'str'> <class 'NoneType'>
```

Détails : [premiers pas](premiers-pas.md).

## Nombres

| Opération | Exemple | Résultat |
| --- | --- | --- |
| Division | `7 / 2` | `3.5` |
| Division entière | `7 // 2` | `3` |
| Reste | `7 % 2` | `1` |
| Puissance | `2 ** 10` | `1024` |
| Arrondi | `round(3.14159, 2)` | `3.14` |
| Conversion | `int("42")`, `float("2.5")`, `str(42)` | `42`, `2.5`, `"42"` |
| Valeur absolue, min, max | `abs(-3)`, `min(4, 2)`, `max(4, 2)` | `3`, `2`, `4` |

```python
import math

print(7 / 2, 7 // 2, 7 % 2, 2 ** 10)
print(round(math.pi, 2), math.sqrt(16))
print(0.1 + 0.2, math.isclose(0.1 + 0.2, 0.3))
print(f"{1234567.891:,.2f}")
```

```sortie
3.5 3 1 1024
3.14 4.0
0.30000000000000004 True
1,234,567.89
```

Détails : [les nombres](nombres.md).

## Conditions et boucles

| Besoin | Syntaxe |
| --- | --- |
| Condition | `if … elif … else` |
| Expression conditionnelle | `"oui" if x > 0 else "non"` |
| Plusieurs cas | `match x:` puis `case 1:` … `case _:` |
| Boucle sur une suite | `for x in liste:` |
| Compter | `for i in range(1, 6):` |
| Indice et valeur | `for i, x in enumerate(liste, start=1):` |
| Deux listes ensemble | `for a, b in zip(l1, l2):` |
| Tant que | `while condition:` |
| Contrôle | `break`, `continue`, `pass` |

```python
for n in range(1, 6):
    if n == 2:
        continue
    print(n, "pair" if n % 2 == 0 else "impair")

match "ajouter":
    case "ajouter" | "add":
        print("ajout")
    case _:
        print("inconnu")
```

```sortie
1 impair
3 impair
4 pair
5 impair
ajout
```

Détails : [conditions et boucles](conditions-boucles.md).

## Texte

| Besoin | Méthode |
| --- | --- |
| Majuscules, minuscules | `s.upper()`, `s.lower()`, `s.title()` |
| Retirer les espaces | `s.strip()` |
| Découper, recoller | `s.split(",")`, `", ".join(liste)` |
| Remplacer | `s.replace("a", "b")` |
| Chercher | `"ab" in s`, `s.find("ab")`, `s.startswith("x")` |
| Longueur, morceau | `len(s)`, `s[0]`, `s[-1]`, `s[1:4]`, `s[::-1]` |
| Aligner | `f"{s:<10}"`, `f"{s:>10}"`, `f"{s:^10}"` |
| Nombre formaté | `f"{x:.2f}"`, `f"{n:03d}"` |

```python
s = "  Bonjour le monde  "
mots = s.strip().split()
print(mots)
print("-".join(mots).upper())
print(f"[{'Python':^10}] [{3.14159:.2f}] [{7:03d}]")
print("radar"[::-1], len("été"))
```

```sortie
['Bonjour', 'le', 'monde']
BONJOUR-LE-MONDE
[  Python  ] [3.14] [007]
radar 3
```

Détails : [le texte](texte.md).

## Listes et tuples

| Besoin | Syntaxe |
| --- | --- |
| Créer | `l = [1, 2, 3]`, `t = (1, 2)` (tuple, non modifiable) |
| Ajouter | `l.append(x)`, `l.extend([4, 5])`, `l.insert(0, x)` |
| Retirer | `l.pop()`, `l.pop(0)`, `l.remove(x)` |
| Trier | `sorted(l)` (nouvelle liste), `l.sort()` (sur place) |
| Trier selon une clé | `sorted(l, key=len, reverse=True)` |
| Tranche | `l[1:3]`, `l[:2]`, `l[-2:]` |
| Agréger | `sum(l)`, `min(l)`, `max(l)`, `len(l)` |
| Tester | `x in l`, `any(...)`, `all(...)` |
| Déballer | `a, b = (1, 2)`, `premier, *reste = l` |

```python
fruits = ["poire", "kiwi", "pomme"]
fruits.append("abricot")
print(sorted(fruits))
print(sorted(fruits, key=len))
premier, *reste = fruits
print(premier, reste)
print(any(len(f) > 6 for f in fruits), fruits[-2:])
```

```sortie
['abricot', 'kiwi', 'poire', 'pomme']
['kiwi', 'poire', 'pomme', 'abricot']
poire ['kiwi', 'pomme', 'abricot']
True ['pomme', 'abricot']
```

Détails : [les collections](collections.md).

## Dictionnaires

| Besoin | Syntaxe |
| --- | --- |
| Créer | `d = {"a": 1, "b": 2}` |
| Lire | `d["a"]` (erreur si absente), `d.get("z", 0)` |
| Écrire, supprimer | `d["c"] = 3`, `del d["a"]`, `d.pop("a")` |
| Parcourir | `for cle, valeur in d.items():` |
| Clés, valeurs | `d.keys()`, `d.values()` |
| Tester une clé | `"a" in d` |
| Fusionner | `d1 \| d2` |
| Compter | `collections.Counter(liste)` |
| Regrouper | `collections.defaultdict(list)` |

```python
from collections import Counter, defaultdict

stock = {"pomme": 3, "kiwi": 0}
stock["poire"] = 5
print(stock.get("banane", 0), "kiwi" in stock)
for fruit, quantite in stock.items():
    print(fruit, quantite)

print(Counter("abracadabra").most_common(2))
par_initiale = defaultdict(list)
for mot in ["ane", "arbre", "bol"]:
    par_initiale[mot[0]].append(mot)
print(dict(par_initiale))
```

```sortie
0 True
pomme 3
kiwi 0
poire 5
[('a', 5), ('b', 2)]
{'a': ['ane', 'arbre'], 'b': ['bol']}
```

Détails : [les dictionnaires](dictionnaires.md).

## Ensembles

| Opération | Syntaxe |
| --- | --- |
| Créer | `{1, 2, 3}`, `set()` (vide), `set(liste)` |
| Ajouter, retirer | `s.add(x)`, `s.discard(x)` |
| Union, intersection | `a \| b`, `a & b` |
| Différence | `a - b`, `a ^ b` (dans un seul des deux) |

```python
a = {1, 2, 3, 4}
b = {3, 4, 5}
print(sorted(a | b), sorted(a & b), sorted(a - b))
print(sorted(set([3, 1, 3, 2, 1])))
```

```sortie
[1, 2, 3, 4, 5] [3, 4] [1, 2]
[1, 2, 3]
```

Un ensemble n'a pas d'ordre : `sorted` donne un affichage stable.

## Compréhensions et générateurs

| Forme | Exemple |
| --- | --- |
| Liste | `[x * 2 for x in l if x > 0]` |
| Dictionnaire | `{x: x * x for x in l}` |
| Ensemble | `{len(m) for m in mots}` |
| Générateur (paresseux) | `sum(x * x for x in l)` |
| Fonction génératrice | `yield` dans une fonction |

```python
nombres = [3, -1, 4, 0, 5]
print([n * 2 for n in nombres if n > 0])
print({n: n * n for n in nombres if n > 3})
print(sum(n * n for n in nombres))


def compte_a_rebours(n):
    while n > 0:
        yield n
        n -= 1


print(list(compte_a_rebours(3)))
```

```sortie
[6, 8, 10]
{4: 16, 5: 25}
51
[3, 2, 1]
```

Détails : [les compréhensions](comprehensions.md), [les itérateurs](iterateurs.md).

## Fonctions

| Élément | Syntaxe |
| --- | --- |
| Définir | `def nom(a, b=0):` puis `return …` |
| Arguments nommés | `nom(1, b=2)` |
| Nombre variable | `def f(*args, **kwargs):` |
| Fonction anonyme | `lambda x: x * 2` |
| Annotations | `def f(x: int) -> str:` |
| Docstring | `"""Ce que fait la fonction."""` |
| Décorateur | `@functools.cache` au-dessus de `def` |

```python
from functools import cache


def saluer(nom: str, formule: str = "Bonjour") -> str:
    """Renvoie une phrase de salutation."""
    return f"{formule} {nom} !"


def total(*nombres):
    return sum(nombres)


@cache
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)


print(saluer("Alice"), saluer("Bob", formule="Salut"))
print(total(1, 2, 3), fib(50))
print(sorted(["bb", "a", "ccc"], key=lambda m: -len(m)))
```

```sortie
Bonjour Alice ! Salut Bob !
6 12586269025
['ccc', 'bb', 'a']
```

Détails : [les fonctions](fonctions.md), [les décorateurs](decorateurs.md),
[le typage](typage.md).

## Classes et héritage

| Élément | Syntaxe |
| --- | --- |
| Classe | `class Chien:` |
| Constructeur | `def __init__(self, nom):` |
| Attribut | `self.nom = nom` |
| Texte lisible | `def __str__(self):` |
| Héritage | `class Chien(Animal):` |
| Appeler le parent | `super().__init__(nom)` |
| Propriété calculée | `@property` |

```python
class Animal:
    def __init__(self, nom):
        self.nom = nom

    def parler(self):
        return "..."

    def __str__(self):
        return f"{self.nom} dit {self.parler()}"


class Chien(Animal):
    def parler(self):
        return "Wouf"


for animal in [Animal("Bidule"), Chien("Rex")]:
    print(animal)
print(isinstance(Chien("Rex"), Animal))
```

```sortie
Bidule dit ...
Rex dit Wouf
True
```

Détails : [les classes](types.md), [l'héritage](heritage.md).

## Dataclasses

`@dataclass` écrit `__init__`, `__repr__` et `__eq__` à votre place.

```python
from dataclasses import dataclass, field


@dataclass
class Point:
    x: float
    y: float = 0.0


@dataclass(frozen=True)
class Couleur:
    nom: str


@dataclass
class Panier:
    articles: list[str] = field(default_factory=list)


p = Point(1, 2)
print(p, p == Point(1, 2), Point(3))
print(Couleur("rouge"))
panier = Panier()
panier.articles.append("pain")
print(panier)
```

```sortie
Point(x=1, y=2) True Point(x=3, y=0.0)
Couleur(nom='rouge')
Panier(articles=['pain'])
```

## Exceptions

| Élément | Syntaxe |
| --- | --- |
| Attraper | `try:` … `except ValueError as e:` |
| Plusieurs types | `except (KeyError, IndexError):` |
| Si tout va bien | `else:` |
| Toujours | `finally:` |
| Lever | `raise ValueError("message")` |
| Sa propre erreur | `class SoldeInsuffisant(Exception): pass` |

```python
class SoldeInsuffisant(Exception):
    pass


def retirer(solde, montant):
    if montant > solde:
        raise SoldeInsuffisant(f"il manque {montant - solde} €")
    return solde - montant


for texte in ["30", "abc", "500"]:
    try:
        print("reste", retirer(100, int(texte)))
    except ValueError:
        print("pas un nombre :", texte)
    except SoldeInsuffisant as erreur:
        print("refusé :", erreur)
    finally:
        print("--")
```

```sortie
reste 70
--
pas un nombre : abc
--
refusé : il manque 400 €
--
```

Détails : [les erreurs](erreurs.md), [déboguer](debogage.md).

## Fichiers

| Besoin | Syntaxe |
| --- | --- |
| Écrire | `with open("f.txt", "w", encoding="utf-8") as f: f.write(texte)` |
| Ajouter à la fin | mode `"a"` |
| Lire tout | `Path("f.txt").read_text(encoding="utf-8")` |
| Lire ligne à ligne | `for ligne in f:` |
| Tester, lister | `Path("f.txt").exists()`, `Path(".").glob("*.txt")` |
| Chemins | `Path("dossier") / "f.txt"`, `p.name`, `p.suffix` |

```python
from pathlib import Path

chemin = Path("notes.txt")
with open(chemin, "w", encoding="utf-8") as f:
    f.write("Alice 15\nBob 12\n")

with open(chemin, encoding="utf-8") as f:
    for ligne in f:
        nom, note = ligne.split()
        print(nom, int(note) + 1)

print(chemin.exists(), chemin.suffix, chemin.read_text(encoding="utf-8").count("\n"))
```

```sortie
Alice 16
Bob 13
True .txt 2
```

Détails : [les fichiers](fichiers.md).

## JSON et CSV

| Besoin | Fonction |
| --- | --- |
| Objet Python vers texte JSON | `json.dumps(obj, indent=2, ensure_ascii=False)` |
| Texte JSON vers objet Python | `json.loads(texte)` |
| Avec un fichier | `json.dump(obj, f)`, `json.load(f)` |
| Lire un CSV | `csv.reader(f)`, `csv.DictReader(f)` |

```python
import csv
import io
import json

donnees = {"nom": "Léa", "notes": [15, 12], "admise": True}
texte = json.dumps(donnees, ensure_ascii=False)
print(texte)
print(json.loads(texte)["notes"])

for ligne in csv.DictReader(io.StringIO("nom,note\nAlice,15\nBob,12\n")):
    print(ligne["nom"], int(ligne["note"]))
```

```sortie
{"nom": "Léa", "notes": [15, 12], "admise": true}
[15, 12]
Alice 15
Bob 12
```

Détails : [les formats de données](formats.md).

## Expressions régulières

| Motif | Signification |
| --- | --- |
| `\d`, `\w`, `\s` | un chiffre, une lettre ou un chiffre, un espace |
| `.` | n'importe quel caractère |
| `+`, `*`, `?`, `{2,4}` | 1 ou plus, 0 ou plus, 0 ou 1, entre 2 et 4 |
| `^`, `$` | début, fin |
| `[a-z]`, `[^0-9]` | un caractère de la liste, hors de la liste |
| `( )`, `(?P<nom> )` | un groupe, un groupe nommé |

```python
import re

texte = "Commande 42 le 2026-03-15, commande 7 le 2026-04-01"
print(re.findall(r"\d+", texte))

m = re.search(r"(?P<annee>\d{4})-(?P<mois>\d{2})", texte)
print(m.group("annee"), m.group("mois"))

print(re.sub(r"\s+", " ", "trop   d'espaces   ici"))
print(bool(re.fullmatch(r"[\w.]+@[\w.]+\.\w+", "alice@example.com")))
```

```sortie
['42', '2026', '03', '15', '7', '2026', '04', '01']
2026 03
trop d'espaces ici
True
```

Détails : [les expressions régulières](regex.md).

## Dates

| Besoin | Syntaxe |
| --- | --- |
| Une date, une date et heure | `date(2026, 3, 15)`, `datetime(2026, 3, 15, 14, 30)` |
| Aujourd'hui, maintenant | `date.today()`, `datetime.now()` |
| Durée | `timedelta(days=7, hours=2)` |
| Formater | `d.strftime("%d/%m/%Y")` |
| Lire un texte | `datetime.strptime("15/03/2026", "%d/%m/%Y")`, `date.fromisoformat("2026-03-15")` |

```python
from datetime import date, datetime, timedelta

rentree = date(2026, 9, 1)
vacances = date(2026, 10, 17)
print((vacances - rentree).days, "jours")
print(rentree + timedelta(weeks=2))
print(rentree.strftime("%d/%m/%Y"), rentree.isoweekday())

rdv = datetime.strptime("15/03/2026 14:30", "%d/%m/%Y %H:%M")
print(rdv.isoformat())
```

```sortie
46 jours
2026-09-15
01/09/2026 2
2026-03-15T14:30:00
```

Détails : [les dates](dates.md).

## Ligne de commande avec argparse

```python
import argparse

parser = argparse.ArgumentParser(description="Calcule une moyenne.")
parser.add_argument("notes", type=float, nargs="+", help="les notes")
parser.add_argument("--arrondi", type=int, default=1)
parser.add_argument("-v", "--verbeux", action="store_true")

args = parser.parse_args(["12", "15", "17", "--arrondi", "2", "-v"])
moyenne = sum(args.notes) / len(args.notes)
if args.verbeux:
    print("Notes :", args.notes)
print(round(moyenne, args.arrondi))
```

```sortie
Notes : [12.0, 15.0, 17.0]
14.67
```

Dans un vrai programme, écrivez `parser.parse_args()` sans liste : il lit `sys.argv`.
`--help` est créé tout seul. Détails : [la ligne de commande](ligne-de-commande.md).

## Concurrence et asyncio

| Élément | Syntaxe |
| --- | --- |
| Fonction asynchrone | `async def f():` |
| Attendre | `await autre()` |
| Lancer le programme | `asyncio.run(main())` |
| En parallèle | `await asyncio.gather(f(), g())` |
| Pause sans bloquer | `await asyncio.sleep(1)` |
| Threads | `concurrent.futures.ThreadPoolExecutor` |

```python
import asyncio


async def telecharger(nom, duree):
    await asyncio.sleep(duree)
    return f"{nom} terminé"


async def main():
    resultats = await asyncio.gather(
        telecharger("image", 0.02),
        telecharger("texte", 0.01),
    )
    print(resultats)


asyncio.run(main())
```

```sortie
['image terminé', 'texte terminé']
```

`gather` rend les résultats dans l'ordre des appels, même si `texte` finit avant.
Détails : [la concurrence](concurrence.md), [async](async.md).

## Tests avec unittest

| Élément | Syntaxe |
| --- | --- |
| Classe de test | `class TestX(unittest.TestCase):` |
| Un test | méthode `def test_…(self):` |
| Égalité | `self.assertEqual(obtenu, attendu)` |
| Vrai, dans, flottants | `assertTrue`, `assertIn`, `assertAlmostEqual` |
| Erreur attendue | `with self.assertRaises(ValueError):` |
| Avant chaque test | `def setUp(self):` |
| Lancer | `unittest.main()` ou `python3 -m unittest` |

```python
import unittest


def double(x):
    return x * 2


class TestDouble(unittest.TestCase):
    def test_double(self):
        self.assertEqual(double(4), 8)

    def test_erreur(self):
        with self.assertRaises(ZeroDivisionError):
            1 / 0


programme = unittest.main(argv=["x"], exit=False)
print(programme.result.testsRun, programme.result.wasSuccessful())
```

```sortie
2 True
```

Détails : [tester son code](tests.md).

## Projet et outils

| Commande | Rôle |
| --- | --- |
| `python3 main.py` | lancer un programme |
| `python3` | console interactive (REPL) |
| `python3 -m venv .venv` | créer un environnement virtuel |
| `source .venv/bin/activate` | l'activer (Linux, macOS) |
| `python -m pip install nom` | installer un paquet |
| `python -m pip install -r requirements.txt` | installer la liste d'un projet |
| `python -m pip install -e .` | installer son propre projet |
| `python3 -m pdb main.py` | déboguer |
| `pytest`, `ruff check`, `black .` | tester, vérifier, formater |
| `uv run main.py` | tout en un avec uv |

Détails : [organiser un projet](projet.md), [bien écrire du Python](style.md). Pour
pratiquer : [les projets pas à pas](projets.md) et [les exercices corrigés](exercices.md).
Comparez avec [l'aide-mémoire Ruby](../ruby/aide-memoire.md) et
[l'aide-mémoire Rust](../rust/aide-memoire.md).
