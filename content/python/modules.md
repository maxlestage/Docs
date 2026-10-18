# Modules, paquets et pip

Cette page explique comment importer du code, découper votre programme en plusieurs fichiers, utiliser la bibliothèque standard et installer des paquets avec pip dans un environnement virtuel.

> **Objectifs :**
>
> - vous saurez utiliser les différentes formes d'`import` ;
> - vous saurez écrire votre propre module et utiliser `if __name__ == "__main__"` ;
> - vous saurez organiser des modules en paquet avec `__init__.py` ;
> - vous saurez trouver de l'aide dans la bibliothèque standard ;
> - vous saurez créer un environnement virtuel et installer des paquets avec pip.

## Qu'est-ce qu'un module ?

Un [module](../debuter/glossaire.md#module) est simplement **un fichier `.py`** dont on
réutilise le contenu : ses fonctions, ses classes, ses variables. Python est livré avec
des centaines de modules prêts à l'emploi : c'est la **bibliothèque standard**.

Un module, c'est une boîte à outils rangée dans l'atelier. `import` va chercher la
boîte ; vous prenez ensuite les outils dont vous avez besoin.

## Les formes d'import

### import module

La forme la plus simple charge tout le module. On utilise ensuite ses éléments avec un
point : `module.nom`.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7))
```

```sortie
4.0
3.141592653589793
3
```

Le préfixe `math.` indique clairement d'où vient chaque fonction.

### from module import nom

Pour utiliser un élément sans préfixe, importez-le directement :

```python
from math import sqrt, pi

print(sqrt(25))
print(round(pi, 2))
```

```sortie
5.0
3.14
```

### Renommer avec as

`as` donne un autre nom, plus court ou plus clair, au module ou à l'élément importé :

```python
import statistics as stats
from collections import Counter as Compteur

notes = [12, 15, 9, 15, 18]
print(stats.mean(notes))
print(stats.median(notes))
print(Compteur(notes).most_common(1))
```

```sortie
13.8
15
[(15, 2)]
```

| Forme | Utilisation | Quand ? |
|-------|-------------|---------|
| `import math` | `math.sqrt(4)` | le cas par défaut, le plus clair |
| `from math import sqrt` | `sqrt(4)` | un ou deux éléments utilisés souvent |
| `import numpy as np` | `np.array(...)` | un nom long, avec un alias habituel |
| `from math import *` | `sqrt(4)` | **à éviter** : on ne sait plus d'où viennent les noms |

> **Astuce :** placez tous les `import` en haut du fichier : on voit tout de suite de
> quoi le programme a besoin.

## Écrire votre propre module

Quand un programme grandit, on le découpe en plusieurs fichiers. Créez deux fichiers
dans le **même dossier** :

```text
calculs/
├── geometrie.py
└── main.py
```

Le fichier `geometrie.py` contient des fonctions :

```python norun
"""Petites fonctions de géométrie."""

PI = 3.14159


def aire_cercle(rayon):
    return PI * rayon ** 2


def perimetre_cercle(rayon):
    return 2 * PI * rayon
```

Le fichier `main.py` les importe par le nom du fichier, **sans** `.py` :

```python norun
import geometrie
from geometrie import perimetre_cercle

print(geometrie.aire_cercle(2))
print(round(perimetre_cercle(1), 2))
```

On lance le programme principal depuis le dossier `calculs` :

```sh
python3 main.py
```

```text
12.56636
6.28
```

Python cherche les modules d'abord dans le **dossier du programme lancé**, puis dans la
bibliothèque standard, puis dans les paquets installés.

## if __name__ == "__main__"

Quand Python importe un module, il **exécute tout son code**, de haut en bas. Chaque
module a une variable spéciale, `__name__` :

- si le fichier est lancé directement (`python3 geometrie.py`), `__name__` vaut
  `"__main__"` ;
- s'il est importé, `__name__` vaut le nom du module, `"geometrie"`.

Ajoutons à la fin de `geometrie.py` :

```python norun
print("geometrie.py est chargé, __name__ =", __name__)

if __name__ == "__main__":
    print("Test rapide :", aire_cercle(1))
```

Selon la façon de lancer le fichier, le résultat change :

```sh
python3 main.py
```

```text
geometrie.py est chargé, __name__ = geometrie
12.56636
6.28
```

```sh
python3 geometrie.py
```

```text
geometrie.py est chargé, __name__ = __main__
Test rapide : 3.14159
```

> **À retenir :** le code sous `if __name__ == "__main__":` ne s'exécute **que** si le
> fichier est lancé directement. On y met des essais ou le point de départ du programme,
> pour que l'import reste silencieux.

Un programme Python bien rangé ressemble souvent à ceci :

```python
def main():
    print("Le programme démarre")

