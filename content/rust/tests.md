# Tester son code

Un test est un petit programme qui vérifie que votre code fait bien ce qu'il doit faire : Rust et Cargo les écrivent et les lancent très simplement.

> **Objectifs :**
>
> - vérifier une valeur avec `assert!`, `assert_eq!` et `assert_ne!` ;
> - écrire un test avec `#[test]` et le lancer avec `cargo test` ;
> - ranger les tests dans un module `#[cfg(test)]` ;
> - tester qu'une fonction s'arrête bien avec `#[should_panic]` ;
> - écrire des tests dans le dossier `tests/` et dans la documentation.

## Pourquoi tester ?

Quand vous modifiez un programme, vous risquez de casser ce qui marchait. Vérifier tout
à la main, à chaque fois, est long et on oublie des cas.

Un test fait cette vérification **automatiquement**, en une commande. C'est comme une
liste de contrôle avant le décollage d'un avion : rapide, et rassurant.

## Les macros de vérification

Trois macros vérifient une condition. Si elle est fausse, elles provoquent un `panic!`
avec un message clair.

| Macro | Vérifie que… |
|-------|--------------|
| `assert!(condition)` | la condition est vraie |
| `assert_eq!(a, b)` | `a` est égal à `b` |
| `assert_ne!(a, b)` | `a` est différent de `b` |

```rust
let total = 2 + 2;
assert!(total > 3);
assert_eq!(total, 4);
assert_ne!(total, 5);
println!("Toutes les vérifications passent");
```

```sortie
Toutes les vérifications passent
```

Vous pouvez ajouter un message, écrit comme dans `println!`. Il s'affiche seulement si
la vérification échoue :

```rust
let age = 15;
assert!(age >= 0, "l'âge ne peut pas être négatif : {age}");
println!("ok");
```

```sortie
ok
```

Quand une vérification échoue, le programme s'arrête et montre les deux valeurs :

```rust norun
assert_eq!(2 + 2, 5);
```

```text
assertion `left == right` failed
  left: 4
 right: 5
```

> **Note :** `left` est la première valeur, `right` la seconde. Par habitude, on met
> le résultat calculé à gauche et la valeur attendue à droite.

## Écrire un test avec #[test]

Un test est une fonction sans paramètre, précédée de `#[test]`. Le test **réussit** s'il
va jusqu'au bout, et **échoue** s'il provoque un `panic!` (par exemple avec une macro
`assert`).

On range les tests dans un module `tests`, à la fin du fichier :

```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajoute_deux_nombres() {
        assert_eq!(ajouter(2, 3), 5);
    }

    #[test]
    fn ajoute_un_negatif() {
        assert_eq!(ajouter(2, -5), -3);
    }
}

fn main() {
    println!("{}", ajouter(2, 3));
}
```

```sortie
5
```

Ligne par ligne :

- `#[cfg(test)]` : ce module n'est compilé que pour les tests. Il n'alourdit pas votre
  programme final ;
- `mod tests` : un [module](modules.md) comme les autres ;
- `use super::*;` : importe tout ce qui est dans le module parent (ici, `ajouter`) ;
- `#[test]` : marque chaque fonction de test.

> **Astuce :** donnez à vos tests des noms qui décrivent ce qu'ils vérifient :
> `ajoute_un_negatif` est plus parlant que `test2`.

## Lancer les tests : cargo test

Dans le dossier du projet, tapez :

```sh
cargo test
```

Cargo compile le projet en mode test, lance chaque fonction `#[test]` et affiche un
résumé :

```text
running 2 tests
test tests::ajoute_deux_nombres ... ok
test tests::ajoute_un_negatif ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Si un test échoue, Cargo indique lequel, avec les deux valeurs comparées. Voici ce qui
s'affiche si l'on écrit par erreur `assert_eq!(ajouter(2, -5), 3);` :

```text
running 2 tests
test tests::ajoute_deux_nombres ... ok
test tests::ajoute_un_negatif ... FAILED

failures:

---- tests::ajoute_un_negatif stdout ----

thread 'tests::ajoute_un_negatif' panicked at src/main.rs:16:9:
assertion `left == right` failed
  left: -3
 right: 3

failures:
    tests::ajoute_un_negatif

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Pour lancer seulement certains tests, donnez un morceau de leur nom :

```sh
cargo test negatif   # lance seulement les tests dont le nom contient « negatif »
```

> **Note :** les tests tournent en parallèle, donc leur ordre d'affichage peut changer.
> Et ce que vos tests affichent avec `println!` est caché, sauf en cas d'échec.

## Tester un arrêt : #[should_panic]

Parfois, le bon comportement est de **s'arrêter**. `#[should_panic]` indique que le test
doit provoquer un `panic!` pour réussir. Avec `expected`, on vérifie aussi le message :

```rust
fn diviser(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division par zéro");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divise_normalement() {
        assert_eq!(diviser(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "division par zéro")]
    fn refuse_zero() {
        diviser(1, 0);
    }
}

fn main() {
    println!("{}", diviser(10, 2));
}
```

```sortie
5
```

Dans le résumé, ce test apparaît ainsi :

```text
test tests::refuse_zero - should panic ... ok
```

## Que tester ?

Pour chaque fonction, pensez à trois sortes de cas :

- un cas **normal** : `ajouter(2, 3)` ;
- un cas **limite** : une liste vide, zéro, un texte vide ;
- un cas **d'erreur** : une entrée invalide doit donner un `Err` (ou un `panic!`).

