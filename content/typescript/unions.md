# Unions et rétrécissement

Cette page explique comment dire qu'une valeur peut avoir plusieurs types, et comment TypeScript devine le type exact grâce à vos tests.

> **Objectifs :**
>
> - vous saurez écrire un type union avec `|` ;
> - vous saurez rétrécir une union avec `typeof`, `in`, `instanceof` et `===` ;
> - vous saurez construire une union discriminée avec un champ `kind` ;
> - vous saurez vérifier qu'un `switch` traite tous les cas grâce à `never` ;
> - vous saurez écrire votre propre garde de type `x is T`.

## Un type union

Parfois, une valeur peut être de deux [types](../debuter/glossaire.md#type) différents.
Un identifiant peut être un nombre (`42`) ou un texte (`"A-42"`). Le **type union**
s'écrit avec une barre verticale `|`, qui se lit « ou ».

```typescript
let identifiant: number | string;

identifiant = 42;
console.log(identifiant);

identifiant = "A-42";
console.log(identifiant);
```

```sortie
42
A-42
```

Une union, c'est comme une boîte aux lettres qui accepte les lettres **ou** les colis.
Elle refuse tout le reste : `identifiant = true` serait une erreur.

Vous pouvez donner un nom à une union avec `type` :

```typescript
type Identifiant = number | string;

function afficher(id: Identifiant) {
  console.log(`Identifiant : ${id}`);
}

afficher(7);
afficher("B-7");
```

```sortie
Identifiant : 7
Identifiant : B-7
```

### Ce qu'on peut faire avec une union

Avec une union, TypeScript n'autorise que ce qui marche pour **tous** les types de
l'union. `toString()` existe sur les nombres et sur les textes : c'est permis.
`toUpperCase()` n'existe que sur les textes : c'est refusé (voir les
[erreurs fréquentes](#erreurs-frequentes)).

```typescript
function enTexte(x: number | string): string {
  return x.toString();
}

console.log(enTexte(3.5) + " / " + enTexte("trois"));
```

```sortie
3.5 / trois
```

Pour aller plus loin, il faut d'abord **savoir** lequel des types vous avez.

## Le rétrécissement avec typeof

Quand vous testez le type d'une valeur dans un `if`, TypeScript s'en souvient. Dans le
bloc du `if`, il **rétrécit** l'union : il sait que la valeur a le type testé. C'est
comme un détective qui élimine les suspects un par un.

`typeof x` donne le nom du type sous forme de texte : `"string"`, `"number"`,
`"boolean"`, `"object"`, `"undefined"`, `"function"`…

```typescript
function crier(x: string | number): string {
  if (typeof x === "string") {
    // ici, x est un string
    return x.toUpperCase();
  }
  // ici, il ne reste que number
  return x.toFixed(1);
}

console.log(crier("bonjour"));
console.log(crier(2));
```

```sortie
BONJOUR
2.0
```

Après le `return`, TypeScript sait que le cas `string` est traité. Il ne reste que
`number`. Pas besoin de `else`.

## Le rétrécissement par vérité

Une valeur optionnelle a souvent le type `string | undefined`. Un simple `if (x)`
élimine `undefined` (et aussi `null`, `""`, `0`) : on parle de test de **vérité**.

```typescript
function saluer(prenom?: string) {
  if (prenom) {
    console.log(`Bonjour, ${prenom} !`);
  } else {
    console.log("Bonjour, inconnu !");
  }
}

saluer("Alice");
saluer();
```

```sortie
Bonjour, Alice !
Bonjour, inconnu !
```

> **Attention :** `if (x)` rejette aussi `0` et la chaîne vide `""`. Pour un nombre,
> testez plutôt `x !== undefined`, sinon `0` sera pris pour « pas de valeur ».

## Le rétrécissement par égalité

Comparer avec `===` rétrécit aussi. Si `a === b`, les deux ont forcément un type commun.
C'est très utile avec `null` :

```typescript
function longueur(texte: string | null): number {
  if (texte === null) {
    return 0;
  }
  return texte.length;
}

console.log(longueur("chat"));
console.log(longueur(null));
```

```sortie
4
0
```

## Le rétrécissement avec in

Pour des objets, `typeof` répond toujours `"object"`. L'opérateur `in` teste si un
objet possède une propriété : `"voler" in animal`.

```typescript
type Oiseau = { nom: string; voler(): string };
type Poisson = { nom: string; nager(): string };

function bouger(animal: Oiseau | Poisson): string {
  if ("voler" in animal) {
    return animal.voler();
  }
  return animal.nager();
}

const pie: Oiseau = { nom: "pie", voler: () => "La pie vole." };
const truite: Poisson = { nom: "truite", nager: () => "La truite nage." };

console.log(bouger(pie));
console.log(bouger(truite));
```

```sortie
La pie vole.
La truite nage.
```

## Le rétrécissement avec instanceof

Pour les objets créés avec `new` (les [classes](types.md)), `instanceof` teste la
classe de l'objet. `Date` est une classe fournie par JavaScript.

```typescript
function formater(valeur: Date | string): string {
  if (valeur instanceof Date) {
    return valeur.toISOString().slice(0, 10);
  }
  return valeur;
}

console.log(formater(new Date(Date.UTC(2024, 0, 15))));
console.log(formater("demain"));
```

```sortie
2024-01-15
demain
```

> **Note :** dans `Date.UTC(2024, 0, 15)`, les mois commencent à 0 : `0` veut dire
> janvier.

## Les unions discriminées

Voici l'outil le plus utile de ce chapitre. Chaque type de l'union porte un champ
commun, souvent appelé `kind` (« sorte »), avec une valeur **fixe** et différente. Ce
champ sert d'étiquette : il suffit de le lire pour savoir à quel type on a affaire.

```typescript
type Cercle = { kind: "cercle"; rayon: number };
type Rectangle = { kind: "rectangle"; largeur: number; hauteur: number };
type Forme = Cercle | Rectangle;

function aire(f: Forme): number {
  if (f.kind === "cercle") {
    return Math.PI * f.rayon ** 2;
  }
  return f.largeur * f.hauteur;
}

console.log(aire({ kind: "rectangle", largeur: 3, hauteur: 4 }));
console.log(aire({ kind: "cercle", rayon: 1 }).toFixed(2));
```

```sortie
12
3.14
```

Dans le `if`, TypeScript sait que `f` est un `Cercle` : `f.rayon` est permis. Après,
il ne reste que `Rectangle`. Le type `"cercle"` est un **type littéral** : il n'accepte
qu'une seule valeur (voir [les types littéraux](litteraux.md)).

### Avec un switch

Avec plusieurs cas, un `switch` sur `kind` se lit mieux qu'une suite de `if` :

```typescript
type Evenement =
  | { kind: "clic"; x: number; y: number }
  | { kind: "touche"; touche: string }
  | { kind: "fermeture" };

function decrire(e: Evenement): string {
  switch (e.kind) {
    case "clic":
      return `Clic en (${e.x}, ${e.y})`;
    case "touche":
      return `Touche ${e.touche}`;
    case "fermeture":
      return "Fenêtre fermée";
  }
}

console.log(decrire({ kind: "clic", x: 10, y: 20 }));
console.log(decrire({ kind: "touche", touche: "Entrée" }));
console.log(decrire({ kind: "fermeture" }));
```

```sortie
Clic en (10, 20)
Touche Entrée
Fenêtre fermée
```

## Vérifier tous les cas avec never

Le type `never` veut dire « impossible » : aucune valeur ne peut l'avoir. Il sert de
garde-fou. Dans le `default` d'un `switch`, si tous les cas ont été traités, la valeur
a le type `never`. Si vous oubliez un cas, elle a un autre type, et TypeScript proteste.

```typescript
type Feu = { kind: "vert" } | { kind: "orange" } | { kind: "rouge" };

function consigne(feu: Feu): string {
  switch (feu.kind) {
    case "vert":
      return "Passez";
    case "orange":
      return "Ralentissez";
    case "rouge":
      return "Arrêtez-vous";
    default: {
      const impossible: never = feu;
      return impossible;
    }
  }
}

console.log(consigne({ kind: "orange" }));
```

```sortie
Ralentissez
```

Le jour où vous ajoutez `{ kind: "clignotant" }` au type `Feu`, la ligne
`const impossible: never = feu` devient une erreur. TypeScript vous montre l'endroit
exact à compléter. C'est un détecteur d'oubli gratuit.

## Vos propres gardes de type

Parfois, le test est trop compliqué pour un simple `typeof`. Vous pouvez écrire une
fonction qui fait le test et renvoie un booléen, avec un type de retour spécial :
`x is T` (« x est un T »). C'est une **garde de type**.

```typescript
type Chat = { nom: string; miauler(): string };
type Chien = { nom: string; aboyer(): string };

function estUnChat(animal: Chat | Chien): animal is Chat {
  return "miauler" in animal;
}

const animaux: (Chat | Chien)[] = [
  { nom: "Minou", miauler: () => "Miaou" },
  { nom: "Rex", aboyer: () => "Ouaf" },
];

for (const a of animaux) {
  if (estUnChat(a)) {
    console.log(`${a.nom} : ${a.miauler()}`);
  } else {
    console.log(`${a.nom} : ${a.aboyer()}`);
  }
}
```

```sortie
Minou : Miaou
Rex : Ouaf
```

Une garde de type marche aussi avec `filter` : le tableau obtenu a le bon type.

```typescript
function estUnNombre(x: number | string): x is number {
  return typeof x === "number";
}

const melange = [3, "a", 7, "b"];
const nombres: number[] = melange.filter(estUnNombre);
console.log(nombres);
```

```sortie
[ 3, 7 ]
```

> **Attention :** TypeScript croit votre garde sur parole. Si la fonction renvoie
> `true` à tort, le type sera faux. Gardez le test simple et juste.

## Erreurs fréquentes

### Utiliser une méthode sans rétrécir

```typescript norun
function crier(x: string | number) {
  return x.toUpperCase();
}
```

```text
main.ts(2,12): error TS2339: Property 'toUpperCase' does not exist on type 'string | number'.
  Property 'toUpperCase' does not exist on type 'number'.
```

`toUpperCase` n'existe pas sur les nombres. Testez d'abord le type :
`if (typeof x === "string") return x.toUpperCase();`.

### Oublier que la valeur peut manquer

```typescript norun
function initiale(nom?: string) {
  return nom[0];
}
```

```text
main.ts(2,10): error TS18048: 'nom' is possibly 'undefined'.
```

Un paramètre optionnel a le type `string | undefined`. Traitez le cas manquant :
`if (nom === undefined) return "?";`, ou donnez une valeur par défaut
`(nom = "?")`.

### Lire un champ qui n'existe que dans un cas

```typescript norun
type Cercle = { kind: "cercle"; rayon: number };
type Carre = { kind: "carre"; cote: number };

function decrire(f: Cercle | Carre) {
  return f.rayon;
}
```

```text
main.ts(5,12): error TS2339: Property 'rayon' does not exist on type 'Cercle | Carre'.
  Property 'rayon' does not exist on type 'Carre'.
```

Un carré n'a pas de rayon. Testez l'étiquette d'abord :
`if (f.kind === "cercle") return f.rayon;`.

### Oublier un cas dans le switch

```typescript norun
type Forme =
  | { kind: "cercle"; rayon: number }
  | { kind: "carre"; cote: number }
  | { kind: "triangle"; base: number; hauteur: number };

function aire(f: Forme): number {
  switch (f.kind) {
    case "cercle":
      return Math.PI * f.rayon ** 2;
    case "carre":
      return f.cote ** 2;
    default: {
      const impossible: never = f;
      return impossible;
    }
  }
}
```

```text
main.ts(13,13): error TS2322: Type '{ kind: "triangle"; base: number; hauteur: number; }' is not assignable to type 'never'.
```

Le message nomme le cas oublié : le triangle. Ajoutez
`case "triangle": return (f.base * f.hauteur) / 2;`.

## Testez-vous

**Question 1 :** dans ce code, quel est le type de `x` à l'intérieur du `if` ?

```typescript norun
function f(x: string | number | boolean) {
  if (typeof x === "number") {
    // ?
  }
}
```

> **Réponse**
>
> `number`. Le test `typeof x === "number"` rétrécit l'union au seul type `number`.
> Après le `if`, `x` peut encore être `string | number | boolean`, car le bloc ne se
> termine pas par un `return`.

**Question 2 :** à quoi sert le champ `kind` dans une union discriminée ?

> **Réponse**
>
> C'est une étiquette. Chaque type de l'union a une valeur différente pour `kind`. En
> testant `kind` (avec `if` ou `switch`), TypeScript sait exactement quel type vous
> avez, et donc quels champs sont disponibles.

**Question 3 :** écrivez une garde de type `estTexte` qui dit si une valeur
`unknown` est un `string`.

> **Réponse**
>
> ```typescript
> function estTexte(x: unknown): x is string {
>   return typeof x === "string";
> }
>
> const valeur: unknown = "salut";
> if (estTexte(valeur)) {
>   console.log(valeur.toUpperCase());
> }
> ```
>
> ```sortie
> SALUT
> ```

## En résumé

- Un type union `A | B` accepte une valeur de type `A` ou de type `B`.
- Sur une union, seul ce qui marche pour tous les types est permis.
- `typeof`, `in`, `instanceof`, `===` et le test de vérité rétrécissent l'union.
- Une union discriminée porte une étiquette commune (`kind`) qu'on teste avec `switch`.
- `const impossible: never = x` dans le `default` signale tout cas oublié.
- Une garde de type `x is T` enferme un test dans une fonction réutilisable.

Suite : [les classes](types.md)
