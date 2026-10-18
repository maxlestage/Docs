# Bien écrire du TypeScript

Cette page présente les habitudes des programmeurs TypeScript : laisser l'inférence travailler, typer les frontières, éviter `any`, préférer les unions, protéger les données avec `readonly`, et confier la mise en forme à Prettier et ESLint.

> **Objectifs :**
>
> - savoir quand écrire un type et quand laisser TypeScript le deviner ;
> - remplacer `any` par `unknown` et des vérifications ;
> - préférer une union de littéraux à un `enum` ;
> - protéger les données avec `readonly` et écrire de petits types ;
> - mettre en place Prettier et ESLint (avec typescript-eslint).

## Pourquoi soigner son style ?

Un programme est lu bien plus souvent qu'il n'est écrit. En TypeScript, le style ne
concerne pas seulement la mise en page : il touche aussi **les types**. Des types bien
choisis expliquent le code mieux qu'un commentaire, et `tsc` vérifie qu'ils restent vrais.

Deux idées guident tout ce chapitre :

1. écrire les types **là où ils servent** à quelqu'un, pas partout ;
2. faire en sorte que les types disent **la vérité** : pas de `any`, pas de `as` pour
   faire taire le compilateur.

## Laisser l'inférence travailler

L'**inférence**, c'est la capacité de TypeScript à deviner le type d'une variable à partir
de sa valeur (voir [les premiers pas](premiers-pas.md)). Inutile de répéter ce qu'il sait déjà.

Avant, des annotations partout :

```typescript
const nom: string = "Alice";
const notes: number[] = [12, 15, 16];
const total: number = notes.reduce((a: number, b: number): number => a + b, 0);
console.log(nom, total);
```

```sortie
Alice 43
```

Après, la même chose, plus lisible :

```typescript
const nom = "Alice";
const notes = [12, 15, 16];
const total = notes.reduce((a, b) => a + b, 0);
console.log(nom, total);
```

```sortie
Alice 43
```

Les types sont exactement les mêmes : survolez `total` dans l'éditeur, c'est un `number`.
Annotez une variable seulement quand l'inférence ne suffit pas, par exemple pour un
tableau vide : `const noms: string[] = [];` (sinon TypeScript ne sait pas ce qu'il
contiendra).

## Annoter les frontières des fonctions

Une fonction est une **frontière** : d'un côté, celui qui l'écrit ; de l'autre, celui qui
l'appelle. C'est là que les types servent le plus.

