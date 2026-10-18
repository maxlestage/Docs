# Projets pas à pas

Cette page vous guide dans trois petits projets complets en TypeScript, construits étape par étape : un jeu, une liste de tâches enregistrée dans un fichier JSON et un bulletin de notes.

> **Objectifs :**
>
> - découper un programme en petites étapes qui marchent chacune ;
> - décrire les données avec des interfaces et des unions de littéraux ;
> - lire et écrire un fichier JSON avec `node:fs/promises`, et vérifier ce qu'on lit ;
> - transformer du texte CSV en données typées, calculer et afficher un rapport aligné.

## Comment suivre ces projets

Chaque projet avance par petites étapes. À chaque étape :

1. lisez le code et l'explication ;
2. tapez le code dans un fichier (par exemple `jeu.ts`) et lancez-le avec
   `npx tsx jeu.ts` (voir [organiser un projet](projet.md)) ;
3. comparez avec la sortie affichée ;
4. modifiez une valeur pour voir ce qui change.

> **Astuce :** `tsx` ne vérifie pas les types. Gardez le fichier ouvert dans votre
> éditeur pour voir les erreurs soulignées, ou lancez `npx tsc --noEmit --strict jeu.ts`.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des nombres ;
le programme répond « trop petit » ou « trop grand », jusqu'à ce que le joueur trouve.

### Étape 1 : comparer un essai

Commencez par le cœur du jeu : comparer **un** essai au nombre secret.

```typescript
const secret: number = 42;
const essai: number = 30;

if (essai < secret) {
  console.log("Trop petit !");
} else if (essai > secret) {
  console.log("Trop grand !");
} else {
  console.log("Gagné !");
}
```

```sortie
Trop petit !
```

Changez `essai` en `50`, puis en `42`, et relancez.

> **Note :** sans l'annotation `: number`, TypeScript donnerait à `essai` le type
> littéral `30` (voir [les types littéraux](litteraux.md)), et refuserait plus tard de le
> comparer à `42` avec `===` : « ces deux valeurs ne peuvent jamais être égales ».

### Étape 2 : une fonction pour l'indice

Rangez cette comparaison dans une fonction. Elle **renvoie** le texte au lieu de
l'afficher. Son type de retour est une union de trois textes précis : impossible d'en
renvoyer un quatrième par erreur.

```typescript
type Indice = "Trop petit !" | "Trop grand !" | "Gagné !";

function indice(secret: number, essai: number): Indice {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

console.log(indice(42, 30));
console.log(indice(42, 50));
console.log(indice(42, 42));
```

```sortie
Trop petit !
Trop grand !
Gagné !
```

### Étape 3 : plusieurs essais et un compteur

Le joueur fait plusieurs essais. Pour l'instant, ses essais sont dans un tableau : le
programme donne ainsi toujours le même résultat, et vous pouvez le vérifier facilement.
`entries()` donne le numéro (à partir de 0) et la valeur ; `break` arrête la boucle quand
c'est gagné.

```typescript
type Indice = "Trop petit !" | "Trop grand !" | "Gagné !";

function indice(secret: number, essai: number): Indice {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

const secret = 42;
const essais = [50, 25, 40, 42, 10];

for (const [i, essai] of essais.entries()) {
  console.log(`Essai ${i + 1} : ${essai} -> ${indice(secret, essai)}`);
  if (essai === secret) {
    console.log(`Bravo, trouvé en ${i + 1} essais !`);
    break;
  }
}
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo, trouvé en 4 essais !
```

Le dernier essai (`10`) n'est jamais lu : `break` a arrêté la boucle avant.

### Étape 4 : limiter le nombre d'essais

Pour corser le jeu, le joueur n'a droit qu'à 5 essais. Rangez la partie dans une
fonction `jouer` qui renvoie `true` si le joueur gagne. Le paramètre `maximum` a une
valeur par défaut ; `slice(0, maximum)` ne garde que les premiers essais.

