# Box, Rc et RefCell

`Box`, `Rc` et `RefCell` sont des « boîtes » qui assouplissent les règles de la possession : ranger une valeur sur le tas, la partager entre plusieurs propriétaires, ou la modifier à travers un emprunt partagé.

> **Objectifs :**
>
> - comprendre la différence entre la pile et le tas ;
> - utiliser `Box` pour un type récursif, comme une liste chaînée ;
> - partager une valeur entre plusieurs propriétaires avec `Rc` ;
> - modifier une valeur partagée avec `RefCell` ;
> - choisir le bon outil grâce à un petit tableau.

## La pile et le tas

Un programme range ses données à deux endroits de la mémoire.

- **La pile** (*stack*) est comme une pile d'assiettes : on pose en haut, on reprend en
  haut. C'est très rapide, mais chaque assiette doit avoir une **taille connue** à
  l'avance. Les `i32`, `f64`, `bool`… vont sur la pile.
- **Le tas** (*heap*) est comme un grand entrepôt : on y range des objets de n'importe
  quelle taille, et on garde sur la pile un petit **ticket** (une adresse) pour les
  retrouver. Une `String` ou un `Vec` gardent leur contenu dans le tas.

```text
     PILE                      TAS
 ┌──────────────┐
 │ age = 12     │
 ├──────────────┤          ┌───────────────┐
 │ nom = ticket ├────────► │ "Léa"         │
 └──────────────┘          └───────────────┘
```

Ce ticket s'appelle un **pointeur** : une valeur qui indique où se trouve une autre
valeur. Les « pointeurs intelligents » de cette page sont des tickets qui, en plus,
libèrent la mémoire tout seuls au bon moment.

## Box : ranger une valeur sur le tas

`Box::new(valeur)` met la valeur dans le tas et vous donne la boîte. Vous l'utilisez
ensuite presque comme la valeur elle-même :

```rust
let boite = Box::new(41);
let resultat = *boite + 1; // `*` ouvre la boîte
println!("{boite} et {resultat}");
```

```sortie
41 et 42
```

Quand la boîte disparaît (fin de son bloc), la valeur dans le tas est libérée, comme
pour une `String`.

### À quoi sert Box ? Les types récursifs

Un type **récursif** se contient lui-même. Une liste chaînée en est l'exemple classique :
une liste est soit vide, soit « un élément suivi d'une liste ».

Sans boîte, Rust ne peut pas calculer la taille d'une telle liste (elle contiendrait une
liste, qui contient une liste…). Avec `Box`, chaque élément contient seulement un
ticket de taille fixe vers la suite :

```rust
enum Liste {
    Element(i32, Box<Liste>),
    Fin,
}

use Liste::{Element, Fin};

fn somme(liste: &Liste) -> i32 {
    match liste {
        Element(valeur, suite) => valeur + somme(suite),
        Fin => 0,
    }
}

fn main() {
    // la liste 1 → 2 → 3 → Fin
    let liste = Element(1, Box::new(Element(2, Box::new(Element(3, Box::new(Fin))))));
    println!("Somme : {}", somme(&liste));
}
```

```sortie
Somme : 6
```

```text
 Element(1, ●)──► Element(2, ●)──► Element(3, ●)──► Fin
```

> **Note :** Box sert aussi à ranger des valeurs de types différents dans une même
> liste, avec `Vec<Box<dyn Trait>>` : voir [Génériques et traits](generiques.md).

## Rc : plusieurs propriétaires

Normalement, une valeur n'a [qu'un seul propriétaire](possession.md). Mais parfois,
plusieurs parties du programme doivent garder la même valeur, sans savoir laquelle
finira en dernier.

`Rc` (*reference counted* : « avec un compteur de références ») résout ce problème. Pensez
à la télévision d'un salon : la dernière personne qui quitte la pièce l'éteint.

- `Rc::new(valeur)` crée la valeur partagée ;
- `Rc::clone(&r)` crée un **propriétaire de plus** (sans copier la valeur) ;
- quand le dernier propriétaire disparaît, la valeur est libérée.