- **Les paramètres** : toujours annotés (avec `--strict`, c'est obligatoire).
- **Le type de retour** : conseillé pour les fonctions exportées ou un peu longues. Il
  sert de contrat : si vous vous trompez dans le corps, l'erreur apparaît **dans** la
  fonction, pas chez celui qui l'appelle.
- **Les variables locales** : laissez l'inférence faire.

```typescript
export function moyenne(notes: readonly number[]): number | undefined {
  if (notes.length === 0) return undefined;
  const total = notes.reduce((a, b) => a + b, 0);
  return total / notes.length;
}

console.log(moyenne([12, 15, 16]));
console.log(moyenne([]));
```

```sortie
14.333333333333334
undefined
```

Le type de retour `number | undefined` prévient tout de suite l'appelant : « je peux ne
rien renvoyer, prévoyez-le ».

> **À retenir :** annotez les entrées et les sorties ; laissez TypeScript deviner
> l'intérieur.

## Éviter any, préférer unknown

`any` veut dire « n'importe quoi, ne vérifie rien ». Une seule valeur `any` désactive
le compilateur partout où elle passe. C'est une porte de sortie, pas une solution.

`unknown` veut aussi dire « je ne sais pas ce que c'est », mais avec une différence
capitale : TypeScript vous **interdit** de l'utiliser avant de l'avoir vérifié.

```typescript
function longueur(valeur: unknown): number {
  if (typeof valeur === "string") {
    return valeur.length; // ici, valeur est une string
  }
  if (Array.isArray(valeur)) {
    return valeur.length; // ici, un tableau
  }
  return 0;
}

console.log(longueur("bonjour"));
console.log(longueur([1, 2, 3]));
console.log(longueur(42));
```

```sortie
7
3
0
```

Utilisez `unknown` pour tout ce qui vient de l'extérieur : `JSON.parse`, un fichier, une
réponse réseau. Les tests `typeof`, `Array.isArray` ou `in` le transforment ensuite en un
vrai type (voir [les unions et le rétrécissement](unions.md)).

## Préférer les unions aux enums

TypeScript propose des `enum`, mais la communauté leur préfère souvent une **union de
littéraux** (voir [les types littéraux](litteraux.md)).

Avec un `enum` :

```typescript
enum Statut {
  Brouillon,
  Publie,
}

const s = Statut.Publie;
console.log(s);
```

```sortie
1
```

Avec une union :

```typescript
type Statut = "brouillon" | "publie";

const s: Statut = "publie";
console.log(s);
```

```sortie
publie
```

Les avantages de l'union :

- la valeur affichée est lisible (`publie` plutôt que `1`) ;
- c'est du JavaScript ordinaire : rien n'est ajouté au code compilé ;
- elle fonctionne avec le simple « effacement des types » de Node (qui refuse les
  `enum`) ;
- l'éditeur complète les valeurs possibles quand vous tapez `"`.

Si vous voulez aussi une liste des valeurs pour boucler dessus, partez d'un tableau
`as const` :

```typescript
const STATUTS = ["brouillon", "publie", "archive"] as const;
type Statut = (typeof STATUTS)[number];

function estStatut(x: string): x is Statut {
  return (STATUTS as readonly string[]).includes(x);
}

console.log(STATUTS.join(", "));
console.log(estStatut("publie"), estStatut("perdu"));
```

```sortie
brouillon, publie, archive
true false
```

## Protéger les données avec readonly

`readonly` interdit de modifier une propriété ou un tableau. Une fonction qui reçoit
un `readonly number[]` promet de ne pas toucher au tableau de l'appelant.

```typescript
interface Point {
  readonly x: number;
  readonly y: number;
}

function deplacer(p: Point, dx: number): Point {
  return { x: p.x + dx, y: p.y }; // un nouveau point, l'ancien est intact
}

const a: Point = { x: 1, y: 2 };
const b = deplacer(a, 10);
console.log(a, b);
```

```sortie
{ x: 1, y: 2 } { x: 11, y: 2 }
```

`readonly` ne coûte rien à l'exécution : c'est une règle vérifiée par `tsc` seulement.
Utilisez-le par défaut pour les paramètres de type tableau, et pour les objets que
personne ne doit modifier.

## Des petits types bien nommés

Un gros type de vingt propriétés est difficile à lire et à réutiliser. Découpez-le en
petits types, puis assemblez-les.

```typescript
interface Adresse {
  ville: string;
  codePostal: string;
}

interface Personne {
  nom: string;
  adresse: Adresse;
}

type Resume = Pick<Personne, "nom">;

const alice: Personne = { nom: "Alice", adresse: { ville: "Lyon", codePostal: "69000" } };
const r: Resume = { nom: alice.nom };
console.log(r, alice.adresse.ville);
```

```sortie
{ nom: 'Alice' } Lyon
```

Les types utilitaires comme `Pick`, `Omit` et `Partial` (voir
[les types avancés](types-avances.md)) dérivent un type d'un autre : quand `Personne`
change, `Resume` suit tout seul.

## Les noms

| Quoi | Style | Exemple |
|------|-------|---------|
| variable, fonction, paramètre | `camelCase` | `prixTotal`, `calculerTva` |
| type, interface, classe | `PascalCase` | `CompteBancaire`, `Statut` |
| constante globale fixe | `UPPER_SNAKE_CASE` | `TAUX_TVA`, `STATUTS` |
| fonction qui renvoie un booléen | commence par `est`, `a`, `peut` | `estValide`, `aDesNotes` |
| fichier | minuscules et `-` | `liste-taches.ts` |
| paramètre générique | une majuscule ou un nom en `PascalCase` | `T`, `Element` |

N'ajoutez pas de `I` devant les interfaces (`IPersonne`) : c'est une ancienne habitude
d'autres langages, que le guide officiel déconseille.

## Avant, après : trois refactorings

Un **refactoring** change la forme du code sans changer ce qu'il fait. En voici trois
classiques.

### Des clauses de garde plutôt que des if imbriqués

Avant :

```typescript
function prixLivraison(poids: number, express: boolean): number {
  let prix = 0;
  if (poids > 0) {
    if (express) {
      prix = 10 + poids * 2;
    } else {
      prix = 5 + poids;
    }
  }
  return prix;
}
console.log(prixLivraison(3, true), prixLivraison(3, false), prixLivraison(0, true));
```

```sortie
16 8 0
```

Après : on traite le cas particulier d'abord, puis on sort.

```typescript
function prixLivraison(poids: number, express: boolean): number {
  if (poids <= 0) return 0;
  if (express) return 10 + poids * 2;
  return 5 + poids;
}
console.log(prixLivraison(3, true), prixLivraison(3, false), prixLivraison(0, true));
```

```sortie
16 8 0
```

### filter et map plutôt qu'une boucle

Avant :

```typescript
const eleves = [
  { nom: "Alice", note: 15 },
  { nom: "Bob", note: 8 },
  { nom: "Chloé", note: 12 },
];
const recus: string[] = [];
for (let i = 0; i < eleves.length; i++) {
  if (eleves[i].note >= 10) {
    recus.push(eleves[i].nom.toUpperCase());
  }
}
console.log(recus);
```

```sortie
[ 'ALICE', 'CHLOÉ' ]
```

Après : chaque étape porte un nom, et `recus` est inféré.

```typescript
const eleves = [
  { nom: "Alice", note: 15 },
  { nom: "Bob", note: 8 },
  { nom: "Chloé", note: 12 },
];
const recus = eleves
  .filter((e) => e.note >= 10)
  .map((e) => e.nom.toUpperCase());
console.log(recus);
```

```sortie
[ 'ALICE', 'CHLOÉ' ]
```

### Une union discriminée plutôt que des options

Avant : des propriétés facultatives qui ne vont pas toujours ensemble. Rien n'empêche
une forme avec `rayon` **et** `cote`, ou sans aucun des deux.

```typescript
interface Forme {
  type: string;
  rayon?: number;
  cote?: number;
}

function aire(f: Forme): number {
  if (f.type === "cercle") return Math.PI * (f.rayon ?? 0) ** 2;
  return (f.cote ?? 0) ** 2;
}
console.log(aire({ type: "carre", cote: 3 }));
```

```sortie
9
```

Après : chaque forme a exactement ses propriétés, et le `switch` est vérifié.

```typescript
type Forme =
  | { type: "cercle"; rayon: number }
  | { type: "carre"; cote: number };

function aire(f: Forme): number {
  switch (f.type) {
    case "cercle":
      return Math.PI * f.rayon ** 2;
    case "carre":
      return f.cote ** 2;
  }
}
console.log(aire({ type: "carre", cote: 3 }));
console.log(aire({ type: "cercle", rayon: 1 }).toFixed(2));
```

```sortie
9
3.14
```

## Prettier pour la mise en forme

**Prettier** met en forme le code automatiquement : indentation, guillemets, espaces,
longueur des lignes. Fini les débats : tout le monde a la même mise en page.

```sh
npm install --save-dev prettier
npx prettier --write src/
```

La plupart des équipes gardent les réglages par défaut (deux espaces, guillemets doubles,
points-virgules). Dans VS Code, l'extension Prettier peut mettre en forme à chaque
enregistrement.

## ESLint avec typescript-eslint

`tsc` vérifie les types ; **ESLint** repère le code suspect : variable inutilisée,
`any` explicite, promesse oubliée… Le projet **typescript-eslint** lui apprend le
TypeScript.

```sh
npm install --save-dev eslint @eslint/js typescript-eslint
```

La configuration se met dans un fichier `eslint.config.mjs` à la racine :

```javascript norun
// eslint.config.mjs
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";

export default tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
);
```

Voici un fichier que `tsc --strict` accepte sans broncher, mais qui cache deux défauts :

```typescript norun
// src/main.ts
const texte = '{"nom": "Alice"}';
const donnees: any = JSON.parse(texte);
const inutile = 42;
console.log(donnees.nom);
```

```sh
npx eslint src/
```

```text
/home/alice/projet/src/main.ts
  3:16  error  Unexpected any. Specify a different type      @typescript-eslint/no-explicit-any
  4:7   error  'inutile' is assigned a value but never used  @typescript-eslint/no-unused-vars

✖ 2 problems (2 errors, 0 warnings)
```

Chaque ligne donne la position, le message et le nom de la règle. Ajoutez des scripts
`"lint"` et `"format"` à votre `package.json` (voir [organiser un projet](projet.md)).

> **Astuce :** Prettier s'occupe de la **forme**, ESLint du **fond**. Utilisez les deux,
> et laissez Prettier gagner sur les questions de mise en page.

## Erreurs fréquentes

### Une variable let trop large pour une union

```typescript norun
type Couleur = "rouge" | "vert" | "bleu";

function peindre(couleur: Couleur): string {
  return `Peint en ${couleur}`;
}

let choix = "rouge";
console.log(peindre(choix));
```

```text
main.ts(8,21): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Couleur'.
```

Avec `let`, TypeScript infère `string` : la variable pourrait changer plus tard. Utilisez
`const choix = "rouge";` (type `"rouge"`), ou annotez : `let choix: Couleur = "rouge";`.

### Utiliser une valeur unknown sans la vérifier

```typescript norun
const donnees: unknown = JSON.parse('"bonjour"');
console.log(donnees.toUpperCase());
```

```text
main.ts(2,13): error TS18046: 'donnees' is of type 'unknown'.
```

C'est voulu : `unknown` oblige à vérifier. Ajoutez un test.

```typescript
const donnees: unknown = JSON.parse('"bonjour"');
if (typeof donnees === "string") {
  console.log(donnees.toUpperCase());
}
```

```sortie
BONJOUR
```

### Modifier un tableau readonly

```typescript norun
const JOURS: readonly string[] = ["lundi", "mardi"];
JOURS.push("mercredi");
```

```text
main.ts(2,7): error TS2339: Property 'push' does not exist on type 'readonly string[]'.
```

Un tableau `readonly` n'a pas de méthodes qui le modifient. Créez un nouveau tableau :
`const semaine = [...JOURS, "mercredi"];`.

## Testez-vous

**Question 1 :** faut-il écrire `const age: number = 30;` ?

> **Réponse**
>
> Non : TypeScript infère déjà le type à partir de la valeur. Écrivez `const age = 30;`
> et réservez les annotations aux paramètres et aux types de retour.

**Question 2 :** quelle est la différence entre `any` et `unknown` ?

> **Réponse**
>
> Les deux acceptent n'importe quelle valeur. Mais `any` désactive toute vérification,
> alors que `unknown` interdit de s'en servir avant de l'avoir vérifiée (`typeof`,
> `Array.isArray`, `in`…).

**Question 3 :** réécrivez `enum Taille { Petit, Grand }` dans le style conseillé.

> **Réponse**
>
> Avec une union de littéraux :
>
> ```typescript
> type Taille = "petit" | "grand";
> const t: Taille = "grand";
> console.log(t);
> ```
>
> ```sortie
> grand
> ```

## En résumé

- Laissez l'inférence deviner les variables locales ; annotez paramètres et retours.
- `unknown` plutôt que `any`, puis vérifiez avec `typeof`, `in` ou une garde de type.
- Une union de littéraux (`"petit" | "grand"`) vaut mieux qu'un `enum`.
- `readonly` protège les données ; de petits types se combinent avec `Pick` et `Omit`.
- `camelCase` pour les valeurs, `PascalCase` pour les types, pas de préfixe `I`.
- Prettier met en forme, ESLint avec typescript-eslint signale le code suspect.

Suite : [organiser un projet](projet.md).
