# Organiser un projet

Cette page montre comment organiser un vrai projet JavaScript avec npm : créer le fichier `package.json`, écrire des scripts, installer des paquets, ranger le code dans des dossiers et relancer le programme automatiquement.

> **Objectifs :**
>
> - créer un projet avec `npm init` et comprendre `package.json` ;
> - lancer le programme et les tests avec `npm start` et `npm test` ;
> - installer des paquets et distinguer `dependencies` et `devDependencies` ;
> - ranger un projet en `src/` et `test/`, avec un `.gitignore` ;
> - relancer le programme à chaque modification avec `node --watch`.

## Un projet, un dossier

Un petit script tient dans un seul fichier. Un vrai projet a plusieurs fichiers : le
code, les tests, des réglages. Tout cela vit dans **un dossier**, et un fichier
`package.json` à la racine décrit le projet.

**npm** (*Node Package Manager*) est installé avec Node. Il crée ce fichier, lance vos
commandes et télécharge les [paquets](../debuter/glossaire.md#paquet) écrits par
d'autres. Vérifiez qu'il est là, dans le [terminal](../debuter/glossaire.md#terminal) :

```sh
npm --version
```

```text
10.9.4
```

## Créer le projet avec npm init

Créez un dossier, entrez dedans, puis lancez `npm init -y`. L'option `-y` (*yes*)
accepte toutes les réponses par défaut.

```sh
mkdir carnet
cd carnet
npm init -y
```

```text
Wrote to /home/alice/carnet/package.json:

{
  "name": "carnet",
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

Sans `-y`, `npm init` vous pose les questions une à une. Dans les deux cas, vous pouvez
modifier le fichier ensuite avec votre éditeur.

## Le fichier package.json

`package.json` est la carte d'identité du projet, écrite en
[JSON](../debuter/glossaire.md#json). Voici ce fichier une fois complété :

```json
{
  "name": "carnet",
  "version": "1.0.0",
  "description": "Un carnet de notes en ligne de commande",
  "type": "module",
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js",
    "dev": "node --watch src/index.js",
    "test": "node --test"
  },
  "author": "Alice",
  "license": "ISC",
  "dependencies": {
    "dayjs": "^1.11.13"
  },
  "devDependencies": {
    "prettier": "^3.3.3"
  }
}
```

| Champ | Rôle |
| --- | --- |
| `name` | le nom du projet, en minuscules, sans espace |
| `version` | la version, sous la forme `majeure.mineure.correctif` |
| `description` | une phrase qui présente le projet |
| `type` | `"module"` : les fichiers `.js` utilisent `import` et `export` |
| `main` | le fichier principal du projet |
| `scripts` | des commandes à lancer avec `npm run …` |
| `dependencies` | les paquets nécessaires pour faire tourner le programme |
| `devDependencies` | les paquets utiles seulement pour développer |
| `license` | la licence : ce que les autres ont le droit de faire du code |

> **À retenir :** ajoutez toujours `"type": "module"`. Sans lui, Node traite les
> fichiers `.js` comme d'anciens modules et affiche un avertissement quand il voit un
> `import` (voir [les modules](modules.md)).

## Les scripts

Le champ `scripts` donne un nom court à une commande. Deux noms sont spéciaux : `start`
et `test` se lancent directement avec `npm start` et `npm test`.

```sh
npm start
```

```text
> carnet@1.0.0 start
> node src/index.js

Moyenne : 12
```

npm affiche d'abord la commande qu'il lance (les lignes en `>`), puis la sortie du
programme.

```sh
npm test
```

```text
> carnet@1.0.0 test
> node --test

✔ moyenne de trois notes (1.36ms)
✔ moyenne d'une liste vide (0.27ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 150.06
```

Les autres scripts se lancent avec `npm run` suivi de leur nom :

```sh
npm run dev
```

> **Astuce :** `npm run` tout seul affiche la liste des scripts du projet. C'est la
> première commande à taper quand vous découvrez un projet.

## Installer des paquets

Un paquet est du code écrit par d'autres et publié sur le registre npm. Par exemple,
`dayjs` manipule les dates.

```sh
npm install dayjs
```

```text
added 1 package, and audited 2 packages in 1s

found 0 vulnerabilities
```

Trois choses se sont passées :

1. le paquet est téléchargé dans le dossier `node_modules/` ;
2. il est ajouté à `dependencies` dans `package.json` ;
3. le fichier `package-lock.json` est créé ou mis à jour.

Vous pouvez alors l'importer par son nom, sans chemin :

```javascript norun
import dayjs from "dayjs";

console.log(dayjs("2025-03-14").format("DD/MM/YYYY"));
```

```text
14/03/2025
```

### dependencies ou devDependencies ?

| Section | Pour quoi ? | Exemples | Commande |
| --- | --- | --- | --- |
| `dependencies` | le programme en a besoin pour tourner | `dayjs`, `express` | `npm install dayjs` |
| `devDependencies` | seulement pour développer | `prettier`, `eslint`, `vitest` | `npm install --save-dev prettier` |

Pensez à un restaurant : les ingrédients vont dans l'assiette du client
(`dependencies`) ; les couteaux et les planches restent en cuisine
(`devDependencies`).

```sh
npm install --save-dev prettier
```

Un outil installé ainsi se lance avec `npx`, qui le cherche dans `node_modules/` :

```sh
npx prettier --check src/
```

### Le numéro de version

`"dayjs": "^1.11.13"` ne veut pas dire « exactement 1.11.13 ». Le `^` accepte les
versions plus récentes **de la même version majeure** : `1.12.0` oui, `2.0.0` non. Une
version majeure peut casser votre code ; npm ne la prend donc jamais tout seul.

### package-lock.json

`package.json` dit « une version 1.x de dayjs ». `package-lock.json` note **la version
exacte** installée, et celles de tous les paquets dont elle dépend :

```json
{
  "name": "carnet",
  "version": "1.0.0",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "node_modules/dayjs": {
      "version": "1.11.13",
      "resolved": "https://registry.npmjs.org/dayjs/-/dayjs-1.11.13.tgz"
    }
  }
}
```

Ne le modifiez jamais à la main, mais **gardez-le** avec votre code (dans Git). Ainsi,
toute l'équipe installe exactement les mêmes versions.

### Installer un projet existant

Quand vous récupérez un projet, le dossier `node_modules/` n'est pas fourni. Une seule
commande installe tout ce que listent `package.json` et `package-lock.json` :

```sh
npm install
```

| Commande | Effet |
| --- | --- |
| `npm install` | installe tous les paquets du projet |
| `npm install nom` | ajoute un paquet à `dependencies` |
| `npm install --save-dev nom` | ajoute un paquet à `devDependencies` |
| `npm uninstall nom` | retire un paquet |
| `npm ls` | liste les paquets installés |
| `npx outil` | lance un outil installé dans le projet |

## Un projet bien rangé

Voici une organisation simple et courante :

```text
carnet/
├── package.json
├── package-lock.json
├── .gitignore
├── README.md
├── src/
│   ├── index.js        le point d'entrée : npm start le lance
│   └── calcul.js       les fonctions, sans affichage
├── test/
│   └── calcul.test.js  les tests de calcul.js
└── node_modules/       les paquets installés (jamais modifié à la main)
```

- `src/` (*source*) contient le code du programme ;
- `test/` contient les tests ; `node --test` les trouve tout seul ;
- `README.md` explique en quelques lignes à quoi sert le projet et comment le lancer.

Chaque fichier est un [module](../debuter/glossaire.md#module) : il **exporte** ce qu'il
offre et **importe** ce dont il a besoin. Le fichier `src/calcul.js` :

```javascript norun
export function moyenne(notes) {
  if (notes.length === 0) return 0;
  const somme = notes.reduce((total, n) => total + n, 0);
  return somme / notes.length;
}
```

Le fichier `src/index.js` :

```javascript norun
import { moyenne } from "./calcul.js";

