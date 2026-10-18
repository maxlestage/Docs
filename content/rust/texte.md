# Travailler avec du texte

Cette page montre comment créer, transformer, découper et parcourir du texte en Rust, avec les deux types `String` et `&str`.

## Deux types de texte : String et &str

Rust a deux types de texte. Pour débuter, retenez ceci :

- `&str` : un texte que vous **regardez**. Les textes entre guillemets sont des `&str`.
- `String` : un texte que vous **possédez** et que vous pouvez modifier.

```rust
let regarde: &str = "bonjour";
let possede: String = String::from("bonjour");
let autre: String = "salut".to_string();
println!("{regarde} {possede} {autre}");
```

```sortie
bonjour bonjour salut
```

Pour passer d'une `String` à un `&str`, ajoutez `&` devant :

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

`len()` compte les **octets**, pas les lettres. Une lettre accentuée prend deux octets.
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

`chars()` donne les caractères un par un :

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

## En résumé

- `&str` pour regarder un texte, `String` pour le posséder et le modifier.
- `push_str`, `format!` et `join` construisent du texte.
- `len()` compte les octets, `chars().count()` compte les caractères.
- `trim`, `to_uppercase`, `replace`, `split` et `contains` transforment et cherchent.
- `parse::<i32>()` lit un nombre et renvoie un `Result`.

Comparez avec [le texte en Ruby](../ruby/texte.md) et [en Rythme](../rythme/texte.md).
Suite : [Collections](collections.md).
