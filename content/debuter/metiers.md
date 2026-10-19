# Les métiers du développement

Savoir programmer ouvre la porte à beaucoup de métiers différents : cette page décrit les principaux en mots simples, avec les langages de ce site qui vont avec et une idée de premier projet pour essayer.

> **Objectifs :**
>
> - découvrir dix métiers du développement et ce qu'on y fait dans une journée ;
> - savoir quels langages de ce site mènent à chaque métier ;
> - avoir une idée de premier projet pour chacun ;
> - savoir comment commencer : petits projets, GitHub, lecture de code ;
> - connaître les chemins de formation et les qualités qui comptent.

## Un seul savoir, beaucoup de métiers

Pensez à la cuisine. Un pâtissier, un chef de restaurant et un boulanger
savent tous cuisiner. Mais leur journée est très différente.

C'est pareil en programmation. Les bases sont les mêmes partout :
[variables](variables.md), [conditions](logique.md),
[boucles](boucles.md), [fonctions](fonctions.md). Ce qui change, c'est
**ce que vous construisez** et **pour qui**.

Pour chaque métier, vous trouverez :

- **la journée** : ce que vous faites, en mots simples ;
- **les langages** de ce site qui vont avec ;
- **un premier projet** pour essayer chez vous.

> **À retenir :** vous n'avez pas à choisir votre métier maintenant.
> Essayez plusieurs petits projets : vous verrez ce qui vous plaît.

## Développeur front-end

**Le front-end** est la partie d'un site que l'on **voit** : les pages,
les boutons, les menus. C'est la vitrine du magasin.

- **La journée** : transformer une maquette (un dessin de la page) en vraie
  page. Rendre un bouton cliquable. Vérifier que le site marche sur un
  téléphone. Corriger un menu qui s'affiche mal.
