# Python : introduction et installation

Cette page présente Python, explique comment l'installer et vous fait écrire puis lancer votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est Python et à quoi il sert ;
> - vous saurez installer Python sur Windows, macOS ou Linux ;
> - vous saurez essayer une ligne de code dans la console `>>>` ;
> - vous saurez écrire un fichier `.py` et le lancer depuis le terminal.

## Qu'est-ce que Python ?

Python est un langage de programmation créé par Guido van Rossum, aux Pays-Bas, en 1991.
Son nom vient d'une émission comique anglaise, les *Monty Python*, pas du serpent.

Un **langage de programmation** est une langue précise, que l'ordinateur sait suivre.
Un [programme](../debuter/glossaire.md#programme) est un texte écrit dans cette langue :
une suite d'ordres, comme une recette de cuisine.

```python
print("Bonjour")
print("Bonjour")
print("Bonjour")
```

```sortie
Bonjour
Bonjour
Bonjour
```

`print` (« imprimer » en anglais) affiche un texte à l'écran. Python exécute les lignes
dans l'ordre, de haut en bas.

Python est un langage **interprété** : vous écrivez un fichier, puis vous le lancez
directement. Un programme appelé [interpréteur](../debuter/glossaire.md#interpreteur)
lit votre fichier et exécute les ordres un par un. Il n'y a pas d'étape de préparation
(de « compilation ») avant.

## À quoi sert Python ?

Python est l'un des langages les plus utilisés au monde. On le trouve presque partout :

- **les scripts** : renommer 500 fichiers, trier des photos, remplir un tableur ;
- **l'analyse de données** : statistiques, graphiques, tableaux de chiffres ;
- **l'intelligence artificielle** : la plupart des outils d'apprentissage automatique ;
- **le web** : des sites et des services en ligne (avec Django ou Flask) ;
- **l'enseignement** : c'est souvent le premier langage appris au lycée et à
  l'université.

Pourquoi ce succès ? Le code se lit presque comme de l'anglais. Il y a peu de symboles à
retenir. Et Python est livré avec une grande
[bibliothèque](../debuter/glossaire.md#bibliotheque) standard : des milliers d'outils
prêts à l'emploi, sans rien installer de plus.

```python
nombres = [4, 8, 15, 16, 23, 42]
print(sum(nombres))
print(max(nombres))
```

```sortie
108
42
```

Lisez ce code à voix haute : « la somme des nombres », « le maximum des nombres ».

> **Note :** ce site présente aussi d'autres langages : comparez avec
> [l'introduction à Ruby](../ruby/introduction.md) ou
> [l'introduction à Rust](../rust/introduction.md).

## Installer Python

Ce site utilise **Python 3.13**. Toute version à partir de 3.10 suffit pour presque tous
les exemples.

> **Attention :** il existe un ancien Python 2, abandonné depuis 2020. Installez
> toujours un Python **3**.

### Sur Windows

1. Allez sur le site officiel, **python.org**, rubrique *Downloads*.
2. Téléchargez l'installeur pour Windows et lancez-le.
3. **Cochez la case « Add python.exe to PATH »** en bas de la première fenêtre.
4. Cliquez sur *Install Now*.

La case « Add to PATH » est importante. Le *PATH* est la liste des dossiers où le
terminal cherche les programmes. Sans elle, le terminal ne trouvera pas Python.

Windows installe aussi le **lanceur** `py`. C'est un petit programme qui trouve le bon
Python pour vous. Sur Windows, vous pouvez donc taper `py` à la place de `python`.

```sh
py --version
py bonjour.py
```

### Sur macOS

macOS ne fournit pas toujours un Python récent. Deux choix :

- téléchargez l'installeur pour macOS sur **python.org**, puis lancez-le ;
- ou, si vous utilisez Homebrew (un gestionnaire de paquets pour Mac) :

```sh
brew install python
```

Sur macOS, la commande s'appelle `python3`, avec un 3 à la fin.

### Sur Linux

Python est presque toujours déjà installé. Sinon, utilisez le gestionnaire de paquets de
votre système : un programme qui installe les autres programmes pour vous.

```sh
sudo apt install python3        # Debian, Ubuntu
sudo dnf install python3        # Fedora
```

Là aussi, la commande s'appelle `python3`.

> **À retenir :** selon le système, la commande est `python3` (macOS, Linux), `python`
> ou `py` (Windows). Dans ce site, nous écrivons `python3`.

## Vérifier l'installation

Ouvrez un terminal et tapez la commande suivante. Si le terminal est nouveau pour vous,
lisez d'abord [le terminal](../debuter/terminal.md).

```sh
python3 --version
```

Vous devez voir un numéro de version :

```text
Python 3.13.0
```

Si le terminal répond « command not found » ou « n'est pas reconnu » (commande
introuvable), deux causes possibles :

- Python n'est pas installé, ou la case « Add to PATH » n'était pas cochée ;
- le terminal a été ouvert avant l'installation : fermez-le et rouvrez-le.

## Essayer Python dans la console

Tapez simplement `python3`, sans nom de fichier. Vous entrez dans la **console
interactive**, aussi appelée [REPL](../debuter/glossaire.md#repl). Vous tapez une ligne,
Python répond tout de suite. C'est comme une calculatrice qui comprend tout Python.

```text
$ python3
Python 3.13.0 (main, Oct  7 2024, 05:02:14) [GCC 13.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 2 + 3
5
>>> "salut".upper()
'SALUT'
>>> len("bonjour")
7
>>> exit()
```

- Le signe `>>>` est l'**invite** : Python attend votre ligne.
- La ligne en dessous est la réponse de Python. Pas besoin de `print` dans la console.
- `exit()` (ou Ctrl+D sur macOS et Linux, Ctrl+Z puis Entrée sur Windows) quitte la
  console.

> **Astuce :** la console est parfaite pour tester une petite idée sans créer de
> fichier. Gardez-en une ouverte pendant que vous lisez ce site.

## Votre premier programme

La console oublie tout quand vous la quittez. Pour garder un programme, écrivez-le dans
un **fichier**. Ouvrez votre [éditeur de code](../debuter/editeur.md). Créez un fichier
`bonjour.py` avec ce contenu :

```python
print("Bonjour, le monde !")
```

```sortie
Bonjour, le monde !
```

- `print(...)` affiche ce qui est entre les parenthèses, puis passe à la ligne.
- Le texte est entre guillemets `"..."`. Sans guillemets, Python croirait que c'est un
  nom de code.
- Le fichier finit par `.py` : c'est ainsi qu'on reconnaît un programme Python.

Un programme peut avoir plusieurs lignes. Python les exécute dans l'ordre.

```python
print("Ligne 1")
print("Ligne 2")
print(2 + 3)
print("2 + 3")
```

```sortie
Ligne 1
Ligne 2
5
2 + 3
```

`print(2 + 3)` affiche `5` : sans guillemets, Python calcule. Avec guillemets, il
affiche le texte tel quel.

## Lancer un programme

Dans le terminal, placez-vous dans le dossier du fichier (avec la commande `cd`), puis
tapez :

```sh
python3 bonjour.py
```

Le programme s'exécute et affiche `Bonjour, le monde !`. Puis le terminal vous rend la
main.

> **À retenir :** un programme Python est un fichier `.py`. Vous le lancez avec
> `python3 nom_du_fichier.py` (ou `py nom_du_fichier.py` sur Windows).

Le guide [le terminal](../debuter/terminal.md) explique comment ouvrir un terminal et
changer de dossier.

### L'indentation compte

En Python, les espaces au début d'une ligne (l'**indentation**) ont un sens. Ils
montrent quelles lignes vont ensemble, comme les retraits d'un plan de dissertation.
Vous le verrez dans [les conditions et les boucles](conditions-boucles.md).

```python
for i in range(2):
    print("dans la boucle")
print("après la boucle")
```

```sortie
dans la boucle
dans la boucle
après la boucle
```

Pour l'instant, retenez une règle : **ne mettez pas d'espace au début d'une ligne** sans
raison.

## Lire un message d'erreur

Quand Python ne comprend pas votre code, il s'arrête et affiche une
[erreur](../debuter/glossaire.md#erreur). Ce n'est pas grave : le message vous aide.

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    Print("Bonjour")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Lisez-le **de bas en haut** :

- la dernière ligne donne le type d'erreur (`NameError`) et la cause (« le nom `Print`
  n'est pas défini ») ;
- au-dessus, la ligne fautive, avec des `^^^^^` sous le morceau en cause ;
- encore au-dessus, le fichier et le numéro de ligne (`line 1`).

Le mot *Traceback* veut dire « retracer le chemin » : Python montre où il était quand
l'erreur est arrivée. Les messages sont en anglais. Quelques mots reviennent souvent :
*not defined* (non défini), *invalid* (invalide), *expected* (j'attendais), *missing*
(il manque), *Did you mean* (vouliez-vous dire).

## Erreurs fréquentes

### Une majuscule à print

```python norun
Print("Bonjour")
```

```text
Traceback (most recent call last):
  File "/home/vous/main.py", line 1, in <module>
    Print("Bonjour")
    ^^^^^
NameError: name 'Print' is not defined. Did you mean: 'print'?
```

Python fait la différence entre majuscules et minuscules. `Print` et `print` sont deux
noms différents. Python propose même la bonne orthographe. Corrigez en
`print("Bonjour")`.

### Oublier les parenthèses

```python norun
print "Bonjour"
```

```text
  File "/home/vous/main.py", line 1
    print "Bonjour"
    ^^^^^^^^^^^^^^^
SyntaxError: Missing parentheses in call to 'print'. Did you mean print(...)?
```

C'était la façon d'écrire de Python 2. En Python 3, `print` est une
[fonction](../debuter/glossaire.md#fonction) : il faut des parenthèses.
Écrivez `print("Bonjour")`.

### Oublier le guillemet de fin

```python norun
print("Bonjour)
```

```text
  File "/home/vous/main.py", line 1
    print("Bonjour)
          ^
SyntaxError: unterminated string literal (detected at line 1)
```

Le texte n'est jamais fermé (*unterminated string*). Ajoutez le guillemet manquant :
`print("Bonjour")`.

### Un espace en trop au début

```python norun
print("Ligne 1")
  print("Ligne 2")
```

```text
  File "/home/vous/main.py", line 2
    print("Ligne 2")
IndentationError: unexpected indent
```

La ligne 2 commence par des espaces sans raison (*unexpected indent* : « retrait
inattendu »). Supprimez-les.

## Testez-vous

**Question 1 :** quelle commande vérifie que Python est installé ?

> **Réponse**
>
> `python3 --version` (ou `py --version` sur Windows). Elle affiche le numéro de
> version.

**Question 2 :** qu'affiche ce programme ?

```python norun
print("4 * 2")
print(4 * 2)
```

> **Réponse**
>
> `4 * 2`, puis `8`. Entre guillemets, c'est un texte affiché tel quel. Sans
> guillemets, Python calcule.

**Question 3 :** vous avez écrit `jeu.py`. Comment le lancer ?

> **Réponse**
>
> Placez-vous dans son dossier avec `cd`, puis tapez `python3 jeu.py`.

## En résumé

- Python est un langage interprété, lisible, utilisé pour les scripts, les données,
  l'IA et le web.
- Installez-le depuis python.org (cochez « Add to PATH » sur Windows) ou avec votre
  gestionnaire de paquets.
- `python3 --version` vérifie l'installation ; `python3` seul ouvre la console `>>>`.
- Un programme est un fichier `.py`, lancé avec `python3 nom_du_fichier.py`.
- `print(...)` affiche une valeur et passe à la ligne.
- Un message d'erreur se lit de bas en haut : type, cause, puis ligne fautive.

Suite : [premiers pas](premiers-pas.md).
