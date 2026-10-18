# Configurer TypeScript (tsconfig)

Cette page explique comment le fichier `tsconfig.json` règle le compilateur TypeScript, quelles options comptent vraiment, et comment compiler ou seulement vérifier un projet.

> **Objectifs :**
>
> - créer un fichier `tsconfig.json` avec `tsc --init` ;
> - comprendre les options importantes : `strict`, `target`, `module`, `outDir`, `rootDir`… ;
> - compiler tout un projet avec une seule commande ;
> - recompiler automatiquement avec le mode « watch » ;
> - vérifier les types sans produire de fichiers avec `tsc --noEmit`.

## À quoi sert tsconfig.json ?

`tsc` est le [compilateur](../debuter/glossaire.md#compilateur) de TypeScript : il lit
vos fichiers `.ts`, vérifie les types, puis écrit des fichiers `.js` que Node ou le
navigateur peuvent exécuter.

Jusqu'ici, vous lui donniez un fichier à la fois. Dans un vrai projet, il y a des dizaines
de fichiers et beaucoup de réglages. Plutôt que de les taper à chaque fois, on les écrit
une fois pour toutes dans un fichier **`tsconfig.json`**, à la racine du projet.

C'est comme les réglages d'un appareil photo : vous les choisissez une fois, puis vous
appuyez simplement sur le bouton.

Quand ce fichier existe, la commande `tsc`, **sans nom de fichier**, compile tout le
projet avec ces réglages.

## Créer le fichier avec tsc --init

Dans le dossier du projet, lancez :

```sh
npx tsc --init
```

```text
Created a new tsconfig.json with:

  target: es2016
  module: commonjs
  strict: true
  esModuleInterop: true
  skipLibCheck: true
  forceConsistentCasingInFileNames: true


You can learn more at https://aka.ms/tsconfig
```

Le fichier créé contient des dizaines d'options en commentaire, chacune avec une courte
explication. Seules quelques-unes sont actives. C'est un bon point de départ, mais les
valeurs par défaut (`es2016`, `commonjs`) sont anciennes : nous allons les changer.

> **Note :** `npx tsc` lance le `tsc` installé dans le projet (`npm install --save-dev typescript`).
> Si TypeScript est installé pour tout l'ordinateur, `tsc` suffit.

## Un tsconfig.json réel

Voici un fichier complet pour un programme Node moderne. Le code source est dans
`src/`, le JavaScript produit va dans `dist/`.

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "rootDir": "src",
    "outDir": "dist",
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "esModuleInterop": true,
    "sourceMap": true,
    "skipLibCheck": true
  },
  "include": ["src"]
}
```

- `compilerOptions` regroupe les réglages du compilateur.
- `include` dit quels dossiers compiler : ici, seulement `src`.

Les guillemets sont obligatoires autour des noms d'options : c'est du
[JSON](../debuter/glossaire.md#json). TypeScript tolère toutefois les commentaires `//`
dans ce fichier.

## Les options importantes

| Option | Rôle | Valeur conseillée |
|--------|------|-------------------|
| `strict` | active toutes les vérifications strictes : `null` interdit sans le dire, paramètres sans type refusés… | `true`, toujours |
| `target` | la version de JavaScript produite : le compilateur traduit la syntaxe trop récente | `"es2022"` pour Node 18 et plus |
| `module` | la façon d'écrire les `import` et `export` dans le JavaScript produit | `"nodenext"` pour Node |
| `rootDir` | le dossier qui contient vos fichiers `.ts` | `"src"` |
| `outDir` | le dossier où écrire les fichiers `.js` | `"dist"` |
| `noUncheckedIndexedAccess` | `tableau[i]` a le type `T \| undefined` : la case peut ne pas exister | `true` |
| `esModuleInterop` | permet `import fs from "fs"` avec les vieux modules CommonJS | `true` |
| `sourceMap` | crée des fichiers `.js.map` qui relient le `.js` au `.ts` d'origine | `true` |
| `skipLibCheck` | ne revérifie pas les fichiers de types des bibliothèques (`.d.ts`) | `true`, c'est plus rapide |