```typescript
type Indice = "Trop petit !" | "Trop grand !" | "Gagné !";

function indice(secret: number, essai: number): Indice {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

function jouer(secret: number, essais: readonly number[], maximum = 5): boolean {
  for (const [i, essai] of essais.slice(0, maximum).entries()) {
    console.log(`Essai ${i + 1} : ${essai} -> ${indice(secret, essai)}`);
    if (essai === secret) return true;
  }
  return false;
}

if (jouer(42, [50, 25, 40, 42])) {
  console.log("Bravo !");
}

console.log("---");

if (!jouer(7, [50, 25, 12, 10, 8, 7])) {
  console.log("Perdu ! Le nombre était 7.");
}
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop petit !
Essai 3 : 40 -> Trop petit !
Essai 4 : 42 -> Gagné !
Bravo !
---
Essai 1 : 50 -> Trop grand !
Essai 2 : 25 -> Trop grand !
Essai 3 : 12 -> Trop grand !
Essai 4 : 10 -> Trop grand !
Essai 5 : 8 -> Trop grand !
Perdu ! Le nombre était 7.
```

> **Note :** `return true` quitte la fonction tout de suite, en pleine boucle. Si la
> boucle se termine sans victoire, la fonction arrive à la dernière ligne et renvoie
> `false`. Le paramètre `essais` est `readonly` : `jouer` promet de ne pas le modifier.

### Étape 5 : la vraie version, au clavier

Il reste deux changements pour un vrai jeu :

