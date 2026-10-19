# Exercices corrigés : deuxième série

Cette page propose 12 nouveaux exercices corrigés en TypeScript, du plus facile au plus difficile, avec des indices, des solutions expliquées et des tableaux de suivi.

## Comment utiliser cette page

Ces exercices sont **nouveaux** : ils ne reprennent ni [les exercices pour bien
démarrer](exercices-faciles.md), ni [la première série](exercices.md). Chacun a un
niveau :

- ★☆☆ : facile, avec les bases seulement ;
- ★★☆ : moyen, avec les tableaux, les objets et les types ;
- ★★★ : plus dur, avec les génériques, les classes et les unions.

Pour chaque exercice :

1. Lisez l'énoncé et la sortie attendue.
2. Faites les étapes sur papier, avec un [tableau de suivi](pas-a-pas.md).
3. Écrivez votre code dans `exercice.ts`, vérifiez-le avec
   `npx tsc --noEmit --strict exercice.ts`, puis lancez-le avec `npx tsx exercice.ts`.
4. Bloqué ? Lisez l'indice. Ouvrez la solution en dernier.

> **Astuce :** une solution différente de la nôtre peut être juste. Ce qui compte : la
> bonne sortie, et aucune erreur de `tsc --strict`.

## Premiers pas

### Exercice 1 : Celsius en Fahrenheit

Niveau : ★☆☆

Écrivez une fonction `enFahrenheit(celsius: number): number`. La formule est :
Fahrenheit = Celsius × 9 / 5 + 32. Affichez la conversion de 0, 20, 37 et 100 degrés.

Sortie attendue :

```text
0 °C = 32 °F
20 °C = 68 °F
37 °C = 98.6 °F
100 °C = 212 °F
```

> **Indice :** écrivez d'abord la fonction et testez-la sur `0` (le résultat doit être
> `32`). Puis parcourez `[0, 20, 37, 100]` avec `for...of`.

