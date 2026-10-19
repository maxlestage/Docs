# Types avancés

Cette page explique comment fabriquer de nouveaux types à partir de types existants, avec les types utilitaires, `keyof`, `typeof` et quelques outils plus puissants.

> **Objectifs :**
>
> - vous saurez utiliser les types utilitaires courants : `Partial`, `Pick`, `Omit`, `Record`… ;
> - vous saurez tirer un type d'une valeur avec `typeof` et des clés avec `keyof` ;
> - vous saurez lire le type d'une propriété avec `Type["cle"]` ;
> - vous saurez lire un type transformé (*mapped type*) et un type conditionnel simple ;
> - vous saurez construire des types de texte avec les *template literal types*.

## Pourquoi fabriquer des types ?

Imaginez un type `Utilisateur`. Pour le formulaire de modification, il vous faut le même
type, mais avec tous les champs facultatifs. Pour l'affichage public, il vous faut le
même type, sans le mot de passe. Recopier trois fois le type serait fragile : le jour
où vous ajoutez un champ, il faudrait penser aux trois copies.

TypeScript permet de **calculer** un type à partir d'un autre, comme une formule de
tableur calcule une cellule à partir d'une autre. Quand l'original change, les types
calculés suivent.

Tout ce chapitre s'appuie sur ce type :

```typescript norun
interface Utilisateur {
  id: number;
  nom: string;
  email: string;
  motDePasse: string;
}
```

## Partial et Required

`Partial<T>` rend **toutes** les propriétés facultatives. C'est idéal pour une mise à
jour, où l'on ne change que certains champs.

```typescript
interface Utilisateur {
  id: number;
  nom: string;
  email: string;
  motDePasse: string;
}

function modifier(u: Utilisateur, changements: Partial<Utilisateur>): Utilisateur {
  return { ...u, ...changements };
}

const alice: Utilisateur = { id: 1, nom: "Alice", email: "a@ex.fr", motDePasse: "x" };
const nouvelle = modifier(alice, { email: "alice@ex.fr" });
console.log(nouvelle.email);
```

```sortie
alice@ex.fr
```

`Required<T>` fait l'inverse : toutes les propriétés deviennent obligatoires.

```typescript
interface Options {
  couleur?: string;
  taille?: number;
}

const parDefaut: Required<Options> = { couleur: "noir", taille: 12 };

function regler(o: Options): Required<Options> {
  return { ...parDefaut, ...o };
}

console.log(regler({ taille: 16 }));
```

```sortie
{ couleur: 'noir', taille: 16 }
```

## Readonly

`Readonly<T>` interdit de modifier les propriétés. Utile pour une configuration qui ne
doit pas bouger.

```typescript
interface Config {
  port: number;
  hote: string;
}

const config: Readonly<Config> = { port: 8080, hote: "localhost" };
console.log(`${config.hote}:${config.port}`);
```

```sortie
localhost:8080
```

Écrire `config.port = 3000` serait refusé. Pour les tableaux, il existe
`ReadonlyArray<T>`, qu'on écrit aussi `readonly T[]`.

## Pick et Omit

`Pick<T, Cles>` garde seulement certaines propriétés ; `Omit<T, Cles>` en retire
certaines. Les clés s'écrivent comme une union de textes.

```typescript
interface Utilisateur {
  id: number;
  nom: string;
  email: string;
  motDePasse: string;
}

type Resume = Pick<Utilisateur, "id" | "nom">;
type Public = Omit<Utilisateur, "motDePasse">;

function resumer(u: Utilisateur): Resume {
  return { id: u.id, nom: u.nom };
}

function rendrePublic(u: Utilisateur): Public {
  const { motDePasse, ...reste } = u;
  return reste;
}

const bob: Utilisateur = { id: 2, nom: "Bob", email: "b@ex.fr", motDePasse: "secret" };
console.log(resumer(bob));
console.log(rendrePublic(bob));
```

```sortie
{ id: 2, nom: 'Bob' }
{ id: 2, nom: 'Bob', email: 'b@ex.fr' }
```

> **Astuce :** choisissez celui qui demande la liste la plus courte. Pour garder 2
> champs sur 10, `Pick` ; pour en retirer 1 sur 10, `Omit`.

## Record

`Record<Cles, Valeur>` décrit un objet dont les clés sont connues et dont toutes les
valeurs ont le même type. Avec une union de littéraux comme clés, TypeScript exige
**chaque** clé.

```typescript
type Jour = "lun" | "mar" | "mer";

const heures: Record<Jour, number> = { lun: 7, mar: 8, mer: 6 };

let total = 0;
for (const jour of Object.keys(heures) as Jour[]) {
  total += heures[jour];
}
console.log(`Total : ${total} h`);

const stock: Record<string, number> = {};
stock["pommes"] = 12;
stock["poires"] = 3;
console.log(stock);
```

```sortie
Total : 21 h
{ pommes: 12, poires: 3 }
```

