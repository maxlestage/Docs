# Nombres et calculs

Cette page explique comment JavaScript calcule : les opérateurs, l'objet `Math`, les surprises des nombres à virgule, `NaN`, `Infinity`, `BigInt` et l'affichage des nombres à la française.

> **Objectifs :**
>
> - vous saurez faire des calculs avec `+ - * /`, le reste `%` et la puissance `**` ;
> - vous saurez arrondir et utiliser les fonctions de `Math` ;
> - vous saurez pourquoi `0.1 + 0.2` ne donne pas exactement `0.3` ;
> - vous saurez transformer un texte en nombre et reconnaître `NaN` ;
> - vous saurez afficher un nombre au format français avec `Intl.NumberFormat`.

## Un seul type de nombre

Les programmes calculent tout le temps : le total d'un panier, un score, l'âge d'une
personne. Avant de faire des calculs, voyons à quoi ressemble un nombre en JavaScript.

Beaucoup de langages séparent les [entiers](../debuter/glossaire.md#entier) (`3`) et les
[flottants](../debuter/glossaire.md#flottant) (`3.5`). JavaScript, lui, n'a qu'un seul
type pour les deux : `number`.

```javascript
console.log(typeof 3);
console.log(typeof 3.5);
console.log(3 === 3.0);
console.log(Number.isInteger(5.0));
```

```sortie
number
number
true
true
```

Ligne par ligne :

- `typeof 3` demande le type de la valeur `3` : la réponse est `number` ;
- `typeof 3.5` donne aussi `number` : même type, avec ou sans virgule ;
- `3 === 3.0` compare les deux valeurs : elles sont égales, donc `true` ;
- `Number.isInteger(5.0)` répond `true`, car `5.0` n'a rien après la virgule.

`3` et `3.0` sont exactement la même valeur. `Number.isInteger` dit si un nombre n'a pas
de partie décimale.

## Les opérateurs

Un programme sert souvent à calculer à votre place, comme une calculatrice : le prix de
trois cafés, la moyenne d'une classe. Les opérateurs sont les signes qui écrivent ces
calculs.

Les [opérateurs](../debuter/glossaire.md#operateur) de calcul sont ceux de l'école, plus
deux nouveaux.

```javascript
console.log(7 + 2);
console.log(7 - 2);
console.log(7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
```

```sortie
9
5
14
3.5
1
1024
```

- `/` est une vraie division : `7 / 2` donne `3.5`, pas `3`.
- `%` donne le **reste** de la division entière : 7 = 2 × 3 + **1**.
- `**` est la puissance : `2 ** 10` veut dire 2 multiplié 10 fois par lui-même.

Les priorités sont celles des maths : `**`, puis `*` `/` `%`, puis `+` `-`. Des
parenthèses changent l'ordre.

```javascript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
```

```sortie
14
20
```

### À quoi sert le reste ?

Le reste `%` répond à des questions de tous les jours : un nombre est-il pair ? Combien
reste-t-il de minutes après les heures pleines ?

```javascript
const n = 17;
console.log(n % 2 === 0);

const minutes = 135;
console.log(`${Math.floor(minutes / 60)} h ${minutes % 60} min`);
```

```sortie
false
2 h 15 min
```

Un nombre est pair si son reste par 2 vaut 0.

### Les raccourcis

Comme `+=` déjà vu, il existe `-=`, `*=`, `/=`. Pour ajouter ou retirer 1, on écrit
`++` et `--`.

```javascript
let vies = 3;
vies -= 1;
vies *= 10;
vies++;
console.log(vies);
```

```sortie
21
```

**À vous :** écrivez un programme qui partage 23 bonbons entre 5 enfants. Il
affiche combien de bonbons chaque enfant reçoit, puis combien il en reste.

> **Solution**
>
> ```javascript
> const bonbons = 23;
> const enfants = 5;
> console.log(Math.floor(bonbons / enfants));
> console.log(bonbons % enfants);
> ```
>
> ```sortie
> 4
> 3
> ```

## L'objet Math

Certains calculs reviennent sans cesse : arrondir un prix, trouver la meilleure note
d'une liste. Plutôt que de les écrire vous-même, vous les prenez tout prêts.

`Math` est une boîte à outils livrée avec JavaScript. Elle contient des fonctions de
calcul et quelques constantes.

```javascript
console.log(Math.round(2.6));
console.log(Math.floor(2.6));
console.log(Math.ceil(2.1));
console.log(Math.trunc(-2.6));
console.log(Math.abs(-4));
console.log(Math.sqrt(16));
console.log(Math.max(3, 8, 1));
console.log(Math.min(3, 8, 1));
console.log(Math.PI);
```

```sortie
3
2
3
-2
4
4
8
1
3.141592653589793
```

| Fonction | Rôle |
| --- | --- |
| `Math.round(x)` | arrondit au plus proche |
| `Math.floor(x)` | arrondit vers le bas (le « plancher ») |
| `Math.ceil(x)` | arrondit vers le haut (le « plafond ») |
| `Math.trunc(x)` | coupe la partie décimale |
| `Math.abs(x)` | valeur absolue (enlève le signe moins) |
| `Math.sqrt(x)` | racine carrée |
| `Math.max(...)`, `Math.min(...)` | le plus grand, le plus petit |

`Math.round` arrondit à l'entier. Pour garder deux chiffres après la virgule, une
astuce : multipliez par 100, arrondissez, puis divisez par 100.

```javascript
console.log(Math.round(3.456 * 100) / 100);
```

```sortie
3.46
```

### Un nombre au hasard

`Math.random()` donne un nombre au hasard entre 0 (compris) et 1 (non compris). Pour un
dé à six faces, on l'étire puis on l'arrondit vers le bas :

```javascript norun
const de = Math.floor(Math.random() * 6) + 1;
console.log(de);
```

```text
4
```

Le résultat change à chaque lancement : le vôtre sera sans doute différent.

**À vous :** écrivez un programme qui affiche la meilleure des trois notes 12, 17
et 9, puis leur moyenne arrondie à l'entier le plus proche.

> **Solution**
>
> ```javascript
> console.log(Math.max(12, 17, 9));
> console.log(Math.round((12 + 17 + 9) / 3));
> ```
>
> ```sortie
> 17
> 13
> ```

## Les nombres à virgule ne sont pas exacts

Cette partie vous évite un piège classique, par exemple un total de panier avec une
quinzaine de chiffres après la virgule. Mieux vaut le connaître avant de le rencontrer.

Voici une surprise célèbre :

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

L'ordinateur range les nombres en binaire (avec des 0 et des 1). Certains nombres
décimaux simples, comme 0,1, ne s'écrivent pas exactement en binaire, tout comme 1/3
ne s'écrit pas exactement en décimal (0,3333…). Il reste une toute petite erreur.

Ce n'est pas un bug de JavaScript : Python, Ruby, Java… font pareil. Deux règles
suffisent :

- pour **afficher**, arrondissez (voir `toFixed` ci-dessous) ;
- pour **comparer**, vérifiez que l'écart est minuscule.

```javascript
const somme = 0.1 + 0.2;
console.log(Math.abs(somme - 0.3) < Number.EPSILON);
```

```sortie
true
```

`Number.EPSILON` est un nombre minuscule, prévu pour ce genre de comparaison.

> **Astuce :** pour de l'argent, comptez en **centimes** avec des entiers : `1999`
> plutôt que `19.99`. Les entiers, eux, sont exacts.

## Nombres et texte

Les nombres et les textes passent souvent de l'un à l'autre. Ce que l'on tape au
clavier arrive comme un texte, même « 42 ». Et pour afficher un prix proprement, on
transforme un nombre en texte. Cette partie fait le pont dans les deux sens.

### Afficher avec toFixed

`toFixed(n)` renvoie un **texte** avec exactement `n` chiffres après la virgule.

```javascript
const prix = 0.1 + 0.2;
console.log(prix.toFixed(2));
console.log((10).toFixed(2));
console.log(typeof prix.toFixed(2));
```

```sortie
0.30
10.00
string
```

Ligne par ligne :

- `prix` reçoit `0.1 + 0.2`, c'est-à-dire `0.30000000000000004` ;
- `prix.toFixed(2)` garde deux chiffres après la virgule et arrondit : `0.30` ;
- `(10).toFixed(2)` ajoute les zéros qui manquent : `10.00` ;
- `typeof` montre que le résultat de `toFixed` est un `string`, donc un texte.

Le résultat est une chaîne : parfait pour l'affichage, pas pour continuer à calculer.

> **Note :** `(10).toFixed(2)` a besoin de parenthèses. Sans elles, `10.` serait lu comme
> le début d'un nombre à virgule.

### Transformer un texte en nombre

Un texte qui contient des chiffres n'est pas un nombre. Trois outils le convertissent :

```javascript
console.log(Number("42"));
console.log(Number("3.5"));
console.log(Number("12px"));
console.log(parseInt("12px"));
console.log(parseInt("3.9"));
console.log(parseFloat("3.5kg"));
```

```sortie
42
3.5
NaN
12
3
3.5
```

- `Number(texte)` est strict : tout le texte doit être un nombre.
- `parseInt(texte)` lit un entier au début du texte et s'arrête au premier caractère
  qui n'est pas un chiffre.
- `parseFloat(texte)` fait pareil, mais garde la partie décimale.

> **À retenir :** préférez `Number(...)`. Il refuse les textes douteux, au lieu d'en
> prendre un morceau en silence.

**À vous :** écrivez un programme qui transforme le texte `"19.99"` en nombre, lui
ajoute 5, puis affiche le résultat avec deux chiffres après la virgule.

> **Solution**
>
> ```javascript
> const texte = "19.99";
> const total = Number(texte) + 5;
> console.log(total.toFixed(2));
> ```
>
> ```sortie
> 24.99
> ```

## NaN : « pas un nombre »

Un utilisateur peut taper « trente » au lieu de « 30 ». Votre programme doit alors
savoir repérer que la conversion a échoué.

Quand un calcul n'a pas de sens, JavaScript ne s'arrête pas : il renvoie `NaN` (*Not a
Number*, « pas un nombre »). Et `NaN` contamine tous les calculs qui l'utilisent.

```javascript
const age = Number("trente");
console.log(age);
console.log(age + 1);
console.log(typeof age);
console.log(age === NaN);
console.log(Number.isNaN(age));
```

```sortie
NaN
NaN
number
false
true
```

Ligne par ligne :

- `Number("trente")` ne sait pas lire ce mot comme un nombre : `age` vaut `NaN` ;
- `age + 1` donne encore `NaN` : un calcul avec `NaN` donne toujours `NaN` ;
- `typeof age` répond `number`, ce qui surprend (voir plus bas) ;
- `age === NaN` répond `false`, alors que `Number.isNaN(age)` répond bien `true`.

Deux bizarreries :

- `typeof NaN` répond `"number"` : `NaN` est une valeur spéciale du type nombre ;
- `NaN` n'est égal à rien, **pas même à lui-même** : `age === NaN` est toujours faux.

Pour tester `NaN`, utilisez `Number.isNaN(x)`. Il existe aussi un vieux `isNaN(x)`, qui
convertit d'abord son argument en nombre : `isNaN("abc")` répond `true` alors que
`"abc"` est un texte, pas `NaN`.

```javascript
console.log(isNaN("abc"));
console.log(Number.isNaN("abc"));
```

```sortie
true
false
```

## Infinity

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Diviser par zéro ne provoque pas d'erreur en JavaScript : on obtient l'infini.

```javascript
console.log(1 / 0);
console.log(-1 / 0);
console.log(0 / 0);
console.log(Number.isFinite(1 / 0));
```

```sortie
Infinity
-Infinity
NaN
false
```

`Number.isFinite(x)` vérifie qu'un nombre est un vrai nombre : ni `Infinity`, ni `NaN`.

## Les très grands entiers : BigInt

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un `number` garde les entiers exacts jusqu'à environ 9 millions de milliards. Au-delà,
il perd en précision.

```javascript
console.log(Number.MAX_SAFE_INTEGER);
console.log(2 ** 53);
console.log(2 ** 53 + 1);
```

```sortie
9007199254740991
9007199254740992
9007199254740992
```

`2 ** 53 + 1` devrait finir par 3. Pour les entiers géants, ajoutez un `n` à la fin :
c'est un `BigInt`, exact quelle que soit sa taille.

```javascript
console.log(2n ** 53n + 1n);
console.log(2n ** 64n);
console.log(7n / 2n);
console.log(BigInt(10) + 5n);
```

```sortie
9007199254740993n
18446744073709551616n
3n
15n
```

La division de deux `BigInt` coupe la partie décimale : `7n / 2n` donne `3n`.

## Afficher à la française

En France, on écrit `1 234 567,89` : une espace entre les milliers et une virgule avant
les décimales. `Intl.NumberFormat` met en forme un nombre selon une langue.

```javascript
const fr = new Intl.NumberFormat("fr-FR");
console.log(fr.format(1234567.891));

const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
console.log(euros.format(1234.5));

const pourcent = new Intl.NumberFormat("fr-FR", { style: "percent" });
console.log(pourcent.format(0.25));
```

```sortie
1 234 567,891
1 234,50 €
25 %
```

- `"fr-FR"` veut dire « français de France ».
- `style: "currency"` affiche une somme d'argent, avec la devise `EUR`.
- `style: "percent"` multiplie par 100 et ajoute `%`.

> **Attention :** les espaces ne sont pas des espaces ordinaires. Entre les milliers,
> c'est une **espace fine insécable** (code Unicode U+202F) ; avant `€` et `%`, une
> **espace insécable** (U+00A0). Elles empêchent un retour à la ligne au milieu d'un
> nombre. Conséquence : `euros.format(1234.5) === "1 234,50 €"` (tapé au clavier)
> répond `false`.

**À vous :** écrivez un programme qui affiche le prix 12500 en euros, au format
français.

> **Solution**
>
> ```javascript
> const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
> console.log(euros.format(12500));
> ```
>
> ```sortie
> 12 500,00 €
> ```

## Erreurs fréquentes

### Additionner un texte et un nombre

```javascript norun
const quantite = "5";
console.log(quantite + 3);
```

```text
53
```

Pas de message d'erreur, mais un résultat faux. Avec un texte, `+` **colle** au lieu
d'additionner : `"5" + 3` donne le texte `"53"`. Convertissez d'abord :
`Number(quantite) + 3` donne `8`.

### Appeler toFixed sur un texte

```javascript norun
const n = "3.14159";
console.log(n.toFixed(2));
```

```text
file:///home/alice/js/main.mjs:2
console.log(n.toFixed(2));
              ^

TypeError: n.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:15

Node.js v22.22.0
```

`toFixed` existe pour les nombres, pas pour les textes (*is not a function*, « n'est pas
une fonction »). Convertissez : `Number(n).toFixed(2)`.

### Mélanger BigInt et number

```javascript norun
const grand = 10n;
console.log(grand + 1);
```

```text
file:///home/alice/js/main.mjs:2
console.log(grand + 1);
                  ^

TypeError: Cannot mix BigInt and other types, use explicit conversions
    at file:///home/alice/js/main.mjs:2:19

Node.js v22.22.0
```

On ne mélange pas un `BigInt` et un `number` dans un calcul. Écrivez `grand + 1n`, ou
convertissez : `Number(grand) + 1`.

### Comparer avec NaN

```javascript norun
const age = Number("abc");
if (age === NaN) {
  console.log("Âge invalide");
}
```

```text
(rien ne s'affiche)
```

`NaN` n'est égal à rien, même pas à lui-même : la condition est toujours fausse.
Écrivez `if (Number.isNaN(age))`.

## Testez-vous

**Question 1 :** que valent `17 % 5` et `2 ** 3` ?

> **Réponse**
>
> ```javascript
> console.log(17 % 5);
> console.log(2 ** 3);
> ```
>
> ```sortie
> 2
> 8
> ```
>
> 17 = 5 × 3 + **2**, et 2 × 2 × 2 = 8.

**Question 2 :** pourquoi `0.1 + 0.2 === 0.3` répond-il `false` ?

> **Réponse**
>
> Parce que 0,1 et 0,2 ne s'écrivent pas exactement en binaire : la somme vaut
> `0.30000000000000004`. Arrondissez pour afficher, et comparez l'écart avec
> `Number.EPSILON`.

**Question 3 :** comment afficher le prix `19.9` sous la forme `19.90` ?

> **Réponse**
>
> ```javascript
> console.log((19.9).toFixed(2));
> ```
>
> ```sortie
> 19.90
> ```
>
> Pour le format français (`19,90 €`), utilisez `Intl.NumberFormat("fr-FR", ...)`.

## En résumé

- JavaScript n'a qu'un type `number` pour les entiers et les nombres à virgule.
- `%` donne le reste, `**` la puissance ; `Math` contient `round`, `floor`, `ceil`,
  `max`, `min`…
- Les nombres à virgule sont approximatifs : arrondissez avec `toFixed` pour afficher.
- `Number("...")` convertit un texte ; un calcul impossible donne `NaN`, à tester avec
  `Number.isNaN`.
- Diviser par zéro donne `Infinity` ; les entiers géants s'écrivent en `BigInt` (`10n`).
- `Intl.NumberFormat("fr-FR")` affiche les nombres à la française.

Suite : [conditions et boucles](conditions-boucles.md).
