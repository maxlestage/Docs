# Projets pas à pas

Cette page vous guide dans trois petits projets Python complets, construits étape par étape : un jeu, une liste de tâches enregistrée dans un fichier JSON et un bulletin de notes calculé à partir d'un texte CSV.

> **Objectifs :**
>
> - découper un programme en petites étapes qui marchent chacune ;
> - réutiliser ce que vous avez appris : conditions, boucles, fonctions, listes,
>   dictionnaires ;
> - enregistrer des données dans un fichier JSON et les relire ;
> - lire des données CSV, calculer et afficher un rapport bien aligné.

## Comment suivre ces projets

Chaque projet avance par petites étapes. À chaque étape :

1. lisez le code et l'explication ;
2. tapez le code dans un fichier (par exemple `jeu.py`) et lancez-le avec
   `python3 jeu.py` ;
3. comparez avec la sortie affichée ;
4. modifiez une valeur pour voir ce qui change.

> **Astuce :** ne copiez pas tout d'un coup. Un programme se construit morceau par
> morceau, en le lançant souvent. C'est ainsi que travaillent les programmeurs.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des nombres ;
le programme répond « trop petit » ou « trop grand », jusqu'à ce que le joueur trouve.

### Étape 1 : comparer un essai

Commencez par le cœur du jeu : comparer **un** essai au nombre secret (voir
[conditions et boucles](conditions-boucles.md)).

```python
secret = 42
essai = 30

if essai < secret:
    print("Trop petit !")
elif essai > secret:
    print("Trop grand !")
else:
    print("Gagné !")
```

```sortie
Trop petit !
```

Changez `essai` en `50`, puis en `42`, et relancez.

### Étape 2 : une fonction pour l'indice

Rangez cette comparaison dans une [fonction](fonctions.md). Elle **renvoie** le texte
au lieu de l'afficher : elle sera plus facile à réutiliser et à tester.

```python
def indice(secret, essai):
    if essai < secret:
        return "Trop petit !"
    if essai > secret:
        return "Trop grand !"
    return "Gagné !"


print(indice(42, 30))
print(indice(42, 50))
print(indice(42, 42))
```

```sortie
Trop petit !
Trop grand !
Gagné !
```

### Étape 3 : plusieurs essais et un compteur

Le joueur fait plusieurs essais. Pour l'instant, ses essais sont dans une **liste
fixe** : le programme donne ainsi toujours le même résultat, et vous pouvez le vérifier
facilement. `enumerate(..., start=1)` donne le numéro de l'essai ; `break` arrête la
boucle quand c'est gagné.

```python
def indice(secret, essai):
    if essai < secret:
        return "Trop petit !"
    if essai > secret:
        return "Trop grand !"
    return "Gagné !"


secret = 42
essais = [50, 25, 40, 42, 10]

for numero, essai in enumerate(essais, start=1):
    print(f"Essai {numero} : {essai} -> {indice(secret, essai)}")
    if essai == secret:
        print(f"Bravo, trouvé en {numero} essais !")
        break
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo, trouvé en 4 essais !
```

Le dernier essai (`10`) n'est jamais lu : `break` a arrêté la boucle avant.

### Étape 4 : limiter le nombre d'essais

Pour corser le jeu, le joueur n'a droit qu'à 5 essais. Rangez la partie dans une
fonction `jouer` qui renvoie `True` si le joueur gagne. `essais[:maximum]` ne garde que
les premiers essais (voir [les collections](collections.md)).

```python
def indice(secret, essai):
    if essai < secret:
        return "Trop petit !"
    if essai > secret:
        return "Trop grand !"
    return "Gagné !"


def jouer(secret, essais, maximum=5):
    for numero, essai in enumerate(essais[:maximum], start=1):
        print(f"Essai {numero} : {essai} -> {indice(secret, essai)}")
        if essai == secret:
            return True
    return False


if jouer(42, [50, 25, 40, 42]):
    print("Bravo !")

print("---")

if not jouer(7, [50, 25, 12, 10, 8, 7]):
    print("Perdu ! Le nombre était 7.")
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo !
---
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop grand !
Essai 3 : 12 -> Trop grand !
Essai 4 : 10 -> Trop grand !
Essai 5 : 8 -> Trop grand !
Perdu ! Le nombre était 7.
```

> **Note :** `return True` quitte la fonction tout de suite, en pleine boucle. Si la
> boucle se termine sans victoire, la fonction arrive à la dernière ligne et renvoie
> `False`.