```rust
use std::rc::Rc;

fn main() {
    let playlist = Rc::new(vec!["Chanson A", "Chanson B"]);
    println!("propriétaires : {}", Rc::strong_count(&playlist));

    let lea = Rc::clone(&playlist);
    let tom = Rc::clone(&playlist);
    println!("propriétaires : {}", Rc::strong_count(&playlist));
    println!("Léa écoute {:?}", lea);

    drop(tom); // Tom s'en va
    println!("propriétaires : {}", Rc::strong_count(&playlist));
    println!("Première chanson : {}", playlist[0]);
}
```

```sortie
propriétaires : 1
propriétaires : 3
Léa écoute ["Chanson A", "Chanson B"]
propriétaires : 2
Première chanson : Chanson A
```

`drop(x)` fait disparaître `x` tout de suite, sans attendre la fin du bloc.

> **À retenir :** `Rc::clone` ne copie pas la playlist : il ajoute 1 au compteur. C'est
> très rapide.

> **Attention :** avec `Rc`, la valeur est **en lecture seule**. Pour la modifier, il
> faut la combiner avec `RefCell` (voir plus bas).

## RefCell : modifier à travers un emprunt partagé

Les règles d'[emprunt](possession.md) disent : soit plusieurs lecteurs, soit un seul
écrivain. D'habitude, le compilateur vérifie cette règle **avant** l'exécution.

`RefCell` déplace cette vérification **pendant** l'exécution. Elle permet de modifier
une valeur même quand on n'a qu'une référence partagée : on parle de **mutabilité
intérieure**.

- `.borrow()` emprunte en lecture ;
- `.borrow_mut()` emprunte en écriture.

```rust
use std::cell::RefCell;

fn main() {
    let panier = RefCell::new(vec!["pain"]); // pas besoin de `mut`

    panier.borrow_mut().push("lait");
    panier.borrow_mut().push("œufs");

    println!("{:?}", panier.borrow());
    println!("{} articles", panier.borrow().len());
}
```

```sortie
["pain", "lait", "œufs"]
3 articles
```

> **Attention :** la règle existe toujours ! Si vous empruntez en écriture pendant
> qu'un autre emprunt est encore en cours, le programme s'arrête avec un `panic!`
> (voir les erreurs fréquentes).

## Rc<RefCell<T>> : un compteur partagé et modifiable

En combinant les deux boîtes, on obtient une valeur **partagée** (grâce à `Rc`) et
**modifiable** (grâce à `RefCell`). Ici, deux boutons partagent le même compteur de
clics :

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Bouton {
    nom: String,
    clics: Rc<RefCell<u32>>,
}

impl Bouton {
    fn cliquer(&self) {
        *self.clics.borrow_mut() += 1;
        println!("clic sur {}", self.nom);
    }
}

fn main() {
    let compteur = Rc::new(RefCell::new(0));

    let ok = Bouton { nom: String::from("OK"), clics: Rc::clone(&compteur) };
    let annuler = Bouton { nom: String::from("Annuler"), clics: Rc::clone(&compteur) };

    ok.cliquer();
    annuler.cliquer();
    ok.cliquer();

    println!("Total des clics : {}", compteur.borrow());
}
```

```sortie
clic sur OK
clic sur Annuler
clic sur OK
Total des clics : 3
```

Les deux boutons et la variable `compteur` pointent tous vers **le même** nombre.

```text
 ok.clics ──────┐
 annuler.clics ─┼──► Rc ──► RefCell ──► 3
 compteur ──────┘
