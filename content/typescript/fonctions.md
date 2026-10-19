# Les fonctions

Cette page explique comment écrire des fonctions en TypeScript, typer leurs paramètres et leur valeur de retour, et savoir où leurs variables existent.

> **Objectifs :**
>
> - vous saurez écrire une fonction avec des paramètres et un type de retour ;
> - vous saurez rendre un paramètre facultatif ou lui donner une valeur par défaut ;
> - vous saurez écrire une fonction fléchée `(x) => ...` ;
> - vous saurez dire où une variable existe : sa portée.

## Définir et appeler une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code nommé, que vous
pouvez réutiliser. C'est comme une recette : vous l'écrivez une fois, puis vous la
suivez autant de fois que vous voulez.

```typescript
function saluer(): void {
  console.log("Bonjour !");
}

saluer();
saluer();
```

```sortie
Bonjour !
Bonjour !
```

- `function` annonce une fonction, `saluer` est son nom ;
- les parenthèses `()` contiennent ses paramètres (ici, aucun) ;
- `: void` dit que la fonction ne renvoie rien d'utile (*void* : « vide ») ;
- le bloc `{ ... }` est le corps de la fonction.

Écrire `function saluer` ne lance rien : cela enregistre la recette. Écrire `saluer()`
**appelle** la fonction, c'est-à-dire l'exécute. Les parenthèses sont obligatoires.

## Les paramètres et leur type

Une recette de crêpes sert pour 4 ou pour 10 personnes : seules les quantités changent.
De la même façon, une fonction peut faire le même travail avec des valeurs différentes
à chaque appel.

