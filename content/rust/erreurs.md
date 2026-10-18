# Gérer les erreurs

Cette page explique les deux sortes d'erreurs de Rust : les erreurs fatales avec `panic!`, et les erreurs prévues avec `Result` et l'opérateur `?`.

## Deux sortes d'erreurs

- Une erreur **irrécupérable** : un bug, une situation impossible. Le programme
  s'arrête avec `panic!`.
- Une erreur **récupérable** : un fichier absent, un texte mal écrit… La fonction
  renvoie un `Result` et l'appelant décide quoi faire.

## Arrêter le programme avec panic!

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[10]);
    // thread 'main' panicked: index out of bounds: the len is 3 but the index is 10
}
```

Vous pouvez aussi déclencher `panic!` vous-même :

```rust norun
panic!("Ceci ne devrait jamais arriver");
```

> **Attention :** réservez `panic!` aux vrais bugs. Pour une erreur normale, utilisez
> `Result`.

## Le type Result

`Result<T, E>` vaut soit :

- `Ok(valeur)` : tout s'est bien passé ;
- `Err(erreur)` : quelque chose a échoué.

```rust
fn diviser(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", diviser(10.0, 4.0));
    println!("{:?}", diviser(1.0, 0.0));
}
```

```sortie
Ok(2.5)
Err("division par zéro")
```

## Traiter un Result avec match

```rust
fn main() {
    let essais = ["42", "abc"];
    for texte in essais {
        match texte.parse::<i32>() {
            Ok(n) => println!("{texte} → le nombre {n}"),
            Err(e) => println!("{texte} → erreur : {e}"),
        }
    }
}
```

```sortie
42 → le nombre 42
abc → erreur : invalid digit found in string
```

## Propager avec ?

L'opérateur `?` simplifie beaucoup le code. Placé après un `Result` :

- si c'est `Ok(v)`, il donne `v` ;
- si c'est `Err(e)`, la fonction s'arrête et renvoie `Err(e)`.

```rust
use std::num::ParseIntError;

fn somme(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", somme("2", "40"));
    println!("{}", somme("2", "quarante").is_err());
}
```

```sortie
Ok(42)
true
```

> **À retenir :** `?` ne s'utilise que dans une fonction qui renvoie elle-même un
> `Result` (ou une `Option`).

`main` peut aussi renvoyer un `Result` :

```rust
fn main() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "7".parse()?;
    println!("{}", n * 6);
    Ok(())
}
```

```sortie
42
```

## unwrap et expect

`unwrap()` récupère la valeur d'un `Ok` (ou d'un `Some`), et provoque un `panic!` sinon.
`expect()` fait pareil, avec votre propre message :

```rust
let n: i32 = "12".parse().unwrap();
let m: i32 = "30".parse().expect("un nombre était attendu");
println!("{}", n + m);
```

```sortie
42
```

```rust norun
let n: i32 = "douze".parse().expect("un nombre était attendu");
// panic : un nombre était attendu: ParseIntError { kind: InvalidDigit }
```

> **Astuce :** `unwrap` est pratique dans les essais et les exemples. Dans un vrai
> programme, préférez `match`, `?` ou une valeur par défaut.

## Valeurs par défaut et combinateurs

Des méthodes courtes évitent d'écrire un `match` complet. Elles marchent sur `Option` et
sur `Result` :

```rust
let vide: Option<i32> = None;
println!("{}", vide.unwrap_or(0));
println!("{:?}", Some(5).map(|n| n * 2));
println!("{}", "abc".parse::<i32>().unwrap_or(-1));
```

```sortie
0
Some(10)
-1
```

| Méthode | Effet |
|---------|-------|
| `unwrap_or(v)` | la valeur, ou `v` si absente |
| `map(f)` | transforme la valeur si elle existe |
| `is_some()` / `is_ok()` | teste la présence ou la réussite |

## Votre propre type d'erreur

Pour un programme plus grand, créez une [énumération](types.md#les-enumerations) qui
liste les erreurs possibles. `match` permet ensuite de réagir à chacune :

```rust
enum ErreurCompte {
    SoldeInsuffisant,
    MontantNul,
}

fn retirer(solde: u32, montant: u32) -> Result<u32, ErreurCompte> {
    if montant == 0 {
        return Err(ErreurCompte::MontantNul);
    }
    if montant > solde {
        return Err(ErreurCompte::SoldeInsuffisant);
    }
    Ok(solde - montant)
}

fn main() {
    for montant in [30, 0, 80] {
        match retirer(50, montant) {
            Ok(reste) => println!("Retrait OK, reste {reste} €"),
            Err(ErreurCompte::MontantNul) => println!("Erreur : le montant doit être positif"),
            Err(ErreurCompte::SoldeInsuffisant) => println!("Erreur : solde insuffisant"),
        }
    }
}
```

```sortie
Retrait OK, reste 20 €
Erreur : le montant doit être positif
Erreur : solde insuffisant
```

## En résumé

- `panic!` arrête le programme : réservez-le aux vrais bugs.
- `Result` vaut `Ok(valeur)` ou `Err(erreur)`.
- `match` traite les deux cas ; `unwrap_or` donne une valeur par défaut.
- `?` renvoie l'erreur à l'appelant, dans une fonction qui renvoie un `Result`.
- `unwrap` et `expect` sont pratiques pour les essais, risqués ailleurs.

Comparez avec [les erreurs en Ruby](../ruby/erreurs.md) et
[en Rythme](../rythme/erreurs.md). Suite : [Lire et écrire : clavier et fichiers](fichiers.md).
