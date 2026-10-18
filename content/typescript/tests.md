# Tester son code

Cette page montre comment écrire des tests automatiques en TypeScript, avec les outils livrés avec Node (`node:assert` et `node:test`), avec Vitest, et même comment tester les types eux-mêmes.

> **Objectifs :**
>
> - comprendre à quoi sert un test automatique ;
> - vérifier des résultats avec `node:assert/strict` ;
> - organiser et lancer des tests avec `node:test` ;
> - écrire les mêmes tests avec Vitest ;
> - vérifier qu'un mauvais usage est bien refusé avec `// @ts-expect-error`.

## Pourquoi tester ?

Un [test](../debuter/glossaire.md#test) est un petit programme qui vérifie un autre
programme. Il appelle votre code avec des valeurs connues et compare le résultat à
celui que vous attendez.

TypeScript attrape déjà beaucoup d'erreurs : un texte à la place d'un nombre, une
propriété mal écrite. Mais il ne sait pas si votre **calcul** est juste. Une fonction
`moyenne` qui divise par le mauvais nombre a des types parfaits… et un résultat faux.
Les types vérifient la **forme**, les tests vérifient le **sens**.

Avec des tests :

- une seule commande vérifie tout, en quelques secondes ;
- si une modification casse quelque chose, vous le savez tout de suite ;
- les tests montrent comment utiliser votre code : une documentation qui fonctionne.

> **À retenir :** un test répond à une question simple : « avec cette entrée, est-ce que
> j'obtiens bien cette sortie ? ».

## Vérifier avec node:assert/strict

Le module `node:assert/strict` fournit des fonctions de vérification, appelées
**assertions**. Si la vérification réussit, rien ne se passe. Si elle échoue, une
erreur arrête le programme.

```typescript
import assert from "node:assert/strict";

function double(n: number): number {
  return n * 2;
}

assert.equal(double(4), 8);
assert.equal(double(0), 0);
assert.ok(double(3) > 5);
console.log("Toutes les vérifications passent.");
```

```sortie
Toutes les vérifications passent.
```

`assert.equal(reel, attendu)` se lit : « vérifie que `double(4)` vaut bien `8` ». La
valeur calculée vient en premier, la valeur attendue en second.

### Les assertions les plus utiles

| Assertion | Vérifie que… |
|-----------|--------------|
| `assert.equal(a, b)` | `a` et `b` sont identiques (`===`) |
| `assert.deepEqual(a, b)` | deux objets ou tableaux ont le même contenu |
| `assert.ok(valeur)` | la valeur est vraie |
| `assert.throws(() => f(), /message/)` | `f()` lève une erreur dont le message correspond |
| `await assert.rejects(promesse)` | une promesse est rejetée |

`deepEqual` compare le **contenu** : deux tableaux créés séparément ne sont jamais le
même objet, mais ils peuvent contenir les mêmes valeurs.

```typescript
import assert from "node:assert/strict";

function trier(nombres: number[]): number[] {
  return [...nombres].sort((a, b) => a - b);
}

assert.deepEqual(trier([3, 1, 2]), [1, 2, 3]);
assert.deepEqual({ nom: "Ada", age: 36 }, { nom: "Ada", age: 36 });
console.log("Contenus identiques.");
```

```sortie
Contenus identiques.
```

### Quand une assertion échoue

Le message d'erreur montre la valeur obtenue et la valeur attendue. Ici, on l'attrape
pour l'afficher :

```typescript
import assert from "node:assert/strict";

function moyenne(notes: number[]): number {
  return notes.reduce((a, b) => a + b, 0) / (notes.length + 1);
}

try {
  assert.equal(moyenne([10, 14, 18]), 14);
} catch (e) {
  if (e instanceof assert.AssertionError) {
    console.log(e.message);
  }
}
```

```sortie
Expected values to be strictly equal:

10.5 !== 14
```

Le test a trouvé le bug : on divise par `notes.length + 1` au lieu de `notes.length`.
TypeScript, lui, n'avait rien vu.

### Tester les erreurs et les promesses

`assert.throws` vérifie qu'une fonction lève bien une erreur. Pour du code asynchrone
(voir [la programmation asynchrone](asynchrone.md)), `assert.rejects` fait la même chose
avec une promesse, et s'utilise avec `await` :

```typescript
import assert from "node:assert/strict";

function racine(n: number): number {
  if (n < 0) {
    throw new RangeError("nombre négatif");
  }
  return Math.sqrt(n);
}

async function charger(id: number): Promise<string> {
  if (id <= 0) {
    throw new Error("identifiant invalide");
  }
  return `élément ${id}`;
}

assert.equal(racine(9), 3);
assert.throws(() => racine(-1), RangeError);
assert.throws(() => racine(-1), /négatif/);

assert.equal(await charger(2), "élément 2");
await assert.rejects(charger(0), /identifiant invalide/);
console.log("Erreurs bien détectées.");
```

```sortie
Erreurs bien détectées.
```

## Organiser les tests avec node:test

Des assertions en vrac, c'est bien pour commencer. Mais dès la première erreur, tout
s'arrête, et on ne sait pas quels autres tests auraient échoué. Le module `node:test`,
livré avec Node, organise les assertions en **tests** nommés, regroupés avec `describe` :

- `describe("nom", () => { … })` regroupe les tests d'une même fonction ;
- `test("ce que l'on vérifie", () => { … })` est un test : une phrase qui décrit le
  comportement attendu.

```typescript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function moyenne(notes: number[]): number {
  if (notes.length === 0) {
    throw new Error("liste vide");
  }
  return notes.reduce((a, b) => a + b, 0) / notes.length;
}

describe("moyenne", () => {
  test("calcule la moyenne de plusieurs notes", () => {
    assert.equal(moyenne([10, 14, 18]), 14);
  });

  test("fonctionne avec une seule note", () => {
    assert.equal(moyenne([12]), 12);
  });

  test("refuse une liste vide", () => {
    assert.throws(() => moyenne([]), /liste vide/);
  });
});
```

Un test peut aussi être `async` : écrivez `test("…", async () => { … })` et utilisez
`await` à l'intérieur.

### Lancer les tests

Rangez ce code dans `src/moyenne.test.ts`. On compile (voir
[configurer TypeScript](configuration.md)), puis `node --test` lance tous les fichiers
de test :

```sh
npx tsc
node --test "dist/**/*.test.js"
```

```text
▶ moyenne
  ✔ calcule la moyenne de plusieurs notes (0.736478ms)
  ✔ fonctionne avec une seule note (0.23265ms)
  ✔ refuse une liste vide (0.831555ms)
✔ moyenne (3.066365ms)
ℹ tests 3
ℹ suites 1
ℹ pass 3
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 121.217153
```

Chaque `✔` est un test réussi. Les durées entre parenthèses changent à chaque lancement.

Si un test échoue, il est marqué `✖`, et le détail s'affiche à la fin, avec la ligne
fautive :

```text
▶ moyenne
  ✖ calcule la moyenne de plusieurs notes (2.054574ms)
  ✔ fonctionne avec une seule note (0.163259ms)
  ✔ refuse une liste vide (0.505934ms)
✖ moyenne (4.078187ms)
ℹ tests 3
ℹ suites 1
ℹ pass 2
ℹ fail 1
...

✖ failing tests:

test at dist/moyenne.test.js:10:5
✖ calcule la moyenne de plusieurs notes (2.054574ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  14 !== 15
```

Les autres tests ont quand même été lancés : vous voyez d'un coup tout ce qui marche et
tout ce qui ne marche pas.

> **Astuce :** ajoutez `"test": "tsc && node --test \"dist/**/*.test.js\""` dans la partie
> `scripts` de `package.json`. Ensuite, `npm test` suffit.

## Vitest

**Vitest** est un outil de test très utilisé dans les projets TypeScript, surtout côté
web. Son grand avantage : il lit directement les fichiers `.ts`, sans compilation
préalable. Installez-le dans le projet :

```sh
npm install --save-dev vitest
```

Le code à tester, dans `src/panier.ts` :

```typescript norun
export interface Article {
  prix: number;
  quantite: number;
}

export function total(articles: Article[]): number {
  return articles.reduce((somme, a) => somme + a.prix * a.quantite, 0);
}
```

Les tests, dans `src/panier.test.ts`. Au lieu de `assert`, Vitest utilise
`expect(valeur)`, suivi d'une vérification : `toBe` (identique), `toEqual` (même
contenu), `toThrow` (lève une erreur).

