# Organiser un projet

Cette page montre comment créer un vrai projet TypeScript avec npm : installer le compilateur, écrire le `tsconfig.json`, ranger le code dans `src/` et `dist/`, ajouter des scripts pour compiler, lancer et tester.

> **Objectifs :**
>
> - créer un projet avec `npm init` et installer TypeScript comme dépendance de développement ;
> - écrire un `tsconfig.json` simple et comprendre ses options principales ;
> - ranger les sources dans `src/` et le JavaScript compilé dans `dist/` ;
> - ajouter les scripts `build`, `start`, `dev`, `typecheck` et `test` ;
> - savoir quoi mettre dans le `.gitignore`.

## Pourquoi un projet ?

Jusqu'ici, chaque exemple tenait dans un seul fichier. Un vrai programme a plusieurs
fichiers, des outils (le compilateur, un lanceur de tests) et des commandes à retenir.
Un **projet** rassemble tout cela dans un dossier, avec un fichier `package.json` qui
décrit le programme et ses outils.

Toutes les commandes de cette page se tapent dans un [terminal](../debuter/terminal.md).
Il vous faut Node.js, qui fournit la commande `npm` (le gestionnaire de
[paquets](../debuter/glossaire.md#paquet) de JavaScript).

## Créer le projet avec npm init

Créez un dossier, entrez dedans, puis lancez `npm init -y` (`-y` accepte toutes les
réponses par défaut) :

```sh
mkdir mon-projet
cd mon-projet
npm init -y
```

npm crée le fichier `package.json`, la carte d'identité du projet :

```text
Wrote to /home/alice/mon-projet/package.json:

{
  "name": "mon-projet",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC"
}
```

Vous allez le compléter au fil de la page.

## Installer TypeScript comme dépendance de développement

TypeScript ne sert qu'à **construire** le programme : une fois compilé en JavaScript,
le programme n'en a plus besoin. C'est une **dépendance de développement**, d'où
l'option `--save-dev` (ou `-D`).

```sh
npm install --save-dev typescript @types/node tsx
```

- `typescript` fournit le compilateur `tsc` ;
- `@types/node` décrit les types des modules de Node (`node:fs`, `process`…) ;
- `tsx` exécute directement un fichier `.ts`, sans étape de compilation : pratique
  pendant le développement.

npm ajoute une section au `package.json` et crée un dossier `node_modules/` qui contient
les paquets téléchargés :

```json
"devDependencies": {
  "@types/node": "^22.20.5",
  "tsx": "^4.23.15",
  "typescript": "^5.8.3"
}
```

Les numéros seront sans doute différents chez vous. Le `^` veut dire « cette version ou
une plus récente compatible ».

> **À retenir :** installez TypeScript **dans le projet**, pas globalement sur
> l'ordinateur. Chaque projet garde ainsi sa propre version, et `npx tsc` utilise
> toujours la bonne.

## Le fichier tsconfig.json

Le `tsconfig.json` donne ses réglages au compilateur. `npx tsc --init` en crée un très
long, plein de commentaires. Pour débuter, ce petit fichier suffit :

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "rootDir": "src",
    "outDir": "dist",
    "strict": true,
    "sourceMap": true,
    "skipLibCheck": true
  },
  "include": ["src"]
}
```

| Option | Rôle |
|--------|------|
| `target` | la version de JavaScript produite (`es2022` : moderne, comprise par Node 22) |
| `module` | le système de modules : `nodenext` suit les règles de Node |
| `rootDir` | où sont les sources `.ts` |
| `outDir` | où écrire les fichiers `.js` compilés |
| `strict` | active toutes les vérifications strictes (indispensable) |
| `sourceMap` | crée les fichiers `.js.map` pour [déboguer](debogage.md#les-erreurs-d-execution-et-les-source-maps) |
| `skipLibCheck` | ne revérifie pas les types des paquets installés (plus rapide) |
| `include` | les dossiers à compiler |

Le chapitre [la configuration](configuration.md) détaille d'autres options.

## Ranger le code : src/ et dist/

La règle est simple : **vous écrivez dans `src/`, `tsc` écrit dans `dist/`**. Ne
modifiez jamais `dist/` à la main : il est recréé à chaque compilation.

```text
mon-projet/
├── package.json        # le projet, ses scripts et ses outils
├── package-lock.json   # les versions exactes installées (créé par npm)
├── tsconfig.json       # les réglages de tsc
├── .gitignore          # ce que Git doit ignorer
├── node_modules/       # les paquets installés (créé par npm)
├── src/                # VOS fichiers TypeScript
│   ├── calcul.ts
│   ├── calcul.test.ts
│   └── main.ts
└── dist/               # le JavaScript compilé (créé par tsc)
    ├── calcul.js
    └── main.js
