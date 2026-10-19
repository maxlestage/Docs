# Les boucles

Cette page montre comment répéter des actions avec `while`, `for` et `for...of`, et comment arrêter une boucle ou sauter un tour.

> **Objectifs :**
>
> - vous saurez répéter du code avec `while`, `for` et `for...of` ;
> - vous saurez sortir d'une boucle avec `break` ou sauter un tour avec `continue` ;
> - vous saurez choisir entre `for...of` (les valeurs) et `for...in` (les clés).

Ce chapitre suit [les conditions](conditions-boucles.md) : une boucle s'arrête grâce à
une condition, comme celles que vous écrivez dans un `if`.

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

**Question 1 :** combien de fois ce code affiche-t-il « Bonjour » ?

```javascript norun
let n = 0;
while (n < 3) {
  console.log("Bonjour");
  n++;
}
```

> **Réponse**
>
> Trois fois. `n` vaut 0, 1 puis 2 pendant les tours ; quand il vaut 3, `n < 3` est
> faux et la boucle s'arrête.

**Question 2 :** qu'affiche ce code ?

```javascript norun
for (const n of [1, 2, 3, 4, 5]) {
  if (n === 2) {
    continue;
  }
  if (n === 4) {
    break;
  }
  console.log(n);
}
```

> **Réponse**
>
> `1` puis `3`. `2` est sauté par `continue`. À `4`, `break` arrête la boucle : `5` n'est jamais
> atteint.

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

- `while` répète tant qu'une condition est vraie, `for` compte, `for...of` parcourt les
  valeurs, `for...in` les clés d'un objet.
- `break` sort de la boucle, `continue` passe au tour suivant.
- Dans un `for` qui compte, le compteur se déclare avec `let` ; dans un `for...of`,
  `const` convient.

Suite : [les fonctions](fonctions.md).
