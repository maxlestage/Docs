# Itérateurs et générateurs

Cette page explique ce qui se passe quand vous écrivez `for...of`, comment rendre vos propres objets parcourables, et comment produire des valeurs une à une avec les générateurs `function*`.

> **Objectifs :**
>
> - vous comprendrez le protocole d'itération derrière `for...of` ;
> - vous saurez rendre un objet parcourable avec `Symbol.iterator` ;
> - vous saurez écrire un générateur avec `function*` et `yield` ;
> - vous saurez manipuler une suite infinie sans bloquer le programme ;
> - vous saurez utiliser `...`, `entries()`, `keys()` et `values()`.

## Ce que for...of parcourt

`for...of` parcourt beaucoup de choses : un tableau, une chaîne, un `Map`, un `Set`…
On dit que ces valeurs sont **itérables** : on peut les parcourir élément par élément.

```javascript
for (const lettre of "chat") {
  console.log(lettre);
}

for (const [cle, valeur] of new Map([["a", 1], ["b", 2]])) {
  console.log(cle, valeur);
}
```

```sortie
c
h
a
t
a 1
b 2
```

Mais un objet ordinaire `{ }` n'est **pas** itérable. Pour parcourir ses propriétés, il
faut passer par `Object.entries` (voir [les objets](objets.md)).

## Le protocole d'itération

