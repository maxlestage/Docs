# Projets pas à pas

Cette page vous guide dans trois petits projets JavaScript complets, construits étape par étape : un jeu, une liste de tâches enregistrée dans un fichier JSON et un bulletin de notes.

> **Objectifs :**
>
> - découper un programme en petites étapes qui marchent chacune ;
> - réutiliser ce que vous avez appris : conditions, boucles, fonctions, tableaux, objets ;
> - enregistrer des données dans un fichier JSON avec `node:fs/promises` ;
> - lire des données en texte, calculer et afficher un rapport bien aligné.

## Comment suivre ces projets

Chaque projet avance par petites étapes. À chaque étape :

1. lisez le code et l'explication ;
2. tapez le code dans un fichier (par exemple `jeu.mjs`) et lancez-le avec
   `node jeu.mjs` ;
3. comparez avec la sortie affichée ;
4. modifiez une valeur pour voir ce qui change.

> **Astuce :** l'extension `.mjs` indique à Node que le fichier utilise `import`. Dans
> un projet avec `"type": "module"` (voir [organiser un projet](projet.md)), `.js`
> suffit.

Ne copiez pas tout d'un coup. Un programme se construit morceau par morceau, en le
lançant souvent. C'est ainsi que travaillent les programmeurs.

## Projet 1 : Le juste nombre

Le programme choisit un nombre secret entre 1 et 100. Le joueur propose des nombres ;
le programme répond « trop petit » ou « trop grand », jusqu'à ce que le joueur trouve.

### Étape 1 : comparer un essai

Commencez par le cœur du jeu : comparer **un** essai au nombre secret.

