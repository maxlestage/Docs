# Modules et crates

Les modules rangent votre code en tiroirs bien étiquetés, et les crates sont des paquets de code que vous pouvez partager ou télécharger.

> **Objectifs :**
>
> - créer un module avec `mod` et rendre son contenu public avec `pub` ;
> - utiliser `use` pour écrire des noms plus courts ;
> - comprendre les chemins `crate::` et `super::` ;
> - répartir un programme en plusieurs fichiers ;
> - distinguer une bibliothèque d'un programme, et ajouter une crate de crates.io.

## Pourquoi des modules ?

Quand un programme grandit, tout mettre dans `main.rs` devient vite le désordre. Un
**module** est comme un tiroir : on y range les fonctions et les types qui vont
ensemble (la géométrie d'un côté, l'affichage de l'autre…).

Un module a deux rôles :

- **ranger** : chaque chose a sa place, avec un nom clair ;
- **cacher** : ce qui est dans le tiroir reste privé, sauf ce que vous choisissez de
  montrer.

## Créer un module avec mod

Le mot `mod`, suivi d'un nom et d'accolades, crée un module. Pour appeler une fonction
du module, on écrit `module::fonction`, avec deux fois deux-points :

```rust
mod salutations {
    pub fn bonjour() {
        println!("Bonjour !");
    }
}

fn main() {
    salutations::bonjour();
}
```

```sortie
Bonjour !
```

Un module peut contenir d'autres modules, comme des tiroirs dans un tiroir :

```rust
mod geometrie {
    pub mod carre {
        pub fn aire(cote: f64) -> f64 {
            cote * cote
        }
    }

    pub mod cercle {
        pub fn aire(rayon: f64) -> f64 {
            3.14 * rayon * rayon
        }
    }
}

fn main() {
    println!("{}", geometrie::carre::aire(3.0));
    println!("{}", geometrie::cercle::aire(2.0));
}
```

```sortie
9
12.56
```

Les deux fonctions s'appellent `aire`, sans conflit : chacune est dans son tiroir.

## Public ou privé : pub

Par défaut, tout ce qui est dans un module est **privé** : seul le module lui-même (et
ses sous-modules) peut s'en servir. Le mot `pub` rend un élément **public**.

```rust
mod banque {
    pub fn afficher_solde() {
        println!("Solde : {} €", solde_secret());
    }

    // pas de `pub` : utilisable seulement dans le module
    fn solde_secret() -> u32 {
        120
    }
}

fn main() {
    banque::afficher_solde();
    // banque::solde_secret(); serait refusé : la fonction est privée
}
```

```sortie
Solde : 120 €
```

### Les structures et leurs champs

Pour une structure, `pub struct` rend le **type** public, mais chaque **champ** reste
privé tant qu'il n'a pas son propre `pub`. On ajoute alors souvent une fonction `new`
publique pour créer la valeur :

```rust
mod jeu {
    pub struct Joueur {
        pub nom: String, // public : lisible partout
        score: u32,      // privé : modifiable seulement par le module
    }

    impl Joueur {
        pub fn new(nom: &str) -> Joueur {
            Joueur { nom: nom.to_string(), score: 0 }
        }

        pub fn marquer(&mut self) {
            self.score += 10;
        }

        pub fn score(&self) -> u32 {
            self.score
        }
    }
}

fn main() {
    let mut j = jeu::Joueur::new("Léa");
    j.marquer();
    j.marquer();
    println!("{} a {} points", j.nom, j.score());
}
```

```sortie
Léa a 20 points
```

Personne, hors du module, ne peut tricher en écrivant `j.score = 1000` : il faut passer
par `marquer`.

> **Note :** pour une énumération, `pub enum` rend toutes ses variantes publiques d'un
> coup.

## Des noms plus courts avec use

Écrire `geometrie::cercle::aire` à chaque fois est long. `use` crée un **raccourci** :

```rust
mod geometrie {
    pub mod cercle {
        pub fn aire(rayon: f64) -> f64 {
            3.14 * rayon * rayon
        }

        pub fn perimetre(rayon: f64) -> f64 {
            2.0 * 3.14 * rayon
        }
    }
}

use geometrie::cercle;
use geometrie::cercle::perimetre;

fn main() {
    println!("{}", cercle::aire(1.0)); // grâce au premier use
    println!("{}", perimetre(1.0));    // grâce au second use
}
```

```sortie
3.14
6.28
```

`use` sert aussi pour la bibliothèque standard. Vous l'avez déjà vu avec
`use std::collections::HashMap;`. On peut importer plusieurs noms d'un coup avec des
accolades :

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let mut vus = HashSet::new();
    vus.insert("rouge");
    let mut stock = HashMap::new();
    stock.insert("pommes", 3);
    println!("{} {}", vus.len(), stock["pommes"]);
}
```

```sortie
1 3
```

> **Astuce :** l'habitude en Rust : importez le **module parent** d'une fonction
> (`use geometrie::cercle;` puis `cercle::aire`), mais le **type** lui-même
> (`use std::collections::HashMap;`).

## Les chemins : crate:: et super::

Un chemin indique où trouver un nom, comme une adresse.

- `crate::` part de la **racine** du projet (le haut de `main.rs` ou `lib.rs`) ;
- `super::` remonte au module **parent** (le tiroir du dessus) ;
- `self::` désigne le module où l'on est.

```rust
fn message() -> &'static str {
    "racine"
}

