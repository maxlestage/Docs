# Objets, interfaces et alias de type

Cette page explique comment regrouper des valeurs dans des objets, décrire leur forme avec `interface` et `type`, et comment TypeScript compare les objets selon leur forme.

> **Objectifs :**
>
> - vous saurez créer un objet et lire ses propriétés ;
> - vous saurez décrire la forme d'un objet avec `interface` ou `type` ;
> - vous saurez utiliser les propriétés facultatives (`?`) et en lecture seule
>   (`readonly`) ;
> - vous saurez étendre une interface et écrire une signature d'index ;
> - vous saurez expliquer le typage structurel et la vérification des propriétés en
>   trop.

## Créer un objet

Un [objet](../debuter/glossaire.md#objet) regroupe plusieurs valeurs sous un même nom.
Chaque valeur a une étiquette : sa **propriété**. On l'écrit entre accolades, avec des
paires `nom: valeur`.

```typescript
const livre = {
  titre: "Dune",
  auteur: "Frank Herbert",
  pages: 600,
};

console.log(livre.titre);
console.log(livre["pages"]);
livre.pages = 612;
console.log(livre);
```

```sortie
Dune
600
{ titre: 'Dune', auteur: 'Frank Herbert', pages: 612 }
```

- `livre.titre` lit une propriété avec un point ;
- `livre["pages"]` fait la même chose avec des crochets et un texte ;
- `livre.pages = 612` modifie une propriété.

Un objet est comme une fiche d'inscription : chaque case a une étiquette (`titre`,
`pages`) et un contenu.

TypeScript devine le type de cet objet : `{ titre: string; auteur: string; pages:
number }`. Il connaît donc chaque propriété et son type. `livre.pages = "beaucoup"`
serait refusé, et une faute de frappe comme `livre.titres` aussi.

## Décrire un objet : le type objet

Pour une fonction qui reçoit un objet, il faut écrire sa forme. On la décrit entre
accolades, comme un objet, mais avec des types à la place des valeurs :

```typescript
function presenter(personne: { nom: string; age: number }): string {
  return `${personne.nom}, ${personne.age} ans`;
}

console.log(presenter({ nom: "Alice", age: 30 }));
```

```sortie
Alice, 30 ans
```

Écrire cette forme à chaque fois serait pénible. On lui donne donc un nom, de deux
façons possibles : `interface` ou `type`.

## interface et type

### interface

Une **interface** donne un nom à une forme d'objet. C'est un plan, un modèle de
fiche : elle dit quelles cases doivent exister et ce qu'elles contiennent.

```typescript
interface Personne {
  nom: string;
  age: number;
}

const alice: Personne = { nom: "Alice", age: 30 };
const bob: Personne = { nom: "Bob", age: 25 };

function presenter(p: Personne): string {
  return `${p.nom}, ${p.age} ans`;
}

console.log(presenter(alice));
console.log(presenter(bob));
```

```sortie
Alice, 30 ans
Bob, 25 ans
```

Par convention, le nom d'une interface commence par une majuscule.

### type : l'alias de type

Le mot `type` crée un **alias de type** : un nom pour n'importe quel type. Vous l'avez
déjà vu pour des unions (`type Reponse = "oui" | "non"`). Il marche aussi pour un objet :

```typescript
type Produit = {
  nom: string;
  prix: number;
};

const cafe: Produit = { nom: "Café", prix: 2.5 };
console.log(`${cafe.nom} : ${cafe.prix} €`);
```

```sortie
Café : 2.5 €
```

### Lequel choisir ?

Pour décrire un objet, les deux se valent presque toujours. La différence :

| | `interface` | `type` |
|---|---|---|
| Décrire un objet | oui | oui |
| Décrire une union, un tuple, une fonction | non | oui |
| Étendre avec `extends` | oui | avec `&` |

> **À retenir :** une règle simple pour débuter : `interface` pour les objets, `type`
> pour tout le reste (unions, tuples, fonctions). Surtout, restez cohérent dans un même
> projet.

## Les propriétés facultatives

