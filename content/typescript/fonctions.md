# Les fonctions

Cette page explique comment écrire des fonctions en TypeScript, typer leurs paramètres et leur valeur de retour, et passer une fonction à une autre.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec des paramètres et un type de retour ;
> - vous saurez rendre un paramètre facultatif ou lui donner une valeur par défaut ;
> - vous saurez écrire une fonction fléchée `(x) => ...` ;
> - vous saurez écrire le type d'une fonction : `(x: number) => number` ;
> - vous saurez passer une fonction en argument (un *callback*) et la typer.

## Définir et appeler une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez.

```typescript
function saluer(): void {
  console.log("Bonjour !");
}

saluer();
saluer();
```

```sortie
Bonjour !
Bonjour !
```

- `function` annonce une fonction, `saluer` est son nom ;
- les parenthèses `()` contiennent ses paramètres (ici, aucun) ;
- `: void` dit que la fonction ne renvoie rien d'utile (*void* : « vide ») ;
- le bloc `{ ... }` est le corps de la fonction.

Écrire `function saluer` ne lance rien : cela enregistre la recette. Écrire `saluer()`
**appelle** la fonction, c'est-à-dire l'exécute. Les parenthèses sont obligatoires.

## Les paramètres et leur type

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la fonction
reçoit. Ce sont des cases vides, remplies au moment de l'appel par les
[arguments](../debuter/glossaire.md#argument). En TypeScript, chaque paramètre a un
type.

```typescript
function presenter(prenom: string, age: number): void {
  console.log(`${prenom} a ${age} ans.`);
}

presenter("Alice", 30);
presenter("Bob", 25);
```

```sortie
Alice a 30 ans.
Bob a 25 ans.
```

Ici, `prenom` et `age` sont les paramètres ; `"Alice"` et `30` sont les arguments.

Le type d'un paramètre est un contrat : « donnez-moi un nombre ». TypeScript vérifie
**chaque appel**. `presenter("Alice", "trente")` est refusé avant même de lancer le
programme.

> **À retenir :** TypeScript ne devine pas le type d'un paramètre. Écrivez-le toujours,
> sinon `tsc --strict` affiche une erreur (voir les
> [erreurs fréquentes](#erreurs-frequentes)).

## Renvoyer une valeur avec return

`return` renvoie une valeur au code qui a appelé la fonction, et arrête la fonction.
Le type de cette valeur s'écrit après les parenthèses.

```typescript
function aire(largeur: number, hauteur: number): number {
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

La valeur renvoyée remplace l'appel : `aire(2, 5) + 1` devient `10 + 1`.

Le type de retour `: number` peut s'omettre : TypeScript le devine à partir du
`return`. L'écrire reste une bonne habitude : si vous vous trompez dans le corps de la
fonction, l'erreur apparaît dans la fonction, pas plus loin chez celui qui l'appelle.

### Afficher ou renvoyer ?

`console.log` **affiche** une valeur à l'écran. `return` la **donne** au code qui a
appelé la fonction, qui peut s'en servir.

```typescript
function doubleAffiche(x: number): void {
  console.log(x * 2);
}

function double(x: number): number {
  return x * 2;
}

doubleAffiche(5);
console.log(double(5) + 1);
```

```sortie
10
11
```

`doubleAffiche(5) + 1` serait refusé : une fonction `void` ne donne aucune valeur
utilisable.

> **Astuce :** préférez les fonctions qui renvoient une valeur. Vous choisirez ensuite
> de l'afficher, de la stocker ou de la réutiliser.

### Sortir plus tôt

Un `return` au milieu de la fonction l'arrête aussitôt :

```typescript
function diviser(a: number, b: number): string {
  if (b === 0) {
    return "division impossible";
  }
  return (a / b).toString();
}

console.log(diviser(10, 4));
console.log(diviser(10, 0));
```

```sortie
2.5
division impossible
```

TypeScript vérifie que **tous les chemins** renvoient une valeur du bon type. S'il
manque un `return`, il le signale.

## Paramètres facultatifs et valeurs par défaut

### Un paramètre facultatif : ?

Un `?` après le nom rend le paramètre facultatif. S'il n'est pas donné, il vaut
`undefined` : son type devient `string | undefined`.

```typescript
function saluer(prenom: string, titre?: string): string {
  if (titre === undefined) {
    return `Bonjour, ${prenom} !`;
  }
  return `Bonjour, ${titre} ${prenom} !`;
}

console.log(saluer("Alice"));
console.log(saluer("Martin", "docteur"));
```

```sortie
Bonjour, Alice !
Bonjour, docteur Martin !
```

TypeScript vous oblige à traiter le cas `undefined` avant d'utiliser `titre` comme un
texte.

### Une valeur par défaut : =

Plus simple : donnez une valeur par défaut. Elle sert si l'argument est absent. Le
type est deviné à partir de cette valeur.

```typescript
function prixTTC(prix: number, taux = 0.2): number {
  return prix * (1 + taux);
}

console.log(prixTTC(100));
console.log(prixTTC(100, 0.055));
```

```sortie
120
105.5
```

> **Attention :** les paramètres facultatifs se placent **après** les obligatoires.
> `function f(a?: number, b: number)` est refusé.

## Un nombre variable d'arguments

Trois points `...` devant le dernier paramètre le transforment en **paramètre du
reste** : il reçoit tous les arguments restants, rangés dans un tableau. Son type est
un tableau, ici `number[]` (« liste de nombres »).

```typescript
function somme(...nombres: number[]): number {
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

Chaque argument est vérifié : `somme(1, "2")` serait refusé.

## Les fonctions fléchées

Une **fonction fléchée** (*arrow function*) est une écriture courte, avec `=>`. On la
range souvent dans une constante.

```typescript
const carre = (x: number): number => x * x;

const presenter = (prenom: string): string => {
  const majuscules = prenom.toUpperCase();
  return `Je suis ${majuscules}.`;
};

console.log(carre(4));
console.log(presenter("léa"));
```

```sortie
16
Je suis LÉA.
```

- Avec une seule expression après `=>`, pas besoin d'accolades ni de `return` : la
  valeur est renvoyée automatiquement.
- Avec plusieurs lignes, on met des accolades et un `return`.

Une fonction sans nom, comme celle-ci, s'appelle aussi une
[lambda](../debuter/glossaire.md#lambda). Les fonctions fléchées sont partout en
TypeScript, surtout comme *callbacks* (plus bas).

## Le type d'une fonction

Une fonction est une valeur, comme un nombre ou un texte : on peut la ranger dans une
variable. Son type s'écrit comme une fonction fléchée : les paramètres, puis `=>`, puis
le type de retour.

```typescript
let operation: (a: number, b: number) => number;

operation = (a, b) => a + b;
console.log(operation(6, 3));

operation = (a, b) => a * b;
console.log(operation(6, 3));
```

```sortie
9
18
```

`(a: number, b: number) => number` se lit : « une fonction qui prend deux nombres et
renvoie un nombre ». Remarquez : dans `(a, b) => a + b`, plus besoin d'écrire les
types des paramètres. TypeScript les connaît déjà grâce au type de `operation`.

Pour réutiliser un type de fonction, donnez-lui un nom avec `type` :

```typescript
type Operation = (a: number, b: number) => number;

const plus: Operation = (a, b) => a + b;
const moins: Operation = (a, b) => a - b;

console.log(plus(10, 4), moins(10, 4));
```

```sortie
14 6
```

## Les callbacks : passer une fonction à une fonction

Un **callback** (« rappel ») est une fonction passée en argument à une autre
fonction, qui l'appellera plus tard. C'est comme laisser votre numéro au boulanger :
il vous rappellera quand le pain sera prêt.

```typescript
function repeter(fois: number, action: (tour: number) => void): void {
  for (let i = 1; i <= fois; i++) {
    action(i);
  }
}

repeter(3, (tour) => {
  console.log(`Tour numéro ${tour}`);
});
```

```sortie
Tour numéro 1
Tour numéro 2
Tour numéro 3
```

Le paramètre `action` a un type de fonction : `(tour: number) => void`. Grâce à lui,
TypeScript sait que `tour` est un `number` dans la fonction fléchée passée à l'appel.

Voici un exemple plus utile : une fonction qui transforme chaque nombre d'une liste.

```typescript
function transformer(nombres: number[], f: (n: number) => number): number[] {
  const resultat: number[] = [];
  for (const n of nombres) {
    resultat.push(f(n));
  }
  return resultat;
}

console.log(transformer([1, 2, 3], (n) => n * 10));
console.log(transformer([1, 2, 3], (n) => n - 1));
```

```sortie
[ 10, 20, 30 ]
[ 0, 1, 2 ]
```

`push` ajoute un élément à la fin d'un tableau. Les tableaux ont déjà une méthode
`map` qui fait exactement cela : vous la verrez dans [les tableaux](collections.md).

## La portée des variables

La [portée](../debuter/glossaire.md#portee) d'une variable est la zone du code où elle
existe. Les paramètres et les variables créées dans une fonction n'existent que dans
cette fonction.

```typescript
const tauxTVA = 0.2;

function prixTTC(prix: number): number {
  const total = prix * (1 + tauxTVA);
  return Math.round(total * 100) / 100;
}

console.log(prixTTC(19.99));
```

```sortie
23.99
```

- La fonction voit `tauxTVA`, créée **en dehors**, au-dessus d'elle.
- Dehors, `total` n'existe pas : `console.log(total)` serait refusé.

> **Astuce :** une fonction est plus facile à comprendre quand elle reçoit tout ce dont
> elle a besoin en paramètres.

## Les surcharges, en bref

Parfois, une fonction accepte plusieurs formes d'appel, avec un type de retour
différent pour chacune. TypeScript permet de les lister avant le corps : ce sont les
**surcharges** (*overloads*).

```typescript
function doubler(x: number): number;
function doubler(x: string): string;
function doubler(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(doubler(21));
console.log(doubler("ha"));
```

```sortie
42
haha
```

Les deux premières lignes sont les formes autorisées ; la troisième est le vrai code.
Grâce à elles, `doubler(21)` est connu comme un `number`, et `doubler("ha")` comme un
`string`. Vous en croiserez dans les bibliothèques ; pour vos débuts, une union
(`number | string`) suffit souvent.

## Erreurs fréquentes

### Oublier le type d'un paramètre

```typescript norun
function double(x) {
  return x * 2;
}
console.log(double(4));
```

```text
main.ts:1:17 - error TS7006: Parameter 'x' implicitly has an 'any' type.

1 function double(x) {
                  ~


Found 1 error in main.ts:1
```

« Le paramètre `x` a implicitement le type `any`. » Sans type, `x` pourrait être
n'importe quoi : le mode strict refuse. Écrivez `function double(x: number)`.

### Oublier un argument

```typescript norun
function aire(largeur: number, hauteur: number): number {
  return largeur * hauteur;
}
console.log(aire(3));
```

```text
main.ts:4:13 - error TS2554: Expected 2 arguments, but got 1.

4 console.log(aire(3));
              ~~~~

  main.ts:1:32
    1 function aire(largeur: number, hauteur: number): number {
                                     ~~~~~~~~~~~~~~~
    An argument for 'hauteur' was not provided.


Found 1 error in main.ts:4
```

La fonction attend 2 arguments (*expected 2*), elle en reçoit 1 (*got 1*). Le message
précise même lequel manque : `hauteur`. Donnez-les tous : `aire(3, 4)`. Ou rendez le
paramètre facultatif avec `?` ou une valeur par défaut.

### Passer un argument du mauvais type

```typescript norun
function aire(largeur: number, hauteur: number): number {
  return largeur * hauteur;
}
console.log(aire(3, "4"));
```

```text
main.ts:4:21 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

4 console.log(aire(3, "4"));
                      ~~~


Found 1 error in main.ts:4
```

`"4"` est un texte, pas un nombre. Écrivez `aire(3, 4)`, ou convertissez le texte avec
`Number("4")`.

### Oublier un return sur un chemin

```typescript norun
function mention(note: number): string {
  if (note >= 10) {
    return "reçu";
  } else if (note < 10) {
    return "refusé";
  }
}
console.log(mention(12));
```

```text
main.ts:1:33 - error TS2366: Function lacks ending return statement and return type does not include 'undefined'.

1 function mention(note: number): string {
                                  ~~~~~~


Found 1 error in main.ts:1
```

« Il manque un `return` à la fin. » TypeScript ne voit pas que les deux tests couvrent
tous les cas. Remplacez `else if (note < 10)` par un simple `else` : tous les chemins
renvoient alors un texte.

## Testez-vous

**Question 1 :** écrivez une fonction fléchée `moyenne` qui prend deux nombres et
renvoie leur moyenne.

> **Réponse**
>
> ```typescript
> const moyenne = (a: number, b: number): number => (a + b) / 2;
>
> console.log(moyenne(3, 4));
> ```
>
> ```sortie
> 3.5
> ```

**Question 2 :** que veut dire le type `(texte: string) => boolean` ?

> **Réponse**
>
> C'est le type d'une fonction qui prend un texte et renvoie `true` ou `false`. Par
> exemple : `(texte) => texte.length > 3`.

**Question 3 :** quelle différence entre `titre?: string` et `titre = "M."` ?

> **Réponse**
>
> Les deux rendent le paramètre facultatif. Avec `?`, il vaut `undefined` quand il est
> absent : il faut tester ce cas avant de l'utiliser. Avec `= "M."`, il vaut `"M."`
> quand il est absent : c'est toujours un `string`.

## En résumé

- Une fonction s'écrit `function nom(param: type): typeRetour { ... }` et s'appelle
  avec `nom(...)`.
- Les paramètres doivent toujours être typés ; le type de retour peut être deviné,
  mais l'écrire aide.
- `?` rend un paramètre facultatif (il vaut alors `undefined`), `= valeur` lui donne
  une valeur par défaut, `...nom: T[]` reçoit tous les arguments restants.
- Une fonction fléchée s'écrit `(x: number) => x * 2` ; `void` veut dire « ne renvoie
  rien ».
- Le type d'une fonction s'écrit `(x: number) => number` : il permet de typer les
  callbacks, dont les paramètres sont alors devinés.

Suite : [travailler avec du texte](texte.md).
