# TypeScript : introduction et installation

Cette page présente TypeScript, explique comment l'installer et vous fait écrire, vérifier puis lancer votre premier programme.

> **Objectifs :**
>
> - vous saurez dire ce qu'est TypeScript et ce qu'il ajoute à JavaScript ;
> - vous saurez installer Node.js et TypeScript, puis vérifier qu'ils marchent ;
> - vous saurez écrire un fichier `.ts`, le vérifier avec `tsc` et le lancer avec `node` ;
> - vous saurez lancer un fichier `.ts` directement, sans étape intermédiaire ;
> - vous saurez lire votre première erreur de type.

## Qu'est-ce que TypeScript ?

TypeScript est un langage de programmation créé par Microsoft en 2012. C'est
**JavaScript avec des types**.

- **JavaScript** est le langage des pages web. Il tourne dans tous les navigateurs,
  et aussi sur votre ordinateur grâce à Node.js.
- Un [type](../debuter/glossaire.md#type) est la *sorte* d'une valeur : un nombre, un
  texte, un vrai/faux…

En TypeScript, vous pouvez écrire la sorte de chaque valeur. Voici un premier exemple :

```typescript
const prenom: string = "Alice";
const age: number = 30;

console.log(prenom);
console.log(age + 1);
```

```sortie
Alice
31
```

- `const prenom` crée une [variable](../debuter/glossaire.md#variable) nommée `prenom`.
- `: string` est l'**annotation de type** : « cette variable contient un texte ».
- `: number` veut dire « cette variable contient un nombre ».
- `console.log(...)` affiche une valeur dans le terminal.

Sans les `: string` et `: number`, ce serait du JavaScript ordinaire. Tout le code
JavaScript est donc presque déjà du TypeScript : vous n'avez pas besoin de connaître
JavaScript pour commencer, ce parcours vous apprend les deux à la fois.

> **Note :** le parcours [JavaScript](../javascript/introduction.md) présente le même
> langage sans les types. Vous pouvez le lire plus tard, par curiosité.

## Pourquoi des types ?

Les types servent à trouver les [bugs](../debuter/glossaire.md#bug) **avant** de lancer
le programme.

Prenez une analogie : un correcteur d'orthographe souligne les fautes pendant que vous
écrivez, avant que la lettre parte. TypeScript fait la même chose avec votre code. Il
relit tout et souligne ce qui ne peut pas marcher.

Voici un programme JavaScript avec une faute. Il calcule un prix total :

```javascript norun
const prix = "10";
const total = prix * 2 + prix;
console.log(total);
```

```text
2010
```

On attendait `30`. Mais `prix` est un texte (`"10"`, entre guillemets). `prix * 2`
donne `20`, puis `20 + "10"` colle les deux textes : `"2010"`. JavaScript ne dit rien,
il affiche un résultat faux.

Avec TypeScript, on déclare que `prix` est un nombre. L'erreur saute aux yeux tout de
suite, avant toute exécution :

```typescript norun
const prix: number = "10";
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const prix: number = "10";
        ~~~~


Found 1 error in main.ts:1
```

Vous corrigez en `const prix: number = 10;` et le programme affiche `30`.

> **À retenir :** JavaScript découvre les erreurs en cours de route, parfois jamais.
> TypeScript les trouve en relisant le code, avant de le lancer.

Les types ont d'autres avantages :

- votre [éditeur de code](../debuter/editeur.md) sait ce que contient chaque variable :
  il vous propose les bonnes actions et complète les noms ;
- le code se documente lui-même : `age: number` dit clairement ce qu'on attend ;
- quand vous modifiez un programme, TypeScript vous montre tous les endroits à adapter.

## Comment ça marche : tsc et node

L'ordinateur ne sait pas exécuter TypeScript directement. Il passe par deux étapes :

1. **`tsc`** (*TypeScript compiler*) vérifie les types, puis traduit le fichier `.ts`
   en un fichier `.js` : il enlève simplement les annotations. C'est un
   [compilateur](../debuter/glossaire.md#compilateur).
2. **`node`** exécute le fichier `.js` obtenu.

```text
bonjour.ts  --(tsc : vérifie et traduit)-->  bonjour.js  --(node : exécute)-->  affichage
```

Les types n'existent donc que pendant la vérification. Au moment de l'exécution, ils
ont disparu : il ne reste que du JavaScript.

## Installer Node.js

TypeScript s'installe avec **npm**, le gestionnaire de paquets fourni avec Node.js.
Installez donc d'abord Node.js. Ce site utilise **Node 22** ; une version plus récente
marche aussi.

### Sur Windows et macOS

Téléchargez l'installeur « LTS » (la version stable) sur le site nodejs.org, puis
lancez-le. Gardez les options proposées par défaut.

### Sur Linux

Passez par le gestionnaire de paquets de votre système, ou par l'outil `nvm`, qui
installe la version de votre choix :

```sh
sudo apt install nodejs npm    # Debian, Ubuntu (version parfois ancienne)
nvm install 22                 # avec nvm : Node 22
```

### Vérifier Node.js

Ouvrez un terminal et tapez ces deux commandes. Si le terminal est nouveau pour vous,
lisez d'abord [le terminal](../debuter/terminal.md).

```sh
node --version
npm --version
```

Vous devez voir deux numéros, par exemple `v22.22.0` et `10.9.4`. Si le terminal répond
« command not found » (commande introuvable), fermez-le et rouvrez-le après
l'installation.

## Installer TypeScript

Une seule commande suffit :

```sh
npm install -g typescript
```

- `npm install` télécharge et installe un paquet ;
- `-g` (*global*) l'installe pour tout l'ordinateur : la commande `tsc` marchera dans
  n'importe quel dossier.

Vérifiez l'installation :

```sh
tsc --version
```

```text
Version 5.8.3
```

Ce site vérifie ses exemples avec TypeScript 5.8 ; une version plus récente marche
aussi.

> **Astuce :** sur Linux ou macOS, si `npm install -g` répond *permission denied*
> (permission refusée), installez Node.js avec `nvm` : il range tout dans votre dossier
> personnel, sans besoin de droits administrateur.

## Votre premier programme

Ouvrez votre [éditeur de code](../debuter/editeur.md). Créez un dossier `essais`, et
dedans un fichier `bonjour.ts` avec ce contenu :

```typescript
const prenom: string = "Alice";
console.log(`Bonjour, ${prenom} !`);
```

```sortie
Bonjour, Alice !
```

- Le texte entre accents graves `` ` `` peut contenir `${...}` : la valeur de
  `prenom` est insérée à cet endroit.
- Chaque instruction finit par un point-virgule `;`.

### Compiler avec tsc

Dans le terminal, placez-vous dans le dossier `essais` (avec `cd`), puis tapez :

```sh
tsc bonjour.ts
```

Si tout va bien, `tsc` n'affiche **rien** : pas de nouvelle, bonne nouvelle. Un fichier
`bonjour.js` est apparu à côté. Ouvrez-le : c'est le même programme, sans l'annotation
`: string`.

```javascript norun
"use strict";
const prenom = "Alice";
console.log(`Bonjour, ${prenom} !`);
```

La ligne `"use strict";` demande à JavaScript d'être plus strict ; vous pouvez
l'ignorer.

### Lancer avec node

```sh
node bonjour.js
```

```text
Bonjour, Alice !
```

> **À retenir :** `tsc fichier.ts` vérifie et produit `fichier.js` ;
> `node fichier.js` l'exécute.

## Lancer un fichier .ts directement

Compiler puis lancer, c'est deux commandes à chaque essai. Deux raccourcis existent.

### Avec Node 22

Node sait lui-même lancer un fichier `.ts` : il efface les annotations à la volée.

```sh
node --experimental-strip-types bonjour.ts
```

Depuis Node 22.18, l'option n'est même plus nécessaire : `node bonjour.ts` suffit.

### Avec tsx

`tsx` est un petit outil qui lance un fichier TypeScript en une commande. `npx` le
télécharge au premier usage :

```sh
npx tsx bonjour.ts
```

> **Attention :** ces deux raccourcis **ne vérifient pas les types**. Ils enlèvent les
> annotations et lancent le programme, même s'il contient une erreur de type. Lancez
> `tsc --noEmit bonjour.ts` pour vérifier sans créer de fichier `.js`.

## Essayer en ligne : le Playground

Le site **typescriptlang.org/play** (le « Playground », le terrain de jeu) permet
d'essayer TypeScript dans le navigateur, sans rien installer.

- À gauche, vous tapez votre code ; les erreurs sont soulignées en rouge.
- À droite, l'onglet « .JS » montre le JavaScript produit.
- Le bouton « Run » exécute le code ; l'onglet « Logs » montre ce qu'affiche
  `console.log`.

> **Astuce :** survolez une variable avec la souris : le Playground affiche son type.
> C'est le meilleur moyen de voir ce que TypeScript a compris.

## Lire une erreur de type

Quand `tsc` trouve un problème, il affiche une erreur. Prenons ce fichier, avec une
faute de frappe dans le nom de la variable :

```typescript norun
const prenom: string = "Alice";
console.log(prenon);
```

```text
main.ts:2:13 - error TS2552: Cannot find name 'prenon'. Did you mean 'prenom'?

2 console.log(prenon);
              ~~~~~~

  main.ts:1:7
    1 const prenom: string = "Alice";
            ~~~~~~
    'prenom' is declared here.


Found 1 error in main.ts:2
```

Lisez-la morceau par morceau :

- `main.ts:2:13` : le fichier, la ligne 2, la colonne 13 ;
- `error TS2552` : le numéro de l'erreur (utile pour chercher sur le web) ;
- `Cannot find name 'prenon'` : « je ne trouve pas le nom `prenon` » ;
- `Did you mean 'prenom'?` : « vouliez-vous dire `prenom` ? » ;
- les vagues `~~~` soulignent l'endroit exact ;
- la partie en retrait montre où `prenom` a été déclaré (*declared here*).

Les messages sont en anglais. Quelques mots reviennent souvent : *type* (type),
*assignable* (qu'on peut ranger dans), *property* (propriété), *argument* (argument),
*expected* (attendu).

## Erreurs fréquentes

### Lancer node sur le fichier .ts avec un ancien Node

```sh
node bonjour.ts
```

```text
bonjour.ts:1
const prenom: string = "Alice";
      ^^^^^^

SyntaxError: Missing initializer in const declaration
    at wrapSafe (node:internal/modules/cjs/loader:1638:18)
```

Le message continue avec d'autres lignes `at ...`, sans intérêt ici. Avant Node 22.18, `node` ne comprend pas les annotations comme `: string`. Compilez
d'abord (`tsc bonjour.ts`) puis lancez `node bonjour.js`. Ou utilisez
`node --experimental-strip-types bonjour.ts`.

### Croire que tsc a bloqué le programme

```typescript norun
const age: number = "trente";
console.log(age);
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const age: number = "trente";
        ~~~


Found 1 error in main.ts:1
```

Par défaut, `tsc` affiche l'erreur **et** crée quand même le fichier `.js`. Ne lancez
pas `node` tant que `tsc` signale des erreurs : corrigez d'abord, ici avec
`const age: number = 30;`.

### Oublier d'installer TypeScript

```sh
tsc bonjour.ts
```

```text
tsc: command not found
```

TypeScript n'est pas installé, ou le terminal a été ouvert avant l'installation.
Tapez `npm install -g typescript`, puis ouvrez un nouveau terminal.

## Testez-vous

**Question 1 :** que fait `tsc bonjour.ts` ?

> **Réponse**
>
> Il vérifie les types du fichier, puis crée `bonjour.js` : le même programme, sans
> les annotations. On le lance ensuite avec `node bonjour.js`.

**Question 2 :** pourquoi TypeScript est-il utile, alors que le programme final est du
JavaScript ?

> **Réponse**
>
> Parce qu'il trouve les erreurs **avant** l'exécution : un texte à la place d'un
> nombre, une faute de frappe dans un nom… Le programme lancé est du JavaScript, mais
> un JavaScript déjà relu.

**Question 3 :** `npx tsx jeu.ts` lance votre programme sans erreur. Est-il bien typé ?

> **Réponse**
>
> Pas forcément : `tsx` ne vérifie pas les types. Lancez `tsc --noEmit jeu.ts` pour en
> être sûr.

## En résumé

- TypeScript, c'est JavaScript avec des types : `const age: number = 30;`.
- Les types trouvent les erreurs avant l'exécution, comme un correcteur d'orthographe.
- On installe Node.js, puis `npm install -g typescript` ; `tsc --version` vérifie.
- `tsc bonjour.ts` vérifie et produit `bonjour.js`, lancé avec `node bonjour.js`.
- `node --experimental-strip-types` et `npx tsx` lancent un `.ts` directement, sans
  vérifier les types.
- Le Playground (typescriptlang.org/play) permet d'essayer sans rien installer.

Suite : [premiers pas](premiers-pas.md).
