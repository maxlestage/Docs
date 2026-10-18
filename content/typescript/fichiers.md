# Lire et écrire des fichiers avec Node

Cette page montre comment lire et écrire des fichiers en TypeScript avec le module `node:fs/promises`, construire des chemins avec `node:path`, et traiter proprement un fichier absent ou mal formé.

> **Objectifs :**
>
> - lire et écrire un fichier texte avec `readFile` et `writeFile` ;
> - construire des chemins avec `node:path` ;
> - parcourir un fichier ligne par ligne ;
> - lire des données JSON en `unknown` puis les valider ;
> - gérer un fichier absent grâce au code `ENOENT`.

## Le module node:fs/promises

Un [fichier](../debuter/glossaire.md#fichier-source) vit sur le disque : il reste là
quand le programme s'arrête. Pour le lire ou l'écrire, Node fournit le module `fs`
(*file system*, « système de fichiers »).

Nous utilisons sa version à promesses, `node:fs/promises` : chaque fonction renvoie une
`Promise`, que l'on attend avec `await` (voir
[la programmation asynchrone](asynchrone.md)). Le préfixe `node:` indique un module
livré avec Node. Les types viennent du paquet `@types/node`, déjà installé avec
TypeScript dans un projet Node.

```typescript
import { writeFile, readFile } from "node:fs/promises";

await writeFile("bonjour.txt", "Bonjour le disque !\n");
const texte: string = await readFile("bonjour.txt", "utf8");
console.log(texte.trim());
```

```sortie
Bonjour le disque !
```

- `writeFile(chemin, texte)` crée le fichier, ou **remplace** son contenu s'il existe.
- `readFile(chemin, "utf8")` lit tout le fichier et renvoie un `Promise<string>`.

