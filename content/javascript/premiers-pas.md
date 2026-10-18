# Premiers pas

Cette page montre comment ranger des valeurs dans des variables avec `let` et `const`, quels sont les types de base de JavaScript et comment les comparer.

> **Objectifs :**
>
> - vous saurez créer une variable avec `let` ou `const`, et saurez lequel choisir ;
> - vous connaîtrez les types de base : nombre, texte, booléen, `null`, `undefined` ;
> - vous saurez afficher plusieurs valeurs et construire un texte avec `${...}` ;
> - vous saurez commenter votre code et bien nommer vos variables ;
> - vous saurez pourquoi il faut comparer avec `===` plutôt qu'avec `==`.

## Les variables avec let et const

Une [variable](../debuter/glossaire.md#variable) est une boîte avec une étiquette : vous
y rangez une valeur, puis vous la retrouvez par son nom. En JavaScript, on crée une
variable avec `const` ou `let`.

```javascript
const prenom = "Alice";
let age = 30;

console.log(prenom);
console.log(age);
```

```sortie
Alice
30
```

Le signe `=` ne veut pas dire « égal » : il veut dire « range cette valeur dans cette
boîte ». On dit qu'on **affecte** une valeur.

### const : une boîte scellée

`const` (de *constant*) crée une variable qu'on ne peut plus réaffecter. `let` crée une
variable qu'on peut changer plus tard.

```javascript
let score = 0;
score = score + 10;
score += 5;
console.log(score);

const pi = 3.14159;
console.log(pi);
```

```sortie
15
3.14159
```

`score += 5` est un raccourci pour `score = score + 5`.

> **À retenir :** utilisez `const` par défaut. Passez à `let` seulement si la valeur
> doit changer. En lisant `const`, le lecteur sait tout de suite que la valeur ne bougera
> plus.

### Et var ?

Vous verrez souvent `var` dans du vieux code ou sur Internet. C'est l'ancienne façon de
créer une variable, d'avant 2015. Elle a des comportements surprenants : on peut
déclarer deux fois la même variable sans erreur, et elle « fuit » hors des blocs `{ }`.

```javascript
var ville = "Lyon";
var ville = "Lille";
console.log(ville);
```

```sortie
Lille
```

Avec `let`, JavaScript vous aurait prévenu de l'erreur. **N'utilisez pas `var`** :
`let` et `const` font la même chose, en plus sûr.

## Les types de base

Chaque valeur a un [type](../debuter/glossaire.md#type) : c'est sa nature (un nombre, un
texte…). Le type dit ce qu'on peut faire avec la valeur. L'opérateur `typeof` donne le
type d'une valeur, sous forme de texte.

```javascript
console.log(typeof 42);
console.log(typeof 3.5);
console.log(typeof "bonjour");
console.log(typeof true);
console.log(typeof undefined);
console.log(typeof 10n);
```

```sortie
number
number
string
boolean
undefined
bigint
```

| Type | Exemples | Rôle |
| --- | --- | --- |
| `number` | `42`, `3.5`, `-7` | tous les nombres, entiers ou à virgule |
| `string` | `"bonjour"`, `'salut'` | du texte |
| `boolean` | `true`, `false` | vrai ou faux |
| `undefined` | `undefined` | « pas encore de valeur » |
| `null` | `null` | « vide, exprès » |
| `bigint` | `10n` | des entiers géants (voir [les nombres](nombres.md)) |
| `symbol` | `Symbol("id")` | un identifiant unique (rare, pour plus tard) |

- Un **nombre** s'écrit avec un point, pas une virgule : `3.5`.
- Une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres) (un texte)
  s'écrit entre guillemets doubles `"..."` ou simples `'...'`.
