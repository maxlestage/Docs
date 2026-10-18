# Modules et npm

Cette page montre comment découper un programme en plusieurs fichiers avec `import` et `export`, puis comment utiliser les paquets d'autres développeurs avec npm.

> **Objectifs :**
>
> - vous saurez exporter et importer du code entre fichiers ;
> - vous saurez faire la différence entre export par défaut et export nommé ;
> - vous saurez créer un projet avec `npm init` et lire un `package.json` ;
> - vous saurez installer un paquet et lancer des scripts avec `npm run` ;
> - vous saurez pourquoi `node_modules` ne va pas dans Git.

## Pourquoi découper en modules ?

Un programme de 50 lignes tient dans un fichier. Un programme de 5 000 lignes, non : on
s'y perd. On le découpe alors en [modules](../debuter/glossaire.md#module) : des
fichiers qui ont chacun un rôle précis.

Pensez à une cuisine : les épices dans un placard, la vaisselle dans un autre. Chaque
placard a une étiquette, et on va chercher ce dont on a besoin.

En JavaScript moderne, **chaque fichier est un module**. Ce qui est déclaré dans un
fichier y reste privé, sauf ce qu'il **exporte** explicitement.

## Les modules intégrés de Node

Node fournit des modules tout prêts, dont le nom commence par `node:`. On les importe
avec `import { ... } from "..."` en tête de fichier.

```javascript
import { join, basename, extname } from "node:path";

const chemin = join("photos", "vacances", "plage.jpg");
console.log(chemin);
console.log(basename(chemin));
console.log(extname(chemin));
```

```sortie
photos/vacances/plage.jpg
plage.jpg
.jpg
```

Entre les accolades, on choisit ce qu'on veut prendre dans le module. On peut aussi
renommer un élément avec `as`, ou tout prendre d'un coup avec `* as` :

```javascript
import { join as assembler } from "node:path";
import * as path from "node:path";

console.log(assembler("a", "b"));
console.log(path.dirname("/maison/cuisine/four.txt"));
```

```sortie
a/b
/maison/cuisine
```

## Exporter et importer vos fonctions

Passons à vos propres modules. Il y a deux façons d'exporter.

### L'export nommé

On écrit `export` devant chaque déclaration à partager. Un fichier peut avoir autant
d'exports nommés qu'il veut. Celui qui importe doit utiliser **le même nom**, entre
accolades.

```javascript norun
// maths.js
export const VERSION = "1.0";

export function somme(nombres) {
  return nombres.reduce((total, n) => total + n, 0);
}

export function moyenne(nombres) {
  return somme(nombres) / nombres.length;
}
```

```javascript norun
// main.js
import { moyenne, VERSION } from "./maths.js";
```

### L'export par défaut

`export default` désigne **la** chose principale d'un fichier. Il n'y en a qu'un par
fichier. Celui qui importe l'écrit **sans accolades**, et choisit le nom qu'il veut.

```javascript norun
// saluer.js
export default function saluer(nom) {
  return `Bonjour, ${nom} !`;
}
```

```javascript norun
// main.js
import saluer from "./saluer.js";
```

| | Export nommé | Export par défaut |
|-|--------------|-------------------|
| Écriture | `export function somme` | `export default function saluer` |
| Combien par fichier ? | autant que voulu | un seul |
| Import | `import { somme } from "./maths.js"` | `import saluer from "./saluer.js"` |
| Nom à l'import | le même (ou `as`) | libre |

> **Astuce :** préférez les exports nommés : les noms restent les mêmes dans tout le
> projet, et l'éditeur vous aide mieux.

## Un projet en plusieurs fichiers

Voici un petit projet complet. L'arbre des fichiers :

```text
mon-projet/
├── package.json
└── src/
    ├── main.js
    ├── maths.js
    └── saluer.js
```

`maths.js` et `saluer.js` sont ceux de la section précédente. `main.js` les utilise :

```javascript norun
// src/main.js
import saluer from "./saluer.js";
import { moyenne, VERSION } from "./maths.js";

console.log(saluer("Ada"));
console.log(`Moyenne : ${moyenne([12, 15, 18])}`);
console.log(`Version ${VERSION}`);
```

