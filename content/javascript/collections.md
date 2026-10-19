# Les tableaux

Cette page explique comment ranger plusieurs valeurs dans un tableau, les ajouter, les retirer, les chercher, les transformer avec `map`, `filter` et `reduce`, et les trier sans tomber dans les pièges.

> **Objectifs :**
>
> - vous saurez créer un tableau, lire un élément et connaître sa longueur ;
> - vous saurez ajouter et retirer des éléments au début ou à la fin ;
> - vous saurez chercher dans un tableau avec `includes`, `indexOf` et `find` ;
> - vous saurez transformer un tableau avec `map`, `filter` et `reduce` ;
> - vous saurez trier des nombres correctement et copier un tableau.

## Créer un tableau

Imaginez une liste de courses de vingt articles. Créer vingt variables serait long et
pénible. Un tableau range toutes ces valeurs sous un seul nom.

Un tableau (*array*) est une [liste](../debuter/glossaire.md#liste) de valeurs rangées
dans l'ordre, comme les wagons d'un train. On l'écrit entre crochets `[ ]`, avec des
virgules entre les éléments.

```javascript
const fruits = ["pomme", "poire", "kiwi"];
const notes = [12, 15, 9];
const vide = [];
const melange = ["Alice", 30, true];

console.log(fruits);
console.log(notes.length);
console.log(vide.length);
console.log(melange);
```

```sortie
[ 'pomme', 'poire', 'kiwi' ]
3
0
[ 'Alice', 30, true ]
```

Ligne par ligne :

- `fruits` contient trois textes, `notes` trois nombres, et `vide` aucun élément ;
- `console.log(fruits)` affiche tout le tableau, entre crochets ;
- `notes.length` vaut 3 : il y a trois notes. `vide.length` vaut 0 ;
- `melange` montre qu'un même tableau peut contenir un texte, un nombre et un booléen.

`length` donne le nombre d'éléments. Un tableau peut mélanger les types, mais en
pratique on y range des valeurs de même nature.

## Lire et modifier un élément

Une fois la liste rangée, il faut pouvoir en sortir un élément précis, comme lire la
troisième ligne de votre liste de courses, ou en corriger un.

Chaque élément a un [indice](../debuter/glossaire.md#indice) : sa position, **à partir
de 0**. Le premier wagon est le wagon 0.

```javascript
const fruits = ["pomme", "poire", "kiwi"];

console.log(fruits[0]);
console.log(fruits[2]);
console.log(fruits.at(-1));
console.log(fruits[5]);

fruits[1] = "mangue";
console.log(fruits);
```

```sortie
pomme
kiwi
kiwi
undefined
[ 'pomme', 'mangue', 'kiwi' ]
```

Ligne par ligne :

- `fruits[0]` lit le premier élément, `"pomme"` ; `fruits[2]` lit le troisième,
  `"kiwi"` ;
- `fruits.at(-1)` compte depuis la fin : il donne le dernier, encore `"kiwi"` ;
- `fruits[5]` n'existe pas : JavaScript répond `undefined` ;
- `fruits[1] = "mangue"` remplace `"poire"`, qui était à l'indice 1.

- `at(-1)` donne le dernier élément, comme pour [le texte](texte.md).
- Un indice qui n'existe pas donne `undefined`, sans erreur.
- `fruits` est une `const`, et pourtant on peut changer son contenu. `const` interdit de
  remplacer **le tableau entier** (`fruits = [...]`), pas de modifier ce qu'il contient.

## Ajouter et retirer

Une liste change sans cesse. Pensez à une file d'attente : les gens arrivent au bout
et partent par le début. Ces méthodes font exactement cela.

Quatre méthodes ajoutent ou retirent un élément à un bout du tableau. Elles **modifient**
le tableau.

```javascript
const file = ["Alice", "Bob"];

file.push("Chloé");
console.log(file);

const dernier = file.pop();
console.log(dernier, file);

file.unshift("Zoé");
console.log(file);

const premier = file.shift();
console.log(premier, file);
```

```sortie
[ 'Alice', 'Bob', 'Chloé' ]
Chloé [ 'Alice', 'Bob' ]
[ 'Zoé', 'Alice', 'Bob' ]
Zoé [ 'Alice', 'Bob' ]
```

Ligne par ligne :

- `push("Chloé")` ajoute Chloé à la fin : la file contient trois prénoms ;
- `pop()` retire Chloé, la dernière, et la range dans `dernier` ;
- `unshift("Zoé")` place Zoé au début, devant Alice ;
- `shift()` retire Zoé du début et la range dans `premier`.

| Méthode | Bout | Action |
| --- | --- | --- |
| `push(x)` | fin | ajoute `x` |
| `pop()` | fin | retire et renvoie le dernier |
| `unshift(x)` | début | ajoute `x` |
| `shift()` | début | retire et renvoie le premier |

**À vous :** écrivez un programme qui crée un tableau `courses` contenant `"pain"` et
`"lait"`, ajoute `"œufs"` à la fin, puis affiche le tableau et sa longueur.

> **Solution**
>
> ```javascript
> const courses = ["pain", "lait"];
> courses.push("œufs");
> console.log(courses);
> console.log(courses.length);
> ```
>
> ```sortie
> [ 'pain', 'lait', 'œufs' ]
> 3
> ```

## slice et splice

Parfois, vous voulez seulement une partie de la liste, par exemple les trois premiers
d'un classement. Ou bien vous voulez retirer un élément au milieu, pas à un bout.

Ces deux noms se ressemblent, mais font des choses très différentes.

- `slice(début, fin)` **copie** un morceau, sans toucher au tableau (comme pour le texte).
- `splice(début, combien, ...ajouts)` **découpe dans** le tableau : il retire des
  éléments, peut en insérer, et modifie le tableau.

```javascript
const a = [1, 2, 3, 4, 5];
console.log(a.slice(1, 3));
console.log(a);

const b = [1, 2, 3, 4, 5];
const retires = b.splice(1, 2);
console.log(retires);
console.log(b);

b.splice(1, 0, "x", "y");
console.log(b);
```

```sortie
[ 2, 3 ]
[ 1, 2, 3, 4, 5 ]
[ 2, 3 ]
[ 1, 4, 5 ]
[ 1, 'x', 'y', 4, 5 ]
```

Ligne par ligne :

- `a.slice(1, 3)` copie les éléments d'indice 1 et 2 (la fin, 3, est exclue) ;
- `console.log(a)` le prouve : `a` n'a pas changé ;
- `b.splice(1, 2)` retire 2 et 3 de `b`, et les renvoie dans `retires` ;
- le dernier `splice` insère deux éléments sans rien retirer.

`b.splice(1, 2)` part de l'indice 1 et retire 2 éléments. `b.splice(1, 0, "x", "y")`
n'en retire aucun et insère `"x"` et `"y"` à l'indice 1.

> **Astuce :** pour vous souvenir, `splice` a un **p** comme « percer » : il modifie le
> tableau. `slice` est une simple tranche, copiée.

## Chercher dans un tableau

Avant d'ajouter un article à votre liste de courses, vous regardez s'il y est déjà.
Ces méthodes posent cette question à votre place, même dans une très longue liste.

```javascript
const fruits = ["pomme", "poire", "kiwi"];

console.log(fruits.includes("kiwi"));
console.log(fruits.indexOf("poire"));
console.log(fruits.indexOf("banane"));
```

```sortie
true
1
-1
```

`includes` répond oui ou non ; `indexOf` donne la position, ou `-1` si l'élément est
absent.

### find et findIndex

Pour chercher selon une **règle**, `find` reçoit une fonction qui répond vrai ou faux.
Il renvoie le premier élément qui convient, ou `undefined`.

```javascript
const notes = [8, 14, 11, 17];

console.log(notes.find((n) => n > 10));
console.log(notes.findIndex((n) => n > 10));
console.log(notes.find((n) => n > 20));
```

```sortie
14
1
undefined
```

La fonction donnée à `find` est un *callback* (voir [les fonctions](fonctions.md#les-callbacks)).
`find` l'appelle sur chaque élément, jusqu'à ce qu'elle réponde `true`.

**À vous :** à partir de `const ages = [12, 17, 21, 15]`, affichez si 21 est dans le
tableau, puis le premier âge supérieur ou égal à 18.

> **Solution**
>
> ```javascript
> const ages = [12, 17, 21, 15];
> console.log(ages.includes(21));
> console.log(ages.find((age) => age >= 18));
> ```
>
> ```sortie
> true
> 21
> ```

## Parcourir et transformer

Très souvent, vous voulez faire la même chose avec chaque élément : afficher chaque
article, ajouter la TVA à chaque prix, garder seulement les bonnes notes. Ces méthodes
le font pour vous, sans écrire le même code pour chaque élément.

### forEach : faire quelque chose pour chaque élément

```javascript
const fruits = ["pomme", "poire"];

fruits.forEach((fruit, i) => {
  console.log(`${i + 1}. ${fruit}`);
});
```

```sortie
1. pomme
2. poire
```

Ligne par ligne :

- `forEach` appelle la fonction une fois par élément, donc deux fois ici ;
- au premier tour, `fruit` vaut `"pomme"` et `i` vaut 0 ; au second, `"poire"` et 1 ;
- `${i + 1}` ajoute 1 à l'indice, pour numéroter à partir de 1.

Le callback reçoit l'élément, puis son indice. Une boucle `for...of` fait le même
travail ; choisissez celle que vous trouvez la plus lisible.

### map : transformer chaque élément

`map` crée un **nouveau tableau**, de même longueur, où chaque élément est transformé.

```javascript
const prix = [10, 20, 30];
const prixTTC = prix.map((p) => p * 1.2);
const noms = ["alice", "bob"].map((nom) => nom.toUpperCase());

console.log(prixTTC);
console.log(noms);
console.log(prix);
```

```sortie
[ 12, 24, 36 ]
[ 'ALICE', 'BOB' ]
[ 10, 20, 30 ]
```

Le tableau d'origine n'a pas changé.

### filter : garder certains éléments

`filter` crée un nouveau tableau avec seulement les éléments pour lesquels le callback
répond `true`.

```javascript
const notes = [8, 14, 11, 17, 6];
const reussies = notes.filter((n) => n >= 10);

console.log(reussies);
console.log(notes.filter((n) => n > 20));
```

```sortie
[ 14, 11, 17 ]
[]
```

### reduce : tout résumer en une valeur

`reduce` parcourt le tableau en gardant un **accumulateur** : une valeur qui grossit à
chaque tour. C'est comme une tirelire : à chaque élément, on y ajoute quelque chose.

```javascript
const notes = [12, 15, 9];

const total = notes.reduce((somme, n) => somme + n, 0);
console.log(total);
console.log(total / notes.length);
```

```sortie
36
12
```

- `0` est la valeur de départ de l'accumulateur `somme`.
- Tour 1 : `0 + 12 = 12` ; tour 2 : `12 + 15 = 27` ; tour 3 : `27 + 9 = 36`.

### Enchaîner les méthodes

`map` et `filter` renvoient des tableaux : on peut les enchaîner.

```javascript
const panier = [4.5, 12, 2, 30];

const total = panier
  .filter((prix) => prix > 3)
  .map((prix) => prix * 0.9)
  .reduce((somme, prix) => somme + prix, 0);

console.log(total.toFixed(2));
```

```sortie
41.85
```

Lisez de haut en bas : on garde les prix au-dessus de 3 €, on applique 10 % de remise,
puis on additionne.

**À vous :** à partir de `const prix = [5, 12, 8]`, créez avec `map` un tableau des
prix doublés et affichez-le. Puis affichez sa somme, calculée avec `reduce`.

> **Solution**
>
> ```javascript
> const prix = [5, 12, 8];
> const doubles = prix.map((p) => p * 2);
> console.log(doubles);
> console.log(doubles.reduce((somme, p) => somme + p, 0));
> ```
>
> ```sortie
> [ 10, 24, 16 ]
> 50
> ```

## Trier

Ranger dans l'ordre sert partout : des notes de la plus basse à la plus haute, un
classement, une liste de noms par ordre alphabétique.

`sort` trie le tableau **sur place** (il le modifie). Avec des textes simples, il marche
comme attendu. Avec des nombres, il y a un piège.

```javascript
const nombres = [10, 1, 5, 100, 25];
nombres.sort();
console.log(nombres);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

Sans indication, `sort` transforme chaque élément en **texte** et les range dans l'ordre
du dictionnaire : `"10"` vient avant `"5"`, comme « dix » avant « cinq ».

Donnez-lui une fonction de comparaison `(a, b) => ...`, qui renvoie un nombre :
négatif si `a` doit venir avant `b`, positif si après.

```javascript
const nombres = [10, 1, 5, 100, 25];

nombres.sort((a, b) => a - b);
console.log(nombres);

nombres.sort((a, b) => b - a);
console.log(nombres);
```

```sortie
[ 1, 5, 10, 25, 100 ]
[ 100, 25, 10, 5, 1 ]
```

`a - b` trie du plus petit au plus grand, `b - a` du plus grand au plus petit. Pour des
mots français, utilisez `(a, b) => a.localeCompare(b, "fr")` (voir
[le texte](texte.md#trier-des-mots-francais)).

> **À retenir :** pour trier des nombres, écrivez **toujours** `sort((a, b) => a - b)`.

**À vous :** triez le tableau `[42, 7, 19, 3]` du plus grand au plus petit, puis
affichez-le.

> **Solution**
>
> ```javascript
> const scores = [42, 7, 19, 3];
> scores.sort((a, b) => b - a);
> console.log(scores);
> ```
>
> ```sortie
> [ 42, 19, 7, 3 ]
> ```

## Le spread et la décomposition

Ces deux écritures courtes servent à assembler plusieurs tableaux en un seul, ou à
sortir des éléments d'un tableau pour les ranger dans des variables bien nommées.

### Le spread : étaler un tableau

Trois points `...` devant un tableau l'**étalent** : ses éléments sont posés un par un.

```javascript
const fruits = ["pomme", "poire"];
const legumes = ["carotte"];

const courses = [...fruits, ...legumes, "pain"];
console.log(courses);
console.log(Math.max(...[3, 8, 1]));
```

```sortie
[ 'pomme', 'poire', 'carotte', 'pain' ]
8
```

Ligne par ligne :

- `...fruits` pose `"pomme"` et `"poire"`, puis `...legumes` pose `"carotte"` ;
- `"pain"` vient ensuite : `courses` est un nouveau tableau de quatre éléments ;
- `Math.max(...[3, 8, 1])` revient à écrire `Math.max(3, 8, 1)` : le plus grand est 8.

### La décomposition

La **décomposition** (*destructuring*) range les éléments d'un tableau dans plusieurs
variables d'un coup.

```javascript
const podium = ["Alice", "Bob", "Chloé", "David"];

const [premier, deuxieme, ...autres] = podium;
console.log(premier);
console.log(deuxieme);
console.log(autres);

let a = 1;
let b = 2;
[a, b] = [b, a];
console.log(a, b);
```

```sortie
Alice
Bob
[ 'Chloé', 'David' ]
2 1
```

La dernière ligne échange deux variables sans variable intermédiaire.

## Fabriquer des tableaux

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Parfois, vous ne voulez pas écrire chaque élément à la main, par exemple les carrés des
nombres de 0 à 4. JavaScript peut fabriquer le tableau pour vous.

`Array.from` fabrique un tableau à partir d'autre chose : un texte, ou une longueur et
une fonction. `flat` aplatit un tableau de tableaux.

```javascript
console.log(Array.from("abc"));
console.log(Array.from({ length: 5 }, (_, i) => i * i));

const grille = [[1, 2], [3, 4], [5]];
console.log(grille.flat());
console.log(Array.isArray(grille));
```

```sortie
[ 'a', 'b', 'c' ]
[ 0, 1, 4, 9, 16 ]
[ 1, 2, 3, 4, 5 ]
true
```

Dans `(_, i) => i * i`, le `_` est un paramètre qu'on n'utilise pas : seul l'indice `i`
nous intéresse. `Array.isArray` vérifie qu'une valeur est un tableau (`typeof` répond
seulement `"object"`).

## Copier ou partager ?

Vous voudrez souvent garder l'original intact et travailler sur une copie, comme on
photocopie un formulaire avant de le remplir. Encore faut-il avoir une vraie copie.

Une variable ne contient pas le tableau lui-même, mais une **référence** vers lui :
une adresse, comme une étiquette attachée à une boîte. Écrire `b = a` colle une
deuxième étiquette sur **la même boîte**.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);

console.log(a);
console.log(a === b);
```

```sortie
[ 1, 2, 3, 4 ]
true
```

Ligne par ligne :

- `const b = a` ne copie rien : `b` désigne le même tableau que `a` ;
- `b.push(4)` ajoute 4 dans ce tableau unique ;
- `a` affiche donc aussi le 4, et `a === b` répond `true` : c'est la même boîte.

Modifier `b` a modifié `a` : c'est le même tableau. Pour une vraie copie, utilisez le
spread (ou `slice()`).

```javascript
const a = [1, 2, 3];
const copie = [...a];
copie.push(4);

console.log(a);
console.log(copie);
console.log([1, 2] === [1, 2]);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
false
```

`===` compare les références, pas le contenu : deux tableaux distincts ne sont jamais
égaux, même avec les mêmes éléments.

## Erreurs fréquentes

### Trier des nombres sans comparateur

```javascript norun
const ages = [9, 25, 100, 3];
ages.sort();
console.log(ages);
```

```text
[ 100, 25, 3, 9 ]
```

Pas d'erreur, mais un ordre faux : `sort` compare des textes. Écrivez
`ages.sort((a, b) => a - b)`.

### Lire après la fin du tableau

```javascript norun
const fruits = ["pomme", "poire"];
console.log(fruits[2].toUpperCase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(fruits[2].toUpperCase());
                      ^

TypeError: Cannot read properties of undefined (reading 'toUpperCase')
    at file:///home/alice/js/main.mjs:2:23

Node.js v22.22.0
```

Le tableau a 2 éléments, d'indices 0 et 1. `fruits[2]` vaut `undefined`, qui n'a pas de
méthode `toUpperCase`. Le dernier élément est `fruits[fruits.length - 1]` ou
`fruits.at(-1)`.

### Oublier la valeur de départ de reduce

```javascript norun
const notes = [];
const total = notes.reduce((somme, n) => somme + n);
```

```text
file:///home/alice/js/main.mjs:2
const total = notes.reduce((somme, n) => somme + n);
                    ^

TypeError: Reduce of empty array with no initial value
    at Array.reduce (<anonymous>)
    at file:///home/alice/js/main.mjs:2:21

Node.js v22.22.0
```

Sans valeur de départ, `reduce` prend le premier élément… qui n'existe pas dans un
tableau vide. Donnez toujours la valeur de départ : `reduce((somme, n) => somme + n, 0)`.

### Croire qu'on a copié un tableau

```javascript norun
const original = ["a", "b"];
const copie = original;
copie.pop();
console.log(original);
```

```text
[ 'a' ]
```

`copie` et `original` désignent le même tableau. Copiez avec `[...original]`.

## Testez-vous

**Question 1 :** que contient `t` après ce code ?

```javascript norun
const t = [1, 2, 3];
t.push(4);
t.shift();
```

> **Réponse**
>
> `[2, 3, 4]`. `push` ajoute 4 à la fin, `shift` retire le 1 du début.

**Question 2 :** à partir de `[3, 8, 12, 5]`, comment obtenir les doubles des nombres
plus grands que 4 ?

> **Réponse**
>
> ```javascript
> const resultat = [3, 8, 12, 5].filter((n) => n > 4).map((n) => n * 2);
> console.log(resultat);
> ```
>
> ```sortie
> [ 16, 24, 10 ]
> ```

**Question 3 :** quelle est la différence entre `slice` et `splice` ?

> **Réponse**
>
> `slice` copie un morceau sans modifier le tableau. `splice` retire (et peut insérer)
> des éléments **dans** le tableau, qu'il modifie.

## En résumé

- Un tableau s'écrit `[a, b, c]` ; ses indices commencent à 0 ; `length` donne sa taille.
- `push`/`pop` agissent à la fin, `unshift`/`shift` au début ; `splice` modifie,
  `slice` copie.
- `includes`, `indexOf`, `find` et `findIndex` cherchent ; `map`, `filter` et `reduce`
  transforment sans modifier l'original.
- Triez les nombres avec `sort((a, b) => a - b)`, jamais avec `sort()` seul.
- `...` étale un tableau ; `const [a, b] = tableau` le décompose.
- `b = a` partage le même tableau ; `[...a]` en fait une copie.

Suite : [les objets](objets.md).
