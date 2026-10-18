# Exercices corrigés

Cette page propose 12 exercices corrigés, du plus facile au plus difficile, pour pratiquer tout le parcours TypeScript.

## Comment utiliser cette page

Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Lisez l'énoncé, puis écrivez votre solution dans un fichier `.ts`. Vérifiez les types
avec `npx tsc --noEmit --strict exercice.ts`, puis lancez-le avec `npx tsx exercice.ts`.
Ouvrez la solution seulement après avoir essayé.

> **Astuce :** un exercice est réussi quand le programme affiche le bon résultat **et**
> que `tsc --strict` ne signale aucune erreur. Toutes les solutions de cette page
> passent ce contrôle.

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux constantes, `prenom` (une chaîne) et `age` (un nombre). Affichez ensuite la
phrase `Je m'appelle Alice et j'ai 30 ans.` avec vos constantes.

> **Indice :** un gabarit entre accents graves : `` `Bonjour ${nom}` `` (voir
> [les premiers pas](premiers-pas.md)).

> **Solution**
>
> ```typescript
> const prenom = "Alice";
> const age = 30;
> console.log(`Je m'appelle ${prenom} et j'ai ${age} ans.`);
> ```
>
> ```sortie
> Je m'appelle Alice et j'ai 30 ans.
> ```
>
> Pas besoin d'annotation : TypeScript infère `string` et `number`.

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Écrivez une fonction `estPair(n: number): boolean`. Utilisez-la pour afficher
`7 est impair` et `10 est pair`.

> **Indice :** `n % 2` donne le reste de la division par 2.

> **Solution**
>
> ```typescript
> function estPair(n: number): boolean {
>   return n % 2 === 0;
> }
>
> for (const n of [7, 10]) {
>   console.log(`${n} est ${estPair(n) ? "pair" : "impair"}`);
> }
> ```
>
> ```sortie
> 7 est impair
> 10 est pair
> ```

### Exercice 3 : FizzBuzz

Niveau : ★☆☆

Pour les nombres de 1 à 15, affichez `Fizz` si le nombre est divisible par 3, `Buzz`
s'il l'est par 5, `FizzBuzz` s'il l'est par les deux, et le nombre sinon. Affichez tout
sur une seule ligne, séparé par des espaces.

> **Indice :** testez d'abord le cas « par 15 ». Rangez les mots dans un tableau
> `string[]`, puis `join(" ")` (voir [conditions et boucles](conditions-boucles.md)).

