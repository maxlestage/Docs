# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, puis comment écrire, relire et ranger des fichiers avec `open` et `pathlib`.

> **Objectifs :**
>
> - vous saurez lire une ligne tapée au clavier avec `input` ;
> - vous saurez ouvrir un fichier avec `with open(...)` et choisir le bon mode ;
> - vous saurez lire un fichier en entier ou ligne par ligne ;
> - vous saurez manipuler des chemins et des dossiers avec `pathlib.Path` ;
> - vous saurez réagir quand un fichier n'existe pas.

## Lire le clavier avec input

`input` affiche un message, puis attend que l'utilisateur tape une ligne et appuie sur
Entrée. Le programme est en pause pendant ce temps, comme un serveur qui attend votre
commande. `input` renvoie le texte tapé, **sans** le retour à la ligne.

```python norun
prenom = input("Votre prénom : ")
print(f"Bonjour, {prenom} !")
```

Dans le [terminal](../debuter/glossaire.md#terminal), cela donne :

```text
Votre prénom : Alice
Bonjour, Alice !
```

### Lire un nombre

`input` renvoie toujours une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres),
même si l'utilisateur tape `30`. Pour calculer, convertissez avec `int` (entier) ou
`float` (nombre à virgule).

```python norun
age = int(input("Votre âge : "))
print(f"Dans 10 ans, vous aurez {age + 10} ans.")
```

```text
Votre âge : 30
Dans 10 ans, vous aurez 40 ans.
```

> **Attention :** si l'utilisateur tape `trente`, `int("trente")` lève une
> `ValueError`. Le chapitre sur [les erreurs](erreurs.md) montre comment l'attraper.

## Écrire un fichier

Un fichier se manipule en trois temps : on l'**ouvre**, on lit ou on écrit, puis on le
**ferme**. Le mot `with` s'occupe de la fermeture à votre place, même en cas d'erreur.
C'est comme une porte qui se referme toute seule derrière vous.

```python
with open("notes.txt", "w", encoding="utf-8") as fichier:
    fichier.write("Première ligne\n")
    fichier.write("Deuxième ligne\n")

print("Fichier écrit.")
```

```sortie
Fichier écrit.
```

Lisez cette ligne de gauche à droite :

- `open("notes.txt", "w", ...)` ouvre le fichier `notes.txt` ;
- `"w"` (*write*, « écrire ») est le **mode** : on veut écrire ;
- `encoding="utf-8"` choisit l'encodage, pour que les accents soient bien enregistrés ;
- `as fichier` donne un nom à l'objet fichier ouvert ;
- le bloc indenté est le moment où le fichier est ouvert.

> **À retenir :** `write` n'ajoute pas de retour à la ligne. Mettez `\n` vous-même à la
> fin de chaque ligne.

### Les trois modes principaux

| Mode | Nom | Si le fichier existe | S'il n'existe pas |
|------|-----|----------------------|-------------------|
| `"r"` | *read*, lire (par défaut) | on le lit | `FileNotFoundError` |
| `"w"` | *write*, écrire | il est **vidé** puis réécrit | il est créé |
| `"a"` | *append*, ajouter | on écrit à la fin | il est créé |

> **Attention :** le mode `"w"` efface tout le contenu précédent, sans prévenir. Pour
> compléter un fichier, utilisez `"a"`.

### Ajouter à la fin

```python
with open("journal.txt", "w", encoding="utf-8") as f:
    f.write("lundi : pluie\n")

with open("journal.txt", "a", encoding="utf-8") as f:
    f.write("mardi : soleil\n")

with open("journal.txt", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
lundi : pluie
mardi : soleil
```

`print(..., end="")` évite une ligne vide en trop : le texte lu finit déjà par `\n`.

## Lire un fichier

Il y a trois façons courantes de lire un fichier texte.

### Tout d'un coup avec read

`read()` renvoie tout le contenu dans une seule chaîne.

```python
with open("fruits.txt", "w", encoding="utf-8") as f:
    f.write("pomme\npoire\nkiwi\n")

with open("fruits.txt", encoding="utf-8") as f:
    contenu = f.read()

print(repr(contenu))
print(len(contenu), "caractères")
```

```sortie
'pomme\npoire\nkiwi\n'
17 caractères
```

`repr` montre la chaîne telle qu'elle est, avec ses `\n` visibles.

### Une liste de lignes avec readlines

`readlines()` renvoie une liste : une case par ligne, chaque ligne avec son `\n`.

```python
with open("fruits.txt", "w", encoding="utf-8") as f:
    f.write("pomme\npoire\nkiwi\n")

with open("fruits.txt", encoding="utf-8") as f:
    lignes = f.readlines()

print(lignes)
```

