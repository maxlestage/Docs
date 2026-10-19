# Premiers pas

Cette page présente les variables, les types de base, l'affichage avec `println!`, les commentaires et les constantes en Rust.

> **Objectifs :**
>
> - créer des variables avec `let`, et les rendre modifiables avec `mut` ;
> - choisir le bon type de nombre et convertir avec `as` ;
> - regrouper des valeurs dans un tuple ;
> - afficher des valeurs avec `println!` et `format!` ;
> - écrire des commentaires et des constantes.

## Les variables avec let

Un programme doit souvent se souvenir d'une valeur pour s'en resservir plus loin : un
nom, un score, un prix. C'est le rôle des variables.

Une [variable](../debuter/glossaire.md#variable) est une étiquette posée sur une
valeur. Elle se crée avec `let`. Par défaut, elle ne peut **pas** changer : elle est
[immuable](../debuter/glossaire.md#immuable).

```rust
let ville = "Lyon";
println!("{ville}");
```

```sortie
Lyon
```

Si vous essayez de la modifier, le compilateur refuse (voir
[Erreurs fréquentes](#erreurs-frequentes)).

## Les variables modifiables avec mut

Certaines valeurs doivent changer pendant le programme : un score qui monte à chaque
point, l'argent d'un porte-monnaie. Pour elles, Rust demande un mot de plus.

Ajoutez `mut` pour rendre la variable [mutable](../debuter/glossaire.md#mutable),
c'est-à-dire modifiable :

```rust
let mut compteur = 0;
compteur = compteur + 1;
compteur += 1;
println!("{compteur}");
```

```sortie
2
```

Ligne par ligne :

- `let mut compteur = 0;` crée une variable modifiable qui vaut 0 ;
- `compteur = compteur + 1;` lit la valeur actuelle (0), ajoute 1, et range le
  résultat, 1, dans `compteur` ;
- `compteur += 1;` fait la même chose en plus court : `compteur` vaut maintenant 2 ;
- `println!` affiche la valeur finale, 2.

`compteur += 1` est un raccourci pour `compteur = compteur + 1`. Il existe aussi `-=`,
`*=` et `/=`.

> **À retenir :** en Rust, tout est constant par défaut. Écrivez `mut` seulement quand
> la valeur doit changer.

**À vous :** écrivez un programme qui crée une variable modifiable `argent` qui vaut
10, lui ajoute 5 avec `+=`, puis affiche sa valeur.

> **Solution**
>
> ```rust
> let mut argent = 10;
> argent += 5;
> println!("{argent}");
> ```
>
> ```sortie
> 15
> ```

## Le masquage

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Vous pouvez déclarer une nouvelle variable avec le même nom. L'ancienne est **masquée**
(en anglais : *shadowing*) : elle existe encore, mais le nom désigne désormais la
nouvelle. Le type peut même changer :

```rust
let texte = "42";
let texte: i32 = texte.parse().unwrap();
let texte = texte * 2;
println!("{texte}");
```

```sortie
84
```

Ici, `texte` est d'abord du texte, puis un nombre. `parse` lit un nombre dans un texte
(détails dans [Travailler avec du texte](texte.md)).

> **Note :** masquer n'est pas modifier. Avec `let`, vous créez une **nouvelle**
> variable ; avec `mut`, vous changez la valeur de la même variable.

## Les types simples

Un nombre et un texte ne se rangent pas de la même façon, et on ne fait pas les mêmes
choses avec : on additionne des prix, pas des prénoms. Le type permet à Rust de savoir
ce qu'il a entre les mains, et de vous prévenir si vous mélangez tout.

Un [type](../debuter/glossaire.md#type) dit quelle sorte de valeur contient une
variable. Rust devine souvent le type tout seul. Vous pouvez aussi l'écrire après `:`.

| Type | Contenu | Exemple |
|------|---------|---------|
| `i32` | [entier](../debuter/glossaire.md#entier) (positif ou négatif) | `-7` |
| `u32` | entier positif | `42` |
| `f64` | nombre à virgule ([flottant](../debuter/glossaire.md#flottant)) | `3.14` |
| `bool` | vrai ou faux ([booléen](../debuter/glossaire.md#booleen)) | `true` |
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

Ligne par ligne :

- les cinq `let` créent une variable de chaque type ; le type est écrit après `:` ;
- `7 / 2` divise deux entiers : le résultat est aussi un entier, `3`. Ce qui vient
  après la virgule est perdu ;
- `7.0 / 2.0` divise deux nombres à virgule : le résultat est `3.5` ;
- `7 % 2` donne le reste de la division de 7 par 2 : `1`.

> **Attention :** une division entre entiers donne un entier : `7 / 2` vaut `3`. Le
> `%` donne le reste de la division : `7 % 2` vaut `1`.

### Les tailles d'entiers

Un entier occupe une place fixe en mémoire. La lettre dit s'il peut être négatif (`i`
pour *integer*, signé) ou non (`u` pour *unsigned*, non signé). Le nombre donne sa
taille, en bits :

| Type | Plus petite valeur | Plus grande valeur |
|------|--------------------|--------------------|
| `u8` | 0 | 255 |
| `i32` | −2 147 483 648 | 2 147 483 647 |
| `u32` | 0 | 4 294 967 295 |
| `i64` | environ −9 × 10¹⁸ | environ 9 × 10¹⁸ |
| `usize` | 0 | dépend de la machine (les positions dans une liste) |

```rust
println!("{} {}", u8::MIN, u8::MAX);
println!("{}", i32::MAX);
```

```sortie
0 255
2147483647
```

> **Astuce :** sans raison particulière, prenez `i32` pour un entier et `f64` pour un
> nombre à virgule. Ce sont les choix par défaut de Rust.

### Le dépassement

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Que se passe-t-il si un calcul sort des limites ? C'est un **dépassement** (en
anglais : *overflow*). Pendant le développement, le programme s'arrête avec une erreur.
Pour choisir vous-même le comportement, des méthodes existent :

```rust
let x: u8 = 250;
println!("{}", x.saturating_add(10));
println!("{}", x.wrapping_add(10));
```

```sortie
255
4
```

- `saturating_add` s'arrête au maximum, comme une jauge pleine ;
- `wrapping_add` repart de zéro, comme un compteur kilométrique qui fait le tour.

### Convertir avec as

Rust ne mélange jamais deux types de nombres tout seul. Pour convertir, écrivez `as`
suivi du type voulu :

```rust
let pommes: i32 = 7;
let moitie = pommes as f64 / 2.0;
println!("{moitie}");
let prix = 3.99;
println!("{}", prix as i32);
let grand: i32 = 300;
println!("{}", grand as u8);
```

```sortie
3.5
3
44
```

> **Attention :** `as` ne vérifie rien. De `f64` vers `i32`, il coupe les décimales
> (`3.99` devient `3`, sans arrondir). Vers un type trop petit, le résultat est
> surprenant : `300` converti en `u8` vaut `44`.

**À vous :** un sac contient `bonbons: i32 = 9`. Écrivez un programme qui partage ces
bonbons entre 2 enfants et affiche la part de chacun, avec la virgule.

> **Solution**
>
> ```rust
> let bonbons: i32 = 9;
> let part = bonbons as f64 / 2.0;
> println!("{part}");
> ```
>
> ```sortie
> 4.5
> ```

## Les tuples

Certaines valeurs vont toujours ensemble : les deux coordonnées d'un point sur une
carte, ou le nom et l'âge d'une personne. Un tuple les range dans un seul paquet, comme
une fiche.

Un **tuple** regroupe quelques valeurs, de types qui peuvent être différents, entre
parenthèses. On lit chaque valeur avec `.0`, `.1`, `.2`… :

```rust
let point = (3, 4);
let eleve = ("Léa", 17, true);
println!("{} {}", point.0, point.1);
println!("{:?}", eleve);
let (nom, age, _) = eleve;
println!("{nom} a {age} ans");
```

```sortie
3 4
("Léa", 17, true)
Léa a 17 ans
```

Ligne par ligne :

- `point` regroupe deux nombres ; `eleve` regroupe un texte, un nombre et un booléen ;
- `point.0` lit la première valeur (3) et `point.1` la deuxième (4) : on compte à
  partir de 0 ;
- `{:?}` affiche le tuple entier, avec ses parenthèses ;
- le dernier `println!` utilise `nom` et `age`, sortis du tuple comme expliqué
  juste en dessous.

La ligne `let (nom, age, _) = eleve;` **décompose** le tuple : chaque valeur va dans
sa variable. Le `_` ignore la troisième.

## Les textes : String et &str

Un programme manipule beaucoup de texte : des prénoms, des messages, des réponses à
afficher. Rust range ce texte de deux façons, selon qu'il doit pouvoir changer ou non.

Rust a deux types de texte (de [chaîne de
caractères](../debuter/glossaire.md#chaine-de-caracteres)) :

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

Ligne par ligne :

- `salut` est un `&str` : un texte fixe, écrit directement dans le code ;
- `String::from(salut)` en fait une `String`, rangée dans `message`. Le `mut` permet
  de la modifier ensuite ;
- `push_str` ajoute un morceau de texte à la fin ; `push` ajoute un seul caractère,
  écrit entre apostrophes ;
- `message.len()` donne la longueur du texte : 18.

Le texte a sa propre page : [Travailler avec du texte](texte.md).

## Afficher avec println! et format!

Afficher, c'est la façon dont votre programme vous parle : il montre un résultat, un
message, une valeur à vérifier. Souvent, vous voulez glisser des valeurs au milieu
d'une phrase.

`println!` affiche, `format!` fabrique une `String` sans l'afficher. Les `{}` sont des
emplacements, remplacés par les valeurs :

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

Ligne par ligne :

- le premier `println!` remplit les deux `{}` dans l'ordre : d'abord `nom`, puis
  `note` ;
- le deuxième écrit le nom des variables entre les accolades, et `:.1` arrondit `note`
  à un chiffre après la virgule ;
- `{:>5}` et `{:<5}` réservent 5 places et collent `42` à droite ou à gauche ; les
  crochets montrent les espaces ;
- `format!` fabrique le texte `"Léa !"` et le range dans `phrase`, sans rien afficher :
  c'est le dernier `println!` qui l'affiche.

| Emplacement | Effet |
|-------------|-------|
| `{}` | affichage normal |
| `{nom}` | affiche la variable `nom` |
| `{:.2}` | deux chiffres après la virgule |
| `{:>5}` | aligné à droite sur 5 caractères |
| `{:?}` | affichage de débogage |

`print!` (sans `ln`) affiche sans aller à la ligne.

**À vous :** écrivez un programme qui range `"pain"` dans `produit` et `1.456` dans
`prix`, puis affiche « Le pain coûte 1.46 euros », avec deux chiffres après la virgule.

> **Solution**
>
> ```rust
> let produit = "pain";
> let prix = 1.456;
> println!("Le {produit} coûte {prix:.2} euros");
> ```
>
> ```sortie
> Le pain coûte 1.46 euros
> ```

## Les commentaires

Un programme se relit souvent : par vous dans un mois, ou par quelqu'un d'autre. Un
commentaire explique ce que fait le code, comme une note dans la marge d'une recette.

Un [commentaire](../debuter/glossaire.md#commentaire) est une note pour les humains :
le compilateur l'ignore.

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

Ligne par ligne :

- les lignes qui commencent par `//`, et tout ce qui est entre `/*` et `*/`, sont
  ignorés ;
- `///` décrit la fonction `double` qui suit : des outils s'en servent pour fabriquer
  la documentation ;
- dans `main`, le commentaire `// affiche 8` est en fin de ligne : le code placé avant
  lui s'exécute normalement.

## Les constantes

Certaines valeurs ne changent jamais et servent à plusieurs endroits, comme le taux de
TVA. Avec un nom, le code se lit mieux, et vous ne la corrigez qu'à un seul endroit.

Une [constante](../debuter/glossaire.md#constante) est une valeur fixée une fois pour
toutes. Elle s'écrit avec `const`, en MAJUSCULES, et son type est obligatoire :

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

Ligne par ligne :

- les deux `const` sont écrites tout en haut, en dehors de `main` : tout le programme
  peut s'en servir ;
- `100.0 * TAUX_TVA` calcule 100 × 0,2, soit `20` ;
- `JOURS_PAR_SEMAINE * 4` donne le nombre de jours dans 4 semaines : `28`.

**À vous :** écrivez un programme avec une constante `PRIX_TICKET`, de type `f64`, qui
vaut `2.5`. Affichez le prix de 4 tickets.

> **Solution**
>
> ```rust
> const PRIX_TICKET: f64 = 2.5;
>
> fn main() {
>     println!("{}", PRIX_TICKET * 4.0);
> }
> ```
>
> ```sortie
> 10
> ```

## Erreurs fréquentes

### Modifier une variable sans mut

```rust norun
fn main() {
    let score = 10;
    score = 20;
    println!("{score}");
}
```

```text
error[E0384]: cannot assign twice to immutable variable `score`
 --> src/main.rs:3:5
  |
2 |     let score = 10;
  |         ----- first assignment to `score`
3 |     score = 20;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable
help: consider making this binding mutable
```

Correction : `let mut score = 10;`.

### Mélanger entiers et nombres à virgule

```rust norun
fn main() {
    let total = 5 + 2.5;
    println!("{total}");
}
```

```text
error[E0277]: cannot add a float to an integer
 --> src/main.rs:2:19
  |
2 |     let total = 5 + 2.5;
  |                   ^ no implementation for `{integer} + {float}`
```

Correction : écrivez `5.0 + 2.5`, ou convertissez une variable avec `as f64`.

### Un nombre trop grand pour son type

```rust norun
fn main() {
    let petit: u8 = 300;
    println!("{petit}");
}
```

```text
error: literal out of range for `u8`
 --> src/main.rs:2:21
  |
2 |     let petit: u8 = 300;
  |                     ^^^
  |
  = note: the literal `300` does not fit into the type `u8` whose range is `0..=255`
```

Correction : prenez un type plus grand, comme `u32` ou `i32`.

### Une constante sans type

```rust norun
const MAX = 10;

fn main() {
    println!("{MAX}");
}
```

```text
error: missing type for `const` item
 --> src/main.rs:1:10
  |
1 | const MAX = 10;
  |          ^ help: provide a type for the constant: `: i32`
```

Correction : `const MAX: i32 = 10;`.

## Testez-vous

**1.** Ce code compile-t-il ? Sinon, comment le corriger ?

```rust norun
let points = 0;
points += 5;
```

> **Réponse**
>
> Non : `points` n'est pas modifiable. Écrivez `let mut points = 0;`.

**2.** Que valent `9 / 4` et `9.0 / 4.0` ?

> **Réponse**
>
> `9 / 4` vaut `2` (division entière) et `9.0 / 4.0` vaut `2.25`.

**3.** Comment afficher la moyenne de `total: i32 = 7` sur `2` élèves, avec la virgule ?

> **Réponse**
>
> Convertissez avec `as` avant de diviser :
>
> ```rust
> let total: i32 = 7;
> println!("{}", total as f64 / 2.0);
> ```
>
> ```sortie
> 3.5
> ```

## En résumé

- `let` crée une variable non modifiable ; `let mut` la rend modifiable.
- Les types simples : `i32`, `u32`, `f64`, `bool`, `char` ; `as` convertit un nombre.
- Chaque entier a des limites (`u8` : 0 à 255) ; les dépasser est une erreur.
- Un tuple `(a, b)` regroupe quelques valeurs, lues avec `.0`, `.1`.
- `println!("{x}")` affiche, `format!` fabrique une `String`.
- `const` crée une constante, en MAJUSCULES, avec son type.

Le même chapitre existe pour [Ruby](../ruby/premiers-pas.md) et
[Rythme](../rythme/premiers-pas.md).

Suite : [Nombres et calculs](nombres.md)
