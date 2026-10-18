# Déboguer en JavaScript

Cette page vous apprend à trouver et corriger un bug en JavaScript : lire une trace d'erreur, afficher des valeurs avec `console`, mettre le programme en pause avec un débogueur et reconnaître les erreurs les plus courantes.

> **Objectifs :**
>
> - lire une trace d'erreur de Node et trouver la ligne fautive ;
> - afficher des valeurs avec `console.log`, `console.table` et `console.dir` ;
> - mettre le programme en pause avec `debugger`, les DevTools ou `node --inspect` ;
> - reconnaître les erreurs courantes et les surprises de `NaN` ;
> - mener une enquête pas à pas.

## Qu'est-ce que déboguer ?

Un [bug](../debuter/glossaire.md#bug) est un défaut dans un programme. **Déboguer**,
c'est trouver ce défaut et le corriger (voir
[le glossaire](../debuter/glossaire.md#debogage)). Il y a deux sortes de bugs :

- le programme **plante** : JavaScript affiche une erreur et s'arrête ;
- le programme **se trompe** : il tourne, mais le résultat est faux.

Dans les deux cas, la méthode est celle d'un détective : observer, faire une hypothèse,
vérifier, puis corriger. On ne corrige jamais au hasard.

> **Note :** la page [déboguer](../debuter/deboguer.md) du parcours « Débuter » donne
> la méthode générale, valable dans tous les langages.

## Lire une trace d'erreur

Quand le programme plante, Node affiche une **trace d'erreur** (*stack trace*). Voici
un programme qui calcule le prix d'un panier ; le deuxième article n'a pas de produit.

```javascript norun
function prixTTC(produit) {
  return produit.prix * 1.2;
}

function total(panier) {
  let somme = 0;
  for (const article of panier) {
    somme += prixTTC(article.produit);
  }
  return somme;
}

const panier = [
  { produit: { nom: "Stylo", prix: 2 } },
  { quantite: 3 },
];
console.log(total(panier));
```

```text
file:///home/alice/projet/main.mjs:2
  return produit.prix * 1.2;
                 ^

TypeError: Cannot read properties of undefined (reading 'prix')
    at prixTTC (file:///home/alice/projet/main.mjs:2:18)
    at total (file:///home/alice/projet/main.mjs:8:14)
    at file:///home/alice/projet/main.mjs:17:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)

Node.js v22.22.0
```

Lisez-la dans cet ordre :

1. **Le type et le message** : `TypeError: Cannot read properties of undefined (reading 'prix')`.
   On a voulu lire `.prix` sur `undefined`.
2. **La ligne montrée** : la ligne 2, avec un `^` sous l'endroit exact.
3. **La pile d'appels**, de haut en bas : l'erreur est dans `prixTTC` (ligne 2),
   appelée par `total` (ligne 8), appelée par le programme principal (ligne 17).

`main.mjs:2:18` veut dire : fichier `main.mjs`, ligne 2, colonne 18. Les lignes
`node:internal/...` viennent de Node lui-même : ignorez-les.

> **À retenir :** la ligne qui plante n'est pas toujours la ligne fautive. Ici, la
> ligne 2 est correcte : le vrai problème est l'article sans `produit`, ligne 15.
> Remontez la pile pour trouver d'où vient la mauvaise valeur.

## Afficher des valeurs avec console

La console est la loupe du programmeur. L'objet `console` propose plusieurs outils.

### console.log et les étiquettes

`console.log` affiche une ou plusieurs valeurs. Une astuce : entourez les variables
d'accolades. Vous obtenez un objet qui affiche **le nom et la valeur** de chacune.

```javascript
const prix = 12;
const quantite = "3";
console.log({ prix, quantite });
console.log("total :", prix * quantite, typeof quantite);
```

```sortie
{ prix: 12, quantite: '3' }
total : 36 string
```

Les guillemets autour de `'3'` trahissent un texte, pas un nombre. `typeof` le confirme.
Ce genre de détail explique beaucoup de bugs.

### console.table pour les listes

Pour un tableau d'objets, `console.table` dessine un vrai tableau, bien plus lisible :

```javascript
const eleves = [
  { nom: "Alice", note: 15 },
  { nom: "Bob", note: 9 },
  { nom: "Chloé", note: 12 },
];
console.table(eleves);
```

```sortie
┌─────────┬─────────┬──────┐
│ (index) │ nom     │ note │
├─────────┼─────────┼──────┤
│ 0       │ 'Alice' │ 15   │
│ 1       │ 'Bob'   │ 9    │
│ 2       │ 'Chloé' │ 12   │
└─────────┴─────────┴──────┘
```

### console.dir pour les objets profonds

`console.log` coupe les objets trop imbriqués et affiche `[Object]`. `console.dir`
avec `{ depth: null }` montre tout :

```javascript
const config = { nom: "app", options: { couleurs: { fond: "noir", texte: { taille: 12 } } } };
console.log(config);
console.dir(config, { depth: null });
```

```sortie
{
  nom: 'app',
  options: { couleurs: { fond: 'noir', texte: [Object] } }
}
{
  nom: 'app',
  options: { couleurs: { fond: 'noir', texte: { taille: 12 } } }
}
```

### console.error pour les messages d'erreur

`console.error` affiche comme `console.log`, mais sur la **sortie d'erreur** (*stderr*),
un canal séparé. Dans le terminal, vous voyez les deux ; mais si vous redirigez la
sortie normale dans un fichier, les erreurs restent à l'écran.

```javascript
console.log("Calcul terminé.");
console.error("Attention : 2 notes manquantes.");
```

```sortie
Calcul terminé.
```

Seul `Calcul terminé.` part sur la sortie normale ; le message d'erreur part sur
l'autre canal.

| Outil | Usage |
| --- | --- |
| `console.log(a, b)` | afficher des valeurs |
| `console.log({ a, b })` | afficher noms et valeurs |
| `console.table(liste)` | une liste d'objets en tableau |
| `console.dir(obj, { depth: null })` | un objet imbriqué en entier |
| `console.error(message)` | un message sur la sortie d'erreur |

> **Astuce :** retirez vos `console.log` de débogage une fois le bug corrigé. Laissés
> partout, ils noient les vrais messages.

## Mettre le programme en pause

Afficher des valeurs, c'est bien. Mettre le programme **en pause** pour tout regarder,
c'est mieux. Un **débogueur** arrête le programme sur une ligne choisie. Vous voyez
alors toutes les variables, puis vous avancez ligne par ligne. C'est un arrêt sur image
dans un film.

### L'instruction debugger et les DevTools du navigateur

Dans un navigateur, ouvrez les outils de développement (**DevTools**) avec `F12`. Puis
placez l'instruction `debugger` dans votre code :

```javascript norun
function appliquerRemise(prix, pourcentage) {
  debugger;
  const remise = prix * pourcentage / 100;
  return prix - remise;
}
```

Quand le navigateur atteint `debugger`, il s'arrête, si les DevTools sont ouverts.
L'onglet **Sources** montre alors :

- la ligne en pause, surlignée ;
- la valeur de chaque variable (`prix`, `pourcentage`…), dans le panneau **Scope** ;
- des boutons pour avancer : ligne suivante (`F10`), entrer dans la fonction (`F11`),
  reprendre (`F8`).

Vous pouvez aussi cliquer sur un numéro de ligne dans l'onglet Sources : c'est un
**point d'arrêt**, qui fait la même chose sans modifier le code.

> **Note :** sans DevTools ouverts, `debugger` ne fait rien. Retirez-le quand même avant
> de publier votre code.

### node --inspect avec Node

Pour un programme Node, lancez-le avec `--inspect-brk` : il s'arrête dès la première
ligne et attend un débogueur.

```sh
node --inspect-brk main.js
```

```text
Debugger listening on ws://127.0.0.1:9229/0f6c1b8e-3d2a-4b7e-9c51-8a2e4f6d7b10
For help, see: https://nodejs.org/en/docs/inspector
```

Ouvrez ensuite Chrome à l'adresse `chrome://inspect` et cliquez sur **inspect** : vous
retrouvez les mêmes DevTools que pour une page web. `--inspect` (sans `-brk`) lance le
programme sans pause au début ; il s'arrêtera aux `debugger` et aux points d'arrêt.

> **Astuce :** les éditeurs comme VS Code ont un débogueur intégré : cliquez dans la
> marge pour poser un point d'arrêt, puis lancez avec `F5`.

## Les erreurs les plus courantes

| Message | Ce qu'il veut dire |
| --- | --- |
| `TypeError: x.toUppercase is not a function` | la méthode n'existe pas : faute de frappe, ou mauvais type |
| `TypeError: Cannot read properties of undefined (reading 'nom')` | on lit `.nom` sur `undefined` |
| `TypeError: Cannot read properties of null (reading 'nom')` | pareil, sur `null` |
| `ReferenceError: totl is not defined` | variable inconnue, souvent une faute de frappe |
| `TypeError: Assignment to constant variable.` | on modifie une variable `const` |
| `TypeError: 5 is not iterable` | `for…of` sur une valeur qui n'est pas une liste |
| `SyntaxError: Unexpected token ';'` | le code est mal écrit : parenthèse, virgule… |
| `RangeError: Maximum call stack size exceeded` | une fonction s'appelle sans fin |

### « is not a function »

Ce message arrive quand on appelle avec `()` quelque chose qui n'est pas une fonction.
Le plus souvent, c'est une faute de frappe (`toUppercase` au lieu de `toUpperCase`) ou
une méthode de tableau appelée sur un texte :

```javascript norun
const texte = "a,b";
texte.map((x) => x);
```

```text
TypeError: texte.map is not a function
```

`map` existe pour les tableaux, pas pour les chaînes. Transformez d'abord le texte en
tableau : `texte.split(",").map((x) => x)`.

### « Cannot read properties of undefined »

C'est l'erreur la plus fréquente en JavaScript. Elle veut dire : la valeur **à gauche
du point** vaut `undefined`. Cherchez pourquoi : propriété mal écrite, élément de
tableau qui n'existe pas, fonction qui ne renvoie rien… Le chaînage optionnel `?.`
évite le plantage quand l'absence est normale :

```javascript
const utilisateur = { nom: "Alice" };
console.log(utilisateur.adresse?.ville);
console.log(utilisateur.adresse?.ville ?? "ville inconnue");
```

```sortie
undefined
ville inconnue
```

### Les surprises de NaN

`NaN` (*Not a Number*) apparaît quand un calcul n'a pas de sens. Il ne plante pas : il
se propage en silence, ce qui le rend difficile à repérer.

```javascript
console.log(Number("12 €"));
console.log(undefined + 1);
console.log("5" * 2);
console.log("5" + 2);
console.log(NaN === NaN);
console.log(Number.isNaN(Number("abc")));
```

```sortie
NaN
NaN
10
52
false
true
```

- `undefined + 1` donne `NaN` : souvent une propriété mal écrite en amont.
- `"5" * 2` convertit le texte en nombre, mais `"5" + 2` **colle** les textes.
- `NaN` n'est égal à rien, même pas à lui-même : testez avec `Number.isNaN(x)`.

> **Astuce :** quand un résultat vaut `NaN`, affichez chaque valeur du calcul avec
> `console.log({ a, b })`. La coupable est souvent un `undefined` ou un texte.

## Une enquête pas à pas

Voici une vraie enquête. Une fonction doit calculer la moyenne de notes écrites dans
un texte, comme `"12,15,9"`. La moyenne attendue est `12`.

```javascript
function moyenne(texte) {
  const notes = texte.split(",");
  const somme = notes.reduce((total, n) => total + n, 0);
  return somme / notes.length;
}

console.log(moyenne("12,15,9"));
```

```sortie
4053
```

`4053` ! Le programme ne plante pas, mais il se trompe.

**Étape 1, observer.** Le résultat est beaucoup trop grand. La division par 3 semble
normale ; regardons la somme.

**Étape 2, afficher.** Ajoutons un `console.log` avec étiquettes :

```javascript
function moyenne(texte) {
  const notes = texte.split(",");
  const somme = notes.reduce((total, n) => total + n, 0);
  console.log({ notes, somme });
  return somme / notes.length;
}

moyenne("12,15,9");
```

```sortie
{ notes: [ '12', '15', '9' ], somme: '012159' }
```

**Étape 3, comprendre.** Les guillemets parlent : les notes sont des **textes**.
`0 + "12"` colle les deux et donne `"012"`, puis `"01215"`, puis `"012159"`. Enfin,
`"012159" / 3` convertit en nombre : `4053`.

**Étape 4, corriger.** Convertissons chaque note en nombre avec `Number`, et retirons
le `console.log` :

```javascript
function moyenne(texte) {
  const notes = texte.split(",").map(Number);
  const somme = notes.reduce((total, n) => total + n, 0);
  return somme / notes.length;
}

console.log(moyenne("12,15,9"));
```

```sortie
12
```

**Étape 5, protéger.** Écrivez un test pour que ce bug ne revienne jamais (voir
[tester son code](tests.md)).

> **À retenir :** observer, afficher, comprendre, corriger, protéger. Ne changez le code
> qu'après avoir compris la cause.

## Erreurs fréquentes

### Comparer avec NaN

```javascript norun
const resultat = Number("abc");
if (resultat === NaN) {
  console.log("Ce n'est pas un nombre.");
} else {
  console.log(`Résultat : ${resultat}`);
}
```

```text
Résultat : NaN
```

`NaN === NaN` vaut `false` : la condition n'est jamais vraie. Écrivez
`if (Number.isNaN(resultat))`.

### Déborder d'un tableau

```javascript norun
const nombres = [1, 2, 3];
for (let i = 0; i <= nombres.length; i++) {
  console.log(nombres[i] * 2);
}
```

```text
2
4
6
NaN
```

Avec `<=`, la boucle lit `nombres[3]`, qui n'existe pas : `undefined * 2` donne `NaN`.
Les indices vont de `0` à `length - 1` : écrivez `i < nombres.length`, ou mieux
`for (const n of nombres)`.

### Oublier la valeur de départ de reduce

```javascript norun
const panier = [{ prix: 2 }, { prix: 3 }];
const total = panier.reduce((somme, article) => somme + article.prix);
console.log(total);
```

```text
[object Object]3
```

Sans valeur de départ, `reduce` part du **premier objet** et l'ajoute à un nombre.
Donnez toujours la valeur initiale : `panier.reduce((somme, a) => somme + a.prix, 0)`.

## Testez-vous

**Question 1 :** dans une trace d'erreur, comment trouver la fonction qui a appelé
celle qui a planté ?

> **Réponse**
>
> Elle est sur la ligne `at …` juste en dessous. La pile se lit de haut en bas : de
> l'endroit de l'erreur jusqu'au programme principal.

**Question 2 :** que veut dire `TypeError: Cannot read properties of undefined (reading 'age')` ?

> **Réponse**
>
> Le code a écrit `quelqueChose.age`, mais `quelqueChose` vaut `undefined`. Il faut
> chercher pourquoi cette valeur est absente, souvent plus haut dans le programme.

**Question 3 :** pourquoi `console.log({ prix })` est-il plus utile que `console.log(prix)` ?

> **Réponse**
>
> Il affiche le **nom** de la variable avec sa valeur : `{ prix: 12 }`. Avec plusieurs
> affichages, vous savez tout de suite quelle valeur est laquelle.

## En résumé

- Une trace d'erreur se lit de haut en bas : type, message, ligne, puis pile d'appels.
- `console.log({ a, b })`, `console.table` et `console.dir` montrent les valeurs ;
  `console.error` écrit sur la sortie d'erreur.
- `debugger`, les DevTools (`F12`) et `node --inspect` mettent le programme en pause.
- `NaN` se propage en silence ; testez-le avec `Number.isNaN`.
- Une enquête : observer, afficher, comprendre, corriger, protéger par un test.

Suite : [bien écrire du JavaScript](style.md). Comparez avec
[le débogage en Ruby](../ruby/debogage.md).
