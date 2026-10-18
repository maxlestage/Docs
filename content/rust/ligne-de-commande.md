# Écrire un outil en ligne de commande

Rust est parfait pour écrire de petits outils qu'on lance dans un terminal : ce chapitre montre comment lire les arguments, signaler les erreurs proprement et installer votre outil sur votre machine.

> **Objectifs :**
>
> - lire les arguments avec `std::env::args` ;
> - analyser des options (`-m`, `--fois 3`) à la main ;
> - renvoyer un code de sortie et écrire les erreurs sur la sortie d'erreur ;
> - lire l'entrée standard, et utiliser la crate clap ;
> - écrire et installer un outil complet : « compter les mots ».

## Les arguments du programme

Quand vous tapez `cargo run -- poeme.txt --fois 3` dans un
[terminal](../debuter/glossaire.md#terminal), les mots après `--` sont donnés à votre
programme : ce sont ses **arguments**. `std::env::args()` les fournit, sous forme de
`String`. On les range presque toujours dans un `Vec` :

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{} argument(s) en tout", args.len());
    println!("Le premier est le nom du programme.");
    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("argument {i} : {arg}");
    }
}
```

```sortie
1 argument(s) en tout
Le premier est le nom du programme.
```

Lancé avec des arguments, le même programme affiche :

```sh
cargo run -q -- poeme.txt --fois 3
```

```text
4 argument(s) en tout
Le premier est le nom du programme.
argument 1 : poeme.txt
argument 2 : --fois
argument 3 : 3
```

> **À retenir :** `args[0]` est le nom du programme. Vos vrais arguments commencent à
> `args[1]`. Le chapitre [Fichiers](fichiers.md#les-arguments-de-la-ligne-de-commande)
> en montre un premier usage.

## Analyser les options à la main

Un outil accepte souvent des **options** : `-m` (court) ou `--majuscules` (long), et
parfois une option suivie d'une valeur, comme `--fois 3`.

Pour pouvoir tester sans terminal, on écrit une fonction `analyser` qui reçoit les
arguments en paramètre. Dans `main`, on lui donnera `env::args().collect()` ; ici, on
simule plusieurs lignes de commande :

```rust
#[derive(Debug)]
struct Options {
    nom: String,
    fois: u32,
    majuscules: bool,
}

fn analyser(args: &[String]) -> Result<Options, String> {
    let mut nom = None;
    let mut fois = 1;
    let mut majuscules = false;

    let mut reste = args.iter().skip(1); // on saute le nom du programme
    while let Some(arg) = reste.next() {
        match arg.as_str() {
            "-m" | "--majuscules" => majuscules = true,
            "-f" | "--fois" => {
                let valeur = reste.next().ok_or("--fois attend un nombre")?;
                fois = valeur.parse().map_err(|_| format!("nombre invalide : {valeur}"))?;
            }
            option if option.starts_with('-') => return Err(format!("option inconnue : {option}")),
            texte => nom = Some(texte.to_string()),
        }
    }

    let nom = nom.ok_or("il manque le nom")?;
    Ok(Options { nom, fois, majuscules })
}

// fabrique un Vec<String> à partir d'une ligne, pour simuler le terminal
fn ligne(texte: &str) -> Vec<String> {
    texte.split_whitespace().map(String::from).collect()
}

fn main() {
    println!("{:?}", analyser(&ligne("salut Léa")));
    println!("{:?}", analyser(&ligne("salut -m --fois 3 Tom")));
    println!("{:?}", analyser(&ligne("salut --fois deux Tom")));
    println!("{:?}", analyser(&ligne("salut --vite Tom")));
    println!("{:?}", analyser(&ligne("salut -m")));
}
```

```sortie
Ok(Options { nom: "Léa", fois: 1, majuscules: false })
Ok(Options { nom: "Tom", fois: 3, majuscules: true })
Err("nombre invalide : deux")
Err("option inconnue : --vite")
Err("il manque le nom")
```

Quelques points à remarquer :

- `reste.next()` à l'intérieur de la boucle **avance** dans les arguments : c'est ainsi
  qu'on récupère la valeur qui suit `--fois` ;
- le `?` (voir [les erreurs](erreurs.md#propager-avec)) arrête l'analyse au premier
  problème ;
- `ok_or` transforme une `Option` en `Result`, avec un message d'erreur.

## Sortie d'erreur et code de sortie

### eprintln! : écrire sur la sortie d'erreur

Un programme a deux sorties : la **sortie standard** (les résultats, avec `println!`) et
la **sortie d'erreur** (les problèmes, avec `eprintln!`). Dans le terminal, les deux
s'affichent pareil. Mais on peut les séparer : si l'utilisateur envoie les résultats
dans un fichier, les erreurs restent visibles à l'écran.

```rust
fn main() {
    println!("résultat : 42");
    eprintln!("attention : fichier absent.txt introuvable");
    println!("résultat : 7");
}
```

```sortie
résultat : 42
résultat : 7
```

Seules les lignes de `println!` apparaissent dans la sortie standard ci-dessus. Dans un
terminal, on le voit en redirigeant la sortie standard vers un fichier avec `>` :

```sh
cargo run -q > resultats.txt
```

```text
attention : fichier absent.txt introuvable
```

Le message d'erreur s'affiche ; les deux résultats sont dans `resultats.txt`.

### Le code de sortie

Quand un programme se termine, il donne au système un **code de sortie** : `0` veut dire
« tout va bien », un autre nombre veut dire « il y a eu un problème ». Les scripts et les
autres programmes s'en servent pour savoir si tout s'est bien passé.

`std::process::exit(code)` arrête le programme **immédiatement** avec ce code :

```rust norun
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage : salut NOM");
        process::exit(2);
    }
    println!("Bonjour, {} !", args[1]);
}
```

```sh
cargo run -q
echo $?
```

```text
Usage : salut NOM
2
```

`echo $?` affiche le code de sortie de la dernière commande.

> **Astuce :** `process::exit` arrête tout, sans laisser les valeurs se nettoyer.
> Une façon plus douce est de faire renvoyer un `std::process::ExitCode` à `main` :
> `return ExitCode::from(2);`. C'est ce que fait l'outil complet plus bas.

Par convention : `0` pour un succès, `1` pour une erreur, `2` pour une mauvaise
utilisation (arguments invalides).

## Lire l'entrée standard

Un outil peut aussi lire ce qu'on lui **envoie** avec un tuyau `|`, comme beaucoup de
commandes Unix. On lit l'entrée standard ligne par ligne :

```rust norun
use std::io::{self, BufRead};

