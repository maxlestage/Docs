# Exercices corrigés

Cette page propose 12 exercices corrigés, du plus facile au plus difficile, pour pratiquer tout le parcours JavaScript.

## Comment utiliser cette page

Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Lisez l'énoncé, puis écrivez votre solution dans un fichier `.mjs` et lancez-le avec
`node fichier.mjs`. Ouvrez la solution seulement après avoir essayé.

> **Astuce :** il existe souvent plusieurs bonnes solutions. Si votre programme affiche
> le bon résultat, c'est gagné !

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux constantes, `prenom` et `age`. Affichez ensuite la phrase
`Je m'appelle Alice et j'ai 30 ans.`, avec vos constantes.

> **Indice :** utilisez un gabarit `` `…${prenom}…` `` (voir
> [les premiers pas](premiers-pas.md)).

> **Solution**
>
> ```javascript
> const prenom = "Alice";
> const age = 30;
> console.log(`Je m'appelle ${prenom} et j'ai ${age} ans.`);
> ```
>
> ```sortie
> Je m'appelle Alice et j'ai 30 ans.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Pour chacun des nombres `7` et `10`, affichez `7 est impair` ou `10 est pair`.

> **Indice :** `n % 2` donne le reste de la division par 2 : `0` pour un nombre pair.

> **Solution**
>
> ```javascript
> for (const n of [7, 10]) {
>   if (n % 2 === 0) {
>     console.log(`${n} est pair`);
>   } else {
>     console.log(`${n} est impair`);
>   }
> }
> ```
>
> ```sortie
> 7 est impair
> 10 est pair
> ```

### Exercice 3 : FizzBuzz

Niveau : ★☆☆

Affichez les nombres de 1 à 15. Mais à la place des multiples de 3, affichez `Fizz` ;
à la place des multiples de 5, `Buzz` ; et des multiples des deux, `FizzBuzz`. Affichez
tout sur une ligne, séparé par des espaces.

> **Indice :** testez d'abord le cas « multiple de 15 », sinon il ne sera jamais atteint
> (voir [conditions et boucles](conditions-boucles.md)).

> **Solution**
>
> ```javascript
> const resultats = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) {
>     resultats.push("FizzBuzz");
>   } else if (i % 3 === 0) {
>     resultats.push("Fizz");
>   } else if (i % 5 === 0) {
>     resultats.push("Buzz");
>   } else {
>     resultats.push(i);
>   }
> }
> console.log(resultats.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Exercice 4 : Convertir des températures

Niveau : ★☆☆

Écrivez une fonction fléchée `enFahrenheit(celsius)` qui renvoie
`celsius * 9 / 5 + 32`. Affichez la conversion de `0`, `37` et `100` degrés.

> **Indice :** une fonction fléchée s'écrit `const f = (x) => …;` (voir
> [les fonctions](fonctions.md)).

