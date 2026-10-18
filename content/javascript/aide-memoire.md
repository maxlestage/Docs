# Aide-mémoire

Cette page résume en un coup d'œil la syntaxe JavaScript vue dans tout le parcours, avec Node.

## Bases

| Élément | Exemple |
| --- | --- |
| Constante | `const age = 30;` |
| Variable qui change | `let total = 0;` |
| Texte | `"Bonjour"`, `'Bonjour'` |
| Gabarit | `` `Bonjour ${nom}` `` |
| Booléens | `true`, `false` |
| Absence | `undefined` (pas de valeur), `null` (vide exprès) |
| Commentaire | `// une ligne`, `/* plusieurs */` |
| Afficher | `console.log(a, b)` |
| Lancer | `node main.mjs` |

```javascript
const nom = "Alice";
let age = 30;
age += 1;
console.log(`Bonjour ${nom}, ${age} ans`);
console.log(typeof nom, typeof age, typeof undefined, typeof null);
```

```sortie
Bonjour Alice, 31 ans
string number undefined object
```

Détails : [premiers pas](premiers-pas.md) et [les types](types.md).

## Nombres

| Besoin | Syntaxe |
| --- | --- |
| Opérations | `+ - * / ** %` |
| Division entière | `Math.trunc(7 / 2)` |
| Arrondir | `Math.round(x)`, `x.toFixed(2)` (texte) |
| Texte vers nombre | `Number("12")`, `parseInt("12px")` |
| Pas un nombre | `Number.isNaN(x)` |
| Grands entiers | `10n ** 20n` (BigInt) |

```javascript
console.log(7 / 2, Math.trunc(7 / 2), 7 % 2, 2 ** 10);
console.log((0.1 + 0.2).toFixed(2), Number("12"), parseInt("12px"));
console.log(Number("abc"), Number.isNaN(Number("abc")));
```

```sortie
3.5 3 1 1024
0.30 12 12
NaN true
```

Détails : [les nombres](nombres.md).

## Conditions et boucles