Un `?` après le nom rend une propriété facultative : l'objet peut l'avoir ou non. Son
type devient `string | undefined`.

```typescript
interface Utilisateur {
  nom: string;
  email?: string;
}

const alice: Utilisateur = { nom: "Alice", email: "alice@exemple.fr" };
const bob: Utilisateur = { nom: "Bob" };

for (const u of [alice, bob]) {
  console.log(`${u.nom} : ${u.email ?? "pas d'adresse"}`);
}
```

```sortie
Alice : alice@exemple.fr
Bob : pas d'adresse
```

Avant d'utiliser `u.email` comme un texte, TypeScript vous oblige à traiter le cas où
elle manque : avec `??`, `?.` ou un `if`.

## Les propriétés en lecture seule

`readonly` devant une propriété interdit de la modifier après la création de l'objet.

```typescript
interface Point {
  readonly x: number;
  readonly y: number;
}

const depart: Point = { x: 0, y: 0 };
const arrivee: Point = { x: depart.x + 3, y: depart.y + 4 };

console.log(arrivee);
```

```sortie
{ x: 3, y: 4 }
```

Pour « déplacer » un point, on en crée un nouveau. Cette vérification n'existe que
dans TypeScript : au moment de l'exécution, l'objet est un objet JavaScript normal.

> **Astuce :** marquez `readonly` les propriétés qui ne doivent jamais changer, comme
> un identifiant. TypeScript surveillera pour vous.

## Les objets imbriqués et les méthodes

Une propriété peut contenir un autre objet, un tableau, ou une fonction. Une fonction
rangée dans un objet s'appelle une [méthode](../debuter/glossaire.md#methode).

```typescript
interface Adresse {
  ville: string;
  codePostal: string;
}

interface Client {
  nom: string;
  adresse: Adresse;
  commandes: number[];
  saluer(): string;
}

const client: Client = {
  nom: "Chloé",
  adresse: { ville: "Nantes", codePostal: "44000" },
  commandes: [25, 40],
  saluer() {
    return `Bonjour, ${this.nom} !`;
  },
};

console.log(client.adresse.ville);
console.log(client.commandes.length);
console.log(client.saluer());
```

```sortie
Nantes
2
Bonjour, Chloé !
```

- `saluer(): string` dans l'interface : une méthode sans paramètre qui renvoie un texte ;
- dans l'objet, `this` désigne l'objet lui-même : `this.nom` vaut `"Chloé"`.

## Étendre une interface

`extends` (« étend ») crée une interface qui reprend toutes les propriétés d'une autre,
et en ajoute. C'est comme une fiche « Élève » qui reprend la fiche « Personne » et
ajoute la classe.

```typescript
interface Personne {
  nom: string;
  age: number;
}

interface Eleve extends Personne {
  classe: string;
}

const lea: Eleve = { nom: "Léa", age: 15, classe: "2nde B" };

function presenter(p: Personne): string {
  return `${p.nom} (${p.age} ans)`;
}

console.log(presenter(lea));
console.log(lea.classe);
```

```sortie
Léa (15 ans)
2nde B
```

Un `Eleve` a tout ce qu'a une `Personne` : on peut donc le passer à `presenter`.

Avec `type`, on obtient le même résultat avec `&` (« et ») :

```typescript
type Animal = { nom: string };
type Chien = Animal & { race: string };

const rex: Chien = { nom: "Rex", race: "berger" };
console.log(rex.nom, rex.race);
```

```sortie
Rex berger
```

## Les signatures d'index

Parfois, on ne connaît pas les noms des propriétés à l'avance : des notes par matière,
des prix par article… Une **signature d'index** décrit « n'importe quelle clé de ce
type, avec une valeur de ce type ».

```typescript
interface Notes {
  [matiere: string]: number;
}

const notes: Notes = { maths: 15, francais: 12 };
notes.anglais = 14;

for (const [matiere, note] of Object.entries(notes)) {
  console.log(`${matiere} : ${note}`);
}
```

```sortie
maths : 15
francais : 12
anglais : 14
```

