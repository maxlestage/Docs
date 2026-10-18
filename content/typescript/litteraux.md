# Types littéraux, enums et as const

Cette page explique comment limiter une valeur à une petite liste de choix précis, avec les types littéraux, les enums et `as const`.

> **Objectifs :**
>
> - vous saurez écrire un type littéral et une union de littéraux ;
> - vous saurez pourquoi `let` et `const` ne donnent pas le même type ;
> - vous saurez créer un `enum` et le comparer à une union ;
> - vous saurez figer un objet avec `as const` et en tirer un type ;
> - vous saurez vérifier une valeur sans perdre sa précision avec `satisfies`.

## Un type littéral

Jusqu'ici, un type désignait une grande famille de valeurs : `string` accepte tous les
textes. Un **type littéral** n'accepte qu'**une seule** valeur. Le type `"rouge"`
accepte le texte `"rouge"`, et rien d'autre.

Seul, il ne sert pas à grand-chose. Mais en [union](unions.md), il devient une liste de
choix, comme un menu déroulant :

```typescript
type Couleur = "rouge" | "vert" | "bleu";

function peindre(c: Couleur) {
  console.log(`Je peins en ${c}.`);
}

peindre("vert");
peindre("bleu");
```

```sortie
Je peins en vert.
Je peins en bleu.
```

Écrire `peindre("rose")` est refusé : `"rose"` n'est pas dans le menu. Les fautes de
frappe (`"rogue"`) sont détectées de la même façon. Et votre éditeur propose les trois
choix à la saisie.

Les nombres et les booléens ont aussi leurs littéraux :

```typescript
type De = 1 | 2 | 3 | 4 | 5 | 6;
type Oui = true;

const lancer: De = 4;
const accepte: Oui = true;
console.log(lancer, accepte);
```

```sortie
4 true
```

## let, const et l'élargissement

TypeScript ne donne pas le même type à `let` et à `const`.

- `const x = "rouge"` ne changera jamais : son type est le littéral `"rouge"`.
- `let y = "rouge"` pourra changer : TypeScript **élargit** son type à `string`.

```typescript
type Couleur = "rouge" | "vert" | "bleu";

function peindre(c: Couleur) {
  console.log(`Je peins en ${c}.`);
}

const fixe = "rouge";
peindre(fixe);

let variable: Couleur = "rouge";
variable = "bleu";
peindre(variable);
```

```sortie
Je peins en rouge.
Je peins en bleu.
```

