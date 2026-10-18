# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, récupérer les arguments de la ligne de commande, et lire ou écrire des fichiers.

## Lire au clavier

`std::io::stdin().read_line` attend que l'utilisateur tape une ligne et appuie sur
Entrée. Le texte est ajouté dans une `String` :

```rust norun
use std::io;

fn main() {
    println!("Comment vous appelez-vous ?");
    let mut reponse = String::new();
    io::stdin().read_line(&mut reponse).expect("lecture impossible");
    let nom = reponse.trim();
    println!("Bonjour, {nom} !");
}
```

```text
Comment vous appelez-vous ?
Alice
Bonjour, Alice !
```

> **Attention :** la ligne lue contient le retour à la ligne final. Pensez à `trim()`.

Pour lire un nombre, combinez avec `parse` (voir [Travailler avec du texte](texte.md)) :

```rust norun
let mut saisie = String::new();
std::io::stdin().read_line(&mut saisie).expect("lecture impossible");
let age: u32 = saisie.trim().parse().expect("un nombre était attendu");
println!("Dans dix ans, vous aurez {} ans.", age + 10);
```

## Les arguments de la ligne de commande

`std::env::args()` donne les mots tapés après le nom du programme. Le premier est
toujours le nom du programme lui-même :

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{} argument(s) en plus", args.len() - 1);
    match args.get(1) {
        Some(nom) => println!("Bonjour, {nom} !"),
        None => println!("Aucun nom donné."),
    }
}
```

```sortie
0 argument(s) en plus
Aucun nom donné.
```

Avec cargo, les arguments se placent après `--` :

```sh
cargo run -- Alice
```

```text
1 argument(s) en plus
Bonjour, Alice !
```

## Écrire un fichier

`fs::write` crée le fichier (ou l'écrase s'il existe) et y écrit le texte :

```rust
use std::fs;

fn main() {
    fs::write("note.txt", "Acheter du pain\n").expect("écriture impossible");
    println!("Fichier écrit.");
}
```

```sortie
Fichier écrit.
```

## Lire un fichier entier

`fs::read_to_string` lit tout le fichier dans une `String` :

```rust
use std::fs;

fn main() {
    fs::write("note.txt", "Acheter du pain\nAppeler Léo\n").unwrap();
    let contenu = fs::read_to_string("note.txt").unwrap();
    print!("{contenu}");
}
```

```sortie
Acheter du pain
Appeler Léo
```

### Ligne par ligne avec lines

`lines()` découpe le texte en lignes, sans le retour à la ligne :

```rust
use std::fs;

fn main() {
    fs::write("courses.txt", "pain\nlait\nœufs\n").unwrap();
    let contenu = fs::read_to_string("courses.txt").unwrap();
    for (i, ligne) in contenu.lines().enumerate() {
        println!("{} - {ligne}", i + 1);
    }
}
```

```sortie
1 - pain
2 - lait
3 - œufs
```

## Gérer les erreurs avec ?

Lire un fichier peut échouer : il n'existe pas, ou on n'a pas le droit de le lire. Au
lieu de `unwrap`, faites renvoyer un `Result` à `main` et utilisez `?` :

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::write("bonjour.txt", "Salut !")?;
    let texte = fs::read_to_string("bonjour.txt")?;
    println!("{texte}");
    Ok(())
}
```

```sortie
Salut !
```

Si un fichier manque, le programme s'arrête proprement avec un message. Pour réagir
vous-même, utilisez `match` :

```rust
use std::fs;

fn main() {
    match fs::read_to_string("absent.txt") {
        Ok(texte) => println!("{texte}"),
        Err(_) => println!("Le fichier est introuvable."),
    }
}
```

```sortie
Le fichier est introuvable.
```

> **À retenir :** `io::Result<()>` signifie « rien en cas de succès, une erreur
> d'entrée-sortie sinon ». Tout est expliqué dans [Gérer les erreurs](erreurs.md).

## Ajouter à la fin d'un fichier

`fs::write` efface l'ancien contenu. Pour **ajouter** des lignes, ouvrez le fichier avec
`OpenOptions` et l'option `append` :

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    fs::write("journal.txt", "début\n")?;
    let mut fichier = OpenOptions::new().append(true).open("journal.txt")?;
    writeln!(fichier, "étape 1")?;
    writeln!(fichier, "étape 2")?;
    print!("{}", fs::read_to_string("journal.txt")?);
    Ok(())
}
```

```sortie
début
étape 1
étape 2
```

> **Note :** `writeln!` marche comme `println!`, mais écrit dans le fichier. Il faut
> importer `std::io::Write` pour l'utiliser.

## Les gros fichiers avec BufReader

`read_to_string` charge tout le fichier en mémoire. Pour un très gros fichier, lisez-le
morceau par morceau avec `BufReader` :

```rust
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    fs::write("nombres.txt", "10\n20\n12\n")?;
    let fichier = File::open("nombres.txt")?;
    let lecteur = BufReader::new(fichier);
    let mut total = 0;
    for ligne in lecteur.lines() {
        let ligne = ligne?;
        total += ligne.trim().parse::<i32>().unwrap_or(0);
    }
    println!("Total : {total}");
    Ok(())
}
```

```sortie
Total : 42
```

> **Astuce :** pour de petits fichiers, `read_to_string` suffit largement.

## En résumé

- `stdin().read_line(&mut s)` lit une ligne au clavier ; pensez à `trim()`.
- `env::args()` donne les arguments de la ligne de commande.
- `fs::write` écrit un fichier, `fs::read_to_string` le lit en entier.
- `OpenOptions::new().append(true)` ajoute à la fin d'un fichier.
- `main` peut renvoyer `io::Result<()>` pour utiliser `?`.

Comparez avec [les fichiers en Ruby](../ruby/fichiers.md) et
[en Rythme](../rythme/fichiers.md). Suite : [Projets, crates et tests](projet.md).