- Un [booléen](../debuter/glossaire.md#booleen) vaut `true` (vrai) ou `false` (faux).

### undefined et null

Deux valeurs veulent dire « rien », avec une nuance.

- `undefined` : la boîte existe, mais **personne n'y a rien mis**. C'est la valeur
  d'une variable déclarée sans valeur.
- `null` : la boîte est **vide exprès**. C'est vous qui l'écrivez, pour dire « aucune
  valeur » (voir [null](../debuter/glossaire.md#null)).

```javascript
let reponse;
console.log(reponse);

let gagnant = null;
console.log(gagnant);
console.log(typeof null);
```

```sortie
undefined
null
object
```

> **Attention :** `typeof null` répond `"object"`. C'est une erreur historique de
> JavaScript, gardée pour ne pas casser les vieux sites. Retenez-la, simplement.

## Les gabarits de texte

Pour mettre une valeur dans un texte, écrivez le texte entre **accents graves**
`` `...` `` (touche **AltGr + 7** sur un clavier français), et la valeur dans `${...}`.
C'est un **gabarit** (*template literal*).

```javascript
const prenom = "Alice";
const age = 30;

console.log(`Bonjour, ${prenom} !`);
console.log(`Dans 5 ans, vous aurez ${age + 5} ans.`);
```

```sortie
Bonjour, Alice !
Dans 5 ans, vous aurez 35 ans.
```

Dans `${...}`, vous pouvez mettre n'importe quel calcul. L'ancienne façon utilise `+`
pour coller les morceaux : `"Bonjour, " + prenom + " !"`. Le gabarit est plus lisible.

> **Attention :** le `${...}` ne marche qu'entre accents graves. Entre guillemets
> `"..."`, il est affiché tel quel.

## Afficher plusieurs valeurs

`console.log` accepte plusieurs valeurs, séparées par des virgules. Il les affiche sur
une ligne, séparées par une espace.

```javascript
const article = "café";
const prix = 2.5;

console.log("Article :", article, "- prix :", prix);
console.log(article, prix, true, null);
```

```sortie
Article : café - prix : 2.5
café 2.5 true null
```

> **Astuce :** pour déboguer, écrivez `console.log("prix", prix)` : le nom devant la
> valeur vous évite de confondre les lignes.

## Les commentaires

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains.
JavaScript l'ignore.

```javascript
// Un commentaire sur une ligne
const tva = 0.2; // aussi en fin de ligne

/*
  Un commentaire
  sur plusieurs lignes
*/
console.log(100 * (1 + tva));
```

```sortie
120
```

Un bon commentaire explique **pourquoi**, pas **quoi** : le code dit déjà ce qu'il fait.

## Bien nommer ses variables

En JavaScript, on écrit les noms en **camelCase** (« en bosses de chameau ») : le
premier mot en minuscules, puis une majuscule au début de chaque mot suivant.

```javascript
const prixUnitaire = 4;
const nombreArticles = 3;
const prixTotal = prixUnitaire * nombreArticles;

console.log(prixTotal);
```

```sortie
12
```

Quelques règles :

- un nom contient des lettres, des chiffres, `_` ou `$`, mais **ne commence pas par un
  chiffre** ;
- les majuscules comptent : `age` et `Age` sont deux variables différentes ;
- certains mots sont réservés (`let`, `const`, `if`, `function`…) ;
- choisissez des noms qui disent ce que contient la boîte : `prixTotal` plutôt que `x`.

> **Note :** les accents sont permis (`const âge = 30`), mais l'usage est de les éviter
> dans les noms de variables. Gardez-les pour le texte entre guillemets.

## Comparer : == ou === ?

Comparer deux valeurs donne un booléen. JavaScript a deux opérateurs d'égalité :

- `===` (égalité **stricte**) : vrai si les deux valeurs ont le même type **et** la
  même valeur ;
- `==` (égalité **large**) : convertit d'abord les valeurs pour qu'elles aient le même
  type, puis compare.

```javascript
console.log(5 === 5);
console.log(5 === "5");
console.log(5 == "5");
console.log(0 == "");
console.log(null == undefined);
console.log(null === undefined);
```

```sortie
true
false
true
true
true
false
```

Avec `==`, le texte `"5"` est converti en nombre, et `0 == ""` répond `true` : c'est
rarement ce que vous voulez. Les règles de conversion sont longues et pleines de
pièges.

> **À retenir :** utilisez toujours `===` et son contraire `!==` (« différent de »).
> Oubliez `==` et `!=`.

## Erreurs fréquentes

### Changer une constante

```javascript norun
const age = 20;
age = 21;
```

```text
file:///home/alice/js/main.mjs:2
age = 21;
    ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:5

Node.js v22.22.0
```

On ne peut pas réaffecter une variable `const` (*assignment to constant variable*,
« affectation à une variable constante »). Si la valeur doit changer, déclarez-la avec
`let age = 20;`.

### Déclarer deux fois la même variable

```javascript norun
let score = 0;
let score = 10;
```

```text
file:///home/alice/js/main.mjs:2
let score = 10;
    ^

SyntaxError: Identifier 'score' has already been declared

Node.js v22.22.0
```

`let` sert à **créer** la variable, une seule fois. Pour la modifier ensuite, écrivez
seulement `score = 10;`, sans `let`.

### Utiliser une variable avant de la créer

```javascript norun
console.log(total);
let total = 5;
```

```text
file:///home/alice/js/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

JavaScript lit de haut en bas. À la ligne 1, `total` n'a pas encore de valeur (*before
initialization*, « avant d'être initialisée »). Déclarez la variable avant de vous en
servir.

### Se tromper de majuscule

```javascript norun
const prixTotal = 12;
console.log(prixtotal);
```

```text
file:///home/alice/js/main.mjs:2
console.log(prixtotal);
            ^

ReferenceError: prixtotal is not defined
    at file:///home/alice/js/main.mjs:2:13

Node.js v22.22.0
```

`prixtotal` et `prixTotal` sont deux noms différents. Recopiez le nom exactement.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
let n = 4;
n = n * 2;
n += 1;
console.log(`n vaut ${n}`);
```

> **Réponse**
>
> `n vaut 9`. `n` passe de 4 à 8, puis à 9.

**Question 2 :** quelle est la différence entre `undefined` et `null` ?

> **Réponse**
>
> `undefined` veut dire « pas encore de valeur » : c'est JavaScript qui la donne à une
> variable vide. `null` veut dire « vide exprès » : c'est vous qui l'écrivez.

**Question 3 :** que répondent `"3" == 3` et `"3" === 3` ? Lequel utiliser ?

> **Réponse**
>
> ```javascript
> console.log("3" == 3);
> console.log("3" === 3);
> ```
>
> ```sortie
> true
> false
> ```
>
> `==` convertit le texte en nombre avant de comparer ; `===` voit deux types
> différents. Utilisez toujours `===`.

## En résumé

- `const` crée une variable qu'on ne réaffecte pas, `let` une variable qui peut changer ;
  évitez `var`.
- Les types de base sont `number`, `string`, `boolean`, `undefined`, `null`, plus
  `bigint` et `symbol` ; `typeof` donne le type.
- Un gabarit `` `Bonjour, ${prenom}` `` insère une valeur dans un texte.
- `console.log(a, b)` affiche plusieurs valeurs ; `//` et `/* */` écrivent des
  commentaires.
- Les noms s'écrivent en camelCase, et les majuscules comptent.
- Comparez toujours avec `===` et `!==`.

Suite : [nombres et calculs](nombres.md).