if __name__ == "__main__":
    main()
```

```sortie
Le programme démarre
```

## Les paquets

Un [paquet](../debuter/glossaire.md#paquet) (*package*) est un **dossier de modules**.
Il contient un fichier `__init__.py`, exécuté au premier import du paquet. Ce fichier
peut être vide.

```text
boutique/
├── main.py
└── magasin/
    ├── __init__.py
    ├── prix.py
    └── stock.py
```

```python norun
# magasin/prix.py
TVA = 0.20


def avec_tva(montant):
    return round(montant * (1 + TVA), 2)
```

```python norun
# magasin/stock.py
STOCK = {"stylo": 12, "cahier": 0}


def disponible(article):
    return STOCK.get(article, 0) > 0
```

Dans `__init__.py`, on peut remonter les éléments principaux pour raccourcir les
imports. Le point de `.prix` veut dire « le module `prix` de **ce** paquet » :

```python norun
# magasin/__init__.py
from .prix import avec_tva
```

On importe ensuite avec des points, qui suivent les dossiers :

```python norun
# main.py
from magasin import avec_tva
from magasin.stock import disponible
import magasin.prix

print(avec_tva(10))
print(disponible("stylo"), disponible("cahier"))
print(magasin.prix.TVA)
```

```text
12.0
True False
0.2
```

## Un tour de la bibliothèque standard

La bibliothèque standard couvre la plupart des besoins courants, sans rien installer.
Quelques modules à connaître :

| Module | Pour… |
|--------|-------|
| `math`, `statistics` | calculs, moyennes, médianes |
| `random` | tirages au hasard |
| `datetime` | dates et heures (voir [les dates](dates.md)) |
| `pathlib`, `os` | chemins et fichiers (voir [les fichiers](fichiers.md)) |
| `json`, `csv` | formats de données (voir [les formats](formats.md)) |
| `collections` | `Counter`, `defaultdict`, `deque` |
| `itertools`, `functools` | outils pour itérateurs et fonctions |
| `re` | [expressions régulières](regex.md) |
| `sys`, `argparse` | [ligne de commande](ligne-de-commande.md) |
| `unittest` | [tests](tests.md) |

Un petit échantillon :

```python
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path
import json

print(Counter("abracadabra").most_common(2))

par_lettre = defaultdict(list)
for mot in ["ane", "bus", "arc"]:
    par_lettre[mot[0]].append(mot)
print(dict(par_lettre))

print(date(2024, 7, 14).strftime("%d/%m/%Y"))
print(Path("photos/vacances/plage.jpg").suffix)
print(json.dumps({"nom": "Zoé", "age": 9}, ensure_ascii=False))
```

```sortie
[('a', 5), ('b', 2)]
{'a': ['ane', 'arc'], 'b': ['bus']}
14/07/2024
.jpg
{"nom": "Zoé", "age": 9}
```

> **Astuce :** pour explorer un module, utilisez `help(math)` ou `dir(math)` dans la
> console Python, ou la documentation officielle sur docs.python.org (en français).

## Installer des paquets avec pip

Des centaines de milliers de paquets écrits par d'autres sont publiés sur **PyPI**
(*Python Package Index*, pypi.org). L'outil **pip** les télécharge et les installe.

Exemple avec `requests`, un paquet très utilisé pour parler aux sites web :

```sh
python3 -m pip install requests
```

```text
Collecting requests
  Using cached requests-2.34.2-py3-none-any.whl.metadata (4.8 kB)
...
Installing collected packages: urllib3, idna, charset_normalizer, certifi, requests
Successfully installed certifi-2026.10.18 charset_normalizer-3.5.2 idna-3.20 requests-2.34.2 urllib3-2.8.0
```

pip a aussi installé les **dépendances** de `requests` : les paquets dont il a besoin
pour fonctionner. Ensuite, on l'importe comme un module de la bibliothèque standard :

```python norun
import requests

