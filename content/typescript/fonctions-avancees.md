# Aller plus loin avec les fonctions

Cette page montre comment une fonction TypeScript reçoit un nombre variable d'arguments, comment écrire le type d'une fonction, la passer à une autre fonction, et ce que sont les surcharges.

> **Objectifs :**
>
> - vous saurez écrire le type d'une fonction : `(x: number) => number` ;
> - vous saurez donner un nom à un type de fonction avec `type` ;
> - vous saurez passer une fonction en argument (un *callback*) et la typer.

Ce chapitre suit [les fonctions](fonctions.md) : vous y retrouverez les fonctions fléchées, qui servent ici de valeurs que l'on range et que l'on passe à d'autres fonctions.

## Un nombre variable d'arguments

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Ligne par ligne :

- `somme(1, 2, 3, 4)` range les quatre arguments dans le tableau `nombres` :
  `[1, 2, 3, 4]` ;
- `total` part de 0, puis la boucle lui ajoute chaque nombre : 1, 3, 6, 10 ;
- `return total` renvoie 10 ;
- `somme()` reçoit un tableau vide : la boucle ne tourne pas, et la fonction renvoie 0.

Chaque argument est vérifié : `somme(1, "2")` serait refusé.

## Le type d'une fonction

Une prise électrique n'accepte qu'une forme de fiche. Le type d'une fonction joue le
même rôle : il dit à l'avance quelle sorte de fonction une variable ou un paramètre
accepte.

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

Ligne par ligne :

- la première ligne crée la variable `operation`, encore vide, et dit ce qu'elle
  pourra contenir : une fonction qui prend deux nombres et renvoie un nombre ;
- on y range d'abord une fonction qui additionne : `operation(6, 3)` donne 9 ;
- on la remplace ensuite par une fonction qui multiplie : le même appel donne 18.

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

Ligne par ligne :

- `repeter` reçoit deux choses : un nombre (`fois`) et une fonction (`action`) ;
- à l'appel, `fois` vaut 3 et `action` est la fonction fléchée écrite entre
  parenthèses ;
- la boucle donne à `i` les valeurs 1, 2 puis 3, et appelle `action(i)` à chaque tour ;
- chaque appel range `i` dans `tour` et affiche la phrase.

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

**À vous :** écrivez une fonction `appliquer` qui prend un nombre et une fonction de
type `(n: number) => number`, et renvoie le résultat de cette fonction sur le nombre.
Affichez `appliquer(5, (n) => n + 100)`.

> **Solution**
>
> ```typescript
> function appliquer(x: number, f: (n: number) => number): number {
>   return f(x);
> }
>
> console.log(appliquer(5, (n) => n + 100));
> ```
>
> ```sortie
> 105
> ```

## Les surcharges, en bref

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

### Placer le paramètre du reste avant un autre

```typescript norun
function inviter(...prenoms: string[], salle: string): void {
  console.log(`${prenoms.join(", ")} : salle ${salle}`);
}
inviter("Alice", "Bob", "B12");
```

```text
main.ts:1:18 - error TS1014: A rest parameter must be last in a parameter list.

1 function inviter(...prenoms: string[], salle: string): void {
                   ~~~

main.ts:4:25 - error TS2554: Expected 2 arguments, but got 3.

4 inviter("Alice", "Bob", "B12");
                          ~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

« Un paramètre du reste doit être le dernier de la liste. » Il prend tous les arguments
restants : il n'en resterait aucun pour `salle`. La deuxième erreur vient de la
première. Placez-le en dernier : `function inviter(salle: string, ...prenoms: string[])`,
puis appelez `inviter("B12", "Alice", "Bob")`.

### Passer un callback du mauvais type

```typescript norun
function repeter(fois: number, action: (tour: number) => void): void {
  for (let i = 1; i <= fois; i++) {
    action(i);
  }
}

repeter(3, (tour: string) => {
  console.log(`Tour numéro ${tour}`);
});
```

```text
main.ts:7:12 - error TS2345: Argument of type '(tour: string) => void' is not assignable to parameter of type '(tour: number) => void'.
  Types of parameters 'tour' and 'tour' are incompatible.
    Type 'number' is not assignable to type 'string'.

7 repeter(3, (tour: string) => {
             ~~~~~~~~~~~~~~~~~~~


Found 1 error in main.ts:7
```

`repeter` appelle `action` avec un nombre : une fonction qui attend un texte ne
convient pas (*types of parameters are incompatible*). Retirez le type du paramètre :
avec `(tour) => { ... }`, TypeScript devine tout seul que `tour` est un `number`.

### Appeler une surcharge qui n'existe pas

```typescript norun
function doubler(x: number): number;
function doubler(x: string): string;
function doubler(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(doubler(true));
```

```text
main.ts:7:21 - error TS2769: No overload matches this call.
  Overload 1 of 2, '(x: number): number', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'number'.
  Overload 2 of 2, '(x: string): string', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'string'.

7 console.log(doubler(true));
                      ~~~~


Found 1 error in main.ts:7
```

« Aucune surcharge ne correspond à cet appel. » TypeScript essaie chaque forme
autorisée et dit pourquoi chacune échoue : `true` n'est ni un `number` ni un `string`.
Passez un nombre ou un texte : `doubler(21)`.

## Testez-vous

**Question 1 :** combien d'arguments peut-on donner à `function f(...mots: string[])` ? Que
vaut `mots` lors de l'appel `f()` ?

> **Réponse**
>
> Autant que l'on veut, même aucun : chacun doit être un `string`. Avec `f()`, `mots`
> est un tableau vide, `[]`.

**Question 2 :** que veut dire le type `(texte: string) => boolean` ?

> **Réponse**
>
> C'est le type d'une fonction qui prend un texte et renvoie `true` ou `false`. Par
> exemple : `(texte) => texte.length > 3`.

**Question 3 :** écrivez une fonction `deuxFois` qui reçoit un callback de type `() => void` et
l'appelle deux fois.

> **Réponse**
>
> ```typescript
> function deuxFois(action: () => void): void {
>   action();
>   action();
> }
>
> deuxFois(() => console.log("Coucou"));
> ```
>
> ```sortie
> Coucou
> Coucou
> ```

## En résumé

- `...nom: T[]` reçoit tous les arguments restants dans un tableau ; il se place en
  dernier.
- Le type d'une fonction s'écrit `(x: number) => number` ; `type` lui donne un nom.
- Un callback est une fonction passée en argument ; grâce au type du paramètre, ses
  propres paramètres sont devinés.
- Les surcharges listent plusieurs formes d'appel ; une union suffit souvent.

Suite : [travailler avec du texte](texte.md).
