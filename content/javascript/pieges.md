# Les pièges du débutant en JavaScript

Cette page rassemble les 11 pièges qui surprennent le plus les débutants en JavaScript, avec pour chacun ce qu'on attend, ce qui se passe vraiment, pourquoi, et comment l'éviter.

> **Objectifs :**
>
> - vous reconnaîtrez les pièges classiques de JavaScript quand vous les rencontrerez ;
> - vous comprendrez pourquoi le résultat est surprenant ;
> - vous saurez écrire la version correcte à chaque fois.

## Comment lire cette page

JavaScript a été créé très vite, en 1995. Il essaie d'être « gentil » : au lieu de
s'arrêter sur une erreur, il devine souvent ce que vous vouliez dire. Parfois, il devine
mal.

Chaque piège suit le même chemin :

```text
 Ce que vous attendez  →  Ce qui se passe  →  Pourquoi  →  La correction
```

Lisez un piège à la fois. Lancez chaque exemple vous-même : un piège qu'on a vu de ses
yeux, on ne l'oublie plus.

## Les pièges des valeurs

### Comparer avec == au lieu de ===

**Vous attendez :** que `0 == ""` réponde `false`. Un nombre n'est pas un texte.

```javascript
console.log(0 == "");
console.log(1 == "1");
console.log(null == undefined);
console.log(0 === "");
console.log(1 === "1");
```

```sortie
true
true
true
false
false
```