### Étape 5 : la vraie version, au clavier

Il reste deux changements pour un vrai jeu :

- `random.randint(1, 100)` tire un nombre au hasard entre 1 et 100 ;
- `int(input("Essai : "))` lit l'essai tapé par le joueur et le transforme en nombre.

Ce code attend le clavier : il n'est pas exécuté ici. Lancez-le chez vous.

```python norun
# jeu.py
import random


def indice(secret, essai):
    if essai < secret:
        return "Trop petit !"
    if essai > secret:
        return "Trop grand !"
    return "Gagné !"


secret = random.randint(1, 100)
print("J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.")

gagne = False
for numero in range(1, 6):
    essai = int(input(f"Essai {numero} : "))
    print(indice(secret, essai))
    if essai == secret:
        gagne = True
        break

print("Bravo !" if gagne else f"Perdu ! Le nombre était {secret}.")
```

Une partie peut ressembler à ceci :

```text
J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.
Essai 1 : 50
Trop grand !
Essai 2 : 25
Trop petit !
Essai 3 : 37
Gagné !
Bravo !
```

### Pour aller plus loin

- Demandez au joueur s'il veut rejouer (`input("Rejouer ? ") == "o"`) et entourez la
  partie d'une boucle `while True:`.
- Si le joueur tape `abc`, `int` lève une `ValueError` : attrapez-la avec `try` et
  redemandez (voir [les erreurs](erreurs.md)).
- Retenez le meilleur score dans un fichier.
- Écrivez des tests pour `indice` avec `unittest` (voir [tester son code](tests.md)).

## Projet 2 : Une liste de tâches

Une petite application en ligne de commande : ajouter une tâche, afficher la liste,
marquer une tâche comme faite. Les tâches sont enregistrées dans un fichier JSON :
elles sont toujours là au prochain lancement.

### Étape 1 : représenter les tâches

Une tâche est un [dictionnaire](dictionnaires.md) avec deux clés : son `"titre"` et
`"faite"` (vrai ou faux). La liste des tâches est une liste de ces dictionnaires.

```python
taches = [
    {"titre": "Acheter du pain", "faite": False},
    {"titre": "Appeler Léa", "faite": True},
]

for numero, tache in enumerate(taches, start=1):
    case = "[x]" if tache["faite"] else "[ ]"
    print(f"{numero}. {case} {tache['titre']}")
```

```sortie
1. [ ] Acheter du pain
2. [x] Appeler Léa
```

> **Note :** dans une f-string entre guillemets doubles, écrivez la clé avec des
> guillemets simples : `{tache['titre']}`.

### Étape 2 : ajouter, afficher, terminer

Chaque action devient une fonction. `afficher` gère aussi la liste vide.

```python
def ajouter(taches, titre):
    taches.append({"titre": titre, "faite": False})


def terminer(taches, numero):
    if 1 <= numero <= len(taches):
        taches[numero - 1]["faite"] = True
    else:
        print(f"Pas de tâche numéro {numero}.")


def afficher(taches):
    if not taches:
        print("Aucune tâche.")
        return
    for numero, tache in enumerate(taches, start=1):
        case = "[x]" if tache["faite"] else "[ ]"
        print(f"{numero}. {case} {tache['titre']}")


taches = []
afficher(taches)
ajouter(taches, "Acheter du pain")
ajouter(taches, "Réviser Python")
terminer(taches, 1)
terminer(taches, 5)
afficher(taches)
```

```sortie
Aucune tâche.
Pas de tâche numéro 5.
1. [x] Acheter du pain
2. [ ] Réviser Python
```

> **Attention :** l'utilisateur compte à partir de 1, Python à partir de 0. D'où
> `taches[numero - 1]`.

### Étape 3 : enregistrer dans un fichier JSON

Pour garder les tâches, écrivez-les dans `taches.json` avec le module `json` (voir
[les formats de données](formats.md)). Au lancement suivant, relisez le fichier. S'il
n'existe pas encore, partez d'une liste vide. `pathlib.Path` facilite le travail avec
les [fichiers](fichiers.md).

```python
import json
from pathlib import Path

FICHIER = Path("taches.json")


def charger():
    if not FICHIER.exists():
        return []
    return json.loads(FICHIER.read_text(encoding="utf-8"))


def sauvegarder(taches):
    texte = json.dumps(taches, indent=2, ensure_ascii=False)
    FICHIER.write_text(texte, encoding="utf-8")


# Premier lancement : le fichier n'existe pas encore.
taches = charger()
print(taches)
taches.append({"titre": "Acheter du pain", "faite": False})
sauvegarder(taches)

# Lancement suivant : la tâche est relue depuis le fichier.
print(charger())
print(FICHIER.read_text(encoding="utf-8"))
```