On lance le fichier principal depuis le dossier `mon-projet` :

```sh
node src/main.js
```

```text
Bonjour, Ada !
Moyenne : 15
Version 1.0
```

Trois règles à retenir :

- un chemin vers **votre** fichier commence par `./` (même dossier) ou `../` (dossier
  parent) ;
- l'extension `.js` est **obligatoire** ;
- les `import` se placent en haut du fichier.

`somme` n'est pas importée dans `main.js`, mais `moyenne` l'utilise quand même : à
l'intérieur de `maths.js`, tout est visible.

### Le champ "type": "module"

Pour que Node lise les fichiers `.js` comme des modules `import`/`export`, le fichier
`package.json` du projet doit contenir `"type": "module"`. Sinon Node affiche un
avertissement. Les fichiers `.mjs` sont toujours des modules, quel que soit le
`package.json`.

## L'ancienne syntaxe : require

Vous rencontrerez souvent, dans du code plus ancien, une autre syntaxe appelée
**CommonJS** :

```javascript norun
// Ancienne syntaxe CommonJS (fichiers .cjs, ou projets sans "type": "module")
const path = require("node:path");

function saluer(nom) {
  return `Bonjour, ${nom} !`;
}

module.exports = { saluer };
```

`require` joue le rôle d'`import`, `module.exports` celui d'`export`. Pour un nouveau
projet, utilisez `import` et `export` : c'est la syntaxe standard, qui marche aussi dans
le navigateur.

## Créer un projet avec npm

**npm** (*Node Package Manager*, « gestionnaire de paquets de Node ») est installé avec
Node. Il gère votre projet et ses [paquets](../debuter/glossaire.md#paquet) : du code
écrit par d'autres, prêt à réutiliser. C'est une immense bibliothèque publique : plus de
deux millions de paquets.

`npm init -y` crée le fichier `package.json`, la carte d'identité du projet (`-y`
accepte toutes les réponses par défaut).

```sh
mkdir mon-projet
cd mon-projet
npm init -y
npm pkg set type=module
```

```json
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
  "license": "ISC",
  "type": "module"
}
```

C'est du [JSON](../debuter/glossaire.md#json). Les champs principaux :

| Champ | Rôle |
|-------|------|
| `name`, `version` | le nom et la version du projet |
| `scripts` | des commandes raccourcies, lancées avec `npm run` |
| `type` | `"module"` pour utiliser `import` et `export` |
| `dependencies` | les paquets dont le projet a besoin (ajouté par `npm install`) |

## Installer un paquet

`npm install` (ou `npm i`) télécharge un paquet et l'ajoute au projet. Prenons
`slugify`, qui transforme un titre en adresse web propre.

```sh
npm install slugify
```

```text
added 1 package, and audited 2 packages in 535ms

found 0 vulnerabilities
```

Trois choses ont changé dans le dossier :

- un dossier `node_modules/` contient le code du paquet ;
- `package.json` a une nouvelle section `"dependencies": { "slugify": "^1.6.9" }` ;
- un fichier `package-lock.json` note la version exacte installée.

On importe un paquet par son **nom**, sans `./` :

```javascript norun
// src/titre.js
import slugify from "slugify";

const titre = "Les fêtes de fin d'année à Lyon";
console.log(slugify(titre, { lower: true, strict: true }));
```

```text
les-fetes-de-fin-dannee-a-lyon
```

> **Note :** `^1.6.9` veut dire « la version 1.6.9 ou une version plus récente
> compatible (1.x) ». Pour retirer un paquet : `npm uninstall slugify`.

## Les scripts npm

La section `scripts` de `package.json` donne des noms courts à des commandes. On les
lance avec `npm run <nom>`. `start` et `test` ont un raccourci : `npm start`, `npm test`.

```json
"scripts": {
  "start": "node src/main.js",
  "titre": "node src/titre.js"
}
```

```sh
npm start
```

```text
> mon-projet@1.0.0 start
> node src/main.js

Bonjour, Ada !
Moyenne : 15
Version 1.0
```

Ainsi, quelqu'un qui découvre le projet n'a pas besoin de deviner quel fichier lancer :
`npm start` suffit.