mod cuisine {
    pub fn recette() -> String {
        format!("{} et {}", super::message(), outils::couteau())
    }

    pub mod outils {
        pub fn couteau() -> &'static str {
            "couteau"
        }

        pub fn tablier() -> String {
            // chemin complet depuis la racine
            format!("tablier près du {}", crate::cuisine::outils::couteau())
        }
    }
}

fn main() {
    println!("{}", cuisine::recette());
    println!("{}", cuisine::outils::tablier());
}
```

```sortie
racine et couteau
tablier près du couteau
```

## Un module par fichier

Dans un vrai projet, chaque module va dans son propre fichier. Prenons ce projet :

```text
mon_projet/
├── Cargo.toml
└── src/
    ├── main.rs
    └── geometrie.rs
```

Le fichier `src/geometrie.rs` contient le **contenu** du module, sans `mod` ni
accolades autour :

```rust norun
// src/geometrie.rs
pub fn aire_carre(cote: f64) -> f64 {
    cote * cote
}
```

Dans `src/main.rs`, la ligne `mod geometrie;` (avec un point-virgule) dit à Rust :
« va chercher le module dans `geometrie.rs` » :

```rust norun
// src/main.rs
mod geometrie;

fn main() {
    println!("{}", geometrie::aire_carre(3.0));
}
```

> **À retenir :** on écrit `mod geometrie;` **une seule fois**, dans le fichier parent.
> Partout ailleurs, on utilise `use` ou le chemin `crate::geometrie::…`.

### Un module avec des sous-modules : un dossier

Si `geometrie` a lui-même des sous-modules, on crée un dossier du même nom :

```text
src/
├── main.rs
├── geometrie.rs          ← déclare : pub mod carre; pub mod cercle;
└── geometrie/
    ├── carre.rs
    └── cercle.rs
```

```rust norun
// src/geometrie.rs
pub mod carre;
pub mod cercle;
```

```rust norun
// src/geometrie/cercle.rs
pub fn aire(rayon: f64) -> f64 {
    3.14 * rayon * rayon
}
```

```rust norun
// src/main.rs
mod geometrie;

use geometrie::cercle;

fn main() {
    println!("{}", cercle::aire(2.0));
}
```

> **Note :** dans d'anciens projets, vous verrez `geometrie/mod.rs` au lieu de
> `geometrie.rs`. Les deux marchent ; la forme `geometrie.rs` est la plus récente.

## Bibliothèque ou programme : lib et bin

Une **crate** est un paquet de code Rust. Il y en a deux sortes :

| Sorte | Fichier de départ | Rôle |
|-------|-------------------|------|
| programme (*bin*) | `src/main.rs` | a une fonction `main`, se lance avec `cargo run` |
| bibliothèque (*lib*) | `src/lib.rs` | pas de `main` ; d'autres programmes l'utilisent |

```sh
cargo new jeu          # crée un programme : src/main.rs
cargo new --lib outils # crée une bibliothèque : src/lib.rs
```

Un même projet peut avoir les deux. Le code utile va dans `lib.rs`, et `main.rs`
l'utilise par le nom du projet :

```text
jeu/
├── Cargo.toml        ← name = "jeu"
└── src/
    ├── lib.rs        ← pub fn regles() …
    └── main.rs