console.log(`Moyenne : ${moyenne([12, 15, 9])}`);
```

Et le fichier `test/calcul.test.js` :

```javascript norun
import { test } from "node:test";
import assert from "node:assert/strict";
import { moyenne } from "../src/calcul.js";

test("moyenne de trois notes", () => {
  assert.strictEqual(moyenne([12, 15, 9]), 12);
});

test("moyenne d'une liste vide", () => {
  assert.strictEqual(moyenne([]), 0);
});
```

> **Astuce :** séparez les calculs (dans `calcul.js`) de l'affichage (dans `index.js`).
> Une fonction qui renvoie une valeur se teste facilement ; un `console.log`, beaucoup
> moins (voir [tester son code](tests.md)).

## Le fichier .gitignore

Avec [Git](../debuter/glossaire.md#git), le fichier `.gitignore` liste ce qu'il ne faut
**pas** enregistrer dans l'historique :

```text
# Les paquets : npm install les recrée
node_modules/

# Les secrets et réglages locaux
.env

# Les fichiers produits automatiquement
coverage/
*.log
```

`node_modules/` peut peser des centaines de mégaoctets : on ne le partage jamais.
`package.json` et `package-lock.json` suffisent à le reconstruire. Les mots de passe
et les clés, rangés dans `.env`, ne doivent jamais être publiés.

## Relancer automatiquement avec node --watch

Pendant le travail, vous modifiez un fichier, vous relancez, vous modifiez encore…
`node --watch` relance le programme **tout seul** à chaque enregistrement :

```sh
node --watch src/index.js
```

```text
Moyenne : 12
Completed running 'src/index.js'. Waiting for file changes before restarting...
Restarting 'src/index.js'
Moyenne : 12
Completed running 'src/index.js'. Waiting for file changes before restarting...
```

Appuyez sur `Ctrl+C` pour arrêter. Le script `dev` du `package.json` lance exactement
cette commande : `npm run dev`.

> **Astuce :** les tests aussi : `node --test --watch` relance les tests à chaque
> modification.

## Erreurs fréquentes

### Lancer un script qui n'existe pas

```sh
npm run lancer
```

```text
npm error Missing script: "lancer"
npm error
npm error To see a list of scripts, run:
npm error   npm run
```

Le nom du script est faux, ou il manque dans `package.json`. Tapez `npm run` pour voir
les scripts disponibles.

### Importer un paquet non installé

```javascript norun
import chalk from "chalk";

