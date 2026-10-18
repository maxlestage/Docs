# Les génériques

Cette page explique comment écrire une fonction ou une classe qui marche avec n'importe quel type, sans perdre la précision des types.

> **Objectifs :**
>
> - vous saurez écrire une fonction générique avec `<T>` ;
> - vous saurez laisser TypeScript deviner le type, ou le donner vous-même ;
> - vous saurez limiter un générique avec `extends` ;
> - vous saurez écrire des interfaces et des classes génériques comme `Boite<T>` ;
> - vous saurez relier deux paramètres avec `K extends keyof T`.

## Le problème

Imaginez une fonction qui renvoie le premier élément d'un tableau. Pour les nombres,
vous écririez :

```typescript
function premierNombre(liste: number[]): number | undefined {
  return liste[0];
}

console.log(premierNombre([10, 20, 30]));
```

```sortie
10
```

Et pour les textes ? Il faudrait copier la fonction en changeant `number` en `string`.
Puis encore pour les booléens… Avec `any`, une seule fonction suffirait, mais
TypeScript ne saurait plus rien du résultat : il ne pourrait plus vous protéger.

Les [génériques](../debuter/glossaire.md#generique) résolvent ce problème : une seule
fonction, et des types toujours précis.

## Une fonction générique

Un générique est un **paramètre de type**. Il s'écrit entre chevrons `<T>`, juste
après le nom de la fonction. `T` est un nom de type « à remplir plus tard », comme
un paramètre ordinaire est une valeur à remplir plus tard.

```typescript
function premier<T>(liste: T[]): T | undefined {
  return liste[0];
}

const n = premier([10, 20, 30]);
const t = premier(["pomme", "poire"]);

console.log(n);
console.log(t?.toUpperCase());
```

```sortie
10
POMME
```

Lisez la signature ainsi : « pour un type `T` quelconque, `premier` reçoit un tableau
de `T` et renvoie un `T` (ou `undefined`) ». C'est comme une étiquette vierge sur un
bocal : vous écrivez dessus « confiture » ou « cornichons » au moment de le remplir.

> **Note :** `T` est une convention (pour *Type*). Vous pouvez choisir un autre nom,
> comme `Element`. Avec plusieurs paramètres, on utilise souvent `T`, `U`, `K`, `V`.

## L'inférence

Dans l'exemple, vous n'avez jamais écrit quel était `T`. TypeScript l'a **inféré**
(deviné) à partir des arguments : `[10, 20, 30]` est un `number[]`, donc `T` vaut
`number`, donc `n` est un `number | undefined`.

Vous pouvez aussi donner `T` vous-même, entre chevrons à l'appel :

```typescript
function enveloppe<T>(valeur: T): T[] {
  return [valeur];
}

const a = enveloppe(5);
const b = enveloppe<string | number>("cinq");
b.push(5);

console.log(a, b);
```

```sortie
[ 5 ] [ 'cinq', 5 ]
```

Sans `<string | number>`, `b` serait un `string[]`, et `b.push(5)` serait refusé.
En général, laissez TypeScript deviner ; précisez seulement quand il devine trop
étroit.

## Plusieurs paramètres de type

Une fonction peut avoir plusieurs génériques. Ici, `paire` assemble deux valeurs de
types différents :

```typescript
function paire<A, B>(a: A, b: B): [A, B] {
  return [a, b];
}

function transformer<T, U>(liste: T[], f: (x: T) => U): U[] {
  const resultat: U[] = [];
  for (const x of liste) {
    resultat.push(f(x));
  }
  return resultat;
}

console.log(paire("âge", 12));
console.log(transformer(["a", "bb", "ccc"], (mot) => mot.length));
```

```sortie
[ 'âge', 12 ]
[ 1, 2, 3 ]
```

Dans `transformer`, TypeScript déduit `T = string` depuis le tableau, puis
`U = number` depuis ce que renvoie la fonction. C'est exactement ainsi qu'est typée la
méthode `map` des tableaux.

## Les contraintes avec extends

Parfois, `T` ne peut pas être **n'importe quoi**. Pour lire `.length`, il faut que la
valeur ait une longueur. `T extends Contrainte` veut dire : « `T` peut être n'importe
quel type, **pourvu qu'il** respecte la contrainte ».

```typescript
function plusLong<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

console.log(plusLong("chat", "éléphant"));
console.log(plusLong([1, 2, 3], [4]));
```

```sortie
éléphant
[ 1, 2, 3 ]
```

Les textes et les tableaux ont tous un `length` : ils sont acceptés. Un nombre serait
refusé. Et le résultat garde le type exact : `string` dans le premier appel,
`number[]` dans le second.

Une contrainte peut aussi être une interface :

```typescript
interface AvecNom {
  nom: string;
}

function trierParNom<T extends AvecNom>(liste: T[]): T[] {
  return [...liste].sort((a, b) => a.nom.localeCompare(b.nom));
}

const eleves = [
  { nom: "Zoé", note: 15 },
  { nom: "Adam", note: 12 },
];

for (const e of trierParNom(eleves)) {
  console.log(`${e.nom} : ${e.note}`);
}
```

```sortie
Adam : 12
Zoé : 15
```

Remarquez que `e.note` est permis : la fonction renvoie des `T`, c'est-à-dire des
objets avec `nom` **et** `note`, pas seulement des `AvecNom`.

## Les interfaces et types génériques

Un type peut aussi avoir un paramètre. `Boite<T>` décrit une boîte qui contient une
valeur de type `T` :

```typescript
interface Boite<T> {
  contenu: T;
  etiquette: string;
}

const boiteDeNombre: Boite<number> = { contenu: 42, etiquette: "réponse" };
const boiteDeListe: Boite<string[]> = { contenu: ["a", "b"], etiquette: "lettres" };

console.log(boiteDeNombre.contenu + 1);
console.log(boiteDeListe.contenu.join(", "));
```

```sortie
43
a, b
```

Vous en utilisez déjà sans le savoir : `Array<number>` est l'autre écriture de
`number[]`, et `Promise<string>` est une promesse qui donnera un texte (voir
[l'asynchrone](asynchrone.md)).

## Les classes génériques

Une classe peut avoir un paramètre de type. Voici une **pile** : comme une pile
d'assiettes, on pose sur le dessus et on reprend sur le dessus.

```typescript
class Pile<T> {
  private elements: T[] = [];

  empiler(x: T): void {
    this.elements.push(x);
  }

  depiler(): T | undefined {
    return this.elements.pop();
  }

  get taille(): number {
    return this.elements.length;
  }
}

const assiettes = new Pile<string>();
assiettes.empiler("bleue");
assiettes.empiler("rouge");
console.log(assiettes.depiler());
console.log(assiettes.taille);

const nombres = new Pile<number>();
nombres.empiler(7);
console.log(nombres.depiler()! * 2);
```

```sortie
rouge
1
14
```

`Pile<string>` n'accepte que des textes, `Pile<number>` que des nombres. Une seule
classe, et chaque pile est protégée contre les mélanges.

> **Note :** le `!` après `depiler()` dit à TypeScript « je sais que ce n'est pas
> `undefined` ». Utilisez-le seulement quand vous en êtes certain, comme ici.

## Les types par défaut

Un paramètre de type peut avoir une valeur par défaut, comme un paramètre de
fonction : `<T = string>`. Si personne ne précise `T`, il vaut `string`.

```typescript
interface Reponse<T = string> {
  code: number;
  donnees: T;
}

const texte: Reponse = { code: 200, donnees: "OK" };
const liste: Reponse<number[]> = { code: 200, donnees: [1, 2, 3] };

console.log(texte.donnees.toLowerCase());
console.log(liste.donnees.length);
```

```sortie
ok
3
```

## Relier deux paramètres avec keyof

`keyof T` est l'union des noms de propriétés de `T`. Pour
`{ nom: string; age: number }`, c'est `"nom" | "age"`. Avec `K extends keyof T`, vous
dites : « `K` doit être une des clés de `T` ».

```typescript
function getProp<T, K extends keyof T>(objet: T, cle: K): T[K] {
  return objet[cle];
}

const chat = { nom: "Minou", age: 3, gourmand: true };

const nom = getProp(chat, "nom");
const age = getProp(chat, "age");

console.log(nom.toUpperCase());
console.log(age + 1);
```

```sortie
MINOU
4
```

Le type de retour `T[K]` se lit « le type de la propriété `K` dans `T` ». Pour
`"nom"`, c'est `string` ; pour `"age"`, c'est `number`. Et une clé qui n'existe pas,
comme `"couleur"`, est refusée avant même l'exécution.

## Erreurs fréquentes

### Utiliser une propriété sans contrainte

```typescript norun
function longueur<T>(x: T): number {
  return x.length;
}
```

```text
main.ts(2,12): error TS2339: Property 'length' does not exist on type 'T'.
```

`T` peut être n'importe quoi, même un nombre, qui n'a pas de `length`. Ajoutez une
contrainte : `function longueur<T extends { length: number }>(x: T)`.

### Demander une clé qui n'existe pas

```typescript norun
function getProp<T, K extends keyof T>(objet: T, cle: K): T[K] {
  return objet[cle];
}
const chat = { nom: "Minou", age: 3 };
console.log(getProp(chat, "couleur"));
```

```text
main.ts(5,27): error TS2345: Argument of type '"couleur"' is not assignable to parameter of type '"nom" | "age"'.
```

Le message donne la liste des clés permises : `"nom" | "age"`. Corrigez la faute de
frappe, ou ajoutez la propriété à l'objet.

### Mélanger les types dans une classe générique

```typescript norun
class Pile<T> {
  private elements: T[] = [];
  empiler(x: T) {
    this.elements.push(x);
  }
}
const pile = new Pile<number>();
pile.empiler("dix");
```

```text
main.ts(8,14): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.
```

La pile a été créée pour des nombres : `T` vaut `number` partout. Empilez `10`, ou
créez une `Pile<string>` pour les textes.

## Testez-vous

**Question 1 :** quel est le type de `x` ?

```typescript norun
function dernier<T>(liste: T[]): T | undefined {
  return liste[liste.length - 1];
}
const x = dernier([true, false]);
```

> **Réponse**
>
> `boolean | undefined`. TypeScript infère `T = boolean` à partir de
> `[true, false]`.

**Question 2 :** que veut dire `<T extends { id: number }>` ?

> **Réponse**
>
> `T` peut être n'importe quel type, à condition d'avoir une propriété `id` de type
> `number`. Dans la fonction, vous pouvez donc lire `x.id`, et le type exact de `T`
> est conservé.

**Question 3 :** écrivez une fonction générique `repeter<T>(valeur: T, n: number)`
qui renvoie un tableau contenant `n` fois la valeur.

> **Réponse**
>
> ```typescript
> function repeter<T>(valeur: T, n: number): T[] {
>   const resultat: T[] = [];
>   for (let i = 0; i < n; i++) {
>     resultat.push(valeur);
>   }
>   return resultat;
> }
>
> console.log(repeter("ha", 3));
> console.log(repeter(0, 2));
> ```
>
> ```sortie
> [ 'ha', 'ha', 'ha' ]
> [ 0, 0 ]
> ```

## En résumé

- Un générique `<T>` est un paramètre de type : une seule fonction pour tous les types.
- TypeScript infère `T` depuis les arguments ; vous pouvez aussi l'écrire à l'appel.
- `T extends Contrainte` limite `T` aux types qui ont ce qu'il faut.
- Les interfaces et les classes peuvent être génériques : `Boite<T>`, `Pile<T>`.
- `<T = string>` donne un type par défaut.
- `K extends keyof T` et `T[K]` relient une clé et le type de sa valeur.

Suite : [types littéraux, enums et as const](litteraux.md)
