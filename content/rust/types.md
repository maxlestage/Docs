# Structures, énumérations et traits

Cette page montre comment créer vos propres types avec `struct` et `enum`, leur ajouter des méthodes avec `impl`, et utiliser `Option` pour une valeur qui peut manquer.

> **Objectifs :**
>
> - regrouper des données dans une structure (`struct`) ;
> - lui ajouter des méthodes et des fonctions associées avec `impl` ;
> - décrire une valeur parmi plusieurs choix avec une énumération (`enum`) ;
> - traiter une valeur absente avec `Option`, `match` et `if let` ;
> - obtenir des traits courants avec `#[derive(...)]`.

## Les structures

Une **structure** (`struct`) regroupe plusieurs valeurs nommées, appelées **champs**.
C'est comme un formulaire : chaque case a un nom et attend un certain type de réponse.
Le nom d'une structure commence par une majuscule.

```rust
struct Personne {
    nom: String,
    age: u32,
}

fn main() {
    let p = Personne {
        nom: String::from("Alice"),
        age: 30,
    };
    println!("{} a {} ans", p.nom, p.age);
}
```

```sortie
Alice a 30 ans
```

Pour modifier un champ, la variable doit être `mut` :

```rust
struct Compteur {
    valeur: u32,
}

fn main() {
    let mut c = Compteur { valeur: 0 };
    c.valeur += 5;
    println!("{}", c.valeur);
}
```

```sortie
5
```

## Les méthodes avec impl