> **Solution**
>
> ```typescript
> function enFahrenheit(celsius: number): number {
>   return (celsius * 9) / 5 + 32;
> }
>
> for (const c of [0, 20, 37, 100]) {
>   console.log(`${c} °C = ${enFahrenheit(c)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 37 °C = 98.6 °F
> 100 °C = 212 °F
> ```
>
> Pourquoi ça marche : la fonction est une petite machine à convertir. La boucle lui
> donne chaque température, une à la fois.
>
> | Tour | c | c × 9 | ÷ 5 | + 32 |
> |---|---|---|---|---|
> | 1 | 0 | 0 | 0 | 32 |
> | 2 | 20 | 180 | 36 | 68 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180 | 212 |

### Exercice 2 : Le plus grand des trois

Niveau : ★☆☆

Écrivez une fonction `plusGrand(a: number, b: number, c: number): number` qui renvoie
le plus grand des trois nombres, **sans** utiliser `Math.max`. Testez-la avec
`(4, 9, 2)`, `(7, 3, 5)` et `(1, 1, 8)`.

Sortie attendue :

```text
9
7
8
```

> **Indice :** rangez `a` dans une variable `max`. Puis comparez `b` à `max`, puis `c`
> à `max`, avec deux `if`.

> **Solution**
>
> ```typescript
> function plusGrand(a: number, b: number, c: number): number {
>   let max = a;
>   if (b > max) max = b;
>   if (c > max) max = c;
>   return max;
> }
>
> console.log(plusGrand(4, 9, 2));
> console.log(plusGrand(7, 3, 5));
> console.log(plusGrand(1, 1, 8));
> ```
>
> ```sortie
> 9
> 7
> 8
> ```
>
> Pourquoi ça marche : `max` est le champion du moment. Chaque nouveau nombre le défie ;
> s'il est plus grand, il prend sa place.

### Exercice 3 : Écrire un mot à l'envers

Niveau : ★☆☆

Écrivez une fonction `aLEnvers(mot: string): string` qui renvoie le mot écrit de droite
à gauche, avec une boucle (sans `reverse`). Testez-la avec `"radis"`.

Sortie attendue :

```text
sidar
```

> **Indice :** partez d'un texte vide `let resultat = "";`. Parcourez les lettres avec
> `for (const lettre of mot)`.

> **Indice :** collez chaque lettre **devant** le résultat : `resultat = lettre + resultat;`.

> **Solution**
>
> ```typescript
> function aLEnvers(mot: string): string {
>   let resultat = "";
>   for (const lettre of mot) {
>     resultat = lettre + resultat;
>   }
>   return resultat;
> }
>
> console.log(aLEnvers("radis"));
> ```
>
> ```sortie
> sidar
> ```
>
> Pourquoi ça marche : chaque nouvelle lettre passe **devant** les autres, comme une
> assiette posée sur une pile.
>
> | Tour | lettre | resultat |
> |---|---|---|
> | début | | `""` |
> | 1 | r | `"r"` |
> | 2 | a | `"ar"` |
> | 3 | d | `"dar"` |
> | 4 | i | `"idar"` |
> | 5 | s | `"sidar"` |

### Exercice 4 : La somme des chiffres

Niveau : ★☆☆

Écrivez une fonction `sommeChiffres(n: number): number` qui additionne les chiffres
d'un entier positif : pour 4721, c'est 4 + 7 + 2 + 1 = 14. N'utilisez pas de texte :
seulement des calculs.

Sortie attendue :

```text
14
```

> **Indice :** `n % 10` donne le dernier chiffre (4721 % 10 = 1).

> **Indice :** `Math.floor(n / 10)` enlève le dernier chiffre (4721 → 472). Répétez
> avec `while (n > 0)`.

> **Solution**
>
> ```typescript
> function sommeChiffres(n: number): number {
>   let somme = 0;
>   while (n > 0) {
>     somme += n % 10;
>     n = Math.floor(n / 10);
>   }
>   return somme;
> }
>
> console.log(sommeChiffres(4721));
> ```
>
> ```sortie
> 14
> ```
>
> Pourquoi ça marche : on « mange » le nombre par la droite, un chiffre par tour.
>
> | Tour | n au début | n % 10 | somme | n à la fin |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> Au tour suivant, le test `0 > 0` est faux : la boucle s'arrête.

## Tableaux, objets et types

### Exercice 5 : Le palindrome

Niveau : ★★☆

Un palindrome se lit pareil dans les deux sens, si l'on ignore les majuscules et les
espaces : « Esope reste ici et se repose ». Écrivez
`estPalindrome(phrase: string): boolean` et testez-la sur trois phrases.

Sortie attendue :

```text
kayak : oui
Esope reste ici et se repose : oui
TypeScript : non
```

> **Indice :** nettoyez d'abord la phrase : `toLowerCase()`, puis
> `replaceAll(" ", "")`.

> **Indice :** pour retourner un texte : `[...texte].reverse().join("")`.

> **Solution**
>
> ```typescript
> function estPalindrome(phrase: string): boolean {
>   const propre = phrase.toLowerCase().replaceAll(" ", "");
>   const inverse = [...propre].reverse().join("");
>   return propre === inverse;
> }
>
> for (const p of ["kayak", "Esope reste ici et se repose", "TypeScript"]) {
>   console.log(`${p} : ${estPalindrome(p) ? "oui" : "non"}`);
> }
> ```
>
> ```sortie
> kayak : oui
> Esope reste ici et se repose : oui
> TypeScript : non
> ```
>
> Pourquoi ça marche : on compare la phrase nettoyée avec son reflet dans un miroir.
> `[...propre]` découpe le texte en un tableau de lettres, `reverse` le retourne,
> `join("")` recolle les lettres.

### Exercice 6 : Compter les lettres

Niveau : ★★☆

Écrivez une fonction `compterLettres(mot: string): Record<string, number>` qui compte
combien de fois chaque lettre apparaît. Affichez le résultat pour `"banane"`, une
lettre par ligne.

Sortie attendue :

```text
b : 1
a : 2
n : 2
e : 1
```

> **Indice :** un objet `Record<string, number>` sert de carnet : la lettre est la clé,
> le nombre est la valeur. Commencez avec `{}`.

> **Indice :** `compte[lettre] = (compte[lettre] ?? 0) + 1;` gère la première
> rencontre, quand la case n'existe pas encore.

> **Solution**
>
> ```typescript
> function compterLettres(mot: string): Record<string, number> {
>   const compte: Record<string, number> = {};
>   for (const lettre of mot) {
>     compte[lettre] = (compte[lettre] ?? 0) + 1;
>   }
>   return compte;
> }
>
> const resultat = compterLettres("banane");
> for (const [lettre, nombre] of Object.entries(resultat)) {
>   console.log(`${lettre} : ${nombre}`);
> }
> ```
>
> ```sortie
> b : 1
> a : 2
> n : 2
> e : 1
> ```
>
> Pourquoi ça marche : la première fois qu'on voit une lettre, sa case vaut
> `undefined`, et `?? 0` la remplace par 0.
>
> | Tour | lettre | compte après le tour |
> |---|---|---|
> | 1 | b | `{ b: 1 }` |
> | 2 | a | `{ b: 1, a: 1 }` |
> | 3 | n | `{ b: 1, a: 1, n: 1 }` |
> | 4 | a | `{ b: 1, a: 2, n: 1 }` |
> | 5 | n | `{ b: 1, a: 2, n: 2 }` |
> | 6 | e | `{ b: 1, a: 2, n: 2, e: 1 }` |

### Exercice 7 : Le minimum et le maximum

Niveau : ★★☆

Écrivez une fonction `bornes(nombres: readonly number[])` qui renvoie un objet
`{ min, max }` en **un seul** parcours du tableau, ou `undefined` si le tableau est
vide. Affichez les bornes des températures `[14, 9, 21, 17, 6]`, puis celles d'un
tableau vide.

Sortie attendue :

```text
min = 6, max = 21
tableau vide
```

> **Indice :** le type de retour est `{ min: number; max: number } | undefined`.

> **Indice :** traitez le cas vide en premier avec un `return`. Ensuite, partez de
> `nombres[0]` pour `min` et `max`.

> **Solution**
>
> ```typescript
> type Bornes = { min: number; max: number };
>
> function bornes(nombres: readonly number[]): Bornes | undefined {
>   if (nombres.length === 0) return undefined;
>   let min = nombres[0];
>   let max = nombres[0];
>   for (const n of nombres) {
>     if (n < min) min = n;
>     if (n > max) max = n;
>   }
>   return { min, max };
> }
>
> for (const liste of [[14, 9, 21, 17, 6], []]) {
>   const b = bornes(liste);
>   console.log(b === undefined ? "tableau vide" : `min = ${b.min}, max = ${b.max}`);
> }
> ```
>
> ```sortie
> min = 6, max = 21
> tableau vide
> ```
>
> Pourquoi ça marche : `min` et `max` sont deux champions qui gardent leur titre tant
> que personne ne les bat. Le type `Bornes | undefined` oblige l'appelant à tester le
> cas vide : sans le test `b === undefined`, `b.min` serait refusé par `tsc`.

### Exercice 8 : Ranger les courses par rayon

Niveau : ★★☆

Une liste de courses contient des articles. Chacun a un `nom` et un `rayon` : `"frais"`,
`"epicerie"` ou `"boissons"`. Écrivez une fonction `parRayon` qui renvoie, pour chaque
rayon, la liste des noms. Affichez une ligne par rayon.

Sortie attendue :

```text
frais : yaourt, beurre
epicerie : pâtes, riz
boissons : jus
```

> **Indice :** décrivez le type `Rayon` comme une union de textes, et le résultat
> comme `Record<Rayon, string[]>`.

> **Indice :** créez le résultat avec trois tableaux vides, puis faites
> `resultat[article.rayon].push(article.nom)` pour chaque article.

> **Solution**
>
> ```typescript
> type Rayon = "frais" | "epicerie" | "boissons";
>
> interface Article {
>   nom: string;
>   rayon: Rayon;
> }
>
> function parRayon(articles: readonly Article[]): Record<Rayon, string[]> {
>   const resultat: Record<Rayon, string[]> = { frais: [], epicerie: [], boissons: [] };
>   for (const article of articles) {
>     resultat[article.rayon].push(article.nom);
>   }
>   return resultat;
> }
>
> const courses: Article[] = [
>   { nom: "yaourt", rayon: "frais" },
>   { nom: "pâtes", rayon: "epicerie" },
>   { nom: "jus", rayon: "boissons" },
>   { nom: "beurre", rayon: "frais" },
>   { nom: "riz", rayon: "epicerie" },
> ];
>
> const ranges = parRayon(courses);
> for (const [rayon, noms] of Object.entries(ranges)) {
>   console.log(`${rayon} : ${noms.join(", ")}`);
> }
> ```
>
> ```sortie
> frais : yaourt, beurre
> epicerie : pâtes, riz
> boissons : jus
> ```
>
> Pourquoi ça marche : le résultat est un meuble à trois tiroirs, un par rayon. Chaque
> article tombe dans le tiroir écrit sur son étiquette. Grâce au type `Rayon`, une
> faute de frappe comme `rayon: "fras"` est refusée par `tsc`.

### Exercice 9 : Vérifier un mot de passe

Niveau : ★★☆

Écrivez une fonction `problemes(motDePasse: string): string[]` qui renvoie la liste
des règles **non** respectées : au moins 8 caractères, au moins un chiffre, au moins
une majuscule. Testez trois mots de passe.

Sortie attendue :

```text
chat : trop court, aucun chiffre, aucune majuscule
chaton2024 : aucune majuscule
Chaton2024 : parfait
```

> **Indice :** pour savoir si un texte contient un chiffre : `/[0-9]/.test(texte)`.
> Pour une majuscule : `/[A-Z]/.test(texte)`.

> **Indice :** partez d'un tableau vide `const liste: string[] = [];` et faites un
> `push` par règle non respectée.

> **Solution**
>
> ```typescript
> function problemes(motDePasse: string): string[] {
>   const liste: string[] = [];
>   if (motDePasse.length < 8) liste.push("trop court");
>   if (!/[0-9]/.test(motDePasse)) liste.push("aucun chiffre");
>   if (!/[A-Z]/.test(motDePasse)) liste.push("aucune majuscule");
>   return liste;
> }
>
> for (const mdp of ["chat", "chaton2024", "Chaton2024"]) {
>   const liste = problemes(mdp);
>   console.log(`${mdp} : ${liste.length === 0 ? "parfait" : liste.join(", ")}`);
> }
> ```
>
> ```sortie
> chat : trop court, aucun chiffre, aucune majuscule
> chaton2024 : aucune majuscule
> Chaton2024 : parfait
> ```
>
> Pourquoi ça marche : les trois `if` sont indépendants, comme trois contrôles à la
> suite. Chaque contrôle raté ajoute une remarque. Une liste vide veut dire que tout
> est bon. L'annotation `string[]` est nécessaire : le tableau est vide au départ.

## Pour aller plus loin

### Exercice 10 : Rendre la monnaie

Niveau : ★★★

Un distributeur rend la monnaie avec le moins de pièces possible. Les pièces valent,
en centimes : 200, 100, 50, 20, 10, 5, 2 et 1. Écrivez
`rendreMonnaie(centimes: number): number[]` qui renvoie les pièces à rendre. Testez
avec 386 centimes.

Sortie attendue :

```text
200 + 100 + 50 + 20 + 10 + 5 + 1
7 pièces
```

> **Indice :** parcourez les pièces de la plus grande à la plus petite.

> **Indice :** pour chaque pièce, utilisez `while (reste >= piece)` : on rend la pièce
> et on la retire du reste, tant qu'on peut.

> **Solution**
>
> ```typescript
> const PIECES = [200, 100, 50, 20, 10, 5, 2, 1] as const;
>
> function rendreMonnaie(centimes: number): number[] {
>   const rendu: number[] = [];
>   let reste = centimes;
>   for (const piece of PIECES) {
>     while (reste >= piece) {
>       rendu.push(piece);
>       reste -= piece;
>     }
>   }
>   return rendu;
> }
>
> const pieces = rendreMonnaie(386);
> console.log(pieces.join(" + "));
> console.log(`${pieces.length} pièces`);
> ```
>
> ```sortie
> 200 + 100 + 50 + 20 + 10 + 5 + 1
> 7 pièces
> ```
>
> Pourquoi ça marche : c'est ce que fait un caissier. Il prend toujours la plus grosse
> pièce qui « rentre » dans ce qu'il reste à rendre.
>
> | piece | reste avant | pièces rendues | reste après |
> |---|---|---|---|
> | 200 | 386 | 200 | 186 |
> | 100 | 186 | 100 | 86 |
> | 50 | 86 | 50 | 36 |
> | 20 | 36 | 20 | 16 |
> | 10 | 16 | 10 | 6 |
> | 5 | 6 | 5 | 1 |
> | 2 | 1 | (aucune : 1 < 2) | 1 |
> | 1 | 1 | 1 | 0 |
>
> On travaille en centimes entiers exprès : avec `3.86` €, les calculs à virgule
> donneraient des restes comme `0.009999999`.

### Exercice 11 : Une file d'attente générique

Niveau : ★★★

Écrivez une classe **générique** `FileAttente<T>`, comme la file d'un guichet : on
entre par l'arrière, on sort par l'avant. Elle a trois méthodes : `entrer(element: T)`,
`sortir(): T | undefined` et `taille(): number`. Faites entrer Alice, Bob et Chloé,
puis faites sortir quatre personnes.

Sortie attendue :

```text
3 personnes en attente
Au suivant : Alice
Au suivant : Bob
Au suivant : Chloé
Personne en attente
```

> **Indice :** rangez les éléments dans un tableau privé `#elements: T[] = []`. `push`
> ajoute à la fin ; `shift` retire le premier et renvoie `undefined` si le tableau est
> vide (voir [les génériques](generiques.md)).

> **Solution**
>
> ```typescript
> class FileAttente<T> {
>   #elements: T[] = [];
>
>   entrer(element: T): void {
>     this.#elements.push(element);
>   }
>
>   sortir(): T | undefined {
>     return this.#elements.shift();
>   }
>
>   taille(): number {
>     return this.#elements.length;
>   }
> }
>
> const guichet = new FileAttente<string>();
> guichet.entrer("Alice");
> guichet.entrer("Bob");
> guichet.entrer("Chloé");
> console.log(`${guichet.taille()} personnes en attente`);
>
> for (let i = 0; i < 4; i++) {
>   const suivant = guichet.sortir();
>   console.log(suivant === undefined ? "Personne en attente" : `Au suivant : ${suivant}`);
> }
> ```
>
> ```sortie
> 3 personnes en attente
> Au suivant : Alice
> Au suivant : Bob
> Au suivant : Chloé
> Personne en attente
> ```
>
> Pourquoi ça marche : `T` est une case vide que l'on remplit à la création
> (`FileAttente<string>`). Ensuite, `guichet.entrer(42)` serait refusé par `tsc`.
>
> | Étape | Action | file (avant → arrière) | Affiche |
> |---|---|---|---|
> | 1 | entrer Alice, Bob, Chloé | Alice, Bob, Chloé | `3 personnes en attente` |
> | 2 | sortir | Bob, Chloé | `Au suivant : Alice` |
> | 3 | sortir | Chloé | `Au suivant : Bob` |
> | 4 | sortir | (vide) | `Au suivant : Chloé` |
> | 5 | sortir → `undefined` | (vide) | `Personne en attente` |

### Exercice 12 : Lire des commandes de stock

Niveau : ★★★

Un magasin reçoit des lignes de texte : `"ajouter 10"`, `"retirer 3"`… Écrivez :

- un type `Commande` : `{ type: "ajouter"; quantite: number }` ou
  `{ type: "retirer"; quantite: number }` ;
- une fonction `lire(ligne: string): Commande | string` qui renvoie la commande, ou un
  message d'erreur (une `string`) si la ligne est mauvaise ;
- une boucle qui part d'un stock de 0, applique les commandes valides et signale les
  autres. Un retrait plus grand que le stock est refusé.

Les lignes : `"ajouter 10"`, `"retirer 3"`, `"vendre 2"`, `"retirer 20"`,
`"ajouter cinq"`, `"ajouter 5"`.

Sortie attendue :

```text
ajouter 10 -> stock 10
retirer 3 -> stock 7
vendre 2 -> erreur : action inconnue « vendre »
retirer 20 -> refusé : stock 7 seulement
ajouter cinq -> erreur : quantité invalide « cinq »
ajouter 5 -> stock 12
```

> **Indice :** `ligne.split(" ")` donne `["ajouter", "10"]`. `Number("cinq")` donne
> `NaN` : testez-le avec `Number.isNaN`.

> **Indice :** après l'appel, `typeof resultat === "string"` sépare les erreurs des
> commandes. Ensuite, `resultat.type` dit quelle commande appliquer (voir
> [les unions](unions.md)).

> **Solution**
>
> ```typescript
> type Commande =
>   | { type: "ajouter"; quantite: number }
>   | { type: "retirer"; quantite: number };
>
> function lire(ligne: string): Commande | string {
>   const [action, texte] = ligne.split(" ");
>   const quantite = Number(texte);
>   if (Number.isNaN(quantite) || quantite <= 0) {
>     return `quantité invalide « ${texte} »`;
>   }
>   if (action === "ajouter" || action === "retirer") {
>     return { type: action, quantite };
>   }
>   return `action inconnue « ${action} »`;
> }
>
> const lignes = ["ajouter 10", "retirer 3", "vendre 2", "retirer 20", "ajouter cinq", "ajouter 5"];
> let stock = 0;
>
> for (const ligne of lignes) {
>   const resultat = lire(ligne);
>   if (typeof resultat === "string") {
>     console.log(`${ligne} -> erreur : ${resultat}`);
>   } else if (resultat.type === "ajouter") {
>     stock += resultat.quantite;
>     console.log(`${ligne} -> stock ${stock}`);
>   } else if (resultat.quantite > stock) {
>     console.log(`${ligne} -> refusé : stock ${stock} seulement`);
>   } else {
>     stock -= resultat.quantite;
>     console.log(`${ligne} -> stock ${stock}`);
>   }
> }
> ```
>
> ```sortie
> ajouter 10 -> stock 10
> retirer 3 -> stock 7
> vendre 2 -> erreur : action inconnue « vendre »
> retirer 20 -> refusé : stock 7 seulement
> ajouter cinq -> erreur : quantité invalide « cinq »
> ajouter 5 -> stock 12
> ```
>
> Pourquoi ça marche : `lire` est un guichet qui trie le courrier. Il rend soit une
> commande propre, soit une lettre d'erreur. Après `typeof resultat === "string"`,
> TypeScript sait que `resultat` est une `Commande` dans les autres branches : il
> autorise `resultat.type` et `resultat.quantite`.
>
> | Ligne | lire renvoie | stock après |
> |---|---|---|
> | ajouter 10 | `{ type: "ajouter", quantite: 10 }` | 10 |
> | retirer 3 | `{ type: "retirer", quantite: 3 }` | 7 |
> | vendre 2 | `"action inconnue « vendre »"` | 7 |
> | retirer 20 | `{ type: "retirer", quantite: 20 }`, mais 20 > 7 | 7 |
> | ajouter cinq | `"quantité invalide « cinq »"` | 7 |
> | ajouter 5 | `{ type: "ajouter", quantite: 5 }` | 12 |

## En résumé

- Les exercices 1 à 4 reprennent les bases : fonctions, conditions, boucles et
  calculs.
- Les exercices 5 à 9 travaillent le texte, les objets `Record`, les unions de textes
  et les retours `| undefined`.
- Les exercices 10 à 12 utilisent les boucles imbriquées, une classe générique et une
  union discriminée.
- Quand une solution vous surprend, refaites son tableau de suivi sur papier.

Suite : [Aide-mémoire](aide-memoire.md)
