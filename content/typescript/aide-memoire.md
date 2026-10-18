# Aide-mémoire

Cette page résume en un coup d'œil la syntaxe TypeScript vue dans tout le parcours, avec un exemple court pour chaque thème.

## Types de base

| Élément | Exemple |
| --- | --- |
| Constante, variable | `const nom = "Alice";`, `let age = 30;` |
| Annotation | `let age: number = 30;` |
| Nombres | `number` : `42`, `3.14`, `NaN` ; `bigint` : `10n` |
| Texte | `string` : `"a"`, `'a'`, `` `Bonjour ${nom}` `` |
| Booléen | `boolean` : `true`, `false` |
| Absence | `undefined`, `null` |
| N'importe quoi, vérifié | `unknown` (à rétrécir avant usage) |
| N'importe quoi, non vérifié | `any` (à éviter) |
| Ne renvoie rien | `void` |
| N'arrive jamais | `never` |
| Commentaire | `// ligne`, `/* bloc */` |
| Afficher | `console.log(a, b)` |

```typescript
const nom = "Alice";
let age: number = 30;
age += 1;
const inconnu: unknown = "texte";
if (typeof inconnu === "string") console.log(inconnu.length);
console.log(`${nom} a ${age} ans`, typeof age, 7 / 2, Math.floor(7 / 2));
```

```sortie
5
Alice a 31 ans number 3.5 3
```

Détails : [premiers pas](premiers-pas.md).

## Conditions et boucles

| Besoin | Syntaxe |
| --- | --- |
| Condition | `if (…) { … } else if (…) { … } else { … }` |
| Ternaire | `x > 0 ? "oui" : "non"` |
| Égalité stricte | `===`, `!==` (jamais `==`) |
| Logique | `&&`, `\|\|`, `!` |
| Valeur par défaut | `x ?? "défaut"` (si `null` ou `undefined`) |
| Accès prudent | `obj?.prop`, `f?.()` |
| Plusieurs cas | `switch (x) { case 1: …; break; default: … }` |
| Boucles | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of tab)` |
| Indice et valeur | `for (const [i, x] of tab.entries())` |
| Contrôle | `break`, `continue` |

```typescript
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

| Élément | Syntaxe |
| --- | --- |
| Déclaration | `function f(a: number, b: number): number { return a + b; }` |
| Fléchée | `const f = (a: number): number => a * 2;` |
| Paramètre facultatif | `function f(nom?: string)` |
| Valeur par défaut | `function f(salut = "Bonjour")` |
| Reste | `function somme(...n: number[])` |
| Type de fonction | `type Op = (a: number, b: number) => number;` |
| Sans retour | `function log(m: string): void` |

```typescript
type Op = (a: number, b: number) => number;

function saluer(nom: string, salut = "Bonjour", poli?: boolean): string {
  const texte = `${salut} ${nom}`;
  return poli ? `${texte}, enchanté.` : texte;
}
const somme = (...n: number[]): number => n.reduce((a, b) => a + b, 0);
const fois: Op = (a, b) => a * b;

console.log(saluer("Alice"));
console.log(saluer("Bob", "Salut", true));
console.log(somme(1, 2, 3), fois(6, 7));
```

```sortie
Bonjour Alice
Salut Bob, enchanté.
6 42
```

Détails : [les fonctions](fonctions.md).

## Texte

| Besoin | Méthode |
| --- | --- |
| Longueur | `s.length` |
| Majuscules, minuscules | `s.toUpperCase()`, `s.toLowerCase()` |
| Espaces autour | `s.trim()` |
| Contient, commence par | `s.includes("x")`, `s.startsWith("x")` |
| Découper, recoller | `s.split(",")`, `tab.join(", ")` |
| Remplacer | `s.replaceAll("a", "b")` |
| Morceau | `s.slice(0, 3)` |
| Aligner | `s.padEnd(10)`, `s.padStart(5, "0")` |
| Nombre → texte | `String(n)`, `n.toFixed(2)` |
| Texte → nombre | `Number("12")` (`NaN` si invalide) |

```typescript
const s = "  Bonjour le monde  ";
console.log(s.trim().toUpperCase());
console.log(s.trim().split(" "));
console.log("7".padStart(3, "0"), (2 / 3).toFixed(2), Number("12") + 1);
```

```sortie
BONJOUR LE MONDE
[ 'Bonjour', 'le', 'monde' ]
007 0.67 13
```

Détails : [le texte](texte.md).

## Tableaux, tuples et Map