| Besoin | Syntaxe |
| --- | --- |
| Condition | `if (…) { … } else if (…) { … } else { … }` |
| Égalité | `===`, `!==` (jamais `==`) |
| Logique | `&&`, `\|\|`, `!` |
| Ternaire | `x > 0 ? "oui" : "non"` |
| Plusieurs cas | `switch (x) { case 1: …; break; default: … }` |
| Boucles | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of liste)` |
| Contrôle | `break`, `continue` |

```javascript
for (let n = 1; n <= 5; n++) {
  if (n === 2) continue;
  console.log(n % 2 === 0 ? `${n} pair` : `${n} impair`);
}
```

```sortie
1 impair
3 impair
4 pair
5 impair
```

Détails : [conditions et boucles](conditions-boucles.md).

## Fonctions

| Forme | Syntaxe |
| --- | --- |
| Déclaration | `function carre(x) { return x * x; }` |
| Fléchée | `const carre = (x) => x * x;` |
| Valeur par défaut | `function saluer(nom = "toi") { … }` |
| Reste des arguments | `function somme(...nombres) { … }` |
| Paramètre objet | `function creer({ nom, age }) { … }` |

```javascript
const carre = (x) => x * x;
function saluer(nom = "toi") {
  return `Salut ${nom} !`;
}
function somme(...nombres) {
  return nombres.reduce((total, n) => total + n, 0);
}
console.log(carre(4), saluer(), saluer("Léa"), somme(1, 2, 3));
```

```sortie
16 Salut toi ! Salut Léa ! 6
```

Détails : [les fonctions](fonctions.md).

## Texte

| Besoin | Méthode |
| --- | --- |
| Longueur | `s.length` |
| Majuscules | `s.toUpperCase()`, `s.toLowerCase()` |
| Chercher | `s.includes("a")`, `s.startsWith("a")`, `s.indexOf("a")` |
| Découper, recoller | `s.split(",")`, `liste.join(", ")` |
| Morceau | `s.slice(0, 3)`, `s.at(-1)` |
| Nettoyer | `s.trim()` |
| Remplacer | `s.replaceAll("a", "b")` |
| Aligner | `s.padStart(5)`, `s.padEnd(5)`, `"-".repeat(10)` |

```javascript
const s = "  Bonjour le monde  ";
const propre = s.trim();
console.log(propre.toUpperCase());
console.log(propre.split(" "), propre.slice(0, 7), propre.at(-1));
console.log(`[${"42".padStart(5)}]`, propre.includes("monde"));
```

```sortie
BONJOUR LE MONDE
[ 'Bonjour', 'le', 'monde' ] Bonjour e
[   42] true
```

Détails : [le texte](texte.md).

## Tableaux

| Besoin | Syntaxe |
| --- | --- |
| Créer, lire | `const t = [1, 2, 3]`, `t[0]`, `t.at(-1)`, `t.length` |
| Ajouter, retirer | `t.push(4)`, `t.pop()`, `t.unshift(0)`, `t.shift()` |
| Transformer | `t.map((x) => x * 2)` |
| Garder | `t.filter((x) => x > 1)` |
| Réduire | `t.reduce((acc, x) => acc + x, 0)` |
| Chercher | `t.find(…)`, `t.includes(2)`, `t.some(…)`, `t.every(…)` |
| Trier sans modifier | `t.toSorted((a, b) => a - b)` |
| Copier, fusionner | `[...t]`, `[...a, ...b]` |
| Décomposer | `const [premier, ...reste] = t` |

```javascript
const notes = [12, 8, 15, 10];
console.log(notes.map((n) => n + 1));
console.log(notes.filter((n) => n >= 10));
console.log(notes.reduce((somme, n) => somme + n, 0));
console.log(notes.toSorted((a, b) => b - a), notes);
const [premiere, ...autres] = notes;
console.log(premiere, autres);
```

```sortie
[ 13, 9, 16, 11 ]
[ 12, 15, 10 ]
45
[ 15, 12, 10, 8 ] [ 12, 8, 15, 10 ]
12 [ 8, 15, 10 ]
```

Détails : [les collections](collections.md).

## Objets, Map et Set

| Besoin | Syntaxe |
| --- | --- |
| Créer | `const p = { nom: "Alice", age: 30 }` |
| Lire | `p.nom`, `p["nom"]`, `p.adresse?.ville` |
| Valeur par défaut | `p.ville ?? "inconnue"` |
| Parcourir | `Object.keys(p)`, `Object.values(p)`, `Object.entries(p)` |
| Copier en changeant | `{ ...p, age: 31 }` |
| Décomposer | `const { nom, age } = p` |
| Dictionnaire | `new Map()`, `m.set(k, v)`, `m.get(k)`, `m.has(k)` |
| Ensemble sans doublon | `new Set([1, 1, 2])` |

```javascript
const p = { nom: "Alice", age: 30 };
const plusVieux = { ...p, age: 31 };
const { nom, age } = plusVieux;
console.log(nom, age, p.age, p.ville ?? "inconnue");
for (const [cle, valeur] of Object.entries(p)) {
  console.log(`${cle} = ${valeur}`);
}
const vus = new Set(["a", "b", "a"]);
const stock = new Map([["pommes", 3]]);
console.log(vus.size, stock.get("pommes"), stock.has("poires"));
```

```sortie
Alice 31 30 inconnue
nom = Alice
age = 30
2 3 false
```

Détails : [les objets](objets.md).

## Classes et héritage

| Élément | Syntaxe |
| --- | --- |
| Classe | `class Animal { constructor(nom) { this.nom = nom; } }` |
| Objet | `new Animal("Rex")` |
| Champ privé | `#secret = 0` |
| Accesseur | `get nom() { … }` |
| Méthode de classe | `static creer() { … }` |
| Héritage | `class Chien extends Animal { … }` |
| Parent | `super(nom)`, `super.parler()` |
| Vérifier | `rex instanceof Animal` |

```javascript
class Animal {
  constructor(nom) {
    this.nom = nom;
  }
  parler() {
    return `${this.nom} fait un bruit.`;
  }
}

class Chien extends Animal {
  parler() {
    return `${this.nom} aboie.`;
  }
}

const animaux = [new Animal("Bob"), new Chien("Rex")];
for (const a of animaux) console.log(a.parler());
console.log(animaux[1] instanceof Animal);
```

```sortie
Bob fait un bruit.
Rex aboie.
true
```

