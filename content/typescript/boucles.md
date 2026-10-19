# Les boucles

Cette page explique comment répéter des actions avec les boucles `for`, `while` et `for...of`, et comment les arrêter avec `break` et `continue`.

> **Objectifs :**
>
> - vous saurez répéter avec `for`, `while` et `for...of` ;
> - vous saurez sauter un tour avec `continue` et sortir d'une boucle avec `break`.

Ce chapitre suit [les conditions](conditions-boucles.md) : une boucle s'arrête grâce à une condition, comme celles que vous savez déjà écrire.

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

## Erreurs fréquentes

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

### Déclarer le compteur avec const

```typescript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
main.ts:1:26 - error TS2588: Cannot assign to 'i' because it is a constant.

1 for (const i = 0; i < 3; i++) {
                           ~


Found 1 error in main.ts:1
```

« Impossible de modifier `i` : c'est une constante. » `i++` change le compteur à
chaque tour : il doit être créé avec `let`, comme dans `for (let i = 0; i < 3; i++)`.
Dans un `for...of`, au contraire, `const` convient : chaque tour crée un nouvel
élément.

## Testez-vous

**Question 1 :** écrivez une boucle qui affiche les nombres de 10 à 0, de 2 en 2.

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

**Question 2 :** qu'affiche ce code ?

```typescript norun
let n = 1;
while (n < 20) {
  n = n * 3;
}
console.log(n);
```

> **Réponse**
>
> `27`. `n` vaut 1, puis 3, 9 et 27. À 27, `n < 20` est faux : la boucle s'arrête et
> `console.log` affiche 27.

**Question 3 :** quelle est la différence entre `break` et `continue` ?

> **Réponse**
>
> `break` sort de la boucle tout de suite. `continue` saute la fin du tour et passe
> directement au tour suivant : la boucle continue.

## En résumé

- Une boucle répète un bloc de code.
- `for` compte : un départ, une condition pour continuer et un pas.
- `while` répète tant que sa condition est vraie : quelque chose doit changer à chaque
  tour.
- `for...of` parcourt une liste, élément par élément.
- `break` sort de la boucle, `continue` passe au tour suivant.

Suite : [les fonctions](fonctions.md).