reponse = requests.get("https://example.com")
print(reponse.status_code)
```

```text
200
```

Quelques commandes pip utiles :

```sh
python3 -m pip list                 # les paquets installés
python3 -m pip show requests        # détails d'un paquet
python3 -m pip install --upgrade requests
python3 -m pip uninstall requests
```

> **Note :** écrivez `python3 -m pip` plutôt que `pip` tout court : vous êtes sûr
> d'installer pour **ce** Python-là, s'il y en a plusieurs sur l'ordinateur.

## Les environnements virtuels

Si tous vos projets installent leurs paquets au même endroit, ils finissent par se
gêner : le projet A veut la version 1 d'un paquet, le projet B la version 2. Un
**environnement virtuel** est un dossier qui contient un Python et des paquets **propres
à un projet**. Chaque projet a sa boîte, isolée des autres.

On le crée avec le module `venv`, en général dans un dossier nommé `.venv` :

```sh
cd mon-projet
python3 -m venv .venv
```

Puis on l'**active**, une fois par terminal :

```sh
# Linux et macOS
source .venv/bin/activate

# Windows (PowerShell)
.venv\Scripts\Activate.ps1
```

Le nom de l'environnement apparaît alors au début de l'invite, par exemple
`(.venv) $`. Désormais, `python` et `pip` sont ceux de l'environnement :

```sh
python -m pip install requests
deactivate
```

`deactivate` sort de l'environnement.

> **Attention :** ne mettez pas le dossier `.venv` dans [Git](../debuter/git.md) :
> ajoutez-le au fichier `.gitignore`. On le recrée à partir de `requirements.txt`.

## requirements.txt

Pour qu'une autre personne (ou vous, sur un autre ordinateur) installe exactement les
mêmes paquets, on les liste dans un fichier `requirements.txt`. `pip freeze` affiche les
paquets installés avec leur version :

```sh
python -m pip freeze > requirements.txt
```

```text
certifi==2026.10.18
charset-normalizer==3.5.2
idna==3.20
requests==2.34.2
urllib3==2.8.0
```

Sur une autre machine, dans un nouvel environnement virtuel, une seule commande
réinstalle tout :

```sh
python -m pip install -r requirements.txt
```

## Erreurs fréquentes

### Importer un paquet non installé

```python norun
import requests
print(requests.get("https://example.com").status_code)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
```

Le paquet n'est pas installé **dans le Python qui exécute le programme**. Activez votre
environnement virtuel, puis `python -m pip install requests`.

### Nommer son fichier comme un module standard

Un fichier nommé `random.py` qui contient :

```python norun
import random
print(random.randint(1, 6))
```

```text
Traceback (most recent call last):
  File "random.py", line 1, in <module>
    import random
  File "random.py", line 2, in <module>
    print(random.randint(1, 6))
          ^^^^^^^^^^^^^^
AttributeError: module 'random' has no attribute 'randint' (consider renaming 'random.py' since it has the same name as the standard library module named 'random' and prevents importing that standard library module)
```

Python cherche d'abord dans le dossier du programme : `import random` importe **votre**
fichier au lieu du module standard. Renommez-le, par exemple `de.py`.

### Importer un nom qui n'existe pas

```python norun
from math import racine
print(racine(16))
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    from math import racine
ImportError: cannot import name 'racine' from 'math' (unknown location)
```

Le module existe, mais pas ce nom : les noms sont en anglais. La fonction s'appelle
`sqrt`. Vérifiez avec `dir(math)`.

## Testez-vous

**Question 1 :** quelle différence entre `import math` et `from math import sqrt` ?

> **Réponse**
>
> Avec `import math`, on écrit `math.sqrt(4)`. Avec `from math import sqrt`, on écrit
> directement `sqrt(4)`, mais on ne voit plus d'où vient la fonction.

**Question 2 :** un fichier `outils.py` contient `print("chargé")` puis une fonction.
Que se passe-t-il quand `main.py` fait `import outils` ?

> **Réponse**
>
> `chargé` s'affiche : importer un module exécute tout son code. Pour l'éviter, placez
> ce `print` sous `if __name__ == "__main__":`.

**Question 3 :** dans quel ordre tapez-vous les commandes pour démarrer un projet qui
utilise `requests` ?

> **Réponse**
>
> ```sh
> python3 -m venv .venv
> source .venv/bin/activate
> python -m pip install requests
> python -m pip freeze > requirements.txt
> ```

## En résumé

- Un module est un fichier `.py` ; un paquet est un dossier de modules avec
  `__init__.py`.
- `import math`, `from math import sqrt` et `import ... as ...` sont les trois formes
  d'import.
- `if __name__ == "__main__":` contient le code exécuté seulement quand le fichier est
  lancé directement.
- La bibliothèque standard couvre beaucoup de besoins avant d'installer quoi que ce soit.
- `python3 -m venv .venv` crée un environnement virtuel ; `pip install` y ajoute des
  paquets ; `requirements.txt` les liste.

Suite : [lire et écrire des fichiers](fichiers.md). Comparez avec
[les modules en Ruby](../ruby/modules.md).
