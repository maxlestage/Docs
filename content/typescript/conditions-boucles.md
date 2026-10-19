# Conditions et boucles

Cette page explique comment faire des choix avec `if`, `switch` et l'opérateur ternaire, comment répéter des actions avec des boucles, et ce que les types apportent à chaque fois.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `if`, `else if` et `else` ;
> - vous saurez comparer avec `===` et reconnaître les valeurs « fausses » ;
> - vous saurez gérer une valeur absente avec `??` et `?.` ;
> - vous saurez écrire un `switch` qui n'oublie aucun cas grâce à `never` ;
> - vous saurez répéter avec `for`, `while` et `for...of`.

## if et else

Un programme ne fait pas toujours la même chose. Vous-même, vous prenez un parapluie
s'il pleut et des lunettes de soleil s'il fait beau. Une condition permet au programme
de faire ce genre de choix.

Une [condition](../debuter/glossaire.md#condition) choisit quel code exécuter. `if`
(« si ») exécute son bloc `{ ... }` seulement si le test entre parenthèses est vrai.
`else` (« sinon ») donne le bloc à exécuter dans le cas contraire.

```typescript
const temperature: number = 28;

if (temperature > 25) {
  console.log("Il fait chaud.");
} else {
  console.log("Il fait bon.");
}
```

```sortie
Il fait chaud.
```

Ligne par ligne :

- `const temperature: number = 28;` range 28 dans la variable `temperature` ;
- `if (temperature > 25)` pose une question : « 28 est-il plus grand que 25 ? ». La
  réponse est oui ;
- le premier bloc s'exécute donc et affiche « Il fait chaud. » ;
- le bloc du `else` est sauté : il ne sert que si la réponse est non.

Pour plus de deux cas, enchaînez avec `else if` (« sinon, si »). Les tests sont lus
dans l'ordre ; le premier vrai gagne, les autres sont ignorés.

```typescript
const note: number = 13;

if (note >= 16) {
  console.log("Très bien");
} else if (note >= 12) {
  console.log("Bien");
} else if (note >= 10) {
  console.log("Passable");
} else {
  console.log("Insuffisant");
}
```

```sortie
Bien
```

**À vous :** écrivez un programme qui range `15` dans une constante `heure`, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```typescript
> const heure: number = 15;
> if (heure < 12) {
>   console.log("Bonjour");
> } else if (heure < 18) {
>   console.log("Bon après-midi");
> } else {
>   console.log("Bonsoir");
> }
> ```
>
> ```sortie
> Bon après-midi
> ```

## Comparer avec ===

Pour décider, un programme pose d'abord une question : « l'âge vaut-il 20 ? »,
« la température dépasse-t-elle 25 ? ». Les opérateurs de comparaison servent à écrire
ces questions. La réponse est toujours oui (`true`) ou non (`false`).

Les opérateurs de comparaison donnent un `boolean` (`true` ou `false`) :

| Opérateur | Sens |
|-----------|------|
| `===` | égal à |
| `!==` | différent de |
| `<`, `<=` | plus petit, plus petit ou égal |
| `>`, `>=` | plus grand, plus grand ou égal |

On les combine avec `&&` (« et »), `||` (« ou ») et `!` (« non ») :

```typescript
const age: number = 20;
const aUnBillet: boolean = true;

console.log(age === 20, age !== 20);
console.log(age >= 18 && aUnBillet);
console.log(age < 12 || age > 65);
console.log(!aUnBillet);
```

```sortie
true false
true
false
false
```

Ligne par ligne :

- `age === 20` est vrai et `age !== 20` est faux : la première ligne affiche
  `true false` ;
- `age >= 18 && aUnBillet` : les deux tests sont vrais, donc `&&` donne `true` ;
- `age < 12 || age > 65` : aucun des deux tests n'est vrai, donc `||` donne `false` ;
- `!aUnBillet` inverse `true` : on obtient `false`.

> **Attention :** écrivez toujours `===` (trois signes), jamais `==` (deux). `==`
> transforme les valeurs avant de comparer : `0 == ""` vaut `true` en JavaScript !
> Et un seul `=` ne compare pas : il range une valeur dans une variable.

TypeScript surveille vos comparaisons. Comparer un nombre et un texte est toujours
faux : c'est sûrement une erreur, et il vous le dit (voir les
[erreurs fréquentes](#erreurs-frequentes)).

**À vous :** écrivez un programme qui range `70` dans une constante `age`, puis affiche
« Tarif réduit » si l'âge est inférieur à 18 ou supérieur ou égal à 65, et
« Plein tarif » sinon.

> **Solution**
>
> ```typescript
> const age: number = 70;
> if (age < 18 || age >= 65) {
>   console.log("Tarif réduit");
> } else {
>   console.log("Plein tarif");
> }
> ```
>
> ```sortie
> Tarif réduit
> ```

## Les valeurs « vraies » et « fausses »

Souvent, vous voulez juste savoir si une case est remplie : un nom saisi, un
message non vide. Un `if` permet de le demander très vite, à condition de connaître ses
règles.

Un `if` accepte n'importe quelle valeur, pas seulement un `boolean`. Six valeurs
comptent comme fausses (*falsy*) :

- `false`, `0`, `NaN` (« pas un nombre »), `""` (le texte vide), `null` et `undefined`.

Toutes les autres comptent comme vraies (*truthy*), même `"0"`, `" "` ou `[]`.

```typescript
const texte = "";

if (texte) {
  console.log("Le texte est rempli.");
} else {
  console.log("Le texte est vide.");
}

console.log(Boolean(0), Boolean(""), Boolean(null));
console.log(Boolean("0"), Boolean(" "), Boolean([]));
```

```sortie
Le texte est vide.
false false false
true true true
```

Ligne par ligne :

- `texte` contient `""`, le texte vide ;
- `if (texte)` : le texte vide compte comme faux, donc c'est le bloc du `else` qui
  s'exécute ;
- les deux dernières lignes montrent comment un `if` voit six valeurs : les trois
  premières comptent comme fausses, les trois suivantes comme vraies.

`Boolean(x)` montre comment `x` est vu par un `if`.

> **Astuce :** quand c'est possible, écrivez le test complet : `texte !== ""` ou
> `nombre !== 0`. Le lecteur comprend tout de suite ce que vous testez.

## L'opérateur ternaire

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Le ternaire `test ? siVrai : siFaux` est un `if/else` qui **donne une valeur**. Il
tient sur une ligne.

```typescript
const age: number = 15;
const statut = age >= 18 ? "majeur" : "mineur";
console.log(statut);

const points: number = 1;
console.log(`${points} point${points > 1 ? "s" : ""}`);
```

```sortie
mineur
1 point
```

Lisez-le comme une question : « âge ≥ 18 ? alors "majeur", sinon "mineur" ».

> **Astuce :** gardez le ternaire pour les choix simples. S'il faut l'imbriquer, un
> `if/else` est plus lisible.

## switch : choisir parmi plusieurs valeurs

Quand une même variable peut prendre beaucoup de valeurs, une longue suite de
`else if` devient pénible à lire. `switch` range les cas les uns sous les autres, comme
les tiroirs étiquetés d'une commode.

`switch` compare une valeur à une liste de cas (`case`). Il exécute le code du cas qui
correspond. `break` sort du `switch` ; `default` traite tous les autres cas.

```typescript
const jour: string = "samedi";

switch (jour) {
  case "samedi":
  case "dimanche":
    console.log("Week-end !");
    break;
  case "mercredi":
    console.log("Mi-semaine.");
    break;
  default:
    console.log("Jour de travail.");
}
```

```sortie
Week-end !
```

Ligne par ligne :

- `jour` contient `"samedi"` ;
- `switch (jour)` compare cette valeur à chaque `case`, de haut en bas ;
- `case "samedi":` correspond : l'exécution commence là, descend jusqu'au
  `console.log` et affiche « Week-end ! » ;
- `break` sort du `switch` : les autres cas sont ignorés.

Deux `case` à la suite partagent le même code : `samedi` et `dimanche` affichent
« Week-end ! ».

> **Attention :** sans `break`, l'exécution continue dans le cas suivant. Oublier
> `break` est un piège classique.

## Les boucles

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100
`console.log` serait long et pénible. Une boucle fait ce travail répétitif à votre
place.

Une [boucle](../debuter/glossaire.md#boucle) répète un bloc de code.

### for : compter

La boucle `for` a trois parties, séparées par des points-virgules : le départ, la
condition pour continuer, et le pas.

```typescript
for (let i = 1; i <= 3; i++) {
  console.log(`Tour ${i}`);
}
```

```sortie
Tour 1
Tour 2
Tour 3
```

- `let i = 1` : le compteur commence à 1 ;
- `i <= 3` : on continue tant que `i` vaut au plus 3 ;
- `i++` : après chaque tour, on ajoute 1 à `i`.

**À vous :** écrivez un programme qui affiche la table de 7, de « 7 x 1 = 7 » à
« 7 x 5 = 35 », une ligne par calcul.

> **Solution**
>
> ```typescript
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

### while : tant que

`while` (« tant que ») répète tant que sa condition est vraie. Utilisez-la quand vous
ne savez pas d'avance combien de tours il faut.

```typescript
let argent = 100;
let annees = 0;

while (argent < 150) {
  argent = argent * 1.1;
  annees++;
}

console.log(`${annees} ans, ${argent.toFixed(2)} euros`);
```

```sortie
5 ans, 161.05 euros
```

Ligne par ligne :

- au départ, `argent` vaut 100 et `annees` vaut 0 ;
- avant chaque tour, `while` vérifie `argent < 150`. 100 est plus petit : il entre
  dans le bloc ;
- à chaque tour, `argent` grandit de 10 % (110, puis 121, 133.1, 146.41, 161.051) et
  `annees` augmente de 1 ;
- après 5 tours, `argent < 150` est faux : la boucle s'arrête, et le dernier
  `console.log` affiche le résultat, arrondi à deux chiffres par `toFixed(2)`.

> **Attention :** si la condition reste toujours vraie, la boucle ne s'arrête jamais.
> Vérifiez que quelque chose change à chaque tour. Pour arrêter un programme bloqué,
> tapez `Ctrl+C` dans le terminal.

### for...of : parcourir une liste

`for...of` prend les éléments d'une liste un par un. La liste s'écrit entre crochets ;
les [tableaux](collections.md) sont expliqués plus loin.

```typescript
const fruits = ["pomme", "poire", "kiwi"];

for (const fruit of fruits) {
  console.log(fruit.toUpperCase());
}
```

```sortie
POMME
POIRE
KIWI
```

TypeScript sait que `fruits` contient des `string` : `fruit` est donc un `string`, et
`toUpperCase()` est autorisé. Un `const` suffit : chaque tour crée un nouveau `fruit`.

**À vous :** écrivez un programme qui range la liste de courses `["pain", "lait",
"œufs"]` dans une constante, puis affiche chaque article sur sa ligne, précédé d'un
tiret.

> **Solution**
>
> ```typescript
> const courses = ["pain", "lait", "œufs"];
>
> for (const article of courses) {
>   console.log(`- ${article}`);
> }
> ```
>
> ```sortie
> - pain
> - lait
> - œufs
> ```

### break et continue

- `break` sort de la boucle tout de suite ;
- `continue` passe directement au tour suivant.

```typescript
for (let i = 1; i <= 10; i++) {
  if (i % 2 === 0) {
    continue;
  }
  if (i > 7) {
    break;
  }
  console.log(i);
}
```

```sortie
1
3
5
7
```

Les nombres pairs sont sautés (`continue`), et la boucle s'arrête à 9 (`break`).

## Le rétrécissement : les types suivent vos tests

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Voici ce que les types apportent aux conditions. Quand une variable peut avoir
plusieurs types, un test **rétrécit** son type dans le bloc (*narrowing*). C'est comme
un vigile à l'entrée d'une salle : une fois passé, on sait qui est à l'intérieur.

```typescript
const reponses: (string | number)[] = ["oui", 3.14159];

for (const reponse of reponses) {
  if (typeof reponse === "string") {
    console.log(reponse.toUpperCase(), "est un texte");
  } else {
    console.log(reponse.toFixed(2), "est un nombre");
  }
}
```

```sortie
OUI est un texte
3.14 est un nombre
```

`(string | number)[]` est une liste dont chaque élément est un texte ou un nombre.

Dans le premier bloc, TypeScript sait que `reponse` est un `string` : il autorise
`toUpperCase()`. Dans le `else`, il sait que c'est un `number` : il autorise
`toFixed()`. Pas besoin de le lui répéter.

## Les valeurs absentes : ?? et ?.

Une information manque parfois : un champ de formulaire laissé vide, un réglage jamais
choisi. Ces deux opérateurs évitent que votre programme plante dans ce cas.

Une valeur de type `string | undefined` peut manquer. Deux opérateurs aident à la
manipuler sans risque.

### ?? : une valeur par défaut

`a ?? b` donne `a`, sauf si `a` vaut `null` ou `undefined` : alors il donne `b`.

```typescript
const volumes: (number | null)[] = [7, 0, null];

for (const volume of volumes) {
  console.log(volume ?? 5, volume || 5);
}
```

```sortie
7 7
0 5
5 5
```

Ligne par ligne :

- `volumes` contient trois réglages : 7, 0 et `null` (aucun réglage) ;
- la boucle les prend un par un et affiche deux colonnes : `volume ?? 5`, puis
  `volume || 5` ;
- pour 7, les deux gardent 7 ; pour `null`, les deux donnent 5 ;
- pour 0, ils ne sont pas d'accord : c'est toute la différence.

Comparez les deux colonnes : `||` remplace **toutes** les valeurs fausses, y compris
`0`. `??` ne remplace que `null` et `undefined`. Un volume à `0` est un vrai réglage :
`??` le garde.

> **À retenir :** pour une valeur par défaut, préférez `??` à `||`.

### ?. : appeler seulement si la valeur existe

`x?.action()` appelle `action()` seulement si `x` existe. Sinon, le résultat est
`undefined`, sans erreur.

```typescript
const saisies: (string | undefined)[] = ["alice", undefined];

for (const saisie of saisies) {
  console.log(saisie?.toUpperCase());
  console.log(saisie?.toUpperCase() ?? "(rien)");
}
```

```sortie
ALICE
ALICE
undefined
(rien)
```

Sans le `?.`, TypeScript refuserait `saisie.toUpperCase()` : `saisie` est peut-être
`undefined`. On combine souvent `?.` et `??` : « le nom en majuscules, ou "(rien)" ».

## Un switch qui n'oublie aucun cas

Voici une technique que seul un langage typé permet. Un type peut lister ses valeurs
possibles : `"rouge" | "orange" | "vert"` (un type qui n'accepte que ces trois textes).
Le mot `type` donne un nom à ce type.

```typescript
type Feu = "rouge" | "orange" | "vert";

const feux: Feu[] = ["vert", "orange", "rouge"];

for (const feu of feux) {
  switch (feu) {
    case "rouge":
      console.log("Stop");
      break;
    case "orange":
      console.log("Ralentir");
      break;
    case "vert":
      console.log("Passer");
      break;
    default: {
      const impossible: never = feu;
      console.log(impossible);
    }
  }
}
```

```sortie
Passer
Ralentir
Stop
```

Ligne par ligne :

- `type Feu = ...` crée un type qui n'accepte que trois textes ;
- `feux` est une liste de `Feu` : TypeScript y refuserait `"bleu"` ;
- la boucle prend chaque feu, et le `switch` affiche le message du `case` qui
  correspond ;
- le `default` n'est jamais atteint ici : il sert à la vérification expliquée
  juste après.

Le type `never` veut dire « aucune valeur possible ». Dans le `default`, tous les cas
ont été traités : `feu` ne peut plus rien valoir, son type est donc `never`, et la
ligne est acceptée.

L'intérêt arrive le jour où vous ajoutez une couleur, par exemple
`"clignotant"`, sans penser à modifier le `switch`. Ce cas tombe alors dans le
`default`, et `tsc` refuse de ranger `"clignotant"` dans une variable `never` :

```text
main.ts:15:13 - error TS2322: Type '"clignotant"' is not assignable to type 'never'.

15       const impossible: never = feu;
               ~~~~~~~~~~


Found 1 error in main.ts:15
```

L'erreur nomme précisément le cas oublié. C'est une alarme : « vous avez ajouté une
valeur, pensez à la traiter partout ».

## Erreurs fréquentes

### Comparer un nombre et un texte

```typescript norun
const age: number = 18;
if (age === "18") {
  console.log("Majeur");
}
```

```text
main.ts:2:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

2 if (age === "18") {
      ~~~~~~~~~~~~


Found 1 error in main.ts:2
```

« Cette comparaison semble involontaire : les types `number` et `string` n'ont rien en
commun. » Un nombre n'est jamais égal à un texte : le `if` serait toujours faux.
Comparez avec le nombre : `age === 18`.

### Utiliser une valeur peut-être absente

```typescript norun
const noms = ["Alice", "Bob"];
const trouve = noms.find((n) => n.startsWith("Z"));
console.log(trouve.toUpperCase());
```

```text
main.ts:3:13 - error TS18048: 'trouve' is possibly 'undefined'.

3 console.log(trouve.toUpperCase());
              ~~~~~~


Found 1 error in main.ts:3
```

`find` cherche un élément et renvoie `undefined` s'il ne trouve rien (ce sera détaillé
dans [les tableaux](collections.md)). TypeScript vous oblige à prévoir ce cas. Écrivez
`trouve?.toUpperCase()`, ou testez d'abord : `if (trouve !== undefined) { ... }`.

### Utiliser le compteur après la boucle

```typescript norun
for (let i = 0; i < 3; i++) {
  console.log(i);
}
console.log(i);
```

```text
main.ts:4:13 - error TS2304: Cannot find name 'i'.

4 console.log(i);
              ~


Found 1 error in main.ts:4
```

Une variable créée avec `let` n'existe que dans son bloc : c'est sa
[portée](../debuter/glossaire.md#portee). Si vous avez besoin de la valeur après la
boucle, créez la variable avant : `let i = 0; for (; i < 3; i++) { ... }`.

### Oublier un cas dans un switch exhaustif

Vous avez ajouté `"clignotant"` au type `Feu`, mais pas au `switch` :

```typescript norun
type Feu = "rouge" | "orange" | "vert" | "clignotant";
const feux: Feu[] = ["rouge", "orange", "vert"];
for (const feu of feux) {
  switch (feu) {
    case "rouge":
      console.log("Stop");
      break;
    case "orange":
      console.log("Ralentir");
      break;
    case "vert":
      console.log("Passer");
      break;
    default: {
      const impossible: never = feu;
      console.log(impossible);
    }
  }
}
```

```text
main.ts:15:13 - error TS2322: Type '"clignotant"' is not assignable to type 'never'.

15       const impossible: never = feu;
               ~~~~~~~~~~


Found 1 error in main.ts:15
```

Ajoutez le cas manquant : `case "clignotant": console.log("Prudence"); break;`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```typescript norun
const stock: number | null = 0;
console.log(stock ?? 10, stock || 10);
```

> **Réponse**
>
> `0 10`. `??` ne remplace que `null` et `undefined` : il garde `0`. `||` remplace
> toutes les valeurs fausses, dont `0`.

**Question 2 :** écrivez une boucle qui affiche les nombres de 10 à 0, de 2 en 2.

> **Réponse**
>
> ```typescript
> for (let i = 10; i >= 0; i -= 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```
>
> `i -= 2` est un raccourci pour `i = i - 2`.

**Question 3 :** à quoi sert `const impossible: never = feu;` dans le `default` d'un
`switch` ?

> **Réponse**
>
> À vérifier que tous les cas sont traités. Si une valeur du type est oubliée, elle
> arrive dans le `default`, et `tsc` refuse de la ranger dans une variable `never` :
> l'erreur nomme le cas oublié.

## En résumé

- `if`, `else if` et `else` choisissent un bloc ; le ternaire `a ? b : c` choisit une
  valeur.
- Comparez avec `===` et `!==`, jamais avec `==`.
- `false`, `0`, `NaN`, `""`, `null` et `undefined` comptent comme faux dans un `if`.
- Un test comme `typeof x === "string"` rétrécit le type de `x` dans le bloc.
- `??` donne une valeur par défaut, `?.` appelle une action seulement si la valeur
  existe.
- `for` compte, `while` répète tant que, `for...of` parcourt une liste ; un `switch`
  avec `never` dans `default` n'oublie aucun cas.

Suite : [les fonctions](fonctions.md).