Les sections suivantes détaillent les plus utiles.

### strict : le filet de sécurité

`strict: true` allume d'un coup une famille de vérifications. Les deux plus importantes :

- `strictNullChecks` : une valeur qui peut être `null` ou `undefined` doit être
  vérifiée avant usage ;
- `noImplicitAny` : un paramètre sans type est refusé, au lieu de devenir `any` en silence.

Tous les exemples de ce site sont vérifiés avec `--strict`. Commencez **toujours** un
nouveau projet avec `strict: true` : c'est beaucoup plus difficile de l'activer plus tard.

### target et module

`target` choisit la version de JavaScript écrite dans `dist/`. Avec `"es2022"`,
le code reste presque identique à votre TypeScript, types en moins. Avec une cible
ancienne, le compilateur réécrit les nouveautés avec de vieilles instructions.

`module` choisit le système de modules. Avec `"nodenext"`, TypeScript suit les règles de
Node : si `package.json` contient `"type": "module"`, les fichiers sont des modules ES,
avec `import` et `export`. Attention, dans ce mode, un `import` local doit finir par
`.js`, même si le fichier source est un `.ts` :

```typescript norun
import { moyenne } from "./calcul.js";
```

C'est le nom du fichier **produit** qui compte, puisque c'est lui que Node chargera.

### rootDir et outDir

Ces deux options rangent le projet : les sources d'un côté, le résultat de l'autre.

```text
mon-projet/
├── package.json
├── tsconfig.json
├── src/
│   ├── calcul.ts
│   └── index.ts
└── dist/            ← créé par tsc
    ├── calcul.js
    ├── calcul.js.map
    ├── index.js
    └── index.js.map
```

La structure de `src/` est recopiée telle quelle dans `dist/`. Ne modifiez jamais les
fichiers de `dist/` à la main : ils sont écrasés à chaque compilation.

### noUncheckedIndexedAccess

Même en mode strict, TypeScript suppose par défaut que `notes[10]` existe. Avec
`noUncheckedIndexedAccess`, il rappelle qu'une case peut manquer : `notes[i]` a le type
`number | undefined`. Vous devez alors gérer le cas vide.

Ce code fonctionne avec ou sans l'option, car il vérifie la case avant de l'utiliser :

```typescript
const notes: number[] = [12, 15];

function noteNumero(i: number): string {
  const note = notes[i];
  if (note === undefined) {
    return "pas de note";
  }
  return `note : ${note.toFixed(1)}`;
}

console.log(noteNumero(0));
console.log(noteNumero(5));
```

```sortie
note : 12.0
pas de note
```

### sourceMap

Quand une erreur se produit dans `dist/index.js`, le message parle du JavaScript, pas de
votre TypeScript. Un fichier `.js.map` (une « carte des sources ») relie chaque ligne du
`.js` à la ligne du `.ts`. Les éditeurs et les débogueurs s'en servent. Avec Node,
ajoutez `--enable-source-maps` pour que les erreurs indiquent les lignes du `.ts` :

```sh
node --enable-source-maps dist/index.js
```

## Compiler le projet

Avec ce `tsconfig.json` et le `package.json` suivant :

```json
{ "type": "module" }
```

une seule commande compile tout le dossier `src` :

```sh
npx tsc
node dist/index.js
```

```text
Moyenne : 12
```

Quand tout va bien, `tsc` n'affiche rien : pas de nouvelles, bonnes nouvelles.

> **Astuce :** ajoutez des raccourcis dans `package.json` :
> `"scripts": { "build": "tsc", "start": "node dist/index.js" }`, puis tapez
> `npm run build` et `npm start`.

## Le mode watch

Recompiler à la main après chaque modification devient vite pénible. Avec `--watch`
(« surveiller »), `tsc` reste ouvert et recompile dès que vous enregistrez un fichier :

```sh
npx tsc --watch
```