Les [paramètres](../debuter/glossaire.md#parametre) sont les valeurs que la fonction
reçoit. Ce sont des cases vides, remplies au moment de l'appel par les
[arguments](../debuter/glossaire.md#argument). En TypeScript, chaque paramètre a un
type.

```typescript
function presenter(prenom: string, age: number): void {
  console.log(`${prenom} a ${age} ans.`);
}

presenter("Alice", 30);
presenter("Bob", 25);
```

```sortie
Alice a 30 ans.
Bob a 25 ans.
```

Ligne par ligne :

- les trois premières lignes enregistrent la fonction : rien ne s'affiche encore ;
- `presenter("Alice", 30)` lance la fonction : `prenom` reçoit `"Alice"` et `age`
  reçoit `30`, puis le corps affiche la phrase ;
- `presenter("Bob", 25)` la relance avec d'autres valeurs : cette fois, `prenom` vaut
  `"Bob"` et `age` vaut `25`.

Ici, `prenom` et `age` sont les paramètres ; `"Alice"` et `30` sont les arguments.

Le type d'un paramètre est un contrat : « donnez-moi un nombre ». TypeScript vérifie
**chaque appel**. `presenter("Alice", "trente")` est refusé avant même de lancer le
programme.

> **À retenir :** TypeScript ne devine pas le type d'un paramètre. Écrivez-le toujours,
> sinon `tsc --strict` affiche une erreur (voir les
> [erreurs fréquentes](#erreurs-frequentes)).

## Renvoyer une valeur avec return

Une calculatrice ne se contente pas d'afficher : elle vous **donne** un résultat, que
vous pouvez réutiliser dans un autre calcul. `return` permet à une fonction de faire
pareil.

`return` renvoie une valeur au code qui a appelé la fonction, et arrête la fonction.
Le type de cette valeur s'écrit après les parenthèses.

```typescript
function aire(largeur: number, hauteur: number): number {
  return largeur * hauteur;
}

console.log(aire(3, 4));
const total = aire(2, 5) + 1;
console.log(total);
```

```sortie
12
11
```

Ligne par ligne :

- `aire(3, 4)` lance la fonction avec `largeur` à 3 et `hauteur` à 4 ;
- `return largeur * hauteur` calcule 12 et le renvoie : `console.log` affiche 12 ;
- `aire(2, 5)` renvoie 10, puis on ajoute 1 : `total` vaut 11 ;
- la dernière ligne affiche `total`.

La valeur renvoyée remplace l'appel : `aire(2, 5) + 1` devient `10 + 1`.

Le type de retour `: number` peut s'omettre : TypeScript le devine à partir du
`return`. L'écrire reste une bonne habitude : si vous vous trompez dans le corps de la
fonction, l'erreur apparaît dans la fonction, pas plus loin chez celui qui l'appelle.

**À vous :** écrivez une fonction `perimetre` qui prend la largeur et la hauteur d'un
rectangle (deux nombres) et renvoie son périmètre. Affichez `perimetre(3, 4)`.

> **Solution**
>
> ```typescript
> function perimetre(largeur: number, hauteur: number): number {
>   return 2 * (largeur + hauteur);
> }
>
> console.log(perimetre(3, 4));
> ```
>
> ```sortie
> 14
> ```

### Afficher ou renvoyer ?

`console.log` **affiche** une valeur à l'écran. `return` la **donne** au code qui a
appelé la fonction, qui peut s'en servir.

```typescript
function doubleAffiche(x: number): void {
  console.log(x * 2);
}

function double(x: number): number {
  return x * 2;
}

doubleAffiche(5);
console.log(double(5) + 1);
```

```sortie
10
11
```

`doubleAffiche(5) + 1` serait refusé : une fonction `void` ne donne aucune valeur
utilisable.

> **Astuce :** préférez les fonctions qui renvoient une valeur. Vous choisirez ensuite
> de l'afficher, de la stocker ou de la réutiliser.

### Sortir plus tôt

Un `return` au milieu de la fonction l'arrête aussitôt :

```typescript
function diviser(a: number, b: number): string {
  if (b === 0) {
    return "division impossible";
  }
  return (a / b).toString();
}

console.log(diviser(10, 4));
console.log(diviser(10, 0));
```

```sortie
2.5
division impossible
```

TypeScript vérifie que **tous les chemins** renvoient une valeur du bon type. S'il
manque un `return`, il le signale.

## Paramètres facultatifs et valeurs par défaut

Certaines informations ne sont pas toujours utiles. Au café, vous pouvez commander
« un café » sans préciser la taille : le serveur prend la taille normale. Une fonction
peut, elle aussi, se passer de certains arguments.

### Un paramètre facultatif : ?

Un `?` après le nom rend le paramètre facultatif. S'il n'est pas donné, il vaut
`undefined` : son type devient `string | undefined`.

```typescript
function saluer(prenom: string, titre?: string): string {
  if (titre === undefined) {
    return `Bonjour, ${prenom} !`;
  }
  return `Bonjour, ${titre} ${prenom} !`;
}

console.log(saluer("Alice"));
console.log(saluer("Martin", "docteur"));
```

```sortie
Bonjour, Alice !
Bonjour, docteur Martin !
```

Ligne par ligne :

- `saluer("Alice")` ne donne qu'un argument : `prenom` vaut `"Alice"` et `titre` vaut
  `undefined` ;
- le test `titre === undefined` est donc vrai : la fonction renvoie la phrase sans
  titre ;
- `saluer("Martin", "docteur")` donne les deux : le test est faux, et la fonction
  renvoie la phrase avec le titre.

TypeScript vous oblige à traiter le cas `undefined` avant d'utiliser `titre` comme un
texte.

### Une valeur par défaut : =

Plus simple : donnez une valeur par défaut. Elle sert si l'argument est absent. Le
type est deviné à partir de cette valeur.

```typescript
function prixTTC(prix: number, taux = 0.2): number {
  return prix * (1 + taux);
}

console.log(prixTTC(100));
console.log(prixTTC(100, 0.055));
```

```sortie
120
105.5
```

> **Attention :** les paramètres facultatifs se placent **après** les obligatoires.
> `function f(a?: number, b: number)` est refusé.

**À vous :** écrivez une fonction `commander` qui prend un nom de boisson et une
taille, avec `"moyen"` comme taille par défaut. Elle renvoie par exemple
`"Un café moyen"`. Affichez `commander("café")` et `commander("thé", "grand")`.

> **Solution**
>
> ```typescript
> function commander(boisson: string, taille = "moyen"): string {
>   return `Un ${boisson} ${taille}`;
> }
>
> console.log(commander("café"));
> console.log(commander("thé", "grand"));
> ```
>
> ```sortie
> Un café moyen
> Un thé grand
> ```

## Les fonctions fléchées

Beaucoup de fonctions sont toutes petites : une seule ligne de calcul. Écrire
`function`, des accolades et `return` pour si peu est long. La fonction fléchée est un
raccourci pour ces cas.

Une **fonction fléchée** (*arrow function*) est une écriture courte, avec `=>`. On la
range souvent dans une constante.

```typescript
const carre = (x: number): number => x * x;

const presenter = (prenom: string): string => {
  const majuscules = prenom.toUpperCase();
  return `Je suis ${majuscules}.`;
};

console.log(carre(4));
console.log(presenter("léa"));
```

```sortie
16
Je suis LÉA.
```

Ligne par ligne :

- `carre` est une constante qui contient une fonction : elle prend `x` et renvoie
  `x * x` ;
- `presenter` contient une fonction plus longue, entre accolades : elle met le prénom
  en majuscules, puis renvoie la phrase ;
- `carre(4)` s'appelle comme une fonction ordinaire et renvoie 16 ;
- `presenter("léa")` renvoie `"Je suis LÉA."`.

- Avec une seule expression après `=>`, pas besoin d'accolades ni de `return` : la
  valeur est renvoyée automatiquement.
- Avec plusieurs lignes, on met des accolades et un `return`.

Une fonction sans nom, comme celle-ci, s'appelle aussi une
[lambda](../debuter/glossaire.md#lambda). Les fonctions fléchées sont partout en
TypeScript, surtout comme *callbacks* (au chapitre suivant).

**À vous :** écrivez une fonction fléchée `triple`, rangée dans une constante, qui
prend un nombre et renvoie ce nombre multiplié par 3. Affichez `triple(7)`.

> **Solution**
>
> ```typescript
> const triple = (x: number): number => x * 3;
>
> console.log(triple(7));
> ```
>
> ```sortie
> 21
> ```

## La portée des variables

Chaque fonction a son propre plan de travail. Ce qu'elle y pose reste chez elle : cela
évite qu'une fonction dérange par erreur les variables du reste du programme.

La [portée](../debuter/glossaire.md#portee) d'une variable est la zone du code où elle
existe. Les paramètres et les variables créées dans une fonction n'existent que dans
cette fonction.

```typescript
const tauxTVA = 0.2;

function prixTTC(prix: number): number {
  const total = prix * (1 + tauxTVA);
  return Math.round(total * 100) / 100;
}

console.log(prixTTC(19.99));
```

```sortie
23.99
```

Ligne par ligne :

- `tauxTVA` est créée tout en haut, en dehors de la fonction : elle vaut 0.2 ;
- `prixTTC(19.99)` lance la fonction : `prix` vaut 19.99 ;
- `total` est créée **dans** la fonction et vaut environ 23.988 ;
- `Math.round(total * 100) / 100` arrondit à deux chiffres après la virgule : la
  fonction renvoie 23.99.

- La fonction voit `tauxTVA`, créée **en dehors**, au-dessus d'elle.
- Dehors, `total` n'existe pas : `console.log(total)` serait refusé.

> **Astuce :** une fonction est plus facile à comprendre quand elle reçoit tout ce dont
> elle a besoin en paramètres.

## Erreurs fréquentes

### Oublier le type d'un paramètre

```typescript norun
function double(x) {
  return x * 2;
}
console.log(double(4));
```

```text
main.ts:1:17 - error TS7006: Parameter 'x' implicitly has an 'any' type.

1 function double(x) {
                  ~


Found 1 error in main.ts:1
```

« Le paramètre `x` a implicitement le type `any`. » Sans type, `x` pourrait être
n'importe quoi : le mode strict refuse. Écrivez `function double(x: number)`.

### Oublier un argument

```typescript norun
function aire(largeur: number, hauteur: number): number {
  return largeur * hauteur;
}
console.log(aire(3));
```

```text
main.ts:4:13 - error TS2554: Expected 2 arguments, but got 1.

4 console.log(aire(3));
              ~~~~

  main.ts:1:32
    1 function aire(largeur: number, hauteur: number): number {
                                     ~~~~~~~~~~~~~~~
    An argument for 'hauteur' was not provided.


Found 1 error in main.ts:4
```

La fonction attend 2 arguments (*expected 2*), elle en reçoit 1 (*got 1*). Le message
précise même lequel manque : `hauteur`. Donnez-les tous : `aire(3, 4)`. Ou rendez le
paramètre facultatif avec `?` ou une valeur par défaut.

### Passer un argument du mauvais type

```typescript norun
function aire(largeur: number, hauteur: number): number {
  return largeur * hauteur;
}
console.log(aire(3, "4"));
```

```text
main.ts:4:21 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

4 console.log(aire(3, "4"));
                      ~~~


Found 1 error in main.ts:4
```

`"4"` est un texte, pas un nombre. Écrivez `aire(3, 4)`, ou convertissez le texte avec
`Number("4")`.

### Oublier un return sur un chemin

```typescript norun
function mention(note: number): string {
  if (note >= 10) {
    return "reçu";
  } else if (note < 10) {
    return "refusé";
  }
}
console.log(mention(12));
```

```text
main.ts:1:33 - error TS2366: Function lacks ending return statement and return type does not include 'undefined'.

1 function mention(note: number): string {
                                  ~~~~~~


Found 1 error in main.ts:1
```

« Il manque un `return` à la fin. » TypeScript ne voit pas que les deux tests couvrent
tous les cas. Remplacez `else if (note < 10)` par un simple `else` : tous les chemins
renvoient alors un texte.

## Testez-vous

**Question 1 :** écrivez une fonction fléchée `moyenne` qui prend deux nombres et
renvoie leur moyenne.

> **Réponse**
>
> ```typescript
> const moyenne = (a: number, b: number): number => (a + b) / 2;
>
> console.log(moyenne(3, 4));
> ```
>
> ```sortie
> 3.5
> ```

**Question 2 :** quelle différence entre `titre?: string` et `titre = "M."` ?

> **Réponse**
>
> Les deux rendent le paramètre facultatif. Avec `?`, il vaut `undefined` quand il est
> absent : il faut tester ce cas avant de l'utiliser. Avec `= "M."`, il vaut `"M."`
> quand il est absent : c'est toujours un `string`.

**Question 3 :** une variable `total` est créée dans une fonction. Après l'appel, peut-on
écrire `console.log(total)` en dehors de la fonction ?

> **Réponse**
>
> Non. La portée de `total` est la fonction : en dehors, elle n'existe pas, et
> TypeScript refuse (*Cannot find name 'total'*). Pour récupérer la valeur, la fonction
> doit la renvoyer avec `return`.

## En résumé

- Une fonction s'écrit `function nom(param: type): typeRetour { ... }` et s'appelle
  avec `nom(...)`.
- Les paramètres doivent toujours être typés ; le type de retour peut être deviné,
  mais l'écrire aide.
- `?` rend un paramètre facultatif (il vaut alors `undefined`), `= valeur` lui donne
  une valeur par défaut.
- Une fonction fléchée s'écrit `(x: number) => x * 2` ; `void` veut dire « ne renvoie
  rien ».
- Les paramètres et les variables créées dans une fonction n'existent que dans cette
  fonction : c'est leur portée.

Suite : [aller plus loin avec les fonctions](fonctions-avancees.md).
