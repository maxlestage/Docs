# Premiers pas

Cette page présente les briques de base de TypeScript : les variables, les types simples, l'affichage, les commentaires et la lecture d'une erreur de type.

> **Objectifs :**
>
> - vous saurez créer une variable avec `let` ou `const` ;
> - vous saurez annoter une variable avec `number`, `string` ou `boolean` ;
> - vous saurez quand TypeScript devine le type tout seul ;
> - vous saurez utiliser `null`, `undefined`, `any` et `unknown` ;
> - vous saurez lire une erreur de type ligne par ligne.

## Afficher avec console.log

`console.log` affiche une valeur dans le terminal, puis passe à la ligne. Il accepte
plusieurs valeurs, séparées par des virgules : il les affiche avec une espace entre
elles.

```typescript
console.log("Bonjour !");
console.log(2 + 3);
console.log("Total :", 2 + 3, "euros");
```

```sortie
Bonjour !
5
Total : 5 euros
```

Un texte s'écrit entre guillemets doubles `"..."` ou simples `'...'`. Sans guillemets,
TypeScript calcule : `2 + 3` donne `5`.

## Les variables : let et const

Une [variable](../debuter/glossaire.md#variable) est une boîte avec une étiquette :
l'étiquette est le nom, le contenu est la valeur. Deux mots permettent d'en créer une :

- `const` : une boîte **scellée**, sa valeur ne change plus ;
- `let` : une boîte **ouverte**, on peut remplacer sa valeur.

```typescript
const prenom = "Alice";
let score = 10;
console.log(prenom, score);

score = 15;
score = score + 5;
console.log(prenom, score);
```

```sortie
Alice 10
Alice 20
```

`score = score + 5` se lit de droite à gauche : on calcule `score + 5` (soit `20`),
puis on range le résultat dans `score`.

> **À retenir :** utilisez `const` par défaut. Passez à `let` seulement si la valeur
> doit changer. Le lecteur sait ainsi tout de suite ce qui bouge dans le programme.

Un nom de variable commence par une lettre, sans espace ni accent conseillé. On écrit
les noms composés en `camelCase` : `prixTotal`, `nombreDeJoueurs`.

> **Note :** vous verrez parfois `var` dans de vieux programmes. C'est l'ancêtre de
> `let`, avec des règles plus piégeuses. N'en écrivez pas.

## Les types simples

Chaque valeur a un [type](../debuter/glossaire.md#type). Les trois types de base sont :

| Type | Contenu | Exemples |
|------|---------|----------|
| `number` | un nombre, entier ou à virgule | `42`, `-3`, `3.14` |
| `string` | un texte (une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres)) | `"Alice"`, `'oui'` |
| `boolean` | vrai ou faux (un [booléen](../debuter/glossaire.md#booleen)) | `true`, `false` |

Vous pouvez écrire le type après le nom, avec deux-points : c'est une **annotation**.

```typescript
const age: number = 30;
const ville: string = "Lyon";
const estMajeur: boolean = true;

console.log(age, ville, estMajeur);
console.log(typeof age, typeof ville, typeof estMajeur);
```

```sortie
30 Lyon true
number string boolean
```

`typeof` donne le type d'une valeur pendant l'exécution, sous forme de texte.

### Calculer avec des nombres

Il n'y a qu'un type pour tous les nombres : `number`. Les opérations courantes :

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
console.log(Math.round(3.6), Math.floor(3.6));
console.log((10 / 3).toFixed(2));
```

```sortie
9 5 14
3.5
1
1024
4 3
3.33
```

- `/` est une vraie division : `7 / 2` donne `3.5`.
- `%` donne le reste de la division (`7 = 2 × 3 + 1`).
- `**` est la puissance : `2 ** 10` vaut 1024.
- `Math.round` arrondit, `Math.floor` arrondit vers le bas.
- `toFixed(2)` donne un texte avec 2 chiffres après la virgule.

## L'inférence : TypeScript devine

Vous n'êtes pas obligé d'écrire le type. Quand une variable reçoit une valeur dès sa
création, TypeScript **devine** son type : c'est l'**inférence**.

```typescript
let score = 10;
const message = "Gagné";

score = score * 2;
console.log(score, message);
```

```sortie
20 Gagné
```

TypeScript a déduit que `score` est un `number` et `message` un `string`. Dans votre
éditeur, survolez `score` : il affiche `let score: number`.

Le type deviné est aussi solide qu'un type écrit à la main. Essayer de ranger un texte
dans `score` donne une erreur :

```typescript norun
let score = 10;
score = "vingt";
```

```text
main.ts:2:1 - error TS2322: Type 'string' is not assignable to type 'number'.

2 score = "vingt";
  ~~~~~


Found 1 error in main.ts:2
```

> **Astuce :** laissez TypeScript deviner quand la valeur est évidente
> (`let score = 10`). Écrivez le type quand la variable est créée sans valeur, ou
> quand vous voulez rendre votre intention claire.

### const et les types précis

Avec `const`, TypeScript va plus loin. Comme la valeur ne changera jamais, il retient
la valeur exacte comme type :

```typescript
const couleur = "rouge";
let autreCouleur = "rouge";

console.log(couleur, autreCouleur);
```

```sortie
rouge rouge
```

Survolez les deux noms dans votre éditeur : `couleur` a le type `"rouge"` (ce texte-là
et aucun autre), `autreCouleur` a le type `string`. Ces types précis seront très utiles
dans [le chapitre sur le texte](texte.md).

## Les gabarits de texte

Un texte entre accents graves `` ` `` est un **gabarit** (*template literal*). Il peut
contenir `${...}` : l'expression entre les accolades est calculée et insérée dans le
texte.

```typescript
const prenom = "Alice";
const age = 30;

console.log(`${prenom} a ${age} ans.`);
console.log(`Dans 5 ans, elle aura ${age + 5} ans.`);
console.log(`Ligne 1
Ligne 2`);
```

```sortie
Alice a 30 ans.
Dans 5 ans, elle aura 35 ans.
Ligne 1
Ligne 2
```

Un gabarit peut aussi s'étendre sur plusieurs lignes.

Avec des guillemets normaux, il faut coller les morceaux avec `+` :

```typescript
const prenom = "Alice";
console.log("Bonjour, " + prenom + " !");
```

```sortie
Bonjour, Alice !
```

## Les commentaires

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains :
TypeScript l'ignore.

```typescript
// Un commentaire sur une ligne.
const tva = 0.2; // Il peut aussi suivre du code.

/*
  Un commentaire
  sur plusieurs lignes.
*/

/** Le prix hors taxes, en euros. */
const prixHT = 100;

console.log(prixHT * (1 + tva));
```

```sortie
120
```

Le commentaire `/** ... */` est un commentaire de documentation : votre éditeur
l'affiche quand vous survolez `prixHT`.

> **Astuce :** expliquez **pourquoi** vous faites quelque chose, pas ce que fait le
> code. `// TVA française` est utile ; `// multiplie par 0.2` ne l'est pas.

## null et undefined : l'absence de valeur

Deux valeurs spéciales représentent « rien » (voir [null](../debuter/glossaire.md#null)) :

- `undefined` : « pas encore de valeur ». C'est ce que contient une variable `let`
  créée sans valeur ;
- `null` : « volontairement vide ». Vous l'écrivez vous-même.

En mode strict (`--strict`, celui de ce site), un `string` ne peut **pas** contenir
`null`. Si une valeur peut manquer, il faut le dire avec une barre `|`, qui veut dire
« ou » :

```typescript
let surnom: string | null = null;
console.log(surnom);

surnom = "Ali";
console.log(surnom);

let reponse: number | undefined;
console.log(reponse);
```

```sortie
null
Ali
undefined
```

`string | null` se lit « un texte ou null ». C'est une **union** de types : vous en
verrez beaucoup dans [le chapitre sur les unions](unions.md).

> **À retenir :** en TypeScript strict, l'absence de valeur se voit dans le type. Vous
> ne pouvez pas l'oublier : c'est la cause de très nombreux bugs dans d'autres
> langages.

## any et unknown : quand on ne sait pas

Parfois, vous ne connaissez pas le type d'une valeur (une donnée venue d'un fichier,
par exemple). Deux types existent pour ça, et ils sont très différents.

### any : on débranche la vérification

`any` veut dire « n'importe quoi, ne vérifie rien ». TypeScript accepte tout, même les
erreurs :

```typescript norun
let valeur: any = "bonjour";
console.log(valeur.toFixed(2));
```

`tsc` ne dit rien. Mais au lancement, le programme plante (extrait du message) :

```text
TypeError: valeur.toFixed is not a function
```

`toFixed` n'existe pas sur un texte. Avec `any`, vous avez perdu toute la protection
de TypeScript.

### unknown : on vérifie avant d'utiliser

`unknown` veut dire « je ne sais pas encore ». Vous pouvez ranger n'importe quoi
dedans, mais pour l'utiliser, vous devez d'abord **vérifier** ce que c'est :

```typescript
const donnee: unknown = "salut";

if (typeof donnee === "string") {
  console.log(donnee.toUpperCase());
}
```

```sortie
SALUT
```

Dans le bloc `if`, TypeScript sait que `donnee` est un `string`, et autorise
`toUpperCase()`. Les conditions sont expliquées dans
[le chapitre suivant](conditions-boucles.md).

> **À retenir :** évitez `any`. Si vous ne connaissez pas le type, utilisez `unknown` :
> c'est un colis fermé qu'on ouvre avec précaution, pas un colis qu'on jette sans
> regarder.

## Lire une erreur de type, ligne par ligne

Voici un programme qui additionne un nombre et un texte :

```typescript norun
let total: number = 0;
const saisie = "42";
total = total + saisie;
```

```text
main.ts:3:1 - error TS2322: Type 'string' is not assignable to type 'number'.

3 total = total + saisie;
  ~~~~~


Found 1 error in main.ts:3
```

Lisons-le dans l'ordre :

1. `main.ts:3:1` : le problème est dans `main.ts`, ligne 3, colonne 1.
2. `error TS2322` : le code de l'erreur. TS2322 est la plus courante : « mauvais type
   pour cette place ».
3. `Type 'string' is not assignable to type 'number'` : « le type `string` ne peut pas
   être rangé dans le type `number` ».
4. La ligne recopiée, avec `~~~~~` sous `total` : c'est la boîte qui refuse la valeur.
5. `Found 1 error` : une seule erreur en tout.

Remontons le raisonnement : `saisie` est un texte (`"42"`). `total + saisie` colle donc
un nombre et un texte, ce qui donne un **texte** (`"042"`). Et `total` n'accepte que des
nombres. La correction : transformer le texte en nombre avec `Number(...)`.

```typescript
let total: number = 0;
const saisie = "42";
total = total + Number(saisie);
console.log(total);
```

```sortie
42
```

> **Astuce :** quand le message est long, cherchez la dernière phrase en anglais : elle
> décrit la cause la plus précise.

## Erreurs fréquentes

### Modifier une constante

```typescript norun
const score = 10;
score = 20;
```

```text
main.ts:2:1 - error TS2588: Cannot assign to 'score' because it is a constant.

2 score = 20;
  ~~~~~


Found 1 error in main.ts:2
```

« Impossible de modifier `score`, c'est une constante. » Si la valeur doit changer,
créez la variable avec `let score = 10;`.

### Utiliser une variable avant de lui donner une valeur

```typescript norun
let ville: string;
console.log(ville);
```

```text
main.ts:2:13 - error TS2454: Variable 'ville' is used before being assigned.

2 console.log(ville);
              ~~~~~


Found 1 error in main.ts:2
```

`ville` est déclarée, mais vide. Donnez-lui une valeur avant de l'utiliser :
`let ville: string = "Lyon";`.

### Ranger null dans un string

```typescript norun
let surnom: string = null;
```

```text
main.ts:1:5 - error TS2322: Type 'null' is not assignable to type 'string'.

1 let surnom: string = null;
      ~~~~~~


Found 1 error in main.ts:1
```

En mode strict, `string` veut dire « toujours un texte ». Si la valeur peut manquer,
dites-le : `let surnom: string | null = null;`.

### Utiliser une valeur unknown sans vérifier

```typescript norun
const donnee: unknown = "salut";
console.log(donnee.toUpperCase());
```

```text
main.ts:2:13 - error TS18046: 'donnee' is of type 'unknown'.

2 console.log(donnee.toUpperCase());
              ~~~~~~


Found 1 error in main.ts:2
```

TypeScript refuse : `donnee` pourrait être un nombre. Vérifiez d'abord avec
`if (typeof donnee === "string") { ... }`.

## Testez-vous

**Question 1 :** quel est le type de `x` après `let x = 3.5;` ?

> **Réponse**
>
> `number`. TypeScript le devine (inférence) : il n'y a qu'un type pour tous les
> nombres, entiers ou à virgule.

**Question 2 :** qu'affiche ce code ?

```typescript norun
const a = 4;
console.log(`${a} x 2 = ${a * 2}`);
```

> **Réponse**
>
> `4 x 2 = 8`. Chaque `${...}` est calculé et inséré dans le gabarit.

**Question 3 :** quelle est la différence entre `any` et `unknown` ?

> **Réponse**
>
> Les deux acceptent n'importe quelle valeur. Mais `any` débranche la vérification :
> tout est permis, même les erreurs. `unknown` oblige à vérifier le type (avec
> `typeof`, par exemple) avant d'utiliser la valeur.

## En résumé

- `const` crée une variable fixe, `let` une variable modifiable ; préférez `const`.
- Les types de base sont `number`, `string` et `boolean` ; on les écrit après `:`.
- TypeScript devine le type d'une variable qui reçoit une valeur : c'est l'inférence.
- En mode strict, `null` et `undefined` doivent apparaître dans le type :
  `string | null`.
- Préférez `unknown` à `any` : il oblige à vérifier avant d'utiliser.
- Un gabarit `` `...${x}...` `` insère des valeurs dans un texte ; `//` commence un
  commentaire.

Suite : [Nombres et calculs](nombres.md)
