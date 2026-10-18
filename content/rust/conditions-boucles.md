# Conditions et boucles

Cette page montre comment choisir entre plusieurs chemins avec `if` et `match`, et comment répéter du code avec `loop`, `while` et `for`.

## La condition if

`if` exécute un bloc seulement si la condition est vraie. Pas besoin de parenthèses,
mais les accolades sont obligatoires.

```rust
let temperature = 25;
if temperature > 30 {
    println!("Il fait chaud.");
} else if temperature > 15 {
    println!("Il fait bon.");
} else {
    println!("Il fait frais.");
}
```

```sortie
Il fait bon.
```

La condition doit être un vrai `bool` :

```rust norun
let n = 1;
if n { } // erreur : expected `bool`, found integer
```

### Les opérateurs de comparaison

| Opérateur | Sens |
|-----------|------|
| `==` / `!=` | égal / différent |
| `<` / `<=` | plus petit / plus petit ou égal |
| `>` / `>=` | plus grand / plus grand ou égal |
| `&&` / `\|\|` / `!` | et / ou / non |

## if est une expression

En Rust, `if` **renvoie une valeur**. Vous pouvez la ranger dans une variable :

```rust
let age = 20;
let statut = if age >= 18 { "majeur" } else { "mineur" };
println!("{statut}");
```

```sortie
majeur
```

> **Attention :** les deux branches doivent donner le même type.

## Le choix avec match

`match` compare une valeur à plusieurs **motifs**. Le premier qui correspond gagne.

```rust
let jour = 6;
let nom = match jour {
    1 => "lundi",
    2 | 3 | 4 => "milieu de semaine",
    5 => "vendredi",
    6 | 7 => "week-end",
    _ => "jour inconnu",
};
println!("{nom}");
```

```sortie
week-end
```

Les motifs peuvent aussi être des intervalles :

```rust
let note = 14;
match note {
    0..=9 => println!("Insuffisant"),
    10..=15 => println!("Bien"),
    _ => println!("Très bien"),
}
```

```sortie
Bien
```

> **À retenir :** `match` doit couvrir **tous** les cas. Le motif `_` attrape tout le
> reste.

`match` devient encore plus utile avec les énumérations : voir
[Structures, énumérations et traits](types.md#match-sur-une-enumeration).

## La boucle loop

`loop` répète sans fin, jusqu'à un `break`. Le `break` peut renvoyer une valeur :

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

## La boucle while

`while` répète tant que la condition est vraie :

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

## La boucle for

`for` parcourt un intervalle ou une collection. C'est la boucle la plus utilisée.

### Sur un intervalle

`1..4` va de 1 à 3. `1..=4` va de 1 à 4 inclus.

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

## continue et break

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

## En résumé

- `if … else` choisit un chemin, et peut renvoyer une valeur.
- `match` compare une valeur à des motifs ; `_` attrape le reste.
- `loop` répète jusqu'à `break`, `while` tant qu'une condition est vraie.
- `for x in 1..=5` ou `for x in liste` : la boucle la plus courante.
- `continue` passe au tour suivant, `break` arrête la boucle.

Comparez avec [Ruby](../ruby/conditions-boucles.md) et
[Rythme](../rythme/conditions-boucles.md). Suite : [Les fonctions](fonctions.md).
