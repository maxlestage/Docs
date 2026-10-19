# Exercices pour bien démarrer

Cette page propose 15 petits exercices très guidés, tous faciles, pour pratiquer les bases de TypeScript : variables, nombres, conditions, boucles, fonctions, texte, tableaux et objets.

## Comment faire ces exercices

Chaque exercice se fait en cinq étapes :

1. **Lisez** l'énoncé et la sortie attendue.
2. **Essayez sur papier** : écrivez les étapes en français, comme une recette.
3. **Écrivez** le code dans un fichier `exercice.ts`, puis **lancez-le**
   (`npx tsx exercice.ts`, voir [l'introduction](introduction.md)).
4. **Comparez** ce qui s'affiche avec la sortie attendue, caractère par caractère.
5. Bloqué ? Lisez le **premier indice**, puis le second. Ouvrez la solution en dernier.

> **Astuce :** une erreur de `tsc` n'est pas un échec. C'est un conseil. Lisez-la ligne
> par ligne, comme dans [les premiers pas](premiers-pas.md#lire-une-erreur-de-type-ligne-par-ligne).

Tous les exercices sont de niveau ★☆☆. Les solutions passent `tsc --strict`.

## Variables et nombres

### Exercice 1 : Une carte de visite

Niveau : ★☆☆

Créez trois constantes : `prenom`, `ville` et `age`. Affichez une phrase avec les
trois.

Sortie attendue :

```text
Je m'appelle Alice, j'habite à Lyon et j'ai 30 ans.
```

> **Indice :** créez une variable avec `const prenom = "Alice";`.

> **Indice :** pour insérer une variable dans un texte, utilisez un gabarit entre
> accents graves : `` `Bonjour ${prenom}` ``.

> **Solution**
>
> ```typescript
> const prenom = "Alice";
> const ville = "Lyon";
> const age = 30;
> console.log(`Je m'appelle ${prenom}, j'habite à ${ville} et j'ai ${age} ans.`);
> ```
>
> ```sortie
> Je m'appelle Alice, j'habite à Lyon et j'ai 30 ans.
> ```
>
> Pourquoi ça marche : chaque `${...}` est remplacé par la valeur de la variable.
> TypeScript devine seul que `age` est un `number`.

### Exercice 2 : L'aire d'un rectangle

Niveau : ★☆☆

Un rectangle mesure 8 mètres de long et 5 mètres de large. Calculez son aire
(longueur × largeur) et son périmètre (2 × (longueur + largeur)).

Sortie attendue :

```text
Aire : 40 m²
Périmètre : 26 m
```

> **Indice :** la multiplication s'écrit `*`.

> **Indice :** pour le périmètre, mettez l'addition entre parenthèses :
> `2 * (longueur + largeur)`.

> **Solution**
>
> ```typescript
> const longueur = 8;
> const largeur = 5;
> const aire = longueur * largeur;
> const perimetre = 2 * (longueur + largeur);
> console.log(`Aire : ${aire} m²`);
> console.log(`Périmètre : ${perimetre} m`);
> ```
>
> ```sortie
> Aire : 40 m²
> Périmètre : 26 m
> ```
>
> Pourquoi ça marche : les parenthèses font calculer `8 + 5` d'abord (13), puis
> `2 * 13`. Sans elles, on aurait `2 * 8 + 5`, soit 21.

### Exercice 3 : Des minutes en heures

Niveau : ★☆☆

Un film dure 137 minutes. Affichez sa durée en heures et minutes.

Sortie attendue :

```text
137 minutes = 2 h 17 min
```

> **Indice :** le nombre d'heures, c'est `137 / 60` arrondi vers le bas :
> `Math.floor(...)`.

> **Indice :** les minutes qui restent, c'est le reste de la division : `137 % 60`.

> **Solution**
>
> ```typescript
> const duree = 137;
> const heures = Math.floor(duree / 60);
> const minutes = duree % 60;
> console.log(`${duree} minutes = ${heures} h ${minutes} min`);
> ```
>
> ```sortie
> 137 minutes = 2 h 17 min
> ```
>
> Pourquoi ça marche : `137 / 60` vaut `2.28…`, arrondi à `2`. Et `2 × 60 = 120`, donc
> il reste `137 - 120 = 17` minutes. Voir [nombres et calculs](nombres.md).

### Exercice 4 : Le prix avec la TVA

Niveau : ★☆☆

Un livre coûte 12.5 € hors taxes. La TVA est de 5.5 %. Affichez le prix TTC avec deux
chiffres après la virgule.

Sortie attendue :

```text
Prix TTC : 13.19 €
```

> **Indice :** 5.5 % s'écrit `0.055` dans un calcul. Le prix TTC est
> `prixHT * (1 + 0.055)`.

> **Indice :** `nombre.toFixed(2)` donne un texte avec deux décimales.

> **Solution**
>
> ```typescript
> const prixHT = 12.5;
> const tva = 0.055;
> const prixTTC = prixHT * (1 + tva);
> console.log(`Prix TTC : ${prixTTC.toFixed(2)} €`);
> ```
>
> ```sortie
> Prix TTC : 13.19 €
> ```
>
> Pourquoi ça marche : le calcul donne `13.1875`. `toFixed(2)` l'arrondit à `13.19`
> pour l'affichage.

## Conditions et boucles

### Exercice 5 : Majeur ou mineur

Niveau : ★☆☆

Pour chaque âge de la liste `[12, 18, 45]`, affichez s'il s'agit d'une personne majeure
(18 ans ou plus) ou mineure.

Sortie attendue :

```text
12 ans : mineur
18 ans : majeur
45 ans : majeur
```

> **Indice :** parcourez la liste avec `for (const age of ages) { ... }`.

> **Indice :** dans la boucle, testez `if (age >= 18)`. Attention : `>=` et non `>`,
> car 18 ans, c'est déjà majeur.

> **Solution**
>
> ```typescript
> const ages = [12, 18, 45];
> for (const age of ages) {
>   if (age >= 18) {
>     console.log(`${age} ans : majeur`);
>   } else {
>     console.log(`${age} ans : mineur`);
>   }
> }
> ```
>
> ```sortie
> 12 ans : mineur
> 18 ans : majeur
> 45 ans : majeur
> ```
>
> Pourquoi ça marche : la boucle prend chaque âge à son tour. Le `if` choisit une des
> deux phrases.

### Exercice 6 : Le compte à rebours

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

> **Indice :** une boucle `for` peut descendre : `for (let i = 5; i >= 1; i--)`.

> **Solution**
>
> ```typescript
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
> Pourquoi ça marche : `i` commence à 5. Après chaque tour, `i--` enlève 1. La boucle
> s'arrête quand `i >= 1` devient faux, donc quand `i` vaut 0. Le dernier
> `console.log` est **après** la boucle : il ne s'affiche qu'une fois.

### Exercice 7 : La table de 7

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

> **Indice :** dans la boucle, affichez `` `7 x ${i} = ${7 * i}` ``.

> **Solution**
>
> ```typescript
> const table = 7;
> for (let i = 1; i <= 5; i++) {
>   console.log(`${table} x ${i} = ${table * i}`);
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
> Pourquoi ça marche : `<=` inclut le 5. Avec `<`, la boucle s'arrêterait à 4. Changez
> `table` pour obtenir une autre table.

### Exercice 8 : Additionner une liste

Niveau : ★☆☆

Calculez la somme des dépenses `[12, 5, 30, 8]` avec une boucle.

Sortie attendue :

```text
Total : 55 €
```

> **Indice :** créez une variable `let total = 0;` avant la boucle.

> **Indice :** dans la boucle `for...of`, ajoutez chaque dépense : `total += depense;`.

> **Solution**
>
> ```typescript
> const depenses = [12, 5, 30, 8];
> let total = 0;
> for (const depense of depenses) {
>   total += depense;
> }
> console.log(`Total : ${total} €`);
> ```
>
> ```sortie
> Total : 55 €
> ```
>
> Pourquoi ça marche : `total` est une tirelire qui grossit à chaque tour.
>
> | Tour | depense | total |
> |---|---|---|
> | début | | 0 |
> | 1 | 12 | 12 |
> | 2 | 5 | 17 |
> | 3 | 30 | 47 |
> | 4 | 8 | 55 |

## Fonctions

### Exercice 9 : Le carré d'un nombre

Niveau : ★☆☆

Écrivez une fonction `carre` qui reçoit un nombre et renvoie ce nombre multiplié par
lui-même. Affichez le carré de 3 et de 12.

Sortie attendue :

```text
9
144
```

> **Indice :** la forme d'une fonction typée :
> `function carre(n: number): number { ... }`.

> **Indice :** dans la fonction, écrivez `return n * n;`.

> **Solution**
>
> ```typescript
> function carre(n: number): number {
>   return n * n;
> }
> console.log(carre(3));
> console.log(carre(12));
> ```
>
> ```sortie
> 9
> 144
> ```
>
> Pourquoi ça marche : `carre(3)` met `3` dans `n`, calcule `3 * 3` et renvoie `9`.
> `console.log` affiche ce qui est renvoyé.

### Exercice 10 : Saluer avec une valeur par défaut

Niveau : ★☆☆

Écrivez une fonction `saluer` qui reçoit un prénom et renvoie `Bonjour, <prénom> !`.
Sans prénom, elle doit dire `Bonjour, inconnu !`.

Sortie attendue :

```text
Bonjour, Léa !
Bonjour, inconnu !
```

> **Indice :** une valeur par défaut s'écrit avec `=` dans les paramètres :
> `function saluer(prenom = "inconnu")`.

> **Solution**
>
> ```typescript
> function saluer(prenom: string = "inconnu"): string {
>   return `Bonjour, ${prenom} !`;
> }
> console.log(saluer("Léa"));
> console.log(saluer());
> ```
>
> ```sortie
> Bonjour, Léa !
> Bonjour, inconnu !
> ```
>
> Pourquoi ça marche : quand on appelle `saluer()` sans argument, `prenom` prend la
> valeur par défaut. Voir [les fonctions](fonctions.md).

### Exercice 11 : Pair ou impair

Niveau : ★☆☆

Écrivez une fonction `estPair` qui renvoie `true` si un nombre est pair, `false`
sinon. Testez-la sur 4 et 7.

Sortie attendue :

```text
4 est pair : true
7 est pair : false
```

> **Indice :** un nombre est pair si le reste de sa division par 2 vaut 0 : `n % 2`.

> **Indice :** la comparaison `n % 2 === 0` donne déjà `true` ou `false`. Vous pouvez
> la renvoyer directement.

> **Solution**
>
> ```typescript
> function estPair(n: number): boolean {
>   return n % 2 === 0;
> }
> console.log(`4 est pair : ${estPair(4)}`);
> console.log(`7 est pair : ${estPair(7)}`);
> ```
>
> ```sortie
> 4 est pair : true
> 7 est pair : false
> ```
>
> Pourquoi ça marche : `4 % 2` vaut `0`, donc `0 === 0` est vrai. `7 % 2` vaut `1`, donc
> la comparaison est fausse.

## Texte

### Exercice 12 : Les initiales

Niveau : ★☆☆

À partir de `prenom = "ada"` et `nom = "lovelace"`, affichez les initiales en
majuscules, suivies de points.

Sortie attendue :

```text
A.L.
```

> **Indice :** le premier caractère d'un texte s'obtient avec `texte[0]`, ou mieux
> `texte.charAt(0)`, qui donne toujours un `string`.

> **Indice :** `.toUpperCase()` met un texte en majuscules.

> **Solution**
>
> ```typescript
> const prenom = "ada";
> const nom = "lovelace";
> const initiales = `${prenom.charAt(0)}.${nom.charAt(0)}.`.toUpperCase();
> console.log(initiales);
> ```
>
> ```sortie
> A.L.
> ```
>
> Pourquoi ça marche : on construit d'abord `"a.l."`, puis `toUpperCase` le transforme
> en `"A.L."`. Les points ne changent pas.

### Exercice 13 : Compter les mots

Niveau : ★☆☆

Comptez les mots de la phrase `"le chat dort sur le canapé"` et affichez le plus long.

Sortie attendue :

```text
6 mots
Le plus long : canapé
```

> **Indice :** `phrase.split(" ")` coupe la phrase à chaque espace et donne un tableau
> de mots. Sa longueur est `.length`.

> **Indice :** pour le plus long, gardez une variable `let plusLong = "";` et
> remplacez-la dans une boucle quand un mot est plus long qu'elle.

> **Solution**
>
> ```typescript
> const phrase = "le chat dort sur le canapé";
> const mots = phrase.split(" ");
> console.log(`${mots.length} mots`);
>
> let plusLong = "";
> for (const mot of mots) {
>   if (mot.length > plusLong.length) {
>     plusLong = mot;
>   }
> }
> console.log(`Le plus long : ${plusLong}`);
> ```
>
> ```sortie
> 6 mots
> Le plus long : canapé
> ```
>
> Pourquoi ça marche : `plusLong` garde le champion du moment. `"le"` remplace `""`,
> puis `"chat"` remplace `"le"`, puis seul `"canapé"` (6 lettres) fait mieux.

## Tableaux et objets

### Exercice 14 : Garder les bonnes notes

Niveau : ★☆☆

À partir des notes `[8, 15, 11, 6, 19]`, gardez seulement celles d'au moins 10, et
affichez-les avec leur nombre.

Sortie attendue :

```text
[ 15, 11, 19 ]
3 notes au-dessus de la moyenne
```

> **Indice :** `filter` garde les éléments pour lesquels une fonction répond `true` :
> `notes.filter((n) => ...)`.

> **Indice :** la condition est `n >= 10`.

> **Solution**
>
> ```typescript
> const notes = [8, 15, 11, 6, 19];
> const bonnes = notes.filter((n) => n >= 10);
> console.log(bonnes);
> console.log(`${bonnes.length} notes au-dessus de la moyenne`);
> ```
>
> ```sortie
> [ 15, 11, 19 ]
> 3 notes au-dessus de la moyenne
> ```
>
> Pourquoi ça marche : `filter` pose la question `n >= 10` à chaque note et garde
> celles qui répondent oui. Le tableau `notes` n'est pas modifié. Voir
> [les tableaux](collections.md).

### Exercice 15 : La fiche d'un livre

Niveau : ★☆☆

Écrivez une interface `Livre` avec un `titre` (texte) et un nombre de `pages`. Créez
deux livres dans un tableau, puis affichez une ligne par livre.

Sortie attendue :

```text
Dune : 600 pages
Le Petit Prince : 96 pages
```

> **Indice :** une interface s'écrit
> `interface Livre { titre: string; pages: number; }`.

> **Indice :** le tableau a le type `Livre[]`. Parcourez-le avec `for...of`.

> **Solution**
>
> ```typescript
> interface Livre {
>   titre: string;
>   pages: number;
> }
>
> const livres: Livre[] = [
>   { titre: "Dune", pages: 600 },
>   { titre: "Le Petit Prince", pages: 96 },
> ];
>
> for (const livre of livres) {
>   console.log(`${livre.titre} : ${livre.pages} pages`);
> }
> ```
>
> ```sortie
> Dune : 600 pages
> Le Petit Prince : 96 pages
> ```
>
> Pourquoi ça marche : l'interface est le modèle de la fiche. TypeScript vérifie que
> chaque livre a bien un `titre` et des `pages`. Voir [les objets](objets.md).

Bravo ! Pour aller plus loin, essayez les [exercices corrigés](exercices.md) de niveau
★★☆.

Suite : [les unions](unions.md).
