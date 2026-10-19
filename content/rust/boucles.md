# Les boucles

Cette page montre comment répéter du code avec `loop`, `while` et `for`, et comment changer le cours d'une boucle avec `break` et `continue`.

> **Objectifs :**
>
> - répéter du code avec `loop`, `while` et `for` ;
> - parcourir un intervalle ou une collection avec `for` ;
> - sortir d'une boucle, même imbriquée, avec `break` et `continue`.

Ce chapitre suit [les conditions](conditions-boucles.md) : une boucle utilise une condition pour savoir quand s'arrêter.

## La boucle loop

Imaginez qu'on vous demande d'afficher les nombres de 1 à 100. Écrire 100 `println!`
serait long et pénible. Une boucle fait ce travail répétitif à votre place.

Une [boucle](../debuter/glossaire.md#boucle) répète un bloc de code. `loop` répète
sans fin, jusqu'à un `break`. Le `break` peut renvoyer une valeur :

```rust
let mut n = 1;
let resultat = loop {
    n = n * 2;
    if n > 100 {
        break n;
    }
};
println!("{resultat}");
```

```sortie
128
```

Ligne par ligne :

- `n` vaut 1 au départ ; `mut` permet de le modifier ensuite ;
- à chaque tour, `n` est multiplié par 2 : il vaut 2, puis 4, 8, 16, 32, 64, 128 ;
- tant que `n` ne dépasse pas 100, le `if` ne fait rien et la boucle recommence ;
- quand `n` vaut 128, `break n;` arrête la boucle et donne 128, rangé dans
  `resultat`.

## La boucle while

Parfois, vous ne savez pas à l'avance combien de tours il faudra. Vous répétez tant
qu'une situation dure, comme on remue une soupe tant qu'elle n'est pas chaude.

`while` répète tant que la condition est vraie. Elle est testée avant chaque tour :

```rust
let mut compte = 3;
while compte > 0 {
    println!("{compte}...");
    compte -= 1;
}
println!("Partez !");
```

```sortie
3...
2...
1...
Partez !
```

Ligne par ligne :

- `compte` vaut 3 au départ ;
- avant chaque tour, Rust pose la question `compte > 0`. `3 > 0` est vrai : il entre
  dans le bloc ;
- le bloc affiche `compte`, puis lui retire 1 (`-= 1`) : il vaut 2, puis 1, puis 0 ;
- quand `compte` vaut 0, `0 > 0` est faux : Rust sort de la boucle et affiche
  « Partez ! ».

**À vous :** écrivez un programme qui affiche les nombres de 1 à 5 avec une boucle
`while`, puis « Fini ! ».

> **Solution**
>
> ```rust
> let mut n = 1;
> while n <= 5 {
>     println!("{n}");
>     n += 1;
> }
> println!("Fini !");
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

Très souvent, vous avez une suite de choses à traiter une par une : chaque article
d'une liste de courses, chaque nombre de 1 à 10. `for` est fait pour cela.

`for` parcourt un intervalle ou une collection, élément par élément. C'est la boucle
la plus utilisée.

### Sur un intervalle

`1..4` va de 1 à 3 (le 4 est exclu). `1..=4` va de 1 à 4 inclus. `rev()` parcourt à
l'envers.

```rust
for i in 1..4 {
    print!("{i} ");
}
println!();
for i in (1..=4).rev() {
    print!("{i} ");
}
println!();
```

```sortie
1 2 3 
4 3 2 1 
```

Ligne par ligne :

- `for i in 1..4` donne à `i` la valeur 1, puis 2, puis 3 ;
- `print!` affiche sans aller à la ligne : les nombres restent côte à côte ;
- `println!()`, vide, sert seulement à passer à la ligne ;
- la deuxième boucle fait pareil de 1 à 4, mais à l'envers grâce à `rev()`.

### Sur une collection

```rust
let fruits = ["pomme", "poire", "kiwi"];
for fruit in fruits {
    println!("J'aime la {fruit}");
}
for (i, fruit) in fruits.iter().enumerate() {
    println!("{i} : {fruit}");
}
```

```sortie
J'aime la pomme
J'aime la poire
J'aime la kiwi
0 : pomme
1 : poire
2 : kiwi
```

`enumerate()` donne, en plus de chaque élément, sa position (à partir de 0).

**À vous :** écrivez un programme qui affiche la table de 7, de « 7 x 1 = 7 » à
« 7 x 5 = 35 », une ligne par calcul.

> **Solution**
>
> ```rust
> for i in 1..=5 {
>     let resultat = 7 * i;
>     println!("7 x {i} = {resultat}");
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

## continue et break

Parfois, vous voulez changer le cours d'une boucle en route : arrêter de chercher dès
que vous avez trouvé, ou ignorer certains éléments. C'est le rôle de ces deux mots.

`continue` passe au tour suivant, `break` arrête la boucle :

```rust
for n in 1..10 {
    if n % 2 == 0 {
        continue;
    }
    if n > 7 {
        break;
    }
    print!("{n} ");
}
println!();
```

```sortie
1 3 5 7 
```

Ligne par ligne :

- `n` prend les valeurs 1, 2, 3… jusqu'à 9 ;
- si `n` est pair (le reste de la division par 2 vaut 0), `continue` passe tout de
  suite au tour suivant : le `print!` n'est pas atteint ;
- si `n` dépasse 7, `break` arrête toute la boucle : 9 n'est jamais affiché ;
- sinon, `print!` affiche le nombre.

### Sortir de deux boucles : les étiquettes

Dans deux boucles imbriquées, `break` n'arrête que la boucle intérieure. Pour arrêter
celle du dehors, donnez-lui une **étiquette** : un nom qui commence par une apostrophe,
comme `'grille`.

```rust
'grille: for ligne in 1..=3 {
    for colonne in 1..=3 {
        if ligne * colonne == 4 {
            println!("Trouvé en ({ligne}, {colonne})");
            break 'grille;
        }
    }
}
println!("Fin");
```

```sortie
Trouvé en (2, 2)
Fin
```

`continue 'grille` marche de la même façon : il passe au tour suivant de la boucle
étiquetée.
## Erreurs fréquentes

### Oublier mut dans une boucle

```rust norun
fn main() {
    let compte = 3;
    while compte > 0 {
        println!("{compte}");
        compte -= 1;
    }
}
```

```text
error[E0384]: cannot assign twice to immutable variable `compte`
 --> src/main.rs:5:9
  |
2 |     let compte = 3;
  |         ------ first assignment to `compte`
...
5 |         compte -= 1;
  |         ^^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

La boucle change `compte` à chaque tour. Déclarez-la modifiable :
`let mut compte = 3;`.

### Un break avec une valeur dans un for

```rust norun
fn main() {
    let nombres = [4, 8, 15];
    let trouve = for n in nombres {
        if n > 5 {
            break n;
        }
    };
}
```

```text
error[E0571]: `break` with value from a `for` loop
 --> src/main.rs:5:13
  |
3 |     let trouve = for n in nombres {
  |                  ---------------- you can't `break` with a value in a `for` loop
4 |         if n > 5 {
5 |             break n;
  |             ^^^^^^^ can only break with a value inside `loop` or breakable block
```

Seule `loop` peut donner une valeur avec `break` : une boucle `for` peut finir sans
jamais atteindre le `break`. Créez la variable avant la boucle
(`let mut trouve = 0;`), puis écrivez `trouve = n; break;` dans la boucle.

## Testez-vous

**1.** Combien de fois ce code affiche-t-il `bip` : `for _ in 0..3 { println!("bip"); }` ?

> **Réponse**
>
> Trois fois : `0..3` donne 0, 1 et 2 (le 3 est exclu).

**2.** Qu'affiche `for i in 1..=4 { if i == 2 { continue; } print!("{i} "); }` ?

> **Réponse**
>
> `1 3 4` : quand `i` vaut 2, `continue` passe au tour suivant sans rien afficher.

**3.** Dans deux boucles imbriquées, comment arrêter les deux d'un coup ?

> **Réponse**
>
> Donnez une étiquette à la boucle du dehors (`'dehors: for …`) et écrivez
> `break 'dehors;` dans la boucle intérieure.

## En résumé

- `loop` répète jusqu'à `break`, `while` tant qu'une condition est vraie.
- `for x in 1..=5` ou `for x in liste` : la boucle la plus courante.
- `continue` passe au tour suivant, `break` arrête la boucle ; une étiquette `'nom`
  vise une boucle précise.
- Une variable modifiée dans une boucle se déclare avec `let mut`.

Suite : [Les fonctions](fonctions.md).
