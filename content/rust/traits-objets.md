# Les objets traits : dyn Trait

Ce chapitre explique comment manipuler des valeurs de types différents qui partagent un même trait, grâce à `dyn Trait`, et quand préférer les génériques.

> **Objectifs :**
>
> - comprendre la différence entre un générique (statique) et `dyn Trait` (dynamique) ;
> - utiliser `&dyn Trait` et `Box<dyn Trait>` ;
> - ranger des valeurs de types différents dans un `Vec<Box<dyn Trait>>` ;
> - choisir entre générique et `dyn` ;
> - reconnaître un trait qui ne peut pas devenir `dyn` ;
> - renvoyer une closure avec `Box<dyn Fn>`.

Ce chapitre suit [Génériques et traits](generiques.md). Relisez-le si le mot
[trait](../debuter/glossaire.md#trait) vous semble flou.

## Le point de départ : des formes et un trait

Prenons trois formes : un carré, un rectangle et un cercle. Elles sont toutes
différentes, mais elles savent toutes deux choses : donner leur **nom** et leur
**aire** (leur surface). Ce point commun devient un trait, `Forme` :

```rust
use std::f64::consts::PI;

trait Forme {
    fn nom(&self) -> String;
    fn aire(&self) -> f64;
}

struct Carre {
    cote: f64,
}

struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

struct Cercle {
    rayon: f64,
}

impl Forme for Carre {
    fn nom(&self) -> String {
        String::from("carré")
    }
    fn aire(&self) -> f64 {
        self.cote * self.cote
    }
}

impl Forme for Rectangle {
    fn nom(&self) -> String {
        String::from("rectangle")
    }
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }
}

impl Forme for Cercle {
    fn nom(&self) -> String {
        String::from("cercle")
    }
    fn aire(&self) -> f64 {
        PI * self.rayon * self.rayon
    }
}

fn main() {
    let c = Carre { cote: 2.0 };
    let r = Rectangle { largeur: 2.0, hauteur: 3.0 };
    let o = Cercle { rayon: 1.0 };
    println!("{} : {:.2}", c.nom(), c.aire());
    println!("{} : {:.2}", r.nom(), r.aire());
    println!("{} : {:.2}", o.nom(), o.aire());
}
```

```sortie
carré : 4.00
rectangle : 6.00
cercle : 3.14
```

`{:.2}` affiche deux chiffres après la virgule. Pour garder les exemples courts, la
suite de la page n'utilise que le carré et le cercle.

## Première façon : le générique (statique)

Vous connaissez déjà `impl Forme` en paramètre. Il veut dire « n'importe quel type qui
a le trait `Forme` » :

```rust
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);
struct Cercle(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn decrire(f: &impl Forme) {
    println!("aire : {}", f.aire());
}

fn main() {
    decrire(&Carre(2.0));
    decrire(&Cercle(1.0));
}
```

```sortie
aire : 4
aire : 3
```

`struct Carre(f64);` est une structure avec un seul champ sans nom : on le lit avec
`self.0`. Ici, le cercle prend 3 pour π, pour avoir des nombres ronds.

Que fait Rust en coulisses ? Pendant la compilation, il **recopie** la fonction pour
chaque type utilisé. C'est comme un imprimeur qui prépare un tampon par forme :

```text
votre code :     decrire(f: &impl Forme)

ce que Rust en fait, à la compilation :
                 decrire_pour_Carre(f: &Carre)    → appelle Carre::aire
                 decrire_pour_Cercle(f: &Cercle)  → appelle Cercle::aire
```

On dit que c'est **statique** : tout est décidé avant que le programme tourne.

> **À retenir :** un générique produit une copie de la fonction par type. C'est très
> rapide, mais chaque copie ne connaît qu'**un seul** type.

## Deuxième façon : dyn Trait (dynamique)

Écrivez `&dyn Forme` à la place de `&impl Forme`. Le programme affiche la même chose :

```rust
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);
struct Cercle(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn decrire(f: &dyn Forme) {
    println!("aire : {}", f.aire());
}

fn main() {
    decrire(&Carre(2.0));
    decrire(&Cercle(1.0));
}
```

```sortie
aire : 4
aire : 3
```

Mais cette fois, il n'existe **qu'une seule** fonction `decrire`. Elle ne sait pas si
elle reçoit un carré ou un cercle. Le mot `dyn` (pour *dynamique*) veut dire : « le vrai
type sera découvert pendant l'exécution ».

Une valeur `&dyn Forme` s'appelle un **objet trait**.

## Comment ça marche : la table des méthodes

Comment `decrire` trouve-t-elle la bonne méthode `aire` ? Un `&dyn Forme` n'est pas
une simple adresse. C'est **deux** informations collées ensemble :

1. où se trouve la valeur (le carré lui-même) ;
2. où se trouve la **table des méthodes** de son type, qu'on appelle la *vtable*.

```text
  &dyn Forme
  ┌────────────┬────────────┐
  │  données ──┼──► Carre(2.0)
  │  table   ──┼──► vtable de Carre
  └────────────┴────────────┘          ┌──────────────────────┐
                                       │ aire → Carre::aire   │
                                       └──────────────────────┘
```

Pensez à une valise avec une étiquette de mode d'emploi attachée. Celui qui la reçoit
n'ouvre pas la valise pour deviner ce qu'elle contient : il lit l'étiquette, qui dit
quelle méthode appeler.

Un appel `f.aire()` se passe en deux temps :

| Étape | Ce que fait le programme |
|-------|--------------------------|
| 1 | il lit la table attachée à `f` |
| 2 | il y trouve l'adresse de la bonne `aire`, puis l'appelle |

Vous pouvez vérifier que l'objet trait est deux fois plus gros qu'une simple
référence :

```rust
use std::mem::size_of;

trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);

fn main() {
    println!("{}", size_of::<&dyn Forme>() == 2 * size_of::<&Carre>());
}
```

```sortie
true
```

> **À retenir :** `dyn Trait` = une seule fonction, et un petit détour par la table à
> chaque appel. C'est un tout petit peu plus lent, mais beaucoup plus souple.

## Box<dyn Trait> : posséder une forme

`&dyn Forme` **emprunte** une forme qui existe ailleurs. Pour **posséder** une forme
dont on ne connaît pas le type exact, on la range dans une `Box` :

```rust
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let f: Box<dyn Forme> = Box::new(Carre(3.0));
    println!("{}", f.aire());
}
```

```sortie
9
```

`Box::new(x)` met `x` dans une boîte, rangée dans une zone de la mémoire appelée le
tas. La `Box` elle-même a toujours la même taille, quelle que soit la forme qu'elle
contient. C'est pour cela que Rust l'accepte. Box est expliquée en détail dans
[Box, Rc et RefCell](pointeurs.md).

Pourquoi pas `let f: dyn Forme = …` tout court ? Parce qu'un carré et un cercle n'ont
pas la même taille. Rust doit connaître la taille de chaque variable avant de lancer le
programme. Une boîte, elle, a toujours la même taille.

Essayez : remplacez `Carre(3.0)` par une autre forme qui a le trait `Forme`. Le type de
`f` ne change pas : c'est toujours `Box<dyn Forme>`.

## Vec<Box<dyn Trait>> : une liste de formes mélangées

Voici enfin la réponse au problème de départ : une liste qui contient des carrés **et**
des cercles.

```rust
trait Forme {
    fn nom(&self) -> String;
    fn aire(&self) -> f64;
}

struct Carre(f64);
struct Cercle(f64);

impl Forme for Carre {
    fn nom(&self) -> String {
        String::from("carré")
    }
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forme for Cercle {
    fn nom(&self) -> String {
        String::from("cercle")
    }
    fn aire(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn main() {
    let formes: Vec<Box<dyn Forme>> = vec![
        Box::new(Carre(2.0)),
        Box::new(Cercle(1.0)),
        Box::new(Carre(1.0)),
    ];

    let mut total = 0.0;
    for f in &formes {
        println!("{} : {}", f.nom(), f.aire());
        total += f.aire();
    }
    println!("total : {total}");
}
```

```sortie
carré : 4
cercle : 3
carré : 1
total : 8
```

Suivons la boucle pas à pas :

| Étape | f | type réel | aire | total |
|-------|---|-----------|------|-------|
| début | – | – | – | 0 |
| 1 | `formes[0]` | `Carre(2.0)` | 4 | 4 |
| 2 | `formes[1]` | `Cercle(1.0)` | 3 | 7 |
| 3 | `formes[2]` | `Carre(1.0)` | 1 | 8 |

Dans la mémoire, le `Vec` contient trois boîtes de même taille. Chacune montre sa
forme et sa table :

```text
formes : [ boîte | boîte | boîte ]
             │       │       │
             ▼       ▼       ▼
          Carre   Cercle   Carre
          (table  (table   (table
          Carre)  Cercle)  Carre)
```

Essayez : ajoutez `Box::new(Cercle(2.0))` à la liste. Une ligne `cercle : 12` apparaît
et le total passe à 20.

### Fabriquer une forme selon un choix

Une fonction peut renvoyer une `Box<dyn Forme>` dont le type dépend d'une valeur
connue seulement pendant l'exécution :

```rust
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);
struct Cercle(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        3.0 * self.0 * self.0
    }
}

fn fabriquer(lettre: char, taille: f64) -> Box<dyn Forme> {
    if lettre == 'c' {
        Box::new(Carre(taille))
    } else {
        Box::new(Cercle(taille))
    }
}

fn main() {
    let commande = "c2 o1 c3";
    let mut formes = Vec::new();
    for mot in commande.split(' ') {
        let lettre = mot.chars().next().unwrap();
        let taille: f64 = mot[1..].parse().unwrap();
        formes.push(fabriquer(lettre, taille));
    }
    for f in &formes {
        println!("{}", f.aire());
    }
}
```

```sortie
4
3
9
```

Avec un générique, ce serait impossible : les deux branches du `if` doivent avoir le
même type. Ici, ce type commun est `Box<dyn Forme>`.

## Générique ou dyn : lequel choisir ?

| Question | Générique (`impl Trait`, `<T: Trait>`) | `dyn Trait` |
|----------|----------------------------------------|-------------|
| Quand le type est-il choisi ? | à la compilation | à l'exécution |
| Plusieurs types dans un même `Vec` ? | non | oui, avec `Box` |
| Vitesse d'appel | la plus rapide | un petit détour par la table |
| Taille du programme | une copie par type | une seule fonction |
| Tous les traits possibles ? | oui | non, voir plus bas |

Une règle simple pour débuter :

- **un seul type à la fois** (une fonction qui reçoit « une forme ») : le générique ;
- **plusieurs types mélangés** (une liste de formes, un choix fait pendant
  l'exécution) : `dyn`.

> **À retenir :** commencez par `impl Trait`. Passez à `dyn Trait` quand vous avez
> besoin de **mélanger** des types différents.

## Renvoyer une closure avec `Box<dyn Fn>`

Une [closure](fonctions.md#les-closures) est une petite fonction sans nom. Chaque
closure a son propre type, que personne ne peut écrire. Pour qu'une fonction renvoie
**l'une ou l'autre** de deux closures, on utilise le trait `Fn` en version `dyn` :

```rust
fn reglage(n: i32, doubler: bool) -> Box<dyn Fn(i32) -> i32> {
    if doubler {
        Box::new(move |x| x * n)
    } else {
        Box::new(move |x| x + n)
    }
}

fn main() {
    let f = reglage(3, true);
    let g = reglage(3, false);
    println!("{}", f(5));
    println!("{}", g(5));
}
```

```sortie
15
8
```

Lisez `Box<dyn Fn(i32) -> i32>` ainsi : « une boîte qui contient une fonction, n'importe
laquelle, qui prend un `i32` et renvoie un `i32` ». `move` fait emporter `n` dans la
closure.

On peut aussi ranger des closures différentes dans un `Vec` :

```rust
fn main() {
    let etapes: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 10),
        Box::new(|x| x - 3),
    ];
    let mut valeur = 2;
    for etape in &etapes {
        valeur = etape(valeur);
        println!("{valeur}");
    }
}
```

```sortie
3
30
27
```

| Étape | calcul | valeur |
|-------|--------|--------|
| début | – | 2 |
| 1 | 2 + 1 | 3 |
| 2 | 3 × 10 | 30 |
| 3 | 30 − 3 | 27 |

Les closures ont leur chapitre complet : [Les closures en détail](closures.md).

## Quand un trait ne peut pas devenir dyn

Certains traits ne peuvent pas servir d'objet trait. Rust dit alors que le trait
n'est pas *dyn compatible* (on disait avant *object safe*).

La raison est simple. Un objet trait n'a que sa **table des méthodes**. Si une méthode
a besoin de connaître le vrai type pour exister, elle ne peut pas entrer dans la
table. Voici les deux cas les plus courants :

| Le trait contient… | Pourquoi ça bloque |
|--------------------|--------------------|
| une fonction sans `self` qui renvoie `Self` (`fn nouvelle() -> Self`) | sans valeur, impossible de lire une table ; et `Self` n'a pas de taille connue |
| une méthode générique (`fn comparer<T>(&self, x: T)`) | il faudrait une case dans la table pour **chaque** type `T` possible |

Voici le message réel :

```rust norun
trait Forme {
    fn nouvelle() -> Self;
    fn aire(&self) -> f64;
}

fn afficher(f: &dyn Forme) {
    println!("{}", f.aire());
}

fn main() {}
```

```text
error[E0038]: the trait `Forme` is not dyn compatible
 --> src/main.rs:6:17
  |
6 | fn afficher(f: &dyn Forme) {
  |                 ^^^^^^^^^ `Forme` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> src/main.rs:2:8
  |
1 | trait Forme {
  |       ----- this trait is not dyn compatible...
2 |     fn nouvelle() -> Self;
  |        ^^^^^^^^ ...because associated function `nouvelle` has no `self` parameter
help: consider turning `nouvelle` into a method by giving it a `&self` argument
  |
2 |     fn nouvelle(&self) -> Self;
  |                 +++++
help: alternatively, consider constraining `nouvelle` so it does not apply to trait objects
  |
2 |     fn nouvelle() -> Self where Self: Sized;
  |                           +++++++++++++++++
```

Le message dit bien « it needs to allow building a vtable » : il faut pouvoir
construire la table. La seconde aide est la plus pratique : `where Self: Sized` retire
cette fonction de la table. Le reste du trait devient utilisable avec `dyn` :

```rust
trait Forme {
    fn nouvelle() -> Self
    where
        Self: Sized;
    fn aire(&self) -> f64;
}

struct Carre(f64);

impl Forme for Carre {
    fn nouvelle() -> Self {
        Carre(1.0)
    }
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

fn afficher(f: &dyn Forme) {
    println!("{}", f.aire());
}

fn main() {
    let c = Carre::nouvelle();
    afficher(&c);
}
```

```sortie
1
```

> **Note :** le trait `Clone` est dans ce cas : `clone` renvoie `Self`. Un trait qui
> exige `Clone` (`trait Forme: Clone`) ne peut donc pas devenir `dyn Forme`.

## Erreurs fréquentes

### Oublier le mot dyn

```rust norun
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let f: Box<Forme> = Box::new(Carre(2.0));
}
```

```text
error[E0782]: expected a type, found a trait
  --> src/main.rs:14:16
   |
14 |     let f: Box<Forme> = Box::new(Carre(2.0));
   |                ^^^^^
   |
help: you can add the `dyn` keyword if you want a trait object
   |
14 |     let f: Box<dyn Forme> = Box::new(Carre(2.0));
   |                +++
```

Un trait n'est pas un type. Écrivez `Box<dyn Forme>`.

### Un Vec<dyn Trait> sans Box

```rust norun
trait Forme {
    fn aire(&self) -> f64;
}

fn main() {
    let formes: Vec<dyn Forme> = Vec::new();
}
```

```text
error[E0277]: the size for values of type `dyn Forme` cannot be known at compilation time
 --> src/main.rs:6:17
  |
6 |     let formes: Vec<dyn Forme> = Vec::new();
  |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `dyn Forme`
```

Les formes n'ont pas toutes la même taille. Mettez chacune dans une boîte :
`Vec<Box<dyn Forme>>`.

### Un type qui n'a pas le trait

```rust norun
trait Forme {
    fn aire(&self) -> f64;
}

struct Carre(f64);
struct Cercle(f64);

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let formes: Vec<Box<dyn Forme>> = vec![Box::new(Carre(2.0)), Box::new(Cercle(1.0))];
    println!("{}", formes.len());
}
```

```text
error[E0277]: the trait bound `Cercle: Forme` is not satisfied
  --> src/main.rs:15:66
   |
15 |     let formes: Vec<Box<dyn Forme>> = vec![Box::new(Carre(2.0)), Box::new(Cercle(1.0))];
   |                                                                  ^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `Forme` is not implemented for `Cercle`
```

Seuls les types qui ont signé le contrat peuvent entrer. Ajoutez
`impl Forme for Cercle { … }`.

### Renvoyer deux closures avec impl Fn

```rust norun
fn reglage(n: i32, doubler: bool) -> impl Fn(i32) -> i32 {
    if doubler {
        move |x| x * n
    } else {
        move |x| x + n
    }
}

fn main() {
    println!("{}", reglage(3, true)(5));
}
```

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:5:9
  |
2 | /     if doubler {
3 | |         move |x| x * n
  | |         --------------
  | |         |
  | |         the expected closure
  | |         expected because of this
4 | |     } else {
5 | |         move |x| x + n
  | |         ^^^^^^^^^^^^^^ expected closure, found a different closure
6 | |     }
  | |_____- `if` and `else` have incompatible types
  |
  = note: expected closure `{closure@src/main.rs:3:9: 3:17}`
             found closure `{closure@src/main.rs:5:9: 5:17}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

`impl Fn` désigne **un seul** type caché. Deux closures ont deux types différents.
Renvoyez `Box<dyn Fn(i32) -> i32>` et mettez chaque closure dans `Box::new(…)`, comme
dans [Renvoyer une closure avec Box<dyn Fn>](#renvoyer-une-closure-avec-box-dyn-fn).

## Testez-vous

**1.** Quelle est la différence entre `fn f(x: &impl Forme)` et `fn f(x: &dyn Forme)` ?

> **Réponse**
>
> Avec `impl Forme`, Rust fabrique une copie de `f` pour chaque type, à la compilation
> (statique). Avec `dyn Forme`, il n'y a qu'une fonction ; elle trouve la bonne
> méthode pendant l'exécution, grâce à la table des méthodes (dynamique).

**2.** Pourquoi écrit-on `Vec<Box<dyn Forme>>` et pas `Vec<dyn Forme>` ?

> **Réponse**
>
> Un carré et un cercle n'ont pas la même taille. Un `Vec` exige des éléments de taille
> connue. Une `Box` a toujours la même taille, quelle que soit la forme qu'elle
> contient.

**3.** Écrivez un trait `Saluer` avec `fn bonjour(&self) -> String`, deux structures
`Fr` et `En`, et une boucle sur un `Vec<Box<dyn Saluer>>` qui affiche `bonjour` puis
`hello`.

> **Réponse**
>
> ```rust
> trait Saluer {
>     fn bonjour(&self) -> String;
> }
>
> struct Fr;
> struct En;
>
> impl Saluer for Fr {
>     fn bonjour(&self) -> String {
>         String::from("bonjour")
>     }
> }
>
> impl Saluer for En {
>     fn bonjour(&self) -> String {
>         String::from("hello")
>     }
> }
>
> fn main() {
>     let liste: Vec<Box<dyn Saluer>> = vec![Box::new(Fr), Box::new(En)];
>     for s in &liste {
>         println!("{}", s.bonjour());
>     }
> }
> ```
>
> ```sortie
> bonjour
> hello
> ```

## En résumé

- Un générique (`impl Trait`, `<T: Trait>`) est statique : une copie par type,
  choisie à la compilation.
- `dyn Trait` est dynamique : une seule fonction, qui lit la table des méthodes
  (vtable) pendant l'exécution.
- `&dyn Trait` emprunte, `Box<dyn Trait>` possède ; `Vec<Box<dyn Trait>>` mélange
  des types.
- Commencez par les génériques ; passez à `dyn` pour mélanger des types.
- Un trait avec une méthode générique ou un `-> Self` sans `self` n'est pas
  dyn compatible (sauf avec `where Self: Sized`).
- `Box<dyn Fn(…) -> …>` renvoie ou range des closures différentes.

Suite : [Les traits de la bibliothèque standard](traits-standard.md)
