# Passer de JavaScript à TypeScript

Cette page montre, étape par étape, comment transformer un projet JavaScript qui fonctionne en projet TypeScript, sans tout casser ni tout réécrire d'un coup.

> **Objectifs :**
>
> - faire vérifier du JavaScript par TypeScript avec `allowJs` et `checkJs` ;
> - décrire des types en JavaScript avec des commentaires JSDoc ;
> - renommer un fichier `.js` en `.ts` et ajouter les types petit à petit ;
> - utiliser `any` comme rustine temporaire, et savoir le retirer ;
> - activer le mode strict à la fin de la migration.

## Migrer petit à petit

Un projet JavaScript existant peut compter des centaines de fichiers. Tout convertir en
une fois, c'est risquer des jours de travail sans pouvoir lancer le programme.

La bonne méthode ressemble à la rénovation d'une maison habitée : une pièce à la fois,
et la maison reste habitable pendant les travaux. À chaque étape, le programme doit
**continuer à tourner**.

Voici le chemin que suit cette page :

1. laisser TypeScript lire le JavaScript (`allowJs`) ;
2. lui demander de le vérifier (`checkJs`), avec des types en commentaires JSDoc ;
3. renommer les fichiers un par un en `.ts` et ajouter de vrais types ;
4. boucher les trous avec `any`, provisoirement ;
5. activer `strict` et retirer les derniers `any`.

## Le point de départ

Voici un petit module JavaScript : il calcule le total d'un panier.

```javascript
function total(panier, remise) {
  let somme = 0;
  for (const article of panier) {
    somme += article.prix * article.quantite;
  }
  if (remise) {
    somme = somme * (1 - remise / 100);
  }
  return somme;
}

const panier = [
  { nom: "Stylo", prix: 2, quantite: 3 },
  { nom: "Cahier", prix: 4, quantite: 2 },
];
console.log(total(panier));
console.log(total(panier, 10));
```

```sortie
14
12.6
```

Rien ne dit ce qu'est un `panier`, ni si `remise` est un nombre ou un texte. Si
quelqu'un appelle `total(panier, "10")`, JavaScript calcule quand même, sans prévenir.

## Étape 1 : allowJs, TypeScript accepte le JavaScript

Dans `tsconfig.json` (voir [configurer TypeScript](configuration.md)), l'option
`allowJs` laisse `tsc` compiler des fichiers `.js` à côté des `.ts`. Les deux langages
peuvent cohabiter et s'importer l'un l'autre pendant toute la migration.

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "rootDir": "src",
    "outDir": "dist",
    "allowJs": true,
    "checkJs": false,
    "strict": false
  },
  "include": ["src"]
}
```

À ce stade, rien ne change dans le code : vous vérifiez juste que `npx tsc` produit un
`dist/` qui fonctionne comme avant.

## Étape 2 : checkJs et les types JSDoc

Avec `"checkJs": true`, TypeScript **vérifie** aussi les fichiers `.js`. Pour activer la
vérification dans un seul fichier, ajoutez plutôt ce commentaire en première ligne :

```javascript norun
// @ts-check
```

Vous pouvez alors décrire les types dans des commentaires **JSDoc** : des commentaires
qui commencent par `/**`. Le fichier reste du JavaScript pur, Node l'exécute tel quel,
mais votre éditeur et `tsc` comprennent les types.

```javascript
// @ts-check

/**
 * @typedef {Object} Article
 * @property {string} nom
 * @property {number} prix
 * @property {number} quantite
 */

/**
 * Calcule le total d'un panier.
 * @param {Article[]} panier
 * @param {number} [remise] pourcentage facultatif
 * @returns {number}
 */
function total(panier, remise) {
  let somme = 0;
  for (const article of panier) {
    somme += article.prix * article.quantite;
  }
  if (remise) {
    somme = somme * (1 - remise / 100);
  }
  return somme;
}

