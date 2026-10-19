# Les tableaux en profondeur

Cette page va plus loin avec les tableaux : aplatir, regrouper, trier sans modifier, chercher depuis la fin, copier vraiment, et comprendre `reduce` pas à pas.

> **Objectifs :**
>
> - vous saurez aplatir un tableau avec `flat` et `flatMap` ;
> - vous saurez regrouper des éléments avec `Object.groupBy` ;
> - vous saurez trier ou modifier un tableau **sans toucher l'original** avec `toSorted`,
>   `toReversed` et `with` ;
> - vous saurez faire une vraie copie avec `structuredClone` ;
> - vous saurez suivre un `reduce` étape par étape.

## Avant de commencer

Cette page suppose que vous connaissez [les tableaux](collections.md) : `push`, `map`,
`filter`, `find` et `sort`. Si ce n'est pas le cas, lisez d'abord cette page.

Une idée revient partout ici. Une méthode de tableau fait l'une de ces deux choses :

```text
 MODIFIE le tableau            RENVOIE un nouveau tableau
 (on écrit dans le cahier)     (on fait une photocopie)
 push, pop, sort, reverse      map, filter, flat, toSorted, with
```

Savoir dans quelle colonne est une méthode évite beaucoup de surprises.

## Le dernier élément avec at

`at(-1)` donne le dernier élément. Les nombres négatifs comptent **depuis la fin**.

```javascript
const file = ["Alice", "Bob", "Chloé"];

console.log(file.at(0));
console.log(file.at(-1));
console.log(file.at(-2));
console.log(file[file.length - 1]);
```

```sortie
Alice
Chloé
Bob
Chloé
```

`file.at(-1)` et `file[file.length - 1]` font la même chose. La première forme se lit
mieux : « l'élément 1 en partant de la fin ».

> **Attention :** `file[-1]` ne marche pas : il donne `undefined`. Seul `at` comprend
> les nombres négatifs.

## Chercher depuis la fin : findLast

`find` part du début. `findLast` part de **la fin** et renvoie le premier élément qui
convient en remontant.

```javascript
const temperatures = [12, 25, 18, 27, 15];

console.log(temperatures.find((t) => t > 20));
console.log(temperatures.findLast((t) => t > 20));
console.log(temperatures.findLastIndex((t) => t > 20));
```

```sortie
25
27
3
```

C'est pratique pour « le **dernier** jour où il a fait plus de 20 degrés ».

Essayez : remplacez `20` par `30`. Les deux premières lignes affichent `undefined`, la
dernière affiche `-1` : rien ne convient.

## some et every : oui ou non ?

Ces deux méthodes répondent par `true` ou `false`.

- `some` : « **au moins un** élément convient ? »
- `every` : « **tous** les éléments conviennent ? »

```javascript
const notes = [12, 8, 15];

console.log(notes.some((n) => n < 10));
console.log(notes.every((n) => n >= 10));
console.log(notes.every((n) => n > 0));
```

```sortie
true
false
true
```

Pensez à un contrôle de sac à l'entrée d'un concert. `some` cherche s'il y a **un**
objet interdit. `every` vérifie que **tous** les objets sont autorisés. Dès que la
réponse est connue, la recherche s'arrête.

> **À retenir :** `some` = « au moins un » ; `every` = « tous ».

## Aplatir avec flat et flatMap

### flat : déplier les boîtes dans les boîtes

Un tableau peut contenir d'autres tableaux. `flat` les **déplie** d'un niveau.

```javascript
const tiroirs = [["chaussettes", "pulls"], ["livres"], ["stylos", "gommes"]];

console.log(tiroirs.flat());
console.log([1, [2, [3, [4]]]].flat());
console.log([1, [2, [3, [4]]]].flat(Infinity));
```

```sortie
[ 'chaussettes', 'pulls', 'livres', 'stylos', 'gommes' ]
[ 1, 2, [ 3, [ 4 ] ] ]
[ 1, 2, 3, 4 ]
```

```text
 [ [a, b], [c] ]   ──flat()──▶   [ a, b, c ]
```

Par défaut, `flat` déplie **un** niveau. `flat(Infinity)` déplie tout, quelle que soit
la profondeur.

### flatMap : transformer puis aplatir

`flatMap(f)` fait un `map(f)`, puis un `flat()`. C'est utile quand chaque élément donne
**plusieurs** résultats.

```javascript
const phrases = ["il fait beau", "allons dehors"];

console.log(phrases.map((p) => p.split(" ")));
console.log(phrases.flatMap((p) => p.split(" ")));
```