```

Voici les deux fichiers sources. `calcul.ts` exporte une fonction, `main.ts` l'importe
(voir [les modules](modules.md)) :

```typescript norun
// src/calcul.ts
export function moyenne(notes: readonly number[]): number {
  if (notes.length === 0) return 0;
  return notes.reduce((a, b) => a + b, 0) / notes.length;
}
```

```typescript norun
// src/main.ts
import { moyenne } from "./calcul.js";

const notes = [12, 15, 18];
console.log(`Moyenne : ${moyenne(notes)}`);
```

> **Attention :** l'import se termine par `.js`, alors que le fichier s'appelle
> `calcul.ts`. C'est normal : l'import désigne le fichier qui **existera** après
> compilation, dans `dist/`.

## Les scripts npm

Plutôt que de retenir de longues commandes, on les range dans la section `scripts` du
`package.json`. Ajoutez aussi `"type": "module"` : il dit à Node que les fichiers `.js`
du projet sont des modules modernes (`import`/`export`).

```json
{
  "name": "mon-projet",
  "version": "1.0.0",
  "type": "module",
  "main": "dist/main.js",
  "scripts": {
    "build": "tsc",
    "start": "node --enable-source-maps dist/main.js",
    "dev": "tsx watch src/main.ts",
    "typecheck": "tsc --noEmit",
    "test": "tsx --test \"src/**/*.test.ts\""
  },
  "devDependencies": {
    "@types/node": "^22.20.5",
    "tsx": "^4.23.15",
    "typescript": "^5.8.3"
  }
}
```

| Script | Commande | Ce qu'il fait |
|--------|----------|---------------|
| `build` | `npm run build` | vérifie les types et compile `src/` vers `dist/` |
| `start` | `npm start` | lance le programme compilé |
| `dev` | `npm run dev` | lance `src/main.ts` directement, et relance à chaque modification |
| `typecheck` | `npm run typecheck` | vérifie les types sans rien écrire |
| `test` | `npm test` | lance les tests |

`start` et `test` sont des noms spéciaux : `npm start` et `npm test` suffisent. Pour les
autres, il faut `npm run` devant.

### Compiler puis lancer

```sh
npm run build
npm start
```

```text
> mon-projet@1.0.0 build
> tsc


> mon-projet@1.0.0 start
> node --enable-source-maps dist/main.js

Moyenne : 15
```

`tsc` n'affiche rien quand tout va bien. S'il trouve une erreur de type, il l'affiche
et s'arrête avec un code d'erreur :

```text
> mon-projet@1.0.0 build
> tsc

src/bad.ts(1,19): error TS7006: Parameter 'x' implicitly has an 'any' type.
```

### Développer avec tsx

Pendant que vous écrivez le code, recompiler à chaque fois est fastidieux. `npm run dev`
lance `tsx watch` : le programme redémarre dès que vous enregistrez un fichier.

```sh
npm run dev
```

> **Attention :** `tsx` **ne vérifie pas les types** : il les efface et exécute. Il est
> rapide, mais il laisse passer les erreurs de type. Lancez `npm run typecheck`
> régulièrement, et laissez l'éditeur souligner les erreurs pendant que vous tapez.

Pour lancer un fichier une seule fois : `npx tsx src/main.ts`.

## Lancer les tests

Node est livré avec un lanceur de [tests](tests.md) : le module `node:test`. Placez les
tests à côté du code, dans des fichiers qui finissent par `.test.ts` :

```typescript norun
// src/calcul.test.ts
import { test } from "node:test";
import assert from "node:assert/strict";
import { moyenne } from "./calcul.js";