Avec `let`, annoncez le type vous-même (`let variable: Couleur`). Sans cela, la
variable serait un `string`, et `peindre(variable)` serait refusé (voir les
[erreurs fréquentes](#erreurs-frequentes)).

## Les unions de littéraux dans un objet

Les littéraux brillent dans les objets. Un statut de commande a quelques valeurs
possibles, pas une infinité :

```typescript
type Statut = "en attente" | "expédiée" | "livrée";

interface Commande {
  numero: number;
  statut: Statut;
}

function message(c: Commande): string {
  switch (c.statut) {
    case "en attente":
      return `Commande ${c.numero} : nous la préparons.`;
    case "expédiée":
      return `Commande ${c.numero} : elle est en route.`;
    case "livrée":
      return `Commande ${c.numero} : bon appétit !`;
  }
}

console.log(message({ numero: 12, statut: "expédiée" }));
```

```sortie
Commande 12 : elle est en route.
```

C'est le même principe que le champ `kind` des
[unions discriminées](unions.md#les-unions-discriminees).

## Les enums

Un `enum` (une [énumération](../debuter/glossaire.md#enumeration)) donne un nom à
chaque choix d'une liste. Il existe dans beaucoup de langages, et TypeScript l'a aussi.

```typescript
enum Direction {
  Haut,
  Bas,
  Gauche,
  Droite,
}

function bouger(d: Direction): string {
  if (d === Direction.Haut) return "On monte";
  if (d === Direction.Bas) return "On descend";
  return "On tourne";
}

console.log(bouger(Direction.Haut));
console.log(Direction.Gauche);
console.log(Direction[Direction.Gauche]);
```

```sortie
On monte
2
Gauche
```

Par défaut, les membres valent `0`, `1`, `2`, `3`… dans l'ordre. `Direction.Gauche`
vaut donc `2`. Ce n'est pas très parlant à l'affichage. Vous pouvez donner des valeurs
texte :

```typescript
enum Niveau {
  Facile = "facile",
  Moyen = "moyen",
  Difficile = "difficile",
}

const choix = Niveau.Moyen;
console.log(`Niveau choisi : ${choix}`);
console.log(Object.values(Niveau));
```

```sortie
Niveau choisi : moyen
[ 'facile', 'moyen', 'difficile' ]
```

> **Note :** un `enum` n'est pas seulement un type : il crée un vrai objet JavaScript
> au moment de la [compilation](../debuter/glossaire.md#compilation). Ici, c'est sans
> souci, car on compile avec `tsc`. Mais certains outils qui « effacent » simplement
> les types (comme l'exécution directe de fichiers `.ts` par Node) refusent les enums.

### Enum ou union de littéraux ?

Beaucoup de projets TypeScript préfèrent les unions de littéraux :

| | Union de littéraux | `enum` |
|---|---|---|
| Écriture d'une valeur | `"facile"` | `Niveau.Facile` |
| Existe à l'exécution | non (rien n'est généré) | oui (un objet) |
| Liste de toutes les valeurs | non, sauf avec `as const` | oui, avec `Object.values` |
| Marche partout | oui | il faut compiler |

Pour débuter, choisissez l'union de littéraux. Si vous avez besoin de la liste des
valeurs à l'exécution, utilisez la technique suivante.

## Figer un objet avec as const

Un objet ordinaire peut changer : TypeScript élargit donc ses valeurs (`"facile"`
devient `string`). `as const` fige l'objet entier :

- toutes les propriétés deviennent `readonly` ;
- toutes les valeurs gardent leur type littéral.

```typescript
const NIVEAUX = {
  FACILE: "facile",
  MOYEN: "moyen",
  DIFFICILE: "difficile",
} as const;

console.log(NIVEAUX.MOYEN);
console.log(Object.values(NIVEAUX).join(" < "));
```

```sortie
moyen
facile < moyen < difficile
```

Sans `as const`, `NIVEAUX.MOYEN` aurait le type `string`. Avec, son type est
exactement `"moyen"`. Ça marche aussi pour un tableau :
`const JOURS = ["lun", "mar"] as const` a le type `readonly ["lun", "mar"]`.

### Tirer un type d'un objet figé

Voici la recette qui remplace un `enum`. On écrit les valeurs **une seule fois** dans
l'objet, puis on en tire le type :

```typescript
const NIVEAUX = {
  FACILE: "facile",
  MOYEN: "moyen",
  DIFFICILE: "difficile",
} as const;

type Niveau = (typeof NIVEAUX)[keyof typeof NIVEAUX];

function points(n: Niveau): number {
  switch (n) {
    case "facile":
      return 1;
    case "moyen":
      return 2;
    case "difficile":
      return 3;
  }
}

console.log(points(NIVEAUX.DIFFICILE));
console.log(points("moyen"));
```

```sortie
3
2
```

Lisons `(typeof NIVEAUX)[keyof typeof NIVEAUX]` de l'intérieur :

1. `typeof NIVEAUX` : le type de l'objet,
   `{ readonly FACILE: "facile"; readonly MOYEN: "moyen"; … }` ;
2. `keyof typeof NIVEAUX` : ses clés, `"FACILE" | "MOYEN" | "DIFFICILE"` ;
3. `(typeof NIVEAUX)[…]` : le type des valeurs pour ces clés,
   `"facile" | "moyen" | "difficile"`.

C'est long à écrire, mais vous l'écrivez une fois. Ces opérateurs sont détaillés dans
[les types avancés](types-avances.md).

Pour un tableau figé, la recette est plus courte :

```typescript
const JOURS = ["lun", "mar", "mer"] as const;
type Jour = (typeof JOURS)[number];

const j: Jour = "mar";
console.log(JOURS.indexOf(j));
```

```sortie
1
```

`[number]` veut dire « le type d'un élément quelconque du tableau ».

## Vérifier sans élargir avec satisfies

Quand vous annotez une variable (`const x: Type = …`), TypeScript vérifie la valeur,
mais il **oublie** ensuite les détails : il ne retient que `Type`. L'opérateur
`satisfies` (« satisfait ») vérifie la valeur **et** garde son type précis.

```typescript
type Couleur = "rouge" | "vert" | "bleu";

const theme = {
  fond: "bleu",
  texte: "rouge",
} satisfies Record<string, Couleur>;

const accent: Couleur = theme.fond;
console.log(accent, theme.texte);
```

```sortie
bleu rouge
```

`Record<string, Couleur>` veut dire « un objet dont les valeurs sont des `Couleur` ».
Grâce à `satisfies` :

- une faute comme `fond: "bleuu"` serait détectée ;
- `theme.fond` garde un type précis, et TypeScript sait que `theme.texte` existe.

Avec l'annotation `const theme: Record<string, Couleur>`, TypeScript accepterait
`theme.titre` (n'importe quelle clé texte), qui vaudrait `undefined` à l'exécution.

> **Astuce :** combinez les deux : `{ … } as const satisfies Type` fige l'objet et
> vérifie sa forme.

## Erreurs fréquentes

### Passer une variable let à une union de littéraux

```typescript norun
type Couleur = "rouge" | "vert" | "bleu";
function peindre(c: Couleur) {
  console.log(`Je peins en ${c}`);
}
let choix = "rouge";
peindre(choix);
```

```text
main.ts(6,9): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Couleur'.
```

`let` élargit le type à `string`. Annoncez le type : `let choix: Couleur = "rouge";`,
ou utilisez `const` si la valeur ne change pas.

### Une valeur hors du menu

```typescript norun
type Couleur = "rouge" | "vert" | "bleu";
const c: Couleur = "rose";
```

```text
main.ts(2,7): error TS2322: Type '"rose"' is not assignable to type 'Couleur'.
```

`"rose"` n'est pas dans la liste. Choisissez une valeur permise, ou ajoutez `"rose"`
au type `Couleur`.

### Modifier un objet figé

```typescript norun
const CONFIG = { port: 8080, hote: "localhost" } as const;
CONFIG.port = 3000;
```

```text
main.ts(2,8): error TS2540: Cannot assign to 'port' because it is a read-only property.
```

`as const` rend tout en lecture seule. Si la valeur doit changer, retirez `as const`
ou créez un nouvel objet : `const config2 = { ...CONFIG, port: 3000 };`.

### Donner un texte à la place d'un enum

```typescript norun
enum Direction {
  Haut,
  Bas,
}
const d: Direction = "Haut";
```

```text
main.ts(5,7): error TS2322: Type '"Haut"' is not assignable to type 'Direction'.
```

Une valeur d'enum s'écrit avec le nom de l'enum : `Direction.Haut`.

## Testez-vous

**Question 1 :** quel est le type de `a` ? Et celui de `b` ?

```typescript norun
const a = "chat";
let b = "chat";
```

> **Réponse**
>
> `a` a le type littéral `"chat"`, car une constante ne change pas. `b` a le type
> `string`, car `let` peut recevoir un autre texte plus tard.

**Question 2 :** que fait `as const` sur un objet ?

> **Réponse**
>
> Il rend toutes ses propriétés `readonly` et garde le type littéral de chaque valeur
> (`"facile"` au lieu de `string`). On peut alors en tirer une union avec
> `(typeof OBJET)[keyof typeof OBJET]`.

**Question 3 :** à partir de `const TAILLES = ["S", "M", "L"] as const;`, écrivez un
type `Taille` et une fonction qui affiche une taille.

> **Réponse**
>
> ```typescript
> const TAILLES = ["S", "M", "L"] as const;
> type Taille = (typeof TAILLES)[number];
>
> function afficher(t: Taille) {
>   console.log(`Taille ${t}`);
> }
>
> afficher("M");
> ```
>
> ```sortie
> Taille M
> ```

## En résumé

- Un type littéral n'accepte qu'une valeur ; une union de littéraux forme un menu de
  choix.
- `const` garde le type littéral, `let` l'élargit : annotez vos `let`.
- Un `enum` nomme des choix et existe à l'exécution ; il demande une compilation.
- `as const` fige un objet ; `(typeof X)[keyof typeof X]` en tire l'union des valeurs.
- `satisfies` vérifie une valeur sans perdre son type précis.

Suite : [les types avancés](types-avances.md)