```sortie
[ [ 'il', 'fait', 'beau' ], [ 'allons', 'dehors' ] ]
[ 'il', 'fait', 'beau', 'allons', 'dehors' ]
```

Avec `map`, on obtient un tableau de tableaux. Avec `flatMap`, une seule liste de mots.

Essayez : `[1, 2, 3].flatMap((n) => [n, n])`. Vous obtenez `[1, 1, 2, 2, 3, 3]` :
chaque nombre est doublé.

## Regrouper avec Object.groupBy

Imaginez que vous rangez du linge par couleur : une pile de blanc, une pile de couleur.
`Object.groupBy` fait ces piles pour vous.

Vous lui donnez le tableau et une fonction qui dit **dans quelle pile** va chaque
élément.

```javascript
const fruits = [
  { nom: "pomme", couleur: "rouge" },
  { nom: "kiwi", couleur: "vert" },
  { nom: "fraise", couleur: "rouge" },
];

const piles = Object.groupBy(fruits, (f) => f.couleur);

console.log(piles.rouge.map((f) => f.nom));
console.log(piles.vert.map((f) => f.nom));
console.log(Object.keys(piles));
```

```sortie
[ 'pomme', 'fraise' ]
[ 'kiwi' ]
[ 'rouge', 'vert' ]
```

Le résultat est un objet : chaque clé est le nom d'une pile, chaque valeur un tableau.

La fonction peut inventer ses propres noms de piles :

```javascript
const notes = [12, 7, 15, 9, 18];

const groupes = Object.groupBy(notes, (n) => (n >= 10 ? "reçu" : "à revoir"));

console.log(groupes["reçu"]);
console.log(groupes["à revoir"]);
```

```sortie
[ 12, 15, 18 ]
[ 7, 9 ]
```

> **Note :** si vous affichez directement le résultat avec `console.log(piles)`, Node
> écrit `[Object: null prototype]` devant. C'est un objet « tout nu », sans les méthodes
> habituelles. Vous pouvez l'utiliser normalement.

## Trier sans modifier : toSorted, toReversed, with

`sort` et `reverse` **modifient** le tableau. Parfois, vous voulez garder l'original
intact : par exemple, une liste dans l'ordre d'arrivée **et** la même liste triée.

Trois méthodes renvoient une **copie** modifiée :

| Modifie l'original | Renvoie une copie |
| --- | --- |
| `sort(f)` | `toSorted(f)` |
| `reverse()` | `toReversed()` |
| `t[i] = x` | `t.with(i, x)` |

```javascript
const arrivees = [42, 7, 19];

const triees = arrivees.toSorted((a, b) => a - b);
const inversees = arrivees.toReversed();
const corrigees = arrivees.with(1, 8);

console.log(triees);
console.log(inversees);
console.log(corrigees);
console.log(arrivees);
```

```sortie
[ 7, 19, 42 ]
[ 19, 7, 42 ]
[ 42, 8, 19 ]
[ 42, 7, 19 ]
```

La dernière ligne le prouve : `arrivees` n'a pas bougé.

Comparez avec `sort`, qui écrit dans le tableau :

```javascript
const arrivees = [42, 7, 19];
const triees = arrivees.sort((a, b) => a - b);

console.log(triees);
console.log(arrivees);
console.log(triees === arrivees);
```

```sortie
[ 7, 19, 42 ]
[ 7, 19, 42 ]
true
```

`sort` a trié `arrivees` lui-même, et `triees` est **le même** tableau.

> **À retenir :** les méthodes qui commencent par `to…` et `with` ne touchent jamais
> l'original.

## Fabriquer un tableau avec Array.from

`Array.from({ length: n }, fonction)` fabrique un tableau de `n` cases. La fonction
reçoit l'indice de chaque case et renvoie ce qu'on y met.

```javascript
const indices = Array.from({ length: 5 }, (_, i) => i);
const pairs = Array.from({ length: 5 }, (_, i) => i * 2);
const etoiles = Array.from({ length: 3 }, (_, i) => "*".repeat(i + 1));

console.log(indices);
console.log(pairs);
console.log(etoiles);
```

```sortie
[ 0, 1, 2, 3, 4 ]
[ 0, 2, 4, 6, 8 ]
[ '*', '**', '***' ]
```

Le `_` est le premier paramètre, qu'on n'utilise pas (la case vide). Seul l'indice `i`
nous intéresse.

| Case | i | `i * 2` |
| --- | --- | --- |
| 1re | 0 | 0 |
| 2e | 1 | 2 |
| 3e | 2 | 4 |
| 4e | 3 | 6 |
| 5e | 4 | 8 |