> **Solution**
>
> ```typescript
> const mots: string[] = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) mots.push("FizzBuzz");
>   else if (i % 3 === 0) mots.push("Fizz");
>   else if (i % 5 === 0) mots.push("Buzz");
>   else mots.push(String(i));
> }
> console.log(mots.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> L'annotation `string[]` est utile ici : le tableau est vide au départ.

### Exercice 4 : L'aire d'un rectangle

Niveau : ★☆☆

Écrivez une fonction `aire(largeur, hauteur)` qui renvoie l'aire d'un rectangle. Si on
ne donne pas la hauteur, le rectangle est un carré : la hauteur vaut la largeur.
Affichez `aire(4, 3)` et `aire(5)`.

> **Indice :** un paramètre peut avoir une valeur par défaut qui dépend d'un paramètre
> précédent (voir [les fonctions](fonctions.md)).

> **Solution**
>
> ```typescript
> function aire(largeur: number, hauteur: number = largeur): number {
>   return largeur * hauteur;
> }
>
> console.log(aire(4, 3));
> console.log(aire(5));
> ```
>
> ```sortie
> 12
> 25
> ```

## Texte, tableaux et objets

### Exercice 5 : Compter les voyelles

Niveau : ★★☆

Écrivez une fonction `voyelles(texte: string): number` qui compte les voyelles (a, e,
i, o, u, y) d'un texte, en majuscules comme en minuscules. Testez-la avec
`"TypeScript est super"`.

> **Indice :** `toLowerCase()`, puis parcourez les lettres avec `for (const c of texte)`
> et testez `"aeiouy".includes(c)` (voir [le texte](texte.md)).

> **Solution**
>
> ```typescript
> function voyelles(texte: string): number {
>   let total = 0;
>   for (const c of texte.toLowerCase()) {
>     if ("aeiouy".includes(c)) total++;
>   }
>   return total;
> }
>
> console.log(voyelles("TypeScript est super"));
> ```
>
> ```sortie
> 6
> ```

### Exercice 6 : La moyenne

Niveau : ★★☆

Écrivez une fonction `moyenne(notes: readonly number[])` qui renvoie la moyenne, ou
`undefined` si le tableau est vide. Affichez la moyenne de `[12, 15, 9, 14]`, puis
celle de `[]` en écrivant `aucune note` dans ce cas.

> **Indice :** le type de retour est `number | undefined`. À l'appel, `??` donne une
> valeur de remplacement.

> **Solution**
>
> ```typescript
> function moyenne(notes: readonly number[]): number | undefined {
>   if (notes.length === 0) return undefined;
>   return notes.reduce((a, b) => a + b, 0) / notes.length;
> }
>
> console.log(moyenne([12, 15, 9, 14]) ?? "aucune note");
> console.log(moyenne([]) ?? "aucune note");
> ```
>
> ```sortie
> 12.5
> aucune note
> ```
>
> Le type de retour oblige l'appelant à prévoir le cas vide : `moyenne([]).toFixed(1)`
> serait refusé par `tsc`.

### Exercice 7 : Le panier

Niveau : ★★☆

Définissez une interface `Article` avec un `nom`, un `prix` unitaire et une `quantite`.
Écrivez `total(panier: readonly Article[]): number`, puis affichez le total de ce
panier avec deux décimales : 3 croissants à 1,20 €, 1 baguette à 1,10 €.

> **Indice :** `reduce` additionne `prix * quantite` ; `toFixed(2)` formate (voir
> [les objets](objets.md)).

> **Solution**
>
> ```typescript
> interface Article {
>   nom: string;
>   prix: number;
>   quantite: number;
> }
>
> function total(panier: readonly Article[]): number {
>   return panier.reduce((somme, a) => somme + a.prix * a.quantite, 0);
> }
>
> const panier: Article[] = [
>   { nom: "croissant", prix: 1.2, quantite: 3 },
>   { nom: "baguette", prix: 1.1, quantite: 1 },
> ];
> console.log(`Total : ${total(panier).toFixed(2)} €`);
> ```
>
> ```sortie
> Total : 4.70 €
> ```

### Exercice 8 : Les formes

Niveau : ★★☆

Une forme est soit un cercle (avec un `rayon`), soit un rectangle (avec une `largeur`
et une `hauteur`). Écrivez le type `Forme` et une fonction `aire(f: Forme): number`.
Affichez l'aire d'un cercle de rayon 1 (deux décimales) et d'un rectangle 2 × 3.

> **Indice :** une union discriminée, avec une propriété `type` commune, et un `switch`
> (voir [les unions](unions.md#les-unions-discriminees)).

> **Solution**
>
> ```typescript
> type Forme =
>   | { type: "cercle"; rayon: number }
>   | { type: "rectangle"; largeur: number; hauteur: number };
>
> function aire(f: Forme): number {
>   switch (f.type) {
>     case "cercle":
>       return Math.PI * f.rayon ** 2;
>     case "rectangle":
>       return f.largeur * f.hauteur;
>   }
> }
>
> console.log(aire({ type: "cercle", rayon: 1 }).toFixed(2));
> console.log(aire({ type: "rectangle", largeur: 2, hauteur: 3 }));
> ```
>
> ```sortie
> 3.14
> 6
> ```

## Des types plus riches

### Exercice 9 : Le dernier élément

Niveau : ★★☆

Écrivez une fonction **générique** `dernier` qui renvoie le dernier élément d'un tableau
de n'importe quel type, ou `undefined` s'il est vide. Le type de retour doit suivre le
type du tableau : `dernier(["a", "b"])` est une `string | undefined`.

> **Indice :** `function dernier<T>(tableau: readonly T[]): T | undefined` (voir
> [les génériques](generiques.md)).

> **Solution**
>
> ```typescript
> function dernier<T>(tableau: readonly T[]): T | undefined {
>   return tableau[tableau.length - 1];
> }
>
> const mot = dernier(["pomme", "poire", "kiwi"]);
> const nombre = dernier([1, 2, 3]);
> console.log(mot?.toUpperCase());
> console.log(nombre);
> console.log(dernier([]));
> ```
>
> ```sortie
> KIWI
> 3
> undefined
> ```
>
> Grâce à `T`, TypeScript sait que `mot` est une chaîne : `toUpperCase` est permis.
> L'opérateur `?.` gère le cas `undefined`.

### Exercice 10 : Le feu tricolore

Niveau : ★★★

Définissez le type `Feu` : `"vert"`, `"orange"` ou `"rouge"`. Écrivez
`suivant(feu: Feu): Feu` (vert → orange → rouge → vert). Ajoutez une vérification
d'exhaustivité : si un jour on ajoute une couleur au type, `tsc` doit signaler l'oubli.
Affichez quatre changements à partir de `"vert"`.

> **Indice :** dans le `default` du `switch`, rangez la valeur dans une variable de type
> `never` (voir [les types littéraux](litteraux.md) et
> [les unions](unions.md#verifier-tous-les-cas-avec-never)).

> **Solution**
>
> ```typescript
> type Feu = "vert" | "orange" | "rouge";
>
> function suivant(feu: Feu): Feu {
>   switch (feu) {
>     case "vert":
>       return "orange";
>     case "orange":
>       return "rouge";
>     case "rouge":
>       return "vert";
>     default: {
>       const impossible: never = feu;
>       throw new Error(`Feu inconnu : ${impossible}`);
>     }
>   }
> }
>
> let feu: Feu = "vert";
> const etapes: Feu[] = [feu];
> for (let i = 0; i < 4; i++) {
>   feu = suivant(feu);
>   etapes.push(feu);
> }
> console.log(etapes.join(" -> "));
> ```
>
> ```sortie
> vert -> orange -> rouge -> vert -> orange
> ```
>
> Ajoutez `"clignotant"` au type `Feu` : la ligne `const impossible: never = feu;`
> devient une erreur, car `feu` peut encore valoir `"clignotant"` à cet endroit.

### Exercice 11 : Le compte en banque

Niveau : ★★★

Écrivez une classe `Compte` avec un `titulaire` en lecture seule et un solde **privé**,
qui commence à 0. Elle a trois méthodes : `deposer(montant)`, `retirer(montant)` et
`solde()`. Un dépôt ou un retrait négatif, ou un retrait plus grand que le solde, lève
une `Error`. Faites un dépôt de 100, un retrait de 30, puis tentez un retrait de 500 en
attrapant l'erreur.

> **Indice :** `private` ou `#solde` cache le solde ; `throw new Error("…")` lève une
> erreur ; dans le `catch`, vérifiez `e instanceof Error` avant de lire `e.message`
> (voir [les erreurs](erreurs.md)).

