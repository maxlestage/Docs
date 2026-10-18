# Les fonctions

Cette page explique comment écrire des fonctions avec `fn`, leur donner des paramètres et un type de retour, et utiliser les closures.

## Déclarer une fonction

Une fonction se déclare avec `fn`, un nom en `minuscules_avec_tirets_bas`, des
parenthèses et un bloc :

```rust
fn saluer() {
    println!("Bonjour !");
}

fn main() {
    saluer();
    saluer();
}
```

```sortie
Bonjour !
Bonjour !
```

L'ordre n'a pas d'importance : une fonction peut être écrite avant ou après `main`.

## Les paramètres

Chaque paramètre a un nom **et un type**. Le type est obligatoire.

```rust
fn presenter(nom: &str, age: u32) {
    println!("{nom} a {age} ans.");
}

fn main() {
    presenter("Hugo", 12);
    presenter("Inès", 34);
}
```

```sortie
Hugo a 12 ans.
Inès a 34 ans.
```

## Le type de retour

Le type renvoyé s'écrit après une flèche `->`. La **dernière expression**, sans
point-virgule, est la valeur renvoyée :

```rust
fn carre(n: i32) -> i32 {
    n * n
}

fn main() {
    let resultat = carre(7);
    println!("{resultat}");
}
```

```sortie
49
```

Pour sortir plus tôt, utilisez `return` :

```rust
fn signe(n: i32) -> &'static str {
    if n < 0 {
        return "négatif";
    }
    if n == 0 {
        return "zéro";
    }
    "positif"
}

fn main() {
    println!("{} {} {}", signe(-3), signe(0), signe(8));
}
```

```sortie
négatif zéro positif
```

> **Note :** `&'static str` désigne un texte écrit directement dans le programme. Le
> `'static` est une durée de vie, présentée dans
> [Possession et emprunt](possession.md#les-durees-de-vie).

### Renvoyer plusieurs valeurs

Un **tuple** regroupe plusieurs valeurs :

```rust
fn division(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, reste) = division(17, 5);
    println!("{quotient} reste {reste}");
}
```

```sortie
3 reste 2
```

## Expressions et instructions

C'est une idée importante en Rust :

- une **expression** produit une valeur : `5`, `a + b`, `if … { … } else { … }`, un
  bloc `{ … }` ;
- une **instruction** fait une action et ne produit rien. Elle se termine par `;`.

Un bloc entre accolades est une expression. Sa valeur est sa dernière ligne :

```rust
let x = {
    let a = 3;
    let b = 4;
    a * b
};
println!("{x}");
```

```sortie
12
```

> **Attention :** un point-virgule en trop transforme l'expression en instruction. La
> fonction ne renvoie alors plus rien :

```rust norun
fn carre(n: i32) -> i32 {
    n * n; // erreur : mismatched types, expected `i32`, found `()`
}
```

## Les closures

Une **closure** est une petite fonction sans nom, que l'on range dans une variable. Ses
paramètres s'écrivent entre barres `|…|` :

```rust
let double = |x: i32| x * 2;
let ajouter = |a, b| a + b;
println!("{}", double(5));
println!("{}", ajouter(2, 3));
```

```sortie
10
5
```

### Capturer l'environnement

Une closure peut utiliser les variables qui l'entourent :

```rust
let bonus = 10;
let avec_bonus = |note: i32| note + bonus;
println!("{}", avec_bonus(12));
```

```sortie
22
```

### Passer une closure à une fonction

Les closures servent surtout à personnaliser le travail d'une autre fonction. Ici, le
paramètre `f` accepte toute fonction qui prend un `i32` et renvoie un `i32` :

```rust
fn appliquer_deux_fois(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", appliquer_deux_fois(|n| n + 3, 1));
    println!("{}", appliquer_deux_fois(|n| n * n, 3));
}
```

```sortie
7
81
```

On les retrouve partout avec les itérateurs : voir
[Collections](collections.md#les-iterateurs).

> **À retenir :** `fn nom(param: Type) -> Retour { … }` pour une fonction,
> `|param| expression` pour une closure.

Comparez avec [les fonctions en Ruby](../ruby/fonctions.md) et
[en Rythme](../rythme/fonctions.md).
