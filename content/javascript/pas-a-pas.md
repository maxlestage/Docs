# Suivre un programme pas à pas en JavaScript

Cette page vous apprend à exécuter un programme JavaScript dans votre tête, ligne par ligne, avec un tableau qui suit la valeur de chaque variable.

> **Objectifs :**
>
> - vous saurez remplir un tableau de trace, ligne après ligne ;
> - vous saurez deviner ce qu'affiche un petit programme avant de le lancer ;
> - vous saurez suivre une condition, une boucle et un appel de fonction ;
> - vous saurez faire afficher ses étapes au programme avec `console.log` ;
> - vous saurez éviter les erreurs les plus courantes quand on suit un programme.

## Pourquoi jouer l'ordinateur ?

L'ordinateur ne devine rien. Il lit votre programme **une ligne à la fois**, de haut en
bas, et fait exactement ce qui est écrit. Rien de plus, rien de moins.

Pour comprendre un programme, faites comme lui. C'est comme suivre une recette de
cuisine avec le doigt : vous posez le doigt sur une ligne, vous la faites, puis vous
descendez d'une ligne.

```text
  ligne 1   ◄── le doigt est ici : je fais la ligne 1
  ligne 2
  ligne 3
```

Ce que vous gagnez :

- vous comprenez **pourquoi** le programme affiche ce qu'il affiche ;
- vous trouvez un [bug](../debuter/glossaire.md#bug) sans même lancer le code ;
- vous lisez plus vite le code des autres.

> **À retenir :** l'ordinateur lit une ligne, la fait, puis passe à la suivante. Suivre
> un programme, c'est faire pareil, sur papier.

## Le tableau de trace

Une [variable](../debuter/glossaire.md#variable) est une boîte avec une étiquette. Sa
valeur peut changer pendant le programme. Pour ne rien oublier, on note tout dans un
**tableau de trace** :

- une colonne par variable ;
- une ligne par étape (une ligne de code exécutée) ;
- une colonne « affiché » pour ce que montre `console.log`.

```text
 ┌───────┬──────┬──────┬───────────┐
 │ Étape │  a   │  b   │ affiché   │   ← une colonne par boîte
 ├───────┼──────┼──────┼───────────┤
 │   1   │  3   │  –   │           │   ← une ligne par étape
 │   2   │  3   │  5   │           │
 └───────┴──────┴──────┴───────────┘
```

Le tiret `–` veut dire « la boîte n'existe pas encore ». Quand une valeur change, vous
écrivez la nouvelle valeur sur la ligne de l'étape. L'ancienne valeur ne compte plus.

C'est comme le carnet d'un caissier : à chaque achat, il écrit le nouveau total. Il ne
garde pas l'ancien en tête.

> **À retenir :** une colonne par variable, une ligne par étape. On lit toujours la
> valeur **la plus récente**, celle de la ligne juste au-dessus.

## Programme 1 : des variables qui changent

Voici un programme de cinq lignes. Il ne contient que des calculs.

```javascript
let a = 3;
let b = 5;
a = a + b;
b = a - b;
a = a - b;
console.log(a, b);
```

**Avant de lancer, devinez ce qui s'affiche.** Prenez une feuille. Ne lisez pas la
suite tout de suite.

Voici la trace. À chaque étape, on calcule la partie à droite du `=` avec les valeurs
**actuelles**, puis on range le résultat dans la boîte à gauche.

| Étape | Ligne | Calcul | a | b | affiché |
| --- | --- | --- | --- | --- | --- |
| 1 | `let a = 3;` | – | 3 | – | |
| 2 | `let b = 5;` | – | 3 | 5 | |
| 3 | `a = a + b;` | 3 + 5 = 8 | 8 | 5 | |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | 3 | |
| 5 | `a = a - b;` | 8 − 3 = 5 | 5 | 3 | |
| 6 | `console.log(a, b);` | – | 5 | 3 | `5 3` |

```sortie
5 3
```

Les deux valeurs ont été **échangées** : `a` valait 3 et vaut 5, `b` valait 5 et vaut
3. À l'étape 4, le calcul utilise le **nouveau** `a` (8), pas l'ancien (3). C'est tout
le secret.

Essayez : remplacez `let a = 3;` par `let a = 10;`. La trace donne `5 10` : l'échange
marche avec n'importe quels nombres.

> **À retenir :** à droite du `=`, on lit les valeurs actuelles. À gauche, on range le
> résultat. Une seule boîte change par ligne.

## Programme 2 : une condition

Une [condition](../debuter/glossaire.md#condition) est un aiguillage : le programme
prend **un seul** chemin. Voici le prix d'un billet de cinéma selon l'âge.

```javascript
const age = 15;
let prix = 10;
if (age < 12) {
  prix = 5;
} else if (age < 18) {
  prix = 7;
}
console.log(`Prix : ${prix} €`);
```

**Avant de lancer, devinez ce qui s'affiche.**

Dans la trace, on ajoute une colonne pour la réponse de chaque test : `true` (vrai) ou
`false` (faux).

| Étape | Ligne | Test | age | prix | affiché |
| --- | --- | --- | --- | --- | --- |
| 1 | `const age = 15;` | – | 15 | – | |
| 2 | `let prix = 10;` | – | 15 | 10 | |
| 3 | `if (age < 12)` | 15 < 12 → `false` | 15 | 10 | |
| 4 | `else if (age < 18)` | 15 < 18 → `true` | 15 | 10 | |
| 5 | `prix = 7;` | – | 15 | 7 | |
| 6 | `console.log(…)` | – | 15 | 7 | `Prix : 7 €` |

```sortie
Prix : 7 €
```

La ligne `prix = 5;` n'est **jamais** exécutée : son test était faux. On la saute,
elle n'a pas de ligne dans la trace.

```text
            age < 12 ?
           /          \
        oui            non
         │              │
     prix = 5       age < 18 ?
                   /          \
                oui            non
                 │              │
             prix = 7     (rien : prix reste 10)
```

Essayez : mettez `const age = 30;`. Les deux tests sont faux, aucun bloc ne s'exécute,
et le programme affiche `Prix : 10 €`. Avec `const age = 12;`, le premier test est faux
(12 n'est pas plus petit que 12) : le prix est 7.

> **À retenir :** à chaque `if`, écrivez le test et sa réponse `true` ou `false`. Seul
> le bloc du premier test vrai s'exécute ; les autres sont sautés.

## Programme 3 : une boucle qui accumule

Une [boucle](../debuter/glossaire.md#boucle) répète des lignes. Ici, une tirelire :
la semaine 1, on met 2 €, la semaine 2, on met 4 €, et ainsi de suite.

```javascript
let epargne = 0;
let semaine = 1;
while (semaine <= 4) {
  epargne = epargne + semaine * 2;
  semaine = semaine + 1;
}
console.log(`Semaine ${semaine}, épargne ${epargne} €`);
```

**Avant de lancer, devinez ce qui s'affiche.** Attention à la valeur de `semaine` à la
fin.

Le test `semaine <= 4` est relu **avant chaque tour**. On le note à chaque fois.

| Étape | Ligne | Test | semaine | epargne |
| --- | --- | --- | --- | --- |
| 1 | `let epargne = 0;` | – | – | 0 |
| 2 | `let semaine = 1;` | – | 1 | 0 |
| 3 | `while (semaine <= 4)` | 1 <= 4 → `true` | 1 | 0 |
| 4 | `epargne = …` | 0 + 1 × 2 | 1 | 2 |
| 5 | `semaine = …` | 1 + 1 | 2 | 2 |
| 6 | `while (semaine <= 4)` | 2 <= 4 → `true` | 2 | 2 |
| 7 | `epargne = …` | 2 + 2 × 2 | 2 | 6 |
| 8 | `semaine = …` | 2 + 1 | 3 | 6 |
| 9 | `while (semaine <= 4)` | 3 <= 4 → `true` | 3 | 6 |
| 10 | `epargne = …` | 6 + 3 × 2 | 3 | 12 |
| 11 | `semaine = …` | 3 + 1 | 4 | 12 |
| 12 | `while (semaine <= 4)` | 4 <= 4 → `true` | 4 | 12 |
| 13 | `epargne = …` | 12 + 4 × 2 | 4 | 20 |
| 14 | `semaine = …` | 4 + 1 | 5 | 20 |
| 15 | `while (semaine <= 4)` | 5 <= 4 → `false` | 5 | 20 |
| 16 | `console.log(…)` | – | 5 | 20 |

```sortie
Semaine 5, épargne 20 €
```

La boucle a fait **4 tours**, mais le test a été lu **5 fois**. Le cinquième test est
faux : c'est lui qui arrête la boucle. Voilà pourquoi `semaine` vaut 5 à la fin, et non
4.

```text
   ┌──► test : semaine <= 4 ? ── non ──► sortir de la boucle
   │          │
   │         oui
   │          │
   │     ajouter à l'épargne
   │     semaine + 1
   └──────────┘
```

Essayez : remplacez `semaine <= 4` par `semaine < 4`. La boucle fait 3 tours, et le
programme affiche `Semaine 4, épargne 12 €`.

> **À retenir :** dans une boucle, notez le test à chaque tour, **y compris le dernier**,
> celui qui est faux. C'est là que se cachent les erreurs d'un tour en trop ou en moins.

## Programme 4 : une fonction appelée deux fois

Une [fonction](../debuter/glossaire.md#fonction) est une petite machine : on lui donne
une valeur, elle travaille, elle rend un résultat. Ici, on l'appelle deux fois.

```javascript
function carre(n) {
  const resultat = n * n;
  console.log(`carre(${n}) renvoie ${resultat}`);
  return resultat;
}

const a = carre(3);
const b = carre(a - 5);
console.log(`a + b = ${a + b}`);
```

**Avant de lancer, devinez ce qui s'affiche.** Combien de lignes ? Dans quel ordre ?

Les lignes 1 à 5 **définissent** la fonction : rien ne s'exécute encore. C'est une
recette rangée dans un tiroir. Le programme commence vraiment à `const a = carre(3);`.

Voici qui appelle qui, et ce qui revient :

```text
 programme                        carre
 ─────────                        ─────
 const a = carre(3)  ── n = 3 ──►  resultat = 9
                                   affiche « carre(3) renvoie 9 »
           a = 9     ◄── 9 ──────  return 9

 const b = carre(4)  ── n = 4 ──►  resultat = 16
                                   affiche « carre(4) renvoie 16 »
           b = 16    ◄── 16 ─────  return 16

 affiche « a + b = 25 »
```

La trace a une colonne « où ? » pour savoir dans quelle partie on est :

| Étape | Où ? | Ligne | n | resultat | a | b | affiché |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | programme | `const a = carre(3);` | – | – | – | – | |
| 2 | carre | `const resultat = n * n;` | 3 | 9 | – | – | |
| 3 | carre | `console.log(…)` | 3 | 9 | – | – | `carre(3) renvoie 9` |
| 4 | carre | `return resultat;` | 3 | 9 | – | – | |
| 5 | programme | `a` reçoit 9 | – | – | 9 | – | |
| 6 | programme | `const b = carre(a - 5);` : 9 − 5 = 4 | – | – | 9 | – | |
| 7 | carre | `const resultat = n * n;` | 4 | 16 | 9 | – | |
| 8 | carre | `console.log(…)` | 4 | 16 | 9 | – | `carre(4) renvoie 16` |
| 9 | carre | `return resultat;` | 4 | 16 | 9 | – | |
| 10 | programme | `b` reçoit 16 | – | – | 9 | 16 | |
| 11 | programme | `console.log(…)` | – | – | 9 | 16 | `a + b = 25` |

```sortie
carre(3) renvoie 9
carre(4) renvoie 16
a + b = 25
```

Trois remarques :

- à chaque appel, `n` et `resultat` sont des boîtes **neuves**. Elles disparaissent au
  `return` (le tiret `–` aux étapes 5 et 10) ;
- avant d'appeler, JavaScript calcule l'[argument](../debuter/glossaire.md#argument) :
  `a - 5` devient `4`, et c'est `4` qui entre dans `n` ;
- le `console.log` **dans** la fonction s'affiche pendant l'appel, avant la dernière
  ligne du programme.

Essayez : remplacez `carre(a - 5)` par `carre(carre(2))`. L'appel intérieur se fait
d'abord (`carre(2) renvoie 4`), puis l'appel extérieur (`carre(4) renvoie 16`). La
sortie devient `carre(3) renvoie 9`, `carre(2) renvoie 4`, `carre(4) renvoie 16`, puis
`a + b = 25`.

> **À retenir :** un appel de fonction, c'est un aller-retour. On part avec
> l'argument, on fait les lignes de la fonction avec des boîtes neuves, on revient avec
> la valeur du `return`.

## Programme 5 : une boucle sur un tableau avec une condition

Le dernier programme mélange tout : un tableau, une boucle et deux conditions. Il
compte les notes réussies (10 ou plus) et cherche la meilleure note.

```javascript
const notes = [12, 7, 15, 9, 18];
let reussies = 0;
let meilleure = notes[0];

for (const note of notes) {
  if (note >= 10) {
    reussies = reussies + 1;
  }
  if (note > meilleure) {
    meilleure = note;
  }
}

console.log(`Réussies : ${reussies}`);
console.log(`Meilleure : ${meilleure}`);
```

**Avant de lancer, devinez ce qui s'affiche.**

`for (const note of notes)` met dans `note` chaque valeur du tableau, l'une après
l'autre. C'est comme distribuer des cartes : une carte par tour.

```text
 notes :  [ 12 ,  7 , 15 ,  9 , 18 ]
            │     │    │    │    │
 tour :     1     2    3    4    5
```

Cette fois, on fait **une ligne par tour** (c'est plus court), avec les deux tests :

| Tour | note | note >= 10 ? | reussies | note > meilleure ? | meilleure |
| --- | --- | --- | --- | --- | --- |
| départ | – | – | 0 | – | 12 |
| 1 | 12 | `true` | 1 | 12 > 12 → `false` | 12 |
| 2 | 7 | `false` | 1 | 7 > 12 → `false` | 12 |
| 3 | 15 | `true` | 2 | 15 > 12 → `true` | 15 |
| 4 | 9 | `false` | 2 | 9 > 15 → `false` | 15 |
| 5 | 18 | `true` | 3 | 18 > 15 → `true` | 18 |

```sortie
Réussies : 3
Meilleure : 18
```

Au tour 4, on compare 9 avec **15**, pas avec 12 : `meilleure` a changé au tour 3. On
lit toujours la valeur de la ligne du dessus.

Essayez : ajoutez `20` au début du tableau. La meilleure note est déjà la première :
`meilleure` ne change jamais, et le programme affiche `Réussies : 4` puis
`Meilleure : 20`.

> **À retenir :** pour une boucle sur un tableau, une ligne par tour suffit : la valeur
> du tour, la réponse de chaque test, puis les boîtes qui changent.

## Faire afficher ses étapes au programme

Sur papier, c'est vous qui tenez la trace. Mais le programme peut aussi l'écrire
lui-même. Ajoutez un `console.log` **avec des étiquettes** dans la boucle :

```javascript
const notes = [12, 7, 15];
let reussies = 0;

for (const note of notes) {
  if (note >= 10) {
    reussies = reussies + 1;
  }
  console.log(`note=${note} reussies=${reussies}`);
}
```

```sortie
note=12 reussies=1
note=7 reussies=1
note=15 reussies=2
```

Chaque ligne affichée est une ligne de votre tableau de trace. Comparez-les : la
première différence montre l'endroit du problème.

Un raccourci pratique : mettez les variables entre accolades. `console.log` écrit
alors le nom **et** la valeur de chacune.

```javascript
let a = 3;
let b = 5;
console.log({ a, b });
a = a + b;
console.log({ a, b });
```

```sortie
{ a: 3, b: 5 }
{ a: 8, b: 5 }
```

Pour obtenir un vrai tableau, rangez chaque étape dans une liste, puis utilisez
`console.table` :

```javascript
const etapes = [];
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i;
  etapes.push({ i, total });
}
console.table(etapes);
```

```sortie
┌─────────┬───┬───────┐
│ (index) │ i │ total │
├─────────┼───┼───────┤
│ 0       │ 1 │ 1     │
│ 1       │ 2 │ 3     │
│ 2       │ 3 │ 6     │
└─────────┴───┴───────┘
```

> **À retenir :** un `console.log` avec étiquettes dans une boucle, c'est un tableau de
> trace écrit par le programme. Retirez-le quand le bug est trouvé.

## Le débogueur en deux phrases

Un [débogueur](../debuter/glossaire.md#debogage) met le programme en pause sur une
ligne (avec l'instruction `debugger;` ou un clic dans la marge de l'éditeur) et vous
montre la valeur de chaque variable. Vous avancez ensuite d'une ligne à la fois : c'est
le tableau de trace, rempli par l'ordinateur ; tout est expliqué dans
[déboguer](debogage.md#mettre-le-programme-en-pause).

## Erreurs fréquentes

### Oublier le dernier test d'une boucle

On pense que la boucle s'arrête « au bon moment », sans vérifier le dernier tour.

```javascript norun
const notes = [12, 15, 9];
let somme = 0;
for (let i = 0; i <= notes.length; i++) {
  somme = somme + notes[i];
}
console.log(somme);
```

```text
NaN
```

La trace montre le problème. `notes.length` vaut 3. Le test `3 <= 3` est vrai : il y a
un **quatrième** tour, avec `i = 3`. Or `notes[3]` n'existe pas : il vaut `undefined`,
et `36 + undefined` donne `NaN`.

| Tour | i | i <= 3 ? | notes[i] | somme |
| --- | --- | --- | --- | --- |
| 1 | 0 | `true` | 12 | 12 |
| 2 | 1 | `true` | 15 | 27 |
| 3 | 2 | `true` | 9 | 36 |
| 4 | 3 | `true` | `undefined` | `NaN` |

Correction : `i < notes.length`. Les [indices](../debuter/glossaire.md#indice) vont de
`0` à `notes.length - 1`.

### Reprendre une ancienne valeur

On veut échanger `a` et `b`, et on lit dans sa tête l'**ancienne** valeur de `a`.

```javascript norun
let a = 3;
let b = 5;
a = b;
b = a;
console.log(a, b);
```

```text
5 5
```

On attendait `5 3`. Mais après `a = b;`, la boîte `a` contient 5 : le 3 est perdu.
`b = a;` recopie donc 5. Dans la trace, la ligne 3 écrit 5 dans la colonne `a`, et la
ligne 4 lit cette colonne.

Correction : gardez l'ancienne valeur dans une troisième boîte.

```javascript
let a = 3;
let b = 5;
const ancienA = a;
a = b;
b = ancienA;
console.log(a, b);
```

```sortie
5 3
```

### Utiliser une boîte avant de la créer

Dans la trace, une colonne qui vaut `–` ne peut pas être lue. Le programme le refuse
aussi.

```javascript norun
let total = total + 5;
console.log(total);
```

```text
let total = total + 5;
            ^

ReferenceError: Cannot access 'total' before initialization
```

Correction : donnez d'abord une valeur de départ (`let total = 0;`), puis modifiez-la
sur la ligne suivante (`total = total + 5;`).

### Faire changer une boîte const

Votre trace montre que `compte` change à chaque tour. Une boîte qui change doit être
créée avec `let`, pas `const`.

```javascript norun
const compte = 0;
for (const n of [4, 8]) {
  compte = compte + n;
}
```

```text
  compte = compte + n;
         ^

TypeError: Assignment to constant variable.
```

Correction : `let compte = 0;`. Astuce : si une colonne de votre tableau change de
valeur, c'est un `let`.

## Testez-vous

**Question 1 :** que vaut `x` à la fin ?

```javascript norun
let x = 2;
x = x * 3;
x = x + x;
```

> **Réponse**
>
> `12`. Trace : `x` vaut 2, puis 2 × 3 = 6, puis 6 + 6 = 12. La troisième ligne lit 6,
> la valeur la plus récente.

**Question 2 :** combien de fois le test `i < 3` est-il lu dans
`for (let i = 0; i < 3; i++)` ?

> **Réponse**
>
> 4 fois : pour `i = 0`, `1` et `2` (vrai, la boucle tourne), puis pour `i = 3` (faux,
> la boucle s'arrête). La boucle fait 3 tours, mais le test est lu une fois de plus.

**Question 3 :** qu'affiche ce programme ?

```javascript norun
function plusUn(n) {
  return n + 1;
}
const r = plusUn(plusUn(5));
console.log(r);
```

> **Réponse**
>
> `7`. L'appel intérieur se fait d'abord : `plusUn(5)` renvoie 6. Puis `plusUn(6)`
> renvoie 7, rangé dans `r`.

## En résumé

- Un tableau de trace a une colonne par variable et une ligne par étape.
- À droite du `=`, on lit la valeur **la plus récente** ; à gauche, on range le
  résultat.
- Pour un `if`, notez le test et sa réponse ; pour une boucle, notez le test à chaque
  tour, y compris le dernier, qui est faux.
- Un appel de fonction est un aller-retour, avec des boîtes neuves à chaque appel.
- `console.log` avec étiquettes fait écrire la trace par le programme ; le débogueur
  la montre ligne par ligne.

Suite : [Les pièges du débutant en JavaScript](pieges.md)