Comment `for...of` fait-il ? Il demande à la valeur un
[itérateur](../debuter/glossaire.md#iterateur) : un objet qui donne les éléments **un
par un**, à la demande. Pensez à un distributeur de tickets : à chaque pression sur le
bouton, il sort le ticket suivant ; quand le rouleau est vide, il le dit.

Le bouton s'appelle `next()`. Il renvoie un petit objet :

- `value` : l'élément suivant ;
- `done` : `true` quand il n'y a plus rien.

On obtient l'itérateur avec une méthode au nom spécial, `[Symbol.iterator]()`.

```javascript
const fruits = ["pomme", "kiwi"];
const iterateur = fruits[Symbol.iterator]();

console.log(iterateur.next());
console.log(iterateur.next());
console.log(iterateur.next());
```

```sortie
{ value: 'pomme', done: false }
{ value: 'kiwi', done: false }
{ value: undefined, done: true }
```

C'est exactement ce que fait `for...of` pour vous : il appelle `next()` en boucle, et
s'arrête dès que `done` vaut `true`.

> **Note :** `Symbol.iterator` est un **symbole** : un nom de propriété spécial, qui ne
> peut pas entrer en conflit avec vos propres noms. On l'écrit entre crochets.

## Rendre votre objet itérable

Pour que `for...of` accepte votre objet, donnez-lui une méthode `[Symbol.iterator]()`
qui renvoie un itérateur, c'est-à-dire un objet avec une méthode `next()`.

Voici un compte à rebours.

```javascript
class CompteARebours {
  constructor(depart) {
    this.depart = depart;
  }

  [Symbol.iterator]() {
    let actuel = this.depart;
    return {
      next() {
        if (actuel < 0) {
          return { value: undefined, done: true };
        }
        const valeur = actuel;
        actuel -= 1;
        return { value: valeur, done: false };
      },
    };
  }
}

for (const n of new CompteARebours(3)) {
  console.log(n);
}
console.log("Décollage !");
```

```sortie
3
2
1
0
Décollage !
```

Chaque appel à `[Symbol.iterator]()` crée une nouvelle variable `actuel` (une
[closure](closures.md)) : on peut parcourir le même compte à rebours plusieurs fois.

Cela marche, mais c'est long. Les générateurs font la même chose en beaucoup plus court.

## Les générateurs : function* et yield

Un **générateur** est une fonction spéciale, écrite `function*` (avec une étoile). Au
lieu de tout calculer d'un coup, elle produit ses valeurs une par une avec `yield`
(« céder », « fournir »).

À chaque `yield`, la fonction se met **en pause** et donne une valeur. Au `next()`
suivant, elle reprend exactement là où elle s'était arrêtée. C'est comme un livre avec un
marque-page.

```javascript
function* troisCouleurs() {
  console.log("(début)");
  yield "rouge";
  console.log("(reprise)");
  yield "vert";
  yield "bleu";
}

const gen = troisCouleurs();
console.log(gen.next().value);
console.log(gen.next().value);

for (const couleur of troisCouleurs()) {
  console.log(couleur);
}
```

```sortie
(début)
rouge
(reprise)
vert
(début)
rouge
(reprise)
vert
bleu
```

Appeler `troisCouleurs()` n'exécute rien : cela renvoie un **objet générateur**, qui est
un itérateur. Le code ne s'exécute qu'à la demande, à chaque `next()`.

### Réécrire le compte à rebours

Avec un générateur, le compte à rebours tient en quelques lignes. On écrit
`*[Symbol.iterator]()` : une méthode génératrice.

```javascript
class CompteARebours {
  constructor(depart) {
    this.depart = depart;
  }

  *[Symbol.iterator]() {
    for (let n = this.depart; n >= 0; n--) {
      yield n;
    }
  }
}

console.log([...new CompteARebours(5)]);
```

```sortie
[ 5, 4, 3, 2, 1, 0 ]
```

Plus de `next()`, de `value` ni de `done` à écrire : le générateur s'en charge.

### Un générateur avec des paramètres

Un générateur reçoit des paramètres comme une fonction ordinaire.

```javascript
function* intervalle(debut, fin, pas = 1) {
  for (let n = debut; n < fin; n += pas) {
    yield n;
  }
}

for (const n of intervalle(0, 10, 3)) {
  console.log(n);
}
```

```sortie
0
3
6
9
```

## Un générateur infini

Puisqu'un générateur ne calcule que ce qu'on lui demande, il peut être **infini** : la
boucle `while (true)` ne bloque pas le programme, car la fonction se met en pause à chaque
`yield`.

Pour n'en prendre qu'une partie, écrivons un petit outil `prendre(n, iterable)` : c'est
lui-même un générateur, qui s'arrête après `n` éléments.

```javascript
function* entiers() {
  let n = 1;
  while (true) {
    yield n;
    n += 1;
  }
}

function* prendre(n, iterable) {
  if (n <= 0) return;
  let compte = 0;
  for (const element of iterable) {
    yield element;
    compte += 1;
    if (compte === n) return;
  }
}

function* carres(iterable) {
  for (const x of iterable) {
    yield x * x;
  }
}

console.log([...prendre(5, entiers())]);
console.log([...prendre(4, carres(entiers()))]);
```

```sortie
[ 1, 2, 3, 4, 5 ]
[ 1, 4, 9, 16 ]
```

`return` dans un générateur le termine : `done` passe à `true`. Les générateurs
s'emboîtent comme des tuyaux : `entiers` → `carres` → `prendre`. Chaque valeur traverse
les tuyaux une à une, et seules quatre valeurs sont jamais calculées.

### La suite de Fibonacci

Chaque nombre est la somme des deux précédents : 0, 1, 1, 2, 3, 5, 8…

```javascript
function* fibonacci() {
  let [a, b] = [0, 1];
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

const premiers = [];
for (const n of fibonacci()) {
  if (n > 50) break;
  premiers.push(n);
}
console.log(premiers.join(", "));
```

```sortie
0, 1, 1, 2, 3, 5, 8, 13, 21, 34
```

`break` arrête la boucle `for...of` ; le générateur est alors abandonné proprement.

## Déplier un itérable avec ...

L'opérateur `...` (**spread**, « étaler ») déplie n'importe quel itérable : tableau,
chaîne, `Set`, `Map`, générateur… On s'en sert pour fabriquer un tableau ou pour passer
des arguments.

```javascript
const lettres = [..."salut"];
console.log(lettres);

const sansDoublons = [...new Set([3, 1, 3, 2, 1])];
console.log(sansDoublons);

function* notes() {
  yield 12;
  yield 17;
  yield 9;
}
console.log(Math.max(...notes()));

const [premier, ...reste] = notes();
console.log(premier, reste);
```

```sortie
[ 's', 'a', 'l', 'u', 't' ]
[ 3, 1, 2 ]
17
12 [ 17, 9 ]
```

`Array.from(iterable)` fait la même chose que `[...iterable]`.

## entries(), keys() et values()

Les tableaux, les `Map` et les `Set` ont trois méthodes qui renvoient des itérateurs :

| Méthode | Donne |
|---------|-------|
| `keys()` | les clés (les indices pour un tableau) |
| `values()` | les valeurs |
| `entries()` | des paires `[clé, valeur]` |

```javascript
const jours = ["lun", "mar", "mer"];

for (const [indice, jour] of jours.entries()) {
  console.log(`${indice + 1}. ${jour}`);
}

const stock = new Map([
  ["pommes", 4],
  ["poires", 0],
]);
console.log([...stock.keys()]);
console.log([...stock.values()]);
```

```sortie
1. lun
2. mar
3. mer
[ 'pommes', 'poires' ]
[ 4, 0 ]
```

Pour un objet ordinaire, utilisez les fonctions `Object.keys`, `Object.values` et
`Object.entries` : elles renvoient directement des tableaux.

```javascript
const scores = { alice: 12, bob: 8 };
for (const [nom, score] of Object.entries(scores)) {
  console.log(`${nom} : ${score}`);
}
```

```sortie
alice : 12
bob : 8
```

## Erreurs fréquentes

### Parcourir un objet ordinaire avec for...of

```javascript norun
const personne = { nom: "Ada", age: 36 };
for (const x of personne) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:2
for (const x of personne) console.log(x);
                ^

TypeError: personne is not iterable
    at file:///home/vous/projet/main.mjs:2:17
```

« `personne` n'est pas itérable » : un objet `{ }` n'a pas de `Symbol.iterator`. Écrivez
`for (const [cle, valeur] of Object.entries(personne))`.

### Oublier d'appeler le générateur

```javascript norun
function* f() { yield 1; }
const it = f;
for (const x of it) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:3
for (const x of it) console.log(x);
                ^

TypeError: it is not iterable
    at file:///home/vous/projet/main.mjs:3:17
```

`f` est la fonction ; c'est `f()`, l'appel, qui renvoie l'objet générateur. Ajoutez les
parenthèses : `const it = f();`.

### Déplier un générateur infini

```javascript norun
function* nombres() { let n = 1; while (true) yield n++; }
console.log([...nombres()].length);
```

```text
<--- JS stacktrace --->
FATAL ERROR: invalid array length Allocation failed - JavaScript heap out of memory
```

Après de longues secondes, Node abandonne : la mémoire est pleine. `...` veut tout
déplier, et la suite ne finit jamais. Limitez-la d'abord : `[...prendre(10, nombres())]`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
function* deux() {
  yield "a";
  yield "b";
}
const g = deux();
console.log([...g]);
console.log([...g]);
```

> **Réponse**
>
> `[ 'a', 'b' ]` puis `[]`. Un objet générateur ne se parcourt qu'**une fois** : il est
> épuisé après le premier `...`. Pour recommencer, rappelez `deux()`.

**Question 2 :** que renvoie `next()` quand un itérateur n'a plus d'éléments ?

> **Réponse**
>
> Un objet dont `done` vaut `true` (et `value` vaut en général `undefined`).

**Question 3 :** écrivez un générateur `pairs()` qui produit 0, 2, 4, 6… sans fin, et
affichez ses 4 premières valeurs.

> **Réponse**
>
> ```javascript
> function* pairs() {
>   for (let n = 0; ; n += 2) {
>     yield n;
>   }
> }
>
> const resultat = [];
> for (const n of pairs()) {
>   if (resultat.length === 4) break;
>   resultat.push(n);
> }
> console.log(resultat);
> ```
>
> ```sortie
> [ 0, 2, 4, 6 ]
> ```

## En résumé

- Un itérable a une méthode `[Symbol.iterator]()` qui renvoie un itérateur.
- Un itérateur a une méthode `next()` qui renvoie `{ value, done }` ; `for...of`
  l'appelle pour vous.
- `function*` et `yield` écrivent un itérateur en quelques lignes : la fonction se met en
  pause à chaque `yield`.
- Un générateur peut être infini : prenez-en une partie avec `break` ou un outil comme
  `prendre`.
- `...`, `entries()`, `keys()` et `values()` fonctionnent avec tous les itérables.

Suite : [gérer les erreurs](erreurs.md). Comparez avec
[les énumérateurs en Ruby](../ruby/enumerateurs.md).