```

```rust norun
// src/main.rs
fn main() {
    jeu::regles(); // `jeu` est le nom de la bibliothèque du projet
}
```

> **Astuce :** cette séparation rend le code plus facile à [tester](tests.md) :
> la bibliothèque se teste seule, et `main.rs` reste très court.

## Ajouter une crate de crates.io

Des milliers de bibliothèques gratuites sont publiées sur [crates.io](https://crates.io).
Pour en ajouter une, tapez dans le dossier du projet :

```sh
cargo add rand
```

Cargo ajoute une ligne dans `Cargo.toml` :

```toml
[dependencies]
rand = "0.9"
```

Au prochain `cargo build` ou `cargo run`, Cargo télécharge la crate et la compile. Vous
l'utilisez ensuite avec `use`, comme un module :

```rust norun
use rand::Rng;

fn main() {
    let de = rand::rng().random_range(1..=6);
    println!("Le dé affiche {de}");
}
```

> **Attention :** chaque crate a sa propre documentation, sur
> [docs.rs](https://docs.rs). Lisez-la : les noms des fonctions changent parfois d'une
> version à l'autre.

## Erreurs fréquentes

### Oublier pub

```rust norun
mod outils {
    fn double(n: i32) -> i32 {
        n * 2
    }
}

fn main() {
    println!("{}", outils::double(4));
}
```

```text
error[E0603]: function `double` is private
 --> main.rs:8:28
  |
8 |     println!("{}", outils::double(4));
  |                            ^^^^^^ private function
```

Ajoutez `pub` devant `fn double`. Pour un champ de structure sans `pub`, le message
est semblable :

```text
error[E0451]: field `score` of struct `Joueur` is private
```

### Appeler la fonction sans son module

```rust norun
mod outils {
    pub fn double(n: i32) -> i32 {
        n * 2
    }
}

fn main() {
    println!("{}", double(4));
}
```

```text
error[E0425]: cannot find function `double` in this scope
help: consider importing this function
  |
1 + use crate::outils::double;
```

Écrivez le chemin (`outils::double(4)`) ou ajoutez `use outils::double;` en haut.

### Oublier le fichier du module

```rust norun
// src/main.rs
mod geometrie;
```

```text
error[E0583]: file not found for module `geometrie`
  = help: to create the module `geometrie`, create file "src/geometrie.rs" or "src/geometrie/mod.rs"
```

Le fichier doit s'appeler exactement comme le module, et se trouver à côté de
`main.rs`.

## Testez-vous

1. Un module contient `fn calcul()` sans `pub`. Peut-on l'appeler depuis `main` ?

> **Réponse**
>
> Non : sans `pub`, la fonction est privée. Seul son module (et ses sous-modules)
> peut l'appeler.

2. Que fait la ligne `mod reseau;` dans `main.rs` ?

> **Réponse**
>
> Elle déclare le module `reseau` et dit à Rust de lire son contenu dans le fichier
> `src/reseau.rs` (ou `src/reseau/mod.rs`).

3. Dans un sous-module, comment appeler une fonction `aide()` définie dans le module
   parent ?

> **Réponse**
>
> Avec `super::aide()`. On peut aussi écrire le chemin complet depuis la racine :
> `crate::…::aide()`.

## En résumé

- `mod nom { … }` crée un module ; `mod nom;` le charge depuis `nom.rs`.
- Tout est privé par défaut ; `pub` rend une fonction, un type ou un champ public.
- `use` crée un raccourci vers un nom.
- `crate::` part de la racine, `super::` remonte d'un niveau.
- `src/main.rs` est un programme, `src/lib.rs` une bibliothèque.
- `cargo add nom` ajoute une crate de [crates.io](https://crates.io).

Comparez avec [les modules en Ruby](../ruby/modules.md) et
[en Rythme](../rythme/modules.md). Suite :
[Lire et écrire : clavier et fichiers](fichiers.md).

Suite : [Lire et écrire : clavier et fichiers](fichiers.md)
