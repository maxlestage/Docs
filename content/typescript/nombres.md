# Nombres et calculs

Cette page montre comment TypeScript calcule : le type `number`, les opérateurs, le reste `%`, les arrondis, l'affichage, la conversion d'un texte en nombre et les petites surprises des nombres à virgule.

> **Objectifs :**
>
> - vous saurez qu'un seul type, `number`, sert aux entiers et aux nombres à virgule ;
> - vous saurez calculer avec `+ - * / ** %` et modifier une variable avec `+=` ;
> - vous saurez arrondir avec `Math.round`, `Math.floor`, `Math.ceil` et `Math.trunc` ;
> - vous saurez afficher un nombre avec `toFixed` et `Intl.NumberFormat` ;
> - vous saurez transformer un texte en nombre et repérer `NaN` ;
> - vous saurez pourquoi `0.1 + 0.2` ne donne pas tout à fait `0.3`.

## Un seul type pour tous les nombres

Dans beaucoup de langages, il y a un type pour les [entiers](../debuter/glossaire.md#entier)
(`7`) et un autre pour les nombres à virgule, les [flottants](../debuter/glossaire.md#flottant)
(`7.5`). En TypeScript, non : les deux sont des `number`.

```typescript
const pommes: number = 7;
const prix: number = 2.5;
const temperature: number = -3;

console.log(pommes, prix, temperature);
console.log(typeof pommes, typeof prix);
console.log(Number.isInteger(pommes), Number.isInteger(prix));
```

```sortie
7 2.5 -3
number number
true false
```

`Number.isInteger` répond à la question « est-ce un entier ? ».

> **Attention :** en code, la virgule décimale s'écrit avec un **point** : `2.5`, jamais
> `2,5`.

## Les opérateurs de calcul

Un [opérateur](../debuter/glossaire.md#operateur) est un symbole qui calcule. Voici
ceux des nombres :

| Opérateur | Rôle | Exemple | Résultat |
|---|---|---|---|
| `+` | addition | `7 + 2` | `9` |
| `-` | soustraction | `7 - 2` | `5` |
| `*` | multiplication | `7 * 2` | `14` |
| `/` | division | `7 / 2` | `3.5` |
| `**` | puissance | `2 ** 3` | `8` |
| `%` | reste de la division | `7 % 2` | `1` |

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(2 ** 3);
console.log(1 / 0, 0 / 0);
```

```sortie
9 5 14
3.5
8
Infinity NaN
```

- `/` fait une vraie division : `7 / 2` donne `3.5`, pas `3`.
- Diviser par zéro ne plante pas : on obtient `Infinity` (l'infini).
- `0 / 0` donne `NaN` : « *Not a Number* », pas un nombre. Nous y reviendrons.

### L'ordre des calculs

Comme à l'école, `*` et `/` passent avant `+` et `-`. Les parenthèses passent avant
tout.

```typescript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
console.log(10 - 4 - 3);
```

```sortie
14
20
3
```

```text
2 + 3 * 4        (2 + 3) * 4
    -----         -------
2 +  12            5    * 4
   14                 20
```

> **Essayez :** remplacez `10 - 4 - 3` par `10 - (4 - 3)`. Le résultat devient `9` :
> la parenthèse est calculée d'abord.

## Le reste : %

`%` donne ce qui **reste** après une division entière. Pensez à des œufs rangés par
boîtes de 6 : avec 14 œufs, vous remplissez 2 boîtes, et il reste 2 œufs.

```text
14 œufs, boîtes de 6 :

[oooooo] [oooooo]  oo
  boîte 1  boîte 2  reste = 14 % 6 = 2
```

```typescript
console.log(14 % 6);
console.log(10 % 2, 11 % 2);
console.log(Math.floor(14 / 6));
```

```sortie
2
0 1
2
```

Deux usages très courants :

- **pair ou impair** : `n % 2` vaut `0` si `n` est pair, `1` sinon ;
- **découper une durée** : 135 minutes, c'est 2 heures et 15 minutes.

```typescript
const minutes = 135;
const heures = Math.floor(minutes / 60);
const reste = minutes % 60;
console.log(`${heures} h ${reste} min`);
```

```sortie
2 h 15 min
```

| Étape | Calcul | Valeur |
|---|---|---|
| 1 | `minutes / 60` | `2.25` |
| 2 | `Math.floor(2.25)` | `2` → `heures` |
| 3 | `135 % 60` | `15` → `reste` |

> **Essayez :** mettez `minutes = 59`. Vous obtenez `0 h 59 min`.

> **À retenir :** `/` divise, `%` donne le reste. Ensemble, ils découpent un nombre en
> morceaux (heures et minutes, boîtes et œufs restants).

## Modifier une variable qui contient un nombre

Pour ajouter quelque chose à une variable, on peut écrire `total = total + 5`. Il
existe des raccourcis :

| Raccourci | Veut dire |
|---|---|
| `total += 5` | `total = total + 5` |
| `total -= 2` | `total = total - 2` |
| `total *= 3` | `total = total * 3` |
| `total++` | `total = total + 1` |
| `total--` | `total = total - 1` |

```typescript
let total = 10;
total += 5;
total -= 2;
total *= 3;
total++;
console.log(total);
```

```sortie
40
```

| Étape | Instruction | total |
|---|---|---|
| 1 | `let total = 10` | 10 |
| 2 | `total += 5` | 15 |
| 3 | `total -= 2` | 13 |
| 4 | `total *= 3` | 39 |
| 5 | `total++` | 40 |

Ces raccourcis ne marchent qu'avec `let` : une `const` ne change jamais.

## Arrondir avec Math

`Math` est une boîte à outils toujours disponible. Quatre outils arrondissent :

- `Math.round` : au plus proche ;
- `Math.floor` : vers le bas (le « plancher ») ;
- `Math.ceil` : vers le haut (le « plafond ») ;
- `Math.trunc` : coupe tout ce qui est après la virgule.

```typescript
for (const x of [2.4, 2.5, 2.6, -2.5, -2.6]) {
  console.log(x, Math.round(x), Math.floor(x), Math.ceil(x), Math.trunc(x));
}
```

```sortie
2.4 2 2 3 2
2.5 3 2 3 2
2.6 3 2 3 2
-2.5 -2 -3 -2 -2
-2.6 -3 -3 -2 -2
```

Regardez les nombres négatifs : `Math.floor(-2.6)` donne `-3`, car `-3` est **plus
bas** que `-2.6`. `Math.trunc` coupe simplement : `-2`.

Un usage concret de `Math.ceil` : combien de bus de 50 places pour 120 élèves ?

```typescript
const eleves = 120;
console.log(eleves / 50);
console.log(Math.ceil(eleves / 50));
```

```sortie
2.4
3
```

Il faut 3 bus : on ne peut pas commander 2,4 bus.

### Plus petit, plus grand, distance

```typescript
console.log(Math.min(4, 9, 2));
console.log(Math.max(4, 9, 2));
console.log(Math.abs(-7), Math.abs(7));
console.log(Math.sqrt(16));
```

```sortie
2
9
7 7
4
```

`Math.abs` donne la valeur sans le signe : la « distance » à zéro. `Math.sqrt` donne
la racine carrée.

> **Essayez :** pour une note entre 0 et 20, écrivez `Math.min(20, Math.max(0, note))`.
> Avec `note = 23`, vous obtenez `20` ; avec `note = -1`, vous obtenez `0`.

## Afficher un nombre joliment

### toFixed : un nombre fixe de décimales

`toFixed(2)` donne le nombre avec 2 chiffres après la virgule.

```typescript
const prix = 19.9;
console.log(prix.toFixed(2));
console.log((10 / 3).toFixed(2));
console.log((2.5).toFixed(0));
console.log(typeof prix.toFixed(2));
```

```sortie
19.90
3.33
3
string
```

> **Attention :** `toFixed` renvoie un **texte** (`string`), pas un nombre. Utilisez-le
> à la toute fin, juste pour afficher. Ne faites plus de calcul avec.

### Intl.NumberFormat : à la française

En France, on écrit `1 234,50 €` : une espace entre les milliers, une virgule pour les
décimales. `Intl.NumberFormat` connaît ces habitudes. Il est présenté en détail dans
[le chapitre sur le texte](texte.md#formater-des-nombres-avec-intl).

```typescript
const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
const nombre = new Intl.NumberFormat("fr-FR", { maximumFractionDigits: 2 });

console.log(euros.format(1234.5));
console.log(nombre.format(1234567.891));
console.log(nombre.format(0.5));
```

```sortie
1 234,50 €
1 234 567,89
0,5
```

> **Note :** les espaces de ce résultat ne sont pas des espaces ordinaires. Ce sont des
> espaces **insécables** (fines ou normales) : elles empêchent de couper un nombre en
> fin de ligne. Ne comparez donc jamais ce texte à un texte tapé au clavier.

## Du texte au nombre

Un texte qui contient des chiffres reste un texte : `"42"` n'est pas `42`. Quand une
valeur vient d'un formulaire, d'un fichier ou du clavier, c'est **toujours** un texte.
Trois outils la transforment en nombre.

```typescript
console.log(Number("42"), parseInt("42"), parseFloat("42"));
console.log(Number("3.9"), parseInt("3.9"), parseFloat("3.9"));
console.log(Number("12px"), parseInt("12px"), parseFloat("12px"));
console.log(Number(""), parseInt(""), parseFloat(""));
console.log(Number(" 7 "), Number("3,5"));
```

```sortie
42 42 42
3.9 3 3.9
NaN 12 12
0 NaN NaN
7 NaN
```

`Number(" 7 ")` ignore les espaces autour. `Number("3,5")` échoue : en code, la
virgule décimale est un point.

| Outil | Comment il lit | `"3.9"` | `"12px"` | `""` |
|---|---|---|---|---|
| `Number` | tout le texte, ou rien | `3.9` | `NaN` | `0` |
| `parseInt` | le début, jusqu'au premier « non-chiffre », sans décimales | `3` | `12` | `NaN` |
| `parseFloat` | le début, avec les décimales | `3.9` | `12` | `NaN` |

> **À retenir :** utilisez `Number` par défaut : il refuse les textes à moitié
> numériques. Attention à son piège : un texte vide donne `0`, pas `NaN`.

### NaN : « pas un nombre »

Quand la conversion échoue, on obtient `NaN`. Bizarrement, `NaN` est de type `number`.
Et il n'est égal à rien, pas même à lui-même. Pour le repérer, utilisez
`Number.isNaN` :

```typescript
for (const saisie of ["12", "douze", "4.5"]) {
  const n = Number(saisie);
  if (Number.isNaN(n)) {
    console.log(`"${saisie}" n'est pas un nombre`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"douze" n'est pas un nombre
4.5 x 2 = 9
```

`NaN` est contagieux : `NaN + 1` donne `NaN`. Un seul `NaN` au début d'un calcul, et
tout le résultat devient `NaN`. Vérifiez donc vos conversions tout de suite.

## 0.1 + 0.2 : la surprise des virgules

```typescript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

Ce n'est pas un bug de TypeScript. L'ordinateur range les nombres en
[binaire](../debuter/glossaire.md#binaire) (avec des 0 et des 1). Or `0.1` ne s'écrit
pas exactement en binaire, comme `1/3` ne s'écrit pas exactement en décimal
(`0.3333…`). Il reste une minuscule erreur, qui apparaît parfois. Python, Java ou
JavaScript font exactement pareil.

Deux solutions :

1. **arrondir pour afficher**, avec `toFixed` ou `Intl` ;
2. **compter en centimes**, avec des entiers, pour l'argent.

```typescript
const total = 0.1 + 0.2;
console.log(total.toFixed(2));
console.log(Math.round(total * 100) / 100);

const prixEnCentimes = 10 + 20;
console.log(prixEnCentimes / 100);
```

```sortie
0.30
0.3
0.3
```

> **À retenir :** ne comparez jamais deux nombres à virgule avec `===`. Pour de
> l'argent, calculez en centimes (des entiers), et divisez par 100 seulement pour
> afficher.

## Les très grands entiers : bigint

Un `number` reste exact jusqu'à `Number.MAX_SAFE_INTEGER`, environ 9 millions de
milliards. Au-delà, il se trompe.

```typescript
console.log(Number.MAX_SAFE_INTEGER);
console.log(9007199254740992 + 1);
```

```sortie
9007199254740991
9007199254740992
```

`+ 1` n'a rien changé ! Pour des entiers plus grands, il existe un autre type :
`bigint`. On l'écrit avec un `n` à la fin.

```typescript
const grand: bigint = 9007199254740992n;
console.log(grand + 1n);
console.log(2n ** 100n);
console.log(typeof grand);
```

```sortie
9007199254740993n
1267650600228229401496703205376n
bigint
```

Un `bigint` n'a pas de virgule : `7n / 2n` donne `3n`. Vous en aurez rarement besoin
pour débuter.

## Les types littéraux de nombres

Un type peut aussi être **un nombre précis** : `type Face = 1 | 2 | 3 | 4 | 5 | 6;`
n'accepte que ces six valeurs, et `const f: Face = 7;` est refusé. Vous verrez cette
idée en détail dans [les types littéraux](litteraux.md).

## Erreurs fréquentes

### Calculer avec le résultat de toFixed

```typescript norun
const prix = 19.9;
const arrondi: number = prix.toFixed(2);
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const arrondi: number = prix.toFixed(2);
        ~~~~~~~


Found 1 error in main.ts:2
```

`toFixed` renvoie un texte. Pour un nombre arrondi, utilisez
`Math.round(prix * 100) / 100`. Gardez `toFixed` pour l'affichage.

### Tester NaN avec ===

```typescript norun
const n = Number("douze");
if (n === NaN) {
  console.log("pas un nombre");
}
```

```text
main.ts:2:5 - error TS2845: This condition will always return 'false'.

2 if (n === NaN) {
      ~~~~~~~~~

  main.ts:2:5
    2 if (n === NaN) {
          ~
    Did you mean 'Number.isNaN(n)'?


Found 1 error in main.ts:2
```

« Cette condition sera toujours fausse. » `NaN` n'est égal à rien. TypeScript propose
la correction : `if (Number.isNaN(n))`.

### Donner deux arguments à Math.round

```typescript norun
console.log(Math.round(3.14159, 2));
```

```text
main.ts:1:33 - error TS2554: Expected 1 arguments, but got 2.

1 console.log(Math.round(3.14159, 2));
                                  ~


Found 1 error in main.ts:1
```

`Math.round` arrondit toujours à l'entier. Pour 2 décimales :
`Math.round(3.14159 * 100) / 100` donne `3.14`.

### Mélanger bigint et number

```typescript norun
const grand = 10n;
console.log(grand + 1);
```

```text
main.ts:2:13 - error TS2365: Operator '+' cannot be applied to types '10n' and '1'.

2 console.log(grand + 1);
              ~~~~~~~~~


Found 1 error in main.ts:2
```

Un `bigint` ne se mélange pas avec un `number`. Écrivez `grand + 1n`, ou convertissez
avec `BigInt(1)`.

## Testez-vous

**Question 1 :** que valent `17 % 5` et `Math.floor(17 / 5)` ?

> **Réponse**
>
> `2` et `3`. Dans 17, il y a 3 fois 5 (soit 15), et il reste 2.

**Question 2 :** qu'affiche `console.log(Number("8 kg"), parseInt("8 kg"));` ?

> **Réponse**
>
> `NaN 8`. `Number` lit tout le texte : `" kg"` le fait échouer. `parseInt` lit le
> début, `8`, et s'arrête au premier caractère qui n'est pas un chiffre.

**Question 3 :** pourquoi compter un prix en centimes plutôt qu'en euros ?

> **Réponse**
>
> Les centimes sont des entiers, et les entiers sont exacts. Les nombres à virgule ont
> de minuscules erreurs : `0.1 + 0.2` donne `0.30000000000000004`.

## En résumé

- `number` sert à tous les nombres, entiers ou à virgule ; la virgule s'écrit `.`.
- `/` divise vraiment, `%` donne le reste, `**` la puissance ; `+=` et `++` modifient
  une variable `let`.
- `Math.round`, `Math.floor`, `Math.ceil` et `Math.trunc` arrondissent chacun à leur
  façon.
- `toFixed` et `Intl.NumberFormat` servent à afficher : ils renvoient du texte.
- `Number(texte)` convertit ; un échec donne `NaN`, qu'on teste avec `Number.isNaN`.
- `0.1 + 0.2` n'est pas exactement `0.3` : arrondissez pour afficher, comptez l'argent
  en centimes.

Suite : [conditions et boucles](conditions-boucles.md).