- `Math.floor(Math.random() * 100) + 1` tire un nombre entier au hasard entre 1 et 100 ;
- le module `node:readline/promises` lit une ligne tapée au clavier ; `await` attend
  la réponse (voir [l'asynchrone](asynchrone.md)).

Ce code attend le clavier : il n'est pas exécuté ici. Lancez-le chez vous avec
`npx tsx jeu.ts`.

```typescript norun
// jeu.ts
import * as readline from "node:readline/promises";
import { stdin as input, stdout as output } from "node:process";

type Indice = "Trop petit !" | "Trop grand !" | "Gagné !";

function indice(secret: number, essai: number): Indice {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

const secret = Math.floor(Math.random() * 100) + 1;
const rl = readline.createInterface({ input, output });
console.log("J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.");

let gagne = false;
for (let i = 1; i <= 5; i++) {
  const reponse = await rl.question(`Essai ${i} : `);
  const essai = Number(reponse);
  if (Number.isNaN(essai)) {
    console.log("Ce n'est pas un nombre.");
    continue;
  }
  console.log(indice(secret, essai));
  if (essai === secret) {
    gagne = true;
    break;
  }
}
rl.close();

if (!gagne) console.log(`Perdu ! Le nombre était ${secret}.`);
```

```text
J'ai choisi un nombre entre 1 et 100. Vous avez 5 essais.
Essai 1 : 50
Trop grand !
Essai 2 : 25
Trop petit !
Essai 3 : 37
Gagné !
```

`rl.question` renvoie toujours une **chaîne** : `Number` la convertit, et
`Number.isNaN` détecte une saisie comme `abc`.

### Pour aller plus loin

- Comptez les essais et affichez « Trouvé en 3 essais ».
- Proposez de rejouer à la fin de la partie.
- Ajoutez un niveau de difficulté : `type Niveau = "facile" | "difficile"`, qui change
  la borne (100 ou 1000) et le nombre d'essais.

## Projet 2 : Une liste de tâches

Un petit gestionnaire de tâches : ajouter une tâche, la marquer comme faite, afficher la
liste. Les tâches sont enregistrées dans un fichier [JSON](../debuter/glossaire.md#json) :
on les retrouve au lancement suivant.

### Étape 1 : décrire une tâche

Commencez par le **type** des données. Une interface décrit une tâche : un numéro, un
titre, et un booléen qui dit si elle est faite (voir [les objets](objets.md)).

```typescript
interface Tache {
  id: number;
  titre: string;
  faite: boolean;
}

const taches: Tache[] = [
  { id: 1, titre: "Acheter du pain", faite: false },
  { id: 2, titre: "Réviser TypeScript", faite: true },
];

for (const t of taches) {
  const coche = t.faite ? "[x]" : "[ ]";
  console.log(`${t.id}. ${coche} ${t.titre}`);
}
```

```sortie
1. [ ] Acheter du pain
2. [x] Réviser TypeScript
```

Essayez d'ajouter une tâche sans `faite`, ou avec `faite: "oui"` : l'éditeur la souligne
aussitôt. L'interface protège toutes les fonctions qui suivent.

### Étape 2 : ajouter, terminer, afficher

Trois fonctions, une par action. Chacune a des paramètres et un type de retour annotés.

```typescript
interface Tache {
  id: number;
  titre: string;
  faite: boolean;
}

function ajouter(taches: Tache[], titre: string): Tache {
  const id = taches.length === 0 ? 1 : Math.max(...taches.map((t) => t.id)) + 1;
  const tache: Tache = { id, titre, faite: false };
  taches.push(tache);
  return tache;
}

function terminer(taches: Tache[], id: number): boolean {
  const tache = taches.find((t) => t.id === id);
  if (tache === undefined) return false;
  tache.faite = true;
  return true;
}

function afficher(taches: readonly Tache[]): void {
  if (taches.length === 0) {
    console.log("Aucune tâche.");
    return;
  }
  for (const t of taches) {
    const coche = t.faite ? "[x]" : "[ ]";
    console.log(`${t.id}. ${coche} ${t.titre}`);
  }
}

const taches: Tache[] = [];
afficher(taches);
ajouter(taches, "Acheter du pain");
ajouter(taches, "Réviser TypeScript");
ajouter(taches, "Appeler Léa");
console.log(terminer(taches, 2));
console.log(terminer(taches, 9));
afficher(taches);
```

```sortie
Aucune tâche.
true
false
1. [ ] Acheter du pain
2. [x] Réviser TypeScript
3. [ ] Appeler Léa
```

Quelques points à remarquer :

- le nouveau numéro est le plus grand numéro existant plus un : il reste unique même si
  on supprime une tâche un jour ;
- `find` renvoie `Tache | undefined` : TypeScript **oblige** à traiter le cas où le
  numéro n'existe pas ;
- `afficher` reçoit un `readonly Tache[]` : elle ne peut pas modifier la liste.

### Étape 3 : enregistrer dans un fichier JSON

Le module `node:fs/promises` lit et écrit des fichiers (voir [les fichiers](fichiers.md)).
Ses fonctions sont **asynchrones** : on les attend avec `await`.

- `JSON.stringify(taches, null, 2)` transforme le tableau en texte JSON lisible ;
- `writeFile` écrit ce texte dans `taches.json` ;
- `readFile` le relit, et `JSON.parse` retransforme le texte en valeurs.

```typescript
import { readFile, writeFile } from "node:fs/promises";

interface Tache {
  id: number;
  titre: string;
  faite: boolean;
}

const FICHIER = "taches.json";

async function enregistrer(taches: readonly Tache[]): Promise<void> {
  await writeFile(FICHIER, JSON.stringify(taches, null, 2) + "\n", "utf8");
}

await enregistrer([
  { id: 1, titre: "Acheter du pain", faite: false },
  { id: 2, titre: "Réviser TypeScript", faite: true },
]);

console.log(await readFile(FICHIER, "utf8"));
```

```sortie
[
  {
    "id": 1,
    "titre": "Acheter du pain",
    "faite": false
  },
  {
    "id": 2,
    "titre": "Réviser TypeScript",
    "faite": true
  }
]
```

Pour relire, une difficulté : `JSON.parse` renvoie `any`. Le fichier a pu être modifié à
la main, ou abîmé. Plutôt que de croire le fichier sur parole, on le range dans une
variable `unknown`, et une **garde de type** `estTache` vérifie chaque élément (voir
[les unions](unions.md)).

Et si le fichier n'existe pas encore (premier lancement) ? `readFile` lance une erreur
de code `ENOENT` (« pas de tel fichier ») : on renvoie alors une liste vide.

```typescript
import { readFile, writeFile } from "node:fs/promises";

interface Tache {
  id: number;
  titre: string;
  faite: boolean;
}

const FICHIER = "taches.json";

function estTache(x: unknown): x is Tache {
  return (
    typeof x === "object" && x !== null &&
    "id" in x && typeof x.id === "number" &&
    "titre" in x && typeof x.titre === "string" &&
    "faite" in x && typeof x.faite === "boolean"
  );
}

async function charger(): Promise<Tache[]> {
  let texte: string;
  try {
    texte = await readFile(FICHIER, "utf8");
  } catch (erreur) {
    if (erreur instanceof Error && "code" in erreur && erreur.code === "ENOENT") {
      return []; // premier lancement : pas encore de fichier
    }
    throw erreur;
  }
  const donnees: unknown = JSON.parse(texte);
  if (!Array.isArray(donnees)) return [];
  return donnees.filter(estTache);
}

console.log(await charger());

await writeFile(FICHIER, '[{"id": 1, "titre": "Lire", "faite": false}, {"id": "deux"}]');
console.log(await charger());
```

```sortie
[]
[ { id: 1, titre: 'Lire', faite: false } ]
```

Le deuxième élément du fichier, abîmé, est simplement ignoré : le reste du programme ne
reçoit que de vraies tâches.

### Étape 4 : le programme complet

Assemblons tout. Une fonction `executer` reçoit une **commande** : une union de trois
formes, distinguées par la propriété `action` (une *union discriminée*). Le `switch`
traite chaque forme, et TypeScript sait, dans chaque `case`, quelles propriétés existent.

```typescript
import { readFile, writeFile } from "node:fs/promises";

interface Tache {
  id: number;
  titre: string;
  faite: boolean;
}

type Commande =
  | { action: "ajouter"; titre: string }
  | { action: "terminer"; id: number }
  | { action: "lister" };

const FICHIER = "taches.json";

function estTache(x: unknown): x is Tache {
  return (
    typeof x === "object" && x !== null &&
    "id" in x && typeof x.id === "number" &&
    "titre" in x && typeof x.titre === "string" &&
    "faite" in x && typeof x.faite === "boolean"
  );
}

async function charger(): Promise<Tache[]> {
  let texte: string;
  try {
    texte = await readFile(FICHIER, "utf8");
  } catch (erreur) {
    if (erreur instanceof Error && "code" in erreur && erreur.code === "ENOENT") {
      return [];
    }
    throw erreur;
  }
  const donnees: unknown = JSON.parse(texte);
  return Array.isArray(donnees) ? donnees.filter(estTache) : [];
}

async function enregistrer(taches: readonly Tache[]): Promise<void> {
  await writeFile(FICHIER, JSON.stringify(taches, null, 2) + "\n", "utf8");
}

function afficher(taches: readonly Tache[]): void {
  if (taches.length === 0) {
    console.log("Aucune tâche.");
    return;
  }
  for (const t of taches) {
    console.log(`${t.id}. ${t.faite ? "[x]" : "[ ]"} ${t.titre}`);
  }
}

async function executer(commande: Commande): Promise<void> {
  const taches = await charger();
  switch (commande.action) {
    case "ajouter": {
      const id = taches.length === 0 ? 1 : Math.max(...taches.map((t) => t.id)) + 1;
      taches.push({ id, titre: commande.titre, faite: false });
      await enregistrer(taches);
      console.log(`Ajouté : ${id}. ${commande.titre}`);
      break;
    }
    case "terminer": {
      const tache = taches.find((t) => t.id === commande.id);
      if (tache === undefined) {
        console.log(`Pas de tâche numéro ${commande.id}.`);
        return;
      }
      tache.faite = true;
      await enregistrer(taches);
      console.log(`Terminé : ${tache.titre}`);
      break;
    }
    case "lister":
      afficher(taches);
      break;
  }
}

await executer({ action: "lister" });
await executer({ action: "ajouter", titre: "Acheter du pain" });
await executer({ action: "ajouter", titre: "Réviser TypeScript" });
await executer({ action: "terminer", id: 1 });
await executer({ action: "terminer", id: 7 });
await executer({ action: "lister" });
```

```sortie
Aucune tâche.
Ajouté : 1. Acheter du pain
Ajouté : 2. Réviser TypeScript
Terminé : Acheter du pain
Pas de tâche numéro 7.
1. [x] Acheter du pain
2. [ ] Réviser TypeScript
```

Chaque appel relit le fichier : les tâches survivent d'un appel à l'autre, exactement
comme elles survivront d'un lancement du programme à l'autre.

> **À retenir :** essayez d'écrire `{ action: "terminer", titre: "pain" }` : `tsc`
> refuse, car la forme `terminer` attend un `id`. Le type `Commande` documente et
> vérifie à la fois toutes les commandes possibles.

### Étape 5 : depuis le terminal avec process.argv

Dernière étape : lire la commande tapée dans le [terminal](../debuter/terminal.md).
`process.argv` contient les mots de la ligne de commande ; les deux premiers sont le
programme `node` et le fichier lancé, d'où le `slice(2)`. Une fonction transforme ces
mots en `Commande`, ou en `undefined` s'ils ne veulent rien dire.

Remplacez les six `await executer(…)` de l'étape 4 par ce code :

```typescript norun
function lireCommande(mots: readonly string[]): Commande | undefined {
  const [action, ...reste] = mots;
  if (action === "ajouter" && reste.length > 0) {
    return { action: "ajouter", titre: reste.join(" ") };
  }
  if (action === "terminer" && reste.length === 1) {
    const id = Number(reste[0]);
    return Number.isInteger(id) ? { action: "terminer", id } : undefined;
  }
  if (action === "lister" || action === undefined) {
    return { action: "lister" };
  }
  return undefined;
}

const commande = lireCommande(process.argv.slice(2));
if (commande === undefined) {
  console.log("Usage : taches.ts [lister | ajouter <titre> | terminer <numéro>]");
  process.exitCode = 1;
} else {
  await executer(commande);
}
```

```sh
npx tsx taches.ts ajouter Arroser les plantes
npx tsx taches.ts ajouter Ranger le bureau
npx tsx taches.ts terminer 1
npx tsx taches.ts
npx tsx taches.ts effacer tout
```

```text
Ajouté : 1. Arroser les plantes
Ajouté : 2. Ranger le bureau
Terminé : Arroser les plantes
1. [x] Arroser les plantes
2. [ ] Ranger le bureau
Usage : taches.ts [lister | ajouter <titre> | terminer <numéro>]
```

### Pour aller plus loin

- Ajoutez une commande `supprimer <numéro>` : une quatrième forme dans `Commande`, un
  quatrième `case`.
- Ajoutez une date limite facultative : `echeance?: string` dans `Tache`.
- Rangez le code en modules : `taches.ts` pour les types et les fonctions, `main.ts`
  pour la ligne de commande (voir [les modules](modules.md)).

## Projet 3 : Le bulletin de notes

Les notes d'une classe arrivent sous forme de texte CSV (*valeurs séparées par des
virgules*), comme un export de tableur. Le programme calcule la moyenne de chaque élève
et de chaque matière, trouve le meilleur élève et affiche un bulletin bien aligné.

### Étape 1 : lire les données

Une ligne CSV donne une note : élève, matière, valeur. Le texte est découpé en lignes
avec `split("\n")`, chaque ligne en morceaux avec `split(",")`. La première ligne est
l'en-tête : on la saute avec `slice(1)`.

```typescript
interface Note {
  eleve: string;
  matiere: string;
  valeur: number;
}

const CSV = `eleve,matiere,note
Alice,maths,15
Alice,français,12
Alice,histoire,17
Bob,maths,9
Bob,français,14
Bob,histoire,11
Chloé,maths,18
Chloé,français,16
Chloé,histoire,13`;

function lireNotes(csv: string): Note[] {
  const notes: Note[] = [];
  for (const ligne of csv.trim().split("\n").slice(1)) {
    const [eleve, matiere, texte] = ligne.split(",");
    const valeur = Number(texte);
    if (eleve && matiere && !Number.isNaN(valeur)) {
      notes.push({ eleve, matiere, valeur });
    }
  }
  return notes;
}

const notes = lireNotes(CSV);
console.log(notes.length, "notes");
console.log(notes[0]);
```

```sortie
9 notes
{ eleve: 'Alice', matiere: 'maths', valeur: 15 }
```

Les morceaux sont des **chaînes** : `"15"` n'est pas `15`. `Number` fait la conversion,
et les lignes incomplètes ou mal écrites sont ignorées.

### Étape 2 : la moyenne de chaque élève

Une `Map` range les notes par élève : la clé est le nom, la valeur un tableau de notes
(voir [les collections](collections.md)). Une petite fonction générique `grouper` fera
le travail pour les élèves comme pour les matières (voir [les génériques](generiques.md)).

```typescript
interface Note {
  eleve: string;
  matiere: string;
  valeur: number;
}

const notes: Note[] = [
  { eleve: "Alice", matiere: "maths", valeur: 15 },
  { eleve: "Alice", matiere: "français", valeur: 12 },
  { eleve: "Alice", matiere: "histoire", valeur: 17 },
  { eleve: "Bob", matiere: "maths", valeur: 9 },
  { eleve: "Bob", matiere: "français", valeur: 14 },
  { eleve: "Bob", matiere: "histoire", valeur: 11 },
];

function grouper<T>(elements: readonly T[], cle: (e: T) => string): Map<string, T[]> {
  const groupes = new Map<string, T[]>();
  for (const e of elements) {
    const k = cle(e);
    const groupe = groupes.get(k) ?? [];
    groupe.push(e);
    groupes.set(k, groupe);
  }
  return groupes;
}

function moyenne(valeurs: readonly number[]): number {
  return valeurs.reduce((a, b) => a + b, 0) / valeurs.length;
}

for (const [eleve, sesNotes] of grouper(notes, (n) => n.eleve)) {
  const m = moyenne(sesNotes.map((n) => n.valeur));
  console.log(`${eleve} : ${m.toFixed(2)}`);
}
```

```sortie
Alice : 14.67
Bob : 11.33
```

`toFixed(2)` arrondit à deux décimales pour l'affichage. Une `Map` garde l'ordre
d'insertion : les élèves sortent dans l'ordre du fichier.

### Étape 3 : le meilleur élève et les moyennes par matière

On transforme chaque groupe en un petit objet `Moyenne`, puis on cherche le maximum
avec `reduce`. Le même `grouper` donne les moyennes par matière.

```typescript
interface Note {
  eleve: string;
  matiere: string;
  valeur: number;
}

interface Moyenne {
  nom: string;
  moyenne: number;
}

const notes: Note[] = [
  { eleve: "Alice", matiere: "maths", valeur: 15 },
  { eleve: "Alice", matiere: "français", valeur: 12 },
  { eleve: "Bob", matiere: "maths", valeur: 9 },
  { eleve: "Bob", matiere: "français", valeur: 14 },
  { eleve: "Chloé", matiere: "maths", valeur: 18 },
  { eleve: "Chloé", matiere: "français", valeur: 16 },
];

function grouper<T>(elements: readonly T[], cle: (e: T) => string): Map<string, T[]> {
  const groupes = new Map<string, T[]>();
  for (const e of elements) {
    const groupe = groupes.get(cle(e)) ?? [];
    groupe.push(e);
    groupes.set(cle(e), groupe);
  }
  return groupes;
}

function moyennes(notes: readonly Note[], cle: (n: Note) => string): Moyenne[] {
  return [...grouper(notes, cle)].map(([nom, groupe]) => ({
    nom,
    moyenne: groupe.reduce((s, n) => s + n.valeur, 0) / groupe.length,
  }));
}

const parEleve = moyennes(notes, (n) => n.eleve);
const meilleur = parEleve.reduce((a, b) => (b.moyenne > a.moyenne ? b : a));
console.log(`Meilleur élève : ${meilleur.nom} (${meilleur.moyenne})`);

for (const m of moyennes(notes, (n) => n.matiere)) {
  console.log(`${m.nom} : ${m.moyenne.toFixed(2)}`);
}
```

```sortie
Meilleur élève : Chloé (17)
maths : 14.00
français : 14.00
```

> **Note :** `reduce` sans valeur de départ prend le premier élément comme départ. Sur
> un tableau vide, il lancerait une erreur : ici, il y a forcément au moins un élève.

### Étape 4 : un bulletin bien aligné

Pour aligner des colonnes, `padEnd(10)` complète un texte avec des espaces à droite
jusqu'à 10 caractères, et `padStart(7)` à gauche (les nombres s'alignent à droite).
Une union de littéraux décrit les mentions possibles.

```typescript
interface Note {
  eleve: string;
  matiere: string;
  valeur: number;
}

interface Moyenne {
  nom: string;
  moyenne: number;
}

type Mention = "Très bien" | "Bien" | "Assez bien" | "Passable" | "Insuffisant";

const CSV = `eleve,matiere,note
Alice,maths,15
Alice,français,12
Alice,histoire,17
Bob,maths,9
Bob,français,14
Bob,histoire,11
Chloé,maths,18
Chloé,français,16
Chloé,histoire,13`;

function lireNotes(csv: string): Note[] {
  const notes: Note[] = [];
  for (const ligne of csv.trim().split("\n").slice(1)) {
    const [eleve, matiere, texte] = ligne.split(",");
    const valeur = Number(texte);
    if (eleve && matiere && !Number.isNaN(valeur)) {
      notes.push({ eleve, matiere, valeur });
    }
  }
  return notes;
}

function moyennes(notes: readonly Note[], cle: (n: Note) => string): Moyenne[] {
  const groupes = new Map<string, number[]>();
  for (const n of notes) {
    const valeurs = groupes.get(cle(n)) ?? [];
    valeurs.push(n.valeur);
    groupes.set(cle(n), valeurs);
  }
  return [...groupes].map(([nom, valeurs]) => ({
    nom,
    moyenne: valeurs.reduce((a, b) => a + b, 0) / valeurs.length,
  }));
}

function mention(moyenne: number): Mention {
  if (moyenne >= 16) return "Très bien";
  if (moyenne >= 14) return "Bien";
  if (moyenne >= 12) return "Assez bien";
  if (moyenne >= 10) return "Passable";
  return "Insuffisant";
}

function bulletin(csv: string): void {
  const notes = lireNotes(csv);
  const eleves = moyennes(notes, (n) => n.eleve);

  console.log(`${"Élève".padEnd(10)}${"Moyenne".padStart(7)}  Mention`);
  console.log("-".repeat(28));
  for (const e of eleves) {
    console.log(`${e.nom.padEnd(10)}${e.moyenne.toFixed(2).padStart(7)}  ${mention(e.moyenne)}`);
  }

  console.log();
  for (const m of moyennes(notes, (n) => n.matiere)) {
    console.log(`${m.nom.padEnd(10)}${m.moyenne.toFixed(2).padStart(7)}`);
  }

  const meilleur = eleves.reduce((a, b) => (b.moyenne > a.moyenne ? b : a));
  console.log();
  console.log(`Meilleur élève : ${meilleur.nom}`);
}

bulletin(CSV);
```

```sortie
Élève     Moyenne  Mention
----------------------------
Alice       14.67  Bien
Bob         11.33  Passable
Chloé       15.67  Bien

maths       14.00
français    14.00
histoire    13.67

Meilleur élève : Chloé
```

Chaque fonction fait une seule chose : `lireNotes` lit, `moyennes` calcule, `mention`
classe, `bulletin` affiche. On peut tester chacune séparément (voir [les tests](tests.md)).

### Pour aller plus loin

- Lisez le CSV depuis un fichier `notes.csv` avec `readFile`.
- Affichez, pour chaque élève, sa meilleure et sa moins bonne matière.
- Signalez les lignes ignorées par `lireNotes`, avec leur numéro.

## Erreurs fréquentes

### Oublier de convertir un morceau de texte en nombre

```typescript norun
const ligne = "Alice,maths,15";
const morceaux = ligne.split(",");
const note: number = morceaux[2];
```

```text
main.ts(3,7): error TS2322: Type 'string' is not assignable to type 'number'.
```

`split` renvoie des chaînes. Convertissez avec `Number(morceaux[2])`, puis vérifiez le
résultat avec `Number.isNaN`.

### Oublier await devant readFile

```typescript norun
import { readFile } from "node:fs/promises";

const texte = readFile("taches.json", "utf8");
const taches = JSON.parse(texte);
```

```text
main.ts(4,27): error TS2345: Argument of type 'Promise<string>' is not assignable to parameter of type 'string'.
```

Sans `await`, `texte` n'est pas le contenu du fichier mais une **promesse** de contenu.
Écrivez `const texte = await readFile("taches.json", "utf8");`.

### Lire un fichier qui n'existe pas encore

```typescript norun
import { readFile } from "node:fs/promises";

const texte = await readFile("taches.json", "utf8");
console.log(texte);
```

```text
Error: ENOENT: no such file or directory, open 'taches.json'
    at async open (node:internal/fs/promises:636:25)
    at async readFile (node:internal/fs/promises:1235:14) {
  errno: -2,
  code: 'ENOENT',
  syscall: 'open',
  path: 'taches.json'
}
```

Au premier lancement, le fichier n'existe pas. Entourez la lecture d'un `try`/`catch`
et renvoyez une liste vide quand le code est `ENOENT`, comme la fonction `charger` de
l'étape 3.

## Testez-vous

**Question 1 :** pourquoi ranger le résultat de `JSON.parse` dans une variable `unknown`
plutôt que de le typer directement en `Tache[]` ?

> **Réponse**
>
> Parce que rien ne garantit que le fichier contient vraiment des tâches. Avec
> `unknown`, TypeScript oblige à vérifier chaque élément (ici avec la garde `estTache`)
> avant de s'en servir. Un `as Tache[]` ferait taire le compilateur sans rien vérifier.

**Question 2 :** dans le juste nombre, à quoi sert le type
`"Trop petit !" | "Trop grand !" | "Gagné !"` ?

> **Réponse**
>
> Il liste exactement les trois réponses possibles de la fonction `indice`. Une faute de
> frappe dans un texte renvoyé devient une erreur de type, et l'éditeur propose ces
> trois valeurs quand on compare le résultat.

**Question 3 :** que fait ce code ?

```typescript
console.log("Bob".padEnd(6) + "|");
console.log("9.5".padStart(6) + "|");
```

> **Réponse**
>
> `padEnd` ajoute des espaces à droite, `padStart` à gauche, jusqu'à 6 caractères :
>
> ```sortie
> Bob   |
>    9.5|
> ```

## En résumé

- Construisez un programme par petites étapes, en le lançant à chaque fois.
- Décrivez d'abord les données : une interface (`Tache`, `Note`), des unions de
  littéraux (`Indice`, `Mention`), une union discriminée (`Commande`).
- Les données lues (clavier, fichier, CSV) arrivent en texte ou en `unknown` :
  convertissez et vérifiez avant de vous en servir.
- `node:fs/promises` lit et écrit les fichiers avec `await` ; `ENOENT` signale un
  fichier absent.
- `padEnd` et `padStart` alignent les colonnes d'un rapport.

Suite : [exercices corrigés](exercices.md).
