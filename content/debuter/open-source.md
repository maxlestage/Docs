# Contribuer à un projet open source

Un projet open source est un logiciel dont le code est ouvert à tous : cette page explique ce que cela veut dire et vous guide, pas à pas, jusqu'à votre première contribution sur GitHub.

> **Objectifs :**
>
> - comprendre ce qu'est l'open source et ce que dit une licence ;
> - savoir comment aider un projet, même en débutant ;
> - lire un projet : README, CONTRIBUTING, issues ;
> - faire une contribution complète : fork, clone, branche, commit, push, pull request ;
> - répondre poliment aux remarques de relecture.

## Qu'est-ce que l'open source ?

Imaginez une recette de cuisine affichée dans la vitrine d'une boulangerie.
Tout le monde peut la lire, la refaire chez soi, et même proposer une
amélioration au boulanger.

Un logiciel **open source** (en français, « à code source ouvert »), c'est
pareil :

- son code est **public** : tout le monde peut le lire ;
- tout le monde peut **l'utiliser** et **le modifier** ;
- tout le monde peut **proposer des changements** au projet.

Vous en utilisez déjà sans le savoir. Python, Node.js, Go, Ruby, Rust,
Git, Firefox et VLC sont open source.

> **À retenir :** open source veut dire « le code est ouvert ». Chacun
> peut le lire, l'utiliser et proposer de l'améliorer.

## Les licences, en mots simples

Une **licence** est un petit texte, en général dans un fichier `LICENSE`.
Il dit ce que vous avez le droit de faire avec le code. C'est le
« mode d'emploi juridique » du projet.

Deux licences reviennent très souvent :

| Licence | En une phrase | Ce qu'elle demande |
|---|---|---|
| **MIT** | « Faites ce que vous voulez avec ce code. » | Garder le nom de l'auteur et le texte de la licence. |
| **GPL** | « Vous pouvez tout faire, mais partagez à votre tour. » | Si vous distribuez une version modifiée, son code doit rester ouvert, sous GPL. |

Une image pour la GPL : on vous prête un livre de recettes, à condition
que vos nouvelles recettes soient prêtées de la même façon.

Il en existe d'autres (Apache, BSD, MPL…). Pour contribuer, vous n'avez pas
besoin de les connaître par cœur.

> **Attention :** un code **sans licence** n'est pas libre, même s'il est
> public sur GitHub. Par défaut, l'auteur garde tous les droits.

## Pourquoi contribuer, même en débutant ?

Vous pensez peut-être : « Je ne suis pas assez bon. » C'est faux. Un projet
a besoin de beaucoup de choses qui ne sont pas du code compliqué :

- **corriger une faute** dans la documentation ;
- **rendre une explication plus claire**, justement parce que vous ne l'avez
  pas comprise du premier coup ;