**Pourquoi :** `==` (égalité « souple ») **convertit** les deux valeurs avant de les
comparer. Le texte vide `""` devient le nombre `0`, donc `0 == ""` est vrai. `===`
(égalité « stricte ») ne convertit rien : des [types](../debuter/glossaire.md#type)
différents sont toujours différents.

**La correction :** utilisez **toujours** `===` et `!==`. Voir aussi
[les premiers pas](premiers-pas.md#comparer-ou).

> **À retenir :** trois signes `===` pour comparer. Deux signes `==`, jamais.

### "1" + 1 ne fait pas 2

**Vous attendez :** que `"1" + 1` donne `2`.

```javascript
console.log("1" + 1);
console.log("5" - 2);
console.log("5" * "2");
console.log("10" + 5 - 3);
```

```sortie
11
3
10
102
```

**Pourquoi :** le `+` a deux métiers. Avec deux nombres, il additionne. Dès qu'il y a
**un texte**, il colle les deux bouts, comme deux morceaux de ruban. Les autres
opérateurs (`-`, `*`, `/`) ne savent que calculer : ils transforment le texte en nombre.

Suivons `"10" + 5 - 3` pas à pas :

| Étape | Calcul | Résultat |
| --- | --- | --- |
| 1 | `"10" + 5` | `"105"` (un texte, collé) |
| 2 | `"105" - 3` | `102` (un nombre, calculé) |

C'est très courant avec une valeur tapée par l'utilisateur : elle arrive **toujours**
sous forme de texte.

**La correction :** transformez le texte en nombre avec `Number(...)` avant de calculer.

```javascript
const saisie = "1";
console.log(Number(saisie) + 1);
```

```sortie
2
```

Essayez : remplacez `"1"` par `"abc"`. Vous obtenez `NaN`, le piège suivant.

### NaN n'est égal à rien, pas même à lui-même

`NaN` veut dire *Not a Number*, « pas un nombre ». C'est le résultat d'un calcul
impossible.

**Vous attendez :** que `resultat === NaN` vous dise si le calcul a raté.

```javascript
const resultat = Number("abc");

console.log(resultat);
console.log(resultat === NaN);
console.log(NaN === NaN);
console.log(Number.isNaN(resultat));
```

```sortie
NaN
false
false
true
```

**Pourquoi :** par convention, `NaN` n'est égal à **aucune** valeur, lui compris. La
comparaison avec `===` répond donc toujours `false`.

**La correction :** testez avec `Number.isNaN(valeur)`. Voir
[les nombres](nombres.md#nan-pas-un-nombre).

### 0.1 + 0.2 ne fait pas 0.3

**Vous attendez :** `0.3`.

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
console.log((0.1 + 0.2).toFixed(2));
```

```sortie
0.30000000000000004
false
0.30
```

**Pourquoi :** l'ordinateur range les nombres à virgule en
[binaire](../debuter/glossaire.md#binaire) (avec des 0 et des 1). Certains nombres,
comme `0.1`, ne tombent pas juste en binaire, comme `1/3` ne tombe pas juste en décimal
(`0,3333…`). Il reste une toute petite erreur.

**La correction :**

- pour **afficher**, arrondissez avec `toFixed(2)` ;
- pour **comparer**, vérifiez que l'écart est minuscule ;
- pour de l'**argent**, comptez en centimes (des nombres entiers).

```javascript
const ecart = Math.abs(0.1 + 0.2 - 0.3);
console.log(ecart < 0.000001);

const totalCentimes = 10 + 20;
console.log(totalCentimes / 100);
```

```sortie
true
0.3
```

Voir [les nombres](nombres.md#les-nombres-a-virgule-ne-sont-pas-exacts).

### undefined et null : deux sortes de « vide »

**Vous attendez :** qu'il existe une seule valeur pour dire « rien ».

```javascript
let prenom;
const chat = { nom: "Félix" };
const donnees = null;

console.log(prenom);
console.log(chat.age);
console.log(donnees);
console.log(typeof undefined);
console.log(typeof null);
```

```sortie
undefined
undefined
null
undefined
object
```

**Pourquoi :** JavaScript a **deux** valeurs vides.

| Valeur | Qui la met ? | Sens |
| --- | --- | --- |
| `undefined` | JavaScript, tout seul | « on n'a jamais rien mis ici » |
| `null` | vous, exprès | « ici, il n'y a rien, je le sais » |

Et `typeof null` répond `"object"` : c'est une vieille erreur du langage, gardée pour
ne pas casser les anciens programmes.

**La correction :** pour tester « vide ou pas », utilisez `??` pour une valeur de
secours et `?.` pour lire sans planter (voir
[les valeurs absentes](conditions-boucles.md#les-valeurs-absentes-et)).

```javascript
const donnees = null;
console.log(donnees ?? "aucune donnée");
console.log(donnees?.length);
```

```sortie
aucune donnée
undefined
```

> **À retenir :** `undefined` = jamais rempli ; `null` = vidé exprès. `??` et `?.`
> traitent les deux de la même façon.

## Les pièges des tableaux et des objets

### sort() range les nombres comme des mots

**Vous attendez :** `[1, 5, 10, 25, 100]`.

```javascript
const nombres = [10, 1, 5, 100, 25];
nombres.sort();
console.log(nombres);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

**Pourquoi :** sans indication, `sort` transforme chaque nombre en texte et range dans
l'ordre du dictionnaire. Dans un dictionnaire, « 10 » vient avant « 5 », car on compare
d'abord le premier caractère : `1` avant `5`.

**La correction :** donnez une fonction de comparaison.

```javascript
const nombres = [10, 1, 5, 100, 25];
nombres.sort((a, b) => a - b);
console.log(nombres);
```

```sortie
[ 1, 5, 10, 25, 100 ]
```

Essayez : écrivez `b - a` au lieu de `a - b`. L'ordre s'inverse. Voir
[les tableaux](collections.md#trier).

### const n'empêche pas de modifier le contenu

**Vous attendez :** qu'un tableau déclaré avec `const` ne puisse plus changer.

```javascript
const panier = ["pomme"];
panier.push("poire");
panier[0] = "kiwi";
console.log(panier);

const chat = { nom: "Félix" };
chat.nom = "Tom";
console.log(chat);
```

```sortie
[ 'kiwi', 'poire' ]
{ nom: 'Tom' }
```

**Pourquoi :** imaginez une boîte avec une étiquette. `const` colle l'étiquette pour
toujours sur **la même boîte**. Mais vous pouvez toujours changer ce qu'il y a **dans**
la boîte.

```text
 panier ──(collé pour toujours)──▶ [ 'kiwi', 'poire' ]
                                     ▲ le contenu peut changer
```

Ce qui est interdit, c'est de mettre l'étiquette sur une **autre** boîte :
`panier = ["poire"]` provoque `TypeError: Assignment to constant variable.`

**La correction :** si le contenu ne doit vraiment pas changer, gelez-le avec
`Object.freeze`.

```javascript
const jours = Object.freeze(["lundi", "mardi"]);
console.log(Object.isFrozen(jours));
```

```sortie
true
```

### b = a ne copie pas

**Vous attendez :** que `b` soit une copie indépendante de `a`.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);
console.log(a);
```

```sortie
[ 1, 2, 3, 4 ]
```

**Pourquoi :** `b = a` colle une **deuxième étiquette** sur la même boîte. Il n'y a
qu'un seul tableau, avec deux noms.

```text
 a ──┐
     ├──▶ [ 1, 2, 3, 4 ]
 b ──┘
```

**La correction :** faites une copie avec le spread `...`.

```javascript
const a = [1, 2, 3];
const b = [...a];
b.push(4);
console.log(a);
console.log(b);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
```

Attention, cette copie ne copie que le premier niveau. Pour un tableau d'objets, voir
[les tableaux en profondeur](tableaux-avances.md).

> **À retenir :** pour les tableaux et les objets, `=` partage. `[...a]` et `{ ...o }`
> copient.

### this se perd quand on détache une méthode

**Vous attendez :** que la méthode marche partout, même rangée dans une variable.

```javascript norun
const compte = {
  titulaire: "Alice",
  resume() {
    return `Compte de ${this.titulaire}`;
  },
};

const afficher = compte.resume;
console.log(afficher());
```

```text
file:///home/alice/js/main.mjs:4
    return `Compte de ${this.titulaire}`;
                             ^

TypeError: Cannot read properties of undefined (reading 'titulaire')
    at resume (file:///home/alice/js/main.mjs:4:30)
    at file:///home/alice/js/main.mjs:9:13

Node.js v22.22.0
```

**Pourquoi :** `this` n'est pas fixé quand on **écrit** la méthode. Il est choisi au
moment de l'**appel** : c'est l'objet avant le point. Dans `compte.resume()`, `this`
est `compte`. Dans `afficher()`, il n'y a pas de point, donc pas d'objet : `this` vaut
`undefined`.

**La correction :** appelez la méthode avec son objet, ou attachez-la avec `bind`.

```javascript
const compte = {
  titulaire: "Alice",
  resume() {
    return `Compte de ${this.titulaire}`;
  },
};

console.log(compte.resume());

const afficher = compte.resume.bind(compte);
console.log(afficher());
```

```sortie
Compte de Alice
Compte de Alice
```

Voir [les méthodes et this](objets.md#les-methodes-et-this).

## Les pièges du temps et des boucles

### Oublier await

**Vous attendez :** que `lireNote()` vous donne la note `15`.

```javascript
function attendre(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function lireNote() {
  await attendre(10);
  return 15;
}

const note = lireNote();
console.log(note);
console.log(note + 1);
```

```sortie
Promise { <pending> }
[object Promise]1
```

**Pourquoi :** une fonction `async` ne renvoie pas tout de suite la valeur. Elle renvoie
une [promesse](../debuter/glossaire.md#promesse) : un ticket de commande, comme au
restaurant rapide. Le ticket n'est pas le plat ! `<pending>` veut dire « en
préparation ».

**La correction :** attendez le plat avec `await`.

```javascript
function attendre(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function lireNote() {
  await attendre(10);
  return 15;
}

const note = await lireNote();
console.log(note);
console.log(note + 1);
```

```sortie
15
16
```

Voir [l'asynchrone](asynchrone.md#oublier-await).

### var dans une boucle : la même boîte pour tous

**Vous attendez :** `0`, `1`, `2`.

```javascript
for (var i = 0; i < 3; i++) {
  setTimeout(() => console.log("var :", i), 0);
}
```

```sortie
var : 3
var : 3
var : 3
```

**Pourquoi :** `setTimeout` affiche **plus tard**, après la fin de la boucle. Avec
`var`, il n'y a **qu'une seule** boîte `i` pour toute la boucle. Quand les messages
s'affichent, la boucle est finie et `i` vaut 3.

| Moment | i | Ce qui se passe |
| --- | --- | --- |
| tour 1 | 0 | on programme un affichage pour plus tard |
| tour 2 | 1 | on programme un affichage pour plus tard |
| tour 3 | 2 | on programme un affichage pour plus tard |
| fin de boucle | 3 | `3 < 3` est faux : on sort |
| plus tard | 3 | les 3 affichages lisent la même boîte : `3` |

**La correction :** utilisez `let`. Avec `let`, **chaque tour** a sa propre boîte `i`.

```javascript
for (let i = 0; i < 3; i++) {
  setTimeout(() => console.log("let :", i), 0);
}
```

```sortie
let : 0
let : 1
let : 2
```

> **À retenir :** n'utilisez jamais `var`. Écrivez `const`, ou `let` si la valeur doit
> changer.

## Erreurs fréquentes

### Lire une propriété de null

```javascript norun
const reponse = { donnees: null };
console.log(reponse.donnees.length);
```

```text
file:///home/alice/js/main.mjs:2
console.log(reponse.donnees.length);
                            ^

TypeError: Cannot read properties of null (reading 'length')
    at file:///home/alice/js/main.mjs:2:29

Node.js v22.22.0
```

On ne peut rien lire dans une boîte vide. Utilisez `reponse.donnees?.length`, qui donne
`undefined` au lieu de planter.

### Appeler une méthode de nombre sur un texte

```javascript norun
const age = "17";
console.log(age.toFixed(1));
```

```text
file:///home/alice/js/main.mjs:2
console.log(age.toFixed(1));
                ^

TypeError: age.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:17

Node.js v22.22.0
```

`"17"` est un texte, pas un nombre : il n'a pas de méthode `toFixed`. Convertissez
d'abord : `Number(age).toFixed(1)`.

### Remplacer un tableau déclaré avec const

```javascript norun
const panier = ["pomme"];
panier = ["poire"];
```

```text
file:///home/alice/js/main.mjs:2
panier = ["poire"];
       ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:8

Node.js v22.22.0
```

Modifiez le contenu (`panier.length = 0; panier.push("poire")`), ou déclarez avec
`let` si vous devez vraiment changer de tableau.

## Testez-vous

**Question 1 :** qu'affiche `console.log("3" + 4 + 5)` ?

> **Réponse**
>
> `345`. On lit de gauche à droite : `"3" + 4` donne le texte `"34"`, puis
> `"34" + 5` donne `"345"`.
>
> ```javascript
> console.log("3" + 4 + 5);
> console.log(3 + 4 + "5");
> ```
>
> ```sortie
> 345
> 75
> ```
>
> Dans la deuxième ligne, `3 + 4` fait d'abord `7` (deux nombres), puis `7 + "5"`
> colle : `"75"`.

**Question 2 :** comment savoir si `x` vaut `NaN` ?

> **Réponse**
>
> Avec `Number.isNaN(x)`. Le test `x === NaN` répond toujours `false`, car `NaN` n'est
> égal à rien.

**Question 3 :** pourquoi `[3, 20, 100].sort()` donne `[100, 20, 3]` ?

> **Réponse**
>
> Parce que `sort()` sans argument compare des **textes** : `"1"` vient avant `"2"`,
> qui vient avant `"3"`. Écrivez `sort((a, b) => a - b)`.

## En résumé

- Comparez avec `===`, jamais avec `==`.
- `+` colle dès qu'il voit un texte : convertissez avec `Number(...)` ; testez `NaN`
  avec `Number.isNaN`.
- Les nombres à virgule ne sont pas exacts : arrondissez pour afficher, comptez en
  centimes pour l'argent.
- `undefined` = jamais rempli, `null` = vidé exprès ; `??` et `?.` gèrent les deux.
- `sort((a, b) => a - b)` pour les nombres ; `const` n'empêche pas de modifier le
  contenu ; `b = a` partage, `[...a]` copie.
- `this` dépend de l'appel ; une fonction `async` renvoie une promesse à attendre avec
  `await` ; `let` plutôt que `var`.

Suite : [Exercices pour bien démarrer](exercices-faciles.md).
