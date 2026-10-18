# Modules et npm

Cette page explique comment découper un programme TypeScript en plusieurs fichiers, comment partager des types entre eux, et comment utiliser des paquets npm avec leurs types.

> **Objectifs :**
>
> - vous saurez exporter et importer des fonctions, des valeurs et des types ;
> - vous saurez quand écrire `import type` ;
> - vous saurez organiser et compiler un projet de plusieurs fichiers ;
> - vous saurez installer un paquet npm et ses types `@types/…` ;
> - vous saurez ce qu'est un fichier de déclaration `.d.ts`.

## Pourquoi des modules ?

Un programme qui grandit devient difficile à lire dans un seul fichier. On le découpe
en [modules](../debuter/glossaire.md#module) : un fichier par sujet. C'est comme
ranger une cuisine : les épices dans un tiroir, les casseroles dans un autre.

En TypeScript, **chaque fichier qui contient `import` ou `export` est un module**. Ce
qu'il déclare reste privé, sauf ce qu'il **exporte** explicitement.

## Importer un module de Node

Node fournit des modules tout prêts. Leur nom commence par `node:`. Vous importez
seulement ce dont vous avez besoin, entre accolades.

```typescript
import { writeFileSync, readFileSync } from "node:fs";
import path from "node:path";

const chemin = path.join("notes", "..", "courses.txt");
console.log(chemin);

writeFileSync(chemin, "pain\nlait\n");
const contenu = readFileSync(chemin, "utf8");
console.log(contenu.split("\n").filter((l) => l !== "").length, "articles");
```

```sortie
courses.txt
2 articles
```

Il y a deux formes d'import :

- `import { a, b } from "…"` : des **exports nommés**, choisis un par un ;
- `import path from "…"` : l'**export par défaut** du module, sous le nom que vous
  voulez.

Grâce aux types de Node, TypeScript connaît la signature de `readFileSync` : il sait
qu'avec `"utf8"`, elle renvoie un `string`.

## Exporter depuis vos fichiers

Placez `export` devant ce que vous voulez partager : une fonction, une constante, une
classe, et aussi un **type** ou une **interface**.

```typescript norun
// src/types.ts
export interface Tache {
  titre: string;
  faite: boolean;
}
```

```typescript norun
// src/outils.ts
import type { Tache } from "./types.js";

export function formater(t: Tache): string {
  return `${t.faite ? "[x]" : "[ ]"} ${t.titre}`;
}

export default function compter(taches: Tache[]): number {
  return taches.filter((t) => !t.faite).length;
}

const SECRET = 42; // pas exporté : invisible depuis les autres fichiers
```

Un fichier a autant d'exports nommés que vous voulez, et au plus un `export default`.

> **Astuce :** préférez les exports nommés. Le nom est le même partout, et votre
> éditeur les retrouve et les renomme facilement.

### L'extension .js dans les imports

```typescript norun
// src/main.ts
import compter, { formater } from "./outils.js";
import type { Tache } from "./types.js";

const taches: Tache[] = [
  { titre: "Acheter du pain", faite: true },
  { titre: "Réviser TypeScript", faite: false },
];

for (const t of taches) {
  console.log(formater(t));
}
console.log(`Reste à faire : ${compter(taches)}`);
```

Le fichier s'appelle `outils.ts`, mais l'import dit `./outils.js`. C'est voulu : Node
exécutera le fichier **compilé**, qui s'appellera `outils.js`. TypeScript comprend
qu'il doit vérifier `outils.ts`. Le `./` indique un fichier de votre projet, pas un
paquet.

## import type

Un type n'existe pas à l'exécution : la [compilation](../debuter/glossaire.md#compilation)
efface tous les types. `import type` dit clairement : « j'importe seulement un type ».
Cet import disparaît entièrement du JavaScript produit.

```typescript norun
import type { Tache } from "./types.js";         // seulement des types
import { formater, type Options } from "./outils.js"; // un mélange
```

Vous pouvez aussi mettre `type` devant un seul nom, au milieu d'un import ordinaire.

Pourquoi s'en soucier ? Le lecteur voit tout de suite ce qui est un type. Et certains
outils, qui retirent les types sans tout analyser, en ont besoin pour savoir quoi
effacer. Prenez l'habitude : un import de type s'écrit avec `type`.

## Un projet de plusieurs fichiers

Voici l'organisation d'un petit projet :

```text
taches/
├── package.json
├── tsconfig.json
├── src/
│   ├── main.ts
│   ├── outils.ts
│   └── types.ts
└── dist/            ← créé par tsc
    ├── main.js
    ├── outils.js
    └── types.js
```

Le code source est dans `src/`, le JavaScript compilé dans `dist/`. Le fichier
`package.json` décrit le projet pour npm. La ligne `"type": "module"` dit à Node
d'utiliser les modules `import` / `export` :

```json
{
  "name": "taches",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "node dist/main.js"
  },
  "devDependencies": {
    "typescript": "^5.8.3"
  }
}
```

Le fichier `tsconfig.json` règle le compilateur (voir [la configuration](configuration.md)
pour le détail) :

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "strict": true,
    "rootDir": "src",
    "outDir": "dist"
  },
  "include": ["src"]
}
```

On compile tout le projet, puis on lance le fichier principal :

```sh
npm run build
npm start
```

```text
[x] Acheter du pain
[ ] Réviser TypeScript
Reste à faire : 1
```

`npm run build` lance `tsc`, qui lit `tsconfig.json`, vérifie **tous** les fichiers de
`src/` ensemble et écrit `dist/`. Si un fichier utilise mal un type exporté par un
autre, l'erreur apparaît à ce moment-là.

> **Note :** `types.js` est presque vide : le fichier ne contenait qu'une interface, et
> les types sont effacés.

## Les paquets npm

npm est le gestionnaire de [paquets](../debuter/glossaire.md#paquet) de JavaScript.
Il télécharge des bibliothèques écrites par d'autres dans le dossier `node_modules/`
et les note dans `package.json`.

```sh
npm install lodash
```

Puis on importe le paquet par son nom, sans `./` :

```typescript norun
import _ from "lodash";