| Élément | Syntaxe |
| --- | --- |
| Tableau | `const t: number[] = [1, 2];` ou `Array<number>` |
| Lecture seule | `readonly number[]` |
| Tuple | `const p: [string, number] = ["Alice", 30];` |
| Ajouter, retirer | `t.push(x)`, `t.pop()` |
| Transformer | `t.map((x) => x * 2)` |
| Filtrer | `t.filter((x) => x > 0)` |
| Chercher | `t.find(…)` → `T \| undefined`, `t.includes(x)` |
| Réduire | `t.reduce((acc, x) => acc + x, 0)` |
| Trier une copie | `[...t].sort((a, b) => a - b)` |
| Déstructurer | `const [premier, ...reste] = t;` |
| Dictionnaire | `new Map<string, number>()`, `get`, `set`, `has` |
| Ensemble | `new Set<string>()`, `add`, `has`, `size` |

```typescript
const notes = [12, 9, 15, 18];
const [premiere, ...autres] = notes;
console.log(notes.filter((n) => n >= 10).map((n) => n * 2));
console.log([...notes].sort((a, b) => b - a), premiere, autres.length);

const personne: [string, number] = ["Alice", 30];
const ages = new Map<string, number>([["Alice", 30]]);
ages.set("Bob", 25);
console.log(personne[0], ages.get("Bob"), ages.get("Zoé") ?? "inconnu");
```

```sortie
[ 24, 30, 36 ]
[ 18, 15, 12, 9 ] 12 3
Alice 25 inconnu
```

Détails : [les collections](collections.md).

## Objets et interfaces

| Élément | Syntaxe |
| --- | --- |
| Interface | `interface Point { x: number; y: number }` |
| Alias de type | `type Point = { x: number; y: number };` |
| Propriété facultative | `nom?: string` |
| Lecture seule | `readonly id: number` |
| Extension | `interface Point3D extends Point { z: number }` |
| Dictionnaire | `Record<string, number>` ou `{ [cle: string]: number }` |
| Déstructurer | `const { x, y } = point;` |
| Copier en modifiant | `{ ...point, x: 5 }` |
| Clés, valeurs | `Object.keys(o)`, `Object.entries(o)` |

```typescript
interface Personne {
  readonly id: number;
  nom: string;
  email?: string;
}

const alice: Personne = { id: 1, nom: "Alice" };
const modifiee = { ...alice, nom: "Alice B." };
const { nom, email } = modifiee;
console.log(nom, email ?? "pas d'email", alice.nom);
console.log(Object.entries({ maths: 15, art: 12 }));
```

```sortie
Alice B. pas d'email Alice
[ [ 'maths', 15 ], [ 'art', 12 ] ]
```

Détails : [les objets](objets.md).

## Unions et rétrécissement

| Besoin | Syntaxe |
| --- | --- |
| Union | `string \| number` |
| Peut manquer | `string \| undefined` |
| Tester un type simple | `typeof x === "string"` |
| Tester une classe | `x instanceof Date` |
| Tester une propriété | `"rayon" in forme` |
| Tester la présence | `if (x !== undefined)`, `if (x)` |
| Union discriminée | `{ type: "a"; … } \| { type: "b"; … }` puis `switch (v.type)` |
| Garde de type | `function estX(v: unknown): v is X` |
| Exhaustivité | `const reste: never = v;` dans le `default` |

```typescript
type Forme = { type: "cercle"; rayon: number } | { type: "carre"; cote: number };

function aire(f: Forme): number {
  switch (f.type) {
    case "cercle":
      return Math.PI * f.rayon ** 2;
    case "carre":
      return f.cote ** 2;
    default: {
      const reste: never = f;
      return reste;
    }
  }
}

function decrire(v: string | number): string {
  return typeof v === "string" ? v.toUpperCase() : v.toFixed(1);
}

console.log(aire({ type: "carre", cote: 3 }), decrire("ok"), decrire(2));
```

```sortie
9 OK 2.0
```

Détails : [les unions](unions.md).

## Classes

| Élément | Syntaxe |
| --- | --- |
| Classe | `class Chat { … }`, `new Chat("Tom")` |
| Propriété | `nom: string;` (à initialiser) |
| Raccourci du constructeur | `constructor(private nom: string) {}` |
| Visibilité | `public` (défaut), `private`, `protected`, `#prive` |
| Lecture seule | `readonly id: number` |
| Héritage | `class Chat extends Animal`, `super(…)` |
| Contrat | `class Chat implements Animal` |
| Statique | `static creer(): Chat` |
| Accesseur | `get nom(): string { … }` |

```typescript
interface Animal {
  crier(): string;
}

class Chat implements Animal {
  static nombre = 0;
  #vies = 9;

  constructor(readonly nom: string) {
    Chat.nombre++;
  }

  get vies(): number {
    return this.#vies;
  }

  crier(): string {
    return `${this.nom} : miaou`;
  }
}

const tom = new Chat("Tom");
console.log(tom.crier(), tom.vies, Chat.nombre);
```