/** @type {Article[]} */
const panier = [
  { nom: "Stylo", prix: 2, quantite: 3 },
  { nom: "Cahier", prix: 4, quantite: 2 },
];
console.log(total(panier, 10));
```

```sortie
12.6
```

| Balise JSDoc | Équivalent TypeScript |
|--------------|-----------------------|
| `@param {number} prix` | `prix: number` |
| `@param {number} [remise]` | `remise?: number` |
| `@returns {number}` | `): number` |
| `@type {Article[]}` | `const panier: Article[]` |
| `@typedef {Object} Article` + `@property` | `interface Article { … }` |

> **Astuce :** JSDoc est une étape très utile pour les fichiers que vous ne pouvez pas
> encore renommer. Certains projets s'arrêtent même là : ils gardent du JavaScript, vérifié
> par TypeScript.

## Étape 3 : renommer en .ts

Quand un fichier est prêt, renommez-le de `.js` en `.ts`. Commencez par les fichiers
« feuilles », ceux qui n'importent rien d'autre du projet : ils sont plus simples.

Les types JSDoc deviennent de vrais types TypeScript. Le `@typedef` devient une
`interface`, les `@param` passent dans la signature :

```typescript
interface Article {
  nom: string;
  prix: number;
  quantite: number;
}

function total(panier: Article[], remise?: number): number {
  let somme = 0;
  for (const article of panier) {
    somme += article.prix * article.quantite;
  }
  if (remise) {
    somme = somme * (1 - remise / 100);
  }
  return somme;
}

const panier: Article[] = [
  { nom: "Stylo", prix: 2, quantite: 3 },
  { nom: "Cahier", prix: 4, quantite: 2 },
];
console.log(total(panier));
console.log(total(panier, 10));
```

```sortie
14
12.6
```

Le programme affiche exactement la même chose qu'avant : c'est la règle d'or de la
migration. Les types ne changent pas ce que fait le code, ils le décrivent.

> **Note :** sous Git (voir [le guide Git](../debuter/git.md)), utilisez `git mv total.js total.ts`
> pour renommer : l'historique du fichier est conservé.

## Étape 4 : any, une rustine provisoire

Parfois, un morceau de code est trop compliqué à typer tout de suite : une donnée venue
d'une vieille bibliothèque, un objet aux formes multiples. Le type `any` dit à TypeScript :
« ne vérifie rien ici ».

```typescript
// TODO : remplacer any par un vrai type
function afficherCommande(commande: any): void {
  console.log(`Commande ${commande.id} : ${commande.articles.length} article(s)`);
}

afficherCommande({ id: 7, articles: ["Stylo", "Cahier"] });
```

```sortie
Commande 7 : 2 article(s)
```

C'est pratique pour avancer, mais `any` **éteint** la vérification : une faute de frappe
comme `commande.artcles` passerait sans un mot. Marquez chaque `any` d'un commentaire
`TODO`, puis remplacez-les un par un :

```typescript
interface Commande {
  id: number;
  articles: string[];
}

function afficherCommande(commande: Commande): void {
  console.log(`Commande ${commande.id} : ${commande.articles.length} article(s)`);
}

afficherCommande({ id: 7, articles: ["Stylo", "Cahier"] });
```

```sortie
Commande 7 : 2 article(s)
```

> **Astuce :** `unknown` est un meilleur choix que `any` quand vous ne connaissez pas
> le type : TypeScript vous oblige à vérifier avant d'utiliser la valeur (voir
> [JSON et validation des données](formats.md)).

## Étape 5 : le mode strict, en dernier

Pendant la migration, `"strict": false` évite d'être noyé sous les erreurs. Une fois
tous les fichiers en `.ts`, activez-le :

```json
{
  "compilerOptions": {
    "strict": true
  }
}
```

De nouvelles erreurs apparaissent, surtout deux sortes :

- des paramètres sans type, qui étaient silencieusement des `any` ;
- des valeurs qui peuvent être `undefined` ou `null` et qui sont utilisées sans
  vérification.

Corrigez-les fichier par fichier. Si la liste est trop longue, activez d'abord les
options une par une (`noImplicitAny`, puis `strictNullChecks`), avant de passer à
`strict: true`.

Voici un exemple typique : `find` peut ne rien trouver, et renvoyer `undefined`.

```typescript
const villes = ["Lyon", "Nantes", "Paris"];

