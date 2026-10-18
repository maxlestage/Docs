# Organiser un projet

Cette page montre comment ranger un vrai projet Python : les dossiers, l'environnement virtuel, les paquets installés avec pip, le fichier `pyproject.toml` et la commande qui lance votre programme.

> **Objectifs :**
>
> - ranger un projet en dossiers : le code, les tests, la configuration ;
> - créer et activer un environnement virtuel avec `venv` ;
> - installer des paquets avec pip et les noter dans `requirements.txt` ;
> - décrire le projet dans un `pyproject.toml` et lui donner une commande ;
> - connaître uv, l'outil qui fait tout cela plus vite.

## D'un fichier à un projet

Au début, tout tient dans un seul fichier `main.py`. Puis le programme grandit : vous
le découpez en [modules](modules.md), vous ajoutez des [tests](tests.md), vous utilisez
des bibliothèques écrites par d'autres. Il faut alors ranger.

Voici la forme la plus simple d'un projet bien rangé :

```text
notes/
├── notes.py            # le code
├── test_notes.py       # les tests
├── requirements.txt    # les bibliothèques à installer
└── README.md           # à quoi sert le projet, comment le lancer
```

Pour un petit outil, cela suffit. La suite de la page montre la forme plus complète,
utilisée par la plupart des projets publiés.

## Le point d'entrée

Un fichier Python peut être **lancé** (`python3 notes.py`) ou **importé** par un autre
fichier (`import notes`). La variable spéciale `__name__` dit dans quel cas on est : elle
vaut `"__main__"` quand le fichier est lancé directement.

On range donc le programme principal dans une fonction `main`, appelée seulement quand
le fichier est lancé :

```python
def moyenne(notes):
    return sum(notes) / len(notes)


def main():
    print(f"Moyenne : {moyenne([12, 15, 18]):.2f}")


if __name__ == "__main__":
    main()
```

```sortie
Moyenne : 15.00
```

Un autre fichier peut maintenant faire `from notes import moyenne` sans déclencher
l'affichage. Les tests, par exemple.

> **À retenir :** `if __name__ == "__main__":` est le **point d'entrée** du programme :
> la porte par laquelle on entre quand on le lance.

## L'environnement virtuel

Chaque projet a besoin de ses propres bibliothèques, parfois dans des versions
différentes. Si vous installez tout au même endroit, les projets se marchent sur les
pieds.

Un **environnement virtuel** (*virtual environment*, ou *venv*) est un dossier qui
contient une copie de Python **rien que pour ce projet**. Ce que vous y installez ne
touche pas le reste de l'ordinateur. Pensez à une boîte à outils par chantier.

Créez-le une fois, dans le dossier du projet (voir [le terminal](../debuter/terminal.md)) :

```sh
cd notes
python3 -m venv .venv
```

Puis **activez-le** à chaque nouvelle session de terminal :

```sh
source .venv/bin/activate        # Linux et macOS
.venv\Scripts\activate           # Windows
```

Le nom de l'environnement apparaît au début de l'invite : `(.venv) $`. Désormais,
`python` et `pip` sont ceux du projet. Pour en sortir :

```sh
deactivate
```

> **Attention :** le dossier `.venv` ne se partage pas et ne se met pas dans Git (voir
> [Git](../debuter/git.md)) : ajoutez `.venv/` au fichier `.gitignore`. Chacun recrée le
> sien à partir de la liste des bibliothèques.

## Installer des paquets avec pip

