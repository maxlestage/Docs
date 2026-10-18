# Structures, énumérations et traits

Cette page montre comment créer vos propres types avec `struct` et `enum`, leur ajouter des méthodes avec `impl`, et partager des comportements avec les traits.

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
| `self` | la méthode consomme l'objet |
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
fn chercher(noms: &[&str], cible: &str) -> Option<usize> {
    for (i, nom) in noms.iter().enumerate() {
        if *nom == cible {
            return Some(i);
        }
    }
    None
}

fn main() {
    let noms = ["Ana", "Bo", "Cy"];
    match chercher(&noms, "Bo") {
        Some(i) => println!("Trouvé à la position {i}"),
        None => println!("Absent"),
    }
    if let Some(i) = chercher(&noms, "Cy") {
        println!("Cy est en {i}");
    }
    println!("{:?}", chercher(&noms, "Dan"));
}
```

```sortie
Trouvé à la position 1
Cy est en 2
None
```

> **Astuce :** `if let` est un raccourci pour un `match` qui ne s'intéresse qu'à un seul
> cas.

## Les traits

Un **trait** décrit un comportement : une liste de méthodes qu'un type promet
d'avoir.

```rust
trait Animal {
    fn nom(&self) -> String;
    fn cri(&self) -> String;

    // Une méthode avec un comportement par défaut.
    fn presenter(&self) -> String {
        format!("{} fait {}", self.nom(), self.cri())
    }
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn nom(&self) -> String { String::from("Le chien") }
    fn cri(&self) -> String { String::from("ouaf") }
}

impl Animal for Chat {
    fn nom(&self) -> String { String::from("Le chat") }
    fn cri(&self) -> String { String::from("miaou") }
}

fn faire_parler(a: &impl Animal) {
    println!("{}", a.presenter());
}

fn main() {
    faire_parler(&Chien);
    faire_parler(&Chat);
}
```

```sortie
Le chien fait ouaf
Le chat fait miaou
```

### Les traits automatiques avec derive

`#[derive(...)]` demande au compilateur d'écrire certains traits pour vous :

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

### Un affichage personnalisé avec Display

Pour afficher un type avec `{}`, implémentez le trait `Display` :

```rust
use std::fmt;

struct Temperature {
    degres: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} °C", self.degres)
    }
}

fn main() {
    let t = Temperature { degres: 21.456 };
    println!("Il fait {t}");
    let texte = t.to_string();
    println!("{}", texte.len());
}
```

```sortie
Il fait 21.5 °C
8
```

Comparez avec [les types en Ruby](../ruby/types.md) et [en Rythme](../rythme/types.md).
Suite : [Gérer les erreurs](erreurs.md).
