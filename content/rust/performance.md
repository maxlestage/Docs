# Écrire du code rapide

Rust est déjà rapide, mais quelques réflexes le rendent encore plus rapide : compiler en mode release, mesurer avant de changer, éviter les copies et les allocations inutiles.

> **Objectifs :**
>
> - comprendre la différence entre le mode debug et le mode release ;
> - mesurer un temps d'exécution avec `std::time::Instant` ;
> - éviter les `clone` et les allocations inutiles, réserver la place avec
>   `Vec::with_capacity` ;
> - préférer les itérateurs et les emprunts (`&str`, `&[T]`) ;
> - mesurer sérieusement avec criterion et savoir quand **ne pas** optimiser.

## Debug ou release : la première règle

`cargo run` compile en mode **debug** : la compilation est rapide, et le programme
garde plein de vérifications utiles pour trouver les bugs. Mais il n'est pas optimisé.

`cargo run --release` compile en mode **release** : la compilation est plus longue, et
le [compilateur](../debuter/glossaire.md#compilateur) optimise le code au maximum.

```sh
cargo run             # mode debug : pour développer
cargo run --release   # mode release : pour mesurer et pour livrer
```

Voici le même programme (une boucle de 100 millions de tours), lancé dans les deux
modes sur le même ordinateur :

```text
$ cargo run
total = 199999997
durée : 733.98ms

$ cargo run --release
total = 199999997
durée : 100.48ms
```

Sept fois plus rapide, sans changer une ligne. Le programme compilé se trouve alors
dans `target/release/` (voir [Organiser un projet avec Cargo](projet.md#mode-debug-et-mode-release)).

> **À retenir :** ne mesurez **jamais** la vitesse en mode debug. Et avant de dire
> « Rust est lent », vérifiez que vous avez bien écrit `--release`.

## Mesurer avec Instant

On n'optimise pas au hasard : on **mesure** d'abord. `Instant::now()` note l'heure de
départ, `elapsed()` donne le temps écoulé depuis, sous forme de `Duration` :

```rust
use std::time::Instant;

fn main() {
    let debut = Instant::now();

    let total: u64 = (1..=1_000_000).sum();

    let duree = debut.elapsed();
    println!("total = {total}");
    println!("moins d'une seconde ? {}", duree.as_secs() < 1);
}
```

```sortie
total = 500000500000
moins d'une seconde ? true
```

Dans vos programmes, affichez directement la durée avec `{:?}` : `println!("{duree:?}")`
donne par exemple `1.42ms`. Ici, on affiche un booléen parce que la durée exacte change
à chaque lancement.

> **Attention :** une seule mesure ne veut pas dire grand-chose : l'ordinateur fait
> d'autres choses en même temps. Répétez la mesure plusieurs fois, ou utilisez criterion
> (plus bas).

## Éviter les clones inutiles

`clone()` copie toutes les données : pour un gros `Vec` ou une longue `String`, cela
coûte du temps et de la mémoire. Souvent, un simple **emprunt** suffit (voir
[La possession](possession.md#emprunter-avec)).

Avant : la fonction prend la liste, alors l'appelant doit la cloner pour la garder.

```rust
fn plus_long(mots: Vec<String>) -> usize {
    mots.iter().map(|m| m.len()).max().unwrap_or(0)
}

fn main() {
    let mots = vec![String::from("chat"), String::from("éléphant")];
    println!("{}", plus_long(mots.clone())); // copie de toute la liste
    println!("{} mots", mots.len());
}
```

```sortie
10
2 mots
```

Après : la fonction **emprunte** la liste. Aucune copie :

```rust
fn plus_long(mots: &[String]) -> usize {
    mots.iter().map(|m| m.len()).max().unwrap_or(0)
}

fn main() {
    let mots = vec![String::from("chat"), String::from("éléphant")];
    println!("{}", plus_long(&mots));
    println!("{} mots", mots.len());
}
```

```sortie
10
2 mots
```

(« éléphant » fait 10 octets : chaque `é` en prend deux.)

## Réserver la place : Vec::with_capacity

Un `Vec` range ses éléments dans un bloc de mémoire. Quand ce bloc est plein, il en
**alloue** un plus grand et y recopie tout. C'est comme déménager à chaque naissance
dans la famille : ça marche, mais c'est du travail.

`capacity()` donne la place réservée. Comptons les déménagements pour 1 000 éléments :

```rust
let mut v = Vec::new();
let mut demenagements = 0;
for i in 0..1000 {
    let avant = v.capacity();
    v.push(i);
    if v.capacity() != avant {
        demenagements += 1;
    }
}
println!("Vec::new : {demenagements} agrandissements");

let mut w = Vec::with_capacity(1000);
let avant = w.capacity();
for i in 0..1000 {
    w.push(i);
}
println!("with_capacity : capacité {} → {}", avant, w.capacity());
```

```sortie
Vec::new : 9 agrandissements
with_capacity : capacité 1000 → 1000
```

Quand vous connaissez la taille à l'avance, `Vec::with_capacity(n)` réserve la place
**une seule fois**. Même chose pour le texte : `String::with_capacity(n)`.

> **Astuce :** `collect()` sur un itérateur de taille connue (`map` sur un `Vec`, par
> exemple) réserve déjà la bonne place tout seul.

## Les itérateurs plutôt que les indices

Avec un indice, `v[i]` vérifie à chaque tour que `i` ne dépasse pas la fin. Avec un
[itérateur](../debuter/glossaire.md#iterateur), Rust sait que tout est dans les limites
et supprime ces vérifications. Le code est aussi plus court :

```rust
let notes = vec![12, 15, 9, 18];

// Avec un indice
let mut total = 0;
for i in 0..notes.len() {
    total += notes[i];
}
println!("{total}");

// Avec un itérateur : plus clair, et au moins aussi rapide
let total: i32 = notes.iter().sum();
println!("{total}");
```

```sortie
54
54
```

Les itérateurs sont des « abstractions à coût zéro » : une chaîne
`iter().filter().map().sum()` donne en release un code aussi rapide qu'une boucle
écrite à la main (voir [Les itérateurs](iterateurs.md)).

## &str plutôt que String en paramètre

Une fonction qui lit seulement un texte doit prendre un `&str`. Elle accepte alors une
`String` empruntée **et** un texte écrit entre guillemets, sans aucune allocation :

```rust
fn compter_voyelles(texte: &str) -> usize {
    texte.chars().filter(|c| "aeiouy".contains(*c)).count()
}

fn main() {
    let nom = String::from("bonjour");
    println!("{}", compter_voyelles(&nom));    // une String empruntée
    println!("{}", compter_voyelles("rustacé")); // un &str, sans .to_string()
}
```

```sortie
3
2
```

Avec `texte: String`, le second appel aurait demandé `"rustacé".to_string()` : une
allocation pour rien. Même idée pour les listes : `&[T]` plutôt que `Vec<T>`.

## Mesurer sérieusement : criterion

Pour comparer deux versions d'une fonction, la crate **criterion** lance le code des
milliers de fois et calcule une moyenne fiable. On l'ajoute comme dépendance de
développement (`cargo add --dev criterion`), et on déclare le benchmark dans
`Cargo.toml` :

```toml
[dev-dependencies]
criterion = "0.8"

[[bench]]
name = "fibo"
harness = false
```

Le benchmark va dans `benches/fibo.rs` et appelle une fonction de votre bibliothèque
(`src/lib.rs`) :

```rust norun
use calcul::fibonacci;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

`black_box` empêche le compilateur de calculer le résultat à l'avance (il est très
malin). Lancez `cargo bench` :

```text
Benchmarking fibonacci 20: Collecting 100 samples in estimated 5.0015 s (187k iterations)
fibonacci 20            time:   [25.623 µs 26.045 µs 26.553 µs]
```

Le nombre du milieu est la meilleure estimation (26 microsecondes), les deux autres
encadrent l'incertitude. Au lancement suivant, criterion indique si le code est devenu
plus rapide ou plus lent.

## Trouver ce qui est lent : le profilage

Un **profileur** observe le programme pendant qu'il tourne et dit dans quelles
fonctions il passe son temps. Sous Linux, `perf` est le classique ; `cargo flamegraph`
(à installer avec `cargo install flamegraph`) dessine un graphique où les fonctions les
plus larges sont les plus coûteuses. Sous macOS, Instruments fait le même travail.

Le profileur évite de deviner : très souvent, la partie lente n'est pas celle que l'on
croyait.

## Ne pas optimiser trop tôt

Le célèbre informaticien Donald Knuth disait : « l'optimisation prématurée est la
source de tous les maux ». Un code compliqué pour gagner une microseconde est plus dur
à lire, à tester et à corriger.

La bonne méthode, dans l'ordre :

1. écrire un code **simple et juste**, avec des [tests](tests.md) ;
2. compiler en `--release` ;
3. si c'est trop lent, **mesurer** pour trouver la partie lente ;
4. optimiser cette partie seulement, puis mesurer à nouveau.

> **Astuce :** le plus gros gain vient presque toujours d'un meilleur
> [algorithme](../debuter/glossaire.md#algorithme) (chercher dans une `HashMap` plutôt
> que parcourir une liste, par exemple), pas des petites astuces.

## Erreurs fréquentes

### Mesurer en mode debug

```sh
cargo run      # puis on conclut que le programme est lent
```

```text
total = 199999997
durée : 733.98ms
```

En debug, le code n'est pas optimisé : ces mesures ne valent rien. Utilisez
`cargo run --release` (ici, 100 ms au lieu de 734).

### Enlever un clone sans emprunter

On retire le `clone` pour gagner du temps, mais la fonction prend toujours le `Vec` :

```rust norun
fn somme(nombres: Vec<i64>) -> i64 {
    nombres.iter().sum()
}

fn main() {
    let nombres = vec![1, 2, 3];
    println!("{}", somme(nombres));
    println!("{}", nombres.len());
}
```

```text
error[E0382]: borrow of moved value: `nombres`
 --> src/main.rs:8:20
  |
6 |     let nombres = vec![1, 2, 3];
  |         ------- move occurs because `nombres` has type `Vec<i64>`, which does not implement the `Copy` trait
7 |     println!("{}", somme(nombres));
  |                          ------- value moved here
8 |     println!("{}", nombres.len());
  |                    ^^^^^^^ value borrowed here after move
```

Le `Vec` a été **donné** à la fonction. Faites-la emprunter :
`fn somme(nombres: &[i64])`, et appelez `somme(&nombres)`.

### Remplir un with_capacity avec des indices

```rust norun
fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v[0] = 1;
}
```

```text
thread 'main' (300) panicked at src/main.rs:3:6:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`with_capacity` réserve de la **place**, mais le vecteur reste **vide** (sa longueur
vaut 0). Ajoutez les éléments avec `push`, ou créez un vecteur rempli avec
`vec![0; 10]`.

## Testez-vous

1. Pourquoi faut-il mesurer en mode release ?

> **Réponse**
>
> Parce qu'en mode debug le code n'est pas optimisé : il peut être plusieurs fois plus
> lent. Les mesures ne reflètent pas la vitesse réelle du programme.

2. Quelle est la différence entre `Vec::with_capacity(100)` et `vec![0; 100]` ?

> **Réponse**
>
> `with_capacity(100)` réserve la place pour 100 éléments mais le vecteur est vide.
> `vec![0; 100]` contient vraiment 100 zéros.

3. Pourquoi écrire `fn f(texte: &str)` plutôt que `fn f(texte: String)` ?

> **Réponse**
>
> La fonction emprunte au lieu de prendre possession : l'appelant n'a pas besoin de
> cloner, et on peut lui passer un texte littéral sans allocation.

## En résumé

- Toujours mesurer et livrer en `cargo run --release`.
- `Instant::now()` et `elapsed()` mesurent un temps ; criterion fait des mesures
  fiables avec `cargo bench`.
- Empruntez (`&str`, `&[T]`) plutôt que cloner ; réservez avec `with_capacity`.
- Les itérateurs sont aussi rapides qu'une boucle à la main, et plus clairs.
- Mesurer d'abord, optimiser ensuite, et seulement ce qui est lent.

Suite : [Tester son code](tests.md)
