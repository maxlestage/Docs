# Génériques et traits

Cette page montre comment écrire une seule fonction pour plusieurs types grâce aux génériques, et comment décrire un comportement commun avec les traits.

## Le problème : du code en double

Imaginez deux fonctions qui font la même chose, pour deux types différents :

```rust
fn plus_grand_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn plus_grand_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", plus_grand_i32(3, 8));
    println!("{}", plus_grand_f64(2.5, 1.5));
}
```

```sortie
8
2.5
```

Le code est identique, seul le type change. Les **génériques** évitent cette répétition.

## Une fonction générique

On remplace le type par une lettre, souvent `T`, écrite entre `<` et `>` après le nom :

```rust
fn plus_grand<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", plus_grand(3, 8));
    println!("{}", plus_grand(2.5, 1.5));
    println!("{}", plus_grand("pomme", "kiwi"));
}
```

```sortie
8
2.5
pomme
```

Lisez `<T: PartialOrd>` ainsi : « pour tout type `T` qui sait se comparer ».

> **À retenir :** `: PartialOrd` est une **contrainte**. Sans elle, Rust refuse `a > b`,
> car il ne sait pas si `T` peut être comparé.

## Plusieurs contraintes

Une contrainte est un **trait** : une capacité que le type doit avoir. Pour en demander
plusieurs, reliez-les avec `+` :

```rust
use std::fmt::Display;

fn afficher_deux_fois<T: Display + Clone>(x: T) {
    let copie = x.clone();
    println!("{x} et {copie}");
}

fn main() {
    afficher_deux_fois(7);
    afficher_deux_fois("ok");
}
```

```sortie
7 et 7
ok et ok
```

## impl Trait en paramètre

Il existe une écriture plus courte : `impl Trait` à la place du type. Elle se lit
« n'importe quel type qui a ce trait » :

```rust
use std::fmt::Display;

fn encadrer(x: impl Display) -> String {
    format!("[ {x} ]")
}

fn main() {
    println!("{}", encadrer(42));
    println!("{}", encadrer("salut"));
    println!("{}", encadrer(3.5));
}
```

```sortie
[ 42 ]
[ salut ]
[ 3.5 ]
```

> **Astuce :** pour débuter, `impl Trait` est souvent l'écriture la plus lisible.

## Définir son propre trait

Un trait décrit un comportement : une liste de méthodes qu'un type promet d'avoir. Une
méthode peut aussi avoir un comportement **par défaut** :

```rust
trait Animal {
    fn nom(&self) -> String;
    fn cri(&self) -> String;

    // Méthode par défaut : les types n'ont pas besoin de l'écrire.
    fn presenter(&self) -> String {
        format!("{} fait {}", self.nom(), self.cri())
    }
}

struct Chien;

impl Animal for Chien {
    fn nom(&self) -> String {
        String::from("Le chien")
    }
    fn cri(&self) -> String {
        String::from("ouaf")
    }
}

fn main() {
    let rex = Chien;
    println!("{}", rex.presenter());
}
```

```sortie
Le chien fait ouaf
```

## Implémenter un trait pour deux structures

Chaque structure écrit sa propre version. Une fonction qui reçoit `&impl Animal`
accepte les deux :

```rust
trait Animal {
    fn nom(&self) -> String;
    fn cri(&self) -> String;
    fn presenter(&self) -> String {
        format!("{} fait {}", self.nom(), self.cri())
    }
}

struct Chien;
struct Chat {
    prenom: String,
}

impl Animal for Chien {
    fn nom(&self) -> String {
        String::from("Le chien")
    }
    fn cri(&self) -> String {
        String::from("ouaf")
    }
}

impl Animal for Chat {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
    fn cri(&self) -> String {
        String::from("miaou")
    }
    // Le chat remplace la méthode par défaut.
    fn presenter(&self) -> String {
        format!("{} dit {} (en baillant)", self.nom(), self.cri())
    }
}

fn faire_parler(a: &impl Animal) {
    println!("{}", a.presenter());
}

fn main() {
    faire_parler(&Chien);
    faire_parler(&Chat { prenom: String::from("Minou") });
}
```

```sortie
Le chien fait ouaf
Minou dit miaou (en baillant)
```

## Une liste d'objets différents

Un `Vec` ne contient qu'un seul type. Pour ranger un chien **et** un chat dans la même
liste, on utilise `Box<dyn Animal>` : « une boîte qui contient un animal, peu importe
lequel ».

```rust
trait Animal {
    fn cri(&self) -> String;
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn cri(&self) -> String {
        String::from("ouaf")
    }
}

impl Animal for Chat {
    fn cri(&self) -> String {
        String::from("miaou")
    }
}

fn main() {
    let animaux: Vec<Box<dyn Animal>> = vec![Box::new(Chien), Box::new(Chat), Box::new(Chien)];
    for a in &animaux {
        println!("{}", a.cri());
    }
}
```

```sortie
ouaf
miaou
ouaf
```

> **Note :** `Box::new(x)` range `x` dans une boîte. Le mot `dyn` signifie que le vrai
> type sera connu seulement pendant l'exécution.

## Les traits courants de la bibliothèque

Rust fournit des traits tout prêts. Beaucoup s'obtiennent gratuitement avec
`#[derive(...)]` :

| Trait | Ce qu'il permet | derive ? |
|-------|-----------------|----------|
| `Debug` | afficher avec `{:?}` | oui |
| `Clone` | copier avec `.clone()` | oui |
| `PartialEq` | comparer avec `==` | oui |
| `Default` | créer une valeur par défaut | oui |
| `Display` | afficher avec `{}` | non, à écrire |
| `From` | convertir depuis un autre type | non, à écrire |

### Debug, Clone, PartialEq et Default

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    let zero = Point::default();
    println!("{:?}", a);
    println!("{}", a == b);
    println!("{:?}", zero);
}
```

```sortie
Point { x: 1, y: 2 }
true
Point { x: 0, y: 0 }
```

### Display : un affichage personnalisé

Pour afficher un type avec `{}`, écrivez vous-même le trait `Display`. Copiez ce modèle,
seule la ligne `write!` change :

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
}
```

```sortie
Il fait 21.5 °C
```

### From : convertir un type en un autre

`From` explique comment fabriquer votre type à partir d'un autre. Vous obtenez alors
`.into()` en cadeau :

```rust
#[derive(Debug)]
struct Euros(f64);

impl From<f64> for Euros {
    fn from(montant: f64) -> Euros {
        Euros(montant)
    }
}

fn main() {
    let a = Euros::from(12.5);
    let prix: f64 = 3.0;
    let b: Euros = prix.into();
    println!("{:?} {:?}", a, b);
}
```

```sortie
Euros(12.5) Euros(3.0)
```

> **Astuce :** vous utilisez `From` depuis le début sans le savoir :
> `String::from("texte")`.

## En résumé

- `<T>` rend une fonction générique : elle marche pour plusieurs types.
- Une contrainte comme `T: Display` exige une capacité du type.
- `impl Trait` en paramètre est une écriture courte et lisible.
- Un trait liste des méthodes ; il peut fournir des méthodes par défaut.
- `Vec<Box<dyn Trait>>` range des objets de types différents.
- `#[derive(Debug, Clone, PartialEq, Default)]` écrit des traits pour vous.

Les types de base sont dans [Structures, énumérations et traits](types.md).
Suite : [Gérer les erreurs](erreurs.md).
