# Conditions et boucles

Cette page montre comment faire des choix avec `if`, `switch` et l'opérateur ternaire, puis comment répéter des actions avec `while`, `for` et `for...of`.

> **Objectifs :**
>
> - vous saurez écrire une condition avec `if`, `else if` et `else` ;
> - vous saurez combiner des comparaisons avec `&&`, `||` et `!` ;
> - vous saurez ce que sont les valeurs « truthy » et « falsy », et utiliser `??` et `?.` ;
> - vous saurez répéter du code avec `while`, `for` et `for...of` ;
> - vous saurez sortir d'une boucle avec `break` ou sauter un tour avec `continue`.

## La condition if

Une [condition](../debuter/glossaire.md#condition) permet au programme de choisir. C'est
un aiguillage : selon la réponse à une question, le train part d'un côté ou de l'autre.

```javascript
const temperature = 28;

if (temperature > 25) {
  console.log("Il fait chaud.");
}
console.log("Fin du programme.");
```

```sortie
Il fait chaud.
Fin du programme.
```

Ligne par ligne :

- `const temperature = 28;` range 28 dans la constante `temperature` ;
- `if (temperature > 25)` pose une question : « 28 est-il plus grand que 25 ? ». La
  réponse est oui ;
- le `console.log` entre les accolades s'exécute donc ;
- le dernier `console.log` est en dehors des accolades : il s'exécute dans tous les cas,
  même s'il faisait froid.

- La question se met entre parenthèses `( )`.
- Le code à exécuter si la réponse est vraie se met entre accolades `{ }`. On appelle
  cela un **bloc**.
- Le bloc est décalé de deux espaces : c'est l'**indentation**. JavaScript ne l'exige
  pas, mais elle rend le code lisible.

### else et else if

Souvent, il n'y a pas qu'un seul cas. Une note peut être très bonne, moyenne ou
mauvaise, et vous voulez un message différent pour chacune, comme sur un bulletin.

`else` (« sinon ») donne le chemin à suivre quand la condition est fausse. `else if`
ajoute d'autres questions, posées dans l'ordre.

```javascript
const note = 13;

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

Dès qu'une question répond vrai, son bloc s'exécute et les suivantes sont ignorées.
Ici, `note >= 10` est vrai aussi, mais on ne le teste jamais.

**À vous :** écrivez un programme qui range `15` dans une constante `heure`, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```javascript
> const heure = 15;
>
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

## Les opérateurs de comparaison

Pour décider, un programme pose d'abord une question : « l'âge vaut-il 18 ? »,
« la température dépasse-t-elle 25 ? ». Les opérateurs de comparaison servent à
écrire ces questions.

Une comparaison répond par un [booléen](../debuter/glossaire.md#booleen) : `true` ou
`false`.

| Opérateur | Sens | Exemple vrai |
| --- | --- | --- |
| `===` | égal à | `3 === 3` |
| `!==` | différent de | `3 !== 4` |
| `<`, `>` | plus petit, plus grand | `2 < 5` |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal | `5 >= 5` |

```javascript
const age = 17;
console.log(age >= 18);
console.log(age !== 18);
console.log("abc" < "abd");
```

```sortie
false
true
true
```

Ligne par ligne :

- `age` vaut 17 ;
- `age >= 18` demande « 17 est-il plus grand que 18, ou égal ? ». Non : `false` ;
- `age !== 18` demande « 17 est-il différent de 18 ? ». Oui : `true` ;
- `"abc" < "abd"` : les deux textes commencent par « ab », et « c » vient avant « d »
  dans l'alphabet : `true`.

Les textes se comparent dans l'ordre alphabétique (plus exactement, l'ordre des codes
des caractères). Rappel de [la page précédente](premiers-pas.md) : utilisez `===` et
`!==`, jamais `==` et `!=`.

## Les opérateurs logiques

Une décision dépend souvent de plusieurs choses à la fois. Pour entrer au cinéma, il
faut avoir l'âge **et** un billet. Ces trois opérateurs relient plusieurs questions en
une seule.

Pour combiner plusieurs questions :

- `&&` (« et ») : vrai si **les deux** côtés sont vrais ;
- `||` (« ou ») : vrai si **au moins un** côté est vrai ;
- `!` (« non ») : inverse la réponse.

```javascript
const age = 25;
const aUnBillet = true;

if (age >= 18 && aUnBillet) {
  console.log("Entrée autorisée");
}

const jour = "dimanche";
if (jour === "samedi" || jour === "dimanche") {
  console.log("C'est le week-end");
}

console.log(!aUnBillet);
```

```sortie
Entrée autorisée
C'est le week-end
false
```

Ligne par ligne :

- `age` vaut 25 et `aUnBillet` vaut `true` ;
- premier `if` : `age >= 18` est vrai **et** `aUnBillet` est vrai, donc l'entrée est
  autorisée ;
- deuxième `if` : `jour` vaut `"dimanche"`. Le premier test est faux, le second est
  vrai, et avec `||` cela suffit ;
- le dernier `console.log` affiche `!aUnBillet` : `true` inversé devient `false`.

> **Astuce :** JavaScript s'arrête dès qu'il connaît la réponse. Dans `a && b`, si `a`
> est faux, `b` n'est même pas regardé.

**À vous :** écrivez un programme qui range `70` dans une constante `age`, puis
affiche « Tarif réduit » si l'âge est inférieur à 18 ou supérieur ou égal à 65, et
« Plein tarif » sinon.

> **Solution**
>
> ```javascript
> const age = 70;
>
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

## Truthy et falsy

Vous voulez souvent savoir si une case est remplie : un pseudo tapé, un nombre
différent de zéro. JavaScript permet de poser cette question très simplement, en
mettant la valeur seule dans le `if`.

Dans un `if`, vous pouvez mettre n'importe quelle valeur, pas seulement un booléen.
JavaScript la transforme alors en `true` ou `false`. Six valeurs deviennent `false` : on
les appelle **falsy** (« fausses-ish »).

- `false`
- `0` (et `-0`, `0n`)
- `""` (le texte vide)
- `null`
- `undefined`
- `NaN`

**Toutes** les autres valeurs sont **truthy** : elles deviennent `true`. Même `"0"`,
`"false"`, un tableau vide `[]` ou un objet vide `{}`.

```javascript
const valeurs = [0, 1, "", "salut", null, undefined, NaN, "0", []];

for (const v of valeurs) {
  console.log(v, "→", Boolean(v));
}
```

```sortie
0 → false
1 → true
 → false
salut → true
null → false
undefined → false
NaN → false
0 → true
[] → true
```

Ligne par ligne :

- `valeurs` est une liste de neuf valeurs, de toutes sortes ;
- `for (const v of valeurs)` prend ces valeurs une par une et range chacune dans `v`.
  Cette boucle est expliquée plus bas sur cette page ;
- `Boolean(v)` transforme la valeur en `true` ou `false`, et `console.log` affiche les
  deux côte à côte ;
- la troisième ligne de la sortie commence par un espace : c'est le texte vide `""`, qui
  n'affiche rien.

`Boolean(v)` montre en quoi une valeur se transforme. Cette règle sert souvent à tester
si un texte est rempli :

```javascript
const pseudo = "";

if (pseudo) {
  console.log(`Bonjour, ${pseudo}`);
} else {
  console.log("Pseudo vide !");
}
```

```sortie
Pseudo vide !
```

## L'opérateur ternaire

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Pour choisir entre **deux valeurs**, le ternaire est plus court qu'un `if` :
`condition ? valeurSiVrai : valeurSiFaux`.

```javascript
const age = 20;
const statut = age >= 18 ? "majeur" : "mineur";
console.log(statut);

const n = 3;
console.log(`${n} article${n > 1 ? "s" : ""}`);
```

```sortie
majeur
3 articles
```

Lisez-le comme une question : « âge ≥ 18 ? si oui majeur, sinon mineur ».

> **Attention :** gardez le ternaire pour des choix simples. Deux ternaires imbriqués se
> lisent très mal : préférez alors un `if`.

## Les valeurs absentes : ?? et ?.

Une valeur manque parfois : un réglage que l'utilisateur n'a pas choisi, une adresse
pas encore remplie. Ces deux opérateurs gèrent ce trou sans écrire un `if` à chaque
fois.

### ?? : une valeur de secours

`a ?? b` donne `a`, sauf si `a` vaut `null` ou `undefined` : il donne alors `b`.

```javascript
let couleur;
console.log(couleur ?? "bleu");

const volume = 0;
console.log(volume ?? 50);
console.log(volume || 50);
```

```sortie
bleu
0
50
```

Ligne par ligne :

- `let couleur;` crée une variable sans valeur : elle vaut `undefined`. `??` donne donc
  la valeur de secours `"bleu"` ;
- `volume` vaut `0`, qui n'est ni `null` ni `undefined` : `volume ?? 50` garde `0` ;
- `volume || 50` donne `50`, car `0` est falsy.

Comparez avec `||`, qu'on voit souvent dans le vieux code : `||` remplace **toutes** les
valeurs falsy, y compris `0`. Le volume `0` est une vraie valeur : `??` la garde, `||`
l'écrase.

### ?. : demander sans planter

`?.` (le **chaînage optionnel**) lit une propriété seulement si la valeur existe. Sinon,
il renvoie `undefined` au lieu de provoquer une erreur.

```javascript
const utilisateur = { nom: "Alice", adresse: null };

console.log(utilisateur.nom);
console.log(utilisateur.adresse?.ville);
console.log(utilisateur.adresse?.ville ?? "ville inconnue");
```

```sortie
Alice
undefined
ville inconnue
```

Les objets `{ ... }` sont expliqués dans [la page sur les objets](objets.md).

## Le choix multiple : switch

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Quand on compare **une même valeur** à plusieurs possibilités, `switch` est plus clair
qu'une longue suite de `else if`.

```javascript
const jour = "mardi";

switch (jour) {
  case "samedi":
  case "dimanche":
    console.log("Repos");
    break;
  case "mercredi":
    console.log("Demi-journée");
    break;
  default:
    console.log("Travail");
}
```

```sortie
Travail
```

- Chaque `case` compare avec `===`.
- `break` sort du `switch`. Sans lui, l'exécution **continue** dans le `case` suivant.
- Deux `case` à la suite partagent le même code (ici samedi et dimanche).
- `default` sert quand aucun `case` ne correspond.

## La boucle while

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100 `console.log`
serait long et pénible. Une boucle fait ce travail répétitif à votre place.

Une [boucle](../debuter/glossaire.md#boucle) répète un bloc de code. `while` (« tant
que ») le répète tant que la condition est vraie.

```javascript
let compteur = 3;

while (compteur > 0) {
  console.log(compteur);
  compteur--;
}
console.log("Décollage !");
```

```sortie
3
2
1
Décollage !
```

Ligne par ligne :

- `let compteur = 3;` : la variable vaut 3 au départ. C'est un `let`, car elle va
  changer ;
- `while (compteur > 0)` : avant chaque tour, JavaScript pose la question. `3 > 0` est
  vrai : il entre dans le bloc ;
- le bloc affiche `compteur`, puis `compteur--` lui retire 1 : il vaut 2, puis 1, puis 0 ;
- quand `compteur` vaut 0, `0 > 0` est faux : JavaScript sort de la boucle et passe au
  dernier `console.log`.

> **Attention :** si la condition reste toujours vraie, la boucle ne s'arrête jamais.
> Ici, `compteur--` rapproche le compteur de 0 à chaque tour. Pour arrêter un programme
> bloqué, tapez **Ctrl + C** dans le terminal.

**À vous :** écrivez un programme qui affiche les nombres de 1 à 5 avec une boucle
`while`, puis « Fini ! ».

> **Solution**
>
> ```javascript
> let n = 1;
>
> while (n <= 5) {
>   console.log(n);
>   n++;
> }
> console.log("Fini !");
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> Fini !
> ```

## La boucle for

Beaucoup de boucles comptent : de 1 à 5, de 1 à 10… Avec `while`, le départ, la
question et le pas sont éparpillés sur plusieurs lignes, et on en oublie facilement un.
`for` les rassemble.

`for` regroupe sur une ligne les trois morceaux d'une boucle qui compte :
`for (départ; condition; pas)`.

```javascript
for (let i = 1; i <= 5; i++) {
  console.log(`${i} x 3 = ${i * 3}`);
}
```

```sortie
1 x 3 = 3
2 x 3 = 6
3 x 3 = 9
4 x 3 = 12
5 x 3 = 15
```

1. **Départ** : `let i = 1`, exécuté une fois au début.
2. **Condition** : `i <= 5`, vérifiée avant chaque tour.
3. **Pas** : `i++`, exécuté après chaque tour.

**À vous :** écrivez un programme qui affiche la table de 7, de « 7 x 1 = 7 » à
« 7 x 5 = 35 », une ligne par calcul.

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

## for...of et for...in

Très souvent, vous avez une liste de choses à traiter une par une : chaque article
d'une liste de courses, chaque élève d'une classe. Ces deux boucles sont faites pour
cela.

### for...of : parcourir les valeurs

Pour parcourir les éléments d'un tableau (ou les lettres d'un texte), utilisez
`for...of`. Pas de compteur à gérer : vous recevez chaque valeur, une par une.

```javascript
const fruits = ["pomme", "poire", "kiwi"];

for (const fruit of fruits) {
  console.log(`J'aime la ${fruit}.`);
}

for (const lettre of "abc") {
  console.log(lettre);
}
```

```sortie
J'aime la pomme.
J'aime la poire.
J'aime la kiwi.
a
b
c
```

Ligne par ligne :

- `fruits` contient un tableau de trois textes ;
- `for (const fruit of fruits)` prend le premier élément, `"pomme"`, et le range dans
  `fruit` ;
- le bloc affiche la phrase avec `"pomme"`. Puis JavaScript revient au `for`, qui
  range `"poire"` dans `fruit`, et ainsi de suite ;
- après `"kiwi"`, il n'y a plus d'élément : la boucle s'arrête. La seconde boucle fait
  pareil avec les lettres de `"abc"`.

Ici `const` convient : à chaque tour, une **nouvelle** variable `fruit` est créée.

### for...in : parcourir les clés

`for...in` parcourt les **noms des propriétés** d'un objet, pas les valeurs.

```javascript
const scores = { alice: 12, bob: 8 };

for (const nom in scores) {
  console.log(nom, scores[nom]);
}
```

```sortie
alice 12
bob 8
```

> **À retenir :** `for...of` pour les **valeurs** d'un tableau ou d'un texte ;
> `for...in` pour les **clés** d'un objet. Ne mélangez pas les deux.

## break et continue

Parfois, vous voulez changer le cours d'une boucle en route : arrêter de chercher dès
que vous avez trouvé, ou ignorer certains éléments. C'est le rôle de ces deux mots.

- `break` sort tout de suite de la boucle.
- `continue` saute la fin du tour en cours et passe au suivant.

```javascript
const nombres = [4, 7, -1, 12, 0, 5];

for (const n of nombres) {
  if (n < 0) {
    continue;
  }
  if (n === 0) {
    break;
  }
  console.log(n);
}
```

```sortie
4
7
12
```

Ligne par ligne :

- `for (const n of nombres)` donne à `n` les valeurs 4, 7, -1, 12… une par une ;
- si `n` est négatif, `continue` passe tout de suite au tour suivant : le
  `console.log` n'est pas atteint ;
- si `n` vaut 0, `break` arrête toute la boucle ;
- sinon, `console.log(n)` affiche le nombre.

`-1` est sauté par `continue`. À `0`, `break` arrête tout : `5` n'est jamais affiché.

## Erreurs fréquentes

### Mettre const dans un for qui compte

```javascript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
0
file:///home/alice/js/main.mjs:1
for (const i = 0; i < 3; i++) {
                          ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:1:27

Node.js v22.22.0
```

Le premier tour marche, puis `i++` essaie de modifier une constante. Dans un `for`
classique, le compteur change : écrivez `let i = 0`. (Dans un `for...of`, `const` est
correct.)

### Écrire = au lieu de ===

```javascript norun
const note = 12;
if (note = 20) {
  console.log("Parfait !");
}
```

```text
file:///home/alice/js/main.mjs:2
if (note = 20) {
         ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:10

Node.js v22.22.0
```

Un seul `=` **range** une valeur, il ne compare pas. Ici, JavaScript essaie de mettre 20
dans `note`. Avec `let`, il n'y aurait pas d'erreur, mais la condition serait toujours
vraie : c'est pire. Comparez avec `===` : `if (note === 20)`.

### Oublier une accolade fermante

```javascript norun
const age = 20;
if (age >= 18) {
  console.log("Majeur");
```

```text
file:///home/alice/js/main.mjs:4



SyntaxError: Unexpected end of input

Node.js v22.22.0
```

Le fichier se termine (*end of input*) alors qu'un bloc `{` est encore ouvert. Chaque
`{` doit avoir son `}`. Un bon éditeur colore les paires d'accolades pour vous aider.

### Utiliser for...in sur un tableau

```javascript norun
const fruits = ["pomme", "poire"];
for (const i in fruits) {
  console.log(i + 1);
}
```

```text
01
11
```

`for...in` donne les **clés** du tableau, sous forme de textes `"0"` et `"1"`. Et
`"0" + 1` colle les deux : `"01"`. Pour les valeurs, utilisez `for...of`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
const x = 7;
if (x > 10) {
  console.log("grand");
} else if (x > 5) {
  console.log("moyen");
} else {
  console.log("petit");
}
```

> **Réponse**
>
> `moyen`. `x > 10` est faux, puis `x > 5` est vrai : son bloc s'exécute et on s'arrête.

**Question 2 :** que donnent `0 ?? 10` et `0 || 10` ?

> **Réponse**
>
> ```javascript
> console.log(0 ?? 10);
> console.log(0 || 10);
> ```
>
> ```sortie
> 0
> 10
> ```
>
> `??` ne remplace que `null` et `undefined`. `||` remplace toute valeur falsy, dont `0`.

**Question 3 :** écrivez une boucle qui affiche les nombres pairs de 2 à 10.

> **Réponse**
>
> ```javascript
> for (let i = 2; i <= 10; i += 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 2
> 4
> 6
> 8
> 10
> ```
>
> Le pas `i += 2` saute les nombres impairs.

## En résumé

- `if`, `else if` et `else` choisissent un chemin ; `switch` compare une valeur à
  plusieurs cas, sans oublier `break`.
- `&&`, `||` et `!` combinent des conditions ; comparez avec `===` et `!==`.
- `false`, `0`, `""`, `null`, `undefined` et `NaN` sont falsy ; tout le reste est truthy.
- `cond ? a : b` choisit une valeur ; `??` donne une valeur de secours ; `?.` lit sans
  planter.
- `while` répète tant qu'une condition est vraie, `for` compte, `for...of` parcourt les
  valeurs, `for...in` les clés d'un objet.
- `break` sort de la boucle, `continue` passe au tour suivant.

Suite : [les fonctions](fonctions.md).