test("moyenne de trois notes", () => {
  assert.equal(moyenne([12, 15, 18]), 15);
});

test("moyenne d'une liste vide", () => {
  assert.equal(moyenne([]), 0);
});
```

```sh
npm test
```

```text
✔ moyenne de trois notes (0.722175ms)
✔ moyenne d'une liste vide (0.113289ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 277.389919
```

Les durées changent à chaque fois. Ce qui compte : `pass 2`, `fail 0`.

## Le fichier .gitignore

Si vous utilisez [Git](../debuter/git.md), certains dossiers ne doivent **pas** être
enregistrés : ils sont recréés par une commande. Créez un fichier `.gitignore` :

```text
node_modules/
dist/
*.log
.env
```

- `node_modules/` est recréé par `npm install`, à partir de `package.json` et
  `package-lock.json` ;
- `dist/` est recréé par `npm run build` ;
- `.env` contient souvent des mots de passe : il ne doit jamais être partagé.

Par contre, enregistrez bien `package.json`, `package-lock.json` et `tsconfig.json`.
Quelqu'un qui récupère votre projet tape simplement :

```sh
npm install
npm run build
npm start
```

## Erreurs fréquentes

### Oublier l'extension .js dans un import

```typescript norun
// src/main.ts
import { moyenne } from "./calcul";
```

```text
src/main.ts(1,25): error TS2835: Relative import paths need explicit file extensions in ECMAScript imports when '--moduleResolution' is 'node16' or 'nodenext'. Did you mean './calcul.js'?
```

Avec `"module": "nodenext"`, les imports relatifs doivent donner l'extension, comme Node
l'exige. Le message propose lui-même la correction : `"./calcul.js"`.

### Lancer le programme avant de l'avoir compilé

```sh
npm start
```

```text
Error: Cannot find module '/home/alice/mon-projet/dist/main.js'
```

Le dossier `dist/` n'existe pas encore (ou a été supprimé). Lancez `npm run build`
d'abord. Pendant le développement, `npm run dev` évite cette étape.

### Donner un fichier à tsc

```sh
npx tsc src/main.ts
ls src
```

```text
calcul.js
calcul.test.ts
calcul.ts
main.js
main.ts
```

Quand on donne un fichier à `tsc`, il **ignore** le `tsconfig.json` : pas de `strict`,
pas de `dist/`. Les `.js` atterrissent à côté des sources. Supprimez-les et tapez
simplement `npx tsc` (ou `npm run build`) : le `tsconfig.json` sera lu.

## Testez-vous

**Question 1 :** pourquoi installer TypeScript avec `--save-dev` ?

> **Réponse**
>
> Parce qu'il ne sert qu'à construire le programme. Une fois compilé en JavaScript, le
> programme tourne avec Node seul : TypeScript est une dépendance de développement.

**Question 2 :** quelle est la différence entre `npm run build` et `npm run dev` ?

> **Réponse**
>
> `build` lance `tsc` : il vérifie les types et écrit le JavaScript dans `dist/`. `dev`
> lance `tsx watch` : il exécute directement les `.ts` sans vérifier les types, et
> relance le programme à chaque modification.

**Question 3 :** quels dossiers mettre dans le `.gitignore` d'un projet TypeScript ?

> **Réponse**
>
> `node_modules/` (recréé par `npm install`) et `dist/` (recréé par `npm run build`),
> ainsi que les fichiers secrets comme `.env`.

## En résumé

- `npm init -y` crée le `package.json` ; `npm install --save-dev typescript @types/node tsx`
  installe les outils dans le projet.
- Le `tsconfig.json` règle `tsc` : `strict`, `rootDir: "src"`, `outDir: "dist"`,
  `module: "nodenext"`.
- Vous écrivez dans `src/`, `tsc` écrit dans `dist/` ; les imports finissent par `.js`.
- Les scripts `build`, `start`, `dev`, `typecheck` et `test` évitent de retenir les
  commandes.
- `node_modules/` et `dist/` vont dans le `.gitignore`.

Suite : [projets pas à pas](projets.md).
