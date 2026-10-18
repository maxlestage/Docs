# Déboguer en TypeScript

Cette page montre comment trouver et corriger un bug en TypeScript : lire les erreurs du compilateur `tsc`, reconnaître les codes d'erreur courants, suivre une erreur d'exécution jusqu'à la bonne ligne et afficher les valeurs.

> **Objectifs :**
>
> - lire un message de `tsc`, même long, en commençant par la dernière ligne ;
> - reconnaître les erreurs TS2322, TS2339, TS2345, TS7006, TS18048 et TS2532 ;
> - retrouver la ligne TypeScript d'une erreur d'exécution grâce aux *source maps* ;
> - afficher l'état du programme avec `console.log` et ses cousins ;
> - savoir quand utiliser `// @ts-expect-error`, et pourquoi éviter `// @ts-ignore`.

## Deux sortes de bugs

Un [bug](../debuter/glossaire.md#bug) est une erreur dans un programme. En TypeScript,
il en existe deux familles :

1. **Les erreurs de type**, trouvées par le [compilateur](../debuter/glossaire.md#compilateur)
   `tsc` **avant** l'exécution. Le programme ne tourne même pas.
2. **Les erreurs d'exécution**, qui arrivent **pendant** que Node exécute le JavaScript
   produit. Les types ne les ont pas vues venir.

C'est une bonne nouvelle : la première famille est la plus fréquente, et `tsc` vous dit
exactement où chercher. Pour la méthode générale (observer, supposer, vérifier), voyez
aussi [déboguer pour débuter](../debuter/deboguer.md).

## Lire une erreur de tsc

Voici un programme avec une faute de type, et ce que `tsc` répond :

```typescript norun
let total: number = "12";
```

```text
main.ts:1:5 - error TS2322: Type 'string' is not assignable to type 'number'.

1 let total: number = "12";
      ~~~~~


Found 1 error in main.ts:1
```

Lisez-le morceau par morceau :

- `main.ts:1:5` : le fichier, la **ligne 1**, la **colonne 5** ;
- `TS2322` : le **code** de l'erreur, toujours le même pour le même problème ;
- `Type 'string' is not assignable to type 'number'` : le message. *Assignable* veut
  dire « peut être rangé dans » : on ne peut pas ranger une chaîne dans un nombre ;
- les `~~~~~` soulignent l'endroit exact.

Le correctif : `let total: number = 12;` (sans guillemets), ou `Number("12")` si la
valeur arrive vraiment sous forme de texte.

> **Astuce :** dans un terminal, `npx tsc --noEmit` vérifie les types de tout le projet
> sans produire de fichiers JavaScript. C'est la commande à lancer avant chaque commit.

## Les messages longs : lire la dernière ligne d'abord

Quand les types sont imbriqués, le message s'étire sur plusieurs lignes, de plus en
plus décalées vers la droite.

```typescript norun
interface Eleve {
  nom: string;
  notes: number[];
}

const donnees = { nom: "Alice", notes: ["12", "15"] };
const alice: Eleve = donnees;
```

```text
main.ts(7,7): error TS2322: Type '{ nom: string; notes: string[]; }' is not assignable to type 'Eleve'.
  Types of property 'notes' are incompatible.
    Type 'string[]' is not assignable to type 'number[]'.
      Type 'string' is not assignable to type 'number'.
```

C'est comme des poupées russes : chaque ligne ouvre la poupée suivante. La **dernière
ligne** est la plus petite poupée, la vraie cause : une `string` là où on attend un
`number`. Les lignes du dessus disent **où** elle se cache : dans la propriété `notes`.

Lisez donc de bas en haut :

1. la dernière ligne : **quoi** (`string` au lieu de `number`) ;
2. les lignes du milieu : **où** (la propriété `notes`) ;
3. la première ligne : le contexte (l'affectation de la ligne 7).

```typescript
interface Eleve {
  nom: string;
  notes: number[];
}

const donnees = { nom: "Alice", notes: [12, 15] };
const alice: Eleve = donnees;
console.log(alice.notes);
```

```sortie
[ 12, 15 ]
```

## Les codes d'erreur courants

Six codes reviennent sans cesse chez les débutants. Apprenez à les reconnaître : vous
gagnerez beaucoup de temps.

| Code | Message (début) | Cause habituelle |
|------|-----------------|------------------|
| TS2322 | `Type 'X' is not assignable to type 'Y'` | une valeur du mauvais type dans une variable |
| TS2339 | `Property 'x' does not exist on type 'Y'` | faute de frappe, ou propriété absente du type |
| TS2345 | `Argument of type 'X' is not assignable to parameter of type 'Y'` | un mauvais argument dans un appel de fonction |
| TS7006 | `Parameter 'x' implicitly has an 'any' type` | un paramètre sans annotation de type |
| TS18048 | `'x' is possibly 'undefined'` | une variable qui peut valoir `undefined` |
| TS2532 | `Object is possibly 'undefined'` | pareil, pour une expression (un appel, un accès) |

### TS2322 : mauvais type dans une variable

```typescript norun
let total: number = "12";
```

```text
main.ts(1,5): error TS2322: Type 'string' is not assignable to type 'number'.
```

Rangez une valeur du bon type, ou changez l'annotation si c'est elle qui se trompe.

### TS2339 : la propriété n'existe pas

```typescript norun
const eleve = { nom: "Alice", age: 15 };
console.log(eleve.nmo);
```

```text
main.ts(2,19): error TS2339: Property 'nmo' does not exist on type '{ nom: string; age: number; }'.
```

Le message affiche le type complet : comparez les noms un à un. Ici, `nmo` au lieu de
`nom`.

### TS2345 : mauvais argument

```typescript norun
function carre(n: number): number {
  return n * n;
}
carre("4");
```

```text
main.ts(4,7): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.
```

L'appel envoie une chaîne, la fonction attend un nombre. Écrivez `carre(4)`.

### TS7006 : paramètre sans type

```typescript norun
function saluer(nom) {
  return `Bonjour ${nom}`;
}
```

```text
main.ts(1,17): error TS7006: Parameter 'nom' implicitly has an 'any' type.
```

Avec `--strict`, TypeScript refuse de deviner le type d'un paramètre. Annotez-le :
`function saluer(nom: string)`.

### TS18048 : peut-être undefined

```typescript norun
const notes = [12, 15, 9];
const bonne = notes.find((n) => n > 18);
console.log(bonne.toFixed(1));
```

```text
main.ts(3,13): error TS18048: 'bonne' is possibly 'undefined'.
```

`find` renvoie `undefined` s'il ne trouve rien. Vérifiez avant d'utiliser la valeur :
c'est le [rétrécissement](unions.md) (*narrowing*).

```typescript
const notes = [12, 15, 9];
const bonne = notes.find((n) => n > 18);
if (bonne === undefined) {
  console.log("Aucune note au-dessus de 18");
} else {
  console.log(bonne.toFixed(1));
}
```

```sortie
Aucune note au-dessus de 18
```

### TS2532 : un objet peut-être undefined

```typescript norun
const ages = new Map([["Alice", 15]]);
console.log(ages.get("Alice").toFixed(0));
```

```text
main.ts(2,13): error TS2532: Object is possibly 'undefined'.
```

Même problème que TS18048, mais sur une expression sans nom (ici l'appel `get`).
Rangez-la dans une variable pour la vérifier, ou donnez une valeur par défaut avec `??` :

```typescript
const ages = new Map([["Alice", 15]]);
console.log((ages.get("Alice") ?? 0).toFixed(0));
console.log((ages.get("Bob") ?? 0).toFixed(0));
```

```sortie
15
0
```

## Voir les types dans l'éditeur

Le meilleur outil de débogage de TypeScript est votre [éditeur](../debuter/editeur.md).
Dans VS Code :

- **survolez** une variable avec la souris : une bulle affiche son type, par exemple
  `const bonne: number | undefined` ;
- les erreurs sont **soulignées en rouge** pendant que vous tapez, avant même de lancer
  `tsc` ;
- **F12** (« Aller à la définition ») ouvre la déclaration d'un type ou d'une fonction ;
- **F2** renomme une variable partout, sans risque d'en oublier une.

Quand une erreur vous surprend, survolez chaque variable de la ligne. Très souvent, l'une
d'elles n'a pas le type que vous imaginiez : `string | undefined` au lieu de `string`,
`any` au lieu d'un vrai type.

> **Astuce :** pour connaître le type d'une expression compliquée, rangez-la dans une
> variable temporaire et survolez-la : `const essai = eleves.map((e) => e.notes);`.

## Les erreurs d'exécution et les source maps

Les types ne protègent pas de tout. Ici, `as Produit[]` **affirme** au compilateur que
les données sont correctes, sans le vérifier. Le deuxième produit n'a pas de prix.

```typescript norun
// src/main.ts
interface Produit {
  nom: string;
  prix: number;
}

function afficher(produit: Produit): void {
  console.log(`${produit.nom} : ${produit.prix.toFixed(2)} €`);
}

const texte = '[{"nom": "Stylo", "prix": 1.5}, {"nom": "Cahier"}]';
const produits = JSON.parse(texte) as Produit[];
produits.forEach(afficher);
```

`tsc` compile `src/main.ts` en `dist/main.js`, puis Node exécute ce fichier JavaScript.
La trace d'erreur parle donc du **JavaScript**, pas de votre code :

```text
Stylo : 1.50 €
file:///home/alice/projet/dist/main.js:2
    console.log(`${produit.nom} : ${produit.prix.toFixed(2)} €`);
                                                 ^

TypeError: Cannot read properties of undefined (reading 'toFixed')
    at afficher (file:///home/alice/projet/dist/main.js:2:50)
    at Array.forEach (<anonymous>)
    at file:///home/alice/projet/dist/main.js:6:10
```

La ligne 2 de `dist/main.js` n'est pas la ligne 7 de `src/main.ts` : les interfaces ont
disparu à la compilation. Une **source map** (« carte des sources ») est un fichier
`.js.map` qui relie chaque ligne du JavaScript à la ligne TypeScript d'origine.

```sh
npx tsc --sourceMap
node --enable-source-maps dist/main.js
```

```text
Stylo : 1.50 €
/home/alice/projet/src/main.ts:7
  console.log(`${produit.nom} : ${produit.prix.toFixed(2)} €`);
                                               ^

TypeError: Cannot read properties of undefined (reading 'toFixed')
    at afficher (/home/alice/projet/src/main.ts:7:48)
    at Array.forEach (<anonymous>)
    at <anonymous> (/home/alice/projet/src/main.ts:12:10)
```

Cette fois, la trace pointe vers `src/main.ts`, ligne 7, appelée depuis la ligne 12.
Mettez `"sourceMap": true` dans le `tsconfig.json` (voir [la configuration](configuration.md))
pour ne plus y penser. L'outil `tsx`, qui exécute directement un fichier `.ts`, affiche
lui aussi les bonnes lignes.

La trace se lit comme en JavaScript : la première ligne `at` est l'endroit de l'erreur,
les suivantes le chemin parcouru pour y arriver. Ici, la cause n'est pas la ligne 7 : ce
sont les **données** de la ligne 10, et le `as` de la ligne 11 qui a fait taire `tsc`.

## Afficher les valeurs avec console.log

`console.log` affiche les valeurs avec des détails utiles pour le programmeur : les
guillemets des chaînes, `null`, `undefined`.

```typescript
const valeurs = ["42", 42, null, undefined, ""];
console.log(valeurs);

const prix = 12;
const remise = undefined;
console.log("DEBUG", { prix, remise });
```

```sortie
[ '42', 42, null, undefined, '' ]
DEBUG { prix: 12, remise: undefined }
```

L'astuce `{ prix, remise }` crée un objet dont les clés sont les noms des variables :
chaque valeur arrive avec son **étiquette**.

### Les objets très imbriqués

Au-delà de deux niveaux d'imbrication, `console.log` abrège en `[Object]`. Pour tout
voir, utilisez `console.dir` avec `depth: null`, ou `JSON.stringify`.

```typescript
const eleve = {
  nom: "Alice",
  adresse: { ville: "Lyon", rue: { nom: "des Lilas", point: { x: 3, y: 4 } } },
};
console.log(eleve);
console.dir(eleve, { depth: null });
console.log(JSON.stringify(eleve));
```

```sortie
{
  nom: 'Alice',
  adresse: { ville: 'Lyon', rue: { nom: 'des Lilas', point: [Object] } }
}
{
  nom: 'Alice',
  adresse: { ville: 'Lyon', rue: { nom: 'des Lilas', point: { x: 3, y: 4 } } }
}
{"nom":"Alice","adresse":{"ville":"Lyon","rue":{"nom":"des Lilas","point":{"x":3,"y":4}}}}
```

### console.table pour les listes

`console.table` présente un tableau d'objets comme une grille.

```typescript
console.table([
  { nom: "Alice", note: 14 },
  { nom: "Bob", note: 11 },
]);
```

```sortie
┌─────────┬─────────┬──────┐
│ (index) │ nom     │ note │
├─────────┼─────────┼──────┤
│ 0       │ 'Alice' │ 14   │
│ 1       │ 'Bob'   │ 11   │
└─────────┴─────────┴──────┘
```

> **Note :** `console.error` écrit sur la sortie d'erreur et non sur la sortie normale.
> Pratique pour les messages de débogage : ils ne se mélangent pas aux résultats.

## Mettre le programme en pause

Plutôt que d'ajouter des `console.log` partout, vous pouvez **arrêter** le programme et
l'explorer. Dans VS Code, cliquez dans la marge à gauche d'une ligne : un point rouge
apparaît, c'est un **point d'arrêt**. Lancez ensuite « Exécuter et déboguer » (F5) avec
les source maps activées : le programme s'arrête sur cette ligne, et vous voyez toutes
les variables du moment. Les boutons « Pas à pas » avancent ligne par ligne.

Dans le code, le mot-clé `debugger` produit le même effet quand un débogueur est
attaché :

```typescript norun
function remise(prix: number, fidele: boolean): number {
  const taux = fidele ? 0.1 : 0;
  debugger; // le programme s'arrête ici sous le débogueur
  return prix - prix * taux;
}
```

```sh
node --inspect-brk --enable-source-maps dist/main.js
```

## @ts-expect-error et @ts-ignore

Il arrive qu'on **veuille** une erreur de type : pour tester qu'une fonction refuse
bien un mauvais argument, par exemple. Le commentaire `// @ts-expect-error` dit : « la
ligne suivante contient une erreur, je le sais ».

```typescript
function carre(n: number): number {
  return n * n;
}

// @ts-expect-error : on vérifie qu'une chaîne est refusée
const r = carre("4");
console.log(r);
```

```sortie
16
```

`tsc` accepte le fichier. Le JavaScript s'exécute quand même, et `"4" * "4"` donne 16 :
voilà pourquoi ces commentaires sont dangereux s'ils traînent dans du vrai code.

Son cousin `// @ts-ignore` fait taire la ligne suivante **sans condition**. La différence
est importante :

| Commentaire | Si la ligne a une erreur | Si la ligne n'a pas d'erreur |
|-------------|--------------------------|------------------------------|
| `// @ts-expect-error` | erreur cachée | **erreur TS2578** : le commentaire est inutile |
| `// @ts-ignore` | erreur cachée | rien, le commentaire reste là pour toujours |

Préférez toujours `@ts-expect-error`, avec une explication après les deux-points : le
jour où le problème est corrigé, `tsc` vous prévient qu'il faut retirer le commentaire.

## Erreurs fréquentes

### Croire que as vérifie les données

```typescript norun
interface Produit {
  nom: string;
  prix: number;
}

const texte = '{"nom": "Stylo"}';
const produit = JSON.parse(texte) as Produit;
console.log(produit.prix.toFixed(2));
```

```text
console.log(produit.prix.toFixed(2));
                         ^

TypeError: Cannot read properties of undefined (reading 'toFixed')
```

`tsc` ne dit rien : `as` est une promesse faite au compilateur, pas une vérification.
Les données venues de l'extérieur (JSON, fichier, réseau) doivent être contrôlées à
l'exécution. Typez-les d'abord en `unknown`, puis vérifiez :

```typescript
interface Produit {
  nom: string;
  prix: number;
}

function estProduit(x: unknown): x is Produit {
  return typeof x === "object" && x !== null &&
    "nom" in x && typeof x.nom === "string" &&
    "prix" in x && typeof x.prix === "number";
}

const donnees: unknown = JSON.parse('{"nom": "Stylo"}');
if (estProduit(donnees)) {
  console.log(donnees.prix.toFixed(2));
} else {
  console.log("Produit invalide");
}
```

```sortie
Produit invalide
```

### Laisser un @ts-expect-error devenu inutile

```typescript norun
// @ts-expect-error
const n: number = 42;
console.log(n);
```

```text
main.ts(1,1): error TS2578: Unused '@ts-expect-error' directive.
```

La ligne suivante n'a plus d'erreur : le commentaire ne sert à rien. Supprimez-le. C'est
justement ce que `@ts-ignore` ne vous aurait jamais signalé.

### Corriger la mauvaise ligne

Devant `'bonne' is possibly 'undefined'`, la tentation est d'ajouter un `!` :
`bonne!.toFixed(1)`. L'erreur disparaît… et revient au moment de l'exécution :

```typescript norun
const notes = [12, 15, 9];
const bonne = notes.find((n) => n > 18);
console.log(bonne!.toFixed(1));
```

```text
console.log(bonne.toFixed(1));
                  ^

TypeError: Cannot read properties of undefined (reading 'toFixed')
```

La trace montre le JavaScript compilé : le `!` a disparu, il n'existe que pour `tsc`.
Il dit « je te jure que ce n'est pas `undefined` ». C'était faux. Traitez le cas
`undefined` avec un `if` ou `??`, comme dans la section sur TS18048.

## Testez-vous

**Question 1 :** un message de `tsc` fait quatre lignes. Laquelle lire en premier, et
pourquoi ?

> **Réponse**
>
> La dernière : c'est la cause la plus précise (par exemple `Type 'string' is not
> assignable to type 'number'`). Les lignes du dessus indiquent où elle se trouve dans
> les types imbriqués.

**Question 2 :** à quoi sert `node --enable-source-maps` ?

> **Réponse**
>
> À afficher dans les traces d'erreur les lignes du fichier TypeScript d'origine, au lieu
> de celles du JavaScript compilé. Il faut avoir compilé avec `sourceMap` activé.

**Question 3 :** quelle différence entre `// @ts-expect-error` et `// @ts-ignore` ?

> **Réponse**
>
> Les deux cachent l'erreur de la ligne suivante. Mais `@ts-expect-error` produit une
> erreur (TS2578) s'il n'y a plus rien à cacher : on n'oublie pas de le retirer.
> `@ts-ignore` reste silencieux pour toujours.

## En résumé

- `tsc` trouve les erreurs de type avant l'exécution : fichier, ligne, colonne, code et
  message.
- Un message long se lit **de bas en haut** : la dernière ligne donne la cause.
- TS2322, TS2339, TS2345, TS7006, TS18048 et TS2532 couvrent la plupart des erreurs de
  débutant.
- Les source maps (`sourceMap` + `node --enable-source-maps`) ramènent les traces vers
  vos fichiers `.ts`.
- `console.log({ x, y })`, `console.dir` et `console.table` montrent l'état du programme ;
  le survol dans l'éditeur montre les types.
- `@ts-expect-error` plutôt que `@ts-ignore`, et jamais `as` ni `!` pour faire taire une
  vraie erreur.

Suite : [bien écrire du TypeScript](style.md).
