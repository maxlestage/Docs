# Bien écrire du JavaScript

Cette page rassemble les habitudes qui rendent un code JavaScript clair et sûr : `const` par défaut, `===`, de bons noms, de petites fonctions, pas de modifications cachées, et deux outils qui vérifient tout cela pour vous, ESLint et Prettier.

> **Objectifs :**
>
> - choisir entre `const` et `let`, et toujours comparer avec `===` ;
> - nommer variables, fonctions et classes selon l'usage ;
> - écrire de petites fonctions avec des retours anticipés ;
> - éviter les modifications cachées avec la décomposition (`...`) et `map` ;
> - vérifier et formater le code avec ESLint et Prettier.

## Pourquoi soigner son style ?

Un programme est lu bien plus souvent qu'il n'est écrit : par vos collègues, et par
vous-même dans trois mois. Un code clair se comprend vite, se modifie sans peur et cache
moins de [bugs](../debuter/glossaire.md#bug).

Le style, c'est comme l'orthographe : le lecteur comprend un texte plein de fautes,
mais il se fatigue, et il peut se tromper de sens.

> **À retenir :** écrivez pour le prochain lecteur. Le code le plus malin n'est pas le
> meilleur ; le plus clair, si.

## const par défaut

Déclarez vos [variables](../debuter/glossaire.md#variable) avec `const`. Passez à `let`
seulement si la variable doit **changer** de valeur. N'utilisez plus `var` : c'est
l'ancienne façon, avec des règles de portée surprenantes.

```javascript
const prenom = "Alice";
const notes = [12, 15];
notes.push(9);

let total = 0;
for (const note of notes) {
  total += note;
}
console.log(prenom, notes, total);
```

```sortie
Alice [ 12, 15, 9 ] 36
```

- `prenom` ne change jamais : `const`.
- `notes` désigne toujours le même tableau, même si son contenu grandit : `const`.
- `total` reçoit une nouvelle valeur à chaque tour : `let`.

Avec `const`, le lecteur sait d'un coup d'œil que la variable ne sera pas remplacée
plus loin.

## Toujours === et !==

`==` convertit les valeurs avant de comparer, avec des règles difficiles à retenir.
`===` compare sans conversion : même type **et** même valeur.

```javascript
console.log("0" == 0, "" == 0, [] == false);
console.log("0" === 0, "" === 0);
```

```sortie
true true true
false false
```

Personne ne veut que `""` soit égal à `0`. Utilisez toujours `===` et `!==` : vous
n'aurez jamais ces surprises (voir [les types](types.md)).

## Les noms

Un bon nom dit **ce que contient** une variable ou **ce que fait** une fonction. Les
conventions de JavaScript :

| Élément | Style | Exemple |
| --- | --- | --- |
| Variable, paramètre | `camelCase` | `prixTotal`, `nombreEleves` |
| Fonction | `camelCase`, avec un verbe | `calculerMoyenne()`, `afficherListe()` |
| Booléen | `est…`, `a…`, `peut…` | `estConnecte`, `aPaye` |
| Classe | `PascalCase` | `CompteBancaire`, `Eleve` |
| Constante fixe du programme | `MAJUSCULES_SOULIGNEES` | `TAUX_TVA`, `MAX_ESSAIS` |
| Fichier, dossier | minuscules et tirets | `liste-taches.js` |

```javascript
const TAUX_TVA = 0.2;

class Article {
  constructor(nom, prixHT) {
    this.nom = nom;
    this.prixHT = prixHT;
  }
}

function calculerPrixTTC(article) {
  return article.prixHT * (1 + TAUX_TVA);
}

const stylo = new Article("Stylo", 2);
const estCher = calculerPrixTTC(stylo) > 10;
console.log(calculerPrixTTC(stylo), estCher);
```

```sortie
2.4 false
```

> **Astuce :** évitez les noms d'une lettre (`x`, `d`), sauf pour un indice de boucle
> `i` ou une flèche très courte comme `(n) => n * 2`.

## De petites fonctions

Une [fonction](../debuter/glossaire.md#fonction) fait **une seule chose**, et son nom
dit laquelle. Si vous avez besoin du mot « et » pour la décrire, coupez-la en deux.

Avant : une fonction qui fait tout.

```javascript
function bulletin(eleve) {
  let somme = 0;
  for (const note of eleve.notes) {
    somme += note;
  }
  const moy = somme / eleve.notes.length;
  let mention = "aucune";
  if (moy >= 16) {
    mention = "très bien";
  } else if (moy >= 14) {
    mention = "bien";
  }
  console.log(eleve.nom + " : " + moy.toFixed(1) + " (" + mention + ")");
}

bulletin({ nom: "Alice", notes: [15, 17, 16] });
```

```sortie
Alice : 16.0 (très bien)
```

Après : trois fonctions courtes, chacune avec un nom clair.

```javascript
function moyenne(notes) {
  return notes.reduce((somme, n) => somme + n, 0) / notes.length;
}

function mention(moy) {
  if (moy >= 16) return "très bien";
  if (moy >= 14) return "bien";
  return "aucune";
}

function bulletin(eleve) {
  const moy = moyenne(eleve.notes);
  console.log(`${eleve.nom} : ${moy.toFixed(1)} (${mention(moy)})`);
}

bulletin({ nom: "Alice", notes: [15, 17, 16] });
```

```sortie
Alice : 16.0 (très bien)
```

Le résultat est identique, mais `moyenne` et `mention` se testent et se réutilisent
seules. Le gabarit `` `…${…}…` `` remplace les `+` (voir [le texte](texte.md)).

## Les retours anticipés

Une pile de `if` imbriqués pousse le code vers la droite et le rend difficile à suivre.
Traitez d'abord les cas particuliers, et **sortez tout de suite** avec `return`. Ces
tests du début s'appellent des **clauses de garde**.

Avant :

```javascript
function livrer(commande) {
  if (commande) {
    if (commande.payee) {
      if (commande.articles.length > 0) {
        return "Livraison lancée";
      } else {
        return "Commande vide";
      }
    } else {
      return "Paiement en attente";
    }
  } else {
    return "Aucune commande";
  }
}

console.log(livrer({ payee: true, articles: ["livre"] }));
console.log(livrer({ payee: false, articles: [] }));
```

```sortie
Livraison lancée
Paiement en attente
```

Après :

```javascript
function livrer(commande) {
  if (!commande) return "Aucune commande";
  if (!commande.payee) return "Paiement en attente";
  if (commande.articles.length === 0) return "Commande vide";
  return "Livraison lancée";
}

console.log(livrer({ payee: true, articles: ["livre"] }));
console.log(livrer({ payee: false, articles: [] }));
```

```sortie
Livraison lancée
Paiement en attente
```

Le cas normal est à la fin, sans aucune indentation : on le trouve tout de suite.

## Éviter les modifications cachées

Un tableau ou un objet n'est pas copié quand on le donne à une autre variable : les deux
noms désignent **le même** objet. Le modifier d'un côté le modifie partout. C'est une
grande source de bugs (voir [les objets](objets.md)).

```javascript
const eleve = { nom: "Alice", note: 12 };
const copie = eleve;
copie.note = 20;
console.log(eleve);
```

```sortie
{ nom: 'Alice', note: 20 }
```

### Créer une copie avec la décomposition

Préférez fabriquer un **nouvel** objet. La décomposition `...` copie les propriétés,
puis vous changez celles que vous voulez :

```javascript
const eleve = { nom: "Alice", note: 12 };
const corrige = { ...eleve, note: 20 };
console.log(eleve);
console.log(corrige);

const notes = [12, 8];
const plusDeNotes = [...notes, 15];
console.log(notes, plusDeNotes);
```

```sortie
{ nom: 'Alice', note: 12 }
{ nom: 'Alice', note: 20 }
[ 12, 8 ] [ 12, 8, 15 ]
```

L'original reste intact. On dit qu'on le traite comme
[immuable](../debuter/glossaire.md#immuable).

### map, filter et toSorted plutôt que modifier

Avant : une boucle qui modifie le tableau reçu. L'appelant a une mauvaise surprise.

```javascript
function augmenter(prix) {
  for (let i = 0; i < prix.length; i++) {
    prix[i] = prix[i] * 1.1;
  }
  return prix;
}

const tarifs = [10, 20];
const nouveaux = augmenter(tarifs);
console.log(tarifs, nouveaux);
```

```sortie
[ 11, 22 ] [ 11, 22 ]
```

Après : `map` fabrique un nouveau tableau. `toSorted` fait de même pour le tri, alors
que `sort` modifie l'original.

```javascript
function augmenter(prix) {
  return prix.map((p) => p * 1.1);
}

const tarifs = [10, 20];
console.log(tarifs, augmenter(tarifs));

const notes = [12, 8, 15];
console.log(notes.toSorted((a, b) => a - b), notes);
```

```sortie
[ 10, 20 ] [ 11, 22 ]
[ 8, 12, 15 ] [ 12, 8, 15 ]
```

## Le chaînage optionnel ?. et ??

Deux opérateurs récents rendent le code plus court et plus sûr :

- `a?.b` lit `b` seulement si `a` existe ; sinon il donne `undefined`, sans planter ;
- `a ?? valeur` donne `valeur` seulement si `a` vaut `null` ou `undefined`.

```javascript
const utilisateur = { nom: "Alice", reglages: { volume: 0 } };
const invite = { nom: "Invité" };

console.log(utilisateur.reglages?.volume ?? 50);
console.log(invite.reglages?.volume ?? 50);
console.log(invite.adresse?.ville ?? "inconnue");
```

```sortie
0
50
inconnue
```

> **Attention :** n'utilisez pas `||` pour une valeur par défaut. `||` remplace aussi
> `0`, `""` et `false`, qui sont des valeurs valables. `??` ne remplace que l'absence.

## Des commentaires utiles et JSDoc

Un bon [commentaire](../debuter/glossaire.md#commentaire) explique **pourquoi**, pas
**quoi** : le code dit déjà ce qu'il fait.

```javascript norun
// Mauvais : répète le code.
// Ajoute 1 à essais.
essais += 1;

// Bon : explique une raison invisible.
// Le serveur refuse plus de 3 essais par minute.
const MAX_ESSAIS = 3;
```

Pour décrire une fonction, utilisez un commentaire **JSDoc** : il commence par `/**` et
décrit les paramètres et la valeur renvoyée. Les éditeurs l'affichent quand vous
survolez la fonction, et vérifient même les types.

```javascript
/**
 * Calcule le prix toutes taxes comprises.
 * @param {number} prixHT - le prix hors taxes, en euros
 * @param {number} [taux=0.2] - le taux de TVA
 * @returns {number} le prix TTC
 */
function prixTTC(prixHT, taux = 0.2) {
  return prixHT * (1 + taux);
}

console.log(prixTTC(10));
```

```sortie
12
```

Les crochets autour de `[taux=0.2]` signalent un paramètre facultatif, avec sa valeur
par défaut.

## ESLint et Prettier

Deux outils, installés avec npm (voir [organiser un projet](projet.md)), appliquent ces
règles pour vous :

- **ESLint** cherche les erreurs et les mauvaises habitudes ;
- **Prettier** remet en forme le code : espaces, guillemets, retours à la ligne.

```sh
npm install --save-dev eslint @eslint/js globals prettier
```

### ESLint

ESLint se règle dans un fichier `eslint.config.js`, à la racine du projet :

```javascript norun
import js from "@eslint/js";
import globals from "globals";

export default [
  js.configs.recommended,
  {
    languageOptions: { globals: globals.node },
    rules: {
      eqeqeq: "error",
      "prefer-const": "error",
    },
  },
];
```

Prenons ce fichier `src/panier.js`, plein de petits défauts :

```javascript norun
let taux = 1.2;
const remise = 5;

function total(prix) {
  if (prix == "0") {
    return 0;
  }
  return prix * taux;
}

console.log(totl(10));
```

```sh
npx eslint src/
```

```text
/home/alice/projet/src/panier.js
   1:5   error  'taux' is never reassigned. Use 'const' instead  prefer-const
   2:7   error  'remise' is assigned a value but never used      no-unused-vars
   4:10  error  'total' is defined but never used                no-unused-vars
   5:12  error  Expected '===' and instead saw '=='              eqeqeq
  11:13  error  'totl' is not defined                            no-undef

✖ 5 problems (5 errors, 0 warnings)
  1 error and 0 warnings potentially fixable with the `--fix` option.
```

Chaque ligne donne la position (`ligne:colonne`), le problème et le nom de la règle. La
faute de frappe `totl` est trouvée **avant** de lancer le programme.
`npx eslint src/ --fix` corrige seul ce qui peut l'être.

### Prettier

Prettier ne discute pas : il impose une mise en page unique. Voici un fichier tassé :

```javascript norun
const personne={nom:'Alice',age:30,ville:"Lyon"}
function saluer(p){return 'Bonjour '+p.nom}
console.log(saluer(personne))
```

```sh
npx prettier --check src/
```

```text
Checking formatting...
[warn] src/moche.js
[warn] Code style issues found in the above file. Run Prettier with --write to fix.
```

```sh
npx prettier --write src/
```

Le fichier devient :

```javascript norun
const personne = { nom: "Alice", age: 30, ville: "Lyon" };
function saluer(p) {
  return "Bonjour " + p.nom;
}
console.log(saluer(personne));
```

> **Astuce :** réglez votre éditeur pour lancer Prettier à chaque enregistrement. Vous
> n'aurez plus jamais à penser aux espaces.

## Erreurs fréquentes

### Utiliser || pour une valeur par défaut

```javascript norun
const reglages = { volume: 0 };
console.log(reglages.volume || 50);
```

```text
50
```

L'utilisateur a choisi le silence (`0`), mais `||` le remplace par `50`, car `0` est
« faux ». Écrivez `reglages.volume ?? 50`, qui affiche `0`.

### Trier sur place sans le vouloir

```javascript norun
const notes = [12, 8, 15];
const triees = notes.sort((a, b) => a - b);
console.log(notes, triees);
```

```text
[ 8, 12, 15 ] [ 8, 12, 15 ]
```

`sort` modifie le tableau d'origine : l'ordre de saisie des notes est perdu. Utilisez
`notes.toSorted((a, b) => a - b)`, qui renvoie un nouveau tableau.

### Réassigner une variable const

```javascript norun
const age = 30;
age = 31;
```

```text
file:///home/alice/projet/main.mjs:2
age = 31;
    ^

TypeError: Assignment to constant variable.
```

C'est voulu : `const` interdit de remplacer la valeur. Si elle doit vraiment changer,
déclarez-la avec `let`.

## Testez-vous

**Question 1 :** `const` ou `let` pour `const panier = []` auquel on ajoute des articles
avec `push` ?

> **Réponse**
>
> `const` suffit. La variable désigne toujours le même tableau ; seul son contenu
> change. `let` n'est utile que si l'on écrit `panier = …` plus loin.

**Question 2 :** que vaut `"" ?? "vide"` ? Et `"" || "vide"` ?

> **Réponse**
>
> `"" ?? "vide"` vaut `""`, car `""` n'est ni `null` ni `undefined`. `"" || "vide"`
> vaut `"vide"`, car `""` est une valeur « fausse ».

**Question 3 :** réécrivez cette fonction avec des retours anticipés.

```javascript norun
function acces(age) {
  if (age >= 0) {
    if (age >= 18) {
      return "autorisé";
    } else {
      return "trop jeune";
    }
  } else {
    return "âge invalide";
  }
}
```

> **Réponse**
>
> ```javascript
> function acces(age) {
>   if (age < 0) return "âge invalide";
>   if (age < 18) return "trop jeune";
>   return "autorisé";
> }
>
> console.log(acces(-2), acces(12), acces(30));
> ```
>
> ```sortie
> âge invalide trop jeune autorisé
> ```

## En résumé

- `const` par défaut, `let` si la valeur change, jamais `var` ; toujours `===`.
- `camelCase` pour les variables et fonctions, `PascalCase` pour les classes,
  `MAJUSCULES` pour les constantes fixes.
- De petites fonctions avec un nom clair ; des retours anticipés plutôt que des `if`
  imbriqués.
- Ne modifiez pas les données reçues : `...`, `map`, `filter`, `toSorted`.
- `?.` et `??` gèrent les valeurs absentes ; ESLint et Prettier vérifient le reste.

Suite : [organiser un projet](projet.md). Comparez avec
[le style en Ruby](../ruby/style.md).
