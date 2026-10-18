# unsafe et les règles qu'on peut enfreindre

Le mot-clé `unsafe` ouvre une petite zone où Rust vous fait confiance au lieu de tout vérifier : ce chapitre montre ce qu'elle permet, quand elle est utile, et pourquoi il faut l'éviter le plus possible.

> **Objectifs :**
>
> - savoir ce que `unsafe` permet, et ce qu'il ne permet pas ;
> - créer et lire un pointeur brut ;
> - comprendre l'idée d'une abstraction sûre, avec `split_at_mut` ;
> - appeler une fonction écrite en C avec `extern "C"` ;
> - connaître la règle d'or : pas d'`unsafe` sans bonne raison, et vérifier avec Miri.

## Pourquoi unsafe existe

Le compilateur de Rust est comme un contrôleur très prudent : il refuse tout programme
dont il ne peut pas **prouver** qu'il est correct (pas de mémoire libérée deux fois, pas
de lecture d'une valeur disparue…). Ces règles de [possession](possession.md) et
d'[emprunt](../debuter/glossaire.md#emprunt) vous protègent.

Mais un contrôleur prudent refuse aussi certains programmes **corrects**, simplement
parce qu'il ne sait pas le vérifier. Et parfois, il faut parler au matériel ou à du code
écrit dans un autre langage, que Rust ne peut pas contrôler.

Un bloc `unsafe { … }` dit au compilateur : « ici, je prends la responsabilité ». C'est
comme la zone « réservée au personnel » d'une cuisine : on y trouve les couteaux les plus
tranchants. On y entre rarement, en sachant ce qu'on fait.

## Les cinq super-pouvoirs

Dans un bloc `unsafe`, vous avez le droit de faire cinq choses de plus, et **seulement**
celles-là :

| Super-pouvoir | En clair |
|---------------|----------|
| déréférencer un pointeur brut | lire ou écrire à une adresse que Rust ne vérifie pas |
| appeler une fonction `unsafe` | utiliser une fonction qui a des conditions à respecter |
| modifier une variable `static mut` | changer une variable globale partagée par tout le programme |
| implémenter un trait `unsafe` | promettre une propriété que Rust ne peut pas vérifier (`Send`, `Sync`) |
| lire le champ d'une `union` | lire une zone mémoire qui peut contenir plusieurs types |

> **À retenir :** `unsafe` n'éteint **pas** les autres vérifications. Les types, la
> possession et l'emprunt des références normales sont toujours contrôlés à l'intérieur
> du bloc.

## Les pointeurs bruts

Une référence `&T` est toujours valide : Rust le garantit. Un **pointeur brut** est une
simple adresse, sans garantie. Il en existe deux sortes :

- `*const T` : pour lire ;
- `*mut T` : pour lire et écrire.

**Créer** un pointeur brut est sans danger. Le **suivre** (avec `*`) demande `unsafe`,
car l'adresse pourrait ne plus rien contenir de valable :

```rust
let mut nombre = 5;

let lecture = &raw const nombre; // un *const i32
let ecriture = &raw mut nombre;  // un *mut i32

unsafe {
    *ecriture += 10;
    println!("valeur lue : {}", *lecture);
}
println!("nombre vaut maintenant {nombre}");
```

```sortie
valeur lue : 15
nombre vaut maintenant 15
```

Remarquez : on a ici un pointeur de lecture et un pointeur d'écriture vers la même
valeur, ce que les références `&` et `&mut` interdisent. C'est à nous de garantir que
tout se passe bien.

> **Note :** `&raw const x` et `&raw mut x` créent un pointeur brut directement. On
> voit aussi souvent `&x as *const i32`, qui fait la même chose.

## Les fonctions unsafe

Une fonction marquée `unsafe fn` a des **conditions** que le compilateur ne vérifie pas.
L'appeler demande un bloc `unsafe` : c'est votre promesse de les respecter.

La bibliothèque standard en a quelques-unes. Par exemple, `get_unchecked` lit un élément
d'une liste **sans vérifier** que la position existe :