const trouvee = villes.find((v) => v.startsWith("P"));
if (trouvee !== undefined) {
  console.log(`${trouvee} a ${trouvee.length} lettres`);
}

const absente = villes.find((v) => v.startsWith("Z"));
console.log(absente ?? "aucune ville en Z");
```

```sortie
Paris a 5 lettres
aucune ville en Z
```

## Erreurs fréquentes

### Des paramètres sans type après le renommage

```typescript norun
function total(prix, quantite) {
  return prix * quantite;
}
console.log(total(3, 4));
```

```text
main.ts(1,16): error TS7006: Parameter 'prix' implicitly has an 'any' type.
main.ts(1,22): error TS7006: Parameter 'quantite' implicitly has an 'any' type.
```

En mode strict, chaque paramètre doit avoir un type. Écrivez
`function total(prix: number, quantite: number): number`.

### Construire un objet propriété par propriété

```typescript norun
const utilisateur = {};
utilisateur.nom = "Ada";
console.log(utilisateur.nom);
```

```text
main.ts(2,13): error TS2339: Property 'nom' does not exist on type '{}'.
main.ts(3,25): error TS2339: Property 'nom' does not exist on type '{}'.
```

TypeScript déduit le type `{}` (un objet vide) et refuse d'y ajouter des propriétés.
Créez l'objet en une fois : `const utilisateur = { nom: "Ada" };`, ou déclarez son type
avec une `interface`.

### Faire confiance à any

```typescript norun
const prix: any = "12";
console.log(prix.toFixed(2));
```

```text
main.js:2
console.log(prix.toFixed(2));
                 ^

TypeError: prix.toFixed is not a function
```

Le code compile, mais plante à l'exécution : `any` a caché le problème. Avec
`const prix: number = Number("12");`, TypeScript aurait vérifié l'appel.

### Oublier qu'une valeur peut être undefined

```typescript norun
const villes = ["Lyon", "Nantes"];
const trouvee = villes.find((v) => v.startsWith("P"));
console.log(trouvee.length);
```

```text
main.ts(3,13): error TS18048: 'trouvee' is possibly 'undefined'.
```

`find` renvoie `undefined` quand rien ne correspond. Vérifiez avec
`if (trouvee !== undefined)`, ou utilisez `trouvee?.length`.

## Testez-vous

**Question 1 :** dans quel ordre activer `allowJs`, `checkJs` et `strict` ?

> **Réponse**
>
> `allowJs` d'abord (TypeScript accepte le JavaScript), puis `checkJs` (il le vérifie),
> et `strict` à la fin, quand tous les fichiers sont en `.ts`.

**Question 2 :** traduisez en TypeScript ce commentaire JSDoc :
`@param {string} nom` et `@returns {boolean}` pour une fonction `estValide(nom)`.

> **Réponse**
>
> ```typescript
> function estValide(nom: string): boolean {
>   return nom.length > 0;
> }
>
> console.log(estValide("Ada"));
> ```
>
> ```sortie
> true
> ```

**Question 3 :** pourquoi faut-il retirer les `any` à la fin de la migration ?

> **Réponse**
>
> Parce que `any` désactive la vérification des types : les erreurs qu'il cache
> n'apparaissent qu'à l'exécution. Un `any` est une dette à rembourser.

## En résumé

- Migrez petit à petit : le programme doit tourner à chaque étape.
- `allowJs` fait cohabiter `.js` et `.ts` ; `checkJs` ou `// @ts-check` vérifie le JavaScript.
- Les commentaires JSDoc (`@param`, `@returns`, `@typedef`) typent du JavaScript pur.
- Renommez les fichiers en `.ts` un par un et remplacez les JSDoc par de vrais types.
- `any` dépanne provisoirement ; `strict: true` arrive en dernier.

Suite : [lire et écrire des fichiers avec Node](fichiers.md).