```sortie
['pomme\n', 'poire\n', 'kiwi\n']
```

### Ligne par ligne avec for

La façon la plus simple, et la plus économe en mémoire : parcourir le fichier avec
une boucle `for`. Chaque tour donne une ligne. `strip()` enlève le `\n` de la fin.

```python
with open("fruits.txt", "w", encoding="utf-8") as f:
    f.write("pomme\npoire\nkiwi\n")

with open("fruits.txt", encoding="utf-8") as f:
    for numero, ligne in enumerate(f, start=1):
        print(numero, ligne.strip())
```

```sortie
1 pomme
2 poire
3 kiwi
```

> **Astuce :** pour un gros fichier (des millions de lignes), préférez la boucle `for` :
> elle ne charge qu'une ligne à la fois en mémoire.

## L'encodage

Un fichier contient des [octets](../debuter/glossaire.md#octet). L'**encodage** est la
règle qui transforme ces octets en lettres. Aujourd'hui, la règle la plus répandue est
UTF-8 (voir [Unicode](../debuter/glossaire.md#unicode)). Sans `encoding="utf-8"`, Python
utilise la règle par défaut du système, qui n'est pas la même partout (sous Windows,
souvent une autre).

```python
with open("accents.txt", "w", encoding="utf-8") as f:
    f.write("Noël à la montagne")

with open("accents.txt", "rb") as f:
    octets = f.read()

print(len("Noël à la montagne"), "caractères")
print(len(octets), "octets")
```

```sortie
18 caractères
20 octets
```

Le mode `"rb"` (*read binary*) lit les octets bruts. `ë` et `à` prennent chacun deux
octets en UTF-8 : d'où 20 octets pour 18 caractères.

> **À retenir :** écrivez toujours `encoding="utf-8"` quand vous ouvrez un fichier texte.

## Les chemins avec pathlib

Un [chemin](../debuter/glossaire.md#chemin) indique où se trouve un fichier :
`donnees/notes.txt`. Le module `pathlib` fournit `Path`, un objet qui représente un
chemin et sait faire beaucoup de choses tout seul.

```python
from pathlib import Path

chemin = Path("donnees") / "notes.txt"
print(chemin)
print(chemin.name)
print(chemin.stem)
print(chemin.suffix)
print(chemin.parent)
```

```sortie
donnees/notes.txt
notes.txt
notes
.txt
donnees
```

L'opérateur `/` colle deux morceaux de chemin, avec le bon séparateur pour le système
(`/` sous Linux et macOS, `\` sous Windows).

### Lire et écrire en une ligne

`write_text` et `read_text` ouvrent, écrivent ou lisent, puis ferment le fichier : tout
en une seule instruction.

```python
from pathlib import Path

fichier = Path("message.txt")
fichier.write_text("Bonjour\nà tous\n", encoding="utf-8")

print(fichier.exists())
print(fichier.read_text(encoding="utf-8").splitlines())
print(Path("absent.txt").exists())
```

```sortie
True
['Bonjour', 'à tous']
False
```

`splitlines()` découpe le texte en lignes, sans les `\n`.

### Créer des dossiers et lister des fichiers

`mkdir` crée un dossier. Avec `parents=True`, il crée aussi les dossiers parents
manquants ; avec `exist_ok=True`, il ne se plaint pas si le dossier existe déjà.
`glob` cherche les fichiers qui correspondent à un motif : `*` veut dire « n'importe
quoi ».

```python
from pathlib import Path

dossier = Path("photos/2024")
dossier.mkdir(parents=True, exist_ok=True)

for nom in ["plage.jpg", "montagne.jpg", "notes.txt"]:
    (dossier / nom).write_text("...", encoding="utf-8")

images = sorted(dossier.glob("*.jpg"))
for image in images:
    print(image.name)

print(len(list(dossier.iterdir())), "fichiers en tout")
```

```sortie
montagne.jpg
plage.jpg
3 fichiers en tout
```

> **Note :** l'ordre de `glob` et `iterdir` dépend du système. Triez avec `sorted` pour
> obtenir toujours le même ordre.

## Le module os en bref

Avant `pathlib`, on utilisait le module `os`. Vous le croiserez souvent dans du code
existant. Voici les équivalents les plus courants.

| Avec `os` | Avec `pathlib` | Rôle |
|-----------|----------------|------|
| `os.path.exists(c)` | `Path(c).exists()` | le fichier existe ? |
| `os.path.join("a", "b")` | `Path("a") / "b"` | coller des chemins |
| `os.listdir(d)` | `Path(d).iterdir()` | lister un dossier |
| `os.makedirs(d, exist_ok=True)` | `Path(d).mkdir(parents=True, exist_ok=True)` | créer un dossier |
| `os.remove(c)` | `Path(c).unlink()` | supprimer un fichier |
| `os.getcwd()` | `Path.cwd()` | dossier courant |

```python
import os

os.makedirs("archives", exist_ok=True)
with open(os.path.join("archives", "a.txt"), "w", encoding="utf-8") as f:
    f.write("x")

print(os.listdir("archives"))
os.remove(os.path.join("archives", "a.txt"))
print(os.listdir("archives"))
```

```sortie
['a.txt']
[]
```

> **Astuce :** pour du code neuf, préférez `pathlib` : il est plus lisible.

## Quand le fichier n'existe pas

Ouvrir en lecture un fichier absent lève une `FileNotFoundError`. Vous pouvez
l'attraper avec `try` et `except`, comme toute [exception](../debuter/glossaire.md#exception).

```python
def lire_config(nom):
    try:
        with open(nom, encoding="utf-8") as f:
            return f.read()
    except FileNotFoundError:
        print(f"{nom} introuvable, on utilise les réglages par défaut.")
        return "langue=fr"

print(lire_config("config.txt"))
```

```sortie
config.txt introuvable, on utilise les réglages par défaut.
langue=fr
```

Vous pouvez aussi vérifier avant, avec `Path(nom).exists()`. Mais `try` reste plus sûr :
le fichier peut disparaître entre la vérification et l'ouverture.

## Erreurs fréquentes

### Ouvrir un fichier qui n'existe pas

```python norun
f = open("notes.txt")
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    f = open("notes.txt")
FileNotFoundError: [Errno 2] No such file or directory: 'notes.txt'
```

Le fichier n'est pas dans le dossier courant (celui d'où vous lancez le programme).
Vérifiez le nom et le chemin, ou attrapez `FileNotFoundError`.

### Écrire autre chose que du texte

```python norun
with open("nombres.txt", "w", encoding="utf-8") as f:
    f.write(42)
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    f.write(42)
    ~~~~~~~^^^^
TypeError: write() argument must be str, not int
```

`write` n'accepte que des chaînes. Convertissez : `f.write(str(42))` ou
`f.write(f"{42}\n")`.

### Oublier le mode d'écriture

```python norun
with open("notes.txt") as f:
    f.write("Bonjour")
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    f.write("Bonjour")
    ~~~~~~~^^^^^^^^^^^
io.UnsupportedOperation: not writable
```

Sans mode, `open` ouvre en lecture (`"r"`). Ajoutez `"w"` ou `"a"`.

### Lire après la fin du with

```python norun
with open("notes.txt", encoding="utf-8") as f:
    pass
print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    print(f.read())
          ~~~~~~^^
ValueError: I/O operation on closed file.
```

À la sortie du bloc `with`, le fichier est fermé. Lisez **dans** le bloc, et gardez le
résultat dans une variable.

## Testez-vous

**Question 1 :** quelle est la différence entre les modes `"w"` et `"a"` ?

> **Réponse**
>
> `"w"` vide le fichier avant d'écrire ; `"a"` écrit à la suite du contenu existant.
> Les deux créent le fichier s'il n'existe pas.

**Question 2 :** l'utilisateur tape `7` à `n = input("Nombre : ")`. Que vaut `n * 2` ?

> **Réponse**
>
> `"77"` : `input` renvoie la chaîne `"7"`, et une chaîne multipliée par 2 est répétée.
> Écrivez `int(input("Nombre : "))` pour obtenir le nombre 14.

**Question 3 :** écrivez un programme qui crée `courses.txt` avec trois articles, puis
affiche le nombre de lignes du fichier.

> **Réponse**
>
> ```python
> from pathlib import Path
>
> fichier = Path("courses.txt")
> fichier.write_text("pain\nlait\nœufs\n", encoding="utf-8")
> lignes = fichier.read_text(encoding="utf-8").splitlines()
> print(len(lignes), "lignes")
> ```
>
> ```sortie
> 3 lignes
> ```

## En résumé

- `input` lit une ligne au clavier et renvoie toujours une chaîne.
- `with open(nom, mode, encoding="utf-8") as f:` ouvre un fichier et le ferme tout seul.
- Les modes : `"r"` pour lire, `"w"` pour écraser, `"a"` pour ajouter à la fin.
- `read`, `readlines` ou une boucle `for` lisent le contenu ; `strip` enlève le `\n`.
- `pathlib.Path` gère chemins, dossiers (`mkdir`, `glob`) et lectures rapides
  (`read_text`, `write_text`).
- Un fichier absent lève `FileNotFoundError` : attrapez-la avec `try`/`except`.

Suite : [JSON et CSV](formats.md). Comparez avec
[les fichiers en Ruby](../ruby/fichiers.md).
