# Génériques et traits

Cette page montre comment écrire une seule fonction pour plusieurs types grâce aux génériques, et comment décrire un comportement commun avec les traits.

> **Objectifs :**
>
> - écrire une fonction et une structure génériques avec `<T>` ;
> - limiter un type générique avec une contrainte (`T: Display`) ;
> - définir un trait et l'implémenter pour plusieurs types ;
> - ranger des objets de types différents avec `Box<dyn Trait>` ;
> - utiliser les traits courants : `Debug`, `Clone`, `Display`, `From`.

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

Une fonction générique est comme un moule à gâteau : la forme est la même, mais vous
choisissez la pâte à chaque fois. On remplace le type par une lettre, souvent `T`
(pour *type*), écrite entre `<` et `>` après le nom :

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

Une contrainte est un **trait** : une capacité que le type doit avoir. Une offre
d'emploi fonctionne pareil : « tout candidat qui sait conduire **et** parle anglais ».
Pour demander plusieurs traits, reliez-les avec `+`. Ici, `Display` permet l'affichage
avec `{}` :

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

## Une structure générique

Une structure peut aussi avoir un type générique. `Vec<T>` et `Option<T>` sont faits
ainsi. Voici une boîte qui contient une valeur de n'importe quel type :

```rust
#[derive(Debug)]
struct Boite<T> {
    contenu: T,
}

impl<T> Boite<T> {
    fn ouvrir(self) -> T {
        self.contenu
    }
}

fn main() {
    let a = Boite { contenu: 42 };
    let b = Boite { contenu: "secret" };
    println!("{:?}", a);
    println!("{}", b.ouvrir());
}
```

```sortie
Boite { contenu: 42 }
secret
```

`impl<T> Boite<T>` se lit : « pour tout type `T`, voici les méthodes de `Boite<T>` ».

## Définir son propre trait

Un trait décrit un comportement : une liste de méthodes qu'un type promet d'avoir.
C'est un contrat : « tout animal sait dire son nom et pousser son cri ». On l'écrit avec
`trait`, puis chaque type le signe avec `impl Trait for Type`. Une méthode peut aussi
avoir un comportement **par défaut** :

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

`struct Chien;` est une structure sans champ : elle ne contient rien, mais elle peut
avoir des méthodes.

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

## Erreurs fréquentes

### Oublier la contrainte

```rust norun
fn plus_grand<T>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", plus_grand(3, 8));
}
```

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:2:10
  |
2 |     if a > b { a } else { b }
  |        - ^ - T
  |        |
  |        T
help: consider restricting type parameter `T` with trait `PartialOrd`
```

Rust ne sait pas si n'importe quel `T` se compare. Écrivez `<T: PartialOrd>`.

### Oublier une méthode du trait

```rust norun
trait Animal {
    fn nom(&self) -> String;
    fn cri(&self) -> String;
}

struct Chien;

impl Animal for Chien {
    fn nom(&self) -> String {
        String::from("Rex")
    }
}

fn main() {}
```

```text
error[E0046]: not all trait items implemented, missing: `cri`
 --> src/main.rs:8:1
  |
3 |     fn cri(&self) -> String;
  |     ------------------------ `cri` from trait
...
8 | impl Animal for Chien {
  | ^^^^^^^^^^^^^^^^^^^^^ missing `cri` in implementation
```

Signer le contrat oblige à écrire toutes les méthodes sans version par défaut. Ajoutez
`fn cri(&self) -> String { … }`.

### Mettre deux types dans le même Vec

```rust norun
struct Chien;
struct Chat;

fn main() {
    let animaux = vec![Chien, Chat];
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:5:31
  |
5 |     let animaux = vec![Chien, Chat];
  |                               ^^^^ expected `Chien`, found `Chat`
```

Donnez-leur un trait commun, puis rangez des boîtes :
`Vec<Box<dyn Animal>>` (voir [Une liste d'objets différents](#une-liste-d-objets-differents)).

### Afficher avec {} sans Display

```rust norun
struct Euros(f64);

fn main() {
    let prix = Euros(12.5);
    println!("{prix}");
}
```

```text
error[E0277]: `Euros` doesn't implement `std::fmt::Display`
 --> src/main.rs:5:15
  |
5 |     println!("{prix}");
  |               ^^^^^^ `Euros` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Écrivez `impl fmt::Display for Euros` (voir
[Display](#display-un-affichage-personnalise)), ou ajoutez `#[derive(Debug)]` et
affichez avec `{:?}`.

## Testez-vous

**1.** Que signifie `fn f<T: Clone>(x: T)` ?

> **Réponse**
>
> `f` accepte une valeur de n'importe quel type `T`, à condition que ce type sache se
> copier avec `.clone()`.

**2.** Écrivez un trait `Aire` avec une méthode `aire(&self) -> f64`, et
implémentez-le pour une structure `Carre { cote: f64 }`.

> **Réponse**
>
> ```rust
> trait Aire {
>     fn aire(&self) -> f64;
> }
>
> struct Carre {
>     cote: f64,
> }
>
> impl Aire for Carre {
>     fn aire(&self) -> f64 {
>         self.cote * self.cote
>     }
> }
>
> fn main() {
>     println!("{}", Carre { cote: 3.0 }.aire());
> }
> ```
>
> ```sortie
> 9
> ```

**3.** Pourquoi écrire `Box<dyn Animal>` plutôt que `Chien` dans un `Vec` ?

> **Réponse**
>
> Pour ranger des animaux de types différents dans la même liste. `dyn Animal` veut
> dire « n'importe quel type qui a le trait `Animal` ».

## En résumé

- `<T>` rend une fonction ou une structure générique : elle marche pour plusieurs
  types.
- Une contrainte comme `T: Display` exige une capacité du type.
- `impl Trait` en paramètre est une écriture courte et lisible.
- Un trait liste des méthodes ; il peut fournir des méthodes par défaut.
- `Vec<Box<dyn Trait>>` range des objets de types différents.
- `#[derive(Debug, Clone, PartialEq, Default)]` écrit des traits pour vous.

Les types de base sont dans [Structures, énumérations et traits](types.md).

Suite : [Les objets traits : dyn Trait](traits-objets.md)
