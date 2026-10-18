# Le filtrage par motif

Le filtrage par motif compare une valeur à des formes possibles et en extrait les morceaux : c'est l'un des outils les plus puissants de Rust, avec `match`, `if let`, `let else` et `while let`.

> **Objectifs :**
>
> - écrire un `match` complet avec des valeurs, des intervalles et des alternatives ;
> - extraire les morceaux d'un tuple, d'une structure, d'une énumération ou d'une tranche ;
> - ajouter une condition à un motif avec une garde `if` ;
> - choisir entre `match`, `if let`, `let else` et `while let` ;
> - tester une forme rapidement avec `matches!`.

Un **motif** (en anglais : *pattern*) est une forme. Pensez au jeu des formes pour
enfants : le cube passe par le trou carré, le cylindre par le trou rond. Rust essaie
chaque trou dans l'ordre, et la valeur passe par le premier qui lui correspond.

## match : tous les cas, dans l'ordre

Vous avez déjà vu [match sur une énumération](types.md#match-sur-une-enumeration).
`match` fonctionne aussi avec des nombres, des caractères ou des textes. Chaque ligne
est un **bras** : un motif, une flèche `=>`, puis le code à exécuter.

```rust
let de = 3;
let texte = match de {
    1 => "un",
    2 => "deux",
    3 => "trois",
    _ => "beaucoup",
};
println!("{texte}");
```

```sortie
trois
```

Deux règles à retenir :

- les bras sont essayés **de haut en bas** ; le premier qui correspond gagne ;
- le `match` doit être **exhaustif** : toutes les valeurs possibles doivent être
  couvertes. Le motif `_` (« n'importe quoi d'autre ») sert de filet.

> **À retenir :** l'exhaustivité est une sécurité. Si vous ajoutez une variante à une
> énumération, le compilateur vous montre chaque `match` à compléter.

`match` est une [expression](../debuter/glossaire.md#expression) : il produit une
valeur, ici rangée dans `texte`. Tous les bras doivent alors produire le même type.

## Plusieurs valeurs : | et les intervalles

La barre `|` signifie « ou ». Un intervalle `a..=b` couvre toutes les valeurs de `a` à
`b` inclus (voir les [intervalles](../debuter/glossaire.md#intervalle)).

```rust
fn appreciation(note: u32) -> &'static str {
    match note {
        0..=7 => "insuffisant",
        8 | 9 => "presque",
        10..=15 => "bien",
        16..=20 => "très bien",
        _ => "note invalide",
    }
}

fn main() {
    for n in [5, 9, 12, 18, 25] {
        println!("{n} : {}", appreciation(n));
    }
}
```

```sortie
5 : insuffisant
9 : presque
12 : bien
18 : très bien
25 : note invalide
```

Les intervalles marchent aussi avec les caractères :

```rust
for c in ['a', 'Z', '7', '?'] {
    let genre = match c {
        'a'..='z' => "minuscule",
        'A'..='Z' => "majuscule",
        '0'..='9' => "chiffre",
        _ => "autre",
    };
    println!("{c} : {genre}");
}
```

```sortie
a : minuscule
Z : majuscule
7 : chiffre
? : autre
```

## Donner un nom avec @

Un motif peut **tester** une valeur et la **nommer** en même temps, avec `nom @ motif` :

```rust
let age = 15;
match age {
    0 => println!("nouveau-né"),
    n @ 1..=17 => println!("mineur de {n} ans"),
    n => println!("majeur de {n} ans"),
}
```

```sortie
mineur de 15 ans
```

Le dernier bras, `n`, est un nom tout seul : il accepte n'importe quelle valeur et la
range dans `n`. C'est comme `_`, mais en gardant la valeur.

## Décomposer un tuple

Un motif peut avoir la **forme** de la valeur, et ranger chaque morceau dans une
variable. C'est la **décomposition** (en anglais : *destructuring*).

```rust
let point = (3, -2);
match point {
    (0, 0) => println!("origine"),
    (x, 0) => println!("sur l'axe horizontal, x = {x}"),
    (0, y) => println!("sur l'axe vertical, y = {y}"),
    (x, y) => println!("ailleurs : ({x}, {y})"),
}
```

```sortie
ailleurs : (3, -2)
```

La décomposition marche aussi hors d'un `match`, avec `let` ou dans un `for` :

```rust
let (nom, age) = ("Zoé", 12);
println!("{nom} a {age} ans");
for (i, fruit) in ["pomme", "kiwi"].iter().enumerate() {
    println!("{i} : {fruit}");
}
```

```sortie
Zoé a 12 ans
0 : pomme
1 : kiwi
```

## Décomposer une structure

Pour une [structure](types.md#les-structures), on écrit le nom des champs. `..`
ignore les champs restants.

```rust
struct Joueur {
    nom: String,
    points: u32,
    niveau: u32,
}

fn main() {
    let j = Joueur { nom: String::from("Léo"), points: 120, niveau: 3 };

    let Joueur { nom, points, .. } = &j;
    println!("{nom} : {points} points");

    match j {
        Joueur { niveau: 1, .. } => println!("débutant"),
        Joueur { niveau: n, .. } => println!("niveau {n}"),
    }
}
```

```sortie
Léo : 120 points
niveau 3
```

`niveau: 1` teste la valeur du champ. `niveau: n` range le champ dans `n`.

## Décomposer une énumération

Les variantes d'une énumération peuvent porter des données. Le motif les extrait :

```rust
enum Forme {
    Cercle(f64),
    Rectangle { largeur: f64, hauteur: f64 },
    Point,
}

fn aire(f: &Forme) -> f64 {
    match f {
        Forme::Cercle(r) => 3.14 * r * r,
        Forme::Rectangle { largeur, hauteur } => largeur * hauteur,
        Forme::Point => 0.0,
    }
}

fn main() {
    let formes = [
        Forme::Cercle(1.0),
        Forme::Rectangle { largeur: 2.0, hauteur: 3.0 },
        Forme::Point,
    ];
    for f in &formes {
        println!("{}", aire(f));
    }
}
```

```sortie
3.14
6
0
```

## Décomposer une tranche

Un motif entre crochets décrit le **contenu** d'une liste. `..` remplace « tous les
éléments du milieu », quel que soit leur nombre.

```rust
fn decrire(v: &[i32]) -> String {
    match v {
        [] => String::from("vide"),
        [seul] => format!("un seul : {seul}"),
        [a, b] => format!("deux : {a} et {b}"),
        [premier, .., dernier] => format!("de {premier} à {dernier}"),
    }
}

fn main() {
    println!("{}", decrire(&[]));
    println!("{}", decrire(&[7]));
    println!("{}", decrire(&[1, 2]));
    println!("{}", decrire(&[1, 2, 3, 4, 5]));
}
```

```sortie
vide
un seul : 7
deux : 1 et 2
de 1 à 5
```

Avec `reste @ ..`, vous pouvez même nommer le milieu : `[premier, reste @ ..]` range
tout sauf le premier dans `reste`.

## Ajouter une condition : la garde

Une **garde** est un `if` placé après le motif. Le bras n'est choisi que si la
condition est vraie :

```rust
let temperatures = [-5, 0, 18, 35];
for t in temperatures {
    let avis = match t {
        t if t < 0 => "gel",
        0 => "zéro pile",
        t if t > 30 => "canicule",
        _ => "normal",
    };
    println!("{t} : {avis}");
}
```

```sortie
-5 : gel
0 : zéro pile
18 : normal
35 : canicule
```

> **Attention :** le compilateur ne regarde pas l'intérieur des gardes pour vérifier
> l'exhaustivité. Un dernier bras sans garde (`_`) est donc souvent nécessaire.

## Des motifs imbriqués

Les motifs s'emboîtent comme des poupées russes. Ici, une `Option` contient un tuple :

```rust
let commandes = [Some(("café", 2)), None, Some(("thé", 0)), Some(("jus", 1))];
for c in commandes {
    match c {
        None => println!("pas de commande"),
        Some((boisson, 0)) => println!("{boisson} : annulé"),
        Some((boisson, 1)) => println!("un {boisson}"),
        Some((boisson, n)) => println!("{n} × {boisson}"),
    }
}
```

```sortie
2 × café
pas de commande
thé : annulé
un jus
```

## if let : un seul cas intéressant

Quand un seul motif vous intéresse, `match` est un peu long. `if let` teste un motif
et exécute le bloc s'il correspond. Un `else` est possible :

```rust
let saisie = "42";
if let Ok(n) = saisie.parse::<i32>() {
    println!("le double vaut {}", n * 2);
} else {
    println!("pas un nombre");
}

let config: Option<&str> = None;
if let Some(theme) = config {
    println!("thème {theme}");
}
println!("fin");
```

```sortie
le double vaut 84
fin
```

## let else : sortir tôt

`let … else` fait l'inverse : il **décompose** la valeur, et si le motif ne
correspond pas, exécute le bloc `else`, qui doit **quitter** la fonction (`return`,
`break`, `continue` ou `panic!`). Après, la variable est disponible sans imbrication.

```rust
fn lire_age(texte: &str) -> String {
    let Ok(age) = texte.parse::<u32>() else {
        return String::from("âge invalide");
    };
    format!("vous avez {age} ans")
}

fn main() {
    println!("{}", lire_age("30"));
    println!("{}", lire_age("trente"));
}
```

```sortie
vous avez 30 ans
âge invalide
```

> **Astuce :** `let else` garde le cas normal tout à gauche du code, et les cas
> d'erreur réglés en haut. Le code reste plat et lisible.

## while let : boucler tant que ça correspond

`while let` répète une boucle tant que le motif correspond. C'est parfait pour vider
une pile :

```rust
let mut pile = vec![1, 2, 3];
while let Some(sommet) = pile.pop() {
    println!("je retire {sommet}");
}
println!("pile vide");
```

```sortie
je retire 3
je retire 2
je retire 1
pile vide
```

`pop` renvoie `Some(…)` tant qu'il reste des éléments, puis `None` : la boucle
s'arrête.

## matches! : une question oui ou non

La [macro](../debuter/glossaire.md#macro) `matches!` répond `true` ou `false` : « cette
valeur a-t-elle cette forme ? ». Elle accepte `|`, les intervalles et les gardes.

```rust
let c = 'e';
println!("{}", matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y'));

let notes = [12, 4, 18, 9];
let bonnes = notes.iter().filter(|n| matches!(n, 10..=20)).count();
println!("{bonnes} bonnes notes");

let resultat: Option<i32> = Some(7);
println!("{}", matches!(resultat, Some(x) if x > 5));
```

```sortie
true
2 bonnes notes
true
```

## Erreurs fréquentes

### Un match qui oublie des valeurs

```rust norun
fn main() {
    let note = 15;
    match note {
        0..=9 => println!("insuffisant"),
        10..=20 => println!("bien"),
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match note {
  |           ^^^^ patterns `i32::MIN..=-1_i32` and `21_i32..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
```

Vous savez qu'une note est entre 0 et 20, mais pas le compilateur : un `i32` peut
valoir -3 ou 1000. Ajoutez un dernier bras `_ => println!("invalide"),`.

### Croire qu'un nom compare avec une variable

```rust norun
fn main() {
    let cible = 5;
    let essai = 3;
    match essai {
        cible => println!("gagné !"),
        _ => println!("perdu"),
    }
}
```

```text
warning: unreachable pattern
 --> src/main.rs:6:9
  |
5 |         cible => println!("gagné !"),
  |         ----- matches any value
6 |         _ => println!("perdu"),
  |         ^ no value can reach this
  |
note: there is a binding of the same name; if you meant to pattern match against the value of that binding, that is a feature of constants that is not available for `let` bindings
```

C'est seulement un avertissement, mais le programme affiche « gagné ! ». Dans un
motif, un nom seul **crée** une nouvelle variable qui accepte tout. Pour comparer,
utilisez une garde : `n if n == cible => …`, ou simplement `if essai == cible`.

### Un let else qui ne quitte pas

```rust norun
fn main() {
    let texte = "abc";
    let Ok(n) = texte.parse::<i32>() else {
        println!("pas un nombre");
    };
    println!("{n}");
}
```

```text
error[E0308]: `else` clause of `let...else` does not diverge
 --> src/main.rs:3:43
  |
3 |       let Ok(n) = texte.parse::<i32>() else {
  |  ___________________________________________^
4 | |         println!("pas un nombre");
5 | |     };
  | |_____^ expected `!`, found `()`
  |
  = note:   expected type `!`
          found unit type `()`
  = help: try adding a diverging expression, such as `return` or `panic!(..)`
  = help: ...or use `match` instead of `let...else`
```

Après le `let else`, `n` doit exister. Le bloc `else` doit donc sortir : ajoutez
`return;` après le `println!`.

## Testez-vous

**1.** Que fait le motif `_` ? Et le motif `x` tout seul ?

> **Réponse**
>
> Les deux acceptent n'importe quelle valeur. `_` l'ignore ; `x` la range dans une
> nouvelle variable `x`.

**2.** Quel motif accepte une tranche d'au moins deux éléments, en nommant le premier
et le dernier ?

> **Réponse**
>
> `[premier, .., dernier]`. Il accepte deux éléments ou plus.

**3.** Écrivez avec `matches!` un test qui dit si un caractère est un chiffre de 0 à 9.

> **Réponse**
>
> ```rust
> let c = '4';
> println!("{}", matches!(c, '0'..='9'));
> println!("{}", matches!('x', '0'..='9'));
> ```
>
> ```sortie
> true
> false
> ```

## En résumé

- `match` essaie les bras dans l'ordre et doit couvrir tous les cas ; `_` sert de filet.
- `|` combine des motifs, `a..=b` couvre un intervalle, `nom @ motif` garde la valeur.
- Un motif peut décomposer un tuple, une structure, une énumération ou une tranche.
- Une garde `if` ajoute une condition à un bras.
- `if let` pour un seul cas, `let else` pour sortir tôt, `while let` pour boucler.
- `matches!` répond `true` ou `false` à « cette valeur a-t-elle cette forme ? ».

Suite : [Génériques et traits](generiques.md)