Un **[paquet](../debuter/glossaire.md#paquet)** est une bibliothèque prête à installer.
Des centaines de milliers sont publiées sur **PyPI** (*Python Package Index*,
pypi.org). L'outil **pip** les télécharge et les installe dans l'environnement actif.

```sh
python -m pip install requests        # installer
python -m pip install "requests==2.34.2"  # une version précise
python -m pip list                    # voir ce qui est installé
python -m pip uninstall requests      # désinstaller
```

> **Astuce :** écrivez `python -m pip` plutôt que `pip` tout court. Vous êtes ainsi sûr
> d'utiliser le pip du Python que vous croyez.

## Le fichier requirements.txt

Pour que d'autres (ou vous, sur un autre ordinateur) installent les mêmes paquets,
notez-les dans un fichier `requirements.txt`. `pip freeze` écrit la liste exacte de ce
qui est installé :

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

`requests` a amené avec lui quatre autres paquets dont il a besoin : ses
**dépendances**. Sur un autre ordinateur, on recrée le même environnement en deux
commandes :

```sh
python3 -m venv .venv && source .venv/bin/activate
python -m pip install -r requirements.txt
```

## La disposition src

Quand le projet grandit, le code devient un **paquet** : un dossier de modules, avec un
fichier `__init__.py` (voir [les modules](modules.md)). La disposition recommandée
range ce paquet dans un dossier `src/` :

```text
notes/
├── pyproject.toml          # la carte d'identité du projet
├── README.md
├── src/
│   └── notes/              # le paquet
│       ├── __init__.py     # fait du dossier un paquet (peut être vide)
│       ├── __main__.py     # lancé par « python -m notes »
│       └── calcul.py
└── tests/
    └── test_calcul.py
```

```python norun
# src/notes/calcul.py
def moyenne(notes):
    return sum(notes) / len(notes)
```

```python norun
# src/notes/__main__.py
import sys

from notes.calcul import moyenne


def main():
    valeurs = [float(x) for x in sys.argv[1:]]
    print(f"Moyenne : {moyenne(valeurs):.2f}")


if __name__ == "__main__":
    main()
```

```python norun
# tests/test_calcul.py
from notes.calcul import moyenne


def test_moyenne():
    assert moyenne([10, 20]) == 15
```

Pourquoi `src/` ? Il oblige à **installer** le paquet avant de l'utiliser. Les tests
vérifient ainsi le paquet tel que vos utilisateurs le recevront, et pas un fichier qui
traîne par hasard dans le dossier courant.

## Le fichier pyproject.toml

`pyproject.toml` décrit le projet : son nom, sa version, la version de Python attendue,
ses dépendances et ses commandes. Il est écrit en TOML, un format de configuration
simple (voir [les formats de données](formats.md)).

```toml
[project]
name = "notes"
version = "0.1.0"
description = "Calcule des moyennes"
requires-python = ">=3.11"
dependencies = []

[project.optional-dependencies]
dev = ["pytest"]

[project.scripts]
notes = "notes.__main__:main"

[build-system]
requires = ["setuptools>=61"]
build-backend = "setuptools.build_meta"
```

| Section | Rôle |
| --- | --- |
| `[project]` | nom, version, description, version de Python |
| `dependencies` | les paquets nécessaires pour faire tourner le programme |
| `[project.optional-dependencies]` | des groupes en option : ici `dev`, pour développer |
| `[project.scripts]` | les commandes créées à l'installation |
| `[build-system]` | l'outil qui construit le paquet (ici setuptools) |

La ligne `notes = "notes.__main__:main"` veut dire : « crée une commande `notes` qui
appelle la fonction `main` du module `notes.__main__` ».

## Installer et lancer le projet

Installez le projet dans son environnement virtuel, en mode **modifiable** (`-e`,
*editable*) : vos changements dans `src/` sont pris en compte sans réinstaller.
`".[dev]"` ajoute le groupe `dev`, donc pytest.

```sh
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -e ".[dev]"
```

Le projet se lance maintenant de trois façons :

```sh
notes 12 15 18            # la commande déclarée dans [project.scripts]
python -m notes 12 15 18  # le fichier __main__.py du paquet
pytest                    # les tests
```

```text
Moyenne : 15.00
Moyenne : 15.00
.                                                                        [100%]
1 passed in 0.03s
```

Pour lire les arguments plus proprement que `sys.argv`, voyez
[la ligne de commande](ligne-de-commande.md).

## uv : tout en un, et plus vite

**uv** est un outil récent qui remplace `venv`, `pip` et quelques autres, en beaucoup
plus rapide. Il s'installe à part (voir sa documentation sur docs.astral.sh/uv). Les
mêmes étapes, avec uv :

```sh
uv init notes          # crée le dossier, pyproject.toml et un main.py
cd notes
uv add requests        # installe et note la dépendance dans pyproject.toml
uv add --dev pytest    # une dépendance de développement
uv run main.py         # lance le programme dans le bon environnement
uv run pytest          # lance les tests
```

uv crée le `.venv` tout seul et note les versions exactes dans un fichier `uv.lock`.
Plus besoin d'activer l'environnement : `uv run` s'en charge.

> **Note :** pip et `venv` sont livrés avec Python et marchent partout : apprenez-les
> d'abord. uv fait la même chose ; vous passerez de l'un à l'autre sans peine.

## Erreurs fréquentes

### Utiliser un paquet qui n'est pas installé

Chaque environnement virtuel part vide. Un paquet installé ailleurs n'y est pas.

```python norun
import requests
```

```text
Traceback (most recent call last):
  File "/home/alice/notes/main.py", line 1, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
```

Vérifiez que l'environnement est activé (`(.venv)` dans l'invite), puis
`python -m pip install requests`, et ajoutez-le à `requirements.txt` ou `pyproject.toml`.