- **Les langages** : [JavaScript](../javascript/introduction.md), puis
  [TypeScript](../typescript/introduction.md), avec
  [HTML](glossaire.md#html) et CSS (voir [le web](web.md)).
- **Un premier projet** : une page « liste de courses » où l'on ajoute et
  coche des articles.

## Développeur back-end

**Le back-end** est la partie **cachée** : le [serveur](glossaire.md#serveur)
qui stocke les données et répond aux demandes. C'est la cuisine du
restaurant, que le client ne voit pas.

- **La journée** : écrire le code qui enregistre une commande. Créer une
  [API](glossaire.md#api) (un guichet où d'autres programmes posent leurs
  questions). Organiser les [bases de données](bases-de-donnees.md).
- **Les langages** : [Python](../python/introduction.md),
  [Go](../go/introduction.md), [Ruby](../ruby/introduction.md),
  JavaScript avec Node.js, ou [Rust](../rust/introduction.md).
- **Un premier projet** : un petit serveur qui renvoie une citation au
  hasard quand on visite une adresse.

## Développeur full-stack

**Full-stack** veut dire « toute la pile » : le front-end **et** le
back-end. C'est la personne qui tient à la fois la vitrine et la cuisine.

- **La journée** : ajouter une fonctionnalité de bout en bout. Par exemple,
  le bouton « J'aime » : le bouton dans la page, et l'enregistrement du
  « j'aime » sur le serveur.
- **Les langages** : JavaScript et TypeScript des deux côtés, ou
  JavaScript devant et Python, Ruby ou Go derrière.
- **Un premier projet** : un livre d'or. Les visiteurs écrivent un message
  dans une page ; le serveur le garde ; la page affiche tous les messages.

## Développeur mobile

Il crée les **applications** de votre téléphone.

- **La journée** : construire un écran de l'application. Tester sur
  plusieurs téléphones. Publier une mise à jour sur les magasins
  d'applications.
- **Les langages** : les langages de ce site servent surtout à apprendre
  les bases. Le métier utilise souvent Kotlin (Android) ou Swift (iPhone).
  Avec JavaScript ou TypeScript, des outils comme React Native créent des
  applications pour les deux.
- **Un premier projet** : un minuteur pour cuire les pâtes.

## Data et intelligence artificielle

On travaille avec des **données** : des tableaux de chiffres, des textes,
des images. On leur pose des questions. Parfois, on apprend à un
programme à reconnaître des choses (c'est l'IA).

- **La journée** : nettoyer un fichier de données plein de trous. Faire un
  graphique. Répondre à une question comme « Quel produit se vend le
  mieux le samedi ? ».
- **Les langages** : [Python](../python/introduction.md), de très loin,
  plus le langage SQL des [bases de données](bases-de-donnees.md).
- **Un premier projet** : lire un fichier de vos dépenses et calculer le
  total par catégorie.

## DevOps et cloud

**DevOps** mélange « développement » et « opérations ». Ces personnes font
en sorte que les programmes **tournent bien**, jour et nuit, sur des
serveurs loués en ligne (le **cloud**, « nuage »). Ce sont les gardiens
de la salle des machines.

- **La journée** : automatiser la mise en ligne d'un site. Surveiller les
  serveurs. Réagir quand un service tombe en panne.
- **Les langages** : [Go](../go/introduction.md) (beaucoup d'outils du
  cloud sont écrits en Go), [Python](../python/introduction.md), et le
  [terminal](terminal.md) tous les jours.
- **Un premier projet** : un script qui vérifie toutes les minutes qu'un
  site répond, et affiche une alerte sinon.

## Développeur embarqué

Le code **embarqué** tourne dans des objets : une machine à laver, une
voiture, un thermostat, une montre. La machine est petite, avec peu de
[mémoire](glossaire.md#memoire-vive).

- **La journée** : faire clignoter une lumière au bon moment. Lire un
  capteur de température. Chercher pourquoi l'appareil redémarre tout
  seul.
- **Les langages** : [Rust](../rust/introduction.md) (de plus en plus),
  et le C, très utilisé dans ce métier.
- **Un premier projet** : avec une petite carte électronique bon marché,
  allumer une lumière quand il fait sombre.

## Développeur de jeux vidéo

Il écrit les **règles** du jeu : comment le personnage saute, ce qui se
passe quand il touche un ennemi, comment le score monte.

- **La journée** : régler la hauteur d'un saut. Corriger un personnage qui
  passe à travers un mur. Rendre le jeu plus fluide.
- **Les langages** : les grands studios utilisent souvent C++ ou C#. Pour
  commencer, [JavaScript](../javascript/introduction.md) (jeux dans le
  navigateur), [Python](../python/introduction.md) ou
  [Rust](../rust/introduction.md) suffisent.
- **Un premier projet** : « Devine le nombre » dans le terminal, puis un
  petit jeu de serpent.

## Sécurité informatique

On **protège** les programmes et les données contre les attaques. C'est le
serrurier de l'informatique : il cherche les portes mal fermées avant les
cambrioleurs.

- **La journée** : relire du code pour trouver des failles. Tester un site
  comme le ferait un attaquant, avec permission. Expliquer à l'équipe
  comment corriger.
- **Les langages** : [Python](../python/introduction.md) pour les petits
  outils, et savoir **lire** beaucoup de langages. Lisez la page
  [sécurité](securite.md).
- **Un premier projet** : un programme qui dit si un mot de passe est
  faible ou solide.

## QA et test

**QA** veut dire *quality assurance*, « assurance qualité ». On vérifie que
le logiciel **fait ce qu'il promet**, avant les utilisateurs.

- **La journée** : écrire des [tests](glossaire.md#test) automatiques.
  Essayer le logiciel de toutes les façons, même bizarres. Écrire des
  rapports de [bugs](glossaire.md#bug) clairs.
- **Les langages** : [Python](../python/introduction.md),
  [JavaScript](../javascript/introduction.md) ou
  [TypeScript](../typescript/introduction.md), selon le projet testé.
- **Un premier projet** : écrire cinq tests pour une fonction qui calcule
  une moyenne. Que se passe-t-il avec une liste vide ?

## Le tableau récapitulatif

| Métier | Ce qu'on construit | Langages de ce site pour commencer |
|---|---|---|
| Front-end | Les pages que l'on voit | JavaScript, TypeScript |
| Back-end | Les serveurs et les données | Python, Go, Ruby, Rust, JavaScript |
| Full-stack | Les deux | JavaScript, TypeScript + un langage serveur |
| Mobile | Les applications du téléphone | JavaScript, TypeScript (puis Kotlin ou Swift) |
| Data / IA | Des réponses à partir de données | Python |
| DevOps / cloud | Des programmes qui tournent bien | Go, Python |
| Embarqué | Le code des objets | Rust (et le C) |
| Jeux vidéo | Les règles du jeu | JavaScript, Python, Rust |
| Sécurité | La protection | Python |
| QA / test | La vérification | Python, JavaScript, TypeScript |

Et [Rythme](../rythme/introduction.md) ? C'est un langage pour
**apprendre**. Il prépare à tous ces métiers, mais aucun ne l'utilise.

> **Essayez :** choisissez deux lignes du tableau qui vous attirent.
> Faites les deux premiers projets cette semaine, en une heure chacun.
> Lequel vous a donné envie de continuer ?

## Comment commencer

### Des petits projets, pas un gros

Un **portfolio**, c'est une vitrine de vos projets. Cinq petits projets
terminés valent mieux qu'un gros projet abandonné.

```text
Semaine 1   → un convertisseur de températures
Semaine 2   → une liste de tâches dans le terminal
Semaine 3   → « Devine le nombre »
Semaine 4   → une page web avec un bouton
Semaine 5   → refaire un des projets en mieux
```

Voir aussi [des petits projets, par niveau](apprendre.md#des-petits-projets-par-niveau).

### Tout mettre sur GitHub

Mettez chaque projet sur GitHub (voir [Git](git.md)), avec un `README.md`
qui dit en trois lignes :

- ce que fait le projet ;
- comment le lancer ;
- ce que vous avez appris.

Une personne qui recrute regarde souvent ces pages.

### Lire le code des autres

On apprend à écrire en lisant. Ouvrez un petit projet sur GitHub et lisez
un fichier. Puis essayez d'y [contribuer](open-source.md).

> **À retenir :** des projets petits, finis et visibles sur GitHub
> montrent ce que vous savez faire mieux qu'un long discours.

## Les chemins de formation

Il n'y a pas **un** bon chemin. Il y en a trois grands, et beaucoup de
gens les mélangent.

| Chemin | Comment | Points forts | Points d'attention |
|---|---|---|---|
| **École** (université, IUT, école d'ingénieurs) | Plusieurs années d'études | Bases solides, diplôme, stages | Long ; demande souvent de commencer jeune |
| **Reconversion** (formation intensive) | Quelques mois, souvent pour adultes | Rapide, encadré, en groupe | Très intense ; vérifiez le programme et les avis d'anciens élèves |
| **Autodidacte** (seul) | Livres, sites comme celui-ci, projets | Gratuit ou presque, à votre rythme | Il faut de la régularité ; personne ne vous guide |

Soyons honnêtes : aucune formation ne **garantit** un emploi. Méfiez-vous
de celles qui le promettent. Ce qui compte, c'est ce que vous savez
**faire**, et ce que vous pouvez **montrer**.

## Les qualités qui comptent vraiment

Programmer, c'est aussi travailler avec des gens. Ces qualités comptent
autant que le code :

- **Poser des questions.** Dire « je ne comprends pas » est une force.
  Voir [poser une bonne question](apprendre.md#poser-une-bonne-question).
- **Écrire clairement.** Un message de commit, une issue, une
  documentation : on écrit beaucoup.
- **Rester calme face à un bug.** Lire le message, chercher, essayer. Voir
  [lire un message d'erreur](lire-une-erreur.md) et [je suis bloqué](bloque.md).
- **Être curieux.** Les outils changent ; on apprend toute sa vie.
- **Accepter les remarques.** Une relecture de code aide, elle ne juge pas.

## Erreurs fréquentes

Ici, pas d'erreurs de code : des **idées fausses** qui découragent
beaucoup de débutants.

### « Il faut être fort en maths »

Faux pour la plupart des métiers. Le front-end, le back-end, le mobile ou
le test demandent surtout de la **logique** : découper un problème en
petites étapes. Les additions et les pourcentages suffisent souvent.

Les maths comptent plus dans quelques domaines : l'IA, les jeux en 3D,
certains outils de sécurité. Et elles s'apprennent aussi, petit à petit.

### « Il faut tout savoir avant de postuler »

Personne ne sait tout, même après vingt ans de métier. Une offre d'emploi
liste souvent beaucoup de choses : c'est une liste de souhaits, pas
d'obligations.

Correction : si vous avez les bases et quelques projets à montrer,
postulez. Vous apprendrez le reste en travaillant.

### « Il faut avoir commencé enfant »

Beaucoup de développeurs ont commencé à 25, 35 ou 45 ans, après un autre
métier. Cet autre métier est même un atout : vous connaissez déjà un
domaine (la santé, le commerce, l'enseignement…).

### « Il faut connaître le meilleur langage »

Il n'y a pas de meilleur langage, seulement des langages adaptés à un
travail. Voir [quel langage choisir](choisir.md). Après le premier, les
suivants s'apprennent bien plus vite.

## Testez-vous

**1.** Quelle est la différence entre le front-end et le back-end ?

> **Réponse**
>
> Le front-end est la partie que l'on voit (les pages, les boutons). Le
> back-end est la partie cachée (le serveur, les données). Le full-stack
> fait les deux.

**2.** Vous aimez les chiffres et les graphiques. Quel métier et quel
langage de ce site essayer en premier ?

> **Réponse**
>
> La data, avec Python. Premier projet : lire un fichier de dépenses et
> calculer le total par catégorie.

**3.** Vous n'êtes pas fort en maths et vous n'avez que trois petits
projets. Devez-vous attendre avant de chercher un stage ?

> **Réponse**
>
> Non. La plupart des métiers demandent surtout de la logique, et personne
> ne sait tout. Mettez vos projets sur GitHub avec un README clair, et
> postulez.

## En résumé

- Les bases de la programmation sont les mêmes pour tous les métiers ; ce qui change, c'est ce que l'on construit.
- Front-end (ce qu'on voit), back-end (ce qui est caché), full-stack (les deux), mais aussi mobile, data, DevOps, embarqué, jeux, sécurité et test.
- Pour commencer : des petits projets finis, sur GitHub, avec un README.
- École, reconversion ou autodidacte : aucun chemin ne garantit un emploi ; ce que vous savez montrer compte.
- Poser des questions et écrire clairement comptent autant que le code.

Suite : [le glossaire](glossaire.md), à garder sous la main.
