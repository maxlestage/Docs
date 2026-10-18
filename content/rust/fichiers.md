# Lire et écrire : clavier et fichiers

Cette page montre comment lire ce que tape l'utilisateur, récupérer les arguments de la ligne de commande, et lire ou écrire des fichiers.

> **Objectifs :**
>
> - lire une ligne tapée au clavier et la convertir en nombre ;
> - récupérer les arguments de la ligne de commande ;
> - écrire, lire et compléter un fichier texte ;
> - gérer l'absence d'un fichier avec `Result` et `?` ;
> - lire un gros fichier ligne par ligne avec `BufReader`.

Les outils d'entrée et de sortie sont rangés dans des
[modules](../debuter/glossaire.md#module) de la bibliothèque standard : `std::io` pour
le clavier, `std::fs` pour les fichiers (*file system*), `std::env` pour
l'environnement. La ligne `use std::fs;` permet ensuite d'écrire `fs::write` au lieu de
`std::fs::write`.

## Lire au clavier

`stdin()` désigne l'**entrée standard** : en général, le clavier. Sa méthode
`read_line` attend que l'utilisateur tape une ligne et appuie sur Entrée. Le texte est
ajouté à la fin d'une `String`, qu'il faut prêter en écriture avec `&mut` :

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

Un programme lancé dans un [terminal](../debuter/glossaire.md#terminal) peut recevoir
des mots après son nom : ce sont ses **arguments**. `std::env::args()` les donne. Le
premier est toujours le nom du programme lui-même :

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

`fs::write` crée le fichier (ou l'écrase s'il existe) et y écrit le texte. Le nom
`note.txt` désigne un fichier du dossier où le programme est lancé :

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
morceau par morceau avec `BufReader`, un lecteur avec **tampon** : il charge un bloc du
fichier, vous le donne ligne par ligne, puis charge le bloc suivant. Comme un seau qu'on
remplit plusieurs fois au lieu de vider la piscine d'un coup :

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

## Erreurs fréquentes

### Lire au clavier dans une String non mut

```rust norun
use std::io;

fn main() {
    let reponse = String::new();
    io::stdin().read_line(&mut reponse).expect("lecture impossible");
}
```

```text
error[E0596]: cannot borrow `reponse` as mutable, as it is not declared as mutable
 --> src/main.rs:5:27
  |
5 |     io::stdin().read_line(&mut reponse).expect("lecture impossible");
  |                           ^^^^^^^^^^^^ cannot borrow as mutable
```

`read_line` écrit dans la `String` : déclarez-la avec `let mut reponse`.

### Oublier use std::fs

```rust norun
fn main() {
    let texte = fs::read_to_string("note.txt").unwrap();
    println!("{texte}");
}
```

```text
error[E0433]: cannot find module or crate `fs` in this scope
 --> src/main.rs:2:17
  |
2 |     let texte = fs::read_to_string("note.txt").unwrap();
  |                 ^^ use of unresolved module or unlinked crate `fs`
help: consider importing this module
  |
1 + use std::fs;
```

Ajoutez `use std::fs;` en haut du fichier.

### writeln! sans importer Write

```rust norun
use std::fs::OpenOptions;

fn main() {
    let mut fichier = OpenOptions::new().append(true).open("journal.txt").unwrap();
    writeln!(fichier, "étape 1").unwrap();
}
```

```text
error[E0599]: cannot write into `File`
 --> src/main.rs:5:14
  |
5 |     writeln!(fichier, "étape 1").unwrap();
  |              ^^^^^^^
  = help: items from traits can only be used if the trait is in scope
help: trait `Write` which provides `write_fmt` is implemented but not in scope; perhaps you want to import it
  |
1 + use std::io::Write;
```

`writeln!` a besoin du trait `Write`. Ajoutez `use std::io::Write;`.

## Testez-vous

**1.** Quelle différence entre `fs::write` et `OpenOptions::new().append(true)` ?

> **Réponse**
>
> `fs::write` remplace tout le contenu du fichier. `append(true)` ajoute à la fin, sans
> effacer ce qui existe.

**2.** Pourquoi `args.len() - 1` pour compter les arguments ?

> **Réponse**
>
> Le premier élément de `env::args()` est le nom du programme lui-même.

**3.** Écrivez un fichier `a.txt` contenant `un` puis `deux` sur deux lignes, et
comptez ses lignes.

> **Réponse**
>
> ```rust
> use std::fs;
> use std::io;
>
> fn main() -> io::Result<()> {
>     fs::write("a.txt", "un\ndeux\n")?;
>     let texte = fs::read_to_string("a.txt")?;
>     println!("{}", texte.lines().count());
>     Ok(())
> }
> ```
>
> ```sortie
> 2
> ```

## En résumé

- `stdin().read_line(&mut s)` lit une ligne au clavier ; pensez à `trim()`.
- `env::args()` donne les arguments de la ligne de commande.
- `fs::write` écrit un fichier, `fs::read_to_string` le lit en entier.
- `OpenOptions::new().append(true)` ajoute à la fin d'un fichier.
- `main` peut renvoyer `io::Result<()>` pour utiliser `?`.

Comparez avec [les fichiers en Ruby](../ruby/fichiers.md) et
[en Rythme](../rythme/fichiers.md). Les projets à plusieurs fichiers et les tests sont
dans [Projets, crates et tests](projet.md).

Suite : [Box, Rc et RefCell](pointeurs.md).
