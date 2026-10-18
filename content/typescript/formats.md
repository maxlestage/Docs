# JSON et validation des données

Cette page explique pourquoi les données venues de l'extérieur (JSON, CSV) sont dangereuses pour TypeScript, et comment les vérifier avant de leur faire confiance.

> **Objectifs :**
>
> - comprendre pourquoi `JSON.parse` renvoie `any`, et le danger que cela pose ;
> - lire du JSON dans une variable `unknown` et écrire un garde de type à la main ;
> - découvrir la bibliothèque zod, qui valide à partir d'un schéma ;
> - transformer des objets en texte avec `JSON.stringify` ;
> - lire et écrire un petit fichier CSV, bien typé.

## JSON en deux mots

Le [JSON](../debuter/glossaire.md#json) est un format de texte pour échanger des données :
objets entre accolades, tableaux entre crochets, textes entre guillemets doubles.

```json
{ "nom": "Ada", "age": 36, "langages": ["TypeScript", "Ruby"] }
```

Deux fonctions font le passage entre texte et objets (on parle de
[sérialisation](../debuter/glossaire.md#serialisation)) :

- `JSON.parse(texte)` : du texte vers un objet ;
- `JSON.stringify(objet)` : d'un objet vers du texte.

```typescript
const texte = '{ "nom": "Ada", "age": 36 }';
const personne = JSON.parse(texte);
console.log(personne.nom, personne.age + 1);
```

```sortie
Ada 37
```

## Le piège : JSON.parse renvoie any

Le type de retour de `JSON.parse` est `any`. Normal : TypeScript ne peut pas deviner,
au moment de compiler, ce que contiendra un texte lu à l'exécution.

Or `any` **éteint** toutes les vérifications. TypeScript vous laisse écrire n'importe
quoi, même une faute de frappe :

```typescript
const personne = JSON.parse('{ "nom": "Ada", "age": 36 }');
console.log(personne.nmo);
console.log(personne.age.toUpperCase === undefined);
```

```sortie
undefined
true
```

Aucune erreur de compilation, alors que `nmo` n'existe pas. Pire : vous pouvez ranger ce
`any` dans une variable typée, et TypeScript vous croit sur parole.

```typescript
interface Personne {
  nom: string;
  age: number;
}

const p: Personne = JSON.parse('{ "nom": "Ada" }');
console.log(p.age);
```

```sortie
undefined
```

TypeScript affirme que `p.age` est un `number`… et c'est `undefined`. Le type ment, et le
bug apparaîtra plus loin, loin de sa cause. C'est comme accepter un colis sans l'ouvrir
parce que l'étiquette dit « vaisselle ».

> **À retenir :** des données venues de l'extérieur (fichier, réseau, utilisateur) ne
> sont jamais sûres. Il faut les **vérifier**, pas seulement les annoter.

## unknown et le garde de type

La solution : ranger le résultat dans une variable de type `unknown`. Contrairement à
`any`, `unknown` interdit tout usage tant que vous n'avez pas vérifié la valeur.

Pour vérifier, on écrit un **garde de type** (*type guard*) : une fonction qui renvoie
un booléen, avec le type de retour spécial `v is Personne`. Il signifie : « si je
renvoie `true`, alors `v` est une `Personne` ».

```typescript
interface Personne {
  nom: string;
  age: number;
}

function estPersonne(v: unknown): v is Personne {
  return (
    typeof v === "object" &&
    v !== null &&
    "nom" in v &&
    typeof v.nom === "string" &&
    "age" in v &&
    typeof v.age === "number"
  );
}

const textes = ['{ "nom": "Ada", "age": 36 }', '{ "nom": "Linus" }', "[1, 2]"];

for (const texte of textes) {
  const donnees: unknown = JSON.parse(texte);
  if (estPersonne(donnees)) {
    console.log(`${donnees.nom} a ${donnees.age} ans`);
  } else {
    console.log(`Données refusées : ${texte}`);
  }
}
```

```sortie
Ada a 36 ans
Données refusées : { "nom": "Linus" }
Données refusées : [1, 2]
```

La vérification se lit étape par étape :

1. `typeof v === "object"` : c'est un objet (ou un tableau, ou `null`) ;
2. `v !== null` : on écarte `null`, car `typeof null` vaut aussi `"object"` ;
3. `"nom" in v` : l'objet a une propriété `nom` ;
4. `typeof v.nom === "string"` : et elle contient un texte.

Après le `if`, TypeScript sait que `donnees` est une `Personne` : l'autocomplétion
fonctionne et les fautes de frappe sont signalées.

### Valider un tableau

Pour un tableau, vérifiez d'abord `Array.isArray`, puis chaque élément avec `every` :

```typescript
interface Personne {
  nom: string;
  age: number;
}

function estPersonne(v: unknown): v is Personne {
  return (
    typeof v === "object" && v !== null &&
    "nom" in v && typeof v.nom === "string" &&
    "age" in v && typeof v.age === "number"
  );
}

function estListePersonnes(v: unknown): v is Personne[] {
  return Array.isArray(v) && v.every(estPersonne);
}

const donnees: unknown = JSON.parse('[{ "nom": "Ada", "age": 36 }, { "nom": "Grace", "age": 45 }]');
if (estListePersonnes(donnees)) {
  const total = donnees.reduce((somme, p) => somme + p.age, 0);
  console.log(`Âge moyen : ${total / donnees.length}`);
}
```

```sortie
Âge moyen : 40.5
```

### Un texte qui n'est pas du JSON

Si le texte est mal formé, `JSON.parse` lève une `SyntaxError`. Attrapez-la :

```typescript
function lireJson(texte: string): unknown {
  try {
    return JSON.parse(texte);
  } catch (e) {
    if (e instanceof SyntaxError) {
      console.log("JSON invalide");
      return undefined;
    }
    throw e;
  }
}

console.log(lireJson("[1, 2, 3]"));
console.log(lireJson("{nom: 'Ada'}"));
```

```sortie
[ 1, 2, 3 ]
JSON invalide
undefined
```

## Valider avec zod

Écrire les gardes de type à la main devient long pour de gros objets. La
[bibliothèque](../debuter/glossaire.md#bibliotheque) **zod** (à installer avec
`npm install zod`) propose une autre approche : vous décrivez la forme attendue dans un
**schéma**, et zod fait les vérifications pour vous. Bonus : il en déduit le type
TypeScript avec `z.infer`, sans rien écrire deux fois.

```typescript norun
import { z } from "zod";

const Utilisateur = z.object({
  nom: z.string().min(1),
  age: z.number().int().min(0),
  email: z.string().email(),
});

type Utilisateur = z.infer<typeof Utilisateur>;

const textes = [
  '{"nom": "Ada", "age": 36, "email": "ada@exemple.fr"}',
  '{"nom": "", "age": -2, "email": "pas-un-email"}',
];

for (const texte of textes) {
  const resultat = Utilisateur.safeParse(JSON.parse(texte));
  if (resultat.success) {
    const u: Utilisateur = resultat.data;
    console.log(`Valide : ${u.nom}, ${u.age} ans`);
  } else {
    for (const probleme of resultat.error.issues) {
      console.log(`Invalide : ${probleme.path.join(".")} : ${probleme.message}`);
    }
  }
}
```

```text
Valide : Ada, 36 ans
Invalide : nom : String must contain at least 1 character(s)
Invalide : age : Number must be greater than or equal to 0
Invalide : email : Invalid email
```

- `safeParse` ne lève pas d'erreur : il renvoie un objet avec `success` à `true` (et
  les données dans `data`) ou à `false` (et les problèmes dans `error`).
- `parse`, lui, lève une erreur si les données sont invalides.
- Les messages précisent **quel** champ pose problème : idéal pour répondre à un
  utilisateur.

> **Note :** cette sortie vient de zod 3. D'autres bibliothèques suivent la même idée :
> valibot, ArkType… Le principe reste : un schéma, une validation, un type déduit.

## JSON.stringify : écrire du JSON

`JSON.stringify` transforme une valeur en texte JSON. Le troisième argument indente le
résultat pour le rendre lisible :

```typescript
interface Produit {
  nom: string;
  prix: number;
  tags: string[];
}

const produit: Produit = { nom: "Lampe", prix: 29.9, tags: ["maison", "bureau"] };

console.log(JSON.stringify(produit));
console.log(JSON.stringify(produit, null, 2));
```

```sortie
{"nom":"Lampe","prix":29.9,"tags":["maison","bureau"]}
{
  "nom": "Lampe",
  "prix": 29.9,
  "tags": [
    "maison",
    "bureau"
  ]
}
```

### Ce que JSON ne sait pas représenter

JSON ne connaît que les textes, nombres, booléens, `null`, tableaux et objets simples.
Le reste est transformé ou perdu :

```typescript
const donnees = {
  nom: "test",
  date: new Date(Date.UTC(2024, 0, 15)),
  inconnu: undefined,
  scores: new Map([["Ada", 10]]),
};

console.log(JSON.stringify(donnees));
```

```sortie
{"nom":"test","date":"2024-01-15T00:00:00.000Z","scores":{}}
```

- une `Date` devient un texte : en relisant, vous obtiendrez un `string`, pas une `Date` ;
- une propriété `undefined` disparaît ;
- une `Map` devient un objet vide. Convertissez-la avant : `Object.fromEntries(map)`.

## Lire un CSV à la main

Le format **CSV** (*Comma-Separated Values*, « valeurs séparées par des virgules ») range
un tableau dans un texte : une ligne par enregistrement, les colonnes séparées par des
virgules, et souvent une première ligne d'en-têtes.

Pour un CSV simple (sans virgule à l'intérieur des valeurs), `split` suffit. On convertit
chaque ligne en objet typé, en vérifiant les nombres :

```typescript
interface Eleve {
  nom: string;
  note: number;
}

const csv = `nom,note
Ada,17
Linus,12
Grace,abc
Alan,15
`;

function lireEleves(texte: string): Eleve[] {
  const eleves: Eleve[] = [];
  const lignes = texte.trim().split("\n").slice(1);
  for (const ligne of lignes) {
    const [nom, noteTexte] = ligne.split(",");
    const note = Number(noteTexte);
    if (nom === undefined || Number.isNaN(note)) {
      console.log(`Ligne ignorée : ${ligne}`);
      continue;
    }
    eleves.push({ nom, note });
  }
  return eleves;
}

const eleves = lireEleves(csv);
for (const e of eleves) {
  console.log(`${e.nom.padEnd(6)} ${e.note}`);
}
```

```sortie
Ligne ignorée : Grace,abc
Ada    17
Linus  12
Alan   15
```

- `slice(1)` saute la ligne d'en-têtes ;
- `Number("abc")` donne `NaN` (« pas un nombre ») : on ignore la ligne.

> **Attention :** un vrai CSV peut contenir `"Dupont, Jean"`, avec une virgule entre
> guillemets. Pour ces fichiers, utilisez une bibliothèque comme `csv-parse`.

### Écrire un CSV

Dans l'autre sens, on assemble chaque ligne avec `join(",")` :

```typescript
interface Eleve {
  nom: string;
  note: number;
}

const eleves: Eleve[] = [
  { nom: "Ada", note: 17 },
  { nom: "Alan", note: 15 },
];

const lignes = ["nom,note", ...eleves.map((e) => [e.nom, e.note].join(","))];
console.log(lignes.join("\n"));
```

```sortie
nom,note
Ada,17
Alan,15
```

## Erreurs fréquentes

### Faire confiance au JSON reçu

```typescript norun
const texte = '{"nom": "Ada"}';
const personne = JSON.parse(texte);
console.log(personne.adresse.ville);
```

```text
main.js:3
console.log(personne.adresse.ville);
                             ^

TypeError: Cannot read properties of undefined (reading 'ville')
```

Avec `any`, TypeScript ne voit rien : l'erreur arrive à l'exécution. Rangez le résultat
dans `unknown` et validez sa forme avant de l'utiliser.

### Un garde de type qui renvoie boolean

```typescript norun
function estPersonne(v: unknown): boolean {
  return typeof v === "object" && v !== null && "nom" in v;
}
const v: unknown = JSON.parse('{"nom": "Ada"}');
if (estPersonne(v)) {
  console.log(v.nom);
}
```

```text
main.ts(6,15): error TS18046: 'v' is of type 'unknown'.
```

Un simple `boolean` ne dit rien à TypeScript sur `v`. Le type de retour doit être
`v is Personne` pour que la vérification serve dans le `if`.

### Parser un texte qui n'est pas du JSON

```typescript norun
const texte = "{nom: 'Ada'}";
const donnees: unknown = JSON.parse(texte);
console.log(donnees);
```

```text
<anonymous_script>:1
{nom: 'Ada'}
 ^

SyntaxError: Expected property name or '}' in JSON at position 1 (line 1 column 2)
    at JSON.parse (<anonymous>)
```

En JSON, les noms de propriétés et les textes s'écrivent entre guillemets **doubles** :
`{"nom": "Ada"}`. Pour un texte qui vient de l'extérieur, entourez `JSON.parse` d'un
`try`/`catch`.

## Testez-vous

**Question 1 :** pourquoi préférer `unknown` à `any` pour le résultat de `JSON.parse` ?

> **Réponse**
>
> `any` laisse tout passer sans vérification. `unknown` interdit d'utiliser la valeur
> tant qu'on n'a pas vérifié sa forme : les erreurs sont attrapées à la compilation.

**Question 2 :** que donne `JSON.stringify({ a: 1, b: undefined })` ?

> **Réponse**
>
> `{"a":1}` : la propriété qui vaut `undefined` disparaît.

**Question 3 :** écrivez un garde de type `estNombres(v: unknown): v is number[]`.

> **Réponse**
>
> ```typescript
> function estNombres(v: unknown): v is number[] {
>   return Array.isArray(v) && v.every((x) => typeof x === "number");
> }
>
> console.log(estNombres(JSON.parse("[1, 2, 3]")));
> console.log(estNombres(JSON.parse('[1, "deux"]')));
> ```
>
> ```sortie
> true
> false
> ```

## En résumé

- `JSON.parse` renvoie `any` : le type peut mentir sur des données venues de l'extérieur.
- Rangez le résultat dans `unknown`, puis vérifiez-le avec un garde de type `v is T`.
- zod décrit la forme attendue dans un schéma, valide, et déduit le type avec `z.infer`.
- `JSON.stringify(valeur, null, 2)` écrit du JSON lisible ; `Date`, `Map` et `undefined`
  ne survivent pas tels quels.
- Un CSV simple se lit avec `split` ; convertissez et vérifiez chaque colonne.

Suite : [TypeScript pour le web](web.md). Comparez avec
[les formats en Ruby](../ruby/formats.md).
