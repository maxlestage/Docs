# Les pièges du débutant en TypeScript

Cette page rassemble douze pièges qui surprennent presque tous les débutants en TypeScript : pour chacun, ce que vous attendez, ce qui se passe vraiment, pourquoi, et comment l'éviter.

> **Objectifs :**
>
> - vous saurez pourquoi `any` et `as` font taire TypeScript, et quand c'est dangereux ;
> - vous saurez que les types disparaissent à l'exécution ;
> - vous saurez vous méfier des cases de tableau vides, des textes qui ressemblent à
>   des nombres et de `||` avec `0` ;
> - vous saurez distinguer `object` et `{}` ;
> - vous comprendrez les surprises du typage structurel et des `enum`.

## Avant de commencer : TypeScript vérifie, puis s'efface

Gardez une image en tête. TypeScript est un **contrôleur** à l'entrée d'une usine. Il
inspecte votre code **avant** le lancement. Ensuite, `tsc` retire tous les types, et
c'est du JavaScript ordinaire qui tourne.

```text
 votre code .ts ──► [ tsc vérifie les types ] ──► code .js sans types ──► node l'exécute
                        le contrôleur                   le contrôleur est parti
```

Beaucoup de pièges viennent de là : le contrôleur ne voit que ce que vous lui
montrez, et il n'est plus là quand le programme tourne.

## Les pièges

### 1. any débranche toute la vérification

**Vous pensez :** « TypeScript me préviendra si je me trompe. »

```typescript norun
let valeur: any = 42;
console.log(valeur.toUpperCase());
```

`tsc` ne dit rien. Mais le programme plante au lancement (extrait du message) :

```text
TypeError: valeur.toUpperCase is not a function
```