```sortie
Tom : miaou 9 1
```

Détails : [les classes](types.md).

## Génériques

| Élément | Syntaxe |
| --- | --- |
| Fonction générique | `function premier<T>(t: T[]): T \| undefined` |
| Type générique | `type Boite<T> = { valeur: T };` |
| Contrainte | `<T extends { length: number }>` |
| Plusieurs paramètres | `function paire<A, B>(a: A, b: B): [A, B]` |
| Valeur par défaut | `<T = string>` |
| Préciser à l'appel | `premier<number>([])` (rarement utile) |

```typescript
function premier<T>(t: readonly T[]): T | undefined {
  return t[0];
}

function plusLong<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

type Boite<T> = { valeur: T };
const b: Boite<number> = { valeur: 42 };

console.log(premier(["a", "b"]), plusLong("chat", "ours"), plusLong([1, 2], [3]), b.valeur);
```

```sortie
a chat [ 1, 2 ] 42
```

Détails : [les génériques](generiques.md).

## Types littéraux

| Élément | Syntaxe |
| --- | --- |
| Littéral | `type Oui = "oui";` |
| Union de littéraux | `type Taille = "S" \| "M" \| "L";` |
| Figer une valeur | `const T = ["S", "M"] as const;` |
| Union depuis un tableau | `type Taille = (typeof T)[number];` |
| Gabarit de type | `` type Id = `user-${number}`; `` |
| Vérifier sans élargir | `const c = { … } satisfies Config;` |

```typescript
const TAILLES = ["S", "M", "L"] as const;
type Taille = (typeof TAILLES)[number];
type Id = `user-${number}`;

const t: Taille = "M";
const id: Id = "user-42";
console.log(t, id, TAILLES.length);
```

```sortie
M user-42 3
```

Détails : [les types littéraux](litteraux.md).

## Types utilitaires et types avancés

| Type | Effet |
| --- | --- |
| `Partial<T>` | toutes les propriétés facultatives |
| `Required<T>` | toutes les propriétés obligatoires |
| `Readonly<T>` | toutes les propriétés en lecture seule |
| `Pick<T, "a" \| "b">` | seulement certaines propriétés |
| `Omit<T, "a">` | toutes sauf certaines |
| `Record<K, V>` | un objet de clés `K` et de valeurs `V` |
| `ReturnType<typeof f>` | le type de retour d'une fonction |
| `Awaited<P>` | le type obtenu après `await` |
| `keyof T` | l'union des noms de propriétés |
| `T[K]` | le type d'une propriété |
| `typeof x` | le type d'une valeur |
| `T extends U ? A : B` | type conditionnel |
| `{ [K in keyof T]: … }` | type transformé (*mapped*) |

```typescript
interface Personne {
  nom: string;
  age: number;
}

function modifier(p: Personne, changements: Partial<Personne>): Personne {
  return { ...p, ...changements };
}

type Resume = Pick<Personne, "nom">;
type Cle = keyof Personne;

const alice: Readonly<Personne> = { nom: "Alice", age: 30 };
const r: Resume = { nom: alice.nom };
const cle: Cle = "age";
console.log(modifier(alice, { age: 31 }), r, alice[cle]);
```

```sortie
{ nom: 'Alice', age: 31 } { nom: 'Alice' } 30
```

Détails : [les types avancés](types-avances.md).

## Erreurs

| Besoin | Syntaxe |
| --- | --- |
| Lever | `throw new Error("message")` |
| Attraper | `try { … } catch (e) { … } finally { … }` |
| Type de `e` | `unknown` : tester `e instanceof Error` |
| Classe d'erreur | `class MonErreur extends Error { … }` |
| Résultat sans exception | `type R<T> = { ok: true; value: T } \| { ok: false; error: string }` |

```typescript
class SaisieInvalide extends Error {
  override name = "SaisieInvalide";
}

function lireAge(texte: string): number {
  const n = Number(texte);
  if (!Number.isInteger(n)) throw new SaisieInvalide(`« ${texte} » n'est pas un entier`);
  return n;
}

for (const saisie of ["34", "abc"]) {
  try {
    console.log(lireAge(saisie));
  } catch (e) {
    if (e instanceof Error) console.log(`${e.name} : ${e.message}`);
  }
}
```

```sortie
34
SaisieInvalide : « abc » n'est pas un entier
```

Détails : [les erreurs](erreurs.md).

## Asynchrone

| Besoin | Syntaxe |
| --- | --- |
| Fonction asynchrone | `async function f(): Promise<string>` |
| Attendre | `const x = await f();` |
| Plusieurs en parallèle | `await Promise.all([f(), g()])` |
| Même si certaines échouent | `await Promise.allSettled([…])` |
| La première finie | `await Promise.race([…])` |
| Attendre un délai | `await setTimeout(100)` avec `node:timers/promises` |
| Erreur asynchrone | `try { await f(); } catch (e) { … }` |

```typescript
import { setTimeout as attendre } from "node:timers/promises";

