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

Ligne par ligne :

- `fn saluer() { … }` décrit la fonction : son nom, `saluer`, et ce qu'elle fait. À ce
  moment, rien ne s'affiche encore ;
- le programme commence toujours par `main` ;
- le premier `saluer();` appelle la fonction : Rust exécute son bloc, affiche
  « Bonjour ! », puis revient dans `main` ;
- le second `saluer();` refait exactement la même chose.

L'ordre n'a pas d'importance : une fonction peut être écrite avant ou après `main`.

## Les paramètres

Une fonction qui fait toujours exactement la même chose sert peu. Les paramètres lui
donnent des informations différentes à chaque appel, comme une recette de crêpes que
l'on fait pour 2 ou pour 6 personnes.

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

Ligne par ligne :

- `fn presenter(nom: &str, age: u32)` annonce deux paramètres : un texte `nom` et un
  nombre entier positif `age` ;
- `presenter("Hugo", 12);` appelle la fonction : `nom` vaut `"Hugo"` et `age` vaut
  `12`, puis le `println!` affiche la phrase ;
- le second appel recommence avec d'autres valeurs : `"Inès"` et `34`.

Ici, `nom` et `age` sont les paramètres ; `"Hugo"` et `12` sont les arguments.

**À vous :** écrivez une fonction `afficher_prix` qui reçoit le nom d'un article
(`&str`) et son prix en euros (`u32`), puis affiche par exemple « Pain : 2 euros ».
Appelez-la pour le pain à 2 euros et pour le fromage à 5 euros.

> **Solution**
>
> ```rust
> fn afficher_prix(article: &str, prix: u32) {
>     println!("{article} : {prix} euros");
> }
>
> fn main() {
>     afficher_prix("Pain", 2);
>     afficher_prix("Fromage", 5);
> }
> ```
>
> ```sortie
> Pain : 2 euros
> Fromage : 5 euros
> ```

## Le type de retour

Une fonction peut aussi faire un calcul et vous **rendre** le résultat, comme une
calculatrice. Vous pouvez alors ranger ce résultat dans une variable et vous en
resservir.

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

Ligne par ligne :

- `fn carre(n: i32) -> i32` : la fonction reçoit un entier `n` et promet de rendre un
  entier ;
- dans `main`, `carre(7)` appelle la fonction : `n` vaut 7 ;
- la dernière ligne de la fonction, `n * n`, calcule 49. Elle n'a pas de `;` : c'est
  la valeur renvoyée ;
- `let resultat = carre(7);` range donc 49 dans `resultat`, que `println!` affiche.

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

**À vous :** écrivez une fonction `perimetre` qui reçoit le côté d'un carré (`i32`)
et renvoie son périmètre, c'est-à-dire 4 fois le côté. Affichez le périmètre d'un
carré de côté 5.

> **Solution**
>
> ```rust
> fn perimetre(cote: i32) -> i32 {
>     4 * cote
> }
>
> fn main() {
>     println!("{}", perimetre(5));
> }
> ```
>
> ```sortie
> 20
> ```

## Expressions et instructions

Cette différence explique une règle que vous venez de voir : la dernière ligne d'une
fonction n'a pas de point-virgule quand c'est elle qui donne la valeur renvoyée.

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

Ligne par ligne :

- `let x = {` ouvre un bloc dont la valeur ira dans `x` ;
- `let a = 3;` et `let b = 4;` sont des instructions : elles finissent par `;` et ne
  produisent pas de valeur ;
- `a * b` n'a pas de `;` : c'est une expression, qui vaut 12. C'est la valeur du
  bloc ;
- `x` reçoit donc 12, et `println!` l'affiche.

> **Attention :** un point-virgule en trop transforme l'expression en instruction. La
> fonction ne renvoie alors plus rien (voir [Erreurs fréquentes](#erreurs-frequentes)).

## Une fonction qui s'appelle elle-même

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Ligne par ligne :

- `factorielle(5)` calcule `5 * factorielle(4)` : il lui faut d'abord la réponse
  pour 4 ;
- de la même façon, `factorielle(4)` demande `factorielle(3)`, puis 2, puis 1 ;
- `factorielle(1)` est le cas d'arrêt : il renvoie 1 sans s'appeler ;
- les réponses remontent : 2 × 1 = 2, puis 3 × 2 = 6, 4 × 6 = 24 et 5 × 24 = 120.

## Les closures

Parfois, vous avez besoin d'une toute petite fonction, juste à un endroit : par
exemple pour dire à une autre fonction quel calcul faire. L'écrire à part avec `fn`
serait lourd. Une closure s'écrit sur une ligne, là où vous en avez besoin.

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

Ligne par ligne :

- `let double = |x: i32| x * 2;` range une closure dans `double` : elle reçoit `x` et
  renvoie `x * 2` ;
- `let ajouter = |a, b| a + b;` reçoit deux valeurs et renvoie leur somme. Rust
  devine leur type ;
- `double(5)` s'appelle comme une fonction et vaut 10 ; `ajouter(2, 3)` vaut 5.

**À vous :** écrivez une closure `est_pair` qui reçoit un `i32` et renvoie `true` si
le nombre est pair (le reste de sa division par 2 vaut 0). Affichez le résultat pour 4,
puis pour 7.

> **Solution**
>
> ```rust
> let est_pair = |n: i32| n % 2 == 0;
> println!("{}", est_pair(4));
> println!("{}", est_pair(7));
> ```
>
> ```sortie
> true
> false
> ```

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