```typescript norun
import { describe, it, expect } from "vitest";
import { total } from "./panier.js";

describe("total", () => {
  it("additionne les articles", () => {
    expect(total([{ prix: 2, quantite: 3 }, { prix: 4, quantite: 1 }])).toBe(10);
  });

  it("vaut 0 pour un panier vide", () => {
    expect(total([])).toBe(0);
  });

  it("renvoie un nouvel objet bien formé", () => {
    const resume = { nombre: 2, total: total([{ prix: 5, quantite: 2 }]) };
    expect(resume).toEqual({ nombre: 2, total: 10 });
  });
});
```

```sh
npx vitest run
```

```text
 RUN  v3.2.4 /home/vous/mon-projet

 ✓ src/panier.test.ts (3 tests) 6ms

 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  21:12:24
   Duration  807ms (transform 181ms, setup 0ms, collect 109ms, tests 6ms, environment 0ms, prepare 156ms)
```

Sans `run`, `npx vitest` reste ouvert et relance les tests à chaque enregistrement,
comme le mode watch de `tsc`.

> **Note :** `it` et `test` sont synonymes. `it("additionne les articles")` se lit comme
> une phrase en anglais : « il additionne les articles ».

## Tester les types avec @ts-expect-error

Avec TypeScript, on peut aussi vérifier qu'un **mauvais** usage est bien **refusé**. Le
commentaire `// @ts-expect-error` dit : « la ligne suivante doit provoquer une erreur de
type ». Deux cas :

