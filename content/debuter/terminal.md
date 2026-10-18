# Le terminal en 10 minutes

Le terminal est une fenêtre où l'on parle à l'ordinateur en tapant du texte : cette page vous apprend à l'ouvrir, à vous y déplacer et à y lancer vos programmes.

## Qu'est-ce que le terminal

D'habitude, vous utilisez l'ordinateur avec la souris : vous cliquez sur
des dossiers et des icônes.

Le **terminal** fait la même chose, mais avec le clavier. Vous tapez une
**commande**, vous appuyez sur Entrée, et l'ordinateur répond par du texte.

C'est là que vous lancerez vos programmes. Pas de panique : une dizaine de
commandes suffisent pour commencer.

## Ouvrir le terminal

### Sous Windows

Ouvrez le menu Démarrer, tapez **Terminal** et appuyez sur Entrée. Si rien
ne s'affiche, tapez **PowerShell**. Les deux conviennent.

### Sous macOS

Appuyez sur **Cmd + Espace**, tapez **Terminal** et appuyez sur Entrée.
L'application est aussi dans *Applications › Utilitaires*.

### Sous Linux

Le raccourci **Ctrl + Alt + T** marche sur la plupart des systèmes. Sinon,
cherchez **Terminal** dans la liste des applications.

## L'invite de commande

Une fois ouvert, le terminal affiche une ligne qui attend votre commande.
C'est l'**invite** (en anglais, *prompt*). Elle ressemble à ceci :

```text
alice@portable:~$
```

ou, sous Windows :

```text
PS C:\Users\Alice>
```

L'invite montre souvent le **dossier où vous êtes**. Dans cette page, les
commandes sont écrites sans l'invite : tapez seulement ce qui est dans le
bloc.

## Les commandes de base

### Où suis-je : pwd

`pwd` affiche le dossier courant (*print working directory*).

```sh
pwd
```

```text
/home/alice
```

### Que contient ce dossier : ls ou dir

`ls` liste les fichiers et les dossiers. Sous Windows, `dir` fait la même
chose (`ls` marche aussi dans PowerShell).

```sh
ls
```

```text
Bureau  Documents  Images  Téléchargements
```

### Entrer dans un dossier : cd

`cd` (*change directory*) vous déplace dans un dossier.

```sh
cd Documents
```

Pour remonter d'un niveau, vers le dossier parent, tapez `cd ..` :

```sh
cd ..
```

### Créer un dossier : mkdir

`mkdir` (*make directory*) crée un dossier. Créez-en un pour vos essais :

```sh
mkdir code
cd code
```

### Nettoyer l'écran : clear

`clear` efface l'écran. Sous Windows, `cls` fait la même chose.

## Les raccourcis qui font gagner du temps

- **Flèche du haut** : rappelle la commande précédente. Appuyez plusieurs
  fois pour remonter plus loin.
- **Tab** : complète un nom. Tapez `cd Doc` puis Tab : le terminal écrit
  `Documents` pour vous.
- **Ctrl + C** : arrête le programme en cours. Utile s'il ne s'arrête plus.

> **Astuce :** utilisez Tab tout le temps. C'est plus rapide, et cela évite
> les fautes de frappe dans les noms de fichiers.

> **Attention :** dans le terminal, Ctrl + C ne copie pas : il arrête. Pour
> copier, utilisez Ctrl + Shift + C (Linux), Cmd + C (macOS) ou la sélection
> à la souris (Windows).

## Lancer un programme

Placez-vous d'abord, avec `cd`, dans le dossier qui contient votre fichier.
Puis tapez le nom du langage suivi du nom du fichier.

Un programme Ruby :

```sh
ruby bonjour.rb
```

Un programme Rythme :

```sh
rythme bonjour.ry
```

Un programme Rust se range dans un **projet**. Dans le dossier du projet,
cette commande compile puis lance le programme :

```sh
cargo run
```

Chaque langage s'installe une fois. Voyez les pages d'introduction :
[Ruby](../ruby/introduction.md), [Rust](../rust/introduction.md) et
[Rythme](../rythme/introduction.md).

## Lire une erreur

Si quelque chose ne va pas, le terminal affiche un message. Par exemple, si
le fichier n'existe pas :

```text
ruby: No such file or directory -- bonjur.rb (LoadError)
```

Lisez-le calmement. Ici, le nom est mal écrit : `bonjur` au lieu de
`bonjour`. Les causes les plus fréquentes :

- une faute de frappe dans le nom ;
- vous n'êtes pas dans le bon dossier (vérifiez avec `pwd` et `ls`) ;
- le langage n'est pas installé : le terminal dit « command not found » ou
  « n'est pas reconnu ».

## Windows et macOS/Linux

La plupart des commandes sont identiques. Voici les différences :

| Action                    | macOS et Linux | Windows (PowerShell)  |
|---------------------------|----------------|-----------------------|
| Où suis-je ?              | `pwd`          | `pwd`                 |
| Lister les fichiers       | `ls`           | `dir` (ou `ls`)       |
| Entrer dans un dossier    | `cd dossier`   | `cd dossier`          |
| Remonter d'un niveau      | `cd ..`        | `cd ..`               |
| Créer un dossier          | `mkdir code`   | `mkdir code`          |
| Effacer l'écran           | `clear`        | `cls` (ou `clear`)    |
| Séparateur des chemins    | `/`            | `\`                   |

Sous Windows, un chemin s'écrit avec des barres obliques inversées :

```powershell
cd C:\Users\Alice\Documents\code
```

## En résumé

- Le terminal permet de parler à l'ordinateur en tapant des commandes.
- `pwd` dit où vous êtes, `ls` (ou `dir`) liste, `cd` se déplace, `mkdir` crée un dossier.
- La flèche du haut rappelle une commande, Tab complète, Ctrl + C arrête.
- On lance un programme avec `ruby fichier.rb`, `rythme fichier.ry` ou `cargo run`.
- Un message d'erreur se lit calmement : il dit presque toujours quoi corriger.