**Pourquoi :** [`any`](../debuter/glossaire.md#any) veut dire « ne vérifie rien ». Le
contrôleur ferme les yeux. Ici, `valeur` est un nombre, et un nombre n'a pas de
`toUpperCase`.

**La correction :** utilisez `unknown`. Il oblige à vérifier avant d'utiliser.

```typescript
const valeur: unknown = 42;

if (typeof valeur === "string") {
  console.log(valeur.toUpperCase());
} else {
  console.log("Ce n'est pas un texte :", valeur);
}
```

```sortie
Ce n'est pas un texte : 42
```

> **À retenir :** chaque `any` est un trou dans le filet. Préférez `unknown`.

### 2. as ment à TypeScript

**Vous pensez :** « `as Utilisateur` transforme mon objet en utilisateur. »

```typescript norun
interface Utilisateur {
  nom: string;
}
const u = {} as Utilisateur;
console.log(u.nom.toUpperCase());
```

`tsc` accepte. Au lancement :

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Pourquoi :** `as` ne transforme rien. C'est une **promesse** faite au contrôleur :
« crois-moi, c'est un `Utilisateur` ». Le contrôleur vous croit. Mais l'objet est
vide : `u.nom` vaut `undefined`.

**La correction :** construisez un vrai objet complet. TypeScript vérifiera alors
qu'il ne manque rien.

```typescript
interface Utilisateur {
  nom: string;
}
const u: Utilisateur = { nom: "Alice" };
console.log(u.nom.toUpperCase());
```

```sortie
ALICE
```

> **Essayez :** retirez `nom: "Alice"`. Cette fois, `tsc` refuse : la propriété `nom`
> manque. Avec `: Utilisateur`, le contrôleur travaille. Avec `as`, il se tait.

### 3. Les types disparaissent à l'exécution (JSON.parse)

**Vous pensez :** « J'ai écrit `prix: number`, donc `prix` est un nombre. »

```typescript
interface Produit {
  nom: string;
  prix: number;
}

const texte = '{"nom": "Café", "prix": "2.5"}';
const produit: Produit = JSON.parse(texte);

console.log(produit.prix + 1);
console.log(typeof produit.prix);
```

```sortie
2.51
string
```

**Pourquoi :** `JSON.parse` lit un texte au moment de l'exécution. TypeScript ne peut
pas savoir ce qu'il contient : il renvoie `any`. Votre annotation `: Produit` est
donc une promesse, comme `as`. Ici, le [JSON](../debuter/glossaire.md#json) contient
`"2.5"` entre guillemets : un texte. `+ 1` colle `"1"` au lieu d'additionner.

**La correction :** vérifiez les données venues de l'extérieur, puis convertissez.

```typescript
const texte = '{"nom": "Café", "prix": "2.5"}';
const brut: unknown = JSON.parse(texte);

if (typeof brut === "object" && brut !== null && "prix" in brut) {
  const prix = Number(brut.prix);
  console.log(prix + 1);
}
```

```sortie
3.5
```

> **À retenir :** les types n'existent pas pendant l'exécution. Tout ce qui vient d'un
> fichier, d'un serveur ou du clavier doit être **vérifié**.

### 4. Ce qui vient du clavier est un texte

**Vous pensez :** « L'utilisateur a tapé 5, j'ajoute 3, j'obtiens 8. »

```typescript
const saisie = "5";
console.log(saisie + 3);
console.log(saisie * 1 === 5 ? "nombre" : "texte");
```

Le second `console.log` ne compile même pas. Retirons-le :

```typescript
const saisie = "5";
console.log(saisie + 3);
```

```sortie
53
```

**Pourquoi :** une saisie (formulaire, clavier, ligne de commande) est **toujours** un
texte. Et `texte + nombre` est permis en TypeScript : cela colle les deux. `"5" + 3`
donne `"53"`.

**La correction :** convertissez d'abord avec `Number`, voir
[nombres et calculs](nombres.md).

```typescript
const saisie = "5";
const n = Number(saisie);
console.log(n + 3);
```

```sortie
8
```

### 5. Une case de tableau peut être vide

**Vous pensez :** « `fruits` est un `string[]`, donc `fruits[2]` est un `string`. »

```typescript norun
const fruits = ["pomme", "poire"];
console.log(fruits[2].toUpperCase());
```

`tsc --strict` accepte. Au lancement :

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Pourquoi :** le tableau a deux cases, numérotées 0 et 1. La case 2 n'existe pas :
elle vaut `undefined`. Mais pour TypeScript, une case d'un `string[]` est un `string`.
Il ne compte pas les cases.

```text
fruits :  [ "pomme" | "poire" ]
indice :      0         1         2 → rien (undefined)
```

**La correction :** testez la case avant de vous en servir, ou utilisez `?.`.

```typescript
const fruits = ["pomme", "poire"];
const troisieme = fruits[2];

if (troisieme !== undefined) {
  console.log(troisieme.toUpperCase());
} else {
  console.log("Pas de troisième fruit.");
}
console.log(fruits[0]?.toUpperCase());
```

```sortie
Pas de troisième fruit.
POMME
```

> **Note :** une option du fichier de configuration, `noUncheckedIndexedAccess`, rend
> TypeScript plus prudent : avec elle, `fruits[2]` a le type `string | undefined`, et
> l'oubli devient une erreur (voir [Erreurs fréquentes](#erreurs-frequentes)). Elle
> n'est pas incluse dans `--strict`. Le chapitre [configuration](configuration.md)
> explique comment l'activer.

### 6. || remplace aussi le 0

**Vous pensez :** « `stock || "inconnu"` affiche « inconnu » seulement s'il n'y a pas de
stock indiqué. »

```typescript
interface Article {
  nom: string;
  stock?: number;
}

const articles: Article[] = [
  { nom: "stylo", stock: 12 },
  { nom: "gomme", stock: 0 },
  { nom: "règle" },
];

for (const a of articles) {
  console.log(a.nom, a.stock || "inconnu");
}
```

```sortie
stylo 12
gomme inconnu
règle inconnu
```

**Pourquoi :** `||` remplace toutes les valeurs « fausses » : `undefined`, mais aussi
`0`, `""` et `false`. Or un stock de 0 est une vraie information.

**La correction :** `??` remplace seulement `null` et `undefined`.

```typescript
const stocks: (number | undefined)[] = [12, 0, undefined];

for (const s of stocks) {
  console.log(s ?? "inconnu");
}
```

```sortie
12
0
inconnu
```

### 7. const n'empêche pas de modifier un tableau

**Vous pensez :** « Ma liste est `const`, elle ne peut plus changer. »

```typescript
const courses = ["pain"];
courses.push("lait");
courses[0] = "beurre";
console.log(courses);
```

```sortie
[ 'beurre', 'lait' ]
```

**Pourquoi :** `const` scelle **l'étiquette**, pas le contenu. Vous ne pouvez pas
mettre un autre tableau dans `courses`. Mais le tableau lui-même reste modifiable,
comme une boîte collée au mur dont on peut changer le contenu.

**La correction :** si le contenu ne doit pas changer, dites-le avec `readonly`.

```typescript
const courses: readonly string[] = ["pain", "lait"];
console.log(courses.length);
```

```sortie
2
```

Avec `readonly`, `courses.push(...)` devient une erreur de type.

### 8. sort() trie les nombres comme des mots

**Vous pensez :** « `[10, 9, 1].sort()` donne `[1, 9, 10]`. »

```typescript
const notes = [10, 9, 1, 25];
notes.sort();
console.log(notes);
```

```sortie
[ 1, 10, 25, 9 ]
```

**Pourquoi :** sans consigne, `sort` transforme chaque valeur en texte, puis range
comme dans un dictionnaire. `"10"` vient avant `"9"`, car `"1"` vient avant `"9"`.

**La correction :** donnez une fonction de comparaison.

```typescript
const notes = [10, 9, 1, 25];
notes.sort((a, b) => a - b);
console.log(notes);
```

```sortie
[ 1, 9, 10, 25 ]
```

`a - b` est négatif si `a` est plus petit : `a` passe devant.

### 9. {} veut dire « presque n'importe quoi »

**Vous pensez :** « `{}` est un objet vide, donc seulement des objets. »

```typescript
const a: {} = 42;
const b: {} = "bonjour";
const c: {} = { x: 1 };
console.log(a, b, c);
```

```sortie
42 bonjour { x: 1 }
```

**Pourquoi :** le type `{}` veut dire « une valeur qui n'est ni `null` ni
`undefined` ». Un nombre ou un texte conviennent. Le type `object`, lui, refuse les
valeurs simples :

```typescript norun
const x: object = 42;
```

```text
main.ts:1:7 - error TS2322: Type 'number' is not assignable to type 'object'.

1 const x: object = 42;
        ~


Found 1 error in main.ts:1
```

**La correction :** n'utilisez ni `{}` ni `object` pour décrire vos données. Écrivez
leur vraie forme avec une `interface` (voir [les objets](objets.md)).

### 10. Une faute de frappe qui passe par une variable

**Vous pensez :** « TypeScript repère toujours les propriétés inconnues. »

Écrit directement, l'objet est bien vérifié :

```typescript norun
interface Contact {
  nom: string;
  email?: string;
}
const c: Contact = { nom: "Alice", emial: "alice@exemple.fr" };
```

```text
main.ts:5:36 - error TS2561: Object literal may only specify known properties, but 'emial' does not exist in type 'Contact'. Did you mean to write 'email'?

5 const c: Contact = { nom: "Alice", emial: "alice@exemple.fr" };
                                     ~~~~~


Found 1 error in main.ts:5
```

Mais si l'objet passe d'abord par une variable, la faute se cache :

```typescript
interface Contact {
  nom: string;
  email?: string;
}

const donnees = { nom: "Alice", emial: "alice@exemple.fr" };
const c: Contact = donnees;
console.log(c.email ?? "pas d'adresse");
```

```sortie
pas d'adresse
```

**Pourquoi :** la vérification des propriétés en trop ne s'applique qu'aux objets
écrits **sur place**. Un objet rangé dans une variable peut avoir des propriétés en
plus. Et comme `email` est facultative, son absence ne gêne pas.

**La correction :** annotez l'objet dès sa création : `const donnees: Contact = { ... }`.

### 11. Deux types de même forme sont interchangeables

**Vous pensez :** « `Euros` et `Dollars` sont deux types différents. »

```typescript
type Euros = number;
type Dollars = number;

function payerEnEuros(montant: Euros): string {
  return `Payé : ${montant} €`;
}

const prixAmericain: Dollars = 100;
console.log(payerEnEuros(prixAmericain));
```

```sortie
Payé : 100 €
```

**Pourquoi :** `type Euros = number` ne crée pas un nouveau type. Il donne un **autre
nom** à `number`. Et TypeScript compare les formes, pas les noms : c'est le
[typage](../debuter/glossaire.md#typage) structurel. Deux étiquettes, une même boîte.

**La correction :** donnez des formes différentes, par exemple un objet avec la
monnaie.

```typescript
interface Montant {
  valeur: number;
  monnaie: "EUR" | "USD";
}

function payerEnEuros(m: Montant): string {
  if (m.monnaie !== "EUR") {
    return "Refusé : il faut des euros.";
  }
  return `Payé : ${m.valeur} €`;
}

console.log(payerEnEuros({ valeur: 100, monnaie: "USD" }));
```

```sortie
Refusé : il faut des euros.
```

### 12. Les enum numériques réservent des surprises

Une [énumération](../debuter/glossaire.md#enumeration) (`enum`) donne un nom à
chaque valeur d'une petite liste.

**Vous pensez :** « `Couleur` contient deux couleurs. »

```typescript
enum Couleur {
  Rouge,
  Vert,
}

console.log(Couleur.Rouge);
console.log(Object.keys(Couleur));
```

```sortie
0
[ '0', '1', 'Rouge', 'Vert' ]
```

**Pourquoi :** un `enum` numérique numérote ses valeurs (`Rouge` vaut `0`). Et c'est un
vrai objet JavaScript, rempli **dans les deux sens** : `Couleur.Rouge` vaut `0`, et
`Couleur[0]` vaut `"Rouge"`. Si vous le parcourez, vous trouvez quatre clés.

**La correction :** pour débuter, préférez une union de textes. Elle est simple, lisible
à l'affichage, et ne crée aucun objet caché.

```typescript
type Couleur = "rouge" | "vert";
const couleurs: Couleur[] = ["rouge", "vert"];

for (const c of couleurs) {
  console.log(c);
}
```

```sortie
rouge
vert
```

Vous verrez ces unions dans [le chapitre sur les unions](unions.md).

## Erreurs fréquentes

### Une case de tableau peut-être vide, avec noUncheckedIndexedAccess

Si votre projet active `noUncheckedIndexedAccess` (piège 5), ce code est refusé :

```typescript norun
const fruits = ["pomme", "poire"];
console.log(fruits[2].toUpperCase());
```

```text
main.ts:2:13 - error TS2532: Object is possibly 'undefined'.

2 console.log(fruits[2].toUpperCase());
              ~~~~~~~~~


Found 1 error in main.ts:2
```

« L'objet est peut-être `undefined`. » C'est une bonne nouvelle : le bug est trouvé
avant le lancement. Testez la case (`if (f !== undefined)`) ou écrivez
`fruits[2]?.toUpperCase()`.

### Comparer un nombre et un texte

```typescript norun
const age = 18;
const saisie = "18";
if (age === saisie) {
  console.log("même âge");
}
```

```text
main.ts:3:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

3 if (age === saisie) {
      ~~~~~~~~~~~~~~


Found 1 error in main.ts:3
```

« Cette comparaison semble involontaire : un `number` et un `string` n'ont rien en
commun. » Elle serait toujours fausse. Convertissez : `age === Number(saisie)`.

### Utiliser as entre deux types sans rapport

```typescript norun
const n = "42" as number;
```

```text
main.ts:1:11 - error TS2352: Conversion of type 'string' to type 'number' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.

1 const n = "42" as number;
            ~~~~~~~~~~~~~~


Found 1 error in main.ts:1
```

Même `as` a des limites : un texte ne devient pas un nombre en changeant d'étiquette.
Le message propose de passer par `unknown`… ne le faites pas ! Convertissez vraiment :
`const n = Number("42");`.

## Testez-vous

**Question 1 :** pourquoi `const p: Produit = JSON.parse(texte)` est-il risqué ?

> **Réponse**
>
> `JSON.parse` renvoie `any` : TypeScript ne vérifie pas le contenu. Si le texte ne
> correspond pas à `Produit`, l'erreur n'apparaîtra qu'à l'exécution. Il faut
> vérifier les données avant de leur faire confiance.

**Question 2 :** qu'affiche ce code ?

```typescript norun
const quantite = 0;
console.log(quantite || 10, quantite ?? 10);
```

> **Réponse**
>
> `10 0`. `||` remplace le `0`, car `0` est une valeur « fausse ». `??` ne remplace que
> `null` et `undefined` : il garde le `0`.

**Question 3 :** que faut-il écrire pour trier `[3, 20, 100]` dans l'ordre des
nombres ?

> **Réponse**
>
> `liste.sort((a, b) => a - b)`. Sans fonction, `sort` compare des textes et donne
> `[100, 20, 3]`.

## En résumé

- `any` et `as` font taire le contrôleur : préférez `unknown` et de vrais objets
  complets.
- Les types disparaissent à l'exécution : vérifiez les données venues de `JSON.parse`,
  d'un fichier ou du clavier (qui donne toujours du texte).
- Une case de tableau peut être `undefined` ; `noUncheckedIndexedAccess` le rappelle.
- `??` garde le `0`, `||` non ; `const` n'empêche pas de modifier un tableau ;
  `sort()` sans fonction trie comme des mots.
- `{}` accepte presque tout, `object` refuse les valeurs simples : décrivez plutôt la
  vraie forme.
- Le typage est structurel : deux types de même forme sont interchangeables ; les
  `enum` numériques ont des clés cachées.

Suite : [exercices pour bien démarrer](exercices-faciles.md).