Essayez : écrivez `(_, i) => i + 1`. Vous obtenez les nombres de 1 à 5.

## Copier vraiment : spread ou structuredClone

### Le spread copie un seul niveau

`[...t]` copie le tableau. Mais si le tableau contient des **objets**, ces objets ne sont
pas copiés : la copie contient les **mêmes** objets.

```javascript
const equipe = [{ nom: "Alice", score: 10 }];
const copie = [...equipe];

copie[0].score = 99;
console.log(equipe[0].score);

copie.push({ nom: "Bob", score: 5 });
console.log(equipe.length);
```

```sortie
99
1
```

Ajouter un élément à `copie` ne change pas `equipe` : les deux tableaux sont différents.
Mais modifier l'objet d'Alice le change **dans les deux**, car c'est le même objet.

```text
 equipe ──▶ [ • ]─────┐
                      ├──▶ { nom: 'Alice', score: 99 }
 copie  ──▶ [ • , • ]─┘          (un seul objet !)
                  └──▶ { nom: 'Bob', score: 5 }
```

C'est ce qu'on appelle une copie **superficielle** : on photocopie la liste, pas les
fiches rangées dedans.

### structuredClone copie tous les niveaux

`structuredClone` fait une copie **profonde** : le tableau, et tout ce qu'il contient.

```javascript
const equipe = [{ nom: "Alice", score: 10 }];
const copie = structuredClone(equipe);

copie[0].score = 99;
console.log(equipe[0].score);
console.log(copie[0].score);
```

```sortie
10
99
```

| Copie | Le tableau | Les objets dedans |
| --- | --- | --- |
| `b = a` | partagé | partagés |
| `[...a]` | copié | partagés |
| `structuredClone(a)` | copié | copiés |

> **À retenir :** un tableau de nombres ou de textes se copie avec `[...a]`. Un tableau
> d'objets que vous allez modifier se copie avec `structuredClone(a)`.

## reduce pas à pas

`reduce` est la méthode qui fait le plus peur. Pourtant, c'est une simple boucle avec
une boîte qui se remplit.

```text
 tableau.reduce((boite, element) => nouvelle valeur de la boîte, valeur de départ)
```

