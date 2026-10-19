# Les gestionnaires de contexte

Cette page explique comment `with` ouvre et libère une ressource à coup sûr, et comment écrire vos propres gestionnaires de contexte avec une classe ou avec `contextlib`.

> **Objectifs :**
>
> - vous saurez pourquoi `with` est plus sûr qu'un `close()` écrit à la main ;
> - vous saurez ouvrir plusieurs ressources dans un seul `with` ;
> - vous saurez écrire un gestionnaire avec une classe, `__enter__` et `__exit__` ;
> - vous saurez écrire un gestionnaire avec `@contextmanager` et `yield` ;
> - vous saurez utiliser `suppress`, `redirect_stdout`, `ExitStack` et
>   `TemporaryDirectory`.

## Le problème : oublier de nettoyer

Certaines choses doivent être **rendues** après usage : un fichier ouvert doit être
fermé, une connexion coupée, un verrou relâché. On appelle cela une **ressource**.

Fermer à la main semble simple :

```python
f = open("notes.txt", "w", encoding="utf-8")
f.write("15\n")
f.close()
print(f.closed)
```

```sortie
True
```

Mais si une [erreur](../debuter/glossaire.md#erreur) arrive entre `open` et `close`, la
ligne `f.close()` n'est jamais atteinte :

```python
f = open("notes.txt", "w", encoding="utf-8")
try:
    f.write("15\n")
    1 / 0
    f.close()
except ZeroDivisionError:
    print("erreur !")
print("fermé ?", f.closed)
```

```sortie
erreur !
fermé ? False
```

Le fichier reste ouvert. Dans un vrai programme, des fichiers oubliés finissent par
bloquer le système, ou des données ne sont jamais écrites sur le disque.

## La solution : with

`with` garantit le nettoyage. À la sortie du bloc indenté, la ressource est libérée,
**qu'il y ait eu une erreur ou non**. C'est comme une porte qui se referme toute seule
derrière vous.

```python
try:
    with open("notes.txt", "w", encoding="utf-8") as f:
        f.write("15\n")
        1 / 0
except ZeroDivisionError:
    print("erreur !")
print("fermé ?", f.closed)
```

```sortie
erreur !
fermé ? True
```

Cette fois, le fichier est fermé malgré l'erreur. L'objet placé après `with` (ici, le
fichier) s'appelle un **gestionnaire de contexte** : il sait quoi faire à l'entrée et à
la sortie du bloc.

> **À retenir :** `with` ne remplace pas `try` / `except`. Il n'attrape pas l'erreur : il
> nettoie, puis laisse l'erreur continuer son chemin.

## with open(...) as f

C'est l'usage le plus courant, vu dans [lire et écrire des fichiers](fichiers.md).
Lisez la ligne de gauche à droite :

- `open(...)` crée le gestionnaire de contexte ;
- `as f` donne un nom à ce qu'il fournit (le fichier ouvert) ;
- le bloc indenté est le moment où le fichier est ouvert.

```python
with open("fruits.txt", "w", encoding="utf-8") as f:
    f.write("pomme\npoire\n")

with open("fruits.txt", encoding="utf-8") as f:
    for ligne in f:
        print(ligne.strip())

print(f.closed)
```

```sortie
pomme
poire
True
```

La variable `f` existe encore après le bloc, mais le fichier est fermé : on ne peut plus
le lire.

## Plusieurs contextes dans un seul with

Pour ouvrir deux fichiers à la fois, séparez-les par une virgule. Ils sont ouverts de
gauche à droite, et fermés dans l'ordre inverse.

```python
with open("source.txt", "w", encoding="utf-8") as f:
    f.write("bonjour\nau revoir\n")

with open("source.txt", encoding="utf-8") as entree, \
     open("copie.txt", "w", encoding="utf-8") as sortie:
    for ligne in entree:
        sortie.write(ligne.upper())

with open("copie.txt", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
BONJOUR
AU REVOIR
```

## Écrire un gestionnaire avec une classe

N'importe quel objet peut servir après `with`, s'il possède deux méthodes spéciales :

- `__enter__(self)` est appelée à l'**entrée** du bloc. Ce qu'elle renvoie est rangé
  dans la variable après `as` ;
- `__exit__(self, type_err, err, trace)` est appelée à la **sortie** du bloc, toujours.

```python
class Section:
    def __init__(self, titre):
        self.titre = titre

    def __enter__(self):
        print(f"[début {self.titre}]")
        return self

    def __exit__(self, type_err, err, trace):
        print(f"[fin {self.titre}]")

with Section("calcul") as s:
    print("dans le bloc :", s.titre)
    print(2 + 2)
```

```sortie
[début calcul]
dans le bloc : calcul
4
[fin calcul]
```

Python fait exactement ceci : il appelle `__enter__`, exécute le bloc, puis appelle
`__exit__`, même si le bloc a levé une [exception](../debuter/glossaire.md#exception).

> **Attention :** n'oubliez pas `return self` dans `__enter__`. Sans `return`, elle
> renvoie `None`, et la variable après `as` vaut `None`.

### Ce que reçoit __exit__

`__exit__` reçoit trois arguments qui décrivent l'erreur éventuelle :

| Argument | S'il n'y a pas d'erreur | S'il y a une erreur |
|----------|-------------------------|---------------------|
| `type_err` | `None` | la classe, par exemple `ZeroDivisionError` |
| `err` | `None` | l'objet exception lui-même |
| `trace` | `None` | la trace (*traceback*) |

```python
class Espion:
    def __enter__(self):
        return self

    def __exit__(self, type_err, err, trace):
        if type_err is None:
            print("sortie normale")
        else:
            print("sortie sur erreur :", type_err.__name__, "-", err)

with Espion():
    print("tout va bien")

try:
    with Espion():
        int("abc")
except ValueError:
    print("l'erreur a continué son chemin")
```

```sortie
tout va bien
sortie normale
sortie sur erreur : ValueError - invalid literal for int() with base 10: 'abc'
l'erreur a continué son chemin
```

`__exit__` **voit** l'erreur, mais ne l'arrête pas : elle remonte ensuite jusqu'au
`except`.

### Renvoyer True pour avaler l'erreur

La valeur renvoyée par `__exit__` compte :

- si elle renvoie une valeur **vraie** (`True`), Python considère l'erreur comme
  réglée : elle disparaît, et le programme continue **après** le bloc `with` ;
- si elle renvoie une valeur fausse (`False`, ou `None` quand il n'y a pas de `return`),
  l'erreur continue de remonter.

```python
class IgnorerZero:
    def __enter__(self):
        return self

    def __exit__(self, type_err, err, trace):
        if type_err is ZeroDivisionError:
            print("division par zéro ignorée")
            return True
        return False

with IgnorerZero():
    print("avant")
    1 / 0
    print("jamais affiché")
print("après le with")
```

```sortie
avant
division par zéro ignorée
après le with
```

Remarquez bien deux choses :

1. `"jamais affiché"` ne s'affiche pas. Avaler l'erreur ne « reprend » pas le bloc là
   où il s'était arrêté : la suite du bloc est perdue.
2. Seule `ZeroDivisionError` est avalée. Une autre erreur, comme `ValueError`, reçoit
   `return False` et remonte normalement.

> **Attention :** n'avalez que les erreurs **précises** que vous attendez. Un `__exit__`
> qui renvoie toujours `True` cache tous les bugs du bloc, comme un `except:` tout seul.

## contextmanager : un gestionnaire avec yield

Écrire une classe pour chaque gestionnaire, c'est un peu long. Le
[décorateur](../debuter/glossaire.md#decorateur) `@contextmanager` du module `contextlib`
transforme un [générateur](../debuter/glossaire.md#generateur) en gestionnaire :

- le code **avant** `yield` joue le rôle de `__enter__` ;
- la valeur donnée à `yield` est rangée dans la variable après `as` ;
- le code **après** `yield` joue le rôle de `__exit__`.

```python
from contextlib import contextmanager

@contextmanager
def balise(nom):
    print(f"<{nom}>")
    yield nom.upper()
    print(f"</{nom}>")

with balise("p") as b:
    print("  bonjour", b)
```

```sortie
<p>
  bonjour P
</p>
```

### Toujours avec try / finally

Si une erreur arrive dans le bloc `with`, Python la relance **à l'endroit du `yield`**.
Sans précaution, le code après `yield` n'est donc pas exécuté. Mettez le nettoyage dans
un `finally` :

```python
from contextlib import contextmanager

@contextmanager
def balise(nom):
    print(f"<{nom}>")
    try:
        yield
    finally:
        print(f"</{nom}>")

try:
    with balise("div"):
        print("  contenu")
        1 / 0
except ZeroDivisionError:
    print("erreur attrapée")
```

```sortie
<div>
  contenu
</div>
erreur attrapée
```

`</div>` s'affiche malgré l'erreur : le nettoyage est garanti.

> **À retenir :** un générateur décoré par `@contextmanager` doit faire **un seul**
> `yield`, entouré de `try` / `finally`.

## suppress : ignorer une erreur précise

`contextlib.suppress` (« supprimer ») fait ce que faisait notre classe `IgnorerZero` :
il avale les erreurs des classes indiquées, et seulement celles-là.

```python
from contextlib import suppress
from pathlib import Path

with suppress(FileNotFoundError):
    Path("brouillon.txt").unlink()
    print("fichier supprimé")
print("on continue")
```

```sortie
on continue
```

Le fichier n'existait pas : `unlink` a levé `FileNotFoundError`, `suppress` l'a
avalée, et la ligne `"fichier supprimé"` a été sautée. C'est un raccourci pour un
`try` / `except FileNotFoundError: pass`.

## redirect_stdout : capturer ce qu'affiche print

`print` écrit sur la **sortie standard** (*stdout*), en général l'écran.
`contextlib.redirect_stdout` l'envoie ailleurs le temps du bloc. Avec `io.StringIO`,
une sorte de fichier gardé en mémoire, vous récupérez le texte dans une chaîne.

```python
import io
from contextlib import redirect_stdout

def saluer():
    print("Bonjour !")
    print("Au revoir !")

tampon = io.StringIO()
with redirect_stdout(tampon):
    saluer()

texte = tampon.getvalue()
print("capturé :", repr(texte))
print(len(texte.splitlines()), "lignes")
```

```sortie
capturé : 'Bonjour !\nAu revoir !\n'
2 lignes
```

Pendant le bloc, rien ne s'affiche à l'écran. À la sortie, `print` écrit de nouveau
normalement. C'est très pratique pour [tester](tests.md) une fonction qui affiche.

## ExitStack : un nombre variable de contextes

Quand le nombre de fichiers n'est connu qu'à l'exécution, on ne peut pas tous les écrire
dans le `with`. `contextlib.ExitStack` (« pile de sorties ») les accumule avec
`enter_context`, puis les ferme tous à la fin du bloc, dans l'ordre inverse.

```python
from contextlib import ExitStack

noms = ["lundi.txt", "mardi.txt", "mercredi.txt"]

with ExitStack() as pile:
    fichiers = [pile.enter_context(open(nom, "w", encoding="utf-8")) for nom in noms]
    for nom, f in zip(noms, fichiers):
        f.write(f"journal de {nom}\n")
    print(len(fichiers), "fichiers ouverts")

print(all(f.closed for f in fichiers))
```

```sortie
3 fichiers ouverts
True
```

> **Note :** vous n'aurez pas souvent besoin d'`ExitStack`. Retenez simplement qu'il
> existe pour le jour où le nombre de ressources varie.

## Exemple pratique : un dossier temporaire

`tempfile.TemporaryDirectory` crée un dossier vide, au nom choisi au hasard, et le
**supprime avec tout son contenu** à la sortie du bloc. C'est idéal pour des fichiers de
travail jetables.

```python
import tempfile
from pathlib import Path

with tempfile.TemporaryDirectory() as nom_dossier:
    dossier = Path(nom_dossier)
    (dossier / "a.txt").write_text("un", encoding="utf-8")
    (dossier / "b.txt").write_text("deux", encoding="utf-8")
    print(sorted(f.name for f in dossier.iterdir()))
    print("existe :", dossier.exists())

print("existe après :", dossier.exists())
```

```sortie
['a.txt', 'b.txt']
existe : True
existe après : False
```

Le nom du dossier change à chaque fois (par exemple `/tmp/tmpk3x9a1`) : on ne l'affiche
donc pas. On n'affiche que ce qui ne dépend pas du hasard.

## Erreurs fréquentes

### Utiliser with sur un objet qui n'est pas un gestionnaire

```python norun
with "notes.txt" as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with "notes.txt" as f:
         ^^^^^^^^^^^
TypeError: 'str' object does not support the context manager protocol
```

Une chaîne n'a ni `__enter__` ni `__exit__`. Il manque `open` :
`with open("notes.txt", encoding="utf-8") as f:`.

### Oublier return self dans __enter__

```python norun
class Compteur:
    def __init__(self):
        self.total = 0

    def __enter__(self):
        print("début")

    def __exit__(self, type_err, err, trace):
        print("fin")

with Compteur() as c:
    c.total += 1
```

```text
début
fin
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    c.total += 1
    ^^^^^^^
AttributeError: 'NoneType' object has no attribute 'total'
```

`__enter__` ne renvoie rien, donc `c` vaut `None`. Ajoutez `return self` à la fin de
`__enter__`.

### Oublier les paramètres de __exit__

```python norun
class Section:
    def __enter__(self):
        return self

    def __exit__(self):
        print("fin")

with Section():
    print("dedans")
```

```text
dedans
Traceback (most recent call last):
  File "main.py", line 8, in <module>
    with Section():
         ~~~~~~~^^
TypeError: Section.__exit__() takes 1 positional argument but 4 were given
```

Python appelle toujours `__exit__` avec trois arguments en plus de `self`. Écrivez
`def __exit__(self, type_err, err, trace):`, même si vous ne vous en servez pas.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```python norun
class Porte:
    def __enter__(self):
        print("ouvre")
        return self

    def __exit__(self, type_err, err, trace):
        print("ferme")

try:
    with Porte():
        print("entre")
        raise ValueError("oups")
except ValueError:
    print("attrapée")
```

> **Réponse**
>
> `ouvre`, `entre`, `ferme`, puis `attrapée`. `__exit__` est appelée avant que l'erreur
> n'atteigne le `except`. Elle ne renvoie rien (donc `None`, une valeur fausse) :
> l'erreur n'est pas avalée.

**Question 2 :** dans une fonction décorée par `@contextmanager`, pourquoi entourer le
`yield` de `try` / `finally` ?

> **Réponse**
>
> Parce qu'une erreur dans le bloc `with` est relancée à l'endroit du `yield`. Sans
> `finally`, le code de nettoyage placé après `yield` ne serait pas exécuté.

**Question 3 :** écrivez avec `suppress` un code qui convertit `"abc"` en entier, et
affiche `"fini"` sans planter.

> **Réponse**
>
> ```python
> from contextlib import suppress
>
> with suppress(ValueError):
>     n = int("abc")
>     print("converti :", n)
> print("fini")
> ```
>
> ```sortie
> fini
> ```

## En résumé

- `with` libère une ressource à la sortie du bloc, même en cas d'erreur ; il n'attrape
  pas l'erreur.
- `with a as x, b as y:` ouvre plusieurs ressources, fermées dans l'ordre inverse.
- Une classe devient un gestionnaire avec `__enter__` (sa valeur va dans `as`) et
  `__exit__(self, type_err, err, trace)`.
- Si `__exit__` renvoie `True`, l'erreur est avalée et le programme reprend après le
  bloc ; sinon, elle remonte.
- `@contextmanager` : le code avant `yield` à l'entrée, après `yield` (dans `finally`) à
  la sortie.
- `contextlib` fournit aussi `suppress`, `redirect_stdout` et `ExitStack` ;
  `tempfile.TemporaryDirectory` crée un dossier jetable.

Suite : [les modules](modules.md).
