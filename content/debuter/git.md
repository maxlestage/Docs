# Sauvegarder son travail avec Git

Git garde l'historique de votre code et vous permet de revenir en arrière : cette page vous apprend à l'installer, à enregistrer vos changements et à les envoyer sur GitHub.

> **Objectifs :**
>
> - comprendre à quoi sert Git ;
> - créer un dépôt avec `git init` ;
> - enregistrer une version avec `git add` et `git commit` ;
> - voir l'état et l'historique avec `git status` et `git log` ;
> - ignorer des fichiers avec `.gitignore` ;
> - envoyer son code sur GitHub et le récupérer ailleurs.

## Pourquoi Git

Vous avez peut-être déjà vu des dossiers comme ceux-ci :

```text
projet.rb
projet_v2.rb
projet_v2_final.rb
projet_v2_final_VRAIMENT.rb
```

**Git** fait ce travail pour vous, proprement. C'est un logiciel de
**gestion de versions** :

- il garde **chaque version** enregistrée de votre code, avec la date et un
  message ;
- il permet de **revenir en arrière** si vous avez tout cassé ;
- il permet d'envoyer votre code **sur Internet** (GitHub), pour le
  sauvegarder, le montrer ou travailler à plusieurs.

Un dossier suivi par Git s'appelle un **dépôt** (en anglais, *repository*,
souvent abrégé *repo*). Une version enregistrée s'appelle un **commit**.

> **À retenir :** un commit est une photo de votre projet à un moment donné.
> Git garde l'album complet.

## Installer Git

### Sous Windows

Téléchargez Git sur le site `git-scm.com` et lancez l'installation.
Acceptez les choix proposés.

### Sous macOS

Tapez `git --version` dans le terminal. Si Git n'est pas installé, macOS
propose de l'installer : acceptez.

### Sous Linux

Installez le paquet `git` avec le gestionnaire de votre système, par
exemple sous Ubuntu ou Debian :

```sh
sudo apt install git
```

### Vérifier et se présenter

Vérifiez l'installation :

```sh
git --version
```

```text
git version 2.43.0
```

Puis dites à Git qui vous êtes, une fois pour toutes. Ce nom et cette
adresse apparaîtront dans chaque commit :

```sh
git config --global user.name "Alice Martin"
git config --global user.email "alice@example.com"
```

## Créer un dépôt : git init

Placez-vous dans le dossier de votre projet (voir
[le terminal](terminal.md)), puis tapez :

```sh
cd code
git init
```

```text
Initialized empty Git repository in /home/alice/code/.git/
```

Git a créé un dossier caché `.git`. C'est là qu'il range tout l'historique.
N'y touchez pas.

## Voir l'état : git status

`git status` dit ce qui a changé depuis le dernier commit. Créez un fichier
`bonjour.rb`, puis :

```sh
git status
```

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	bonjour.rb

nothing added to commit but untracked files present (use "git add" to track)
```

*Untracked files* : Git voit le fichier, mais ne le suit pas encore.

> **Astuce :** tapez `git status` avant et après chaque commande. Il dit
> toujours où vous en êtes, et souvent quoi faire ensuite.

## Préparer : git add

`git add` choisit les fichiers à mettre dans le prochain commit. C'est comme
mettre des objets dans un carton avant de le fermer.

```sh
git add bonjour.rb
```

Pour ajouter tous les fichiers modifiés du dossier :

```sh
git add .
```

`git status` montre maintenant le fichier prêt :

```text
Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
	new file:   bonjour.rb
```

## Enregistrer : git commit

`git commit` ferme le carton et l'étiquette. Le message, après `-m`, dit ce
que vous avez fait :

```sh
git commit -m "Premier programme"
```

```text
[main (root-commit) 499728c] Premier programme
 1 file changed, 1 insertion(+)
 create mode 100644 bonjour.rb
```

`499728c` est le début de l'**identifiant** du commit : son numéro unique.

Écrivez des messages courts et clairs, qui disent **ce qui change** :
« Ajoute le calcul de la moyenne », « Corrige la division par zéro ».
Évitez « modifs » ou « ça marche ».

## Voir l'historique : git log

`git log` liste les commits, du plus récent au plus ancien :

```sh
git log
```

```text
commit 499728c1890cc53a9d89212eaccef8917f0d2eda
Author: Alice <alice@example.com>
Date:   Sun Oct 18 19:55:35 2026 +0000

    Premier programme
