# Lire et écrire des fichiers avec Node

Cette page montre comment Node écrit, lit et complète des fichiers texte, liste un dossier et gère un fichier absent.

> **Objectifs :**
>
> - vous saurez écrire et lire un fichier avec `writeFile` et `readFile` ;
> - vous saurez ajouter du texte à la fin d'un fichier avec `appendFile` ;
> - vous saurez construire un chemin avec `path.join` et lister un dossier ;
> - vous saurez lire un fichier ligne par ligne ;
> - vous saurez réagir à un fichier absent (l'erreur `ENOENT`).

## Pourquoi des fichiers ?

Les variables disparaissent à la fin du programme. Un **fichier** garde des données sur
le disque, sous un nom : il survit à l'arrêt de l'ordinateur. C'est la différence entre
une ardoise qu'on efface et un cahier qu'on range.

Dans le navigateur, JavaScript n'a pas le droit de toucher au disque (voir
[le DOM](dom.md) et `localStorage`). Dans **Node**, si : le module `node:fs` (*file
system*, « système de fichiers ») s'en charge.

## Le module node:fs/promises

`node:fs/promises` contient les fonctions de fichiers en version **asynchrone** : elles
renvoient une promesse, et vous attendez le résultat avec `await` (voir
[l'asynchrone](asynchrone.md)). Dans un module, `await` fonctionne directement au
niveau principal du fichier.

```javascript norun
import { readFile, writeFile, appendFile, readdir, mkdir } from "node:fs/promises";
```

> **Note :** le préfixe `node:` indique un module livré avec Node. Rien à installer.

## Écrire un fichier avec writeFile

`writeFile(nom, texte)` crée le fichier et y écrit le texte. Si le fichier existe déjà,
son contenu est **remplacé**.

```javascript
import { writeFile } from "node:fs/promises";

await writeFile("message.txt", "Bonjour !\n");
console.log("Fichier écrit.");
```

```sortie
Fichier écrit.
```

Un nom simple comme `"message.txt"` désigne un fichier du **dossier courant** : celui
depuis lequel vous avez lancé `node`.

> **Attention :** `writeFile` efface l'ancien contenu sans prévenir. Et il n'ajoute pas
> de retour à la ligne : écrivez `\n` vous-même.

## Lire un fichier avec readFile

`readFile(nom, "utf8")` renvoie tout le contenu du fichier, en un seul texte.

```javascript
import { readFile, writeFile } from "node:fs/promises";

await writeFile("message.txt", "Bonjour !\nÀ bientôt.\n");
const contenu = await readFile("message.txt", "utf8");
console.log(contenu);
```

```sortie
Bonjour !
À bientôt.
```

Le deuxième argument, `"utf8"`, est l'**encodage** : la façon de transformer les octets
du disque en lettres. UTF-8 sait écrire toutes les lettres, accents compris (voir
[Unicode](../debuter/glossaire.md#unicode)). Sans lui, vous recevez des octets bruts
(voir les erreurs fréquentes).

## Ajouter à la fin avec appendFile

`appendFile` ajoute du texte **après** le contenu existant. Si le fichier n'existe pas,
il est créé. C'est parfait pour un journal.

```javascript
import { readFile, writeFile, appendFile } from "node:fs/promises";

await writeFile("journal.txt", "lundi : vélo\n");
await appendFile("journal.txt", "mardi : piscine\n");
await appendFile("journal.txt", "mercredi : repos\n");

console.log(await readFile("journal.txt", "utf8"));
```

```sortie
lundi : vélo
mardi : piscine
mercredi : repos
```

| Fonction | Si le fichier existe | S'il n'existe pas |
|----------|----------------------|-------------------|
| `writeFile` | il est remplacé | il est créé |
| `appendFile` | on écrit à la fin | il est créé |
| `readFile` | il est lu | erreur `ENOENT` |

## Lire ligne par ligne

Un petit fichier se lit en entier, puis se découpe avec `split("\n")`. La dernière
ligne d'un fichier se termine souvent par `\n` : le découpage laisse alors une case
vide à la fin. `filter` la retire.

```javascript
import { readFile, writeFile } from "node:fs/promises";

await writeFile("courses.txt", "pain\nlait\nœufs\n");

const texte = await readFile("courses.txt", "utf8");
const lignes = texte.split("\n").filter((ligne) => ligne !== "");

console.log(lignes);
for (const [i, article] of lignes.entries()) {
  console.log(`${i + 1}. ${article}`);
}
```

```sortie
[ 'pain', 'lait', 'œufs' ]
1. pain
2. lait
3. œufs
```

### Un très gros fichier

Pour un fichier énorme (des millions de lignes), mieux vaut ne pas tout charger en
mémoire. `node:readline` lit le fichier une ligne à la fois, avec `for await`.

```javascript
import { writeFile } from "node:fs/promises";
import { createReadStream } from "node:fs";
import { createInterface } from "node:readline";

await writeFile("nombres.txt", "4\n8\n15\n16\n23\n42\n");

const lecteur = createInterface({ input: createReadStream("nombres.txt") });
let total = 0;
for await (const ligne of lecteur) {
  total += Number(ligne);
}
console.log(`Total : ${total}`);
```

```sortie
Total : 108
```

## Construire un chemin avec path.join

Un [chemin](../debuter/glossaire.md#chemin) indique où se trouve un fichier :
`donnees/notes.txt`. Le séparateur change selon le système (`/` sur Linux et macOS,
`\` sur Windows). `path.join` colle les morceaux avec le bon séparateur.

```javascript
import path from "node:path";

const chemin = path.join("donnees", "2024", "notes.txt");
console.log(chemin);
console.log(path.basename(chemin));
console.log(path.extname(chemin));
console.log(path.dirname(chemin));
```

```sortie
donnees/2024/notes.txt
notes.txt
.txt
donnees/2024
```

- `basename` garde le nom du fichier ;
- `extname` donne l'extension ;
- `dirname` donne le dossier.

## Les dossiers

- `mkdir(nom, { recursive: true })` crée un dossier (et ses parents), sans erreur s'il
  existe déjà ;
- `readdir(nom)` renvoie la liste de ce qu'il contient.

L'ordre de `readdir` dépend du système : triez la liste pour un affichage stable.

```javascript
import { mkdir, writeFile, readdir } from "node:fs/promises";
import path from "node:path";

await mkdir("notes", { recursive: true });
await writeFile(path.join("notes", "mardi.txt"), "piscine");
await writeFile(path.join("notes", "lundi.txt"), "vélo");
await writeFile(path.join("notes", "photo.jpg"), "");

const tous = (await readdir("notes")).sort();
console.log(tous);

const textes = tous.filter((nom) => path.extname(nom) === ".txt");
console.log(textes);
```

```sortie
[ 'lundi.txt', 'mardi.txt', 'photo.jpg' ]
[ 'lundi.txt', 'mardi.txt' ]
```

## Vérifier qu'un fichier existe

`existsSync` (du module `node:fs`, sans `/promises`) répond tout de suite `true` ou
`false`. Le suffixe `Sync` veut dire **synchrone** : pas de promesse, pas d'`await`.

```javascript
import { existsSync } from "node:fs";
import { writeFile } from "node:fs/promises";

console.log(existsSync("absent.txt"));
await writeFile("present.txt", "ok");
console.log(existsSync("present.txt"));
```

```sortie
false
true
```

## Gérer un fichier absent

Lire un fichier qui n'existe pas lève une erreur. Son `code` vaut `"ENOENT"` (*Error, NO
ENTry* : « pas d'entrée à ce nom »). Attrapez-la avec `try...catch` (voir
[les erreurs](erreurs.md)) et regardez `code` pour savoir ce qui s'est passé.

```javascript
import { readFile } from "node:fs/promises";

async function lireNotes(nom) {
  try {
    return await readFile(nom, "utf8");
  } catch (erreur) {
    if (erreur.code === "ENOENT") {
      return "Pas encore de notes.";
    }
    throw erreur;
  }
}

console.log(await lireNotes("notes.txt"));
```

```sortie
Pas encore de notes.
```

> **À retenir :** relancez (`throw erreur`) les erreurs que vous ne savez pas traiter :
> un fichier interdit en lecture (`EACCES`) ne doit pas passer pour un fichier vide.

Cette façon de faire est souvent préférable à `existsSync` : entre la vérification et
la lecture, un autre programme peut supprimer le fichier.

## Lire l'entrée standard

L'**entrée standard** (*stdin*) est ce que reçoit le programme : le clavier, ou la
sortie d'une autre commande avec `|`. `process.stdin` se lit ligne par ligne avec
`readline`.

```javascript norun
// majuscules.mjs
import { createInterface } from "node:readline";

const lecteur = createInterface({ input: process.stdin });
for await (const ligne of lecteur) {
  console.log(ligne.toUpperCase());
}
```

```sh
printf 'pain\nlait\n' | node majuscules.mjs
```

```text
PAIN
LAIT
```

Pour poser une question à l'utilisateur, `node:readline/promises` propose `question` :

```javascript norun
import { createInterface } from "node:readline/promises";

const rl = createInterface({ input: process.stdin, output: process.stdout });
const prenom = await rl.question("Votre prénom : ");
console.log(`Bonjour, ${prenom} !`);
rl.close();
```

```text
Votre prénom : Alice
Bonjour, Alice !
```

> **Astuce :** pensez à `rl.close()`. Sans lui, le programme continue d'attendre une
> saisie et ne s'arrête pas.

## Erreurs fréquentes

### Lire un fichier qui n'existe pas

```javascript norun
import { readFile } from "node:fs/promises";

const texte = await readFile("notes.txt", "utf8");
```

```text
Error: ENOENT: no such file or directory, open 'notes.txt'
    at async open (node:internal/fs/promises:636:25)
    at async readFile (node:internal/fs/promises:1235:14)
    at async file:///…/main.mjs:3:15 {
  errno: -2,
  code: 'ENOENT',
  syscall: 'open',
  path: 'notes.txt'
}
```

Le fichier est introuvable. Vérifiez son nom et le dossier depuis lequel vous lancez
`node`. Pour un fichier facultatif, attrapez l'erreur et testez `erreur.code ===
"ENOENT"`.

### Oublier l'encodage

```javascript norun
import { readFile, writeFile } from "node:fs/promises";

await writeFile("a.txt", "Salut");
console.log(await readFile("a.txt"));
```

```text
<Buffer 53 61 6c 75 74>
```

Sans `"utf8"`, `readFile` renvoie un `Buffer` : les octets bruts, en hexadécimal. Ajoutez
le deuxième argument : `readFile("a.txt", "utf8")`.

### Oublier await

```javascript norun
import { readFile, writeFile } from "node:fs/promises";

await writeFile("a.txt", "Salut");
const texte = readFile("a.txt", "utf8");
console.log(texte);
```

```text
Promise { <pending> }
```

Sans `await`, vous recevez la promesse, pas le contenu : la lecture n'est pas encore
finie. Écrivez `const texte = await readFile("a.txt", "utf8");`.

## Testez-vous

**Question 1 :** quelle différence entre `writeFile` et `appendFile` ?

> **Réponse**
>
> `writeFile` remplace tout le contenu du fichier. `appendFile` ajoute le texte à la
> fin, sans effacer ce qui y est déjà.

**Question 2 :** que contient `erreur.code` quand on lit un fichier absent ?

> **Réponse**
>
> `"ENOENT"`.

**Question 3 :** écrivez un programme qui écrit `"a\nb\nc\n"` dans `lettres.txt`, puis
affiche le nombre de lignes du fichier.

> **Réponse**
>
> ```javascript
> import { readFile, writeFile } from "node:fs/promises";
>
> await writeFile("lettres.txt", "a\nb\nc\n");
> const texte = await readFile("lettres.txt", "utf8");
> const lignes = texte.split("\n").filter((l) => l !== "");
> console.log(lignes.length);
> ```
>
> ```sortie
> 3
> ```

## En résumé

- `node:fs/promises` fournit `writeFile` (remplace), `appendFile` (ajoute) et `readFile`.
- Donnez toujours l'encodage `"utf8"` en lecture, et n'oubliez pas `await`.
- `split("\n")` découpe un petit fichier en lignes ; `readline` lit les gros fichiers.
- `path.join` construit un chemin ; `readdir` liste un dossier (triez le résultat).
- Un fichier absent lève une erreur de code `ENOENT` : attrapez-la avec `try...catch`.

Suite : [JSON et CSV](formats.md)
