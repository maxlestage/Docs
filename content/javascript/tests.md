# Tester son code

Cette page montre comment écrire des tests automatiques en JavaScript avec les outils livrés avec Node : `console.assert`, le module `node:assert` et le lanceur de tests `node:test`.

> **Objectifs :**
>
> - comprendre à quoi sert un test automatique ;
> - vérifier des valeurs avec `assert.strictEqual`, `assert.deepStrictEqual` et `assert.throws` ;
> - écrire et lancer des tests avec `test()` et `describe()` ;
> - lire un test qui échoue ;
> - écrire le test avant le code.

## Pourquoi tester ?

Un **test** est un petit programme qui vérifie un autre programme (voir
[le glossaire](../debuter/glossaire.md#test)). Il appelle votre code avec des valeurs
connues et compare le résultat à celui que vous attendez.

Sans tests, vous vérifiez à la main, avec des `console.log`, et vous recommencez après
chaque changement. Avec des tests :

- une seule commande vérifie tout, en une seconde ;
- si un changement casse quelque chose, vous le savez tout de suite ;
- les tests montrent comment utiliser votre code : c'est une documentation qui marche.

C'est comme le contrôle technique d'une voiture : une liste de points précis, vérifiés
toujours de la même façon.

> **À retenir :** un test répond à une question simple : « avec cette entrée, est-ce que
> j'obtiens bien cette sortie ? ».

## Le plus simple : console.assert

`console.assert(condition, message)` vérifie une condition. Si elle est vraie, rien ne
se passe. Si elle est fausse, le message s'affiche.

```javascript norun
console.assert(1 + 1 === 3, "le calcul est faux");
console.assert(2 + 2 === 4, "jamais affiché");
console.log("Fin");
```

```text
Assertion failed: le calcul est faux
Fin
```

Quand tout va bien, `console.assert` reste silencieux :

```javascript
function double(x) {
  return x * 2;
}

console.assert(double(4) === 8, "double(4) devrait valoir 8");
console.assert(double(0) === 0, "double(0) devrait valoir 0");
console.log("Vérifications terminées.");
```

```sortie
Vérifications terminées.
```

> **Attention :** `console.assert` n'arrête pas le programme : `Fin` s'affiche quand même.
> C'est pratique pour une vérification rapide, mais un vrai test doit **s'arrêter** et
> signaler l'échec. C'est le rôle de `node:assert`.

## Le module node:assert

Node fournit le module `node:assert`. Importez sa version **stricte**,
`node:assert/strict` : elle compare avec `===`, sans conversion de type cachée.

### assert.strictEqual

`assert.strictEqual(obtenu, attendu)` vérifie que deux valeurs sont identiques. Si
c'est le cas, rien ne s'affiche.

```javascript
import assert from "node:assert/strict";

function double(x) {
  return x * 2;
}

assert.strictEqual(double(4), 8);
assert.strictEqual(double(-3), -6);
console.log("Tous les tests passent.");
```

```sortie
Tous les tests passent.
```

L'ordre compte : d'abord la valeur **obtenue** (`actual`), puis la valeur **attendue**
(`expected`). Ainsi, le message d'échec est juste.

### Lire un échec

Si les valeurs diffèrent, `assert` lève une
[exception](../debuter/glossaire.md#exception) `AssertionError` et le programme
s'arrête :

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual(0.1 + 0.2, 0.3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ 0.30000000000000004
- 0.3
     ^

    at file:///home/alice/projet/main.mjs:3:8
```

La ligne `+` donne la valeur obtenue, la ligne `-` la valeur attendue. Le `^` montre
où elles commencent à différer. Ici, c'est la surprise des nombres à virgule (voir
[les nombres](nombres.md)).

### assert.deepStrictEqual pour les tableaux et les objets

Deux tableaux qui contiennent les mêmes valeurs ne sont **pas** le même tableau : `===`
compare les adresses, pas le contenu. Pour comparer le contenu, utilisez
`assert.deepStrictEqual` (« profondément égal »).

```javascript
import assert from "node:assert/strict";

function pairs(nombres) {
  return nombres.filter((n) => n % 2 === 0);
}

assert.deepStrictEqual(pairs([1, 2, 3, 4]), [2, 4]);
assert.deepStrictEqual({ nom: "Alice", age: 30 }, { age: 30, nom: "Alice" });
console.log("Contenus identiques.");
```

```sortie
Contenus identiques.
```

L'ordre des propriétés d'un objet ne compte pas ; l'ordre des éléments d'un tableau,
si. En cas d'échec, le message montre la différence ligne par ligne :

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:
+ actual - expected

  {
+   age: 30,
-   age: 31,
    nom: 'Alice'
  }
```

### assert.throws pour vérifier une erreur

Un bon code refuse les mauvaises valeurs. `assert.throws` vérifie qu'une fonction
**lève** bien une erreur. Donnez-lui une fonction à appeler, pas son résultat : d'où la
flèche `() => …`.

```javascript
import assert from "node:assert/strict";

function verifierAge(age) {
  if (age < 0) {
    throw new RangeError("âge négatif");
  }
  return age;
}

assert.throws(() => verifierAge(-1), RangeError);
assert.throws(() => verifierAge(-1), { message: "âge négatif" });
assert.strictEqual(verifierAge(12), 12);
console.log("Les erreurs sont bien levées.");
```

```sortie
Les erreurs sont bien levées.
```

Le second argument précise l'erreur attendue : une classe (`RangeError`), une
expression régulière (`/négatif/`) ou un objet avec les propriétés à vérifier.

### Les autres vérifications

| Vérification | Réussit si… |
| --- | --- |
| `assert.ok(valeur)` | la valeur est vraie (*truthy*) |
| `assert.strictEqual(a, b)` | `a === b` |
| `assert.notStrictEqual(a, b)` | `a !== b` |
| `assert.deepStrictEqual(a, b)` | même contenu (tableaux, objets) |
| `assert.throws(() => f())` | `f()` lève une erreur |
| `assert.match(texte, /motif/)` | le texte correspond au motif |

## Le lanceur de tests node:test

Des `assert` les uns après les autres, c'est bien. Mais au premier échec, tout
s'arrête, et vous ne savez rien des vérifications suivantes. Le module `node:test`
range chaque vérification dans un **test** qui a un nom. Il lance tous les tests, même
si l'un échoue, puis affiche un bilan.

### Un premier test avec test()

`test(nom, fonction)` déclare un test. Le nom décrit ce que le test vérifie.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function somme(a, b) {
  return a + b;
}

test("additionne deux nombres", () => {
  assert.strictEqual(somme(2, 3), 5);
});

test("fonctionne avec des négatifs", () => {
  assert.strictEqual(somme(-2, -3), -5);
});
```

Lancez le fichier comme n'importe quel programme :

```sh
node somme.test.js
```

Dans un terminal, Node affiche un bilan comme celui-ci :

```text
✔ additionne deux nombres (0.81ms)
✔ fonctionne avec des négatifs (0.22ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 52.17
```

- `✔` : le test passe ; `✖` : il échoue.
- Le nombre entre parenthèses est la durée du test : il change à chaque lancement.
- `pass` compte les tests réussis, `fail` les échecs.

> **Note :** quand la sortie ne va pas vers un terminal (dans un fichier, par exemple),
> Node utilise un autre format, appelé TAP : `ok 1 - additionne deux nombres`. Le sens
> est le même.

### Grouper les tests avec describe()

`describe(nom, fonction)` regroupe les tests qui parlent de la même chose. C'est un
chapitre ; chaque `test` est un paragraphe.

```javascript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function majuscule(mot) {
  return mot.charAt(0).toUpperCase() + mot.slice(1);
}

describe("majuscule", () => {
  test("met la première lettre en majuscule", () => {
    assert.strictEqual(majuscule("alice"), "Alice");
  });

  test("ne change pas un mot vide", () => {
    assert.strictEqual(majuscule(""), "");
  });
});
```

```text
▶ majuscule
  ✔ met la première lettre en majuscule (0.62ms)
  ✔ ne change pas un mot vide (0.15ms)
✔ majuscule (1.21ms)
ℹ tests 2
ℹ suites 1
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 49.88
```

### Lancer tous les tests avec node --test

Rangez vos tests dans des fichiers qui finissent par `.test.js`, ou dans un dossier
`test/`. La commande `node --test` les trouve et les lance tous :

```sh
node --test
```

Dans un projet, on la range dans le script `test` du `package.json`, pour taper
simplement `npm test` (voir [organiser un projet](projet.md)).

### Lire un test qui échoue

Voici une fonction `somme` qui se trompe : elle soustrait au lieu d'additionner.

```javascript norun
function somme(a, b) {
  return a - b;
}

test("additionne deux nombres", () => {
  assert.strictEqual(somme(2, 3), 5);
});
```

```text
✖ additionne deux nombres (1.49ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 51.02

✖ failing tests:

test at somme.test.js:8:1
✖ additionne deux nombres (1.49ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  -1 !== 5

      at TestContext.<anonymous> (file:///home/alice/projet/somme.test.js:9:10)
```

Le bilan donne tout : le nom du test, sa position (`somme.test.js:8`), la valeur
obtenue (`-1`) et la valeur attendue (`5`).

### Un test asynchrone

Si la fonction testée renvoie une promesse, écrivez un test `async` et utilisez
`await` (voir [l'asynchrone](asynchrone.md)). Le test attend la fin de la promesse.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

async function chargerScore() {
  return 42;
}

test("charge le score", async () => {
  const score = await chargerScore();
  assert.strictEqual(score, 42);
});
```

## Vitest et Jest

Dans les gros projets, on utilise souvent **Vitest** ou **Jest**. Ce sont des
[paquets](../debuter/glossaire.md#paquet) à installer avec npm. Leur syntaxe ressemble
beaucoup à `node:test` : `describe`, `test` (ou `it`) et `expect` à la place de
`assert`.

```javascript norun
import { describe, test, expect } from "vitest";
import { somme } from "../src/somme.js";

describe("somme", () => {
  test("additionne deux nombres", () => {
    expect(somme(2, 3)).toBe(5);
  });
});
```

```sh
npm install --save-dev vitest
npx vitest run
```

`expect(obtenu).toBe(attendu)` correspond à `assert.strictEqual`, et `toEqual` à
`assert.deepStrictEqual`. Commencez avec `node:test` : rien à installer, et les idées
sont les mêmes.

## Que faut-il tester ?

Inutile de tester toutes les valeurs possibles. Choisissez quelques cas bien placés :

| Cas | Exemple pour `moyenne(notes)` |
| --- | --- |
| Le cas normal | `[10, 12, 14]` donne `12` |
| Les limites | une seule note, une liste vide |
| Les valeurs bizarres | des notes à virgule, des zéros |
| Les erreurs | une note négative doit être refusée |

```javascript
import assert from "node:assert/strict";

function moyenne(notes) {
  if (notes.length === 0) return 0;
  if (notes.some((n) => n < 0)) throw new RangeError("note négative");
  const somme = notes.reduce((total, n) => total + n, 0);
  return somme / notes.length;
}

assert.strictEqual(moyenne([10, 12, 14]), 12);
assert.strictEqual(moyenne([15]), 15);
assert.strictEqual(moyenne([]), 0);
assert.strictEqual(moyenne([0, 0]), 0);
assert.throws(() => moyenne([10, -2]), RangeError);
console.log("5 cas vérifiés.");
```

```sortie
5 cas vérifiés.
```

> **Astuce :** quand vous trouvez un [bug](../debuter/glossaire.md#bug), écrivez
> d'abord un test qui le montre, puis corrigez. Le bug ne reviendra jamais sans que
> vous le sachiez.

## Écrire le test d'abord

Une méthode répandue consiste à écrire le test **avant** le code. On avance en trois
temps, toujours les mêmes :

1. **Rouge** : écrivez un test pour une fonction qui n'existe pas encore. Il échoue.
2. **Vert** : écrivez le code le plus simple qui fait passer le test.
3. **Améliorer** : rendez le code plus clair, en relançant les tests à chaque fois.

Exemple : une fonction `estPalindrome` qui dit si un mot se lit dans les deux sens.
D'abord les tests, qui décrivent ce que l'on veut :

```javascript norun
test("kayak est un palindrome", () => {
  assert.strictEqual(estPalindrome("kayak"), true);
});

test("ignore les majuscules", () => {
  assert.strictEqual(estPalindrome("Kayak"), true);
});

test("javascript n'est pas un palindrome", () => {
  assert.strictEqual(estPalindrome("javascript"), false);
});
```

Lancés tels quels, ils échouent avec `ReferenceError: estPalindrome is not defined` :
c'est l'étape rouge. Puis le code qui les fait passer, l'étape verte :

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function estPalindrome(mot) {
  const minuscules = mot.toLowerCase();
  const envers = [...minuscules].reverse().join("");
  return minuscules === envers;
}

test("kayak est un palindrome", () => {
  assert.strictEqual(estPalindrome("kayak"), true);
});

test("ignore les majuscules", () => {
  assert.strictEqual(estPalindrome("Kayak"), true);
});

test("javascript n'est pas un palindrome", () => {
  assert.strictEqual(estPalindrome("javascript"), false);
});
```

Écrire le test d'abord vous oblige à réfléchir à **ce que** la fonction doit faire,
avant de penser à **comment** la faire.

## Erreurs fréquentes

### Donner le résultat à assert.throws au lieu d'une fonction

```javascript norun
import assert from "node:assert/strict";

function verifierAge(age) {
  if (age < 0) throw new RangeError("âge négatif");
  return age;
}

assert.throws(verifierAge(-1), RangeError);
```

```text
file:///home/alice/projet/main.mjs:4
  if (age < 0) throw new RangeError("âge négatif");
                     ^

RangeError: âge négatif
    at verifierAge (file:///home/alice/projet/main.mjs:4:22)
    at file:///home/alice/projet/main.mjs:8:15
```

`verifierAge(-1)` est appelé **avant** `assert.throws` : l'erreur part trop tôt et
arrête le programme. Passez une fonction : `assert.throws(() => verifierAge(-1), RangeError)`.

### Oublier d'importer test

```javascript norun
import assert from "node:assert/strict";

test("double", () => {
  assert.strictEqual(2 * 2, 4);
});
```

```text
file:///home/alice/projet/main.mjs:3
test("double", () => {
^

ReferenceError: test is not defined
    at file:///home/alice/projet/main.mjs:3:1
```

Contrairement à Jest, `test` n'existe pas tout seul. Ajoutez
`import { test } from "node:test";` en haut du fichier.

### Comparer des tableaux avec strictEqual

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual([1, 2], [1, 2]);
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

[
  1,
  2
]

    at file:///home/alice/projet/main.mjs:3:8
```

Les deux tableaux ont le même contenu, mais ce sont deux objets différents. Pour
comparer le contenu, utilisez `assert.deepStrictEqual([1, 2], [1, 2])`.

## Testez-vous

**Question 1 :** quelle est la différence entre `console.assert` et `assert.strictEqual` ?

> **Réponse**
>
> `console.assert` affiche un message si la condition est fausse, mais le programme
> continue. `assert.strictEqual` lève une `AssertionError` qui arrête le programme (ou
> marque le test comme échoué avec `node:test`).

**Question 2 :** pourquoi ce test échoue-t-il, et comment le corriger ?
`assert.strictEqual({ x: 1 }, { x: 1 })`

> **Réponse**
>
> Ce sont deux objets différents : `===` compare les objets par adresse, pas par
> contenu. Utilisez `assert.deepStrictEqual({ x: 1 }, { x: 1 })`.

**Question 3 :** écrivez une vérification qui s'assure que `JSON.parse("{oups")` lève
une `SyntaxError`.

> **Réponse**
>
> ```javascript
> import assert from "node:assert/strict";
>
> assert.throws(() => JSON.parse("{oups"), SyntaxError);
> console.log("SyntaxError bien levée.");
> ```
>
> ```sortie
> SyntaxError bien levée.
> ```

## En résumé

- Un test vérifie qu'une entrée connue donne la sortie attendue, sans intervention.
- `import assert from "node:assert/strict"` donne `strictEqual`, `deepStrictEqual` et
  `throws` ; un échec lève une `AssertionError`.
- `test()` et `describe()` de `node:test` nomment et regroupent les tests ;
  `node --test` les lance tous.
- Testez le cas normal, les limites et les erreurs ; écrivez un test pour chaque bug.
- Rouge, vert, améliorer : écrire le test d'abord clarifie ce que le code doit faire.

Suite : [déboguer en JavaScript](debogage.md). Comparez avec
[les tests en Ruby](../ruby/tests.md).