console.log(chalk.green("ok"));
```

```text
Error [ERR_MODULE_NOT_FOUND]: Cannot find package 'chalk' imported from /home/alice/carnet/src/index.js
```

Le paquet n'est pas dans `node_modules/`. Installez-le avec `npm install chalk`. Sur un
projet récupéré, lancez d'abord `npm install`.

### Oublier l'extension .js dans un import

```javascript norun
import { moyenne } from "./calcul";
```

```text
Error [ERR_MODULE_NOT_FOUND]: Cannot find module '/home/alice/carnet/src/calcul' imported from /home/alice/carnet/src/index.js
```

Avec `import`, Node ne devine pas l'extension. Écrivez le nom complet :
`import { moyenne } from "./calcul.js";`.

## Testez-vous

**Question 1 :** vous installez ESLint pour vérifier votre code. Dans quelle section de
`package.json` doit-il aller, et avec quelle commande ?

> **Réponse**
>
> Dans `devDependencies`, car il ne sert qu'à développer :
> `npm install --save-dev eslint`.

**Question 2 :** faut-il enregistrer `node_modules/` et `package-lock.json` dans Git ?

> **Réponse**
>
> `node_modules/` : non, il va dans `.gitignore` ; `npm install` le recrée.
> `package-lock.json` : oui, il garantit que tout le monde installe les mêmes versions.

**Question 3 :** quelle ligne ajouter à `package.json` pour que `npm start` lance
`node src/app.js` ?

> **Réponse**
>
> Dans `scripts` : `"start": "node src/app.js"`.

## En résumé

- `npm init -y` crée `package.json`, la carte d'identité du projet ; ajoutez
  `"type": "module"`.
- `scripts` donne des noms courts aux commandes : `npm start`, `npm test`, `npm run dev`.
- `npm install nom` ajoute un paquet à `dependencies`, `--save-dev` à `devDependencies`.
- `package-lock.json` fixe les versions exactes : gardez-le ; ignorez `node_modules/`.
- Rangez le code dans `src/`, les tests dans `test/`, et relancez avec `node --watch`.

Suite : [projets pas à pas](projets.md). Comparez avec
[un projet Ruby](../ruby/projet.md).