```rust
let notes = [12, 15, 9];

let sure = notes.get(5); // vérifie : renvoie None
println!("{:?}", sure);

// SAFETY : 1 est plus petit que notes.len(), qui vaut 3.
let rapide = unsafe { notes.get_unchecked(1) };
println!("{rapide}");
```

```sortie
None
15
```

Avec une mauvaise position, `get_unchecked` lirait n'importe quoi en mémoire. C'est
pourquoi on écrit, juste au-dessus, un commentaire `// SAFETY :` qui explique pourquoi
l'appel est correct. C'est une habitude de toute la communauté Rust.

Vous pouvez écrire vos propres fonctions `unsafe`. Expliquez leurs conditions dans la
documentation, dans une section « Safety » :

```rust
/// Renvoie le premier élément sans vérifier.
///
/// # Safety
///
/// La liste ne doit pas être vide.
unsafe fn premier_sans_verifier(liste: &[i32]) -> i32 {
    // SAFETY : l'appelant a promis que la liste n'est pas vide.
    unsafe { *liste.get_unchecked(0) }
}

fn main() {
    let v = vec![7, 8, 9];
    // SAFETY : v contient trois éléments.
    let x = unsafe { premier_sans_verifier(&v) };
    println!("{x}");
}
```

```sortie
7
```

## Une abstraction sûre : split_at_mut

La bonne façon d'utiliser `unsafe` est de le **cacher** dans une petite fonction sûre :
la fonction vérifie elle-même les conditions, et ceux qui l'appellent n'ont jamais besoin
d'`unsafe`.

L'exemple classique est `split_at_mut` : elle coupe une liste en deux moitiés
modifiables. Elle est sûre à utiliser :

```rust
let mut nombres = [1, 2, 3, 4, 5, 6];
let (gauche, droite) = nombres.split_at_mut(3);
gauche[0] = 100;
droite[0] = 400;
println!("{:?}", nombres);
```

```sortie
[100, 2, 3, 400, 5, 6]
```

Essayez de l'écrire vous-même avec deux emprunts `&mut` de la même liste, et le
compilateur refuse : il ne voit pas que les deux moitiés ne se chevauchent pas. Voici
comment on l'écrit avec un peu d'`unsafe`, protégé par une vérification :

```rust
use std::slice;

fn couper(liste: &mut [i32], milieu: usize) -> (&mut [i32], &mut [i32]) {
    let longueur = liste.len();
    assert!(milieu <= longueur, "milieu trop grand"); // la vérification

    let debut = liste.as_mut_ptr(); // un *mut i32 vers le premier élément
    // SAFETY : milieu <= longueur, et les deux morceaux ne se chevauchent pas.
    unsafe {
        (
            slice::from_raw_parts_mut(debut, milieu),
            slice::from_raw_parts_mut(debut.add(milieu), longueur - milieu),
        )
    }
}

fn main() {
    let mut nombres = [1, 2, 3, 4, 5, 6];
    let (gauche, droite) = couper(&mut nombres, 2);
    gauche[1] = 20;
    droite[0] = 30;
    println!("{:?}", nombres);
}
```

```sortie
[1, 20, 30, 4, 5, 6]
```

La fonction `couper` n'est pas marquée `unsafe` : quoi qu'on lui donne, elle est
correcte, grâce à l'`assert!`. C'est le cœur de l'idée : **un petit bloc `unsafe`, bien
vérifié, enfermé dans une fonction sûre**.

## Appeler du C : extern "C"

Beaucoup de bibliothèques sont écrites en C. Rust peut les appeler. On déclare les
fonctions dans un bloc `unsafe extern "C"`. Par exemple, `abs` de la bibliothèque
standard du C, toujours présente :

```rust
unsafe extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    // SAFETY : abs accepte n'importe quel i32 (sauf le plus petit, qu'on n'utilise pas).
    let resultat = unsafe { abs(-42) };
    println!("abs(-42) = {resultat}");
}
```

```sortie
abs(-42) = 42
```

Pourquoi `unsafe` ? Parce que Rust ne peut pas lire le code C : il ne peut pas vérifier
que la déclaration est juste, ni ce que la fonction fait de la mémoire.

Pour votre propre bibliothèque C, il faut aussi dire à Cargo de la lier. Par exemple, pour
une bibliothèque `libmeteo` :