À chaque tour, la fonction reçoit **la boîte** (l'accumulateur) et **l'élément**. Ce
qu'elle renvoie devient la nouvelle boîte pour le tour suivant.

### Une somme

```javascript
const panier = [4, 10, 6];
const total = panier.reduce((boite, prix) => boite + prix, 0);
console.log(total);
```

```sortie
20
```

| Tour | boite (avant) | prix | boite + prix (après) |
| --- | --- | --- | --- |
| départ | – | – | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 6 | 20 |

Après le dernier tour, `reduce` renvoie la boîte : `20`.

### Le même calcul avec une boucle

Voici exactement ce que fait `reduce`, écrit avec une boucle :

```javascript
const panier = [4, 10, 6];

let boite = 0;
for (const prix of panier) {
  boite = boite + prix;
}
console.log(boite);
```

```sortie
20
```

Si `reduce` vous paraît obscur, écrivez d'abord la boucle. Ensuite, transformez-la.

### Le plus grand élément

La boîte peut contenir autre chose qu'un total : ici, le plus grand nombre vu jusque-là.

```javascript
const scores = [7, 21, 13];
const meilleur = scores.reduce((boite, s) => (s > boite ? s : boite), 0);
console.log(meilleur);
```

```sortie
21
```

| Tour | boite (avant) | s | s > boite ? | boite (après) |
| --- | --- | --- | --- | --- |
| départ | – | – | – | 0 |
| 1 | 0 | 7 | oui | 7 |
| 2 | 7 | 21 | oui | 21 |
| 3 | 21 | 13 | non | 21 |

### Compter avec un objet

La boîte peut aussi être un objet. Comptons les votes :

```javascript
const votes = ["oui", "non", "oui", "oui"];

const compte = votes.reduce((boite, v) => {
  boite[v] = (boite[v] ?? 0) + 1;
  return boite;
}, {});

console.log(compte);
```

```sortie
{ oui: 3, non: 1 }
```

| Tour | v | boite (après) |
| --- | --- | --- |
| départ | – | `{}` |
| 1 | `"oui"` | `{ oui: 1 }` |
| 2 | `"non"` | `{ oui: 1, non: 1 }` |
| 3 | `"oui"` | `{ oui: 2, non: 1 }` |
| 4 | `"oui"` | `{ oui: 3, non: 1 }` |

Avec des accolades, la flèche doit finir par `return boite;`. Sans lui, la boîte du
tour suivant serait `undefined`.

> **À retenir :** dans `reduce`, la fonction renvoie toujours **la nouvelle boîte**.
> Donnez toujours une valeur de départ.

## Erreurs fréquentes

### Croire que sort ne touche pas l'original

```javascript norun
const arrivees = [42, 7, 19];
const triees = arrivees.sort((a, b) => a - b);
console.log("Premier arrivé :", arrivees[0]);
```

```text
Premier arrivé : 7
```

Pas d'erreur, mais un résultat faux : le premier arrivé était `42`. `sort` a modifié
`arrivees`. Utilisez `arrivees.toSorted((a, b) => a - b)`.

### Appeler groupBy sur le tableau

```javascript norun
const mots = ["un", "deux", "trois"];
console.log(mots.groupBy((m) => m.length));
```

```text
file:///home/alice/js/main.mjs:2
console.log(mots.groupBy((m) => m.length));
                 ^

TypeError: mots.groupBy is not a function
    at file:///home/alice/js/main.mjs:2:18

Node.js v22.22.0
```

`groupBy` n'est pas une méthode des tableaux : c'est une fonction de `Object`. Écrivez
`Object.groupBy(mots, (m) => m.length)`.

### Utiliser with avec un indice qui n'existe pas

```javascript norun
const notes = [12, 15];
const nouvelles = notes.with(5, 20);
```

```text
file:///home/alice/js/main.mjs:2
const nouvelles = notes.with(5, 20);
                        ^

RangeError: Invalid index : 5
    at Array.with (<anonymous>)
    at file:///home/alice/js/main.mjs:2:25

Node.js v22.22.0
```

`with` remplace une case qui **existe déjà**. Ici, il n'y a que les indices 0 et 1.
Pour ajouter à la fin sans modifier l'original, écrivez `[...notes, 20]`.

### Copier avec structuredClone un objet qui contient une fonction

```javascript norun
const bouton = { texte: "OK", clic() {} };
const copie = structuredClone(bouton);
```

```text
node:internal/per_context/domexception:76
    const self = new Error();
                 ^

DOMException [DataCloneError]: clic() {} could not be cloned.
    at new DOMException (node:internal/per_context/domexception:76:18)
    at structuredClone (node:internal/worker/js_transferable:126:26)
    at file:///home/alice/js/main.mjs:2:15

Node.js v22.22.0
```

`structuredClone` copie des **données** (nombres, textes, tableaux, objets, dates), pas
des fonctions. Gardez vos données dans des objets simples, sans méthode.

## Testez-vous

**Question 1 :** que vaut `[[1, 2], [3]].flat().at(-1)` ?

> **Réponse**
>
> ```javascript
> console.log([[1, 2], [3]].flat().at(-1));
> ```
>
> ```sortie
> 3
> ```
>
> `flat()` donne `[1, 2, 3]`, et `at(-1)` prend le dernier élément.

**Question 2 :** vous voulez la liste triée **et** garder la liste d'origine. Quelle
méthode choisir ?

> **Réponse**
>
> `toSorted`, qui renvoie une copie triée. `sort` modifierait la liste d'origine.

**Question 3 :** remplissez le tableau de suivi de
`[2, 3, 4].reduce((boite, n) => boite * n, 1)`.

> **Réponse**
>
> | Tour | boite (avant) | n | boite (après) |
> | --- | --- | --- | --- |
> | départ | – | – | 1 |
> | 1 | 1 | 2 | 2 |
> | 2 | 2 | 3 | 6 |
> | 3 | 6 | 4 | 24 |
>
> ```javascript
> console.log([2, 3, 4].reduce((boite, n) => boite * n, 1));
> ```
>
> ```sortie
> 24
> ```

## En résumé

- `at(-1)` lit le dernier élément ; `findLast` cherche depuis la fin ; `some` et
  `every` répondent « au moins un » et « tous ».
- `flat` déplie les tableaux imbriqués ; `flatMap` transforme puis déplie.
- `Object.groupBy(tableau, f)` range les éléments en piles nommées par `f`.
- `toSorted`, `toReversed` et `with` renvoient une copie : l'original ne change pas.
- `[...a]` copie un niveau ; `structuredClone(a)` copie tout.
- `reduce` fait passer une boîte d'un tour à l'autre : suivez-la dans un tableau.

Suite : [gérer les erreurs](erreurs.md).
