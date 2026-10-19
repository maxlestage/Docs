# Les conditions

Cette page montre comment choisir entre plusieurs chemins avec `if` et `match`, et comment lire une valeur facultative avec `if let`.

> **Objectifs :**
>
> - écrire une [condition](../debuter/glossaire.md#condition) avec `if`, `else if` et
>   `else` ;
> - comparer des valeurs avec `==`, `<`, `&&`, `||` et `!` ;
> - choisir parmi plusieurs cas avec `match` et `matches!` ;
> - lire une valeur facultative avec `if let` et `while let`.

## La condition if

Un programme doit souvent décider. S'il fait chaud, il conseille de boire ; s'il fait
froid, de prendre un manteau. `if` (« si ») sert à faire ce choix.

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

Ligne par ligne :

- `let temperature = 25;` range 25 dans la variable `temperature` ;
- premier test : « 25 est-il plus grand que 30 ? ». Non : Rust saute ce bloc ;
- deuxième test, avec `else if` : « 25 est-il plus grand que 15 ? ». Oui : Rust
  affiche « Il fait bon. » ;
- le bloc `else` n'est pas exécuté, car un test a déjà réussi.

Rust teste les conditions dans l'ordre et s'arrête à la première vraie. `else`
attrape tous les autres cas.

La condition doit être un vrai [booléen](../debuter/glossaire.md#booleen) (`bool`) :
`true` ou `false`. Un nombre ne suffit pas (voir [Erreurs
fréquentes](#erreurs-frequentes)).

### Les opérateurs de comparaison

| Opérateur | Sens |
|-----------|------|
| `==` / `!=` | égal / différent |
| `<` / `<=` | plus petit / plus petit ou égal |
| `>` / `>=` | plus grand / plus grand ou égal |
| `&&` / `\|\|` / `!` | et / ou / non |

```rust
let age = 15;
let accompagne = true;
if age >= 18 || accompagne {
    println!("Entrée autorisée");
}
if !(age >= 18) && age > 12 {
    println!("Adolescent");
}
```

```sortie
Entrée autorisée
Adolescent
```

**À vous :** écrivez un programme qui range `heure = 15` dans une variable, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```rust
> let heure = 15;
> if heure < 12 {
>     println!("Bonjour");
> } else if heure < 18 {
>     println!("Bon après-midi");
> } else {
>     println!("Bonsoir");
> }
> ```
>
> ```sortie
> Bon après-midi
> ```

## if est une expression

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Quand une valeur peut prendre beaucoup de formes, une longue suite de `else if`
devient pénible à lire. `match` range tous ces cas proprement, l'un sous l'autre.

`match` compare une valeur à plusieurs **motifs** (des formes possibles). Le premier
qui correspond gagne. Pensez à un tri postal : chaque lettre va dans la première case
qui lui convient.

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

Ligne par ligne :

- `jour` vaut 6 ;
- `match jour` compare 6 à chaque motif, de haut en bas : 1 ? non. 2, 3 ou 4 ? non.
  5 ? non ;
- `6 | 7` correspond : le `match` donne le texte `"week-end"`, rangé dans `nom` ;
- le motif `_` ne sert pas ici : il ne compte que si aucun autre motif ne correspond.

`|` signifie « ou ». Les motifs peuvent aussi être des intervalles :

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

**À vous :** écrivez un programme qui range `pieces = 3` dans une variable, puis
utilise `match` pour afficher « un » pour 1, « deux » pour 2, « trois » pour 3, et
« beaucoup » pour tout le reste.

> **Solution**
>
> ```rust
> let pieces = 3;
> match pieces {
>     1 => println!("un"),
>     2 => println!("deux"),
>     3 => println!("trois"),
>     _ => println!("beaucoup"),
> }
> ```
>
> ```sortie
> trois
> ```

### Les gardes

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un motif peut être suivi de `if` et d'une condition : c'est une **garde**. Le nom `n`
reçoit la valeur testée :

```rust
let temperature = -4;
let etat = match temperature {
    n if n < 0 => "glace",
    0..=99 => "liquide",
    _ => "vapeur",
};
println!("{etat}");
```

```sortie
glace
```

### Tester un seul motif avec matches!

`matches!(valeur, motif)` répond `true` ou `false`. C'est pratique dans un `if` :

```rust
let lettre = 'e';
let voyelle = matches!(lettre, 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
println!("{voyelle}");
println!("{}", matches!(42, 0..=9));
```

```sortie
true
false
```

`match` devient encore plus utile avec les énumérations : voir
[Structures, énumérations et traits](types.md#match-sur-une-enumeration).

## if let et while let

Pensez à une boîte aux lettres : parfois il y a une lettre, parfois rien. Avant de
lire la lettre, il faut vérifier qu'elle est là. `if let` et `while let` font cette
vérification pour vous.

Certaines valeurs sont **facultatives** : elles valent `Some(x)` s'il y a quelque chose,
ou `None` s'il n'y a rien. C'est le type `Option`, présenté en détail dans
[Structures, énumérations et traits](types.md#le-type-option).

`if let` exécute un bloc seulement si la valeur correspond au motif, et donne un nom à
ce qu'elle contient :

```rust
let mut pile = vec![1, 2, 3];
if let Some(haut) = pile.pop() {
    println!("Retiré : {haut}");
}
```

```sortie
Retiré : 3
```

`pop()` retire le dernier élément d'une liste et renvoie `Some(élément)`, ou `None` si
la liste est vide. `while let` répète tant que le motif correspond :

```rust
let mut pile = vec![1, 2, 3];
while let Some(haut) = pile.pop() {
    print!("{haut} ");
}
println!();
```

```sortie
3 2 1 
```

> **Astuce :** `if let` est un `match` raccourci, quand un seul cas vous intéresse.
## Erreurs fréquentes

### Une condition qui n'est pas un booléen

```rust norun
fn main() {
    let n = 1;
    if n {
        println!("oui");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if n {
  |        ^ expected `bool`, found integer
```

Rust ne transforme pas un nombre en booléen. Écrivez la comparaison : `if n != 0`.

### = au lieu de ==

```rust norun
fn main() {
    let x = 5;
    if x = 5 {
        println!("cinq");
    }
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if x = 5 {
  |        ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
```

`=` range une valeur, `==` compare. Écrivez `if x == 5`.

### Un match qui oublie des cas

```rust norun
fn main() {
    let note = 14;
    match note {
        0..=9 => println!("Insuffisant"),
        10..=20 => println!("Bien"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match note {
  |           ^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
```

Un `i32` peut aussi valoir `-5` ou `300`. Ajoutez une dernière ligne
`_ => println!("Note invalide"),`.

### Deux branches de types différents

```rust norun
fn main() {
    let age = 20;
    let statut = if age >= 18 { "majeur" } else { 0 };
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:3:51
  |
3 |     let statut = if age >= 18 { "majeur" } else { 0 };
  |                                 --------          ^ expected `&str`, found integer
  |                                 |
  |                                 expected because of this
```

Une variable n'a qu'un type : les deux branches doivent donner du texte, par exemple
`else { "mineur" }`.

## Testez-vous

**1.** Qu'affiche ce code ?

```rust norun
let x = 7;
if x > 10 {
    println!("grand");
} else if x > 5 {
    println!("moyen");
} else {
    println!("petit");
}
```

> **Réponse**
>
> `moyen` : 7 n'est pas plus grand que 10, mais il est plus grand que 5. Rust
> s'arrête à la première condition vraie.

**2.** Écrivez un `match` qui affiche `petit` pour 0 à 9, `moyen` pour 10 à 99 et
`grand` sinon, pour `n = 42`.

> **Réponse**
>
> ```rust
> let n = 42;
> match n {
>     0..=9 => println!("petit"),
>     10..=99 => println!("moyen"),
>     _ => println!("grand"),
> }
> ```
>
> ```sortie
> moyen
> ```

**3.** Que fait `if let Some(haut) = pile.pop() { … }` quand la pile est vide ?

> **Réponse**
>
> `pop()` renvoie `None`, qui ne correspond pas au motif `Some(haut)` : le bloc
> n'est pas exécuté, et le programme continue après.

## En résumé

- `if … else` choisit un chemin, et peut renvoyer une valeur.
- `==`, `<`, `&&`, `||` et `!` construisent des conditions ; une condition est
  toujours un `bool`.
- `match` compare une valeur à des motifs ; `_` attrape le reste ; `matches!` teste un
  seul motif.
- `if let Some(x) = …` et `while let` lisent une valeur facultative.

Comparez avec [Ruby](../ruby/conditions-boucles.md) et
[Rythme](../rythme/conditions-boucles.md).

Suite : [Les boucles](boucles.md).