- **traduire** une page en français ;
- **signaler un [bug](glossaire.md#bug)** proprement, dans une issue ;
- **ajouter un exemple** ou un [test](glossaire.md#test).

Et vous y gagnez :

- vous lisez du vrai code, écrit par d'autres ;
- vous apprenez à travailler en équipe, avec [Git](glossaire.md#git) ;
- vous recevez des conseils de gens expérimentés ;
- vos contributions sont visibles sur votre profil GitHub.

> **À retenir :** une faute corrigée dans une documentation est une vraie
> contribution. Tout le monde commence par là.

## Lire un projet avant d'y toucher

Un projet sur GitHub, c'est un [dépôt](glossaire.md#depot). Avant de
changer quoi que ce soit, lisez trois choses.

### Le README

Le fichier `README.md` s'affiche sur la page d'accueil du dépôt. Il dit :

- à quoi sert le projet ;
- comment l'installer et le lancer.

### Le CONTRIBUTING

Le fichier `CONTRIBUTING.md` est le **règlement intérieur** du projet. Il
dit comment proposer un changement : comment nommer la branche, comment
écrire le message de commit, quels tests lancer.

Lisez-le **en entier**. Chaque projet a ses habitudes.

### Les issues

Une **issue** (en français, « sujet » ou « ticket ») est une page de
discussion sur GitHub : un bug signalé, une idée, une question. Elles sont
dans l'onglet **Issues** du dépôt.

Les projets accueillants mettent une **étiquette** (un *label*) sur les
tâches faciles :

```text
good first issue      ← une bonne première tâche
help wanted           ← le projet cherche de l'aide
documentation         ← la tâche concerne la documentation
```

Avant de commencer une issue, écrivez un message dessous :

```text
Bonjour ! Je débute et j'aimerais m'occuper de cette issue.
Est-ce qu'elle est encore disponible ?
```

Ainsi, deux personnes ne font pas le même travail.

> **Astuce :** sur GitHub, cherchez `label:"good first issue"` dans la barre
> de recherche des issues pour filtrer les tâches faciles.

## Le chemin d'une contribution

Voici tout le voyage, avant de le faire pas à pas. On utilise les mots de
GitHub, que l'on explique juste après.

```text
  LE PROJET (github.com/projet/outil)
      │                         ▲
      │ 1. fork                 │ 6. pull request
      ▼                         │
  VOTRE COPIE (github.com/alice/outil)
      │                         ▲
      │ 2. clone                │ 5. push
      ▼                         │
  VOTRE ORDINATEUR : 3. branche → 4. commit
```

| Étape | Mot | En clair |
|---|---|---|
| 1 | fork | Faire **votre copie** du projet, sur GitHub. |
| 2 | clone | Télécharger votre copie sur votre ordinateur. |
| 3 | branche | Ouvrir un **brouillon séparé** pour votre changement. |
| 4 | commit | Enregistrer votre changement (voir [Git](git.md)). |
| 5 | push | Envoyer le brouillon sur votre copie GitHub. |
| 6 | pull request | **Demander** au projet d'accepter votre changement. |

Dans les exemples, vous vous appelez `alice` et le projet s'appelle
`projet/outil`.

## Étape 1 : faire un fork

Vous n'avez pas le droit d'écrire directement dans le dépôt du projet.
Alors vous en faites une copie à vous : un **fork**.

1. Ouvrez la page du projet sur GitHub.
2. Cliquez sur le bouton **Fork**, en haut à droite.
3. Cliquez sur **Create fork**.

Vous avez maintenant `github.com/alice/outil`. Cette copie vous appartient.

## Étape 2 : cloner votre fork

Téléchargez **votre** copie (pas celle du projet) sur votre ordinateur,
dans le [terminal](terminal.md) :

```sh norun
git clone https://github.com/alice/outil.git
cd outil
```

```text
Cloning into 'outil'...
```

Puis notez l'adresse du projet d'origine. Par habitude, on l'appelle
`upstream` (« en amont ») :

```sh norun
git remote add upstream https://github.com/projet/outil.git
git remote -v
```

```text
origin	https://github.com/alice/outil.git (fetch)
origin	https://github.com/alice/outil.git (push)
upstream	https://github.com/projet/outil.git (fetch)
upstream	https://github.com/projet/outil.git (push)
```

- `origin`, c'est **votre** copie.
- `upstream`, c'est **le projet**.

## Étape 3 : créer une branche

Une **branche** est un brouillon séparé. La branche `main` reste propre ;
votre travail va sur une nouvelle branche.

Donnez-lui un nom qui dit ce que vous faites :

```sh norun
git checkout -b corrige-faute-readme
```

```text
Switched to a new branch 'corrige-faute-readme'
```

> **À retenir :** une contribution = une branche. Ne travaillez jamais
> directement sur `main`.

## Étape 4 : modifier et faire un commit

Ouvrez le fichier dans votre [éditeur](editeur.md), faites **un seul**
petit changement, enregistrez. Puis :

```sh norun
git status
git add README.md
git commit -m "Corrige une faute dans le README"
```

```text
[corrige-faute-readme 3f2a1b7] Corrige une faute dans le README
 1 file changed, 1 insertion(+), 1 deletion(-)
```

Si le projet a des tests, lancez-les maintenant. Le CONTRIBUTING dit
comment.

## Étape 5 : pousser la branche

Envoyez votre branche sur **votre** fork, `origin` :

```sh norun
git push -u origin corrige-faute-readme
```

GitHub répond avec un lien tout prêt :

```text
remote:
remote: Create a pull request for 'corrige-faute-readme' on GitHub by visiting:
remote:      https://github.com/alice/outil/pull/new/corrige-faute-readme
remote:
To https://github.com/alice/outil.git
 * [new branch]      corrige-faute-readme -> corrige-faute-readme
branch 'corrige-faute-readme' set up to track 'origin/corrige-faute-readme'.
```

## Étape 6 : ouvrir la pull request

Une **pull request** (souvent abrégée **PR**) est une demande : « Voici
mon changement, voulez-vous l'ajouter au projet ? »

1. Ouvrez le lien affiché par `git push`, ou la page de votre fork.
2. Cliquez sur **Compare & pull request**.
3. Écrivez un titre clair et une courte description.
4. Cliquez sur **Create pull request**.

Une bonne description répond à trois questions :

```text
Titre : Corrige une faute dans le README

Quoi : « installé » était écrit « instalé » dans la section Installation.
Pourquoi : une faute gêne la lecture.
Lien : corrige l'issue #42.
```

> **Astuce :** écrire `corrige l'issue #42` (ou en anglais `Fixes #42`)
> relie la PR à l'issue. GitHub fermera l'issue quand la PR sera acceptée.

## Répondre à la relecture

Quelqu'un du projet va **relire** votre PR. C'est la *review*. Il peut :

- l'accepter tout de suite (on dit **merger**, « fusionner ») ;
- demander des changements ;
- poser une question.

Une demande de changement n'est **pas** un refus. C'est une étape normale,
même pour les experts.

Pour corriger, restez sur la même branche, modifiez, puis :

```sh norun
git add README.md
git commit -m "Applique la remarque de la relecture"
git push
```

La PR se met à jour toute seule. Pas besoin d'en ouvrir une nouvelle.

Puis répondez au commentaire :

```text
Merci pour la relecture ! C'est corrigé dans le dernier commit.
```

> **À retenir :** une remarque porte sur le code, pas sur vous. Corrigez,
> remerciez, poussez.

## Être poli et patient

Les projets open source sont souvent tenus par des bénévoles, sur leur
temps libre.

- Dites **bonjour** et **merci**.
- Soyez **patient** : une réponse peut prendre plusieurs jours, ou semaines.
- Après une semaine sans nouvelles, un petit message gentil suffit :
  « Bonjour, avez-vous eu le temps de regarder ? Merci ! »
- Si vous ne comprenez pas une remarque, **demandez**. C'est normal.
- Respectez le **code de conduite** (fichier `CODE_OF_CONDUCT.md`).

## Garder son fork à jour

Pendant que vous travaillez, le projet avance. Avant une nouvelle
contribution, récupérez ses changements :

```sh norun
git checkout main
git pull upstream main
git push origin main
```

Puis créez une **nouvelle** branche pour la contribution suivante.

## Idées de première contribution

Classées de la plus facile à la moins facile :

| Idée | Où chercher |
|---|---|
| Corriger une faute de frappe | README, documentation |
| Rendre une phrase plus claire | Une page que vous avez eu du mal à comprendre |
| Traduire une page en français | Un dossier `docs/fr` ou `translations` |
| Signaler un bug avec les étapes pour le reproduire | Onglet Issues, bouton **New issue** |
| Ajouter un exemple à la documentation | Une fonction sans exemple |
| Résoudre une issue `good first issue` | Onglet Issues, filtre par étiquette |

> **Essayez :** choisissez un outil que vous utilisez. Ouvrez son dépôt,
> lisez le README et cherchez une issue `good first issue`. Ne faites rien
> d'autre aujourd'hui : juste lire.

## Erreurs fréquentes

### Travailler directement sur main

Vous modifiez le code sans créer de branche, puis vous voulez faire une
deuxième PR. Les deux changements se mélangent dans la même PR.

```sh norun
git checkout main
git commit -am "Corrige une faute"
git commit -am "Ajoute une traduction"
```

Correction : avant **chaque** contribution, partez de `main` à jour et
créez une branche :

```sh norun
git checkout main
git checkout -b ajoute-traduction
```

### Pousser une nouvelle branche sans dire où

```sh norun
git checkout -b corrige-faute
git push
```

```text
fatal: The current branch corrige-faute has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream origin corrige-faute

To have this happen automatically for branches without a tracking
upstream, see 'push.autoSetupRemote' in 'git help config'.
```

Git ne sait pas encore où envoyer cette branche. Faites ce qu'il dit :
`git push -u origin corrige-faute` (`-u` est la forme courte de
`--set-upstream`). Ensuite, `git push` suffit.

### Un git push refusé

Le dépôt en ligne contient des commits que vous n'avez pas sur votre
ordinateur (par exemple, vous avez modifié un fichier directement sur
GitHub).

```sh norun
git push origin main
```

```text
To https://github.com/alice/outil.git
 ! [rejected]        main -> main (fetch first)
error: failed to push some refs to 'https://github.com/alice/outil.git'
hint: Updates were rejected because the remote contains work that you do not
hint: have locally. This is usually caused by another repository pushing to
hint: the same ref. If you want to integrate the remote changes, use
hint: 'git pull' before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

Récupérez d'abord les commits en ligne avec `git pull`, puis refaites
`git push`.

### Une PR énorme, sans avoir lu le CONTRIBUTING

Vous changez 40 fichiers, le style du code et trois fonctions d'un coup.
Personne n'ose relire : la PR reste ouverte des mois, ou est fermée.

Correction :

- lisez le `CONTRIBUTING.md` avant de commencer ;
- **un** changement par PR, le plus petit possible ;
- pour une grosse idée, ouvrez d'abord une issue pour en discuter.

## Testez-vous

**1.** Quelle est la différence entre un fork et un clone ?

> **Réponse**
>
> Un fork est votre copie du projet **sur GitHub**. Un clone est une copie
> **sur votre ordinateur**. On fait d'abord le fork, puis on clone ce fork.

**2.** Un relecteur demande un changement dans votre PR. Devez-vous ouvrir
une nouvelle PR ?

> **Réponse**
>
> Non. Modifiez sur la même branche, faites un commit et `git push` : la PR
> se met à jour toute seule. Puis remerciez et répondez au commentaire.

**3.** Vous débutez et ne savez pas quoi faire. Citez deux contributions
possibles.

> **Réponse**
>
> Corriger une faute dans la documentation, traduire une page, signaler un
> bug avec les étapes pour le reproduire, ou prendre une issue marquée
> `good first issue`.

## En résumé

- Open source : le code est public, chacun peut le lire, l'utiliser et proposer des changements.
- La licence dit ce qui est permis : MIT laisse tout faire, GPL demande de partager à son tour.
- Lisez le README, le CONTRIBUTING et les issues `good first issue` avant de commencer.
- Le chemin : fork, clone, branche, commit, push, pull request.
- Une remarque de relecture est normale : corrigez sur la même branche, poussez, remerciez.
- Petites PR, une branche par contribution, et beaucoup de politesse.

Suite : [Les métiers du développement](metiers.md)