async function charger(nom: string, ms: number): Promise<string> {
  await attendre(ms);
  return `${nom} chargé`;
}

const resultats = await Promise.all([charger("profil", 20), charger("notes", 10)]);
console.log(resultats);
```

```sortie
[ 'profil chargé', 'notes chargé' ]
```

`Promise.all` garde l'ordre du tableau, même si `notes` finit en premier. Détails :
[l'asynchrone](asynchrone.md).

## Modules

| Besoin | Syntaxe |
| --- | --- |
| Exporter | `export function f() {}`, `export const X = 1;` |
| Export par défaut | `export default class Panier {}` |
| Importer | `import { f, X } from "./outils.js";` |
| Renommer | `import { f as g } from "./outils.js";` |
| Tout importer | `import * as outils from "./outils.js";` |
| Importer un type | `import type { Personne } from "./types.js";` |
| Module de Node | `import { readFile } from "node:fs/promises";` |

```typescript norun
// outils.ts
export interface Personne {
  nom: string;
}
export function saluer(p: Personne): string {
  return `Bonjour ${p.nom}`;
}

// main.ts
import { saluer, type Personne } from "./outils.js";
const alice: Personne = { nom: "Alice" };
console.log(saluer(alice));
```

L'import se termine par `.js`, le nom du fichier **compilé**. Détails :
[les modules](modules.md).

## Fichiers et JSON

| Besoin | Syntaxe (`node:fs/promises`) |
| --- | --- |
| Écrire | `await writeFile("f.txt", texte, "utf8")` |
| Lire | `await readFile("f.txt", "utf8")` |
| Ajouter | `await appendFile("f.txt", texte)` |
| Fichier absent | erreur de code `"ENOENT"` |
| Vers JSON | `JSON.stringify(v, null, 2)` |
| Depuis JSON | `JSON.parse(t)` : rangez-le dans un `unknown` |
| Arguments | `process.argv.slice(2)` |

```typescript
import { appendFile, readFile, writeFile } from "node:fs/promises";

await writeFile("f.txt", "un\n", "utf8");
await appendFile("f.txt", "deux\n");
console.log((await readFile("f.txt", "utf8")).trim().split("\n"));

const donnees: unknown = JSON.parse(JSON.stringify({ ok: true }));
if (typeof donnees === "object" && donnees !== null && "ok" in donnees) {
  console.log(donnees.ok);
}
```

```sortie
[ 'un', 'deux' ]
true
```

Détails : [les fichiers](fichiers.md), [les formats](formats.md).

## tsconfig.json

| Option | Rôle |
| --- | --- |
| `"strict": true` | toutes les vérifications strictes (indispensable) |
| `"target": "es2022"` | version de JavaScript produite |
| `"module": "nodenext"` | modules à la manière de Node |
| `"rootDir": "src"` | dossier des sources |
| `"outDir": "dist"` | dossier du JavaScript compilé |
| `"sourceMap": true` | fichiers `.js.map` pour déboguer |
| `"noUncheckedIndexedAccess": true` | `t[i]` est `T \| undefined` |
| `"noEmit": true` | vérifier sans rien écrire |
| `"skipLibCheck": true` | ne pas revérifier les paquets installés |
| `"include": ["src"]` | fichiers à compiler |

Détails : [la configuration](configuration.md).

## Commandes

| Commande | Rôle |
| --- | --- |
| `npm init -y` | créer un `package.json` |
| `npm install -D typescript @types/node tsx` | installer les outils |
| `npx tsc` | vérifier et compiler selon `tsconfig.json` |
| `npx tsc --noEmit` | vérifier les types seulement |
| `npx tsc --init` | créer un `tsconfig.json` |
| `npx tsx fichier.ts` | exécuter directement (sans vérifier les types) |
| `npx tsx watch fichier.ts` | relancer à chaque modification |
| `node --enable-source-maps dist/main.js` | lancer avec les lignes TypeScript dans les erreurs |
| `npx tsx --test` | lancer les tests `node:test` |
| `npx eslint src/`, `npx prettier --write src/` | vérifier et mettre en forme |

Détails : [organiser un projet](projet.md), [les tests](tests.md),
[déboguer](debogage.md), [bien écrire du TypeScript](style.md). Pour pratiquer :
[les projets pas à pas](projets.md) et [les exercices corrigés](exercices.md). Comparez
avec [l'aide-mémoire Ruby](../ruby/aide-memoire.md) et
[l'aide-mémoire Rust](../rust/aide-memoire.md).