```rust
fn moyenne(notes: &[f64]) -> Option<f64> {
    if notes.is_empty() {
        return None;
    }
    Some(notes.iter().sum::<f64>() / notes.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moyenne_normale() {
        assert_eq!(moyenne(&[10.0, 20.0]), Some(15.0));
    }

    #[test]
    fn moyenne_d_une_liste_vide() {
        assert_eq!(moyenne(&[]), None);
    }

    #[test]
    fn analyse_d_un_texte_invalide() {
        assert!("douze".parse::<i32>().is_err());
    }
}

fn main() {
    println!("{:?} {:?}", moyenne(&[10.0, 20.0]), moyenne(&[]));
}
```

```sortie
Some(15.0) None
```

> **Attention :** pour comparer deux nombres à virgule calculés, l'égalité exacte peut
> échouer (`0.1 + 0.2` ne vaut pas exactement `0.3`). Vérifiez plutôt que l'écart est
> très petit : `assert!((x - 0.3).abs() < 1e-9);`.

## Les tests d'intégration : le dossier tests/

Les tests du module `tests` vérifient chaque fonction de l'intérieur. Les **tests
d'intégration** utilisent votre code de l'extérieur, comme le ferait un autre
programme. Ils demandent une bibliothèque (`src/lib.rs`, voir
[Modules et crates](modules.md)) :

```text
calcul/
├── Cargo.toml
├── src/
│   └── lib.rs           ← pub fn ajouter(…)
└── tests/
    └── calculs.rs       ← les tests d'intégration
```

Chaque fichier de `tests/` est compilé à part. On importe la bibliothèque par le nom du
projet, et on n'a pas besoin de `#[cfg(test)]` :

```rust norun
// tests/calculs.rs
use calcul::ajouter;

#[test]
fn ajoute_depuis_l_exterieur() {
    assert_eq!(ajouter(10, 32), 42);
}
```

`cargo test` lance alors les deux sortes de tests, l'une après l'autre :

```text
     Running unittests src/lib.rs
running 3 tests
...
     Running tests/calculs.rs
running 1 test
test ajoute_depuis_l_exterieur ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Les tests dans la documentation

Les commentaires `///` documentent une fonction. Un exemple de code écrit dedans, entre
trois accents graves, est **aussi un test** : `cargo test` le compile et le lance. Votre
documentation reste donc toujours juste.

````rust norun
/// Additionne deux nombres.
///
/// ```
/// let total = calcul::ajouter(2, 3);
/// assert_eq!(total, 5);
/// ```
pub fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}
````

Dans le résumé de `cargo test`, ces tests apparaissent à la fin :

```text
   Doc-tests calcul

running 1 test
test src/lib.rs - ajouter (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

> **Note :** les tests de documentation ne marchent que dans une bibliothèque
> (`src/lib.rs`).

## Erreurs fréquentes

### Oublier use super::*

```rust norun
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn ajoute() {
        assert_eq!(ajouter(2, 3), 5);
    }
}
```

```text
error[E0425]: cannot find function `ajouter` in this scope
help: consider importing this function
  |
7 +     use crate::ajouter;
```

Le module `tests` est un tiroir à part : il ne voit pas les fonctions du parent. Ajoutez
`use super::*;` au début du module.

### Comparer des structures sans PartialEq ni Debug

```rust norun
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    assert_eq!(a, Point { x: 1, y: 2 });
}
```

```text
error[E0369]: binary operation `==` cannot be applied to type `Point`
help: consider annotating `Point` with `#[derive(PartialEq)]`

error[E0277]: `Point` doesn't implement `Debug`
help: consider annotating `Point` with `#[derive(Debug)]`
```

`assert_eq!` a besoin de comparer (`PartialEq`) et d'afficher (`Debug`) les valeurs.
Ajoutez les deux d'un coup :

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    assert_eq!(a, Point { x: 1, y: 2 });
    println!("égaux");
}
```

```sortie
égaux
```

### Un test should_panic qui ne s'arrête pas

Si la fonction ne provoque pas de `panic!`, le test `#[should_panic]` échoue :

```text
test tests::refuse_zero - should panic ... FAILED

---- tests::refuse_zero stdout ----
note: test did not panic as expected
```

Vérifiez que vous appelez bien la fonction avec une valeur qui doit l'arrêter (ici,
`diviser(1, 0)` et non `diviser(4, 2)`).

## Testez-vous

1. Quelle est la différence entre `assert!` et `assert_eq!` ?

> **Réponse**
>
> `assert!` vérifie qu'une condition est vraie. `assert_eq!` vérifie que deux valeurs
> sont égales, et affiche les deux (`left` et `right`) en cas d'échec.

2. À quoi sert `#[cfg(test)]` devant `mod tests` ?

> **Réponse**
>
> Le module n'est compilé que pour `cargo test`. Il n'est pas inclus dans le
> programme final.

3. Un test doit vérifier qu'une fonction s'arrête avec `panic!`. Que faut-il ajouter ?

> **Réponse**
>
> L'attribut `#[should_panic]` sous `#[test]` (avec `expected = "…"` pour vérifier le
> message).

## En résumé

- `assert!`, `assert_eq!` et `assert_ne!` vérifient une condition, sinon `panic!`.
- Un test est une fonction `#[test]`, rangée dans un module `#[cfg(test)] mod tests`
  avec `use super::*;`.
- `cargo test` lance tous les tests et affiche un résumé.
- `#[should_panic]` teste qu'une fonction s'arrête bien.
- Le dossier `tests/` contient les tests d'intégration ; les exemples des commentaires
  `///` sont aussi des tests.

Comparez avec [les tests en Ruby](../ruby/tests.md) et
[en Rythme](../rythme/tests.md). Suite :
[Organiser un projet avec Cargo](projet.md).

Suite : [Déboguer en Rust](debogage.md)