> **Solution**
>
> ```typescript
> class Compte {
>   #solde = 0;
>
>   constructor(readonly titulaire: string) {}
>
>   deposer(montant: number): void {
>     if (montant <= 0) throw new Error("Le dépôt doit être positif");
>     this.#solde += montant;
>   }
>
>   retirer(montant: number): void {
>     if (montant <= 0) throw new Error("Le retrait doit être positif");
>     if (montant > this.#solde) throw new Error(`Solde insuffisant (${this.#solde} €)`);
>     this.#solde -= montant;
>   }
>
>   solde(): number {
>     return this.#solde;
>   }
> }
>
> const compte = new Compte("Alice");
> compte.deposer(100);
> compte.retirer(30);
> console.log(`${compte.titulaire} : ${compte.solde()} €`);
>
> try {
>   compte.retirer(500);
> } catch (e) {
>   console.log(e instanceof Error ? `Refusé : ${e.message}` : "Erreur inconnue");
> }
> console.log(`${compte.titulaire} : ${compte.solde()} €`);
> ```
>
> ```sortie
> Alice : 70 €
> Refusé : Solde insuffisant (70 €)
> Alice : 70 €
> ```
>
> `#solde` est vraiment privé, même à l'exécution : `compte.#solde` est impossible hors
> de la classe. Le solde ne change que par les méthodes, qui vérifient tout.