```sortie
[]
[{'titre': 'Acheter du pain', 'faite': False}]
[
  {
    "titre": "Acheter du pain",
    "faite": false
  }
]
```

- `indent=2` écrit un JSON aéré, facile à lire et à corriger à la main.
- `ensure_ascii=False` garde les accents tels quels (`"Léa"` au lieu de `"Léa"`).
- En JSON, `False` s'écrit `false` : le module `json` traduit dans les deux sens.

### Étape 4 : le programme complet

Réunissez tout. Une fonction `executer` reçoit une commande (`"ajouter"`, `"liste"`,
`"faite"`) et son argument. Chaque commande charge les tâches, agit, puis enregistre.
Ici, plusieurs commandes sont lancées à la suite, comme si l'utilisateur lançait le
programme plusieurs fois. `match` choisit selon la commande.

```python
import json
from pathlib import Path

FICHIER = Path("taches.json")


def charger():
    if not FICHIER.exists():
        return []
    return json.loads(FICHIER.read_text(encoding="utf-8"))


def sauvegarder(taches):
    texte = json.dumps(taches, indent=2, ensure_ascii=False)
    FICHIER.write_text(texte, encoding="utf-8")


def afficher(taches):
    if not taches:
        print("Aucune tâche.")
        return
    for numero, tache in enumerate(taches, start=1):
        case = "[x]" if tache["faite"] else "[ ]"
        print(f"{numero}. {case} {tache['titre']}")


def executer(commande, argument=None):
    taches = charger()

    match commande:
        case "ajouter":
            taches.append({"titre": argument, "faite": False})
            print(f"Ajouté : {argument}")
        case "faite":
            numero = int(argument)
            if 1 <= numero <= len(taches):
                tache = taches[numero - 1]
                tache["faite"] = True
                print(f"Bravo : {tache['titre']}")
            else:
                print(f"Pas de tâche numéro {numero}.")
        case "liste":
            afficher(taches)
        case _:
            print("Commandes : ajouter TITRE, liste, faite NUMÉRO")

    sauvegarder(taches)


executer("liste")
executer("ajouter", "Acheter du pain")
executer("ajouter", "Réviser Python")
executer("faite", "2")
executer("liste")
executer("aide")
```

```sortie
Aucune tâche.
Ajouté : Acheter du pain
Ajouté : Réviser Python
Bravo : Réviser Python
1. [ ] Acheter du pain
2. [x] Réviser Python
Commandes : ajouter TITRE, liste, faite NUMÉRO
```

### Étape 5 : depuis le terminal avec sys.argv

Pour l'utiliser vraiment, remplacez les appels de test de la fin par quelques lignes.
`sys.argv` contient le nom du programme puis les mots tapés après lui (voir
[la ligne de commande](ligne-de-commande.md)).

```python norun
# À la fin de taches.py, à la place des appels executer(...)
import sys

if __name__ == "__main__":
    commande = sys.argv[1] if len(sys.argv) > 1 else "aide"
    argument = sys.argv[2] if len(sys.argv) > 2 else None
    executer(commande, argument)
```

```sh
python3 taches.py ajouter "Acheter du pain"
python3 taches.py ajouter "Réviser Python"
python3 taches.py faite 1
python3 taches.py liste
```

```text
Ajouté : Acheter du pain
Ajouté : Réviser Python
Bravo : Acheter du pain
1. [x] Acheter du pain
2. [ ] Réviser Python
```

> **Astuce :** les guillemets autour de `"Acheter du pain"` en font **un seul**
> argument. Sans eux, `sys.argv[2]` vaudrait seulement `"Acheter"`.

### Pour aller plus loin

- Ajoutez une commande `supprimer NUMÉRO` avec `taches.pop(numero - 1)`.
- Ajoutez une commande `menage` qui garde seulement les tâches non faites, avec une
  compréhension : `[t for t in taches if not t["faite"]]`.
- Affichez à la fin de la liste : « 1 tâche sur 2 terminée ».
- Remplacez `sys.argv` par `argparse`, qui gère l'aide `--help` tout seul.
- Transformez les tâches en `dataclass` (voir [les classes](types.md)).

## Projet 3 : Le bulletin de notes

