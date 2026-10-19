# Map, Set et Record

Cette page explique comment associer des clés à des valeurs avec `Map` et `Record`, et comment garder des valeurs sans doublons avec `Set`.

> **Objectifs :**
>
> - vous saurez associer des clés à des valeurs avec `Map<K, V>` ;
> - vous saurez gérer le `undefined` renvoyé par `get` ;
> - vous saurez retirer les doublons avec `Set<T>` ;
> - vous saurez décrire un objet clé-valeur avec `Record` et choisir entre `Record` et
>   `Map`.

Ce chapitre suit [tableaux et tuples](collections.md) : vous y retrouverez `for...of`,
la déstructuration `[a, b]` et le réflexe `??` vu avec `find`.

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

### Lire une Map avec des crochets

```typescript norun
const stock = new Map<string, number>([["pommes", 10]]);
console.log(stock["pommes"]);
```

```text
main.ts:2:13 - error TS7052: Element implicitly has an 'any' type because type 'Map<string, number>' has no index signature. Did you mean to call 'stock.get'?

2 console.log(stock["pommes"]);
              ~~~~~~~~~~~~~~~


Found 1 error in main.ts:2
```

Une `Map` n'est pas un objet : ses valeurs ne se lisent pas avec des crochets. Le
message le suggère : écrivez `stock.get("pommes")`.

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

### Oublier une clé dans un Record

```typescript norun
type Jour = "lundi" | "mardi" | "mercredi";
const heures: Record<Jour, number> = { lundi: 7, mardi: 8 };
```

```text
main.ts:2:7 - error TS2741: Property 'mercredi' is missing in type '{ lundi: number; mardi: number; }' but required in type 'Record<Jour, number>'.

2 const heures: Record<Jour, number> = { lundi: 7, mardi: 8 };
        ~~~~~~


Found 1 error in main.ts:2
```

Avec une union comme type des clés, chaque jour doit avoir sa valeur. Ajoutez la clé
manquante : `{ lundi: 7, mardi: 8, mercredi: 4 }`.

## Testez-vous

**Question 1 :** que vaut `new Set(["a", "b", "a"]).size` ?

> **Réponse**
>
> `2`. Un `Set` ne garde chaque valeur qu'une fois : le second `"a"` est ignoré.

**Question 2 :** `ages` est une `Map<string, number>`. Quel est le type de
`ages.get("Bob")`, et pourquoi ?

> **Réponse**
>
> `number | undefined`. La clé `"Bob"` n'existe peut-être pas : dans ce cas, `get`
> renvoie `undefined`. Traitez-le avec `if`, `?.` ou `??`.

**Question 3 :** complétez pour afficher le nombre de prénoms différents.

```typescript norun
const inscrits = ["Léa", "Tom", "Léa", "Inès", "Tom"];
```

> **Réponse**
>
> ```typescript
> const inscrits = ["Léa", "Tom", "Léa", "Inès", "Tom"];
> console.log(new Set(inscrits).size);
> ```
>
> ```sortie
> 3
> ```
>
> Le `Set` retire les doublons ; `size` compte ce qui reste.

## En résumé

- `Map<K, V>` associe des clés à des valeurs : `set`, `get`, `has`, `delete` et
  `size`.
- `get` renvoie `V | undefined` : traitez le cas avec `if`, `?.` ou `??`.
- `Set<T>` garde chaque valeur une seule fois ; `[...new Set(tableau)]` retire les
  doublons d'un tableau.
- `Record<string, number>` décrit un objet clé-valeur ; avec une union de clés, il
  les exige toutes.
- `Record` pour des données fixes écrites dans le code, `Map` quand les clés changent
  souvent.

Suite : [objets, interfaces et alias de type](objets.md).
