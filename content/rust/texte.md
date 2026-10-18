# Travailler avec du texte

Cette page montre comment créer, transformer, découper et parcourir du texte en Rust, avec les deux types `String` et `&str`.

> **Objectifs :**
>
> - choisir entre `String` et `&str` ;
> - construire un texte avec `push_str`, `format!` et `join` ;
> - chercher, remplacer et découper un texte ;
> - convertir un texte en nombre, et l'inverse ;
> - parcourir les caractères sans couper une lettre accentuée.

## Deux types de texte : String et &str

Un texte, ou [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres), est
une suite de lettres. Rust a deux types de texte. Pour débuter, retenez ceci :

- `&str` : un texte que vous **regardez**. Les textes entre guillemets sont des `&str`.
- `String` : un texte que vous **possédez** et que vous pouvez modifier.

Pensez à un livre : `&str`, c'est le livre de la bibliothèque, que vous lisez sans
écrire dedans ; `String`, c'est votre propre cahier, où vous pouvez ajouter des pages.

```rust
let regarde: &str = "bonjour";
let possede: String = String::from("bonjour");
let autre: String = "salut".to_string();
println!("{regarde} {possede} {autre}");
```

```sortie
bonjour bonjour salut
```

`String::from("…")` et `.to_string()` fabriquent une `String` à partir d'un `&str`.
Dans l'autre sens, pour passer d'une `String` à un `&str`, ajoutez `&` devant :

```rust
fn afficher(t: &str) {
    println!("[{t}]");
}

fn main() {
    let nom = String::from("Léa");
    afficher(&nom);
    afficher("Tom");
}
```

```sortie
[Léa]
[Tom]
```

> **À retenir :** un paramètre de fonction de type `&str` accepte les deux sortes de
> texte. C'est le choix le plus pratique.

## Agrandir une String

Une `String` déclarée avec `mut` peut grandir. `push_str` ajoute un texte, `push` ajoute
un seul caractère :

```rust
let mut phrase = String::from("Bonjour");
phrase.push_str(" tout le monde");
phrase.push('!');
println!("{phrase}");
```

```sortie
Bonjour tout le monde!
```

L'opérateur `+` colle aussi deux textes, mais à gauche il faut une `String` :

```rust
let debut = String::from("Bon");
let mot = debut + "jour";
println!("{mot}");
```

```sortie
Bonjour
```

## Assembler avec format!

`format!` fonctionne comme `println!`, mais il **renvoie** une `String` au lieu de
l'afficher :

```rust
let prenom = "Alice";
let age = 30;
let fiche = format!("{prenom} ({age} ans)");
println!("{fiche}");
```

```sortie
Alice (30 ans)
```

> **Astuce :** `format!` est le moyen le plus simple de coller plusieurs morceaux de
> texte et de nombres.

## La longueur d'un texte

`len()` compte les **octets**, pas les lettres. Un octet est la plus petite case de la
mémoire. Une lettre simple prend un octet, une lettre accentuée en prend deux.
Pour compter les caractères, utilisez `chars().count()` :

```rust
let mot = "été";
println!("{}", mot.len());
println!("{}", mot.chars().count());
```

```sortie
5
3
```

> **Attention :** pour un texte en français, préférez `chars().count()`.

## Changer la casse et nettoyer

```rust
let brut = "  Bonjour Rust  ";
println!("[{}]", brut.trim());
println!("{}", brut.trim().to_uppercase());
println!("{}", brut.trim().to_lowercase());
```

```sortie
[Bonjour Rust]
BONJOUR RUST
bonjour rust
```

- `trim()` enlève les espaces au début et à la fin ;
- `to_uppercase()` met en majuscules ;
- `to_lowercase()` met en minuscules.

## Chercher dans un texte

Ces méthodes répondent par `true` ou `false` :

```rust
let phrase = "Le chat dort";
println!("{}", phrase.contains("chat"));
println!("{}", phrase.starts_with("Le"));
println!("{}", phrase.ends_with("!"));
```

```sortie
true
true
false
```

## Remplacer

`replace` renvoie une **nouvelle** `String`. Le texte de départ ne change pas :

```rust
let avant = "J'aime le thé";
let apres = avant.replace("thé", "café");
println!("{avant}");
println!("{apres}");
```

```sortie
J'aime le thé
J'aime le café
```

## Découper avec split

`split` coupe le texte à chaque séparateur. `collect` range les morceaux dans un `Vec` :

```rust
let liste = "pomme,poire,kiwi";
let fruits: Vec<&str> = liste.split(',').collect();
println!("{:?}", fruits);
println!("{}", fruits.len());
```

```sortie
["pomme", "poire", "kiwi"]
3
```

Pour couper sur les espaces, `split_whitespace` est plus pratique :

```rust
for mot in "un  deux trois".split_whitespace() {
    println!("{mot}");
}
```

```sortie
un
deux
trois
```

Pour faire l'inverse, `join` recolle les morceaux :

```rust
let mots = vec!["a", "b", "c"];
println!("{}", mots.join("-"));
```

```sortie
a-b-c
```

## Transformer un texte en nombre

`parse` lit un nombre dans un texte. Indiquez le type voulu avec `::<i32>`. Le résultat
est un `Result`, car le texte n'est peut-être pas un nombre :

```rust
let bon = "42".parse::<i32>();
let mauvais = "abc".parse::<i32>();
println!("{:?}", bon);
println!("{}", mauvais.is_err());
let n = "  7 ".trim().parse::<i32>().unwrap();
println!("{}", n * 2);
```

```sortie
Ok(42)
true
14
```

