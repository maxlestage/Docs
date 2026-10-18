# Les fonctions

Cette page explique comment écrire des fonctions avec `fn`, leur donner des paramètres et un type de retour, et utiliser les closures.

> **Objectifs :**
>
> - déclarer une fonction avec `fn` et l'appeler ;
> - lui donner des paramètres typés et un type de retour ;
> - comprendre la différence entre expression et instruction ;
> - renvoyer plusieurs valeurs avec un tuple ;
> - écrire une closure et la passer à une autre fonction.

## Déclarer une fonction

Une [fonction](../debuter/glossaire.md#fonction) est un morceau de code qui porte un
nom. Vous l'écrivez une fois, puis vous l'**appelez** autant de fois que vous voulez,
comme une recette de cuisine que l'on refait.

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

Un [paramètre](../debuter/glossaire.md#parametre) est une variable que la fonction
reçoit. La valeur donnée au moment de l'appel s'appelle un
[argument](../debuter/glossaire.md#argument).

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

Ici, `nom` et `age` sont les paramètres ; `"Hugo"` et `12` sont les arguments.

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

Une fonction sans `->` renvoie `()`, prononcé « unit » : une valeur vide, qui veut
dire « rien ».

### Renvoyer plusieurs valeurs

Un **tuple** regroupe plusieurs valeurs (voir [Premiers pas](premiers-pas.md#les-tuples)) :

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
> fonction ne renvoie alors plus rien (voir [Erreurs fréquentes](#erreurs-frequentes)).

## Une fonction qui s'appelle elle-même

Une fonction peut s'appeler elle-même : c'est la
[récursion](../debuter/glossaire.md#recursion). Il faut toujours un cas d'arrêt, sinon
elle ne s'arrête jamais :

```rust
fn factorielle(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorielle(n - 1)
    }
}

fn main() {
    println!("{}", factorielle(5));
}
```

```sortie
120
```

## Les closures

Une [closure](../debuter/glossaire.md#closure) est une petite fonction sans nom, que
l'on range dans une variable. Ses paramètres s'écrivent entre barres `|…|`. Les types
sont souvent devinés :

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

Une closure peut utiliser les variables qui l'entourent. Elle les **capture**, comme
un sac à dos qu'elle emporte avec elle :

```rust
let bonus = 10;
let avec_bonus = |note: i32| note + bonus;
println!("{}", avec_bonus(12));
```

```sortie
22
```

Une closure qui modifie une variable capturée doit être rangée dans une variable `mut` :

```rust
let mut total = 0;
let mut ajouter = |n: i32| total += n;
ajouter(5);
ajouter(7);
println!("{total}");
```

```sortie
12
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

Lisez `impl Fn(i32) -> i32` ainsi : « n'importe quelle fonction qui prend un `i32` et
renvoie un `i32` ». On les retrouve partout avec les itérateurs : voir
[Collections](collections.md#les-iterateurs).

## Erreurs fréquentes

### Oublier le type d'un paramètre

```rust norun
fn double(x) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```text
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:1:12
  |
1 | fn double(x) -> i32 {
  |            ^ expected one of `:`, `@`, or `|`
help: if this is a parameter name, give it a type
```

Correction : `fn double(x: i32) -> i32`.

### Un point-virgule après la valeur renvoyée

```rust norun
fn carre(n: i32) -> i32 {
    n * n;
}

fn main() {
    println!("{}", carre(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:1:21
  |
1 | fn carre(n: i32) -> i32 {
  |    -----            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * n;
  |          - help: remove this semicolon to return this value
```

Correction : enlevez le `;` après `n * n`.

### Oublier le type de retour

```rust norun
fn carre(n: i32) {
    n * n
}

fn main() {
    println!("{:?}", carre(3));
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn carre(n: i32) {
  |                 - help: try adding a return type: `-> i32`
2 |     n * n
  |     ^^^^^ expected `()`, found `i32`
```

Sans `->`, la fonction ne renvoie rien. Ajoutez `-> i32`.

### Oublier un argument

```rust norun
fn presenter(nom: &str, age: u32) {
    println!("{nom} a {age} ans.");
}

fn main() {
    presenter("Hugo");
}
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> src/main.rs:6:5
  |
6 |     presenter("Hugo");
  |     ^^^^^^^^^-------- argument #2 of type `u32` is missing
```

Donnez un argument par paramètre, dans l'ordre : `presenter("Hugo", 12);`.

## Testez-vous

**1.** Que renvoie `fn f() -> i32 { 1; 2 }` ?

> **Réponse**
>
> `2` : c'est la dernière expression, sans point-virgule.

**2.** Écrivez une fonction `moyenne(a: f64, b: f64) -> f64`.

> **Réponse**
>
> ```rust
> fn moyenne(a: f64, b: f64) -> f64 {
>     (a + b) / 2.0
> }
>
> fn main() {
>     println!("{}", moyenne(12.0, 15.0));
> }
> ```
>
> ```sortie
> 13.5
> ```

**3.** Écrivez une closure `triple` qui multiplie un nombre par 3.

> **Réponse**
>
> ```rust
> let triple = |x: i32| x * 3;
> println!("{}", triple(4));
> ```
>
> ```sortie
> 12
> ```

## En résumé

- `fn nom(param: Type) -> Retour { … }` déclare une fonction.
- Le type de chaque paramètre est obligatoire.
- La dernière expression, sans `;`, est la valeur renvoyée ; `return` sort plus tôt.
- Un tuple `(a, b)` renvoie plusieurs valeurs.
- `|x| x * 2` est une closure : une petite fonction sans nom, qui peut capturer les
  variables autour d'elle.

Comparez avec [les fonctions en Ruby](../ruby/fonctions.md) et
[en Rythme](../rythme/fonctions.md).

Suite : [Travailler avec du texte](texte.md).
