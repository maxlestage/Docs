# Exercices pour bien démarrer

Cette page propose 15 exercices très guidés, tous faciles, pour pratiquer les bases de JavaScript : variables, nombres, conditions, boucles, fonctions, texte, tableaux et objets.

## Comment faire ces exercices

Prenez-les dans l'ordre. Chacun ne demande qu'**une** notion nouvelle.

Pour chaque exercice :

1. **Lisez** l'énoncé et la sortie attendue.
2. **Essayez sur papier** : écrivez en français les étapes (« d'abord je crée…, ensuite
   j'affiche… »).
3. **Écrivez** le code dans un fichier `exercice.mjs`.
4. **Lancez-le** avec `node exercice.mjs`.
5. **Comparez** ce qui s'affiche avec la sortie attendue, caractère par caractère.
6. Bloqué ? Lisez **le premier indice**, réessayez. Puis le second. La solution en
   dernier.

```text
 lire → papier → écrire → lancer → comparer
                   ▲                  │
                   └── pas pareil ? ──┘
```

> **Astuce :** une erreur n'est pas un échec : c'est un indice de plus. Lisez le message
> de haut en bas (voir [lire une erreur](../debuter/lire-une-erreur.md)).

## Variables et nombres

### Exercice 1 : Une carte de visite

Niveau : ★☆☆

Créez trois constantes : `prenom`, `ville` et `metier`. Affichez-les sur trois lignes,
avec une étiquette devant chacune.

Sortie attendue :

```text
Prénom : Alice
Ville : Lyon
Métier : boulangère
```

> **Indice :** une constante se crée avec `const prenom = "Alice";`.

> **Indice :** pour mettre une variable dans un texte, utilisez un gabarit avec des
> accents graves : `` `Prénom : ${prenom}` `` (voir [les premiers pas](premiers-pas.md)).

> **Solution**
>
> ```javascript
> const prenom = "Alice";
> const ville = "Lyon";
> const metier = "boulangère";
>
> console.log(`Prénom : ${prenom}`);
> console.log(`Ville : ${ville}`);
> console.log(`Métier : ${metier}`);
> ```
>
> ```sortie
> Prénom : Alice
> Ville : Lyon
> Métier : boulangère
> ```
>
> Pourquoi ça marche : dans un gabarit, `${prenom}` est remplacé par la valeur rangée
> dans la boîte `prenom`.

### Exercice 2 : L'aire d'un rectangle

Niveau : ★☆☆

Un rectangle mesure 7 m de long et 4 m de large. Calculez son aire (longueur × largeur)
et affichez-la.

Sortie attendue :

```text
Aire : 28 m²
```

> **Indice :** rangez la longueur et la largeur dans deux constantes.

> **Indice :** la multiplication s'écrit `*` : `const aire = longueur * largeur;`.

> **Solution**
>
> ```javascript
> const longueur = 7;
> const largeur = 4;
> const aire = longueur * largeur;
>
> console.log(`Aire : ${aire} m²`);
> ```
>
> ```sortie
> Aire : 28 m²
> ```
>
> Pourquoi ça marche : `7 * 4` est calculé une fois, rangé dans `aire`, puis affiché.

### Exercice 3 : Le prix avec la TVA

Niveau : ★☆☆

Un livre coûte 12,50 € hors taxes. La TVA ajoute 5,5 %. Affichez le prix toutes taxes
comprises, avec **deux** chiffres après la virgule.

Sortie attendue :

```text
Prix TTC : 13.19 €
```

> **Indice :** ajouter 5,5 %, c'est multiplier par `1.055`. En JavaScript, la virgule
> s'écrit avec un **point**.

> **Indice :** `nombre.toFixed(2)` arrondit et garde deux chiffres après la virgule
> (voir [les nombres](nombres.md#afficher-avec-tofixed)).

> **Solution**
>
> ```javascript
> const prixHT = 12.5;
> const prixTTC = prixHT * 1.055;
>
> console.log(`Prix TTC : ${prixTTC.toFixed(2)} €`);
> ```
>
> ```sortie
> Prix TTC : 13.19 €
> ```
>
> Pourquoi ça marche : `12.5 * 1.055` vaut `13.1875`. `toFixed(2)` l'arrondit à
> `"13.19"`.

### Exercice 4 : Des minutes en heures

Niveau : ★☆☆

Un film dure 135 minutes. Affichez sa durée en heures et minutes.

Sortie attendue :

```text
2 h 15 min
```

> **Indice :** une heure, c'est 60 minutes. Combien de fois 60 « rentre » dans 135 ?
> C'est `Math.floor(135 / 60)`.

> **Indice :** ce qui reste après les heures, c'est le **reste** de la division :
> `135 % 60`.

> **Solution**
>
> ```javascript
> const duree = 135;
> const heures = Math.floor(duree / 60);
> const minutes = duree % 60;
>
> console.log(`${heures} h ${minutes} min`);
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Pourquoi ça marche : `135 / 60` vaut `2.25` ; `Math.floor` garde `2`. Et
> `135 - 2 × 60` laisse `15`, ce que donne `135 % 60`.

## Conditions et boucles

### Exercice 5 : Majeur ou mineur

Niveau : ★☆☆

Pour chaque âge `15` et `18`, affichez s'il s'agit d'une personne majeure ou mineure.

Sortie attendue :

```text
15 ans : mineur
18 ans : majeur
```

> **Indice :** commencez avec un seul âge : `const age = 15;` et un `if (age >= 18)`.

> **Indice :** pour traiter les deux âges, mettez le `if` dans une boucle
> `for (const age of [15, 18])` (voir [conditions et boucles](conditions-boucles.md)).

> **Solution**
>
> ```javascript
> for (const age of [15, 18]) {
>   if (age >= 18) {
>     console.log(`${age} ans : majeur`);
>   } else {
>     console.log(`${age} ans : mineur`);
>   }
> }
> ```
>
> ```sortie
> 15 ans : mineur
> 18 ans : majeur
> ```
>
> Pourquoi ça marche : `>=` veut dire « plus grand ou égal ». `18 >= 18` est vrai, donc
> 18 ans est majeur.

### Exercice 6 : Le plus grand des deux

Niveau : ★☆☆

Avec `a = 8` et `b = 13`, affichez le plus grand des deux nombres, en une seule ligne
de calcul.

Sortie attendue :

```text
Le plus grand : 13
```

> **Indice :** l'opérateur ternaire s'écrit `condition ? siVrai : siFaux`.

> **Indice :** la condition est `a > b`.

> **Solution**
>
> ```javascript
> const a = 8;
> const b = 13;
> const plusGrand = a > b ? a : b;
>
> console.log(`Le plus grand : ${plusGrand}`);
> ```
>
> ```sortie
> Le plus grand : 13
> ```
>
> Pourquoi ça marche : `8 > 13` est faux, donc on prend la valeur après le `:`, `b`.
> `Math.max(a, b)` donne aussi le bon résultat.

### Exercice 7 : Le compte à rebours

Niveau : ★☆☆

Affichez un compte à rebours de 5 à 1, puis `Partez !`.

Sortie attendue :

```text
5
4
3
2
1
Partez !
```

> **Indice :** une boucle `for` peut **descendre** : on part de 5 et on retire 1 à chaque
> tour avec `i--`.

> **Indice :** la boucle continue tant que `i >= 1`. Le message final se place **après**
> l'accolade fermante de la boucle.

> **Solution**
>
> ```javascript
> for (let i = 5; i >= 1; i--) {
>   console.log(i);
> }
> console.log("Partez !");
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> Partez !
> ```
>
> Pourquoi ça marche : quand `i` vaut 0, `0 >= 1` est faux, la boucle s'arrête. La
> ligne suivante s'exécute une seule fois.

### Exercice 8 : La table de 7

Niveau : ★☆☆

Affichez la table de multiplication de 7, de 1 à 5.

Sortie attendue :

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Indice :** une boucle `for (let i = 1; i <= 5; i++)` donne `i` = 1, 2, 3, 4, 5.

> **Indice :** dans le gabarit, le calcul peut se faire directement : `${7 * i}`.

> **Solution**
>
> ```javascript
> for (let i = 1; i <= 5; i++) {
>   console.log(`7 x ${i} = ${7 * i}`);
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Pourquoi ça marche : à chaque tour, `i` change, et la même ligne affiche un nouveau
> calcul.

### Exercice 9 : La somme de 1 à 10

Niveau : ★☆☆

Additionnez tous les nombres de 1 à 10 avec une boucle, puis affichez le total.

Sortie attendue :

```text
Total : 55
```

> **Indice :** créez une boîte `total` qui vaut `0` **avant** la boucle. Elle doit
> pouvoir changer : utilisez `let`.

> **Indice :** à chaque tour, ajoutez `i` dans la boîte : `total = total + i;` (ou
> `total += i;`). Voici les premiers tours :
>
> | Étape | i | total |
> | --- | --- | --- |
> | départ | – | 0 |
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solution**
>
> ```javascript
> let total = 0;
> for (let i = 1; i <= 10; i++) {
>   total += i;
> }
> console.log(`Total : ${total}`);
> ```
>
> ```sortie
> Total : 55
> ```
>
> Pourquoi ça marche : `total` est créé hors de la boucle, donc il garde sa valeur d'un
> tour à l'autre. S'il était créé dedans, il repartirait de 0 à chaque tour.

## Fonctions

### Exercice 10 : Dire bonjour

Niveau : ★☆☆

Écrivez une fonction `direBonjour(prenom)` qui **renvoie** `Bonjour, <prénom> !`.
Appelez-la avec `"Alice"` puis `"Bob"`, et affichez les résultats.

Sortie attendue :

```text
Bonjour, Alice !
Bonjour, Bob !
```

> **Indice :** une fonction s'écrit `function direBonjour(prenom) { ... }`.

> **Indice :** à l'intérieur, `return` renvoie le texte. À l'extérieur,
> `console.log(direBonjour("Alice"))` l'affiche (voir [les fonctions](fonctions.md)).

> **Solution**
>
> ```javascript
> function direBonjour(prenom) {
>   return `Bonjour, ${prenom} !`;
> }
>
> console.log(direBonjour("Alice"));
> console.log(direBonjour("Bob"));
> ```
>
> ```sortie
> Bonjour, Alice !
> Bonjour, Bob !
> ```
>
> Pourquoi ça marche : le paramètre `prenom` reçoit `"Alice"` au premier appel, puis
> `"Bob"` au second. La même recette sert deux fois.

### Exercice 11 : Le carré d'un nombre

Niveau : ★☆☆

Écrivez une fonction fléchée `carre(n)` qui renvoie `n × n`. Affichez le carré de 3,
puis la somme des carrés de 3 et de 4.

Sortie attendue :

```text
9
25
```

> **Indice :** une fonction fléchée courte s'écrit `const carre = (n) => n * n;`.

> **Indice :** le résultat d'une fonction est une valeur : on peut additionner
> `carre(3) + carre(4)`.

> **Solution**
>
> ```javascript
> const carre = (n) => n * n;
>
> console.log(carre(3));
> console.log(carre(3) + carre(4));
> ```
>
> ```sortie
> 9
> 25
> ```
>
> Pourquoi ça marche : sans accolades, une flèche renvoie automatiquement son calcul.
> `carre(3) + carre(4)` devient `9 + 16`.

## Texte, tableaux et objets

### Exercice 12 : Crier un mot

Niveau : ★☆☆

Avec le mot `"bonjour"`, affichez-le en majuscules, suivi de son nombre de lettres.

Sortie attendue :

```text
BONJOUR (7 lettres)
```

> **Indice :** `mot.toUpperCase()` met en majuscules.

> **Indice :** `mot.length` donne le nombre de caractères. Attention : **sans**
> parenthèses (voir [le texte](texte.md)).

> **Solution**
>
> ```javascript
> const mot = "bonjour";
> console.log(`${mot.toUpperCase()} (${mot.length} lettres)`);
> ```
>
> ```sortie
> BONJOUR (7 lettres)
> ```
>
> Pourquoi ça marche : `toUpperCase()` renvoie un **nouveau** texte en majuscules ;
> `mot` lui-même ne change pas.

### Exercice 13 : Les initiales

Niveau : ★☆☆

Avec le nom complet `"Ada Lovelace"`, affichez ses initiales.

Sortie attendue :

```text
A.L.
```

> **Indice :** `nom.split(" ")` coupe le texte aux espaces et donne un tableau :
> `["Ada", "Lovelace"]`.

> **Indice :** la première lettre d'un mot `m` est `m[0]`.

> **Solution**
>
> ```javascript
> const nom = "Ada Lovelace";
> const [prenom, famille] = nom.split(" ");
>
> console.log(`${prenom[0]}.${famille[0]}.`);
> ```
>
> ```sortie
> A.L.
> ```
>
> Pourquoi ça marche : `split` donne deux mots, rangés dans `prenom` et `famille`.
> L'indice `[0]` prend le premier caractère de chacun.

### Exercice 14 : La moyenne des notes

Niveau : ★☆☆

Avec le tableau `[12, 15, 9, 16]`, calculez la moyenne des notes et affichez-la.

Sortie attendue :

```text
Moyenne : 13
```

> **Indice :** d'abord la somme, comme dans l'exercice 9, mais avec
> `for (const note of notes)`.

> **Indice :** la moyenne, c'est la somme divisée par le nombre de notes :
> `notes.length` (voir [les tableaux](collections.md)).

> **Solution**
>
> ```javascript
> const notes = [12, 15, 9, 16];
>
> let somme = 0;
> for (const note of notes) {
>   somme += note;
> }
>
> console.log(`Moyenne : ${somme / notes.length}`);
> ```
>
> ```sortie
> Moyenne : 13
> ```
>
> Pourquoi ça marche : la somme vaut `52`, il y a `4` notes, et `52 / 4` fait `13`.

### Exercice 15 : La fiche d'un livre

Niveau : ★☆☆

Créez un objet `livre` avec un `titre` (`"Le Petit Prince"`), un `auteur`
(`"Saint-Exupéry"`) et une `annee` (`1943`). Affichez chaque propriété sur une ligne.

Sortie attendue :

```text
titre : Le Petit Prince
auteur : Saint-Exupéry
annee : 1943
```

> **Indice :** un objet s'écrit entre accolades : `{ titre: "...", auteur: "...", ... }`.

> **Indice :** `Object.entries(livre)` donne la liste des paires `[clé, valeur]`.
> Parcourez-la avec `for (const [cle, valeur] of ...)` (voir
> [les objets](objets.md#parcourir-un-objet)).

> **Solution**
>
> ```javascript
> const livre = {
>   titre: "Le Petit Prince",
>   auteur: "Saint-Exupéry",
>   annee: 1943,
> };
>
> for (const [cle, valeur] of Object.entries(livre)) {
>   console.log(`${cle} : ${valeur}`);
> }
> ```
>
> ```sortie
> titre : Le Petit Prince
> auteur : Saint-Exupéry
> annee : 1943
> ```
>
> Pourquoi ça marche : `Object.entries` transforme l'objet en tableau de paires, dans
> l'ordre où les propriétés ont été écrites. Chaque paire est décomposée en `cle` et
> `valeur`.

## Et ensuite ?

Bravo ! Si vous avez fait ces 15 exercices, vous maîtrisez les bases. D'autres
exercices, plus variés, vous attendent dans [les exercices corrigés](exercices.md).

Suite : [Les classes](types.md).