```

> **Note :** `Rc` et `RefCell` ne marchent que dans un seul fil d'exécution. Pour
> partager entre plusieurs [threads](concurrence.md), on utilise leurs cousins
> `Arc` et `Mutex`.

## Lequel choisir ?

| Besoin | Outil | Vérification |
|--------|-------|--------------|
| un seul propriétaire, valeur sur le tas (type récursif…) | `Box<T>` | à la compilation |
| plusieurs propriétaires, lecture seule | `Rc<T>` | à la compilation |
| modifier à travers une référence partagée | `RefCell<T>` | à l'exécution |
| plusieurs propriétaires **et** modification | `Rc<RefCell<T>>` | à l'exécution |
| la même chose entre plusieurs threads | `Arc<Mutex<T>>` | à l'exécution |

> **Astuce :** commencez toujours sans boîte, avec de simples variables et des
> références `&`. N'ajoutez `Box`, `Rc` ou `RefCell` que quand le compilateur vous
> montre que vous en avez vraiment besoin.

## Erreurs fréquentes

### Un type récursif sans Box

```rust norun
enum Liste {
    Element(i32, Liste),
    Fin,
}
```

```text
error[E0072]: recursive type `Liste` has infinite size
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Element(i32, Box<Liste>),
  |                  ++++     +
```

Le compilateur vous donne la solution : mettez la suite dans une `Box`.

### Modifier une valeur dans un Rc

```rust norun
use std::rc::Rc;

fn main() {
    let notes = Rc::new(vec![12, 15]);
    notes.push(18);
}
```

```text
error[E0596]: cannot borrow data in an `Rc` as mutable
 --> main.rs:5:5
  |
5 |     notes.push(18);
  |     ^^^^^ cannot borrow as mutable
```

`Rc` ne donne qu'un accès en lecture. Ajoutez une `RefCell` :

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let notes = Rc::new(RefCell::new(vec![12, 15]));
    notes.borrow_mut().push(18);
    println!("{:?}", notes.borrow());
}
```

```sortie
[12, 15, 18]
```

### Deux emprunts en même temps dans une RefCell

```rust norun
use std::cell::RefCell;

fn main() {
    let panier = RefCell::new(vec![1]);
    let lecture = panier.borrow();
    panier.borrow_mut().push(2); // lecture est encore en cours !
    println!("{:?}", lecture);
}
```

Le programme compile, mais s'arrête à l'exécution :

```text
thread 'main' panicked at main.rs:6:12:
RefCell already borrowed
```

Terminez la lecture avant d'écrire, par exemple en la plaçant dans son propre bloc :

```rust
use std::cell::RefCell;

fn main() {
    let panier = RefCell::new(vec![1]);
    {
        let lecture = panier.borrow();
        println!("{:?}", lecture);
    } // la lecture se termine ici
    panier.borrow_mut().push(2);
    println!("{:?}", panier.borrow());
}
```

```sortie
[1]
[1, 2]
```

## Testez-vous

1. Pourquoi `enum Arbre { Noeud(i32, Arbre, Arbre), Feuille }` ne compile-t-il pas ?

> **Réponse**
>
> Le type se contient lui-même : sa taille serait infinie. Il faut une boîte :
> `Noeud(i32, Box<Arbre>, Box<Arbre>)`.

2. Après `let a = Rc::new(5); let b = Rc::clone(&a);`, que vaut
   `Rc::strong_count(&a)` ?

> **Réponse**
>
> `2` : `a` et `b` sont deux propriétaires de la même valeur. Le `5` n'a pas été copié.

3. Quelle combinaison utiliser pour une liste de courses partagée par plusieurs
   structures, et que chacune peut compléter ?

> **Réponse**
>
> `Rc<RefCell<Vec<…>>>` : `Rc` pour partager, `RefCell` pour modifier avec
> `borrow_mut()`.

## En résumé

- La pile est rapide mais demande une taille connue ; le tas range tout, derrière un
  pointeur.
- `Box<T>` met une valeur sur le tas : indispensable pour les types récursifs.
- `Rc<T>` permet plusieurs propriétaires ; `Rc::clone` ajoute un propriétaire.
- `RefCell<T>` permet de modifier via `borrow_mut()`, avec une vérification à
  l'exécution.
- `Rc<RefCell<T>>` : une valeur partagée et modifiable, dans un seul thread.

Suite : [Faire plusieurs choses à la fois](concurrence.md).