```rust norun
#[link(name = "meteo")]
unsafe extern "C" {
    fn temperature_actuelle() -> f64;
}
```

> **Astuce :** pour une vraie bibliothèque C, cherchez d'abord sur crates.io : il existe
> souvent une crate qui fait déjà l'enrobage sûr pour vous.

## La règle d'or

- **N'écrivez pas d'`unsafe`** tant que vous n'en avez pas un vrai besoin : parler à du
  C, au matériel, ou optimiser un passage mesuré comme lent (voir
  [la performance](performance.md)).
- Gardez chaque bloc `unsafe` **le plus petit possible**.
- Écrivez un commentaire `// SAFETY :` qui explique pourquoi c'est correct.
- Enfermez-le dans une fonction sûre qui vérifie les conditions.
- Faites-le vérifier par **Miri**, un outil qui exécute votre programme en surveillant
  chaque accès mémoire et signale les comportements interdits :

```sh
rustup +nightly component add miri
cargo +nightly miri run
cargo +nightly miri test
```

Dans la plupart des programmes, vous n'écrirez **jamais** `unsafe` : la bibliothèque
standard et les crates l'utilisent pour vous, derrière des fonctions sûres.

## Erreurs fréquentes

### Suivre un pointeur brut hors d'un bloc unsafe

```rust norun
let nombre = 5;
let p = &raw const nombre;
println!("{}", *p);
```

```text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> main.rs:5:16
  |
5 | println!("{}", *p);
  |                ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
```

Créer le pointeur est permis ; le suivre demande `unsafe { *p }`. Mais demandez-vous
d'abord : une simple référence `&nombre` ne suffirait-elle pas ?

### Appeler une fonction unsafe sans bloc unsafe

```rust norun
let notes = [12, 15, 9];
let x = notes.get_unchecked(1);
```

```text
error[E0133]: call to unsafe function `core::slice::<impl [T]>::get_unchecked` is unsafe and requires unsafe block
 --> main.rs:4:9
  |
4 | let x = notes.get_unchecked(1);
  |         ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

Ici, la vraie correction est d'utiliser la version sûre : `notes[1]` ou
`notes.get(1)`.

### Un bloc extern sans unsafe

```rust norun
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    println!("{}", unsafe { abs(-3) });
}
```

```text
error: extern blocks must be unsafe
 --> main.rs:2:1
  |
2 | / extern "C" {
3 | |     fn abs(x: i32) -> i32;
4 | | }
  | |_^
```

Depuis l'édition 2024 de Rust, on écrit `unsafe extern "C" { … }` : c'est vous qui
promettez que les déclarations sont justes.

## Testez-vous

1. Dans un bloc `unsafe`, le vérificateur d'emprunts est-il désactivé ?

> **Réponse**
>
> Non. `unsafe` ajoute seulement cinq possibilités (pointeurs bruts, fonctions `unsafe`,
> `static mut`, traits `unsafe`, unions). Tout le reste est vérifié comme d'habitude.

2. Créer un pointeur brut demande-t-il `unsafe` ?

> **Réponse**
>
> Non, le créer est sans danger. C'est le **suivre** (`*p`) qui demande un bloc
> `unsafe`.

3. Qu'est-ce qu'une abstraction sûre ?

> **Réponse**
>
> Une fonction normale (pas `unsafe`) qui utilise un petit bloc `unsafe` à l'intérieur,
> mais vérifie elle-même les conditions. Ceux qui l'appellent ne peuvent pas provoquer
> d'erreur mémoire, comme avec `split_at_mut`.

## En résumé

- `unsafe { … }` veut dire : « je garantis moi-même ce que Rust ne peut pas vérifier ».
- Il permet cinq choses, dont suivre un pointeur brut et appeler une fonction `unsafe`.
- Le reste des règles s'applique toujours.
- On cache l'`unsafe` dans une fonction sûre, avec un commentaire `// SAFETY :`.
- `unsafe extern "C"` permet d'appeler du code C.
- Évitez-le sauf nécessité, et faites vérifier votre code par Miri.

Suite : [Écrire un outil en ligne de commande](ligne-de-commande.md).
