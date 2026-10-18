# Les fonctions

Cette page explique comment écrire des fonctions en JavaScript, avec `function` ou avec une flèche `=>`, leur donner des paramètres et récupérer leur résultat.

> **Objectifs :**
>
> - vous saurez déclarer une fonction avec `function` et l'appeler ;
> - vous saurez renvoyer une valeur avec `return` ;
> - vous saurez utiliser des paramètres par défaut et des paramètres « reste » ;
> - vous saurez écrire une fonction fléchée `=>` ;
> - vous saurez passer une fonction à une autre fonction (un *callback*).

## Déclarer une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez.

```javascript
function saluer() {
  console.log("Bonjour !");
}

saluer();
saluer();
```

```sortie
Bonjour !
Bonjour !
```

- `function` annonce une fonction, suivie de son nom (`saluer`) et de parenthèses.
- Le code de la fonction, son **corps**, est entre accolades `{ }`.
- Écrire la fonction ne lance rien : cela enregistre la recette. Écrire `saluer()`
  **appelle** la fonction, c'est-à-dire l'exécute. Les parenthèses sont obligatoires.

## Les paramètres

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la fonction
reçoit. Ce sont des cases vides, remplies au moment de l'appel. Les valeurs données à
l'appel s'appellent les [arguments](../debuter/glossaire.md#argument).

```javascript
function saluer(prenom) {
  console.log(`Bonjour, ${prenom} !`);
}

saluer("Alice");
saluer("Bob");
```

```sortie
Bonjour, Alice !
Bonjour, Bob !
```

Ici, `prenom` est le paramètre ; `"Alice"` et `"Bob"` sont les arguments. Plusieurs
paramètres se séparent par des virgules, et les arguments sont donnés dans le même
ordre.

## Renvoyer une valeur avec return

`return` arrête la fonction et **renvoie** une valeur au code qui l'a appelée. La valeur
renvoyée remplace l'appel.

```javascript
function aire(largeur, hauteur) {
  return largeur * hauteur;
}

console.log(aire(3, 4));
const total = aire(2, 5) + 1;
console.log(total);
```

```sortie
12
11
```

`aire(2, 5) + 1` devient `10 + 1`. Une fonction sans `return` renvoie `undefined`.

`return` sert aussi à sortir plus tôt :

```javascript
function diviser(a, b) {
  if (b === 0) {
    return "division impossible";
  }
  return a / b;
}

console.log(diviser(10, 4));
console.log(diviser(10, 0));
```

```sortie
2.5
division impossible
```

### Afficher ou renvoyer ?

`console.log` **affiche** une valeur à l'écran. `return` la **donne** au code
appelant, qui peut s'en servir : la stocker, la recalculer, l'afficher.

```javascript
function doubleAffiche(x) {
  console.log(x * 2);
}

function double(x) {
  return x * 2;
}

const a = doubleAffiche(5);
const b = double(5);
console.log(a, b);
```

```sortie
10
undefined 10
```

> **À retenir :** préférez les fonctions qui renvoient une valeur. Vous choisirez
> ensuite quoi en faire.

## Les paramètres par défaut

Si un argument manque, son paramètre vaut `undefined`. Une **valeur par défaut**, écrite
avec `=`, le remplace.

```javascript
function saluer(prenom, salutation = "Bonjour") {
  return `${salutation}, ${prenom} !`;
}

console.log(saluer("Alice"));
console.log(saluer("Bob", "Salut"));
```

```sortie
Bonjour, Alice !
Salut, Bob !
```

## Les paramètres reste

Avec trois points `...` devant le dernier paramètre, la fonction reçoit **tous les
arguments restants**, rangés dans un tableau.

```javascript
function somme(...nombres) {
  let total = 0;
  for (const n of nombres) {
    total += n;
  }
  return total;
}

console.log(somme(1, 2));
console.log(somme(1, 2, 3, 4));
console.log(somme());
```

```sortie
3
10
0
```

C'est un peu comme un sac : on y met autant d'arguments qu'on veut. Les tableaux sont
expliqués dans [les tableaux](collections.md).

## Les fonctions fléchées

Une **fonction fléchée** (*arrow function*) est une façon plus courte d'écrire une
fonction. On la range dans une constante.

```javascript
const carre = (x) => x * x;
const ajouter = (a, b) => a + b;
const direBonjour = () => "Bonjour !";

console.log(carre(4));
console.log(ajouter(2, 3));
console.log(direBonjour());
```

```sortie
16
5
Bonjour !
```

- Les paramètres sont avant la flèche `=>`, le résultat après.
- Sur une ligne, le résultat est **renvoyé automatiquement**, sans `return`.
- Sans paramètre, on écrit des parenthèses vides `()`.

Quand le corps fait plusieurs lignes, on met des accolades, et `return` redevient
obligatoire :

```javascript
const prixTTC = (prix, taux = 0.2) => {
  const total = prix * (1 + taux);
  return Math.round(total * 100) / 100;
};

console.log(prixTTC(50));
console.log(prixTTC(50, 0.055));
```

```sortie
60
52.75
```

> **Astuce :** utilisez `function` pour les fonctions principales de votre programme, et
> les flèches pour les petites fonctions passées à d'autres (voir plus bas). Les deux
> sont correctes.

## Les fonctions sont des valeurs

En JavaScript, une fonction est une valeur, comme un nombre ou un texte. On peut la
ranger dans une variable, la mettre dans un tableau, ou la donner à une autre fonction.

```javascript
function double(x) {
  return x * 2;
}

const operation = double;
console.log(operation(21));
console.log(typeof double);
```

```sortie
42
function
```

Attention à la différence : `double` est la fonction elle-même (la recette) ;
`double(21)` est son résultat (le plat).

### Les callbacks

Une fonction donnée en argument à une autre s'appelle un **callback** (« rappel ») :
l'autre fonction la « rappellera » quand elle en aura besoin.

```javascript
function appliquerDeuxFois(f, valeur) {
  return f(f(valeur));
}

console.log(appliquerDeuxFois((x) => x + 10, 1));
console.log(appliquerDeuxFois((x) => x * x, 3));
```

```sortie
21
81
```

Les tableaux utilisent beaucoup les callbacks. `map` applique une fonction à chaque
élément :

```javascript
const prix = [10, 20, 30];
const prixTTC = prix.map((p) => p * 1.2);
console.log(prixTTC);
```

```sortie
[ 12, 24, 36 ]
```

Vous reverrez `map`, `filter` et leurs amis dans [les tableaux](collections.md).

## Le hissage

Une fonction déclarée avec `function` peut être appelée **avant** l'endroit où elle est
écrite. JavaScript la « hisse » (*hoisting*) en haut du fichier avant de commencer.

```javascript
console.log(triple(5));

function triple(x) {
  return x * 3;
}
```

```sortie
15
```

Ce n'est **pas** le cas d'une fonction fléchée rangée dans une `const` : la constante
n'existe qu'à partir de sa ligne. Voir les erreurs fréquentes plus bas.

## Une fonction qui s'appelle elle-même

Une fonction peut s'appeler elle-même : c'est la
[récursion](../debuter/glossaire.md#recursion). Il faut un **cas d'arrêt**, sinon elle
ne s'arrête jamais.

```javascript
function compteARebours(n) {
  if (n === 0) {
    console.log("Partez !");
    return;
  }
  console.log(n);
  compteARebours(n - 1);
}

compteARebours(3);
```

```sortie
3
2
1
Partez !
```

Un exemple classique est la factorielle : 4! = 4 × 3 × 2 × 1.

```javascript
function factorielle(n) {
  if (n <= 1) {
    return 1;
  }
  return n * factorielle(n - 1);
}

console.log(factorielle(4));
```

```sortie
24
```

## Erreurs fréquentes

### Oublier return dans une flèche avec accolades

```javascript norun
const double = (x) => {
  x * 2;
};
console.log(double(5));
```

```text
undefined
```

Avec des accolades, le résultat n'est plus renvoyé tout seul : la fonction renvoie
`undefined`. Ajoutez `return x * 2;`, ou enlevez les accolades : `(x) => x * 2`.

### Appeler une flèche avant de la définir

```javascript norun
console.log(carre(4));

const carre = (x) => x * x;
```

```text
file:///home/alice/js/main.mjs:1
console.log(carre(4));
            ^

ReferenceError: Cannot access 'carre' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

Une `const` n'existe qu'à partir de sa ligne. Placez la définition avant l'appel, ou
utilisez une déclaration `function carre(x) { ... }`, qui est hissée.

### Oublier les parenthèses de l'appel

```javascript norun
function saluer() {
  return "Bonjour";
}
console.log(saluer);
```

```text
[Function: saluer]
```

Sans parenthèses, vous parlez de la fonction elle-même, sans l'exécuter. Écrivez
`saluer()`.

### Oublier le cas d'arrêt

```javascript norun
function compter(n) {
  return compter(n - 1);
}
compter(3);
```

```text
file:///home/alice/js/main.mjs:2
  return compter(n - 1);
  ^

RangeError: Maximum call stack size exceeded
    at compter (file:///home/alice/js/main.mjs:2:3)
    at compter (file:///home/alice/js/main.mjs:2:10)
    at compter (file:///home/alice/js/main.mjs:2:10)
```

La fonction s'appelle sans fin, jusqu'à remplir la mémoire réservée aux appels (*maximum
call stack size exceeded*, « taille maximale de la pile d'appels dépassée »). Ajoutez un
cas d'arrêt : `if (n === 0) return 0;`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
function f(x) {
  return x + 1;
  return x * 2;
}
console.log(f(5));
```

> **Réponse**
>
> `6`. Le premier `return` arrête la fonction : la ligne suivante n'est jamais exécutée.

**Question 2 :** écrivez une fonction fléchée `moyenne(a, b)` qui renvoie la moyenne de
deux nombres.

> **Réponse**
>
> ```javascript
> const moyenne = (a, b) => (a + b) / 2;
> console.log(moyenne(3, 4));
> ```
>
> ```sortie
> 3.5
> ```
>
> Les parenthèses autour de `a + b` font l'addition avant la division.

**Question 3 :** que vaut `appliquer((n) => n * 10, 4)` si
`function appliquer(f, x) { return f(x); }` ?

> **Réponse**
>
> `40`. `appliquer` appelle le callback `f` avec `4`, qui renvoie `4 * 10`.

## En résumé

- `function nom(paramètres) { ... }` déclare une fonction ; `nom(arguments)` l'appelle.
- `return` renvoie une valeur et arrête la fonction ; sans `return`, on obtient
  `undefined`.
- `param = valeur` donne une valeur par défaut ; `...reste` rassemble les arguments
  restants dans un tableau.
- `(x) => x * 2` est une fonction fléchée : sans accolades, elle renvoie son résultat
  toute seule.
- Une fonction est une valeur : on peut la passer en argument (un callback).
- Une déclaration `function` est hissée ; une flèche dans une `const` ne l'est pas.

Suite : [travailler avec du texte](texte.md).