## node_modules et .gitignore

`node_modules/` peut contenir des milliers de fichiers. On ne l'enregistre **jamais**
dans [Git](../debuter/git.md) : `package.json` et `package-lock.json` suffisent pour le
recréer. Ajoutez-le au fichier `.gitignore` du projet :

```text
node_modules/
```

Quelqu'un qui récupère votre projet tape simplement :

```sh
npm install
```

Sans nom de paquet, `npm install` lit `package.json` et installe tout ce qui est listé.
C'est comme une liste de courses : on garde la liste, pas le contenu du frigo.

## Lancer un outil avec npx

`npx` lance un outil publié sur npm **sans l'installer** dans votre projet : il le
télécharge, l'exécute, et c'est tout. Pratique pour essayer un outil ou pour créer un
projet.

```sh
npx --yes cowsay "Bonjour"
```

```text
 _________
< Bonjour >
 ---------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

`--yes` évite que `npx` demande la permission de télécharger.

## Erreurs fréquentes

### Oublier l'extension .js

```javascript norun
import { moyenne } from "./maths";
```

```text
node:internal/modules/esm/resolve:274
    throw new ERR_MODULE_NOT_FOUND(
          ^

Error [ERR_MODULE_NOT_FOUND]: Cannot find module '/home/vous/mon-projet/src/maths' imported from /home/vous/mon-projet/src/main.js
```

Node cherche un fichier qui s'appelle exactement `maths`, sans extension. Écrivez le nom
complet : `"./maths.js"`.

### Importer un nom qui n'est pas exporté

```javascript norun
import { moyenne, mediane } from "./maths.js";
```

```text
file:///home/vous/mon-projet/src/main.js:2
import { moyenne, mediane } from "./maths.js";
                  ^^^^^^^
SyntaxError: The requested module './maths.js' does not provide an export named 'mediane'
```

« Le module ne fournit pas d'export nommé `mediane` » : la fonction n'existe pas, ou
elle n'a pas de `export` devant. Vérifiez l'orthographe et ajoutez `export`. Le même
message apparaît si vous écrivez `import { saluer }` pour un export **par défaut** :
retirez alors les accolades.

### Utiliser require dans un module

```javascript norun
const fs = require("node:fs");
```

```text
file:///home/vous/mon-projet/src/vieux.js:1
const fs = require("node:fs");
           ^

ReferenceError: require is not defined in ES module scope, you can use import instead
```

Dans un module (`.mjs`, ou `"type": "module"`), `require` n'existe pas. Écrivez
`import fs from "node:fs";` ou `import { existsSync } from "node:fs";`.

## Testez-vous

**Question 1 :** un fichier contient `export default function afficher() {}` et
`export const LIMITE = 10;`. Comment importer les deux ?

> **Réponse**
>
> `import afficher, { LIMITE } from "./fichier.js";` : le défaut sans accolades, d'abord,
> puis les noms entre accolades.

**Question 2 :** quelle est la différence entre `"./slugify.js"` et `"slugify"` dans un
`import` ?

> **Réponse**
>
> `"./slugify.js"` est **votre** fichier, à côté du fichier courant. `"slugify"`, sans
> `./`, est un **paquet** installé avec npm, cherché dans `node_modules`.

**Question 3 :** un ami récupère votre projet depuis Git, mais il n'y a pas de dossier
`node_modules`. Que doit-il faire ?

> **Réponse**
>
> Taper `npm install` dans le dossier du projet. npm lit `package.json` et
> `package-lock.json`, et réinstalle tous les paquets.

## En résumé

- Chaque fichier est un module ; `export` partage, `import` récupère.
- Export nommé : `import { nom }` ; export par défaut : `import nom`, sans accolades.
- Les chemins de vos fichiers commencent par `./` et finissent par `.js`.
- `npm init` crée `package.json` ; `npm install` ajoute un paquet ; `npm run` lance un
  script.
- `node_modules/` va dans `.gitignore` ; `npx` lance un outil sans l'installer.

Suite : [la programmation asynchrone](asynchrone.md). Comparez avec
[les modules en Ruby](../ruby/modules.md).