```javascript
const secret = 42;
const essai = 30;

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

### Étape 2 : une fonction pour l'indice

Rangez cette comparaison dans une fonction. Elle **renvoie** le texte au lieu de
l'afficher : elle sera plus facile à réutiliser et à tester.

```javascript
function indice(secret, essai) {
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
`entries()` donne le numéro et la valeur de chaque essai ; `break` arrête la boucle
quand c'est gagné.

```javascript
function indice(secret, essai) {
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
fonction `jouer` qui renvoie `true` si le joueur gagne. `slice(0, maximum)` ne garde
que les premiers essais.

```javascript
function indice(secret, essai) {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

function jouer(secret, essais, maximum = 5) {
  for (const [i, essai] of essais.slice(0, maximum).entries()) {
    console.log(`Essai ${i + 1} : ${essai} -> ${indice(secret, essai)}`);
    if (essai === secret) return true;
  }
  return false;
}

if (jouer(42, [50, 42])) {
  console.log("Partie 1 : gagnée");
}
if (!jouer(7, [1, 2, 3, 4, 5, 6, 7])) {
  console.log("Partie 2 : perdue, le nombre était 7");
}
```

```sortie
Essai 1 : 50 -> Trop grand !
Essai 2 : 42 -> Gagné !
Partie 1 : gagnée
Essai 1 : 1 -> Trop petit !
Essai 2 : 2 -> Trop petit !
Essai 3 : 3 -> Trop petit !
Essai 4 : 4 -> Trop petit !
Essai 5 : 5 -> Trop petit !
Partie 2 : perdue, le nombre était 7
```

### Étape 5 : la vraie version, au clavier

Il reste deux changements pour jouer pour de vrai :

- `Math.random()` tire un nombre au hasard entre 0 et 1 ; `Math.floor(Math.random() * 100) + 1`
  donne un entier de 1 à 100 ;
- le module `node:readline/promises` lit ce que le joueur tape, avec `await rl.question(…)`.

Ce code lit le clavier et tire au hasard : il n'est pas exécuté ici.

```javascript norun
import * as readline from "node:readline/promises";
import { stdin as input, stdout as output } from "node:process";

function indice(secret, essai) {
  if (essai < secret) return "Trop petit !";
  if (essai > secret) return "Trop grand !";
  return "Gagné !";
}

const secret = Math.floor(Math.random() * 100) + 1;
const rl = readline.createInterface({ input, output });
const maximum = 5;
let gagne = false;

for (let i = 1; i <= maximum && !gagne; i++) {
  const reponse = await rl.question(`Essai ${i} : `);
  const essai = Number(reponse);
  if (Number.isNaN(essai)) {
    console.log("Ce n'est pas un nombre.");
    continue;
  }
  console.log(indice(secret, essai));
  gagne = essai === secret;
}

console.log(gagne ? "Bravo !" : `Perdu, le nombre était ${secret}.`);
rl.close();
```

Une partie ressemble à ceci :

```text
Essai 1 : 50
Trop grand !
Essai 2 : 25
Trop petit !
Essai 3 : 37
Trop grand !
Essai 4 : 31
Gagné !
Bravo !
```

> **Note :** `rl.question` renvoie toujours du **texte**. Sans `Number(...)`,
> `"42" === 42` est faux et le joueur ne gagnerait jamais.

### Pour aller plus loin

- Demandez au joueur s'il veut rejouer, et entourez la partie d'une boucle `while`.
- Refusez un essai hors de 1 à 100 sans le compter.
- Retenez le meilleur score (le plus petit nombre d'essais) dans un fichier.
- Écrivez des tests pour `indice` avec `node:test` (voir [tester son code](tests.md)).

## Projet 2 : Une liste de tâches

Une petite application en ligne de commande : ajouter une tâche, afficher la liste,
marquer une tâche comme faite. Les tâches sont enregistrées dans un fichier
[JSON](../debuter/glossaire.md#json) : elles sont toujours là au prochain lancement.

### Étape 1 : représenter les tâches

Une tâche est un objet avec deux propriétés : son `titre` et `faite` (vrai ou faux).
La liste est un tableau de ces objets.

```javascript
const taches = [
  { titre: "Acheter du pain", faite: false },
  { titre: "Appeler Léa", faite: true },
];

for (const [i, tache] of taches.entries()) {
  const caseACocher = tache.faite ? "[x]" : "[ ]";
  console.log(`${i + 1}. ${caseACocher} ${tache.titre}`);
}
```

```sortie
1. [ ] Acheter du pain
2. [x] Appeler Léa
```

### Étape 2 : ajouter, afficher, terminer

Chaque action devient une fonction. `afficher` gère aussi la liste vide.

```javascript
function ajouter(taches, titre) {
  taches.push({ titre, faite: false });
}

function terminer(taches, numero) {
  const tache = taches[numero - 1];
  if (!tache) {
    console.log(`Pas de tâche numéro ${numero}.`);
    return;
  }
  tache.faite = true;
}

function afficher(taches) {
  if (taches.length === 0) {
    console.log("Aucune tâche.");
    return;
  }
  for (const [i, tache] of taches.entries()) {
    const caseACocher = tache.faite ? "[x]" : "[ ]";
    console.log(`${i + 1}. ${caseACocher} ${tache.titre}`);
  }
}

const taches = [];
afficher(taches);
ajouter(taches, "Acheter du pain");
ajouter(taches, "Réviser JavaScript");
terminer(taches, 1);
terminer(taches, 5);
afficher(taches);
```

```sortie
Aucune tâche.
Pas de tâche numéro 5.
1. [x] Acheter du pain
2. [ ] Réviser JavaScript
```

> **Attention :** l'utilisateur compte à partir de 1, JavaScript à partir de 0. D'où
> `taches[numero - 1]`.

### Étape 3 : enregistrer dans un fichier JSON

Pour garder les tâches, écrivez-les dans `taches.json` avec le module
`node:fs/promises` (voir [les fichiers](fichiers.md)) :

- `JSON.stringify(taches, null, 2)` transforme le tableau en texte JSON, indenté de
  2 espaces ;
- `writeFile` écrit ce texte dans le fichier ;
- `readFile` relit le fichier, et `JSON.parse` retransforme le texte en tableau.

Au premier lancement, le fichier n'existe pas encore : `readFile` lève une erreur dont
le code est `"ENOENT"`. On l'attrape et on part d'une liste vide.

```javascript
import { readFile, writeFile } from "node:fs/promises";

const FICHIER = "taches.json";

async function charger() {
  try {
    const texte = await readFile(FICHIER, "utf8");
    return JSON.parse(texte);
  } catch (erreur) {
    if (erreur.code === "ENOENT") return [];
    throw erreur;
  }
}

async function sauver(taches) {
  await writeFile(FICHIER, JSON.stringify(taches, null, 2));
}

const avant = await charger();
console.log("Au départ :", avant);

await sauver([{ titre: "Acheter du pain", faite: false }]);
console.log(await readFile(FICHIER, "utf8"));

const apres = await charger();
console.log("Relu :", apres[0].titre);
```

```sortie
Au départ : []
[
  {
    "titre": "Acheter du pain",
    "faite": false
  }
]
Relu : Acheter du pain
```

> **Note :** `throw erreur` relance les autres erreurs (un fichier JSON abîmé, par
> exemple). On n'avale que l'erreur prévue (voir [les erreurs](erreurs.md)).

### Étape 4 : le programme complet

Assemblez les morceaux. Une fonction `executer` reçoit une commande et ses arguments,
comme on les taperait dans le terminal. Pour vérifier le programme, une liste fixe de
commandes simule trois lancements successifs ; le fichier garde les tâches entre deux.

```javascript
import { readFile, writeFile } from "node:fs/promises";

const FICHIER = "taches.json";

async function charger() {
  try {
    return JSON.parse(await readFile(FICHIER, "utf8"));
  } catch (erreur) {
    if (erreur.code === "ENOENT") return [];
    throw erreur;
  }
}

async function sauver(taches) {
  await writeFile(FICHIER, JSON.stringify(taches, null, 2));
}

function afficher(taches) {
  if (taches.length === 0) {
    console.log("Aucune tâche.");
    return;
  }
  for (const [i, tache] of taches.entries()) {
    const caseACocher = tache.faite ? "[x]" : "[ ]";
    console.log(`${i + 1}. ${caseACocher} ${tache.titre}`);
  }
}

async function executer(commande, ...args) {
  const taches = await charger();
  switch (commande) {
    case "ajouter": {
      const titre = args.join(" ");
      taches.push({ titre, faite: false });
      console.log(`Ajoutée : ${titre}`);
      break;
    }
    case "faite": {
      const tache = taches[Number(args[0]) - 1];
      if (!tache) {
        console.log(`Pas de tâche numéro ${args[0]}.`);
        return;
      }
      tache.faite = true;
      console.log(`Terminée : ${tache.titre}`);
      break;
    }
    case "liste":
      afficher(taches);
      return;
    default:
      console.log("Commandes : ajouter <titre>, faite <numéro>, liste");
      return;
  }
  await sauver(taches);
}

await executer("liste");
await executer("ajouter", "Acheter", "du", "pain");
await executer("ajouter", "Réviser", "JavaScript");
await executer("faite", "1");
await executer("faite", "9");
await executer("liste");
await executer("supprimer");
```

```sortie
Aucune tâche.
Ajoutée : Acheter du pain
Ajoutée : Réviser JavaScript
Terminée : Acheter du pain
Pas de tâche numéro 9.
1. [x] Acheter du pain
2. [ ] Réviser JavaScript
Commandes : ajouter <titre>, faite <numéro>, liste
```

Chaque appel relit le fichier : c'est exactement ce qui se passe quand on relance le
programme.

### Étape 5 : depuis le terminal avec process.argv

Pour utiliser le vrai terminal, remplacez la liste d'appels par une seule ligne.
`process.argv` contient les mots tapés après `node taches.mjs` à partir de l'indice 2
(voir [la ligne de commande](ligne-de-commande.md)) :

```javascript norun
await executer(...process.argv.slice(2));
```

```sh
node taches.mjs ajouter Acheter du pain
node taches.mjs ajouter Réviser JavaScript
node taches.mjs faite 1
node taches.mjs liste
```

```text
Ajoutée : Acheter du pain
Ajoutée : Réviser JavaScript
Terminée : Acheter du pain
1. [x] Acheter du pain
2. [ ] Réviser JavaScript
```

### Pour aller plus loin

- Ajoutez une commande `supprimer <numéro>` avec `splice`.
- Ajoutez une date limite à chaque tâche et triez la liste par date.
- Affichez le nombre de tâches restantes : `taches.filter((t) => !t.faite).length`.

## Projet 3 : Le bulletin de notes

Le programme lit des notes écrites en texte, au format **CSV** (des valeurs séparées
par des virgules), calcule les moyennes et affiche un bulletin bien aligné.

### Étape 1 : lire les données

La première ligne donne les noms des colonnes ; chaque ligne suivante, un élève.
`trim()` enlève les sauts de ligne en trop, `split("\n")` coupe en lignes, puis
`split(",")` coupe chaque ligne en cases.

```javascript
const CSV = `
nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13
`;

const [entete, ...lignes] = CSV.trim().split("\n");
const matieres = entete.split(",").slice(1);

const eleves = lignes.map((ligne) => {
  const [nom, ...notes] = ligne.split(",");
  return { nom, notes: notes.map(Number) };
});

console.log(matieres);
console.log(eleves[0]);
```

```sortie
[ 'maths', 'francais', 'histoire' ]
{ nom: 'Alice', notes: [ 15, 12, 17 ] }
```

> **Attention :** `split` donne des **textes**. `notes.map(Number)` les convertit en
> nombres. Sans cela, les additions colleraient les textes.

### Étape 2 : la moyenne de chaque élève

Une fonction `moyenne` additionne avec `reduce` puis divise. `toFixed(1)` arrondit à
un chiffre après la virgule, pour l'affichage.

```javascript
function moyenne(nombres) {
  return nombres.reduce((somme, n) => somme + n, 0) / nombres.length;
}

const eleves = [
  { nom: "Alice", notes: [15, 12, 17] },
  { nom: "Bob", notes: [9, 14, 11] },
  { nom: "Chloé", notes: [18, 16, 13] },
];

for (const eleve of eleves) {
  console.log(`${eleve.nom} : ${moyenne(eleve.notes).toFixed(1)}`);
}
```

```sortie
Alice : 14.7
Bob : 11.3
Chloé : 15.7
```

### Étape 3 : le meilleur élève et les moyennes par matière

Le meilleur élève : on garde, avec `reduce`, l'élève dont la moyenne est la plus
haute. Pour une matière, on prend la note à la même position chez chaque élève.

```javascript
function moyenne(nombres) {
  return nombres.reduce((somme, n) => somme + n, 0) / nombres.length;
}

const matieres = ["maths", "francais", "histoire"];
const eleves = [
  { nom: "Alice", notes: [15, 12, 17] },
  { nom: "Bob", notes: [9, 14, 11] },
  { nom: "Chloé", notes: [18, 16, 13] },
];

const meilleur = eleves.reduce((a, b) => (moyenne(b.notes) > moyenne(a.notes) ? b : a));
console.log(`Meilleure moyenne : ${meilleur.nom}`);

for (const [i, matiere] of matieres.entries()) {
  const notes = eleves.map((eleve) => eleve.notes[i]);
  console.log(`${matiere} : ${moyenne(notes).toFixed(1)}`);
}
```

```sortie
Meilleure moyenne : Chloé
maths : 14.0
francais : 14.0
histoire : 13.7
```

### Étape 4 : un bulletin bien aligné

Pour former des colonnes, chaque case doit avoir la même largeur. `padEnd(10)`
complète un texte avec des espaces à droite ; `padStart(8)` à gauche, pour aligner
les nombres (voir [le texte](texte.md)).

```javascript
const CSV = `
nom,maths,francais,histoire
Alice,15,12,17
Bob,9,14,11
Chloé,18,16,13
`;

function moyenne(nombres) {
  return nombres.reduce((somme, n) => somme + n, 0) / nombres.length;
}

function lireCSV(texte) {
  const [entete, ...lignes] = texte.trim().split("\n");
  const matieres = entete.split(",").slice(1);
  const eleves = lignes.map((ligne) => {
    const [nom, ...notes] = ligne.split(",");
    return { nom, notes: notes.map(Number) };
  });
  return { matieres, eleves };
}

function ligne(nom, nombres) {
  const cases = nombres.map((n) => n.toFixed(1).padStart(10));
  return nom.padEnd(8) + cases.join("");
}

const { matieres, eleves } = lireCSV(CSV);

console.log("Élève".padEnd(8) + [...matieres, "moyenne"].map((m) => m.padStart(10)).join(""));
console.log("-".repeat(48));
for (const eleve of eleves) {
  console.log(ligne(eleve.nom, [...eleve.notes, moyenne(eleve.notes)]));
}
console.log("-".repeat(48));
const parMatiere = matieres.map((_, i) => moyenne(eleves.map((e) => e.notes[i])));
console.log(ligne("Classe", [...parMatiere, moyenne(parMatiere)]));
```

```sortie
Élève        maths  francais  histoire   moyenne
------------------------------------------------
Alice         15.0      12.0      17.0      14.7
Bob            9.0      14.0      11.0      11.3
Chloé         18.0      16.0      13.0      15.7
------------------------------------------------
Classe        14.0      14.0      13.7      13.9
```

Le programme est découpé en petites fonctions : `lireCSV` lit, `moyenne` calcule,
`ligne` met en forme. Chacune se comprend et se teste seule.

### Pour aller plus loin

- Lisez le CSV depuis un fichier `notes.csv` avec `readFile`.
- Ajoutez une colonne « mention » (`très bien` à partir de 16, `bien` à partir de 14…).
- Triez les élèves par moyenne décroissante avec `toSorted`.

## Erreurs fréquentes

### Oublier Number après split

```javascript norun
const notes = "12,15,9".split(",");
const somme = notes.reduce((total, n) => total + n, 0);
console.log(somme);
```

```text
012159
```

`split` renvoie des textes, et `+` colle des textes. Convertissez d'abord :
`"12,15,9".split(",").map(Number)`.

### Oublier await devant readFile

```javascript norun
import { readFile, writeFile } from "node:fs/promises";

await writeFile("taches.json", "[]");
const texte = readFile("taches.json", "utf8");
console.log(texte);
console.log(JSON.parse(texte));
```

```text
Promise { <pending> }
<anonymous_script>:1
[object Promise]
 ^

SyntaxError: Unexpected token 'o', "[object Promise]" is not valid JSON
```

`readFile` renvoie une **promesse**, pas le texte. Écrivez
`const texte = await readFile("taches.json", "utf8");` (voir
[l'asynchrone](asynchrone.md)).

### Lire un fichier qui n'existe pas encore

```javascript norun
import { readFile } from "node:fs/promises";

const texte = await readFile("taches.json", "utf8");
console.log(JSON.parse(texte));
```

```text
Error: ENOENT: no such file or directory, open 'taches.json'
    at async open (node:internal/fs/promises:636:25)
    at async readFile (node:internal/fs/promises:1235:14)
    at async file:///home/alice/taches/taches.mjs:3:15 {
  errno: -2,
  code: 'ENOENT',
  syscall: 'open',
  path: 'taches.json'
}
```

Au premier lancement, le fichier n'existe pas. Entourez la lecture d'un
`try … catch` et renvoyez `[]` quand `erreur.code === "ENOENT"`, comme à l'étape 3.

## Testez-vous

**Question 1 :** pourquoi la fonction `indice` renvoie-t-elle un texte au lieu de
l'afficher avec `console.log` ?

> **Réponse**
>
> Une fonction qui renvoie une valeur se réutilise (on peut l'afficher, la comparer, la
> combiner) et se teste facilement avec `assert.strictEqual`.

**Question 2 :** que fait `JSON.stringify(taches, null, 2)` ?

> **Réponse**
>
> Il transforme le tableau `taches` en texte JSON. Le `2` indente le texte de deux
> espaces par niveau, pour qu'il soit lisible dans le fichier.

**Question 3 :** affichez `Bob` sur 8 caractères suivi de la note `9` alignée à droite
sur 5 caractères, avec un chiffre après la virgule.

> **Réponse**
>
> ```javascript
> console.log("Bob".padEnd(8) + (9).toFixed(1).padStart(5) + "|");
> ```
>
> ```sortie
> Bob       9.0|
> ```
>
> La barre `|` montre où s'arrête la ligne.

## En résumé

- Construisez un programme par petites étapes, en le lançant à chaque fois.
- Des données fixes (une liste d'essais, de commandes) rendent le programme facile à
  vérifier ; le clavier et le hasard viennent à la fin.
- `JSON.stringify` et `JSON.parse`, avec `readFile` et `writeFile` de
  `node:fs/promises`, enregistrent des données d'un lancement à l'autre.
- `split`, `map(Number)` et `reduce` transforment un texte CSV en calculs.
- `padEnd`, `padStart` et `toFixed` alignent un rapport en colonnes.

Suite : [exercices corrigés](exercices.md). Comparez avec
[les projets en Ruby](../ruby/projets.md).