console.log(_.chunk([1, 2, 3, 4, 5], 2));
```

Surprise : TypeScript refuse de compiler.

```text
src/main.ts(1,15): error TS7016: Could not find a declaration file for module 'lodash'. '/home/alice/taches/node_modules/lodash/lodash.js' implicitly has an 'any' type.
  Try `npm i --save-dev @types/lodash` if it exists or add a new declaration (.d.ts) file containing `declare module 'lodash';`
```

`lodash` est écrit en JavaScript, sans types. TypeScript ne sait pas ce que contient
`_`.

### Les paquets @types

Pour beaucoup de paquets JavaScript, des bénévoles ont écrit les types à part, dans le
projet **DefinitelyTyped**. Ils sont publiés sous le nom `@types/<paquet>`. Le
message d'erreur vous le suggère :

```sh
npm install --save-dev @types/lodash
```

```text
[ [ 1, 2 ], [ 3, 4 ], [ 5 ] ]
```

Le programme compile et s'exécute. `--save-dev` range ce paquet dans
`devDependencies` : les types ne servent qu'au développement, pas à l'exécution.

```json
{
  "dependencies": {
    "lodash": "^4.18.1"
  },
  "devDependencies": {
    "@types/lodash": "^4.17.26"
  }
}
```

Les types de Node eux-mêmes viennent de `@types/node`.

### Les paquets qui ont leurs types

Les paquets récents sont souvent écrits en TypeScript et **fournissent leurs types**.
Rien à ajouter : `npm install zod` suffit. Pour savoir où vous en êtes :

| Situation | Que faire ? |
|-----------|-------------|
| Le paquet fournit ses types | rien |
| Erreur TS7016, et `@types/<nom>` existe | `npm install --save-dev @types/<nom>` |
| Erreur TS7016, et rien n'existe | écrire un petit fichier `.d.ts` |

> **Astuce :** sur le site de npm, un petit badge « TS » à côté du nom indique un
> paquet qui fournit ses types ; un badge « DT » indique des types disponibles dans
> `@types`.

## Les fichiers de déclaration .d.ts

Un fichier `.d.ts` contient **seulement des types**, sans aucun code exécutable.
C'est la notice d'un module : il décrit ce que le module exporte et avec quels types.
Les paquets `@types/…` ne contiennent que cela.

### Écrire une déclaration

Imaginons un paquet `couleurs-texte`, en JavaScript, sans types nulle part. Vous
pouvez écrire sa notice vous-même, dans `src/types/couleurs-texte.d.ts` :

```typescript norun
declare module "couleurs-texte" {
  export function rouge(texte: string): string;
  export function vert(texte: string): string;
}
```

`declare` veut dire : « ceci existe ailleurs, je décris seulement sa forme ».
TypeScript vous croit sur parole : décrivez seulement les fonctions que vous utilisez,
et décrivez-les juste.

### Produire des déclarations

À l'inverse, si vous écrivez une bibliothèque, l'option `"declaration": true` de
`tsconfig.json` demande à `tsc` d'écrire un `.d.ts` à côté de chaque `.js`. Pour
`outils.ts`, il produit :

```typescript norun
import type { Tache } from "./types.js";
export declare function formater(t: Tache): string;
export default function compter(taches: Tache[]): number;
```

Les corps des fonctions ont disparu : il ne reste que les signatures. Ceux qui
utiliseront votre bibliothèque auront ainsi les types, sans rien installer de plus.

## Erreurs fréquentes

### Oublier l'extension .js

```typescript norun
// src/main.ts
import { formater } from "./outils";
```

```text
src/main.ts(2,26): error TS2835: Relative import paths need explicit file extensions in ECMAScript imports when '--moduleResolution' is 'node16' or 'nodenext'. Did you mean './outils.js'?
```

Avec `"module": "nodenext"`, Node exige l'extension complète. Écrivez
`"./outils.js"`, comme le message le propose.

### Importer ce qui n'est pas exporté

```typescript norun
// src/main.ts
import { SECRET } from "./outils.js";
```

```text
src/main.ts(2,10): error TS2614: Module '"./outils.js"' has no exported member 'SECRET'. Did you mean to use 'import SECRET from "./outils.js"' instead?
```

`SECRET` existe dans `outils.ts`, mais sans `export`. Ajoutez `export` devant sa
déclaration, si vous voulez vraiment le partager. Ne suivez pas la suggestion du
message : `import SECRET from …` importerait l'export par défaut, c'est-à-dire la
fonction `compter`.

### Utiliser comme valeur un import de type

```typescript norun
// src/main.ts
import type { formater } from "./outils.js";
console.log(formater({ titre: "Lire", faite: false }));
```

```text
src/main.ts(3,13): error TS1361: 'formater' cannot be used as a value because it was imported using 'import type'.
```

`import type` est effacé à la compilation : la fonction n'existerait plus à
l'exécution. Pour une fonction, une classe ou une constante, utilisez un `import`
ordinaire.

## Testez-vous

**Question 1 :** dans `src/main.ts`, pourquoi écrit-on `import { f } from "./outils.js"`
alors que le fichier s'appelle `outils.ts` ?

> **Réponse**
>
> Parce que Node exécute le code compilé, où le fichier s'appelle `outils.js`. Avec
> `"module": "nodenext"`, TypeScript exige le chemin que Node utilisera, et sait qu'il
> doit vérifier `outils.ts` à la place.

**Question 2 :** vous installez un paquet et obtenez l'erreur TS7016. Que faites-vous ?

> **Réponse**
>
> Le paquet ne fournit pas ses types. Installez `@types/<nom>` avec
> `npm install --save-dev @types/<nom>` s'il existe ; sinon, écrivez un fichier `.d.ts`
> avec `declare module "<nom>" { … }`.

**Question 3 :** avec le module `node:path`, affichez l'extension et le nom de base
(sans dossier) du chemin `"photos/vacances/plage.jpg"`.

> **Réponse**
>
> ```typescript
> import path from "node:path";
>
> const chemin = "photos/vacances/plage.jpg";
> console.log(path.extname(chemin));
> console.log(path.basename(chemin));
> ```
>
> ```sortie
> .jpg
> plage.jpg
> ```

## En résumé

- Un fichier avec `import` ou `export` est un module ; seul ce qui est exporté est
  visible ailleurs.
- On importe ses propres fichiers avec `./nom.js`, et les modules de Node avec
  `node:…`.
- `import type` importe seulement un type : il disparaît à la compilation.
- `tsc` compile tout le projet de `src/` vers `dist/`, guidé par `tsconfig.json`.
- Un paquet npm sans types se complète avec `@types/<nom>` ou un fichier `.d.ts`.

Suite : [la programmation asynchrone](asynchrone.md)