### Donner à son fichier le nom d'un module de Python

Un fichier `random.py` dans votre dossier **cache** le module `random` de Python :
`import random` importe votre propre fichier.

```python norun
# random.py
import random

print(random.randint(1, 6))
```

```text
Traceback (most recent call last):
  File "/home/alice/jeu/random.py", line 1, in <module>
    import random
  File "/home/alice/jeu/random.py", line 3, in <module>
    print(random.randint(1, 6))
          ^^^^^^^^^^^^^^
AttributeError: module 'random' has no attribute 'randint' (consider renaming '/home/alice/jeu/random.py' since it has the same name as the standard library module named 'random' and prevents importing that standard library module)
```

Python vous le dit lui-même : renommez le fichier, par exemple `de.py`.

### Lancer un paquet src sans l'avoir installé

Avec la disposition `src/`, Python ne trouve pas le paquet tant qu'il n'est pas
installé.

```sh
python3 -m notes 12 15
```

```text
/usr/bin/python3: No module named notes
```

Le message montre aussi `/usr/bin/python3` : ce n'est pas le Python de l'environnement
virtuel. Activez `.venv`, lancez `python -m pip install -e .`, puis réessayez.

## Testez-vous

**1.** À quoi sert un environnement virtuel ?

> **Réponse**
>
> À isoler les paquets d'un projet : ce qu'on y installe ne touche ni Python ni les
> autres projets. Chaque projet a sa propre « boîte à outils ».

**2.** Comment recréer les paquets d'un projet qui a un `requirements.txt` ?

> **Réponse**
>
> Créer et activer un environnement virtuel, puis lancer
> `python -m pip install -r requirements.txt`.

**3.** Que fait `if __name__ == "__main__":` ?

> **Réponse**
>
> Le bloc en dessous ne s'exécute que si le fichier est lancé directement, pas quand il
> est importé par un autre fichier. C'est le point d'entrée du programme.

## En résumé

- Un projet range son code, ses tests et sa configuration ; la disposition `src/` est
  la plus sûre.
- `python3 -m venv .venv` puis `source .venv/bin/activate` : un environnement par projet.
- `python -m pip install` installe un paquet ; `requirements.txt` en garde la liste.
- `pyproject.toml` décrit le projet, ses dépendances et ses commandes
  (`[project.scripts]`).
- `if __name__ == "__main__":` marque le point d'entrée ; uv réunit tous ces outils.

Suite : mettez tout en pratique avec [les projets pas à pas](projets.md). Comparez avec
[l'organisation d'un projet Ruby](../ruby/projet.md) et [Rust](../rust/projet.md).