> **Attention :** sans le deuxième argument `"utf8"`, `readFile` renvoie un `Buffer`
> (des octets bruts), pas un texte. Le type vous le rappelle : voir les
> [erreurs fréquentes](#erreurs-frequentes).

### Ajouter à la fin d'un fichier

`appendFile` ajoute du texte à la fin, sans effacer ce qui existe. C'est l'outil idéal
pour un journal :

```typescript
import { appendFile, readFile } from "node:fs/promises";

await appendFile("journal.txt", "Démarrage\n");
await appendFile("journal.txt", "Calcul terminé\n");
await appendFile("journal.txt", "Arrêt\n");

console.log(await readFile("journal.txt", "utf8"));
```

```sortie
Démarrage
Calcul terminé
Arrêt

```

## Construire des chemins avec node:path

Un [chemin](../debuter/glossaire.md#chemin) désigne un fichier : `donnees/notes.txt`.
Le séparateur change selon le système (`/` sous Linux et macOS, `\` sous Windows).
Plutôt que de coller des morceaux de texte, utilisez `path.join`, qui choisit le bon
séparateur :

```typescript
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

| Fonction | Rôle | Pour `donnees/2024/notes.txt` |
|----------|------|-------------------------------|
| `path.join(...)` | assemble des morceaux | — |
| `path.basename(c)` | le nom du fichier | `notes.txt` |
| `path.extname(c)` | l'extension | `.txt` |
| `path.dirname(c)` | le dossier parent | `donnees/2024` |

### Créer un dossier

`mkdir` avec `{ recursive: true }` crée le dossier et ses parents, sans erreur s'il existe
déjà. `readdir` liste son contenu :

```typescript
import { mkdir, writeFile, readdir } from "node:fs/promises";
import path from "node:path";

const dossier = path.join("donnees", "notes");
await mkdir(dossier, { recursive: true });
await writeFile(path.join(dossier, "maths.txt"), "15\n");
await writeFile(path.join(dossier, "anglais.txt"), "12\n");

const fichiers: string[] = await readdir(dossier);
console.log(fichiers.sort());
```

```sortie
[ 'anglais.txt', 'maths.txt' ]
```

> **Note :** l'ordre de `readdir` dépend du système. Triez avec `sort()` pour obtenir
> toujours le même résultat.

## Lire un fichier ligne par ligne

Le plus simple : lire tout le texte, puis le découper avec `split("\n")`. Pensez à
retirer les lignes vides, souvent présentes à la fin du fichier.

```typescript
import { writeFile, readFile } from "node:fs/promises";

await writeFile("courses.txt", "pain\nlait\n\nœufs\n");

const texte = await readFile("courses.txt", "utf8");
const lignes: string[] = texte
  .split("\n")
  .map((ligne) => ligne.trim())
  .filter((ligne) => ligne !== "");

lignes.forEach((ligne, i) => console.log(`${i + 1}. ${ligne}`));
```

```sortie
1. pain
2. lait
3. œufs
```

### Les très gros fichiers

Pour un fichier énorme (des millions de lignes), tout charger en mémoire est un
gaspillage. La méthode `readLines()` d'un fichier ouvert lit les lignes **une par
une**, avec une boucle `for await` :

```typescript
import { writeFile, open } from "node:fs/promises";

await writeFile("mesures.txt", "12\n7\n30\n");

const fichier = await open("mesures.txt");
let total = 0;
for await (const ligne of fichier.readLines()) {
  total += Number(ligne);
}
console.log(`Total : ${total}`);
```

```sortie
Total : 49
```

`open` ouvre le fichier et `readLines()` donne ses lignes au fur et à mesure. Le
fichier est fermé automatiquement à la fin de la lecture.

## Lire des données JSON

Un fichier `.json` contient des données structurées (voir
[le glossaire](../debuter/glossaire.md#json)). `JSON.parse` transforme le texte en objet.
Mais TypeScript ne peut pas savoir ce que contient un fichier : il est écrit par
quelqu'un d'autre, peut-être mal.

La méthode sûre :

1. ranger le résultat de `JSON.parse` dans une variable de type `unknown` (« inconnu ») ;
2. **vérifier** la forme des données avec une fonction ;
3. seulement ensuite, s'en servir avec le bon type.

```typescript
import { writeFile, readFile } from "node:fs/promises";

interface Config {
  nom: string;
  port: number;
}

function estConfig(v: unknown): v is Config {
  return (
    typeof v === "object" &&
    v !== null &&
    "nom" in v &&
    typeof v.nom === "string" &&
    "port" in v &&
    typeof v.port === "number"
  );
}

await writeFile("config.json", '{ "nom": "mon-site", "port": 8080 }');

const brut: unknown = JSON.parse(await readFile("config.json", "utf8"));
if (estConfig(brut)) {
  console.log(`${brut.nom} écoute sur le port ${brut.port}`);
} else {
  console.log("config.json est mal formé");
}
```

```sortie
mon-site écoute sur le port 8080
```

Le type de retour `v is Config` fait de `estConfig` un **garde de type** : dans le
`if`, TypeScript sait que `brut` est un `Config`. Le chapitre
[JSON et validation des données](formats.md) détaille cette technique.

### Écrire du JSON

`JSON.stringify(valeur, null, 2)` transforme un objet en texte, indenté de 2 espaces
pour rester lisible :

```typescript
import { writeFile, readFile } from "node:fs/promises";

interface Tache {
  titre: string;
  faite: boolean;
}

const taches: Tache[] = [
  { titre: "Acheter du pain", faite: true },
  { titre: "Réviser", faite: false },
];

await writeFile("taches.json", JSON.stringify(taches, null, 2) + "\n");
console.log(await readFile("taches.json", "utf8"));
```

```sortie
[
  {
    "titre": "Acheter du pain",
    "faite": true
  },
  {
    "titre": "Réviser",
    "faite": false
  }
]

```

## Gérer un fichier absent

Lire un fichier qui n'existe pas lève une [erreur](../debuter/glossaire.md#erreur). Son
**code** vaut `"ENOENT"` (*Error NO ENTry*, « pas d'entrée »). D'autres codes existent :
`"EACCES"` (accès refusé), `"EISDIR"` (c'est un dossier).

Dans le `catch`, l'erreur est de type `unknown`. Node décrit ses erreurs système avec le
type `NodeJS.ErrnoException`, qui a un champ `code`. Une petite fonction vérifie que
l'erreur a bien ce champ :

```typescript
import { readFile } from "node:fs/promises";

function estErreurSysteme(e: unknown): e is NodeJS.ErrnoException {
  return e instanceof Error && "code" in e;
}

async function lireOuDefaut(chemin: string, defaut: string): Promise<string> {
  try {
    return await readFile(chemin, "utf8");
  } catch (e) {
    if (estErreurSysteme(e) && e.code === "ENOENT") {
      return defaut;
    }
    throw e;
  }
}

const contenu = await lireOuDefaut("absent.txt", "(fichier vide)");
console.log(contenu);
```

```sortie
(fichier vide)
```

Remarquez `throw e` : une erreur qu'on ne sait pas traiter (un accès refusé, par
exemple) est **relancée**. On n'avale que l'erreur attendue.

> **À retenir :** n'écrivez pas « vérifier si le fichier existe, puis le lire ». Le
> fichier peut disparaître entre les deux. Lisez directement, et traitez `ENOENT`.

### Un exemple complet : un compteur de visites

Le programme lit un nombre dans un fichier, l'augmente et le réécrit. La première fois,
le fichier n'existe pas : on part de zéro.

```typescript
import { readFile, writeFile } from "node:fs/promises";

function estErreurSysteme(e: unknown): e is NodeJS.ErrnoException {
  return e instanceof Error && "code" in e;
}

async function visiter(chemin: string): Promise<number> {
  let compte = 0;
  try {
    compte = Number(await readFile(chemin, "utf8"));
  } catch (e) {
    if (!(estErreurSysteme(e) && e.code === "ENOENT")) {
      throw e;
    }
  }
  compte += 1;
  await writeFile(chemin, String(compte));
  return compte;
}

for (let i = 0; i < 3; i++) {
  console.log(`Visite numéro ${await visiter("visites.txt")}`);
}
```

```sortie
Visite numéro 1
Visite numéro 2
Visite numéro 3
```

## Erreurs fréquentes

### Oublier l'encodage "utf8"

```typescript norun
import { readFile, writeFile } from "node:fs/promises";

await writeFile("notes.txt", "bonjour\n");
const texte: string = await readFile("notes.txt");
console.log(texte);
```

```text
main.ts(4,7): error TS2322: Type 'Buffer<ArrayBufferLike>' is not assignable to type 'string'.
```

Sans encodage, `readFile` renvoie des octets (`Buffer`). Écrivez
`await readFile("notes.txt", "utf8")`.

### Lire err.code sans vérifier le type

```typescript norun
import { readFile } from "node:fs/promises";

try {
  await readFile("absent.txt", "utf8");
} catch (err) {
  if (err.code === "ENOENT") {
    console.log("Fichier introuvable");
  }
}
```

```text
main.ts(6,7): error TS18046: 'err' is of type 'unknown'.
```

L'erreur attrapée est `unknown`. Vérifiez-la d'abord avec une fonction comme
`estErreurSysteme` (voir plus haut), qui la transforme en `NodeJS.ErrnoException`.

### Ne pas traiter le fichier absent

```typescript norun
import { readFile } from "node:fs/promises";

const texte = await readFile("config.json", "utf8");
console.log(texte);
```

```text
node:internal/fs/promises:636
  return new FileHandle(await PromisePrototypeThen(
                        ^

Error: ENOENT: no such file or directory, open 'config.json'
    at async open (node:internal/fs/promises:636:25)
    at async readFile (node:internal/fs/promises:1235:14)
    at async file:///home/vous/projet/main.js:2:15 {
  errno: -2,
  code: 'ENOENT',
  syscall: 'open',
  path: 'config.json'
}
```

Le programme s'arrête. Le chemin est relatif au dossier **depuis lequel** vous lancez
`node`, pas au dossier du script. Vérifiez le chemin, et entourez la lecture d'un
`try`/`catch` qui traite `ENOENT`.

### Utiliser des données JSON sans les valider

```typescript norun
const donnees: unknown = JSON.parse('{"nom": "Ada"}');
console.log(donnees.nom);
```

```text
main.ts(2,13): error TS18046: 'donnees' is of type 'unknown'.
```

C'est voulu : TypeScript refuse d'utiliser des données non vérifiées. Écrivez un garde
de type (`v is Config`) et utilisez la valeur dans le `if`.

## Testez-vous

**Question 1 :** quelle est la différence entre `writeFile` et `appendFile` ?

> **Réponse**
>
> `writeFile` remplace tout le contenu du fichier ; `appendFile` ajoute à la fin, sans
> effacer ce qui existe.

**Question 2 :** que renvoie `path.extname("photos/vacances.jpg")` ?

> **Réponse**
>
> `".jpg"`, l'extension avec son point.

**Question 3 :** écrivez un programme qui écrit trois prénoms dans `noms.txt` (un par
ligne), puis les relit et affiche leur nombre.

> **Réponse**
>
> ```typescript
> import { writeFile, readFile } from "node:fs/promises";
>
> await writeFile("noms.txt", "Ada\nLinus\nGrace\n");
> const texte = await readFile("noms.txt", "utf8");
> const noms = texte.split("\n").filter((l) => l !== "");
> console.log(`${noms.length} noms`);
> ```
>
> ```sortie
> 3 noms
> ```

## En résumé

- `node:fs/promises` fournit `readFile`, `writeFile`, `appendFile`, `mkdir`, `readdir`,
  à utiliser avec `await`.
- Donnez toujours l'encodage `"utf8"` pour lire du texte.
- `path.join` construit des chemins valables sur tous les systèmes.
- Lisez le JSON en `unknown`, puis validez-le avec un garde de type.
- Un fichier absent donne le code `ENOENT` ; testez-le via `NodeJS.ErrnoException`.

Suite : [JSON et validation des données](formats.md). Comparez avec
[les fichiers en Ruby](../ruby/fichiers.md).