Le programme lit les notes d'une classe, écrites en texte au format CSV. Il calcule la
moyenne de chaque élève et de chaque matière, trouve le meilleur élève, puis affiche un
bulletin bien aligné.

### Étape 1 : lire les données

Les notes sont dans un texte CSV : une ligne de titres, puis une ligne par élève. Un
texte sur plusieurs lignes s'écrit entre triples guillemets. Le module `csv` découpe
chaque ligne ; `io.StringIO` lui fait lire un texte comme s'il lisait un fichier.

```python
import csv
import io

DONNEES = """nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13
"""

lecteur = csv.reader(io.StringIO(DONNEES))
titres = next(lecteur)
print(titres)

eleves = []
for nom, *notes in lecteur:
    eleves.append({"nom": nom, "notes": [int(n) for n in notes]})
print(eleves[0])
```

```sortie
['nom', 'maths', 'francais', 'histoire']
{'nom': 'Alice', 'notes': [15, 12, 17]}
```

- `next(lecteur)` lit la première ligne : les titres.
- `nom, *notes` range le premier morceau dans `nom` et **tous les autres** dans la
  liste `notes`.
- `[int(n) for n in notes]` transforme les textes `"15"` en nombres `15`.

> **Note :** dans un vrai programme, les données viendraient d'un fichier :
> `with open("notes.csv", encoding="utf-8", newline="") as f:` puis `csv.reader(f)`.

### Étape 2 : la moyenne de chaque élève

Une moyenne, c'est la somme divisée par le nombre de notes. En Python, `/` donne
toujours un flottant ; `round(..., 1)` garde un chiffre après la virgule.

```python
def moyenne(notes):
    return round(sum(notes) / len(notes), 1)


eleves = [
    {"nom": "Alice", "notes": [15, 12, 17]},
    {"nom": "Bob", "notes": [9, 14, 11]},
    {"nom": "Chloé", "notes": [18, 16, 13]},
]

for eleve in eleves:
    print(f"{eleve['nom']} : {moyenne(eleve['notes'])}")
```

```sortie
Alice : 14.7
Bob : 11.3
Chloé : 15.7
```

### Étape 3 : le meilleur élève et les moyennes par matière

`max(..., key=...)` trouve l'élève qui a la plus grande moyenne : `key` dit **sur quoi**
comparer. Pour une matière, prenez la note numéro `i` de chaque élève.

```python
def moyenne(notes):
    return round(sum(notes) / len(notes), 1)


matieres = ["maths", "francais", "histoire"]
eleves = [
    {"nom": "Alice", "notes": [15, 12, 17]},
    {"nom": "Bob", "notes": [9, 14, 11]},
    {"nom": "Chloé", "notes": [18, 16, 13]},
]

meilleur = max(eleves, key=lambda eleve: moyenne(eleve["notes"]))
print(f"Meilleure moyenne : {meilleur['nom']}")

for i, matiere in enumerate(matieres):
    notes = [eleve["notes"][i] for eleve in eleves]
    print(f"{matiere} : {moyenne(notes)}")
```

```sortie
Meilleure moyenne : Chloé
maths : 14.0
francais : 14.0
histoire : 13.7
```

`lambda eleve: moyenne(eleve["notes"])` est une petite fonction sans nom, écrite sur
place (voir [les fonctions](fonctions.md)).

### Étape 4 : un bulletin bien aligné

Pour un tableau lisible, chaque colonne doit avoir la même largeur. Dans une f-string,
`{nom:<8}` complète le texte avec des espaces jusqu'à 8 caractères, et `{m:5.1f}` écrit
un nombre sur 5 caractères avec un chiffre après la virgule (voir [le texte](texte.md)).

```python
import csv
import io

DONNEES = """nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13
"""


def moyenne(notes):
    return round(sum(notes) / len(notes), 1)


def mention(note):
    if note >= 14:
        return "Bien"
    if note >= 10:
        return "Passable"
    return "Insuffisant"


lecteur = csv.reader(io.StringIO(DONNEES))
next(lecteur)
eleves = [{"nom": nom, "notes": [int(n) for n in notes]} for nom, *notes in lecteur]

print("BULLETIN DE LA CLASSE")
print("=" * 30)
classement = sorted(eleves, key=lambda eleve: moyenne(eleve["notes"]), reverse=True)
for eleve in classement:
    m = moyenne(eleve["notes"])
    print(f"{eleve['nom']:<8}{m:5.1f}   {mention(m)}")
print("=" * 30)

toutes = [note for eleve in eleves for note in eleve["notes"]]
print(f"Moyenne de la classe : {moyenne(toutes)}")
print(f"Félicitations à {classement[0]['nom']} !")
```