`[matiere: string]: number` se lit : « pour toute clé texte, une valeur nombre ». Le mot
`matiere` n'est qu'un nom pour la documentation. C'est la même chose que
`Record<string, number>`, vu dans [les tableaux](collections.md#record-un-objet-cle-valeur).

> **Attention :** TypeScript annonce `notes.histoire` comme un `number`, mais la clé
> n'existe pas : la valeur est `undefined`. Avec une signature d'index, vérifiez
> qu'une clé existe (`"histoire" in notes`) avant de vous fier à sa valeur.

## Le typage structurel : seule la forme compte

Voici une idée importante, propre à TypeScript. Pour savoir si un objet convient,
TypeScript ne regarde pas **son nom**, mais **sa forme** : ses propriétés et leurs
types. On parle de **typage structurel**.

Prenez une analogie : une prise électrique. Elle accepte n'importe quelle fiche qui a
la bonne forme, peu importe la marque de l'appareil. De même, une fonction qui attend
un `Point` accepte n'importe quel objet qui a un `x` et un `y` numériques.

```typescript
interface Point {
  x: number;
  y: number;
}

function distanceOrigine(p: Point): number {
  return Math.sqrt(p.x ** 2 + p.y ** 2);
}

const pixel = { x: 3, y: 4, couleur: "rouge" };

console.log(distanceOrigine(pixel));
console.log(distanceOrigine({ x: 6, y: 8 }));
```

```sortie
5
10
```

`pixel` n'a jamais été déclaré comme un `Point`. Il a même une propriété en plus,
`couleur`. Mais il possède `x` et `y`, des nombres : il a la forme d'un `Point`, il est
accepté. La fiche entre dans la prise.

> **Note :** d'autres langages, comme Java, demandent de déclarer explicitement qu'un
> objet « est un » `Point`. TypeScript non : il suffit d'avoir la bonne forme. On dit
> parfois « si ça marche comme un canard et cancane comme un canard, c'est un canard ».

## La vérification des propriétés en trop

Une exception à la règle précédente : quand vous écrivez un objet **directement** à
l'endroit où un type est attendu, TypeScript refuse les propriétés en trop.

```typescript norun
interface Livre {
  titre: string;
  pages: number;
}
const livre: Livre = { titre: "Dune", pages: 600, auteur: "Herbert" };
```

```text
main.ts:5:51 - error TS2353: Object literal may only specify known properties, and 'auteur' does not exist in type 'Livre'.

5 const livre: Livre = { titre: "Dune", pages: 600, auteur: "Herbert" };
                                                    ~~~~~~


Found 1 error in main.ts:5
```

« Un objet écrit ainsi (*object literal*) ne peut avoir que des propriétés connues, et
`auteur` n'existe pas dans `Livre`. »

Pourquoi cette différence avec `pixel` ? Un objet écrit sur place ne sert qu'à cet
endroit : une propriété inconnue y est presque toujours une faute de frappe
(`titres` au lieu de `titre`) ou une propriété oubliée dans l'interface. TypeScript préfère vous prévenir. Un objet déjà rangé dans une variable,
lui, peut servir ailleurs avec ses autres propriétés : il est accepté.

Les deux corrections possibles :

- retirez la propriété en trop ;
- ou ajoutez-la à l'interface, si elle a du sens : `auteur?: string;`.

```typescript
interface Livre {
  titre: string;
  pages: number;
  auteur?: string;
}

const livre: Livre = { titre: "Dune", pages: 600, auteur: "Herbert" };
console.log(`${livre.titre}, de ${livre.auteur ?? "auteur inconnu"}`);
```

```sortie
Dune, de Herbert
```

## Erreurs fréquentes

### Oublier une propriété obligatoire

```typescript norun
interface Livre {
  titre: string;
  pages: number;
}
const livre: Livre = { titre: "Dune" };
```

```text
main.ts:5:7 - error TS2741: Property 'pages' is missing in type '{ titre: string; }' but required in type 'Livre'.

5 const livre: Livre = { titre: "Dune" };
        ~~~~~

  main.ts:3:3
    3   pages: number;
        ~~~~~
    'pages' is declared here.


Found 1 error in main.ts:5
```

« La propriété `pages` manque, mais `Livre` l'exige. » Ajoutez-la
(`{ titre: "Dune", pages: 600 }`), ou rendez-la facultative dans l'interface
(`pages?: number`).

### Se tromper de nom de propriété

```typescript norun
interface Livre {
  titre: string;
  pages: number;
}
const livre: Livre = { titre: "Dune", pages: 600 };
console.log(livre.titres);
```

```text
main.ts:6:19 - error TS2551: Property 'titres' does not exist on type 'Livre'. Did you mean 'titre'?

6 console.log(livre.titres);
                    ~~~~~~

  main.ts:2:3
    2   titre: string;
        ~~~~~
    'titre' is declared here.


Found 1 error in main.ts:6
```

TypeScript connaît toutes les propriétés de `Livre` et propose la bonne :
`livre.titre`. En JavaScript seul, `livre.titres` aurait donné `undefined` sans un mot.

### Utiliser une propriété facultative sans vérifier

```typescript norun
interface Utilisateur {
  nom: string;
  email?: string;
}
const u: Utilisateur = { nom: "Alice" };
console.log(u.email.toLowerCase());
```

```text
main.ts:6:13 - error TS18048: 'u.email' is possibly 'undefined'.

6 console.log(u.email.toLowerCase());
              ~~~~~~~


Found 1 error in main.ts:6
```

`email` est facultative : elle peut manquer. Écrivez `u.email?.toLowerCase()`, ou
testez `if (u.email !== undefined)` avant.

### Modifier une propriété readonly

```typescript norun
interface Point {
  readonly x: number;
  readonly y: number;
}
const p: Point = { x: 1, y: 2 };
p.x = 5;
```

```text
main.ts:6:3 - error TS2540: Cannot assign to 'x' because it is a read-only property.

6 p.x = 5;
    ~


Found 1 error in main.ts:6
```

`x` est en lecture seule. Créez un nouvel objet : `const q: Point = { ...p, x: 5 };`.
Les `...p` recopient toutes les propriétés de `p`, puis `x: 5` remplace `x`.

## Testez-vous

**Question 1 :** écrivez une interface `Film` avec un `titre` (texte), une `annee`
(nombre) et une `note` facultative (nombre), puis un film sans note.

> **Réponse**
>
> ```typescript
> interface Film {
>   titre: string;
>   annee: number;
>   note?: number;
> }
>
> const film: Film = { titre: "Le Voyage de Chihiro", annee: 2001 };
> console.log(film.note ?? "pas encore noté");
> ```
>
> ```sortie
> pas encore noté
> ```

**Question 2 :** une fonction attend un `{ nom: string }`. Peut-on lui passer une
variable `const chat = { nom: "Félix", age: 3 }` ?

> **Réponse**
>
> Oui. Grâce au typage structurel, seule la forme compte : `chat` a bien un `nom`
> texte. La propriété `age` en plus ne gêne pas, car l'objet est passé par une
> variable et non écrit directement dans l'appel.

**Question 3 :** quelle est la différence entre `interface Eleve extends Personne`
et `type Eleve = Personne & { classe: string }` ?

> **Réponse**
>
> Le résultat est le même : un `Eleve` a toutes les propriétés de `Personne`, plus
> `classe`. `extends` s'utilise avec `interface`, `&` avec `type`.

## En résumé

- Un objet regroupe des valeurs nommées : `{ nom: "Alice", age: 30 }` ; on lit une
  propriété avec `objet.nom`.
- `interface` et `type` donnent un nom à la forme d'un objet ; `interface` pour les
  objets, `type` pour le reste est une bonne règle de départ.
- `?` rend une propriété facultative (à vérifier avant usage), `readonly` l'empêche de
  changer.
- `extends` (ou `&`) étend une forme ; `[cle: string]: number` accepte n'importe
  quelle clé.
- Le typage est structurel : un objet convient s'il a la bonne forme, même avec des
  propriétés en plus, sauf s'il est écrit directement à cet endroit.

Suite : [Suivre un programme pas à pas](pas-a-pas.md)
