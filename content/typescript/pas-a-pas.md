# Suivre un programme pas à pas

Cette page vous apprend à exécuter un programme TypeScript dans votre tête, ligne par ligne, avec un tableau de suivi, sur cinq petits programmes de plus en plus riches.

> **Objectifs :**
>
> - vous saurez remplir un tableau de suivi : une colonne par variable, une ligne par
>   étape ;
> - vous saurez suivre une condition, une boucle et l'appel d'une fonction ;
> - vous saurez deviner ce qu'affiche un programme **avant** de le lancer ;
> - vous saurez faire afficher ses étapes au programme avec `console.log` ;
> - vous saurez où chercher quand votre tableau et l'ordinateur ne sont pas d'accord.

## Pourquoi suivre un programme dans sa tête

L'ordinateur est un cuisinier très obéissant, mais sans aucune imagination. Il lit la
recette une ligne à la fois, de haut en bas. Il fait **exactement** ce qui est écrit,
même si c'est absurde.

Pour comprendre un programme, faites comme lui. Prenez un papier et un crayon. Lisez
une ligne, faites ce qu'elle dit, notez le résultat. Puis passez à la ligne suivante.

Ce papier s'appelle un **tableau de suivi**. C'est la meilleure arme contre les
[bugs](../debuter/glossaire.md#bug) : la plupart viennent d'un écart entre ce que vous
**croyez** que le programme fait et ce qu'il fait **vraiment**.

```text
   ce que vous croyez            ce que fait l'ordinateur
   ┌────────────────┐            ┌────────────────┐
   │ total vaut 20  │    ≠       │ total vaut 12  │   ◄── le bug est ici
   └────────────────┘            └────────────────┘
            le tableau de suivi montre l'écart
```

> **À retenir :** l'ordinateur ne devine jamais. Il suit les lignes une par une. Pour
> le comprendre, faites la même chose, sur papier.

## La méthode du tableau de suivi

Un tableau de suivi ressemble à un carnet de notes :

- une colonne **Étape** (ou **Ligne**) : où en est le programme ;
- une colonne par [variable](../debuter/glossaire.md#variable) : sa
  [valeur](../debuter/glossaire.md#valeur) à ce moment ;
- une colonne **Affiche** : ce que `console.log` écrit à l'écran.

Quatre règles simples :

1. Une ligne du tableau pour **chaque** ligne de code exécutée, dans l'ordre.
2. Quand une variable change, écrivez sa **nouvelle** valeur. Barrez l'ancienne dans
   votre tête : elle n'existe plus.
3. Une case vide veut dire « pas encore créée » ou « pas changée ».
4. Avant de lancer le programme, **devinez** la sortie. Puis comparez.

Dans les programmes de cette page, chaque ligne porte un commentaire `// L1`, `// L2`…
C'est le numéro de ligne : il sert à remplir la colonne **Étape**.

> **À retenir :** une ligne de code exécutée = une ligne du tableau. Ne sautez jamais
> une ligne, même si elle semble évidente.

## Programme 1 : des variables qui changent

Voici trois lignes qui jouent avec deux boîtes, `a` et `b`.

```typescript
let a = 5; // L1
let b = 3; // L2
a = a + b; // L3
b = a - b; // L4
a = a - b; // L5
console.log(a, b); // L6
```

**Avant de lancer, devinez ce qui s'affiche.** Écrivez votre réponse sur un papier.

```sortie
3 5
```

Voici le tableau de suivi. Lisez-le ligne par ligne.

| Étape | Ce qui se passe | a | b | Affiche |
|---|---|---|---|---|
| L1 | on crée `a` | 5 | | |
| L2 | on crée `b` | 5 | 3 | |
| L3 | `a + b` = 5 + 3 = 8 | **8** | 3 | |
| L4 | `a - b` = 8 − 3 = 5 | 8 | **5** | |
| L5 | `a - b` = 8 − 5 = 3 | **3** | 5 | |
| L6 | on affiche | 3 | 5 | `3 5` |

Le piège est à la ligne L5. On calcule `a - b` avec la **nouvelle** valeur de `b`
(5), pas l'ancienne (3). Le programme a donc **échangé** les valeurs de `a` et `b`.

Le signe `=` ne veut pas dire « est égal à » comme en maths. Il veut dire : « calcule
la droite, puis range le résultat dans la boîte de gauche ».

```text
   a = a + b
   ▲   └─┬─┘
   │     └── 1. on calcule avec les valeurs ACTUELLES : 5 + 3 = 8
   └──────── 2. on range 8 dans la boîte a (l'ancien 5 est jeté)
```

> **Essayez :** remplacez `let b = 3;` par `let b = 10;`. Refaites le tableau.
> Le programme affiche `10 5` : il échange toujours les deux valeurs.

> **À retenir :** à chaque ligne, utilisez la valeur la plus **récente** de chaque
> variable, celle de la ligne juste au-dessus dans votre tableau.

## Programme 2 : une condition

Un cinéma fait payer 10 €, 5 € pour les moins de 12 ans, et 7 € pour les moins de
18 ans.

```typescript
const age = 15; // L1
let prix = 10; // L2
if (age < 12) { // L3
  prix = 5; // L4
} else if (age < 18) { // L5
  prix = 7; // L6
}
console.log(`Prix : ${prix} €`); // L7
```

**Avant de lancer, devinez ce qui s'affiche.**

```sortie
Prix : 7 €
```

Une [condition](../debuter/glossaire.md#condition) est un aiguillage de train : le
programme prend **un seul** chemin. Ajoutez une colonne **Test** pour noter le
résultat de chaque question : `true` (vrai) ou `false` (faux).

| Étape | Test | age | prix | Affiche |
|---|---|---|---|---|
| L1 | | 15 | | |
| L2 | | 15 | 10 | |
| L3 | `15 < 12` → `false` | 15 | 10 | |
| L4 | *sautée* | | | |
| L5 | `15 < 18` → `true` | 15 | 10 | |
| L6 | | 15 | **7** | |
| L7 | | 15 | 7 | `Prix : 7 €` |

La ligne L4 n'est **jamais** exécutée : le premier test est faux, le programme saute
directement au `else if`.

```text
              age < 12 ?
             /          \
          oui            non
           │              │
       prix = 5       age < 18 ?
                     /          \
                  oui            non
                   │              │
               prix = 7      (rien : prix reste 10)
```

> **Essayez :** mettez `const age = 30;`. Les deux tests sont faux : le programme
> affiche `Prix : 10 €`. Puis mettez `const age = 12;` : `12 < 12` est **faux**, donc
> on obtient `Prix : 7 €`. Les cas « pile sur la limite » méritent toujours une ligne
> dans votre tableau.

> **À retenir :** pour une condition, écrivez le test avec les vraies valeurs
> (`15 < 12`) et son résultat (`false`). Puis suivez **un seul** chemin.

## Programme 3 : une boucle qui accumule

Une [boucle](../debuter/glossaire.md#boucle) répète les mêmes lignes. Ici, on ajoute
le double de `i` à `total`, quatre fois.

```typescript
let total = 0; // L1
for (let i = 1; i <= 4; i++) { // L2
  total = total + i * 2; // L3
  console.log(`i = ${i}, total = ${total}`); // L4
}
console.log(`Fin : ${total}`); // L5
```

**Avant de lancer, devinez ce qui s'affiche.** Combien de lignes ? Quelle est la
dernière ?

```sortie
i = 1, total = 2
i = 2, total = 6
i = 3, total = 12
i = 4, total = 20
Fin : 20
```

La ligne L2 fait trois choses, à trois moments différents :

```text
   for (let i = 1;   i <= 4;   i++)
        └──┬────┘    └──┬─┘    └┬┘
     1. une seule   2. avant    3. à la fin
        fois au       chaque       de chaque
        début         tour         tour
```

Voici le tableau complet. Il est long : c'est normal, une boucle fait beaucoup
d'étapes.

| Étape | Test `i <= 4` | i | total | Affiche |
|---|---|---|---|---|
| L1 | | | 0 | |
| L2 (début) | | 1 | 0 | |
| L2 (test) | `1 <= 4` → `true` | 1 | 0 | |
| L3 | | 1 | 0 + 1 × 2 = **2** | |
| L4 | | 1 | 2 | `i = 1, total = 2` |
| L2 (`i++`, test) | `2 <= 4` → `true` | **2** | 2 | |
| L3 | | 2 | 2 + 2 × 2 = **6** | |
| L4 | | 2 | 6 | `i = 2, total = 6` |
| L2 (`i++`, test) | `3 <= 4` → `true` | **3** | 6 | |
| L3 | | 3 | 6 + 3 × 2 = **12** | |
| L4 | | 3 | 12 | `i = 3, total = 12` |
| L2 (`i++`, test) | `4 <= 4` → `true` | **4** | 12 | |
| L3 | | 4 | 12 + 4 × 2 = **20** | |
| L4 | | 4 | 20 | `i = 4, total = 20` |
| L2 (`i++`, test) | `5 <= 4` → `false` | **5** | 20 | |
| L5 | | | 20 | `Fin : 20` |

Regardez l'avant-dernière ligne : `i` vaut **5**, le test est faux, la boucle
s'arrête. Ce dernier test est l'étape que tout le monde oublie.

`total` se comporte comme une tirelire : à chaque tour, on y glisse une pièce, et on
ne la vide jamais.

> **Essayez :** remplacez `i <= 4` par `i < 4`. La boucle fait un tour de moins. Le
> programme affiche trois lignes, puis `Fin : 12`.

> **À retenir :** dans une boucle, notez le test à **chaque** tour, y compris le
> dernier, celui qui est faux et qui arrête tout.

## Programme 4 : une fonction appelée deux fois

Une [fonction](../debuter/glossaire.md#fonction) est une machine : on lui donne des
valeurs, elle rend un résultat. Ici, `prixRemise` calcule un prix après une réduction
en pourcentage.

```typescript
function prixRemise(prix: number, pourcent: number): number { // L1
  const reduction = (prix * pourcent) / 100; // L2
  return prix - reduction; // L3
}

const livre = prixRemise(20, 10); // L4
const jeu = prixRemise(50, 20); // L5
console.log(livre, jeu, livre + jeu); // L6
```

**Avant de lancer, devinez ce qui s'affiche.**

```sortie
18 40 58
```

Attention au premier piège : les lignes L1 à L3 ne font **rien** quand le programme
les lit la première fois. Elles **décrivent** la machine, sans la mettre en marche.
Le programme commence vraiment à la ligne L4.

Voici qui appelle qui, et ce qui revient :

```text
   programme principal                    prixRemise
   ───────────────────                    ──────────
L4 livre = prixRemise(20, 10)  ────────►  prix = 20, pourcent = 10
                                          L2 reduction = 2
          livre = 18           ◄────────  L3 renvoie 18
L5 jeu = prixRemise(50, 20)    ────────►  prix = 50, pourcent = 20
                                          L2 reduction = 10
          jeu = 40             ◄────────  L3 renvoie 40
L6 console.log(...)
```

Chaque appel ouvre une **nouvelle feuille** de brouillon pour la fonction. Ses
[paramètres](../debuter/glossaire.md#parametre) `prix` et `pourcent` reçoivent les
[arguments](../debuter/glossaire.md#argument) de l'appel. Au `return`, la feuille est
jetée : `reduction` n'existe plus.

| Étape | Où | prix | pourcent | reduction | livre | jeu | Affiche |
|---|---|---|---|---|---|---|---|
| L4 | principal : appel `prixRemise(20, 10)` | | | | | | |
| L1 | dans la fonction | 20 | 10 | | | | |
| L2 | dans la fonction | 20 | 10 | 20 × 10 / 100 = 2 | | | |
| L3 | renvoie 20 − 2 = 18 | 20 | 10 | 2 | | | |
| L4 | principal : on range 18 | | | | **18** | | |
| L5 | principal : appel `prixRemise(50, 20)` | | | | 18 | | |
| L1 | dans la fonction (feuille neuve) | 50 | 20 | | 18 | | |
| L2 | dans la fonction | 50 | 20 | 50 × 20 / 100 = 10 | 18 | | |
| L3 | renvoie 50 − 10 = 40 | 50 | 20 | 10 | 18 | | |
| L5 | principal : on range 40 | | | | 18 | **40** | |
| L6 | principal | | | | 18 | 40 | `18 40 58` |

Les cases vides de `prix`, `pourcent` et `reduction` dans le programme principal ne
sont pas un oubli : ces variables n'existent **que** pendant l'appel.

> **Essayez :** remplacez `prixRemise(20, 10)` par `prixRemise(10, 20)`. L'ordre des
> arguments compte : `prix` vaut maintenant 10 et `pourcent` 20. Le livre coûte `8`,
> et le programme affiche `8 40 48`.

> **À retenir :** pour une fonction, dessinez la flèche aller (les arguments) et la
> flèche retour (la valeur renvoyée). Chaque appel a sa propre feuille, toute neuve.

## Programme 5 : une boucle sur un tableau, avec une condition

Dernier programme : il réunit tout. On parcourt des notes. On compte les notes
réussies (10 ou plus) et on cherche la meilleure.

```typescript
const notes = [12, 8, 15, 9, 18]; // L1
let reussies = 0; // L2
let meilleure = 0; // L3
for (const note of notes) { // L4
  if (note >= 10) { // L5
    reussies = reussies + 1; // L6
  }
  if (note > meilleure) { // L7
    meilleure = note; // L8
  }
}
console.log(`Réussies : ${reussies} sur ${notes.length}`); // L9
console.log(`Meilleure : ${meilleure}`); // L10
```

**Avant de lancer, devinez ce qui s'affiche.**

```sortie
Réussies : 3 sur 5
Meilleure : 18
```

`for...of` prend les éléments du tableau un par un, comme on prend les cartes d'un
paquet, de la première à la dernière.

```text
   notes :  [ 12 | 8 | 15 | 9 | 18 ]
              ▲    ▲    ▲   ▲    ▲
   tour :     1    2    3   4    5      puis plus de carte : fin de la boucle
```

Pour aller plus vite, on regroupe un tour de boucle sur une seule ligne du tableau,
avec ses deux tests :

| Tour | note | `note >= 10` ? | reussies | `note > meilleure` ? | meilleure |
|---|---|---|---|---|---|
| avant | | | 0 | | 0 |
| 1 | 12 | `true` | **1** | `12 > 0` → `true` | **12** |
| 2 | 8 | `false` | 1 | `8 > 12` → `false` | 12 |
| 3 | 15 | `true` | **2** | `15 > 12` → `true` | **15** |
| 4 | 9 | `false` | 2 | `9 > 15` → `false` | 15 |
| 5 | 18 | `true` | **3** | `18 > 15` → `true` | **18** |
| après | | | 3 | | 18 |

Les deux `if` sont **indépendants** : ce n'est pas un `if … else`. À chaque tour, le
programme pose les deux questions.

> **Essayez :** remplacez les notes par `[-3, -1, -7]`. Le programme affiche
> `Meilleure : 0`, alors que 0 n'est pas dans le tableau ! Votre tableau de suivi le
> montre tout de suite : aucun test `note > meilleure` n'est vrai. Pour corriger,
> commencez avec `let meilleure = notes[0];`.

> **À retenir :** pour une boucle sur un tableau, une ligne du tableau de suivi par
> élément. Écrivez chaque test avec les vraies valeurs.

## Faire afficher les étapes au programme

Le tableau sur papier, c'est votre prédiction. Pour vérifier, demandez au programme
d'écrire **lui-même** son tableau, avec `console.log`.

### Des étiquettes dans console.log

Un nombre seul à l'écran ne dit pas d'où il vient. Ajoutez toujours une étiquette.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log("tour", i, "| total =", total);
}
```

```sortie
tour 1 | total = 2
tour 2 | total = 6
tour 3 | total = 12
```

### L'astuce des accolades

Entourez vos variables d'accolades : `console.log({ i, total })`. Node affiche alors
le **nom** et la **valeur** de chaque variable, sans que vous tapiez les étiquettes.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log({ i, total });
}
```

```sortie
{ i: 1, total: 2 }
{ i: 2, total: 6 }
{ i: 3, total: 12 }
```

### Entrer et sortir d'une fonction

Pour une fonction, affichez ce qui entre et ce qui sort. Une flèche aide à lire.

```typescript
function prixRemise(prix: number, pourcent: number): number {
  console.log(`  → entrée : prix = ${prix}, pourcent = ${pourcent}`);
  const resultat = prix - (prix * pourcent) / 100;
  console.log(`  ← sortie : ${resultat}`);
  return resultat;
}

console.log("appel 1");
const livre = prixRemise(20, 10);
console.log("appel 2");
const jeu = prixRemise(50, 20);
console.log("fin :", livre, jeu);
```

```sortie
appel 1
  → entrée : prix = 20, pourcent = 10
  ← sortie : 18
appel 2
  → entrée : prix = 50, pourcent = 20
  ← sortie : 40
fin : 18 40
```

> **Essayez :** dans le programme 5, ajoutez
> `console.log({ note, reussies, meilleure });` à la fin de la boucle. Vous obtenez
> cinq lignes : comparez-les avec votre tableau de suivi.

> **À retenir :** un `console.log` avec une étiquette, c'est un tableau de suivi
> écrit par l'ordinateur. Retirez-les quand le bug est trouvé.

## Le débogueur, en deux phrases

Un [débogueur](../debuter/glossaire.md#debogage) arrête le programme sur une ligne
choisie (un point d'arrêt) et vous laisse avancer d'une ligne à la fois, en montrant
toutes les variables : c'est un tableau de suivi automatique. Dans VS Code, cliquez
dans la marge à gauche d'une ligne, puis lancez « Exécuter et déboguer » ; tout est
expliqué dans [le chapitre sur le débogage](debogage.md#mettre-le-programme-en-pause).

## Erreurs fréquentes

### Oublier le dernier test de la boucle

On pense souvent que `i` vaut 4 après une boucle `for (let i = 1; i <= 4; i++)`. Mais
la boucle s'arrête parce que le test devient faux : `i` vaut alors 5. Un débutant écrit
par exemple :

```typescript norun
let i = 1;
while (i <= 4) {
  i++;
}
console.log(`La boucle a fait ${i} tours`);
```

```text
La boucle a fait 5 tours
```

Le programme ment : il a fait 4 tours. Le tableau de suivi le montre, car la dernière
ligne est `5 <= 4` → `false`. **La correction :** comptez les tours dans une variable à
part.

```typescript
let i = 1;
let tours = 0;
while (i <= 4) {
  tours++;
  i++;
}
console.log(`La boucle a fait ${tours} tours, et i vaut ${i}`);
```

```sortie
La boucle a fait 4 tours, et i vaut 5
```

### Réutiliser une ancienne valeur

Dans le programme 1, beaucoup de personnes calculent la ligne L5 avec `b = 3`, la
valeur du début. Elles trouvent `a = 8 − 3 = 5`. C'est faux : la ligne L4 a déjà mis
`b` à 5.

**La correction :** regardez toujours la ligne **juste au-dessus** dans votre
tableau, jamais la première.

### Se tromper d'un tour (le décalage de un)

Un tableau de 3 notes a les positions 0, 1 et 2. Ce code va une case trop loin :

```typescript norun
const notes = [12, 8, 15];
for (let i = 0; i <= notes.length; i++) {
  console.log(notes[i].toFixed(1));
}
```

`tsc` accepte ce code. Le programme affiche trois notes, puis plante (extrait du
message) :

```text
TypeError: Cannot read properties of undefined (reading 'toFixed')
```

Votre tableau de suivi aurait montré le tour de trop : `i = 3`, test `3 <= 3` →
`true`, et `notes[3]` vaut `undefined`. **La correction :** `<` au lieu de `<=`, ou
mieux, `for...of`.

```typescript
const notes = [12, 8, 15];
for (const note of notes) {
  console.log(note.toFixed(1));
}
```

```sortie
12.0
8.0
15.0
```

### Croire qu'une variable de la fonction existe encore

Dans le programme 4, `reduction` vit sur la feuille de brouillon de la fonction. Après
le `return`, elle a disparu.

```typescript norun
function prixRemise(prix: number, pourcent: number): number {
  const reduction = (prix * pourcent) / 100;
  return prix - reduction;
}

const livre = prixRemise(20, 10);
console.log(livre, reduction);
```

```text
main.mts(7,20): error TS2304: Cannot find name 'reduction'.
```

**La correction :** si vous avez besoin d'une valeur, faites-la **renvoyer** par la
fonction, ou recalculez-la dehors. Le chapitre sur [les fonctions](fonctions.md#la-portee-des-variables)
détaille cette règle de portée.

## Testez-vous

1. Que vaut `x` à la fin ? `let x = 2; x = x * 3; x = x - 1; x = x * x;`

   > **Réponse**
   >
   > `x` vaut 25. Tableau : 2, puis 2 × 3 = 6, puis 6 − 1 = 5, puis 5 × 5 = 25. À
   > chaque ligne, on part de la valeur juste au-dessus.

2. Combien de fois s'exécute `console.log` dans `for (let i = 0; i < 3; i++) { console.log(i); }`,
   et combien de fois le test `i < 3` est-il évalué ?

   > **Réponse**
   >
   > `console.log` s'exécute 3 fois (`i` = 0, 1, 2). Le test est évalué 4 fois : la
   > quatrième fois, `3 < 3` est faux et arrête la boucle.

3. Qu'affiche ce programme ?
   `function f(n: number): number { return n + 1; }` puis `console.log(f(f(1)));`

   > **Réponse**
   >
   > `3`. On calcule d'abord l'appel intérieur : `f(1)` renvoie 2. Puis l'appel
   > extérieur : `f(2)` renvoie 3. Deux appels, deux feuilles de brouillon.

## En résumé

- Un tableau de suivi : une colonne par variable, une ligne par ligne de code
  exécutée.
- Utilisez toujours la valeur la plus récente d'une variable.
- Pour une condition, écrivez le test avec les vraies valeurs et suivez un seul
  chemin.
- Pour une boucle, notez chaque test, y compris le dernier, qui est faux.
- Pour une fonction, dessinez l'aller (arguments) et le retour (valeur renvoyée).
- `console.log({ i, total })` fait écrire son tableau de suivi au programme.

Suite : [Les pièges du débutant en TypeScript](pieges.md)