> **Astuce :** pensez à `trim()` avant `parse` : un espace ou un retour à la ligne fait
> échouer la lecture. Les erreurs sont expliquées dans [Gérer les erreurs](erreurs.md).

Dans l'autre sens, `to_string()` transforme un nombre en texte :

```rust
let texte = 2026.to_string();
println!("{}", texte.len());
```

```sortie
4
```

## Parcourir les caractères

`chars()` donne les caractères un par un (des valeurs de type `char`) :

```rust
let mot = "Noël";
for c in mot.chars() {
    print!("{c}.");
}
println!();
let voyelles = mot.chars().filter(|c| "aeiouyë".contains(*c)).count();
println!("{voyelles} voyelles");
```

```sortie
N.o.ë.l.
2 voyelles
```

Pour retourner un texte, combinez `chars`, `rev` et `collect` :

```rust
let envers: String = "radar".chars().rev().collect();
println!("{envers}");
```

```sortie
radar
```

## Prendre un morceau avec prudence

On peut prendre une partie d'un texte avec `&texte[début..fin]`. Les nombres sont des
**octets** :

```rust
let jour = "lundi matin";
println!("{}", &jour[0..5]);
println!("{}", &jour[6..]);
```

```sortie
lundi
matin
```

> **Attention :** si la coupure tombe au milieu d'une lettre accentuée, le programme
> s'arrête avec un `panic!`.

```rust norun
let mot = "été";
println!("{}", &mot[0..1]); // panic : byte index 1 is not a char boundary
```

Pour un texte avec des accents, passez par les caractères :

```rust
let mot = "été";
let debut: String = mot.chars().take(2).collect();
println!("{debut}");
```

```sortie
ét
```

## Les textes bruts

Un texte brut s'écrit `r"…"`. Les barres obliques `\` y sont gardées telles quelles.
Avec `r#"…"#`, il peut même contenir des guillemets :

```rust
let chemin = r"C:\dossier\nouveau";
let citation = r#"Il a dit "bonjour""#;
println!("{chemin}");
println!("{citation}");
```

```sortie
C:\dossier\nouveau
Il a dit "bonjour"
```

> **Note :** dans un texte normal, `\n` est un retour à la ligne. Dans un texte brut,
> c'est juste une barre et un `n`.

## Erreurs fréquentes

### Mettre un &str dans une String

```rust norun
fn main() {
    let nom: String = "Léa";
    println!("{nom}");
}
```

```text
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let nom: String = "Léa";
  |              ------   ^^^^^ expected `String`, found `&str`
  |              |
  |              expected due to this
help: try using a conversion method
```

Un texte entre guillemets est un `&str`. Convertissez-le : `"Léa".to_string()` ou
`String::from("Léa")`.

### Lire une lettre avec [0]

```rust norun
fn main() {
    let mot = "bonjour";
    let premiere = mot[0];
    println!("{premiere}");
}
```

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:24
  |
3 |     let premiere = mot[0];
  |                        ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

Une lettre peut occuper plusieurs octets : Rust refuse de deviner. Écrivez
`mot.chars().next()` pour la première lettre, ou `mot.chars().nth(2)` pour la
troisième. Les deux renvoient une `Option` (voir
[le type Option](types.md#le-type-option)).

### Additionner deux &str

```rust norun
fn main() {
    let phrase = "Bon" + "jour";
    println!("{phrase}");
}
```

```text
error[E0369]: cannot add `&str` to `&str`
 --> src/main.rs:2:24
  |
2 |     let phrase = "Bon" + "jour";
  |                  ----- ^ ------ &str
  |                  |     |
  |                  |     `+` cannot be used to concatenate two `&str` strings
  |                  &str
  |
  = note: string concatenation requires an owned `String` on the left
```

À gauche du `+`, il faut une `String` : `String::from("Bon") + "jour"`. Plus simple :
`format!("{}{}", "Bon", "jour")`.

### Agrandir une String sans mut

```rust norun
fn main() {
    let phrase = String::from("Bonjour");
    phrase.push_str(" !");
    println!("{phrase}");
}
```

```text
error[E0596]: cannot borrow `phrase` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
3 |     phrase.push_str(" !");
  |     ^^^^^^ cannot borrow as mutable
```

Correction : `let mut phrase = String::from("Bonjour");`.

## Testez-vous

**1.** Que valent `"noël".len()` et `"noël".chars().count()` ?

> **Réponse**
>
> `5` et `4` : le `ë` occupe deux octets, mais c'est un seul caractère.

**2.** Comment obtenir `["a", "b", "c"]` à partir de `"a;b;c"` ?

> **Réponse**
>
> ```rust
> let morceaux: Vec<&str> = "a;b;c".split(';').collect();
> println!("{:?}", morceaux);
> ```
>
> ```sortie
> ["a", "b", "c"]
> ```

**3.** Pourquoi écrire `.trim()` avant `.parse()` sur une saisie au clavier ?

> **Réponse**
>
> La saisie se termine par un retour à la ligne. `parse` échoue sur ce caractère ;
> `trim()` l'enlève.

## En résumé

- `&str` pour regarder un texte, `String` pour le posséder et le modifier.
- `push_str`, `format!` et `join` construisent du texte.
- `len()` compte les octets, `chars().count()` compte les caractères.
- `trim`, `to_uppercase`, `replace`, `split` et `contains` transforment et cherchent.
- `parse::<i32>()` lit un nombre et renvoie un `Result`.

Comparez avec [le texte en Ruby](../ruby/texte.md) et [en Rythme](../rythme/texte.md).

Suite : [Collections](collections.md).