fn main() {
    let mut non_vides = 0;
    for ligne in io::stdin().lock().lines() {
        let ligne = ligne.expect("lecture impossible");
        if !ligne.trim().is_empty() {
            non_vides += 1;
        }
    }
    println!("{non_vides} lignes non vides");
}
```

```sh
printf "un\n\ndeux\ntrois\n" | cargo run -q
```

```text
3 lignes non vides
```

Sans tuyau, le programme attend que vous tapiez du texte au clavier ; terminez avec
`Ctrl+D` (ou `Ctrl+Z` puis Entrée sous Windows).

## Aller plus vite avec clap

Analyser les options à la main devient vite long : aide, valeurs par défaut, messages
d'erreur… La crate **clap** fait tout cela pour vous. Ajoutez-la dans `Cargo.toml` :

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Vous décrivez les options dans une structure, et `#[derive(Parser)]` (une
[macro](macros.md#les-macros-derive-et-les-attributs)) écrit l'analyse. Les commentaires
`///` deviennent le texte d'aide :

```rust norun
use clap::Parser;

/// Dit bonjour, une ou plusieurs fois.
#[derive(Parser)]
#[command(version)]
struct Options {
    /// Le nom de la personne à saluer
    nom: String,

    /// Nombre de fois
    #[arg(short, long, default_value_t = 1)]
    fois: u32,

    /// Écrire en majuscules
    #[arg(short, long)]
    majuscules: bool,
}

fn main() {
    let options = Options::parse();
    let mut texte = format!("Bonjour, {} !", options.nom);
    if options.majuscules {
        texte = texte.to_uppercase();
    }
    for _ in 0..options.fois {
        println!("{texte}");
    }
}
```

```sh
salut --help
salut Léa --fois 2
salut Léa --fois deux
```

```text
Dit bonjour, une ou plusieurs fois

Usage: salut [OPTIONS] <NOM>

Arguments:
  <NOM>  Le nom de la personne à saluer

Options:
  -f, --fois <FOIS>  Nombre de fois [default: 1]
  -m, --majuscules   Écrire en majuscules
  -h, --help         Print help
  -V, --version      Print version

Bonjour, Léa !
Bonjour, Léa !

error: invalid value 'deux' for '--fois <FOIS>': invalid digit found in string

For more information, try '--help'.
```

L'aide, la conversion en `u32` et le message d'erreur (avec le code de sortie 2) sont
gratuits.

## Un outil complet : compter les mots

Voici un petit outil qui, comme la commande Unix `wc`, compte les lignes, les mots et
les caractères de chaque fichier donné. Il :

- écrit les résultats avec `println!` et les problèmes avec `eprintln!` ;
- continue si un fichier manque, mais le signale par son code de sortie.

Dans cette version, `main` crée elle-même deux fichiers et simule la ligne de commande,
pour que l'exemple fonctionne partout :

```rust
use std::fs;

struct Compte {
    lignes: usize,
    mots: usize,
    caracteres: usize,
}

fn compter(texte: &str) -> Compte {
    Compte {
        lignes: texte.lines().count(),
        mots: texte.split_whitespace().count(),
        caracteres: texte.chars().count(),
    }
}

/// Fait tout le travail et renvoie le code de sortie.
fn lancer(args: &[String]) -> u8 {
    let fichiers = &args[1..];
    if fichiers.is_empty() {
        eprintln!("Usage : compter FICHIER...");
        return 2;
    }

    let mut code = 0;
    let mut total = 0;
    for nom in fichiers {
        match fs::read_to_string(nom) {
            Ok(texte) => {
                let c = compter(&texte);
                println!("{:>3} lignes {:>4} mots {:>5} caractères  {nom}", c.lignes, c.mots, c.caracteres);
                total += c.mots;
            }
            Err(erreur) => {
                eprintln!("compter : {nom} : {erreur}");
                code = 1;
            }
        }
    }
    println!("{total} mots en tout");
    code
}

fn main() {
    // préparer deux fichiers pour l'exemple
    fs::write("poeme.txt", "Le vent se lève\nil faut tenter de vivre\n").unwrap();
    fs::write("courses.txt", "pain\nlait\nœufs\n").unwrap();

    // simuler : compter poeme.txt courses.txt absent.txt
    let args: Vec<String> = ["compter", "poeme.txt", "courses.txt", "absent.txt"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let code = lancer(&args);
    println!("code de sortie : {code}");
}
```

```sortie
  2 lignes    9 mots    40 caractères  poeme.txt
  3 lignes    3 mots    15 caractères  courses.txt
12 mots en tout
code de sortie : 1
```

Dans le vrai programme, seul `main` change : il lit les vrais arguments et renvoie le
code au système avec `ExitCode` :

```rust norun
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    ExitCode::from(lancer(&args))
}
```

```sh
cargo run -q -- poeme.txt absent.txt
```

```text
  2 lignes    9 mots    40 caractères  poeme.txt
compter : absent.txt : No such file or directory (os error 2)
9 mots en tout
```

## Installer son outil

Pour utiliser votre outil comme n'importe quelle commande, sans `cargo run`, installez-le
depuis le dossier du projet :

```sh
cargo install --path .
```

Cargo compile le programme en mode optimisé et copie l'exécutable dans `~/.cargo/bin`,
un dossier déjà présent dans le chemin des commandes. Vous pouvez ensuite taper, depuis
n'importe quel dossier :

```sh
compter poeme.txt
```

Pour le désinstaller : `cargo uninstall compter`.

## Erreurs fréquentes

### Indexer directement env::args()

```rust norun
let nom = std::env::args()[1];
```

```text
error[E0608]: cannot index into a value of type `Args`
 --> main.rs:3:27
  |
3 | let nom = std::env::args()[1];
  |                           ^^^
```

`args()` donne un itérateur, pas une liste. Rangez d'abord les arguments :
`let args: Vec<String> = std::env::args().collect();` puis utilisez `args[1]`.

### Oublier que parse renvoie un Result

```rust norun
let args: Vec<String> = std::env::args().collect();
let nombre: u32 = args[1].parse();
```

```text
error[E0308]: mismatched types
 --> main.rs:4:19
  |
4 | let nombre: u32 = args[1].parse();
  |             ---   ^^^^^^^^^^^^^^^ expected `u32`, found `Result<_, _>`
  |             |
  |             expected due to this
```

Le texte pourrait ne pas être un nombre. Traitez l'erreur, par exemple avec `match`, ou
avec `map_err(…)?` comme dans `analyser`.

### Lire un argument absent

```rust norun
let args: Vec<String> = std::env::args().collect();
let nom = &args[1];
println!("Bonjour {nom}");
```

```text
thread 'main' (29481) panicked at main.rs:4:16:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Lancé sans argument, `args` ne contient que le nom du programme. Utilisez
`args.get(1)`, qui renvoie une `Option`, et affichez un message d'usage si elle vaut
`None`.

## Testez-vous

1. Que contient `args[0]` ?

> **Réponse**
>
> Le nom (ou le chemin) du programme lui-même. Les arguments de l'utilisateur
> commencent à `args[1]`.

2. Quelle est la différence entre `println!` et `eprintln!` ?

> **Réponse**
>
> `println!` écrit sur la sortie standard (les résultats), `eprintln!` sur la sortie
> d'erreur (les problèmes). On peut ainsi rediriger les résultats dans un fichier tout
> en voyant les erreurs.

3. Quel code de sortie signifie « tout s'est bien passé » ?

> **Réponse**
>
> `0`. Tout autre code signale un problème, par exemple `1` pour une erreur et `2` pour
> une mauvaise utilisation.

## En résumé

- `env::args().collect()` donne un `Vec<String>` ; `args[0]` est le programme.
- Une fonction `analyser(&[String])` se teste facilement avec des arguments simulés.
- Les résultats vont sur `println!`, les erreurs sur `eprintln!`.
- Le code de sortie (`0` = succès) se donne avec `ExitCode` ou `process::exit`.
- La crate clap écrit l'analyse des options et l'aide pour vous.
- `cargo install --path .` installe votre outil comme une vraie commande.

Suite : [JSON, CSV et TOML avec serde](formats.md).