`Record<string, number>` sert de [dictionnaire](../debuter/glossaire.md#dictionnaire)
simple : n'importe quelle clé texte, des valeurs numériques.

> **Note :** `Object.keys` renvoie toujours un `string[]`. Le `as Jour[]` dit à
> TypeScript que ce sont bien des `Jour`, ce qui est vrai ici.

## typeof et keyof

Vous les avez croisés dans les chapitres précédents. Rappel :

- `typeof valeur`, **dans une position de type**, donne le type d'une valeur
  existante ;
- `keyof Type` donne l'union des noms de propriétés d'un type.

```typescript
const reglages = { volume: 5, muet: false, langue: "fr" };

type Reglages = typeof reglages;
type NomReglage = keyof Reglages;

function lire(nom: NomReglage): string {
  return `${nom} = ${reglages[nom]}`;
}

console.log(lire("volume"));
console.log(lire("langue"));
```

```sortie
volume = 5
langue = fr
```

Ici, `Reglages` vaut `{ volume: number; muet: boolean; langue: string }` et
`NomReglage` vaut `"volume" | "muet" | "langue"`. Vous n'avez écrit aucun type à la
main : ils suivent l'objet.

## Les types d'accès indexé

`Type["cle"]` donne le type d'**une** propriété. C'est la même écriture que pour lire
une valeur dans un objet, mais appliquée aux types.

```typescript
interface Commande {
  numero: number;
  client: { nom: string; ville: string };
  lignes: { produit: string; quantite: number }[];
}

type Client = Commande["client"];
type Ligne = Commande["lignes"][number];

const c: Client = { nom: "Léa", ville: "Lyon" };
const l: Ligne = { produit: "Cahier", quantite: 3 };
console.log(`${c.nom} commande ${l.quantite} × ${l.produit}`);
```

```sortie
Léa commande 3 × Cahier
```

`[number]` après un type tableau donne le type d'un élément. Pas besoin de déclarer
`Client` et `Ligne` séparément : on les extrait.

## ReturnType, Parameters et Awaited

Ces trois utilitaires lisent le type d'une **fonction** :

- `ReturnType<F>` : le type de ce qu'elle renvoie ;
- `Parameters<F>` : les types de ses paramètres, sous forme de tuple ;
- `Awaited<P>` : le type obtenu après `await` sur une promesse.

```typescript
function creerJoueur(nom: string, niveau: number) {
  return { nom, niveau, vies: 3 };
}

type Joueur = ReturnType<typeof creerJoueur>;
type Arguments = Parameters<typeof creerJoueur>;

const args: Arguments = ["Sam", 2];
const j: Joueur = creerJoueur(...args);
console.log(j);

async function chargerScore(): Promise<number> {
  return 1200;
}

type Score = Awaited<ReturnType<typeof chargerScore>>;
const s: Score = await chargerScore();
console.log(s + 1);
```

```sortie
{ nom: 'Sam', niveau: 2, vies: 3 }
1201
```

`Joueur` vaut `{ nom: string; niveau: number; vies: number }`, sans que vous l'ayez
écrit. Si la fonction change, le type suit.

## NonNullable

`NonNullable<T>` retire `null` et `undefined` d'une union.

```typescript
type PeutManquer = string | null | undefined;
type Present = NonNullable<PeutManquer>;

const noms: PeutManquer[] = ["Ana", null, "Léo", undefined];
const presents: Present[] = noms.filter((n): n is Present => n != null);
console.log(presents);
```

```sortie
[ 'Ana', 'Léo' ]
```

`n != null` (avec `!=`, pas `!==`) élimine à la fois `null` et `undefined`. La garde
de type `n is Present` vient du chapitre sur [les unions](unions.md#vos-propres-gardes-de-type).

## Les types transformés

Comment `Partial` est-il écrit ? Avec un **type transformé** (*mapped type*). Il
parcourt les clés d'un type, comme une boucle `for`, et construit un nouveau type.

```typescript
type Drapeaux<T> = {
  [K in keyof T]: boolean;
};

interface Formulaire {
  nom: string;
  age: number;
  email: string;
}

const modifies: Drapeaux<Formulaire> = { nom: true, age: false, email: false };
const champs = Object.entries(modifies)
  .filter(([, v]) => v)
  .map(([k]) => k);
console.log(`Champs modifiés : ${champs.join(", ")}`);
```

```sortie
Champs modifiés : nom
```

`[K in keyof T]: boolean` se lit : « pour chaque clé `K` de `T`, une propriété de type
`boolean` ». Le résultat a les mêmes clés que `Formulaire`, avec des booléens. Vous
pouvez aussi ajouter `?` ou `readonly` : `Partial<T>` est simplement
`{ [K in keyof T]?: T[K] }`.

## Les types conditionnels

Un **type conditionnel** choisit un type selon une condition. Il ressemble à
l'opérateur ternaire `a ? b : c`, mais pour les types :
`T extends U ? Oui : Non`.

```typescript
type Description<T> = T extends string ? "texte" : T extends number ? "nombre" : "autre";

function decrire<T>(valeur: T, description: Description<T>): string {
  return `${String(valeur)} est un ${description}`;
}

console.log(decrire("abc", "texte"));
console.log(decrire(42, "nombre"));
console.log(decrire(true, "autre"));
```

```sortie
abc est un texte
42 est un nombre
true est un autre
```

Ici, `decrire(42, "texte")` serait refusé : pour un nombre, `Description<number>`
vaut `"nombre"`. `NonNullable` et `Awaited` sont écrits avec des types conditionnels.

> **Note :** vous lirez souvent des types conditionnels dans les bibliothèques. Vous en
> écrirez rarement au début : les types utilitaires suffisent dans la plupart des cas.

## Les template literal types

Les gabarits de texte `` `Bonjour ${nom}` `` existent aussi pour les types. Combinés à
des unions, ils produisent toutes les combinaisons.

```typescript
type Taille = "petit" | "grand";
type Couleur = "rouge" | "bleu";
type Variante = `${Taille}-${Couleur}`;

const v1: Variante = "petit-rouge";
const v2: Variante = "grand-bleu";
console.log(v1, v2);

type Evenement = "clic" | "survol";
type Gestionnaire = `on${Capitalize<Evenement>}`;

const gestionnaires: Record<Gestionnaire, () => string> = {
  onClic: () => "cliqué",
  onSurvol: () => "survolé",
};
console.log(gestionnaires.onClic());
```

```sortie
petit-rouge grand-bleu
cliqué
```

`Variante` vaut `"petit-rouge" | "petit-bleu" | "grand-rouge" | "grand-bleu"`.
`Capitalize` met la première lettre en majuscule : `Gestionnaire` vaut
`"onClic" | "onSurvol"`. Il existe aussi `Uppercase`, `Lowercase` et `Uncapitalize`.

## Erreurs fréquentes

### Lire une propriété retirée par Omit

```typescript norun
type Utilisateur = { nom: string; email: string; motDePasse: string };
type Public = Omit<Utilisateur, "motDePasse">;
function afficher(u: Public) {
  console.log(u.motDePasse);
}
```

```text
main.ts(4,17): error TS2339: Property 'motDePasse' does not exist on type 'Public'.
```

C'est le but d'`Omit` : ce champ n'existe plus. Si vous en avez besoin ici, prenez le
type complet `Utilisateur`.

### Oublier une clé dans un Record

```typescript norun
type Jour = "lun" | "mar" | "mer";
const heures: Record<Jour, number> = { lun: 7, mar: 8 };
```

```text
main.ts(2,7): error TS2741: Property 'mer' is missing in type '{ lun: number; mar: number; }' but required in type 'Record<Jour, number>'.
```

Avec des clés littérales, chaque clé est obligatoire. Ajoutez `mer`, ou utilisez
`Partial<Record<Jour, number>>` si certaines peuvent manquer.

### Écrire Type.cle au lieu de Type["cle"]

```typescript norun
type Produit = { nom: string; prix: number };
type Prix = Produit.prix;
```

```text
main.ts(2,13): error TS2713: Cannot access 'Produit.prix' because 'Produit' is a type, but not a namespace. Did you mean to retrieve the type of the property 'prix' in 'Produit' with 'Produit["prix"]'?
```

Pour les types, le point ne marche pas. Utilisez les crochets : `Produit["prix"]`.
Le message propose d'ailleurs la correction.

## Testez-vous

**Question 1 :** quel type utilitaire choisir pour une fonction de mise à jour qui
reçoit seulement les champs à changer ?

> **Réponse**
>
> `Partial<T>`. Toutes les propriétés deviennent facultatives : on peut envoyer
> `{ email: "…" }` seul.

**Question 2 :** que vaut `keyof { x: number; y: number }` ?

> **Réponse**
>
> L'union des clés : `"x" | "y"`.

**Question 3 :** à partir de `interface Article { titre: string; prix: number; stock: number }`,
créez un type `Etiquette` qui contient seulement `titre` et `prix`, et utilisez-le.

> **Réponse**
>
> ```typescript
> interface Article {
>   titre: string;
>   prix: number;
>   stock: number;
> }
>
> type Etiquette = Pick<Article, "titre" | "prix">;
>
> const e: Etiquette = { titre: "Gomme", prix: 1.5 };
> console.log(`${e.titre} : ${e.prix} €`);
> ```
>
> ```sortie
> Gomme : 1.5 €
> ```
>
> `Omit<Article, "stock">` donne le même résultat.

## En résumé

- Les types utilitaires calculent un type à partir d'un autre : quand l'original
  change, ils suivent.
- `Partial`, `Required`, `Readonly` changent toutes les propriétés ; `Pick` et `Omit`
  en choisissent ; `Record` crée un objet clé → valeur.
- `typeof` donne le type d'une valeur, `keyof` l'union des clés, `T["cle"]` le type
  d'une propriété.
- `ReturnType`, `Parameters`, `Awaited` et `NonNullable` lisent des fonctions et des
  unions.
- Les types transformés (`[K in keyof T]`), conditionnels (`A extends B ? X : Y`) et
  les gabarits (`` `on${Nom}` ``) sont les briques des types utilitaires.

Suite : [Les décorateurs](decorateurs.md)