Un bloc `impl` (pour *implémentation*) ajoute des fonctions à une structure. Une
[méthode](../debuter/glossaire.md#methode) est une fonction attachée à un type, que
l'on appelle avec un point : `r.aire()`. Son premier paramètre, `self`, désigne
l'objet lui-même :

```rust
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    // Une fonction associée : elle crée un Rectangle.
    fn carre(cote: f64) -> Rectangle {
        Rectangle { largeur: cote, hauteur: cote }
    }

    // Une méthode qui lit l'objet.
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    // Une méthode qui modifie l'objet.
    fn agrandir(&mut self, facteur: f64) {
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }
}

fn main() {
    let mut r = Rectangle { largeur: 3.0, hauteur: 2.0 };
    println!("{}", r.aire());
    r.agrandir(2.0);
    println!("{}", r.aire());
    let c = Rectangle::carre(4.0);
    println!("{}", c.aire());
}
```

```sortie
6
24
16
```

| Premier paramètre | Sens |
|-------------------|------|
| `&self` | la méthode lit l'objet |
| `&mut self` | la méthode modifie l'objet |
| aucun | fonction associée, appelée avec `Type::nom()` |

### Le constructeur new

Par habitude, la fonction associée qui crée un objet s'appelle `new`. Dans un bloc
`impl`, `Self` est un raccourci pour le nom du type :

```rust
struct Joueur {
    nom: String,
    points: u32,
}

impl Joueur {
    fn new(nom: &str) -> Self {
        Self { nom: nom.to_string(), points: 0 }
    }

    fn marquer(&mut self, n: u32) {
        self.points += n;
    }
}

fn main() {
    let mut j = Joueur::new("Sam");
    j.marquer(3);
    j.marquer(2);
    println!("{} : {} points", j.nom, j.points);
}
```

```sortie
Sam : 5 points
```

`String::new()` et `Vec::new()`, que vous connaissez déjà, sont des fonctions associées
du même genre.

## Les énumérations

Une **énumération** (`enum`) est un type qui vaut **une** valeur parmi une liste de
choix, appelés **variantes**. Un feu tricolore est vert, orange **ou** rouge, jamais
deux à la fois. On écrit une variante avec `::` : `Feu::Orange`.

```rust
enum Feu {
    Vert,
    Orange,
    Rouge,
}

fn main() {
    let f = Feu::Orange;
    let action = match f {
        Feu::Vert => "passez",
        Feu::Orange => "ralentissez",
        Feu::Rouge => "arrêtez-vous",
    };
    println!("{action}");
}
```

```sortie
ralentissez
```

### Des variantes avec des données

Chaque variante peut transporter ses propres données : un nombre pour le cercle (son
rayon), deux champs nommés pour le rectangle, rien pour le point.

```rust
enum Forme {
    Cercle(f64),
    Rectangle { largeur: f64, hauteur: f64 },
    Point,
}
```

## match sur une énumération

`match` ouvre chaque variante et récupère ses données :

```rust
enum Forme {
    Cercle(f64),
    Rectangle { largeur: f64, hauteur: f64 },
    Point,
}

fn aire(f: &Forme) -> f64 {
    match f {
        Forme::Cercle(rayon) => 3.14 * rayon * rayon,
        Forme::Rectangle { largeur, hauteur } => largeur * hauteur,
        Forme::Point => 0.0,
    }
}

fn main() {
    let formes = vec![
        Forme::Cercle(1.0),
        Forme::Rectangle { largeur: 2.0, hauteur: 5.0 },
        Forme::Point,
    ];
    for f in &formes {
        println!("{}", aire(f));
    }
}
```

```sortie
3.14
10
0
```

> **À retenir :** si vous oubliez une variante dans un `match`, le programme ne compile
> pas. Ajouter une variante vous montre tous les endroits à mettre à jour.

## Le type Option

Beaucoup de langages ont une valeur [null](../debuter/glossaire.md#null), qui veut dire
« rien », et qui provoque des erreurs quand on l'oublie. Rust n'a pas de `null`. Une
valeur qui peut manquer a le type `Option<T>` : une boîte qui contient une valeur de
type `T`, ou qui est vide.

- `Some(valeur)` : il y a une valeur ;
- `None` : il n'y a rien.

```rust
fn diviser(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match diviser(10, 2) {
        Some(n) => println!("Résultat : {n}"),
        None => println!("Division impossible"),
    }
    if let Some(n) = diviser(9, 3) {
        println!("9 / 3 = {n}");
    }
    println!("{:?}", diviser(1, 0));
}
```

```sortie
Résultat : 5
9 / 3 = 3
None
```

> **Astuce :** `if let` est un raccourci pour un `match` qui ne s'intéresse qu'à un seul
> cas.

### Les méthodes utiles d'Option

Pour les cas simples, des méthodes évitent d'écrire un `match` :

```rust
let present: Option<i32> = Some(4);
let absent: Option<i32> = None;
println!("{} {}", present.is_some(), absent.is_none());
println!("{}", absent.unwrap_or(0));
println!("{:?}", present.map(|n| n * 10));
```

```sortie
true true
0
Some(40)
```

| Méthode | Effet |
|---------|-------|
| `is_some()` / `is_none()` | y a-t-il une valeur ? |
| `unwrap_or(v)` | la valeur, ou `v` si la boîte est vide |
| `map(f)` | transforme la valeur si elle existe |
| `unwrap()` | la valeur, ou arrête le programme si la boîte est vide |

## Les traits

Un **trait** décrit un comportement : une liste de méthodes qu'un type promet
d'avoir. C'est comme un permis : un type qui a le trait `Clone` « a le droit » d'être
copié avec `.clone()`. Le plus simple est d'obtenir les traits courants avec
`#[derive(...)]`, une ligne placée juste avant la structure :

| Trait | Ce qu'il permet |
|-------|-----------------|
| `Debug` | afficher avec `{:?}` |
| `Clone` | copier avec `.clone()` |
| `PartialEq` | comparer avec `==` |

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{:?}", a);
    println!("{}", a == b);
}
```

```sortie
Point { x: 1, y: 2 }
true
```

Écrire vos propres traits est expliqué dans [Génériques et traits](generiques.md).

## Erreurs fréquentes

### Oublier un champ

```rust norun
struct Personne {
    nom: String,
    age: u32,
}

fn main() {
    let p = Personne { nom: String::from("Alice") };
}
```

```text
error[E0063]: missing field `age` in initializer of `Personne`
 --> src/main.rs:7:13
  |
7 |     let p = Personne { nom: String::from("Alice") };
  |             ^^^^^^^^ missing `age`
```

Tous les champs doivent recevoir une valeur : `Personne { nom: …, age: 30 }`.

### Afficher une structure sans Debug

```rust norun
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

```text
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:8:22
  |
8 |     println!("{:?}", p);
  |               ----   ^ `Point` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |               |
  |               required by this formatting parameter
  |
  = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
```

Ajoutez `#[derive(Debug)]` juste avant `struct Point`.

### Appeler une méthode &mut self sur une variable non mut

```rust norun
struct Compteur {
    valeur: u32,
}

impl Compteur {
    fn augmenter(&mut self) {
        self.valeur += 1;
    }
}

fn main() {
    let c = Compteur { valeur: 0 };
    c.augmenter();
}
```

```text
error[E0596]: cannot borrow `c` as mutable, as it is not declared as mutable
  --> src/main.rs:13:5
   |
13 |     c.augmenter();
   |     ^ cannot borrow as mutable
```

La méthode modifie l'objet : la variable doit être `mut`. `let mut c = …`.

### Oublier une variante dans un match

```rust norun
enum Feu {
    Vert,
    Orange,
    Rouge,
}

fn main() {
    let f = Feu::Rouge;
    match f {
        Feu::Vert => println!("passez"),
        Feu::Orange => println!("ralentissez"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Feu::Rouge` not covered
  --> src/main.rs:9:11
   |
 9 |     match f {
   |           ^ pattern `Feu::Rouge` not covered
```

Ajoutez la variante oubliée : `Feu::Rouge => println!("arrêtez-vous"),`.

## Testez-vous

**1.** Quelle différence entre `fn aire(&self)` et `fn carre(cote: f64)` dans un bloc
`impl Rectangle` ?

> **Réponse**
>
> `aire` est une méthode : on l'appelle sur un objet, `r.aire()`. `carre` est une
> fonction associée, sans objet : on l'appelle avec `Rectangle::carre(4.0)`.

**2.** Écrivez une énumération `Piece` avec les variantes `Pile` et `Face`.

> **Réponse**
>
> ```rust
> #[derive(Debug)]
> enum Piece {
>     Pile,
>     Face,
> }
>
> fn main() {
>     let p = Piece::Face;
>     println!("{:?}", p);
> }
> ```
>
> ```sortie
> Face
> ```

**3.** Que vaut `None.unwrap_or(7)` ? Et `Some(3).unwrap_or(7)` ?

> **Réponse**
>
> `7` puis `3` : `unwrap_or` donne la valeur si elle existe, sinon la valeur de
> secours.

## En résumé

- `struct` regroupe des champs nommés ; `impl` lui ajoute des méthodes.
- `&self` lit l'objet, `&mut self` le modifie ; sans `self`, c'est une fonction
  associée, comme `Type::new()`.
- `enum` vaut une valeur parmi une liste de variantes.
- `match` traite chaque variante et oblige à n'en oublier aucune.
- `Option` vaut `Some(valeur)` ou `None` : Rust n'a pas de `null`.
- `#[derive(Debug, Clone, PartialEq)]` donne des traits courants à un type.

Comparez avec [les types en Ruby](../ruby/types.md) et [en Rythme](../rythme/types.md).

Suite : [Génériques et traits](generiques.md).
