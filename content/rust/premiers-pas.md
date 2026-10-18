# Premiers pas

Cette page présente les variables, les types de base, l'affichage avec `println!`, les commentaires et les constantes en Rust.

## Les variables avec let

Une variable se crée avec `let`. Par défaut, elle ne peut **pas** changer.

```rust
let ville = "Lyon";
println!("{ville}");
```

```sortie
Lyon
```

Si vous essayez de la modifier, le compilateur refuse :

```rust norun
let x = 5;
x = 6; // erreur : cannot assign twice to immutable variable
```

## Les variables modifiables avec mut

Ajoutez `mut` pour pouvoir changer la valeur :

```rust
let mut compteur = 0;
compteur = compteur + 1;
compteur += 1;
println!("{compteur}");
```

```sortie
2
```

> **À retenir :** en Rust, tout est constant par défaut. Écrivez `mut` seulement quand
> la valeur doit changer.

## Le masquage

Vous pouvez déclarer une nouvelle variable avec le même nom. L'ancienne est **masquée**
(en anglais : *shadowing*). Le type peut même changer :

```rust
let texte = "42";
let texte: i32 = texte.parse().unwrap();
let texte = texte * 2;
println!("{texte}");
```

```sortie
84
```

## Les types simples

Rust devine souvent le type tout seul. Vous pouvez aussi l'écrire après `:`.

| Type | Contenu | Exemple |
|------|---------|---------|
| `i32` | entier (positif ou négatif) | `-7` |
| `u32` | entier positif | `42` |
| `f64` | nombre à virgule | `3.14` |
| `bool` | vrai ou faux | `true` |
| `char` | un caractère | `'é'` |

```rust
let entier: i32 = -7;
let positif: u32 = 42;
let prix: f64 = 9.99;
let actif: bool = true;
let lettre: char = 'é';
println!("{entier} {positif} {prix} {actif} {lettre}");
println!("{}", 7 / 2);
println!("{}", 7.0 / 2.0);
println!("{}", 7 % 2);
```

```sortie
-7 42 9.99 true é
3
3.5
1
```

> **Attention :** une division entre entiers donne un entier : `7 / 2` vaut `3`.

## Les textes : String et &str

Rust a deux types de texte :

- `&str` : un texte fixe, souvent écrit entre guillemets dans le code.
- `String` : un texte qui vous appartient et que vous pouvez agrandir.

```rust
let salut: &str = "Bonjour";
let mut message = String::from(salut);
message.push_str(", le monde");
message.push('!');
println!("{message}");
println!("{}", message.len());
```

```sortie
Bonjour, le monde!
18
```

La différence entre les deux est expliquée dans [Possession et emprunt](possession.md).

## Afficher avec println! et format!

`println!` affiche, `format!` fabrique une `String` sans l'afficher. Les `{}` sont des
emplacements :

```rust
let nom = "Léa";
let note = 17.456;
println!("{} a eu {}", nom, note);
println!("{nom} a eu {note:.1}");
println!("[{:>5}]", 42);
println!("[{:<5}]", 42);
println!("{:?}", "avec guillemets");
let phrase = format!("{nom} !");
println!("{phrase}");
```

```sortie
Léa a eu 17.456
Léa a eu 17.5
[   42]
[42   ]
"avec guillemets"
Léa !
```

| Emplacement | Effet |
|-------------|-------|
| `{}` | affichage normal |
| `{nom}` | affiche la variable `nom` |
| `{:.2}` | deux chiffres après la virgule |
| `{:>5}` | aligné à droite sur 5 caractères |
| `{:?}` | affichage de débogage |

## Les commentaires

```rust
// Un commentaire sur une ligne.
/* Un commentaire
   sur plusieurs lignes. */
/// Un commentaire de documentation (avant une fonction, une structure…).
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", double(4)); // affiche 8
}
```

```sortie
8
```

## Les constantes

Une constante s'écrit avec `const`, en MAJUSCULES, et son type est obligatoire :

```rust
const TAUX_TVA: f64 = 0.2;
const JOURS_PAR_SEMAINE: u32 = 7;

fn main() {
    println!("{}", 100.0 * TAUX_TVA);
    println!("{}", JOURS_PAR_SEMAINE * 4);
}
```

```sortie
20
28
```

> **Astuce :** le même chapitre existe pour [Ruby](../ruby/premiers-pas.md) et
> [Rythme](../rythme/premiers-pas.md). Suite : [Conditions et boucles](conditions-boucles.md).