> **Solution**
>
> ```javascript
> const enFahrenheit = (celsius) => celsius * 9 / 5 + 32;
>
> for (const c of [0, 37, 100]) {
>   console.log(`${c} °C = ${enFahrenheit(c).toFixed(1)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```

## Texte et collections

### Exercice 5 : Compter les voyelles

Niveau : ★★☆

Écrivez une fonction `compterVoyelles(texte)` qui renvoie le nombre de voyelles
(`a`, `e`, `i`, `o`, `u`, `y`) d'un texte, majuscules comprises. Testez-la avec
`"JavaScript"`.

> **Indice :** `[...texte]` donne un tableau de caractères ; `"aeiouy".includes(c)` dit
> si `c` est une voyelle (voir [le texte](texte.md)).

> **Solution**
>
> ```javascript
> function compterVoyelles(texte) {
>   return [...texte.toLowerCase()].filter((c) => "aeiouy".includes(c)).length;
> }
>
> console.log(compterVoyelles("JavaScript"));
> console.log(compterVoyelles("Bonjour"));
> ```
>
> ```sortie
> 3
> 3
> ```

### Exercice 6 : Le palindrome

Niveau : ★★☆

Un palindrome se lit dans les deux sens, comme `kayak` ou `Été`. Écrivez
`estPalindrome(mot)` qui ignore les majuscules. Testez `"kayak"`, `"Radar"` et
`"code"`.

> **Indice :** retournez le mot avec `[...mot].reverse().join("")`.

> **Solution**
>
> ```javascript
> function estPalindrome(mot) {
>   const m = mot.toLowerCase();
>   return m === [...m].reverse().join("");
> }
>
> for (const mot of ["kayak", "Radar", "code"]) {
>   console.log(`${mot} : ${estPalindrome(mot)}`);
> }
> ```
>
> ```sortie
> kayak : true
> Radar : true
> code : false
> ```

### Exercice 7 : Compter les mots

Niveau : ★★☆

Dans la phrase `"le chat et le chien et le lapin"`, comptez combien de fois apparaît
chaque mot. Affichez les mots du plus fréquent au moins fréquent.

> **Indice :** un objet sert de compteur : `compte[mot] = (compte[mot] ?? 0) + 1`.
> `Object.entries` le transforme en tableau de paires, que l'on peut trier (voir
> [les objets](objets.md)).

> **Solution**
>
> ```javascript
> const phrase = "le chat et le chien et le lapin";
> const compte = {};
> for (const mot of phrase.split(" ")) {
>   compte[mot] = (compte[mot] ?? 0) + 1;
> }
>
> const tries = Object.entries(compte).toSorted((a, b) => b[1] - a[1]);
> for (const [mot, nombre] of tries) {
>   console.log(`${mot} : ${nombre}`);
> }
> ```
>
> ```sortie
> le : 3
> et : 2
> chat : 1
> chien : 1
> lapin : 1
> ```
>
> À égalité, `toSorted` garde l'ordre d'origine : `chat`, `chien`, puis `lapin`.

### Exercice 8 : Le panier

Niveau : ★★☆

Voici un panier :

```javascript norun
const panier = [
  { nom: "Livre", prix: 12, quantite: 2 },
  { nom: "Stylo", prix: 1.5, quantite: 4 },
  { nom: "Sac", prix: 30, quantite: 1 },
];
```

Avec `filter`, `map` et `reduce`, affichez les noms des articles de moins de 20 €,
puis le total du panier.

> **Indice :** le total d'une ligne est `prix * quantite` ; `reduce` les additionne en
> partant de `0` (voir [les collections](collections.md)).

> **Solution**
>
> ```javascript
> const panier = [
>   { nom: "Livre", prix: 12, quantite: 2 },
>   { nom: "Stylo", prix: 1.5, quantite: 4 },
>   { nom: "Sac", prix: 30, quantite: 1 },
> ];
>
> const petitsPrix = panier.filter((a) => a.prix < 20).map((a) => a.nom);
> console.log(petitsPrix.join(", "));
>
> const total = panier.reduce((somme, a) => somme + a.prix * a.quantite, 0);
> console.log(`Total : ${total.toFixed(2)} €`);
> ```
>
> ```sortie
> Livre, Stylo
> Total : 60.00 €
> ```

## Objets, classes et fonctions

### Exercice 9 : Un compteur privé

Niveau : ★★☆

Écrivez une fonction `creerCompteur()` qui renvoie un objet avec deux méthodes :
`incrementer()`, qui ajoute 1 et renvoie la nouvelle valeur, et `valeur()`. La variable
du compte ne doit pas être accessible de l'extérieur. Deux compteurs doivent être
indépendants.

> **Indice :** c'est une [closure](../debuter/glossaire.md#closure) : les méthodes se
> souviennent d'une variable `let` déclarée dans `creerCompteur` (voir
> [les closures](closures.md)).

> **Solution**
>
> ```javascript
> function creerCompteur() {
>   let compte = 0;
>   return {
>     incrementer() {
>       compte += 1;
>       return compte;
>     },
>     valeur() {
>       return compte;
>     },
>   };
> }
>
> const a = creerCompteur();
> const b = creerCompteur();
> a.incrementer();
> a.incrementer();
> b.incrementer();
> console.log(a.valeur(), b.valeur(), a.compte);
> ```
>
> ```sortie
> 2 1 undefined
> ```

### Exercice 10 : Le compte en banque

Niveau : ★★★

Écrivez une classe `CompteBancaire` avec un titulaire et un solde **privé**
(`#solde`). Elle a trois méthodes : `deposer(montant)`, `retirer(montant)` et un
accesseur `solde`. `retirer` lève une `RangeError` si le solde est insuffisant ;
`deposer` en lève une si le montant n'est pas positif.

> **Indice :** un champ qui commence par `#` n'est lisible que dans la classe ;
> `get solde()` crée un accesseur en lecture (voir [l'héritage](heritage.md) et
> [les erreurs](erreurs.md)).

> **Solution**
>
> ```javascript
> class CompteBancaire {
>   #solde = 0;
>
>   constructor(titulaire) {
>     this.titulaire = titulaire;
>   }
>
>   get solde() {
>     return this.#solde;
>   }
>
>   deposer(montant) {
>     if (montant <= 0) throw new RangeError("montant invalide");
>     this.#solde += montant;
>   }
>
>   retirer(montant) {
>     if (montant > this.#solde) throw new RangeError("solde insuffisant");
>     this.#solde -= montant;
>   }
> }
>
> const compte = new CompteBancaire("Alice");
> compte.deposer(100);
> compte.retirer(30);
> console.log(`${compte.titulaire} : ${compte.solde} €`);
>
> try {
>   compte.retirer(500);
> } catch (erreur) {
>   console.log(`Refusé : ${erreur.message}`);
> }
> console.log(`Solde inchangé : ${compte.solde} €`);
> ```
>
> ```sortie
> Alice : 70 €
> Refusé : solde insuffisant
> Solde inchangé : 70 €
> ```

## Pour aller plus loin

### Exercice 11 : Un générateur de Fibonacci

Niveau : ★★★

La suite de Fibonacci commence par `0, 1`, puis chaque nombre est la somme des deux
précédents : `0, 1, 1, 2, 3, 5, 8…`. Écrivez un générateur `fibonacci()` sans fin, et
une fonction `prendre(iterable, n)` qui renvoie les `n` premières valeurs dans un
tableau. Affichez les 10 premiers nombres.

> **Indice :** un générateur s'écrit `function*` et produit ses valeurs avec `yield`
> (voir [les itérateurs](iterateurs.md)). Dans `prendre`, arrêtez la boucle `for…of`
> avec `break`.

> **Solution**
>
> ```javascript
> function* fibonacci() {
>   let [a, b] = [0, 1];
>   while (true) {
>     yield a;
>     [a, b] = [b, a + b];
>   }
> }
>
> function prendre(iterable, n) {
>   const resultat = [];
>   if (n <= 0) return resultat;
>   for (const valeur of iterable) {
>     resultat.push(valeur);
>     if (resultat.length === n) break;
>   }
>   return resultat;
> }
>
> console.log(prendre(fibonacci(), 10).join(", "));
> ```
>
> ```sortie
> 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
> ```

### Exercice 12 : Charger en parallèle

Niveau : ★★★

Une fonction simule le chargement d'une note depuis un serveur lent :

```javascript norun
function chargerNote(eleve) {
  const notes = { Alice: 15, Bob: 9, Chloé: 12 };
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (eleve in notes) resolve(notes[eleve]);
      else reject(new Error(`élève inconnu : ${eleve}`));
    }, 50);
  });
}
```

1. Chargez les notes d'Alice, Bob et Chloé **en même temps** avec `Promise.all`, et
   affichez leur moyenne.
2. Chargez ensuite la note de `"Zoé"` en attrapant l'erreur avec `try … catch`.

> **Indice :** `await Promise.all(eleves.map(chargerNote))` attend toutes les
> promesses à la fois (voir [l'asynchrone](asynchrone.md)).

> **Solution**
>
> ```javascript
> function chargerNote(eleve) {
>   const notes = { Alice: 15, Bob: 9, Chloé: 12 };
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       if (eleve in notes) resolve(notes[eleve]);
>       else reject(new Error(`élève inconnu : ${eleve}`));
>     }, 50);
>   });
> }
>
> const eleves = ["Alice", "Bob", "Chloé"];
> const notes = await Promise.all(eleves.map(chargerNote));
> const moyenne = notes.reduce((s, n) => s + n, 0) / notes.length;
> console.log(`Notes : ${notes.join(", ")}`);
> console.log(`Moyenne : ${moyenne}`);
>
> try {
>   await chargerNote("Zoé");
> } catch (erreur) {
>   console.log(`Erreur : ${erreur.message}`);
> }
> ```
>
> ```sortie
> Notes : 15, 9, 12
> Moyenne : 12
> Erreur : élève inconnu : Zoé
> ```
>
> Les trois chargements se font en même temps : le tout prend environ 50 ms, et non
> 150 ms. `Promise.all` garde l'ordre du tableau de départ.

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Essayez toujours seul avant d'ouvrir la solution.
- Les exercices 1 à 4 reprennent les bases : constantes, conditions, boucles, fonctions.
- Les exercices 5 à 8 travaillent le texte, les tableaux, les objets et `map`,
  `filter`, `reduce`.
- Les exercices 9 à 12 combinent tout : closures, classes, erreurs, générateurs et
  promesses.

Suite : [Exercices corrigés : deuxième série](exercices-2.md)
