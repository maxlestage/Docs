# Installer un éditeur de code

Un éditeur de code est le logiciel dans lequel vous écrivez vos programmes : cette page vous aide à installer VS Code, à créer votre premier fichier et à ajouter les bonnes extensions.

## Un éditeur, pas un traitement de texte

Word ou LibreOffice servent à écrire des lettres. Ils ajoutent des polices,
des couleurs, des marges. Pour l'ordinateur, tout cela est du bruit : un
programme doit être du **texte brut**.

Un **éditeur de code** écrit du texte brut, et il vous aide :

- il **colore** le code pour le rendre lisible ;
- il **signale les erreurs** pendant que vous tapez ;
- il **complète** les mots que vous commencez ;
- il contient un **terminal** pour lancer vos programmes.

> **Attention :** n'écrivez jamais de code dans Word. Les guillemets et les
> espaces y sont transformés, et le programme ne marche plus.

## Installer VS Code

Cette page utilise **Visual Studio Code** (VS Code). Il est gratuit, existe
sur tous les systèmes, et convient aux trois langages.

### Sous Windows

Allez sur le site `code.visualstudio.com`, cliquez sur **Download for
Windows**, puis ouvrez le fichier téléchargé. Acceptez les choix proposés.
Cochez **Ajouter à PATH** si la case apparaît.

### Sous macOS

Téléchargez la version macOS sur le même site. Ouvrez le fichier `.zip`,
puis glissez **Visual Studio Code** dans le dossier **Applications**.

### Sous Linux

Téléchargez le paquet `.deb` (Ubuntu, Debian) ou `.rpm` (Fedora) sur le
site, et ouvrez-le. Vous pouvez aussi passer par la logithèque de votre
système : cherchez **Visual Studio Code**.

## Ouvrir un dossier

Un programme vit dans un **dossier**. Créez un dossier `code` (voir
[le terminal en 10 minutes](terminal.md#creer-un-dossier-mkdir)), puis, dans
VS Code : menu **File › Open Folder…** (Fichier › Ouvrir le dossier), et
choisissez-le.

Le dossier s'affiche à gauche, dans l'**explorateur**. Tous vos fichiers y
apparaîtront.

> **Astuce :** ouvrez toujours un dossier, pas un fichier seul. L'éditeur
> comprend mieux votre projet, et le terminal démarre au bon endroit.

## Créer un fichier

Dans l'explorateur, cliquez sur l'icône **Nouveau fichier** et tapez son
nom, par exemple `bonjour.rb`. Écrivez dedans :

```ruby
puts "Bonjour !"
```

```sortie
Bonjour !
```

### Les noms de fichiers

L'**extension**, la fin du nom, indique le langage :

| Langage | Extension | Exemple       |
|---------|-----------|---------------|
| Ruby    | `.rb`     | `bonjour.rb`  |
| Rust    | `.rs`     | `main.rs`     |
| Rythme  | `.ry`     | `bonjour.ry`  |

Choisissez des noms simples : en minuscules, sans espaces ni accents. Pour
séparer les mots, utilisez `_` : `calcul_moyenne.rb`.

## Enregistrer

Un fichier non enregistré n'existe pas pour le terminal. Enregistrez avec
**Ctrl + S** (Windows, Linux) ou **Cmd + S** (macOS).

Un point blanc sur l'onglet du fichier signale des changements non
enregistrés.

### L'enregistrement automatique

Pour ne plus y penser, activez le menu **File › Auto Save**. VS Code
enregistre alors tout seul, quelques instants après chaque modification.

> **À retenir :** si votre programme ne voit pas vos changements, vérifiez
> d'abord que le fichier est enregistré.

## Le terminal intégré

VS Code contient un terminal. Ouvrez-le avec le menu **Terminal › New
Terminal**, ou avec **Ctrl + `** (la touche de l'accent grave).

Il s'ouvre en bas de la fenêtre, déjà placé dans votre dossier. Lancez
votre programme :

```sh
ruby bonjour.rb
```

```text
Bonjour !
```

## Les extensions utiles

Une **extension** ajoute des fonctions à VS Code. Cliquez sur l'icône des
**Extensions** à gauche (quatre petits carrés), cherchez un nom et cliquez
sur **Install**.

- **Ruby LSP** (de Shopify) : couleurs, complétion et erreurs pour Ruby.
- **rust-analyzer** : l'aide indispensable pour Rust. Il montre les types
  et les erreurs avant même de compiler.
- **Rythme** : couleurs, aide au survol, erreurs expliquées en français et
  un bouton **▶ Lancer** au-dessus du code. Installez d'abord Rythme lui-même
  (voir l'[introduction à Rythme](../rythme/introduction.md)).

> **Note :** si l'extension Rythme n'apparaît pas encore dans la recherche,
> le dépôt de Rythme explique comment l'installer depuis son dossier
> `editors/vscode`.

## D'autres éditeurs

VS Code n'est pas le seul choix. En voici trois autres :

- **Zed** : très rapide et moderne, avec une extension pour Rythme.
- **Sublime Text** : léger et rapide, payant après une période d'essai.
- **Neovim** : s'utilise entièrement au clavier, dans le terminal ; puissant,
  mais long à apprendre.

Pour débuter, restez sur VS Code : vous changerez plus tard si vous le
souhaitez.

## En résumé

- Un éditeur de code écrit du texte brut et vous aide à le lire et à le corriger.
- Installez VS Code, puis ouvrez un dossier avec *File › Open Folder*.
- L'extension du fichier donne le langage : `.rb`, `.rs` ou `.ry`.
- Enregistrez avec Ctrl + S (Cmd + S sur macOS), ou activez *Auto Save*.
- Ajoutez les extensions Ruby LSP, rust-analyzer et Rythme.
