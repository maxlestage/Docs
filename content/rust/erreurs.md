# Gérer les erreurs

Cette page explique les deux sortes d'erreurs de Rust : les erreurs fatales avec `panic!`, et les erreurs prévues avec `Result` et l'opérateur `?`.

> **Objectifs :**
>
> - distinguer une erreur fatale (`panic!`) d'une erreur prévue (`Result`) ;
> - renvoyer et traiter un `Result` avec `match` ;
> - transmettre une erreur à l'appelant avec `?` ;
> - choisir entre `unwrap`, `expect` et une valeur par défaut ;
> - créer votre propre type d'erreur.

## Deux sortes d'erreurs

Une [erreur](../debuter/glossaire.md#erreur), c'est quand le programme ne peut pas
faire ce qu'on lui demande. Rust en distingue deux sortes, comme une voiture : une
panne moteur vous arrête net, un bouchon vous oblige seulement à changer d'itinéraire.

- Une erreur **irrécupérable** : un bug, une situation impossible. Le programme
  s'arrête avec `panic!`.
- Une erreur **récupérable** : un fichier absent, un texte mal écrit… La fonction
  renvoie un `Result` et l'appelant décide quoi faire.

## Arrêter le programme avec panic!

`panic!` arrête le programme et affiche un message. Lire la case 10 d'une liste de 3
éléments, par exemple, provoque un `panic!` :

```rust norun
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[10]);
}
```

```text
thread 'main' panicked at src/main.rs:3:21:
index out of bounds: the len is 3 but the index is 10
```

Vous pouvez aussi déclencher `panic!` vous-même :

```rust norun
panic!("Ceci ne devrait jamais arriver");
```

> **Attention :** réservez `panic!` aux vrais bugs. Pour une erreur normale, utilisez
> `Result`.

## Le type Result

`Result<T, E>` est une [énumération](types.md#les-enumerations) à deux variantes. `T`
est le type de la valeur en cas de réussite, `E` celui de l'erreur. Un `Result` vaut
soit :

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

Pensez à une enveloppe de résultat d'examen : elle contient soit le diplôme, soit une
lettre qui explique l'échec. Vous devez l'ouvrir pour savoir.

## Traiter un Result avec match

`match` ouvre le `Result` et traite les deux cas :

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

L'opérateur `?` simplifie beaucoup le code. Il **transmet** l'erreur à la fonction qui
a appelé, comme un employé qui renvoie un dossier à son chef au lieu de le traiter.
Placé après un `Result` :

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

`ParseIntError` est le type d'erreur renvoyé par `parse` pour un entier. `main` peut
aussi renvoyer un `Result`. `Ok(())` signifie « réussi, sans valeur à donner » :

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

Si la valeur manque, le programme s'arrête avec votre message :

```rust norun
fn main() {
    let n: i32 = "douze".parse().expect("un nombre était attendu");
}
```

```text
thread 'main' panicked at src/main.rs:2:34:
un nombre était attendu: ParseIntError { kind: InvalidDigit }
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

## Erreurs fréquentes

### Utiliser ? dans une fonction qui ne renvoie pas de Result

```rust norun
fn main() {
    let n: i32 = "7".parse()?;
    println!("{n}");
}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:29
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: i32 = "7".parse()?;
  |                             ^ cannot use the `?` operator in a function that returns `()`
```

`?` doit pouvoir renvoyer l'erreur. Faites renvoyer un `Result` à la fonction :
`fn main() -> Result<(), std::num::ParseIntError>`, avec `Ok(())` à la fin.

### Prendre un Result pour sa valeur

```rust norun
fn main() {
    let n: i32 = "42".parse::<i32>();
    println!("{n}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let n: i32 = "42".parse::<i32>();
  |            ---   ^^^^^^^^^^^^^^^^^^^ expected `i32`, found `Result<i32, ParseIntError>`
  |            |
  |            expected due to this
```

`parse` renvoie l'enveloppe, pas le nombre. Ouvrez-la : `match`, `?`, `unwrap_or(0)` ou
`expect("…")`.

### Oublier le cas Err

```rust norun
fn main() {
    match "42".parse::<i32>() {
        Ok(n) => println!("{n}"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:2:11
  |
2 |     match "42".parse::<i32>() {
  |           ^^^^^^^^^^^^^^^^^^^ pattern `Err(_)` not covered
```

Rust vous oblige à prévoir l'échec. Ajoutez `Err(e) => println!("erreur : {e}"),`.

## Testez-vous

**1.** Quand utiliser `panic!` plutôt que `Result` ?

> **Réponse**
>
> Pour un bug, une situation qui ne devrait jamais arriver. Une erreur prévisible
> (fichier absent, saisie invalide) se traite avec `Result`.

**2.** Que fait `?` après un `Err(e)` ?

> **Réponse**
>
> Il arrête la fonction tout de suite et renvoie `Err(e)` à l'appelant.

**3.** Lisez un âge dans le texte `"abc"`, avec `0` si la lecture échoue.

> **Réponse**
>
> ```rust
> let age: u32 = "abc".parse().unwrap_or(0);
> println!("{age}");
> ```
>
> ```sortie
> 0
> ```

## En résumé

- `panic!` arrête le programme : réservez-le aux vrais bugs.
- `Result` vaut `Ok(valeur)` ou `Err(erreur)`.
- `match` traite les deux cas ; `unwrap_or` donne une valeur par défaut.
- `?` renvoie l'erreur à l'appelant, dans une fonction qui renvoie un `Result`.
- `unwrap` et `expect` sont pratiques pour les essais, risqués ailleurs.

Comparez avec [les erreurs en Ruby](../ruby/erreurs.md) et
[en Rythme](../rythme/erreurs.md).

Suite : [Les closures en détail](closures.md)