Détails : [l'héritage](heritage.md).

## Closures

Une fonction se souvient des variables de l'endroit où elle a été créée.

```javascript
function multiplicateur(facteur) {
  return (x) => x * facteur;
}
const double = multiplicateur(2);
const triple = multiplicateur(3);
console.log(double(5), triple(5));
```

```sortie
10 15
```

Détails : [les closures](closures.md).

## Itérateurs et générateurs

| Élément | Syntaxe |
| --- | --- |
| Parcourir un itérable | `for (const x of iterable)` |
| Indice et valeur | `for (const [i, x] of t.entries())` |
| Générateur | `function* gen() { yield 1; }` |
| Tout récupérer | `[...gen()]`, `Array.from(gen())` |

```javascript
function* compteARebours(n) {
  while (n > 0) {
    yield n;
    n -= 1;
  }
}
console.log([...compteARebours(3)]);
for (const [i, lettre] of ["a", "b"].entries()) {
  console.log(i, lettre);
}
```

```sortie
[ 3, 2, 1 ]
0 a
1 b
```

Détails : [les itérateurs](iterateurs.md).

## Erreurs

| Besoin | Syntaxe |
| --- | --- |
| Attraper | `try { … } catch (erreur) { … }` |
| Toujours exécuter | `finally { … }` |
| Lever | `throw new Error("message")` |
| Types courants | `TypeError`, `RangeError`, `SyntaxError`, `ReferenceError` |
| Votre erreur | `class MonErreur extends Error { … }` |

```javascript
function verifierAge(age) {
  if (age < 0) throw new RangeError("âge négatif");
  return age;
}

try {
  verifierAge(-1);
} catch (erreur) {
  console.log(`${erreur.name} : ${erreur.message}`);
} finally {
  console.log("Vérification terminée");
}
```

```sortie
RangeError : âge négatif
Vérification terminée
```

Détails : [les erreurs](erreurs.md).

## Modules

| Besoin | Syntaxe |
| --- | --- |
| Exporter | `export function f() {}`, `export const X = 1;` |
| Export par défaut | `export default class Panier {}` |
| Importer | `import { f, X } from "./outils.js";` |
| Importer le défaut | `import Panier from "./panier.js";` |
| Module de Node | `import { readFile } from "node:fs/promises";` |

```javascript norun
// outils.js
export const TVA = 0.2;
export function prixTTC(ht) {
  return ht * (1 + TVA);
}

// main.js
import { prixTTC } from "./outils.js";
console.log(prixTTC(10));
```

Détails : [les modules](modules.md).

## Asynchrone

| Besoin | Syntaxe |
| --- | --- |
| Fonction asynchrone | `async function charger() { … }` |
| Attendre une promesse | `const x = await promesse;` |
| Créer une promesse | `new Promise((resolve, reject) => { … })` |
| Plus tard | `setTimeout(f, 100)` |
| Tout en parallèle | `await Promise.all([p1, p2])` |

```javascript
const attendre = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function charger(nom, ms) {
  await attendre(ms);
  return `${nom} chargé`;
}

console.log("Début");
const resultats = await Promise.all([charger("profil", 30), charger("notes", 10)]);
console.log(resultats);
```

```sortie
Début
[ 'profil chargé', 'notes chargé' ]
```

Détails : [l'asynchrone](asynchrone.md).

## Le DOM, dans le navigateur

| Besoin | Syntaxe |
| --- | --- |
| Trouver un élément | `document.querySelector("#titre")` |
| Plusieurs éléments | `document.querySelectorAll(".carte")` |
| Changer le texte | `el.textContent = "Bonjour"` |
| Classes CSS | `el.classList.add("actif")`, `toggle`, `remove` |
| Créer, ajouter | `document.createElement("li")`, `liste.append(li)` |
| Réagir | `bouton.addEventListener("click", () => { … })` |

```javascript norun
const bouton = document.querySelector("#compter");
let clics = 0;
bouton.addEventListener("click", () => {
  clics += 1;
  bouton.textContent = `Cliqué ${clics} fois`;
});
```

Détails : [le DOM](dom.md).

## Fichiers et JSON

| Besoin | Syntaxe |
| --- | --- |
| Lire | `await readFile("f.txt", "utf8")` |
| Écrire, ajouter | `await writeFile("f.txt", texte)`, `await appendFile(…)` |
| Fichier absent | `erreur.code === "ENOENT"` |
| Objet vers JSON | `JSON.stringify(obj, null, 2)` |
| JSON vers objet | `JSON.parse(texte)` |

```javascript
import { readFile, writeFile } from "node:fs/promises";

const donnees = { nom: "Alice", notes: [12, 15] };
await writeFile("eleve.json", JSON.stringify(donnees, null, 2));
const relu = JSON.parse(await readFile("eleve.json", "utf8"));
console.log(relu.notes);

try {
  await readFile("absent.txt", "utf8");
} catch (erreur) {
  console.log(erreur.code);
}
```

```sortie
[ 12, 15 ]
ENOENT
```

Détails : [les fichiers](fichiers.md) et [les formats](formats.md).

## Expressions régulières

| Élément | Sens |
| --- | --- |
| `\d`, `\w`, `\s` | un chiffre, une lettre ou chiffre, un espace |
| `+`, `*`, `?`, `{2,4}` | un ou plus, zéro ou plus, facultatif, de 2 à 4 |
| `^`, `$` | début, fin |
| `( )`, `(?<nom> )` | groupe, groupe nommé |
| Drapeaux | `g` (tous), `i` (sans casse) |
| Tester, chercher | `/re/.test(s)`, `s.match(/re/)`, `s.matchAll(/re/g)` |
| Remplacer | `s.replace(/re/g, "…")` |

```javascript
const texte = "Commande 42 le 2025-03-14, commande 7 le 2025-04-01";
console.log(/\d+/.test(texte));
console.log(texte.match(/\d+/g).join(" "));
const date = texte.match(/(?<annee>\d{4})-(?<mois>\d{2})/);
console.log(date.groups.annee, date.groups.mois);
console.log("a  b   c".replace(/\s+/g, " "));
```

```sortie
true
42 2025 03 14 7 2025 04 01
2025 03
a b c
```

Détails : [les expressions régulières](regex.md).

## Dates

| Besoin | Syntaxe |
| --- | --- |
| Maintenant | `new Date()` |
| Une date précise | `new Date("2025-03-14T10:30:00Z")` |
| Morceaux | `d.getFullYear()`, `d.getMonth()` (0 = janvier), `d.getDate()` |
| Format ISO | `d.toISOString()` |
| Format français | `d.toLocaleDateString("fr-FR", { … })` |
| Écart | `d2 - d1` (en millisecondes) |

```javascript
const d = new Date("2025-03-14T10:30:00Z");
console.log(d.toISOString());
console.log(d.getUTCFullYear(), d.getUTCMonth() + 1, d.getUTCDate());
console.log(d.toLocaleDateString("fr-FR", { dateStyle: "full", timeZone: "UTC" }));
const fin = new Date("2025-03-24T10:30:00Z");
console.log((fin - d) / (1000 * 60 * 60 * 24), "jours");
```

```sortie
2025-03-14T10:30:00.000Z
2025 3 14
vendredi 14 mars 2025
10 jours
```

Détails : [les dates](dates.md).

## Ligne de commande et web

| Besoin | Syntaxe |
| --- | --- |
| Arguments | `process.argv.slice(2)` |
| Variable d'environnement | `process.env.HOME` |
| Quitter avec un code | `process.exit(1)` |
| Erreur à l'écran | `console.error("…")` |
| Requête HTTP | `const rep = await fetch(url); await rep.json()` |
| Petit serveur | `http.createServer((req, res) => { … }).listen(3000)` |

```javascript norun
const [nom = "inconnu"] = process.argv.slice(2);
console.log(`Bonjour ${nom}`);

const reponse = await fetch("https://api.example.com/eleves");
if (!reponse.ok) throw new Error(`HTTP ${reponse.status}`);
const eleves = await reponse.json();
```

Détails : [la ligne de commande](ligne-de-commande.md) et [le web](web.md).

## Tests

| Besoin | Syntaxe |
| --- | --- |
| Importer | `import { test, describe } from "node:test";` |
| Assertions | `import assert from "node:assert/strict";` |
| Égalité | `assert.strictEqual(obtenu, attendu)` |
| Contenu | `assert.deepStrictEqual([1, 2], [1, 2])` |
| Erreur levée | `assert.throws(() => f(), RangeError)` |
| Lancer | `node --test` ou `npm test` |

```javascript
import assert from "node:assert/strict";

const double = (x) => x * 2;
assert.strictEqual(double(4), 8);
assert.deepStrictEqual([1, 2].map(double), [2, 4]);
assert.throws(() => JSON.parse("{oups"), SyntaxError);
console.log("3 vérifications réussies");
```

```sortie
3 vérifications réussies
```

Détails : [tester son code](tests.md).

## Débogage et style

| Besoin | Outil |
| --- | --- |
| Noms et valeurs | `console.log({ a, b })` |
| Liste d'objets | `console.table(liste)` |
| Objet profond | `console.dir(obj, { depth: null })` |
| Pause | `debugger;` avec les DevTools (`F12`) |
| Pause avec Node | `node --inspect-brk main.js` puis `chrome://inspect` |
| Style | `const` par défaut, `===`, retours anticipés, `?.` et `??` |
| Vérifier, formater | `npx eslint src/`, `npx prettier --write src/` |

Détails : [déboguer](debogage.md) et [bien écrire du JavaScript](style.md).

## Projet et outils

| Commande | Rôle |
| --- | --- |
| `node main.js` | lancer un programme |
| `node --watch main.js` | relancer à chaque modification |
| `npm init -y` | créer `package.json` |
| `npm install nom` | ajouter un paquet (`dependencies`) |
| `npm install --save-dev nom` | ajouter un outil (`devDependencies`) |
| `npm install` | installer les paquets d'un projet récupéré |
| `npm start`, `npm test`, `npm run nom` | lancer un script |
| `npx outil` | lancer un outil du projet |

Détails : [organiser un projet](projet.md). Pour pratiquer :
[les projets pas à pas](projets.md) et [les exercices corrigés](exercices.md). Comparez
avec [l'aide-mémoire Ruby](../ruby/aide-memoire.md) et
[l'aide-mémoire Rust](../rust/aide-memoire.md).