- la ligne contient bien une erreur : elle est ignorée, tout va bien ;
- la ligne ne contient **pas** d'erreur : c'est `tsc` qui signale le commentaire inutile.

C'est donc un test : si quelqu'un affaiblit un jour vos types, la compilation échoue.

```typescript
interface Utilisateur {
  nom: string;
  age: number;
}

function creer(nom: string, age: number): Utilisateur {
  return { nom, age };
}

function verifierLesTypes(): void {
  // @ts-expect-error : l'âge doit être un nombre
  creer("Ada", "36");

  // @ts-expect-error : il manque l'âge
  creer("Ada");

  const u = creer("Ada", 36);
  // @ts-expect-error : la propriété email n'existe pas
  console.log(u.email);
}

console.log(creer("Grace", 45));
```

```sortie
{ nom: 'Grace', age: 45 }
```

La fonction `verifierLesTypes` n'est jamais appelée : elle ne sert qu'au compilateur.
Lancez `tsc --noEmit` pour exécuter ces « tests de types ».

> **Attention :** n'utilisez pas `@ts-expect-error` (ni `@ts-ignore`) pour faire taire
> une vraie erreur dans votre programme. Corrigez-la plutôt.

## Que faut-il tester ?

Vous ne pouvez pas tout tester. Concentrez-vous sur :

- **le cas normal** : une entrée ordinaire et son résultat attendu ;
- **les cas limites** : liste vide, zéro, nombre négatif, texte vide, un seul élément ;
- **les erreurs** : une entrée invalide doit lever la bonne erreur ;
- **les bugs déjà trouvés** : chaque bug corrigé mérite un test, pour qu'il ne revienne pas.

Inutile, en revanche, de tester ce que TypeScript garantit déjà : qu'une fonction
`(n: number) => number` renvoie un nombre, par exemple.

Une bonne habitude : écrire le test **avant** le code. Écrivez un test qui échoue (la
fonction n'existe pas encore), puis le code le plus simple qui le fait passer, puis
améliorez le code en gardant les tests au vert.

## Erreurs fréquentes

### Comparer deux objets avec equal

```typescript norun
import assert from "node:assert/strict";

assert.equal({ x: 1 }, { x: 1 });
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

{
  x: 1
}
```

`equal` vérifie que c'est **le même objet**. Pour comparer le contenu, utilisez
`assert.deepEqual` (ou `toEqual` avec Vitest).

### Oublier await dans un test asynchrone

```typescript norun
import assert from "node:assert/strict";

async function charger(): Promise<number> {
  return 3;
}

assert.equal(charger(), 3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ Promise {
+   3
+ }
- 3
```

Vous comparez une promesse avec un nombre. Écrivez `assert.equal(await charger(), 3);`.

### Un @ts-expect-error qui n'attend rien

```typescript norun
function double(n: number): number {
  return n * 2;
}

// @ts-expect-error
double(4);
```

```text
main.ts(5,1): error TS2578: Unused '@ts-expect-error' directive.
```

`double(4)` est correct : il n'y a aucune erreur à attendre. Retirez le commentaire, ou
placez-le au-dessus d'un appel vraiment invalide, comme `double("4")`.

## Testez-vous

**Question 1 :** quelle différence entre `assert.equal` et `assert.deepEqual` ?

> **Réponse**
>
> `equal` vérifie que les deux valeurs sont identiques (`===`) : pour des objets, que ce
> soit le même objet. `deepEqual` compare le contenu, propriété par propriété.

**Question 2 :** pourquoi tester une fonction dont les types sont déjà vérifiés par
TypeScript ?

> **Réponse**
>
> Les types vérifient la forme des données, pas la justesse du calcul. Une fonction
> peut avoir des types parfaits et renvoyer un mauvais résultat.

**Question 3 :** écrivez une fonction `initiales(nom: string): string` (« Ada Lovelace »
donne « AL ») et vérifiez-la avec deux assertions, dont une sur un texte vide.

> **Réponse**
>
> ```typescript
> import assert from "node:assert/strict";
>
> function initiales(nom: string): string {
>   return nom
>     .split(" ")
>     .filter((mot) => mot !== "")
>     .map((mot) => mot[0]?.toUpperCase() ?? "")
>     .join("");
> }
>
> assert.equal(initiales("Ada Lovelace"), "AL");
> assert.equal(initiales(""), "");
> console.log("OK");
> ```
>
> ```sortie
> OK
> ```

## En résumé

- Les types vérifient la forme du code ; les tests vérifient qu'il fait le bon calcul.
- `node:assert/strict` : `equal`, `deepEqual`, `ok`, `throws`, `rejects`.
- `node:test` regroupe les tests avec `describe` et `test` ; `node --test` les lance.
- Vitest lit directement le TypeScript et utilise `expect(...).toBe(...)`.
- `// @ts-expect-error` vérifie qu'un mauvais usage est bien refusé par le compilateur.
- Testez le cas normal, les cas limites, les erreurs et les bugs déjà corrigés.

Suite : [déboguer un programme](debogage.md). Comparez avec
[les tests en Ruby](../ruby/tests.md).
