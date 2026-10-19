# Tableaux et tuples

Cette page explique comment ranger plusieurs valeurs dans des tableaux, des tuples, des `Map` et des `Set`, et comment les types gardent ces collections cohérentes.

> **Objectifs :**
>
> - vous saurez créer un tableau typé (`number[]`), le lire et le modifier ;
> - vous saurez protéger un tableau avec `readonly` et décrire un tuple
>   `[string, number]` ;
> - vous saurez transformer un tableau avec `map`, `filter` et `reduce` ;
> - vous saurez gérer le `undefined` renvoyé par `find` ;
> - vous saurez utiliser `Map`, `Set` et `Record`.

## Créer un tableau

Imaginez une liste de courses : une seule feuille, plusieurs articles, dans un ordre
précis. Un tableau joue ce rôle dans un programme : il range plusieurs valeurs sous un
seul nom.

Un tableau est une [liste](../debuter/glossaire.md#liste) ordonnée de valeurs, écrite
entre crochets. Son type s'écrit avec le type des éléments suivi de `[]` :

```typescript
const notes: number[] = [12, 15, 9];
const prenoms: string[] = ["Alice", "Bob", "Chloé"];
const vide: number[] = [];

console.log(notes);
console.log(prenoms.length, vide.length);
```

```sortie
[ 12, 15, 9 ]
3 0
```

Ligne par ligne :

- `notes` reçoit trois nombres ; `number[]` annonce « uniquement des nombres » ;
- `prenoms` reçoit trois textes, et `vide` un tableau sans aucun élément ;
- `console.log(notes)` affiche le tableau entier, entre crochets ;
- `.length` donne le nombre d'éléments : 3 pour `prenoms`, 0 pour `vide`.

`number[]` se lit « tableau de nombres ». Une autre écriture existe, qui veut dire
exactement la même chose : `Array<number>`. Les chevrons `< >` donnent le type des
éléments ; vous les retrouverez dans [les génériques](generiques.md).

```typescript
const villes: Array<string> = ["Lyon", "Nantes"];
console.log(villes);
```

```sortie
[ 'Lyon', 'Nantes' ]
```

Avec des valeurs au départ, TypeScript devine le type : `const notes = [12, 15, 9]`
donne aussi un `number[]`. Écrivez le type pour un tableau vide : sans lui, TypeScript
ne peut pas savoir ce que vous allez y ranger.

> **À retenir :** tous les éléments d'un tableau ont le même type. Un tableau de
> nombres refuse un texte : c'est la garantie que chaque élément se manipule de la même
> façon.

## Lire et modifier

Une liste vit : vous regardez un article, vous en changez un, vous en ajoutez un en
bas, vous rayez le dernier. Un tableau permet tout cela.

Chaque élément a un [indice](../debuter/glossaire.md#indice) qui commence à **0**.

```typescript
const fruits: string[] = ["pomme", "poire", "kiwi"];

console.log(fruits[0]);
console.log(fruits.at(-1));
fruits[1] = "mangue";
fruits.push("fraise");
console.log(fruits);

const dernier = fruits.pop();
console.log(dernier, fruits);
console.log(fruits.includes("kiwi"), fruits.indexOf("kiwi"));
```

```sortie
pomme
kiwi
[ 'pomme', 'mangue', 'kiwi', 'fraise' ]
fraise [ 'pomme', 'mangue', 'kiwi' ]
true 2
```

Ligne par ligne :

- `fruits[0]` lit la première case : `"pomme"` ; `at(-1)` compte depuis la fin :
  `"kiwi"` ;
- `fruits[1] = "mangue"` remplace `"poire"`, puis `push` ajoute `"fraise"` : 4 éléments ;
- `pop()` retire `"fraise"` et la range dans `dernier` : il reste 3 éléments ;
- `"kiwi"` est bien là, à la position 2 (la troisième case, car on compte depuis 0).

Les méthodes à connaître :

- `fruits[1] = ...` remplace un élément ;
- `push` ajoute à la fin, `pop` retire et renvoie le dernier ;
- `includes` répond `true` ou `false`, `indexOf` donne la position (`-1` si absent).

Même avec `const`, le tableau peut changer : `const` empêche seulement de remplacer
**tout** le tableau par un autre.

> **Attention :** lire un indice qui n'existe pas (`fruits[10]`) donne `undefined`,
> mais TypeScript annonce quand même un `string`. Vérifiez la longueur avant, ou
> préférez `at()`, dont le type `string | undefined` vous rappelle le risque.

**À vous :** écrivez un programme qui crée un tableau `courses` avec `"pain"` et
`"lait"`, ajoute `"œufs"` à la fin, puis affiche le nombre d'articles et le tableau.

> **Solution**
>
> ```typescript
> const courses: string[] = ["pain", "lait"];
> courses.push("œufs");
> console.log(courses.length);
> console.log(courses);
> ```
>
> ```sortie
> 3
> [ 'pain', 'lait', 'œufs' ]
> ```

### Parcourir un tableau

`for...of` donne chaque élément. Avec `entries()`, vous obtenez aussi l'indice.

```typescript
const podium = ["Alice", "Bob", "Chloé"];

for (const [i, nom] of podium.entries()) {
  console.log(`${i + 1}. ${nom}`);
}
```

```sortie
1. Alice
2. Bob
3. Chloé
```

`const [i, nom]` découpe chaque paire en deux variables : c'est la
**déstructuration**, expliquée avec les tuples plus bas.

## Les tableaux en lecture seule

Certaines listes ne doivent jamais changer : les jours de la semaine, les mois de
l'année. `readonly` vous protège d'une modification faite par erreur.

Ajoutez `readonly` devant le type : le tableau ne peut plus être modifié. `push`,
`pop` ou `fruits[0] = ...` sont refusés.

```typescript
const jours: readonly string[] = ["lundi", "mardi", "mercredi"];

console.log(jours[0], jours.length);
console.log(jours.includes("mardi"));
```

```sortie
lundi 3
true
```

Seules les méthodes qui **lisent** restent disponibles. C'est utile pour une liste
fixe, ou pour promettre qu'une fonction ne touchera pas au tableau qu'on lui donne :

```typescript
function moyenne(valeurs: readonly number[]): number {
  let total = 0;
  for (const v of valeurs) {
    total += v;
  }
  return total / valeurs.length;
}

console.log(moyenne([12, 15, 9]));
```

```sortie
12
```

## Les tuples

Parfois, deux ou trois valeurs de types différents vont ensemble, comme une ligne d'un
tableau des scores : un prénom et des points. Le tuple les garde côte à côte, dans un
ordre fixe.

Un [tuple](../debuter/glossaire.md#tuple) est un tableau de **longueur fixe**, où
chaque position a son propre type. `[string, number]` veut dire : « exactement deux
éléments, un texte puis un nombre ».

```typescript
const joueur: [string, number] = ["Alice", 1200];

console.log(joueur[0].toUpperCase());
console.log(joueur[1] + 50);

const [nom, score] = joueur;
console.log(`${nom} a ${score} points.`);
```

```sortie
ALICE
1250
Alice a 1200 points.
```

Ligne par ligne :

- `joueur` contient deux cases : `"Alice"` (un texte) puis `1200` (un nombre) ;
- `joueur[0].toUpperCase()` met le texte en majuscules : `ALICE` ;
- `joueur[1] + 50` calcule avec le nombre : `1250`. `joueur` ne change pas ;
- la fin range `"Alice"` dans `nom` et `1200` dans `score`, puis affiche la phrase.

TypeScript sait que `joueur[0]` est un `string` et `joueur[1]` un `number`. La ligne
`const [nom, score] = joueur;` range le premier élément dans `nom` et le second dans
`score`.

Un tuple est pratique pour renvoyer deux valeurs d'une fonction :

```typescript
function minMax(valeurs: number[]): [number, number] {
  return [Math.min(...valeurs), Math.max(...valeurs)];
}

const [petit, grand] = minMax([4, 9, 1, 7]);
console.log(petit, grand);
```

```sortie
1 9
```

`...valeurs` (la **décomposition**, *spread*) étale le tableau en arguments séparés :
`Math.min(4, 9, 1, 7)`.

> **Astuce :** au-delà de deux ou trois éléments, un tuple devient difficile à lire
> (que veut dire `joueur[3]` ?). Préférez alors un objet avec des noms de propriétés :
> voir [les objets](objets.md).

**À vous :** écrivez un programme qui range un produit dans un tuple
`[string, number]` : `"stylo"` et `2`. Découpez-le en deux variables, puis affichez
« stylo coûte 2 € ».

> **Solution**
>
> ```typescript
> const produit: [string, number] = ["stylo", 2];
> const [article, prix] = produit;
> console.log(`${article} coûte ${prix} €`);
> ```
>
> ```sortie
> stylo coûte 2 €
> ```

## Transformer : map, filter, reduce

Très souvent, vous voulez faire la même chose à toute une liste : augmenter tous les
prix, garder les bonnes notes, calculer un total. Ces méthodes le font en une ligne,
sans écrire de boucle.

Ces trois méthodes prennent un *callback* (une fonction, voir
[les fonctions](fonctions.md#les-callbacks-passer-une-fonction-a-une-fonction)). Elles
ne modifient pas le tableau d'origine : elles en renvoient un nouveau, ou une valeur.

### map : transformer chaque élément

```typescript
const prix = [10, 20, 35];
const prixTTC = prix.map((p) => p * 1.2);
const etiquettes = prix.map((p) => `${p} €`);

console.log(prixTTC);
console.log(etiquettes);
```

```sortie
[ 12, 24, 42 ]
[ '10 €', '20 €', '35 €' ]
```

Ligne par ligne :

- `prix` contient trois nombres ;
- `prix.map((p) => p * 1.2)` donne chaque prix à `p`, l'un après l'autre, et range
  chaque résultat dans un nouveau tableau : `[12, 24, 42]` ;
- la ligne suivante fait de même, mais le callback fabrique un texte : `"10 €"`… ;
- `prix` lui-même n'a pas changé : il vaut toujours `[10, 20, 35]`.

TypeScript devine tout : `p` est un `number` (un élément de `prix`), `prixTTC` est un
`number[]`, et `etiquettes` un `string[]`, car le callback renvoie un texte. Survolez
ces noms dans l'éditeur pour le voir.

### filter : garder certains éléments

```typescript
const notes = [12, 7, 15, 9, 18];
const recus = notes.filter((n) => n >= 10);

console.log(recus);
console.log(notes);
```

```sortie
[ 12, 15, 18 ]
[ 12, 7, 15, 9, 18 ]
```

Le callback répond `true` (garder) ou `false` (jeter). `notes` n'a pas changé.

### reduce : tout résumer en une valeur

`reduce` parcourt le tableau en gardant un **accumulateur** : une valeur qui grossit à
chaque tour, comme une tirelire. Le deuxième argument est sa valeur de départ.

```typescript
const achats = [4.5, 12, 3.5];
const total = achats.reduce((somme, prix) => somme + prix, 0);
console.log(total);

const mots = ["chat", "éléphant", "ours"];
const lettres = mots.reduce((acc, mot) => acc + mot.length, 0);
console.log(lettres);
```

```sortie
20
16
```

Au premier tour, `somme` vaut `0` (la valeur de départ) ; puis `4.5`, `16.5`, et enfin
`20`. Le type de l'accumulateur vient de la valeur de départ : `0` donne un `number`.

**À vous :** à partir de `const nombres = [2, 5, 8];`, écrivez un programme qui crée
le tableau des doubles avec `map`, puis calcule la somme de `nombres` avec `reduce`.
Affichez les deux résultats.

> **Solution**
>
> ```typescript
> const nombres = [2, 5, 8];
> const doubles = nombres.map((n) => n * 2);
> const somme = nombres.reduce((acc, n) => acc + n, 0);
> console.log(doubles);
> console.log(somme);
> ```
>
> ```sortie
> [ 4, 10, 16 ]
> 15
> ```

### Enchaîner

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Comme chaque méthode renvoie un tableau, on peut les enchaîner :

```typescript
const notes = [12, 7, 15, 9, 18];

const totalAvecBonus = notes
  .filter((n) => n >= 10)
  .map((n) => n + 1)
  .reduce((acc, n) => acc + n, 0);

console.log(totalAvecBonus);
```

```sortie
48
```

Lisez de haut en bas : on garde les notes reçues (`12, 15, 18`), on ajoute un point
de bonus à chacune (`13, 16, 19`), puis on fait la somme.

### Trier

`sort` trie le tableau **sur place**. Attention : sans callback, il trie comme du texte,
même des nombres ! Donnez toujours une fonction de comparaison `(a, b) => a - b`.

```typescript
const nombres = [10, 9, 1, 100];

console.log([...nombres].sort());
console.log([...nombres].sort((a, b) => a - b));
console.log([...nombres].sort((a, b) => b - a));
```

```sortie
[ 1, 10, 100, 9 ]
[ 1, 9, 10, 100 ]
[ 100, 10, 9, 1 ]
```

`[...nombres]` crée une copie, pour ne pas modifier `nombres` lui-même.

## find : un résultat peut-être absent

Chercher dans une liste, c'est comme chercher un prénom dans un carnet d'adresses :
parfois, il n'y est pas. TypeScript vous oblige à prévoir ce cas.

`find` renvoie le **premier** élément qui satisfait le test. S'il n'en trouve aucun,
il renvoie `undefined`. Son type le dit franchement : pour un `string[]`, c'est
`string | undefined`.

```typescript
const prenoms = ["Alice", "Bob", "Chloé"];

const avecC = prenoms.find((p) => p.startsWith("C"));
const avecZ = prenoms.find((p) => p.startsWith("Z"));

console.log(avecC, avecZ);
```

```sortie
Chloé undefined
```

Ligne par ligne :

- `find` essaie chaque prénom avec le test `p.startsWith("C")` (« commence par C ? ») ;
- `"Alice"` et `"Bob"` échouent, `"Chloé"` réussit : `avecC` vaut `"Chloé"` ;
- aucun prénom ne commence par Z : `avecZ` vaut `undefined`.

C'est là que TypeScript vous protège : vous ne pouvez pas utiliser le résultat comme
un `string` sans traiter le cas `undefined`. Trois façons de le faire :

```typescript
const prenoms = ["Alice", "Bob", "Chloé"];
const trouve = prenoms.find((p) => p.startsWith("B"));

// 1. Tester avant d'utiliser
if (trouve !== undefined) {
  console.log(trouve.toUpperCase());
} else {
  console.log("Personne.");
}

// 2. Appeler seulement si la valeur existe
console.log(trouve?.toLowerCase());

// 3. Donner une valeur par défaut
const nom = trouve ?? "inconnu";
console.log(nom.length);
```

```sortie
BOB
bob
3
```

Après le test `trouve !== undefined`, TypeScript sait que `trouve` est un `string`.

> **À retenir :** un type `T | undefined` est une question que TypeScript vous pose :
> « et si rien n'est trouvé ? ». Répondez-y, avec `if`, `?.` ou `??`.

## Map : associer des clés à des valeurs

Une `Map` est un [dictionnaire](../debuter/glossaire.md#dictionnaire) : elle associe
une **clé** à une **valeur**, comme un annuaire associe un nom à un numéro. Son type
s'écrit `Map<TypeDesCles, TypeDesValeurs>`.

```typescript
const stock = new Map<string, number>();

stock.set("pommes", 10);
stock.set("poires", 4);
stock.set("pommes", 12);

console.log(stock.get("pommes"));
console.log(stock.has("kiwis"), stock.size);

for (const [fruit, quantite] of stock) {
  console.log(`${fruit} : ${quantite}`);
}
```

```sortie
12
false 2
pommes : 12
poires : 4
```

Ligne par ligne :

- `new Map<string, number>()` crée une `Map` vide : des clés textes, des valeurs
  nombres ;
- les `set` rangent `pommes → 10` et `poires → 4`, puis remplacent 10 par 12 ;
- `get("pommes")` donne 12 ; la clé `"kiwis"` n'existe pas, et la `Map` a 2 paires ;
- la boucle `for...of` donne chaque paire, dans l'ordre d'ajout.

Les méthodes à connaître :

- `set(clé, valeur)` ajoute ou remplace (une clé est unique) ;
- `get(clé)` lit la valeur, `has(clé)` teste si la clé existe ;
- `size` donne le nombre de paires ; `delete(clé)` en retire une.

`get` renvoie `number | undefined` : la clé n'existe peut-être pas. Même réflexe
qu'avec `find` :

```typescript
const stock = new Map<string, number>([["pommes", 10]]);

const kiwis = stock.get("kiwis") ?? 0;
console.log(kiwis + 5);
```

```sortie
5
```

**À vous :** écrivez un programme qui crée une `Map` des âges, y range `"Alice"`
(30 ans) et `"Bob"` (25 ans), puis affiche l'âge de Bob et celui de `"Zoé"`, avec 0
si elle est absente.

> **Solution**
>
> ```typescript
> const ages = new Map<string, number>();
> ages.set("Alice", 30);
> ages.set("Bob", 25);
> console.log(ages.get("Bob"));
> console.log(ages.get("Zoé") ?? 0);
> ```
>
> ```sortie
> 25
> 0
> ```

## Set : une collection sans doublons

Pensez à la liste des invités d'une fête : écrire deux fois le même nom ne sert à
rien. Un `Set` s'en charge pour vous et ignore les doublons.

Un `Set` est un [ensemble](../debuter/glossaire.md#ensemble) : chaque valeur n'y
apparaît qu'une fois. Son type s'écrit `Set<T>`.

```typescript
const visiteurs = new Set<string>();

visiteurs.add("Alice");
visiteurs.add("Bob");
visiteurs.add("Alice");

console.log(visiteurs.size);
console.log(visiteurs.has("Bob"));

const sansDoublons = [...new Set([3, 1, 3, 2, 1])];
console.log(sansDoublons);
```

```sortie
2
true
[ 3, 1, 2 ]
```

Ligne par ligne :

- `visiteurs` commence vide ;
- `add` ajoute `"Alice"`, puis `"Bob"`. Le second `"Alice"` est ignoré : elle y est
  déjà ;
- `size` vaut donc 2, et `has("Bob")` répond `true` ;
- la fin crée un `Set` à partir du tableau, puis le remet en tableau avec `[...]`.

`new Set(tableau)` puis `[...]` est l'astuce classique pour retirer les doublons d'un
tableau, en gardant l'ordre de première apparition.

## Record : un objet clé-valeur

Un `Record` ressemble à une fiche de scores : un nom, un nombre, un nom, un nombre.
Vous l'écrivez d'un coup, comme un objet ordinaire.

`Record<string, number>` décrit un objet simple dont les clés sont des textes et les
valeurs des nombres. C'est plus léger qu'une `Map`, et on l'écrit directement avec des
accolades.

```typescript
const scores: Record<string, number> = {
  alice: 12,
  bob: 8,
};

scores.chloe = 15;
scores["bob"] += 1;

console.log(scores);
console.log(Object.keys(scores));

for (const [nom, points] of Object.entries(scores)) {
  console.log(nom.padEnd(6), points);
}
```

```sortie
{ alice: 12, bob: 9, chloe: 15 }
[ 'alice', 'bob', 'chloe' ]
alice  12
bob    9
chloe  15
```

Ligne par ligne :

- `scores` démarre avec deux paires : `alice → 12` et `bob → 8` ;
- `scores.chloe = 15` ajoute une clé ; `scores["bob"] += 1` fait passer Bob à 9 ;
- `Object.keys` liste les clés, dans l'ordre d'ajout ;
- la boucle donne chaque paire ; `padEnd(6)` complète le nom avec des espaces, pour
  aligner les colonnes.

`Object.keys` donne les clés, `Object.entries` les paires `[clé, valeur]`.

Avec une union de types littéraux comme clés, `Record` exige **toutes** les clés :

```typescript
type Jour = "lundi" | "mardi" | "mercredi";

const heures: Record<Jour, number> = {
  lundi: 7,
  mardi: 8,
  mercredi: 4,
};

console.log(heures.mardi);
```

```sortie
8
```

Oublier `mercredi` serait une erreur.

> **Astuce :** utilisez `Record` pour des données fixes écrites dans le code, et `Map`
> quand vous ajoutez et retirez souvent des clés.

## Erreurs fréquentes

### Ranger une valeur du mauvais type

```typescript norun
const notes: number[] = [12, 15];
notes.push("18");
```

```text
main.ts:2:12 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

2 notes.push("18");
             ~~~~


Found 1 error in main.ts:2
```

`"18"` est un texte. Un `number[]` n'accepte que des nombres : `notes.push(18)`.

### Modifier un tableau readonly

```typescript norun
const jours: readonly string[] = ["lundi", "mardi"];
jours.push("mercredi");
```

```text
main.ts:2:7 - error TS2339: Property 'push' does not exist on type 'readonly string[]'.

2 jours.push("mercredi");
        ~~~~


Found 1 error in main.ts:2
```

Un tableau `readonly` n'a pas de méthode `push`. Créez un nouveau tableau à la place :
`const semaine = [...jours, "mercredi"];`.

### Inverser l'ordre d'un tuple

```typescript norun
const joueur: [string, number] = [42, "Alice"];
```

```text
main.ts:1:35 - error TS2322: Type 'number' is not assignable to type 'string'.

1 const joueur: [string, number] = [42, "Alice"];
                                    ~~

main.ts:1:39 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const joueur: [string, number] = [42, "Alice"];
                                        ~~~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

Dans un tuple, l'ordre compte : d'abord le texte, puis le nombre.
Écrivez `["Alice", 42]`.

### Utiliser le résultat de get sans vérifier

```typescript norun
const stock = new Map<string, number>([["pommes", 10]]);
const n = stock.get("poires");
console.log(n + 1);
```

```text
main.ts:3:13 - error TS18048: 'n' is possibly 'undefined'.

3 console.log(n + 1);
              ~


Found 1 error in main.ts:3
```

La clé `"poires"` n'existe peut-être pas. Donnez une valeur par défaut :
`const n = stock.get("poires") ?? 0;`.

## Testez-vous

**Question 1 :** quel est le type de `["a", "b"].map((s) => s.length)` ?

> **Réponse**
>
> `number[]`. Le callback renvoie `s.length`, un nombre, pour chaque élément.

**Question 2 :** quelle différence entre `string[]` et `[string, string]` ?

> **Réponse**
>
> `string[]` est un tableau de textes de n'importe quelle longueur. `[string, string]`
> est un tuple : exactement deux textes.

**Question 3 :** complétez pour afficher le premier nombre pair, ou `aucun`.

```typescript norun
const nombres = [3, 7, 8, 5];
const pair = nombres.find((n) => n % 2 === 0);
```

> **Réponse**
>
> ```typescript
> const nombres = [3, 7, 8, 5];
> const pair = nombres.find((n) => n % 2 === 0);
> console.log(pair ?? "aucun");
> ```
>
> ```sortie
> 8
> ```
>
> `pair` est de type `number | undefined` : `??` traite le cas où rien n'est trouvé.

## En résumé

- Un tableau s'écrit `number[]` ou `Array<number>` ; tous ses éléments ont le même
  type. `readonly number[]` l'empêche de changer.
- Un tuple `[string, number]` a une longueur fixe et un type par position.
- `map`, `filter` et `reduce` renvoient de nouvelles valeurs, dont TypeScript devine le
  type ; triez les nombres avec `sort((a, b) => a - b)`.
- `find` et `Map.get` renvoient `T | undefined` : traitez le cas avec `if`, `?.` ou
  `??`.
- `Map<K, V>` associe des clés à des valeurs, `Set<T>` évite les doublons,
  `Record<string, number>` décrit un objet clé-valeur.

Suite : [objets, interfaces et alias de type](objets.md).