### Exercice 12 : Charger des profils

Niveau : ★★★

Voici une fonction asynchrone qui simule un serveur lent :

```typescript norun
const PROFILS: Record<number, string> = { 1: "Alice", 2: "Bob" };

function chargerNom(id: number): Promise<string> {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      const nom = PROFILS[id];
      if (nom === undefined) reject(new Error(`Profil ${id} introuvable`));
      else resolve(nom);
    }, 10);
  });
}
```

Écrivez une fonction `async function chargerTous(ids: number[])` qui charge tous les
profils **en même temps**, et renvoie pour chacun un résultat :
`{ ok: true; nom: string }` ou `{ ok: false; erreur: string }`. Affichez les résultats
pour les numéros 1, 3 et 2, dans cet ordre.

> **Indice :** `Promise.allSettled` attend toutes les promesses, réussies ou non, et
> garde l'ordre. Chaque réponse a un `status` : `"fulfilled"` (avec `value`) ou
> `"rejected"` (avec `reason`) (voir [l'asynchrone](asynchrone.md)).

> **Solution**
>
> ```typescript
> type Resultat = { ok: true; nom: string } | { ok: false; erreur: string };
>
> const PROFILS: Record<number, string> = { 1: "Alice", 2: "Bob" };
>
> function chargerNom(id: number): Promise<string> {
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       const nom = PROFILS[id];
>       if (nom === undefined) reject(new Error(`Profil ${id} introuvable`));
>       else resolve(nom);
>     }, 10);
>   });
> }
>
> async function chargerTous(ids: number[]): Promise<Resultat[]> {
>   const reponses = await Promise.allSettled(ids.map(chargerNom));
>   return reponses.map((r): Resultat => {
>     if (r.status === "fulfilled") return { ok: true, nom: r.value };
>     const message = r.reason instanceof Error ? r.reason.message : String(r.reason);
>     return { ok: false, erreur: message };
>   });
> }
>
> for (const r of await chargerTous([1, 3, 2])) {
>   console.log(r.ok ? `OK : ${r.nom}` : `Échec : ${r.erreur}`);
> }
> ```
>
> ```sortie
> OK : Alice
> Échec : Profil 3 introuvable
> OK : Bob
> ```
>
> Avec `Promise.all`, le premier échec aurait tout arrêté. `allSettled` laisse chaque
> chargement aller au bout. Le type de retour annoté `(r): Resultat` aide TypeScript à
> reconnaître l'union discriminée.

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Essayez toujours seul, et vérifiez vos types avec `tsc --strict` avant d'ouvrir la
  solution.
- Les exercices 1 à 4 reprennent les bases : constantes, conditions, boucles, fonctions.
- Les exercices 5 à 8 travaillent le texte, les tableaux, les interfaces et les unions.
- Les exercices 9 à 12 utilisent les génériques, les littéraux, les classes, les
  erreurs et l'asynchrone.

Suite : gardez [l'aide-mémoire](aide-memoire.md) sous la main. Comparez avec
[les exercices en Ruby](../ruby/exercices.md).