```text
9:04:09 PM - Starting compilation in watch mode...

9:04:11 PM - Found 0 errors. Watching for file changes.

9:04:12 PM - File change detected. Starting incremental compilation...

src/index.ts(5,7): error TS2322: Type 'string' is not assignable to type 'number'.

9:04:12 PM - Found 1 error. Watching for file changes.
```

Laissez ce [terminal](../debuter/glossaire.md#terminal) ouvert pendant que vous codez.
`Ctrl+C` arrête la surveillance.

## Vérifier sans compiler : tsc --noEmit

Parfois, un autre outil produit le JavaScript (un outil de construction web, ou Node
qui sait retirer lui-même les types simples). Vous voulez seulement savoir si les types
sont corrects. L'option `--noEmit` (« ne rien émettre ») vérifie tout, sans écrire un
seul fichier :

```sh
npx tsc --noEmit
```

```text
src/index.ts(5,7): error TS2322: Type 'string' is not assignable to type 'number'.
```

La commande se termine avec un code d'erreur s'il y a un problème : parfait pour une
vérification automatique avant de publier.

## Erreurs fréquentes

### Oublier le cas undefined avec noUncheckedIndexedAccess

```typescript norun
const noms: string[] = ["Ada", "Linus"];
const premier: string = noms[0];
console.log(premier.toUpperCase());
```

```text
main.ts(2,7): error TS2322: Type 'string | undefined' is not assignable to type 'string'.
  Type 'undefined' is not assignable to type 'string'.
```

Avec cette option, `noms[0]` peut être `undefined`. Vérifiez la valeur
(`if (premier !== undefined)`), ou donnez une valeur par défaut :
`const premier = noms[0] ?? "personne";`.

### Importer par défaut un module CommonJS sans esModuleInterop

```typescript norun
import fs from "fs";
console.log(fs.existsSync("main.ts"));
```

```text
main.ts(1,8): error TS1192: Module '"fs"' has no default export.
```

Le module `fs` est un ancien module CommonJS, sans export par défaut. Activez
`"esModuleInterop": true`, ou importez seulement ce dont vous avez besoin :
`import { existsSync } from "node:fs";`.

### Un fichier .ts en dehors de rootDir

Un fichier `outil.ts` traîne à la racine du projet, alors que `rootDir` vaut `"src"` et
qu'il n'y a pas de `include` :

```text
error TS6059: File '/home/vous/projet/outil.ts' is not under 'rootDir' '/home/vous/projet/src'. 'rootDir' is expected to contain all source files.
  The file is in the program because:
    Matched by default include pattern '**/*'
```

Sans `include`, `tsc` prend **tous** les `.ts` du dossier. Rangez le fichier dans `src/`,
ou ajoutez `"include": ["src"]`.

## Testez-vous

**Question 1 :** quelle commande vérifie les types de tout le projet sans créer de
fichiers `.js` ?

> **Réponse**
>
> `npx tsc --noEmit` (ou `tsc --noEmit`).

**Question 2 :** avec `"rootDir": "src"` et `"outDir": "dist"`, où se retrouve le
JavaScript de `src/outils/texte.ts` ?

> **Réponse**
>
> Dans `dist/outils/texte.js` : la structure des dossiers est recopiée.

**Question 3 :** que change `noUncheckedIndexedAccess` pour `const x = liste[3]` quand
`liste` est un `number[]` ?

> **Réponse**
>
> Le type de `x` devient `number | undefined` au lieu de `number`. Vous devez vérifier
> que la case existe avant de vous en servir.

## En résumé

- `tsconfig.json` regroupe les réglages ; `tsc` seul compile alors tout le projet.
- `npx tsc --init` crée un fichier de départ, à adapter.
- Gardez toujours `strict: true` ; ajoutez `noUncheckedIndexedAccess` pour les tableaux.
- `rootDir` et `outDir` séparent les sources (`src/`) du JavaScript produit (`dist/`).
- `tsc --watch` recompile à chaque enregistrement ; `tsc --noEmit` vérifie seulement.

Suite : [passer de JavaScript à TypeScript](migrer.md).
