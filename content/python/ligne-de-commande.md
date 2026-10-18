# Écrire un outil en ligne de commande

Cette page montre comment écrire un programme qu'on lance dans le terminal avec des arguments, comme les vraies commandes `ls` ou `git`.

> **Objectifs :**
>
> - vous saurez lire les arguments du programme avec `sys.argv` ;
> - vous saurez décrire des arguments et des options avec `argparse` ;
> - vous saurez signaler une réussite ou un échec avec un code de sortie ;
> - vous saurez séparer résultats et messages d'erreur (`stdout` et `stderr`) ;
> - vous saurez écrire un outil complet qui compte les mots d'un fichier.

## Un programme qui reçoit des arguments

Dans le [terminal](../debuter/glossaire.md#terminal), vous pouvez écrire des valeurs
après le nom du programme. Ce sont ses **arguments** :

```sh
python saluer.py Alice 3 "Jean Dupont"
```

Le programme les reçoit dans la liste `sys.argv` (*argument vector*, « liste des
arguments »). La case 0 contient le nom du programme ; les vrais arguments commencent
à la case 1.

```python norun
# saluer.py
import sys

print("Nom du programme :", sys.argv[0])
print("Arguments :", sys.argv[1:])
```

```text
Nom du programme : saluer.py
Arguments : ['Alice', '3', 'Jean Dupont']
```

Les guillemets autour de `"Jean Dupont"` en font **un seul** argument, malgré l'espace.

> **À retenir :** les arguments sont **toujours** des chaînes. `'3'` n'est pas le
> nombre 3 : convertissez avec `int`.

### Tester sans terminal

Pour essayer, vous pouvez remplir `sys.argv` vous-même. Ici, on simule la commande
`python addition.py 12 30` :

```python
import sys

sys.argv = ["addition.py", "12", "30"]

nombres = [int(a) for a in sys.argv[1:]]
print(" + ".join(sys.argv[1:]), "=", sum(nombres))
```

```sortie
12 + 30 = 42
```

`sys.argv` suffit pour un petit script. Dès qu'il y a des options, utilisez `argparse`.

## argparse : décrire ses arguments

`argparse` lit les arguments à votre place. Vous décrivez ce que vous attendez ; il
vérifie, convertit, et écrit même l'aide. C'est un guichetier qui contrôle les
formulaires avant de vous les passer.

```python
import argparse

parser = argparse.ArgumentParser(prog="saluer", description="Salue quelqu'un.")
parser.add_argument("nom", help="la personne à saluer")
parser.add_argument("--fois", type=int, default=1, help="nombre de saluts")
parser.add_argument("--crier", action="store_true", help="tout en majuscules")

args = parser.parse_args(["Alice", "--fois", "2", "--crier"])

message = f"Bonjour, {args.nom} !"
if args.crier:
    message = message.upper()
for _ in range(args.fois):
    print(message)
```

```sortie
BONJOUR, ALICE !
BONJOUR, ALICE !
```

Lisez les trois `add_argument` :

- `"nom"` est un argument **positionnel** : obligatoire, reconnu par sa place ;
- `"--fois"` est une **option** : facultative, avec une valeur par défaut, et
  `type=int` la convertit en entier ;
- `"--crier"` est un **drapeau** : `action="store_true"` donne `True` s'il est présent,
  `False` sinon.

> **Note :** dans un vrai programme, on écrit `parser.parse_args()` sans rien : il lit
> alors `sys.argv`. Ici, on lui donne une liste fixe pour que l'exemple tourne tout
> seul, sans terminal.

### L'aide gratuite

`argparse` fabrique un message d'aide à partir de vos descriptions. L'utilisateur
l'obtient avec `-h` ou `--help`.

```python
import argparse

parser = argparse.ArgumentParser(prog="saluer", description="Salue quelqu'un.")
parser.add_argument("nom", help="la personne à saluer")
parser.add_argument("--fois", type=int, default=1, help="nombre de saluts")
parser.add_argument("--crier", action="store_true", help="tout en majuscules")
parser.print_help()
```

```sortie
usage: saluer [-h] [--fois FOIS] [--crier] nom

Salue quelqu'un.

positional arguments:
  nom          la personne à saluer

options:
  -h, --help   show this help message and exit
  --fois FOIS  nombre de saluts
  --crier      tout en majuscules
```

### Options courtes et choix limités

Une option peut avoir un nom court (`-n`) et un nom long (`--nombre`). `choices`
limite les valeurs permises ; `nargs="+"` accepte une ou plusieurs valeurs.

```python
import argparse

parser = argparse.ArgumentParser(prog="calcul")
parser.add_argument("nombres", type=float, nargs="+")
parser.add_argument("-o", "--operation", choices=["somme", "moyenne"], default="somme")

for essai in [["4", "8", "15"], ["4", "8", "15", "-o", "moyenne"]]:
    args = parser.parse_args(essai)
    total = sum(args.nombres)
    resultat = total if args.operation == "somme" else total / len(args.nombres)
    print(args.operation, ":", resultat)
```

```sortie
somme : 27.0
moyenne : 9.0
```

Si l'utilisateur tape `-o produit`, `argparse` refuse et affiche l'erreur tout seul.

## Les codes de sortie

Quand un programme se termine, il renvoie un **code de sortie** au système : `0` veut
dire « tout s'est bien passé », un autre nombre veut dire « problème ». Les autres
programmes (scripts, outils d'automatisation) lisent ce code pour savoir si la commande
a réussi.

`sys.exit(code)` arrête le programme avec ce code.

```python norun
# verif.py
import sys

if len(sys.argv) != 2:
    print("Usage : python verif.py NOMBRE", file=sys.stderr)
    sys.exit(1)

print("Le double est", int(sys.argv[1]) * 2)
```

Dans le terminal, `echo $?` affiche le code de la dernière commande :

```text
$ python verif.py
Usage : python verif.py NOMBRE
$ echo $?
1
$ python verif.py 21
Le double est 42
$ echo $?
0
```

| Code | Usage habituel |
|------|----------------|
| `0` | succès (c'est aussi le code quand le programme finit normalement) |
| `1` | erreur générale |
| `2` | mauvaise utilisation (c'est le code d'`argparse` quand un argument manque) |

## Deux sorties : stdout et stderr

Un programme a deux sorties :

- `stdout` (*standard output*) : les **résultats**, ce que `print` écrit par défaut ;
- `stderr` (*standard error*) : les **messages** d'erreur ou d'information.

Pourquoi deux ? Parce qu'on redirige souvent les résultats dans un fichier
(`python outil.py > resultat.txt`). Les erreurs, elles, doivent rester visibles à
l'écran. Écrivez-les avec `print(..., file=sys.stderr)`.

```python
import sys

print("résultat : 42")
print("attention : 3 lignes ignorées", file=sys.stderr)
```

```sortie
résultat : 42
```

Seul le résultat apparaît dans la sortie standard ; l'avertissement est parti sur
`stderr`.

## Lire l'entrée standard

Un outil peut aussi lire ce qu'on lui envoie par un **tube** (`|`) : la sortie d'une
commande devient l'entrée de la suivante. Cette entrée s'appelle `stdin` (*standard
input*). On la parcourt comme un fichier.

```python norun
# maj.py
import sys

for ligne in sys.stdin:
    print(ligne.rstrip("\n").upper())
```

```text
$ printf 'bonjour\nà tous\n' | python maj.py
BONJOUR
À TOUS
```

Sans tube, le programme attend que vous tapiez au clavier ; `Ctrl+D` (ou `Ctrl+Z` puis
Entrée sous Windows) signale la fin.

## Un outil complet : compter les mots

Voici un vrai petit outil, `compter`, qui affiche le nombre de lignes, de mots et de
caractères d'un fichier, et éventuellement ses mots les plus fréquents.

Le programme est rangé dans une fonction `main(arguments)`. C'est une bonne habitude :
on peut l'appeler avec une liste pour le tester, et avec `None` (donc `sys.argv`) en
vrai. Ici, le début crée un fichier d'exemple, puis appelle `main` comme si on avait
tapé des commandes.

```python
import argparse
import re
import sys
from collections import Counter
from pathlib import Path


def compter(texte):
    mots = re.findall(r"\w+", texte.lower())
    return len(texte.splitlines()), mots, len(texte)


def main(arguments=None):
    parser = argparse.ArgumentParser(prog="compter", description="Compte les mots d'un fichier.")
    parser.add_argument("fichier", type=Path, help="le fichier texte à lire")
    parser.add_argument("--top", type=int, default=0, help="afficher les N mots les plus fréquents")
    args = parser.parse_args(arguments)

    try:
        texte = args.fichier.read_text(encoding="utf-8")
    except FileNotFoundError:
        print(f"compter: {args.fichier}: fichier introuvable", file=sys.stderr)
        return 1

    nb_lignes, mots, nb_caracteres = compter(texte)
    print(f"{nb_lignes} lignes, {len(mots)} mots, {nb_caracteres} caractères")

    if args.top > 0:
        frequences = Counter(mots).most_common(args.top)
        for mot, nombre in frequences:
            print(f"  {mot:<8} {nombre}")
    return 0


Path("poeme.txt").write_text(
    "Le chat dort.\nLe chien dort aussi.\nLe soleil brille sur le chat.\n",
    encoding="utf-8",
)

print("$ compter poeme.txt")
code = main(["poeme.txt"])
print("code :", code)

print("$ compter poeme.txt --top 3")
code = main(["poeme.txt", "--top", "3"])
print("code :", code)

print("$ compter absent.txt")
code = main(["absent.txt"])
print("code :", code)
```

```sortie
$ compter poeme.txt
3 lignes, 13 mots, 65 caractères
code : 0
$ compter poeme.txt --top 3
3 lignes, 13 mots, 65 caractères
  le       4
  chat     2
  dort     2
code : 0
$ compter absent.txt
code : 1
```

Pour la troisième commande, le message « fichier introuvable » part sur `stderr` : il
ne fait pas partie des résultats.

Dans le vrai fichier `compter.py`, on remplace les appels d'essai par ces deux lignes :

```python norun
if __name__ == "__main__":
    sys.exit(main())
```

`if __name__ == "__main__":` veut dire « seulement si ce fichier est lancé directement,
pas importé » (voir [les modules](modules.md)). `sys.exit(main())` transmet le code
renvoyé par `main` au système.

```text
$ python compter.py poeme.txt --top 1
3 lignes, 13 mots, 65 caractères
  le       4
```

## Erreurs fréquentes

### Lire un argument absent

```python norun
import sys
print(sys.argv[1])
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(sys.argv[1])
          ~~~~~~~~^^^
IndexError: list index out of range
```

Le programme a été lancé sans argument : `sys.argv` ne contient que le nom du
programme. Vérifiez `len(sys.argv)` avant, ou utilisez `argparse`.

### Oublier type=int

```python norun
import argparse
p = argparse.ArgumentParser(prog="repeter")
p.add_argument("fois")
a = p.parse_args(["3"])
print(a.fois + 1)
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    print(a.fois + 1)
          ~~~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

Sans `type=int`, `argparse` garde la chaîne `"3"`. Écrivez
`p.add_argument("fois", type=int)`.

### Oublier un argument obligatoire

```python norun
import argparse
p = argparse.ArgumentParser(prog="compter")
p.add_argument("fichier")
p.parse_args([])
```

```text
usage: compter [-h] fichier
compter: error: the following arguments are required: fichier
```

Ce n'est pas un bug du programme : `argparse` a fait son travail. Il affiche l'usage
sur `stderr` et quitte avec le code `2`. Donnez l'argument, ou rendez-le facultatif
avec `--fichier` et une valeur par défaut.

## Testez-vous

**Question 1 :** on lance `python outil.py a b`. Que vaut `sys.argv` ?

> **Réponse**
>
> `['outil.py', 'a', 'b']` : le nom du programme d'abord, puis les arguments, tous en
> chaînes.

**Question 2 :** où écrire un message d'erreur, et avec quel code quitter ?

> **Réponse**
>
> Sur `stderr`, avec `print(..., file=sys.stderr)`, puis `sys.exit(1)` (ou un autre
> code différent de 0).

**Question 3 :** écrivez un analyseur `argparse` avec un argument `texte` et un drapeau
`--inverser`, puis testez-le avec `["radar", "--inverser"]` et `["python",
"--inverser"]`.

> **Réponse**
>
> ```python
> import argparse
>
> parser = argparse.ArgumentParser(prog="miroir")
> parser.add_argument("texte")
> parser.add_argument("--inverser", action="store_true")
>
> for essai in [["radar", "--inverser"], ["python", "--inverser"]]:
>     args = parser.parse_args(essai)
>     print(args.texte[::-1] if args.inverser else args.texte)
> ```
>
> ```sortie
> radar
> nohtyp
> ```

## En résumé

- `sys.argv` contient le nom du programme puis les arguments, tous en chaînes.
- `argparse` décrit arguments positionnels, options (`--fois`) et drapeaux
  (`store_true`), convertit avec `type=` et génère l'aide.
- `parser.parse_args(liste)` permet de tester sans terminal.
- `sys.exit(0)` signale un succès, un autre code un échec.
- Les résultats vont sur `stdout`, les messages d'erreur sur `stderr` ; `sys.stdin` lit
  l'entrée d'un tube.
- Rangez le programme dans `main(arguments=None)` et appelez `sys.exit(main())`.

Suite : [faire plusieurs choses à la fois](concurrence.md). Comparez avec
[la ligne de commande en Rust](../rust/ligne-de-commande.md).