```

Pour une ligne par commit :

```sh
git log --oneline
```

```text
499728c Premier programme
```

Pour quitter un long historique, appuyez sur **q**.

## Ignorer des fichiers : .gitignore

Certains fichiers ne doivent pas aller dans le dépôt : les fichiers créés
par le compilateur, les mots de passe, les fichiers du système. Écrivez
leurs noms dans un fichier `.gitignore`, à la racine du projet :

```text
# Ce que Rust compile (cargo build)
target/
# Les fichiers de macOS
.DS_Store
# Les secrets
.env
```

Git ne verra plus ces fichiers. Ajoutez le `.gitignore` lui-même au dépôt :

```sh
git add .gitignore
git commit -m "Ajoute le .gitignore"
```

> **Note :** `cargo new` crée déjà un `.gitignore` pour vos projets Rust.

> **Attention :** ne mettez jamais un mot de passe dans un commit. Même
> effacé plus tard, il reste dans l'historique.

## GitHub : sauvegarder en ligne

Votre dépôt est pour l'instant **sur votre ordinateur**. **GitHub** est un
site qui héberge des dépôts Git en ligne. Il en existe d'autres, comme
GitLab ou Codeberg ; ils marchent de la même façon.

### Créer un dépôt sur GitHub

1. Créez un compte gratuit sur `github.com`.
2. Cliquez sur **+** en haut à droite, puis sur **New repository**.
3. Donnez-lui un nom, par exemple `code`.
4. Laissez-le **vide** : ne cochez pas « Add a README ».
5. Cliquez sur **Create repository**.

GitHub affiche alors l'adresse du dépôt, par exemple
`https://github.com/alice/code.git`.

### Envoyer son code : git push

Dites à votre dépôt local où se trouve le dépôt en ligne. Le nom `origin`
est l'usage :

```sh
git remote add origin https://github.com/alice/code.git
```

Puis envoyez vos commits :

```sh
git push -u origin main
```

La première fois, GitHub vous demande de vous connecter. Ensuite, un simple
`git push` suffit. Rechargez la page GitHub : votre code est en ligne.

### Récupérer un dépôt : git clone

Sur un autre ordinateur, ou pour récupérer le projet de quelqu'un d'autre,
`git clone` télécharge tout le dépôt, avec son historique :

```sh
git clone https://github.com/alice/code.git
cd code
```

Si le dépôt a changé en ligne depuis, `git pull` récupère les nouveaux
commits.

## La routine de tous les jours

Une fois le dépôt créé, le travail quotidien tient en quelques commandes :

| Quand                        | Commande                    |
|------------------------------|-----------------------------|
| En arrivant                  | `git pull`                  |
| Pour voir où vous en êtes    | `git status`                |
| Après un petit pas qui marche | `git add .`                |
| Pour enregistrer ce pas      | `git commit -m "Message"`   |
| Pour voir l'historique       | `git log --oneline`         |
| En partant                   | `git push`                  |

```sh
git pull
git status
git add .
git commit -m "Ajoute la fonction moyenne"
git push
```

> **Astuce :** faites des petits commits, souvent : chaque fois que
> quelque chose marche. Si vous cassez tout ensuite, vous reviendrez à une
> version qui marchait.

## Erreurs fréquentes

### Lancer git en dehors d'un dépôt

```sh
git status
```

```text
fatal: not a git repository (or any of the parent directories): .git
```

Vous n'êtes pas dans le dossier du projet, ou vous n'avez pas fait
`git init`. Vérifiez avec `pwd`, puis faites `cd code` ou `git init`.

### Faire un commit sans git add

```sh
git commit -m "Premier programme"
```

```text
On branch main

Initial commit

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	bonjour.rb

nothing added to commit but untracked files present (use "git add" to track)
```

Le carton est vide. Faites d'abord `git add bonjour.rb` (ou `git add .`).

### Oublier de se présenter

```sh
git commit -m "Premier programme"
```

```text
Author identity unknown

*** Please tell me who you are.

Run

  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"

to set your account's default identity.
Omit --global to set the identity only in this repository.
```

Tapez les deux commandes `git config` avec votre nom et votre adresse, puis
refaites le commit.

### Pousser sans dépôt en ligne

```sh
git push
```

```text
fatal: No configured push destination.
Either specify the URL from the command-line or configure a remote repository using

    git remote add <name> <url>

and then push using the remote name

    git push <name>
```

Git ne sait pas où envoyer. Ajoutez l'adresse avec
`git remote add origin …`, puis `git push -u origin main`.

## Testez-vous

**1.** Quelle est la différence entre `git add` et `git commit` ?

> **Réponse**
>
> `git add` choisit les changements à enregistrer (on remplit le carton).
> `git commit` les enregistre pour de bon, avec un message (on ferme et on
> étiquette le carton).

**2.** Votre projet Rust contient un dossier `target/` très lourd. Comment
éviter de l'envoyer sur GitHub ?

> **Réponse**
>
> Écrivez `target/` dans le fichier `.gitignore`, à la racine du projet.

**3.** Vous changez d'ordinateur. Comment récupérer votre projet depuis
GitHub ?

> **Réponse**
>
> Avec `git clone` suivi de l'adresse du dépôt, par exemple
> `git clone https://github.com/alice/code.git`.

## En résumé

- Git garde chaque version de votre code ; un dossier suivi est un dépôt, une version est un commit.
- `git init` crée le dépôt, une seule fois.
- `git add` prépare, `git commit -m "…"` enregistre, `git status` et `git log` montrent où vous en êtes.
- `.gitignore` liste les fichiers à ne pas suivre.
- `git push` envoie sur GitHub, `git clone` récupère un dépôt, `git pull` le met à jour.
- Faites des petits commits, souvent.

Suite : [Comment marche le web](web.md)
