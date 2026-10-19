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

Presque tous les programmes manipulent du texte : un prénom, un message d'accueil,
une liste de courses. Bien choisir son type vous évitera beaucoup d'erreurs du
compilateur.

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

Ligne par ligne :

- `regarde` est un `&str` : il regarde le texte `"bonjour"` écrit dans le programme ;
- `possede` est une `String` fabriquée avec `String::from` : une copie qui lui
  appartient ;
- `autre` est aussi une `String`, fabriquée cette fois avec `.to_string()` ;
- `println!` affiche les trois : à l'écran, rien ne les distingue.

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

Un texte se construit souvent petit à petit, comme une liste de courses que vous
complétez au fil de la semaine. Pour cela, il faut une `String` modifiable.

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

Ligne par ligne :

- `let mut phrase` crée une `String` qui contient « Bonjour ». `mut` lui permet de
  changer ;
- `push_str(" tout le monde")` colle ce texte à la fin : `phrase` vaut maintenant
  « Bonjour tout le monde » ;
- `push('!')` ajoute un seul caractère. Un caractère seul s'écrit entre apostrophes ;
- `println!` affiche le résultat final.

L'opérateur `+` colle aussi deux textes, mais à gauche il faut une `String` :

```rust
let debut = String::from("Bon");
let mot = debut + "jour";
println!("{mot}");
```

```sortie
Bonjour
```

**À vous :** écrivez un programme qui part de la `String` « Il fait », lui ajoute
« beau » avec `push_str`, puis un point avec `push`, et affiche le résultat.

> **Solution**
>
> ```rust
> let mut meteo = String::from("Il fait");
> meteo.push_str(" beau");
> meteo.push('.');
> println!("{meteo}");
> ```
>
> ```sortie
> Il fait beau.
> ```

## Assembler avec format!

Un programme a souvent besoin de fabriquer une phrase avec des valeurs : une
étiquette, un message de bienvenue, une ligne de facture. `format!` la prépare sans
l'afficher tout de suite.

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

Ligne par ligne :

- `prenom` contient le texte « Alice », et `age` le nombre 30 ;
- `format!` remplace `{prenom}` et `{age}` par leurs valeurs, et renvoie la `String`
  « Alice (30 ans) » ;
- cette `String` est rangée dans `fiche` : rien n'est encore affiché ;
- c'est `println!` qui l'affiche enfin à l'écran.

> **Astuce :** `format!` est le moyen le plus simple de coller plusieurs morceaux de
> texte et de nombres.

## La longueur d'un texte

Compter les lettres sert souvent : vérifier qu'un mot de passe est assez long, ou
qu'un pseudo n'est pas trop long. En Rust, il y a un petit piège.

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

Ligne par ligne :

- `mot` contient « été » : trois lettres, dont deux accentuées ;
- `mot.len()` compte les octets : 2 pour `é`, 1 pour `t`, 2 pour `é`, soit 5 ;
- `mot.chars().count()` compte les caractères : 3.

> **Attention :** pour un texte en français, préférez `chars().count()`.

## Changer la casse et nettoyer

Ce que tape une personne est rarement propre : des espaces en trop, des majuscules
au hasard. Avant de comparer ou de ranger un texte, on le nettoie.

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

**À vous :** écrivez un programme qui part du texte `"   alice "`, enlève les
espaces autour, le met en majuscules et l'affiche entre crochets.

> **Solution**
>
> ```rust
> let saisie = "   alice ";
> println!("[{}]", saisie.trim().to_uppercase());
> ```
>
> ```sortie
> [ALICE]
> ```

## Chercher dans un texte

Un programme doit souvent vérifier un texte : une adresse contient-elle un `@` ? Un
nom de fichier finit-il par `.txt` ? C'est comme la recherche de votre traitement de
texte.

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

Remplacer un mot par un autre sert à corriger une faute partout, ou à remplir un
modèle de lettre avec le bon prénom.

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

Un texte contient souvent plusieurs informations collées : des fruits séparés par
des virgules, une phrase faite de mots. Pour les traiter une par une, il faut d'abord
les séparer, comme on casse une tablette de chocolat en carrés.

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

Ligne par ligne :

- `liste` contient trois fruits séparés par des virgules ;
- `split(',')` coupe à chaque virgule : on obtient « pomme », « poire » et « kiwi » ;
- `collect()` range ces morceaux dans `fruits`, un `Vec<&str>` (une liste de textes) ;
- `{:?}` affiche la liste avec ses crochets, et `len()` donne le nombre de morceaux.

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

**À vous :** écrivez un programme qui découpe le texte `"lundi mardi mercredi"` en
mots, puis les recolle avec `" / "` entre eux.

> **Solution**
>
> ```rust
> let jours: Vec<&str> = "lundi mardi mercredi".split_whitespace().collect();
> println!("{}", jours.join(" / "));
> ```
>
> ```sortie
> lundi / mardi / mercredi
> ```

## Transformer un texte en nombre

Pour Rust, `"42"` n'est pas un nombre : c'est un texte de deux caractères, avec
lequel on ne peut pas calculer. Or ce que vous tapez au clavier arrive toujours sous
forme de texte. Il faut donc le convertir.

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

Ligne par ligne :

- `"42".parse::<i32>()` réussit : le résultat est `Ok(42)`, le nombre dans une boîte
  « réussite » ;
- `"abc".parse::<i32>()` échoue : `is_err()` répond `true`, « oui, c'est une erreur » ;
- l'avant-dernière ligne enlève les espaces avec `trim()`, lit 7, et `unwrap()` sort
  le nombre de sa boîte `Ok` ;
- `n` vaut donc le nombre 7, et `n * 2` donne 14.

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

**À vous :** écrivez un programme qui lit les nombres des textes `"12"` et `"30"`
avec `parse`, puis affiche leur somme.

> **Solution**
>
> ```rust
> let a = "12".parse::<i32>().unwrap();
> let b = "30".parse::<i32>().unwrap();
> println!("{}", a + b);
> ```
>
> ```sortie
> 42
> ```

## Parcourir les caractères

Parfois, il faut examiner un texte lettre par lettre : compter les voyelles,
vérifier qu'un mot de passe contient un chiffre, ou retourner un mot.

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

Ligne par ligne :

- `for c in mot.chars()` donne à `c` les lettres « N », « o », « ë », puis « l » ;
- `print!` (sans `ln`) affiche chaque lettre suivie d'un point, sur la même ligne ;
- `println!()`, vide, passe simplement à la ligne ;
- `filter` garde les lettres présentes dans `"aeiouyë"`, et `count` les compte : « o »
  et « ë », donc 2. La partie `|c| …` est une petite fonction sans nom, expliquée
  dans [les closures](closures.md).

Pour retourner un texte, combinez `chars`, `rev` et `collect` :

```rust
let envers: String = "radar".chars().rev().collect();
println!("{envers}");
```

```sortie
radar
```

## Prendre un morceau avec prudence

Prendre un morceau d'un texte sert à n'en garder qu'une partie : le jour dans
« lundi matin », l'année dans une date. Il faut le faire avec soin, à cause des
accents.

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Certains textes contiennent beaucoup de `\` ou de guillemets : un chemin de fichier
Windows, une citation. Un texte brut les garde tels quels, sans rien à échapper.

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
