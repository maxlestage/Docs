# Structures, énumérations et traits

Cette page montre comment créer vos propres types avec `struct` et `enum`, leur ajouter des méthodes avec `impl`, et utiliser `Option` pour une valeur qui peut manquer.

## Les structures

Une **structure** (`struct`) regroupe plusieurs valeurs nommées, appelées **champs** :

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

Un bloc `impl` ajoute des fonctions à une structure. Le premier paramètre `self`
désigne l'objet lui-même :

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

## Les énumérations

Une **énumération** (`enum`) est un type qui vaut **une** valeur parmi une liste :

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

Chaque variante peut transporter ses propres données :

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

Rust n'a pas de valeur `null`. Une valeur qui peut manquer a le type `Option<T>` :

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

## Les traits

Un **trait** décrit un comportement : une liste de méthodes qu'un type promet
d'avoir. Le plus simple est de les obtenir avec `#[derive(...)]` :

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

## En résumé

- `struct` regroupe des champs nommés ; `impl` lui ajoute des méthodes.
- `&self` lit l'objet, `&mut self` le modifie.
- `enum` vaut une valeur parmi une liste de variantes.
- `match` traite chaque variante et oblige à n'en oublier aucune.
- `Option` vaut `Some(valeur)` ou `None` : Rust n'a pas de `null`.

Comparez avec [les types en Ruby](../ruby/types.md) et [en Rythme](../rythme/types.md).
Suite : [Génériques et traits](generiques.md).