```sortie
BULLETIN DE LA CLASSE
==============================
Chloé    15.7   Bien
Alice    14.7   Bien
Bob      11.3   Passable
==============================
Moyenne de la classe : 13.9
Félicitations à Chloé !
```

- `sorted(..., reverse=True)` trie de la plus grande moyenne à la plus petite.
- La compréhension à deux `for` rassemble toutes les notes de tous les élèves dans une
  seule liste.
- `"=" * 30` répète le texte `"="` 30 fois.

### Pour aller plus loin

- Lisez les notes depuis un vrai fichier `notes.csv`.
- Utilisez `csv.DictReader` : chaque ligne devient un dictionnaire
  `{"nom": "Alice", "maths": "15", …}`.
- Enregistrez le bulletin dans `bulletin.txt` avec `Path.write_text`.
- Ignorez les cases vides (`Alice,15,,17`) avec `if n` dans la compréhension.
- Affichez pour chaque matière la meilleure note et le nom de l'élève qui l'a obtenue.

## Erreurs fréquentes

### Oublier int() après split, input ou csv

`split`, `input` et `csv` donnent du **texte**. On ne peut pas additionner des textes
avec des nombres.

```python norun
notes = "15,12,17".split(",")
print(sum(notes))
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 2, in <module>
    print(sum(notes))
          ~~~^^^^^^^
TypeError: unsupported operand type(s) for +: 'int' and 'str'
```

Transformez d'abord en nombres : `sum(int(n) for n in notes)`. Même chose avec
`input` : `int(input("Essai : "))`.

### Lire le fichier JSON avant qu'il existe

Au tout premier lancement, `taches.json` n'existe pas encore.

```python norun
import json

with open("taches.json", encoding="utf-8") as f:
    taches = json.load(f)
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 3, in <module>
    with open("taches.json", encoding="utf-8") as f:
         ~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
FileNotFoundError: [Errno 2] No such file or directory: 'taches.json'
```

Vérifiez d'abord avec `Path("taches.json").exists()`, ou attrapez l'erreur avec
`try` / `except FileNotFoundError` et partez d'une liste vide.

### Un numéro de tâche décalé

L'utilisateur tape `1` pour la première tâche, mais Python compte à partir de 0. Sans
`- 1`, vous terminez la mauvaise tâche, ou vous sortez de la liste.

```python norun
taches = [{"titre": "Pain", "faite": False}]
taches[1]["faite"] = True
```

```text
Traceback (most recent call last):
  File "/home/alice/main.py", line 2, in <module>
    taches[1]["faite"] = True
    ~~~~~~^^^
IndexError: list index out of range
```

Écrivez `taches[numero - 1]`, et vérifiez que `1 <= numero <= len(taches)` avant.

## Testez-vous

**1.** Dans le juste nombre, pourquoi utiliser une liste d'essais plutôt que `input`
pendant la mise au point ?

> **Réponse**
>
> Le programme donne toujours le même résultat, sans rien taper : on peut le relancer
> et le vérifier vite. On passe à `input` une fois que tout marche.

**2.** Que fait `nom, *notes = "Alice,15,12".split(",")` ?

> **Réponse**
>
> `nom` reçoit `"Alice"` et `notes` reçoit la liste de tous les autres morceaux :
> `["15", "12"]`, encore sous forme de texte.

**3.** À quoi servent `indent=2` et `ensure_ascii=False` dans `json.dumps` ?

> **Réponse**
>
> `indent=2` écrit un JSON aéré, sur plusieurs lignes. `ensure_ascii=False` garde les
> lettres accentuées telles quelles au lieu de les coder en `é`.

## En résumé

- Un projet se construit par petites étapes, en lançant le programme à chaque fois.
- Remplacer `input` et le hasard par des valeurs fixes rend un programme facile à
  vérifier.
- Des fonctions courtes (`indice`, `ajouter`, `moyenne`) rendent le code clair et
  testable.
- `json.dumps` et `json.loads` enregistrent et relisent des données ; `Path` lit et
  écrit les fichiers.
- `csv.reader` découpe un CSV ; `{x:<8}` et `{m:5.1f}` alignent un rapport en colonnes.

Suite : entraînez-vous avec [les exercices corrigés](exercices.md). Comparez avec
[les projets en Ruby](../ruby/projets.md) et [en Rust](../rust/projets.md).
