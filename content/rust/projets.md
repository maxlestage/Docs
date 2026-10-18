# Projets pas à pas

Trois petits programmes complets, construits étape par étape, pour assembler tout ce que vous avez appris : un jeu, une liste de tâches enregistrée dans un fichier et un bulletin de notes.

> **Objectifs :**
>
> - construire un programme petit à petit, en le testant à chaque étape ;
> - combiner `match`, les boucles, les structures et les itérateurs ;
> - lire et écrire un fichier texte ;
> - analyser un texte au format CSV et afficher un tableau bien aligné.

## Comment lire cette page

Chaque projet avance par **étapes**. Chaque étape est un programme complet : vous pouvez
le copier dans `src/main.rs` d'un projet créé avec `cargo new`, puis le lancer avec
`cargo run`. La sortie affichée en dessous est exactement celle que vous obtiendrez.

> **Astuce :** tapez le code vous-même plutôt que de le copier. Faites des erreurs
> exprès, et lisez les messages du compilateur : c'est comme ça qu'on apprend.

Pour que les exemples donnent toujours le même résultat, ils n'attendent rien au
clavier : les réponses du joueur sont écrites dans une liste. La version « vraie », qui
lit le clavier, est montrée à la fin de chaque projet.

## Projet 1 : Le juste nombre

L'ordinateur choisit un nombre entre 1 et 100. Le joueur propose des nombres, et
l'ordinateur répond « C'est plus ! » ou « C'est moins ! » jusqu'à ce qu'il trouve.

### Étape 1 : comparer deux nombres

La méthode `cmp` compare deux nombres et renvoie une valeur de l'énumération
`Ordering` : `Less` (plus petit), `Greater` (plus grand) ou `Equal` (égal). Un `match`
choisit la réponse :

```rust
use std::cmp::Ordering;

fn indice(essai: u32, secret: u32) -> &'static str {
    match essai.cmp(&secret) {
        Ordering::Less => "C'est plus !",
        Ordering::Greater => "C'est moins !",
        Ordering::Equal => "Gagné !",
    }
}

fn main() {
    let secret = 42;
    println!("50 → {}", indice(50, secret));
    println!("20 → {}", indice(20, secret));
    println!("42 → {}", indice(42, secret));
}
```

```sortie
50 → C'est moins !
20 → C'est plus !
42 → Gagné !
```

### Étape 2 : jouer plusieurs coups

On parcourt les essais du joueur avec une boucle `for`, on compte les coups, et on
s'arrête (`return`) dès que le nombre est trouvé. La fonction renvoie une `Option` :
`Some(coups)` si le joueur a gagné, `None` sinon.

```rust
use std::cmp::Ordering;

fn jouer(secret: u32, essais: &[u32]) -> Option<u32> {
    let mut coups = 0;
    for &essai in essais {
        coups += 1;
        match essai.cmp(&secret) {
            Ordering::Less => println!("{essai} : c'est plus !"),
            Ordering::Greater => println!("{essai} : c'est moins !"),
            Ordering::Equal => {
                println!("{essai} : gagné !");
                return Some(coups);
            }
        }
    }
    None // plus d'essais : perdu
}

fn main() {
    let essais = [50, 25, 37, 43, 40, 42, 10];
    match jouer(42, &essais) {
        Some(coups) => println!("Trouvé en {coups} coups."),
        None => println!("Perdu !"),
    }
}
```

```sortie
50 : c'est moins !
25 : c'est plus !
37 : c'est plus !
43 : c'est moins !
40 : c'est plus !
42 : gagné !
Trouvé en 6 coups.
```

Le `10` n'est jamais joué : la partie s'est arrêtée avant.

### Étape 3 : vérifier ce que tape le joueur

Un vrai joueur tape du **texte**, et peut se tromper : des lettres, un nombre trop
grand, des espaces… La fonction `lire_essai` nettoie le texte avec `trim`, le convertit
avec `parse`, et renvoie une [erreur](erreurs.md) claire si quelque chose ne va pas.
Les essais invalides ne comptent pas comme des coups.

```rust
use std::cmp::Ordering;

fn lire_essai(texte: &str) -> Result<u32, String> {
    let nombre: u32 = texte
        .trim()
        .parse()
        .map_err(|_| format!("« {} » n'est pas un nombre", texte.trim()))?;
    if nombre < 1 || nombre > 100 {
        return Err(format!("{nombre} n'est pas entre 1 et 100"));
    }
    Ok(nombre)
}

fn main() {
    let secret = 42;
    let saisies = ["50", "abc", "150", " 30 ", "42"];
    let mut coups = 0;

    for saisie in saisies {
        let essai = match lire_essai(saisie) {
            Ok(n) => n,
            Err(message) => {
                println!("Oups : {message}");
                continue; // on passe à la saisie suivante
            }
        };
        coups += 1;
        match essai.cmp(&secret) {
            Ordering::Less => println!("{essai} : c'est plus !"),
            Ordering::Greater => println!("{essai} : c'est moins !"),
            Ordering::Equal => {
                println!("{essai} : gagné en {coups} coups !");
                break;
            }
        }
    }
}
```

```sortie
50 : c'est moins !
Oups : « abc » n'est pas un nombre
Oups : 150 n'est pas entre 1 et 100
30 : c'est plus !
42 : gagné en 3 coups !
```

### La version au clavier

Voici le jeu complet, qui lit le clavier avec `read_line` (voir
[clavier et fichiers](fichiers.md)). Pour choisir un nombre « au hasard » sans
bibliothèque, on utilise les nanosecondes de l'horloge :

```rust norun
use std::cmp::Ordering;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn lire_essai(texte: &str) -> Result<u32, String> {
    let nombre: u32 = texte
        .trim()
        .parse()
        .map_err(|_| format!("« {} » n'est pas un nombre", texte.trim()))?;
    if nombre < 1 || nombre > 100 {
        return Err(format!("{nombre} n'est pas entre 1 et 100"));
    }
    Ok(nombre)
}

fn main() {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    let secret = nanos % 100 + 1;
    let mut coups = 0;
    println!("J'ai choisi un nombre entre 1 et 100. À vous !");

    loop {
        let mut saisie = String::new();
        let lus = io::stdin().read_line(&mut saisie).expect("lecture impossible");
        if lus == 0 {
            break; // plus rien à lire (Ctrl+D) : on arrête
        }
        let essai = match lire_essai(&saisie) {
            Ok(n) => n,
            Err(message) => {
                println!("Oups : {message}");
                continue;
            }
        };
        coups += 1;
        match essai.cmp(&secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Gagné en {coups} coups !");
                break;
            }
        }
    }
}
```

> **Note :** pour un vrai hasard, les programmes Rust utilisent la crate `rand`
> (voir [Modules et crates](modules.md#ajouter-une-crate-de-crates-io)).

### Pour aller plus loin

- Limitez la partie à 7 coups, et affichez le nombre secret si le joueur perd.
- Proposez de rejouer à la fin de la partie (« Rejouer ? o/n »).
- Gardez le meilleur score (le plus petit nombre de coups) dans une variable.
- Ajoutez un niveau de difficulté : 1 à 10, 1 à 100 ou 1 à 1000.

## Projet 2 : Une liste de tâches

Un petit gestionnaire de tâches : on ajoute des tâches, on les affiche, on les coche
quand elles sont faites. La liste est enregistrée dans un fichier texte, pour la
retrouver la fois suivante.

### Étape 1 : une structure et un affichage

Une tâche a un texte et un état (faite ou non). On range les tâches dans un `Vec`, et
on les affiche numérotées avec `enumerate` :

```rust
struct Tache {
    texte: String,
    faite: bool,
}

fn afficher(taches: &[Tache]) {
    if taches.is_empty() {
        println!("(aucune tâche)");
        return;
    }
    for (i, tache) in taches.iter().enumerate() {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        println!("{}. {case} {}", i + 1, tache.texte);
    }
}

fn main() {
    let mut taches = Vec::new();
    afficher(&taches);

    taches.push(Tache { texte: String::from("Acheter du pain"), faite: false });
    taches.push(Tache { texte: String::from("Réviser Rust"), faite: true });
    afficher(&taches);
}
```

```sortie
(aucune tâche)
1. [ ] Acheter du pain
2. [x] Réviser Rust
```

### Étape 2 : ajouter et cocher

Deux fonctions modifient la liste : elles reçoivent un emprunt modifiable
`&mut Vec<Tache>`. Pour cocher, le joueur donne un **numéro** qui commence à 1, alors
que les positions d'un `Vec` commencent à 0 : on retire donc 1. Un numéro invalide
donne une erreur, au lieu d'arrêter le programme.

```rust
struct Tache {
    texte: String,
    faite: bool,
}

fn afficher(taches: &[Tache]) {
    for (i, tache) in taches.iter().enumerate() {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        println!("{}. {case} {}", i + 1, tache.texte);
    }
}

fn ajouter(taches: &mut Vec<Tache>, texte: &str) {
    taches.push(Tache { texte: texte.to_string(), faite: false });
}

fn terminer(taches: &mut Vec<Tache>, numero: usize) -> Result<(), String> {
    if numero == 0 || numero > taches.len() {
        return Err(format!("pas de tâche numéro {numero}"));
    }
    taches[numero - 1].faite = true;
    Ok(())
}

fn main() {
    let mut taches = Vec::new();
    ajouter(&mut taches, "Acheter du pain");
    ajouter(&mut taches, "Réviser Rust");
    ajouter(&mut taches, "Appeler Mamie");

    terminer(&mut taches, 2).unwrap();
    if let Err(message) = terminer(&mut taches, 9) {
        println!("Erreur : {message}");
    }
    afficher(&taches);
}
```

```sortie
Erreur : pas de tâche numéro 9
1. [ ] Acheter du pain
2. [x] Réviser Rust
3. [ ] Appeler Mamie
```

### Étape 3 : enregistrer dans un fichier

On enregistre chaque tâche sur une ligne du fichier `taches.txt`, avec la même case
qu'à l'écran :

```text
[ ] Acheter du pain
[x] Réviser Rust
```

- `sauvegarder` fabrique ce texte et l'écrit avec `fs::write` ;
- `charger` lit le fichier avec `fs::read_to_string`. Si le fichier n'existe pas encore
  (premier lancement), on part d'une liste vide. `strip_prefix` enlève le début de la
  ligne (`"[x] "`) et donne le reste : le texte de la tâche.

```rust
use std::fs;
use std::io;

struct Tache {
    texte: String,
    faite: bool,
}

fn afficher(taches: &[Tache]) {
    if taches.is_empty() {
        println!("(aucune tâche)");
    }
    for (i, tache) in taches.iter().enumerate() {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        println!("{}. {case} {}", i + 1, tache.texte);
    }
}

fn sauvegarder(taches: &[Tache], chemin: &str) -> io::Result<()> {
    let mut texte = String::new();
    for tache in taches {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        texte.push_str(&format!("{case} {}\n", tache.texte));
    }
    fs::write(chemin, texte)
}

fn charger(chemin: &str) -> io::Result<Vec<Tache>> {
    let texte = match fs::read_to_string(chemin) {
        Ok(texte) => texte,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut taches = Vec::new();
    for ligne in texte.lines() {
        if let Some(reste) = ligne.strip_prefix("[x] ") {
            taches.push(Tache { texte: reste.to_string(), faite: true });
        } else if let Some(reste) = ligne.strip_prefix("[ ] ") {
            taches.push(Tache { texte: reste.to_string(), faite: false });
        }
    }
    Ok(taches)
}

fn main() -> io::Result<()> {
    println!("Premier lancement :");
    let mut taches = charger("taches.txt")?;
    afficher(&taches);

    taches.push(Tache { texte: String::from("Acheter du pain"), faite: false });
    taches.push(Tache { texte: String::from("Réviser Rust"), faite: true });
    sauvegarder(&taches, "taches.txt")?;

    println!("Contenu du fichier :");
    print!("{}", fs::read_to_string("taches.txt")?);

    println!("Lancement suivant :");
    let relues = charger("taches.txt")?;
    afficher(&relues);
    Ok(())
}
```

```sortie
Premier lancement :
(aucune tâche)
Contenu du fichier :
[ ] Acheter du pain
[x] Réviser Rust
Lancement suivant :
1. [ ] Acheter du pain
2. [x] Réviser Rust
```

> **À retenir :** `main` renvoie ici un `io::Result<()>`, ce qui permet d'utiliser `?`
> sur les opérations de fichier. Voir [Gérer les erreurs](erreurs.md).

### Étape 4 : des commandes

Il reste à comprendre des **commandes** comme `ajouter Acheter du pain`, `lister` ou
`faire 1`. `split_once(' ')` coupe la ligne au premier espace : le premier morceau est
la commande, le second son argument. Chaque commande charge la liste, agit, puis
enregistre : exactement comme si on lançait le programme plusieurs fois.

```rust
use std::fs;
use std::io;

struct Tache {
    texte: String,
    faite: bool,
}

fn afficher(taches: &[Tache]) {
    if taches.is_empty() {
        println!("(aucune tâche)");
    }
    for (i, tache) in taches.iter().enumerate() {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        println!("{}. {case} {}", i + 1, tache.texte);
    }
}

fn sauvegarder(taches: &[Tache], chemin: &str) -> io::Result<()> {
    let mut texte = String::new();
    for tache in taches {
        let case = if tache.faite { "[x]" } else { "[ ]" };
        texte.push_str(&format!("{case} {}\n", tache.texte));
    }
    fs::write(chemin, texte)
}

fn charger(chemin: &str) -> io::Result<Vec<Tache>> {
    let texte = match fs::read_to_string(chemin) {
        Ok(texte) => texte,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut taches = Vec::new();
    for ligne in texte.lines() {
        if let Some(reste) = ligne.strip_prefix("[x] ") {
            taches.push(Tache { texte: reste.to_string(), faite: true });
        } else if let Some(reste) = ligne.strip_prefix("[ ] ") {
            taches.push(Tache { texte: reste.to_string(), faite: false });
        }
    }
    Ok(taches)
}

fn executer(ligne: &str, chemin: &str) -> Result<(), String> {
    let mut taches = charger(chemin).map_err(|e| e.to_string())?;
    let (commande, argument) = ligne.split_once(' ').unwrap_or((ligne, ""));

    match commande {
        "ajouter" => {
            taches.push(Tache { texte: argument.to_string(), faite: false });
            println!("Ajouté : {argument}");
        }
        "lister" => afficher(&taches),
        "faire" => {
            let numero: usize = argument
                .parse()
                .map_err(|_| format!("« {argument} » n'est pas un numéro"))?;
            if numero == 0 || numero > taches.len() {
                return Err(format!("pas de tâche numéro {numero}"));
            }
            taches[numero - 1].faite = true;
            println!("Bravo, « {} » est faite !", taches[numero - 1].texte);
        }
        _ => return Err(format!("commande inconnue : {commande}")),
    }

    sauvegarder(&taches, chemin).map_err(|e| e.to_string())
}

fn main() {
    let commandes = [
        "ajouter Acheter du pain",
        "ajouter Réviser Rust",
        "lister",
        "faire 2",
        "faire 7",
        "dormir",
        "lister",
    ];
    for ligne in commandes {
        println!("> {ligne}");
        if let Err(message) = executer(ligne, "taches.txt") {
            println!("Erreur : {message}");
        }
    }
}
```

```sortie
> ajouter Acheter du pain
Ajouté : Acheter du pain
> ajouter Réviser Rust
Ajouté : Réviser Rust
> lister
1. [ ] Acheter du pain
2. [ ] Réviser Rust
> faire 2
Bravo, « Réviser Rust » est faite !
> faire 7
Erreur : pas de tâche numéro 7
> dormir
Erreur : commande inconnue : dormir
> lister
1. [ ] Acheter du pain
2. [x] Réviser Rust
```

### La version en ligne de commande

Pour un vrai outil, la commande vient des arguments du terminal
(`cargo run -- ajouter Acheter du pain`). Il suffit de remplacer `main` par celui-ci,
qui recolle les arguments avec des espaces :

```rust norun
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let ligne = arguments.join(" ");
    if let Err(message) = executer(&ligne, "taches.txt") {
        println!("Erreur : {message}");
    }
}
```

```sh
cargo run -- ajouter Acheter du pain
cargo run -- faire 1
cargo run -- lister
```

### Pour aller plus loin

- Ajoutez une commande `supprimer 2`, avec `taches.remove(numero - 1)`.
- Ajoutez une commande `nettoyer` qui enlève toutes les tâches faites (avec `retain`).
- Affichez à la fin de `lister` un résumé : « 1 tâche faite sur 3 ».
- Écrivez des [tests](tests.md) pour `charger` et `sauvegarder`.

## Projet 3 : Le bulletin de notes

On reçoit les notes d'une classe au format **CSV** : un texte où chaque ligne est un
élève, et où les valeurs sont séparées par des virgules. On veut la moyenne de chaque
élève, le meilleur élève, et un joli tableau.

```text
nom,maths,français,histoire
Léa,15,12.5,17
Tom,9,14,11
Inès,18,16,15.5
Hugo,12,8.5,13
```

### Étape 1 : lire le CSV

On découpe le texte en lignes avec `lines()`, on saute la ligne de titres avec
`skip(1)`, puis on découpe chaque ligne avec `split(',')`. Le premier morceau est le nom,
les autres sont les notes, converties en `f64` :

```rust
const CSV: &str = "nom,maths,français,histoire
Léa,15,12.5,17
Tom,9,14,11
Inès,18,16,15.5
Hugo,12,8.5,13";

#[derive(Debug)]
struct Eleve {
    nom: String,
    notes: Vec<f64>,
}

fn lire_eleve(ligne: &str) -> Result<Eleve, String> {
    let mut morceaux = ligne.split(',');
    let nom = morceaux.next().ok_or("ligne vide")?.trim().to_string();
    let mut notes = Vec::new();
    for morceau in morceaux {
        let note: f64 = morceau
            .trim()
            .parse()
            .map_err(|_| format!("note invalide pour {nom} : « {morceau} »"))?;
        notes.push(note);
    }
    Ok(Eleve { nom, notes })
}

fn main() {
    let mut eleves = Vec::new();
    for ligne in CSV.lines().skip(1) {
        match lire_eleve(ligne) {
            Ok(eleve) => eleves.push(eleve),
            Err(message) => println!("Ligne ignorée : {message}"),
        }
    }
    for eleve in &eleves {
        println!("{:?}", eleve);
    }
    println!("Erreur testée : {:?}", lire_eleve("Zoé,12,abs").map(|e| e.nom));
}
```

```sortie
Eleve { nom: "Léa", notes: [15.0, 12.5, 17.0] }
Eleve { nom: "Tom", notes: [9.0, 14.0, 11.0] }
Eleve { nom: "Inès", notes: [18.0, 16.0, 15.5] }
Eleve { nom: "Hugo", notes: [12.0, 8.5, 13.0] }
Erreur testée : Err("note invalide pour Zoé : « abs »")
```

> **Note :** `ok_or("ligne vide")` transforme une `Option` en `Result` : `None` devient
> une erreur, que `?` renvoie.

### Étape 2 : calculer les moyennes

Une méthode `moyenne` additionne les notes avec `sum()` et divise par leur nombre.
`len()` donne un `usize` : on le convertit en `f64` avec `as f64`, car Rust ne mélange
jamais deux types de nombres sans qu'on le demande. `{:.1}` affiche un seul chiffre
après la virgule :

```rust
struct Eleve {
    nom: String,
    notes: Vec<f64>,
}

impl Eleve {
    fn moyenne(&self) -> f64 {
        if self.notes.is_empty() {
            return 0.0;
        }
        self.notes.iter().sum::<f64>() / self.notes.len() as f64
    }
}

fn main() {
    let eleves = vec![
        Eleve { nom: String::from("Léa"), notes: vec![15.0, 12.5, 17.0] },
        Eleve { nom: String::from("Tom"), notes: vec![9.0, 14.0, 11.0] },
    ];
    for eleve in &eleves {
        println!("{} : {:.1}", eleve.nom, eleve.moyenne());
    }
}
```

```sortie
Léa : 14.8
Tom : 11.3
```

### Étape 3 : le meilleur élève et la moyenne de la classe

Pour trouver le maximum de nombres `f64`, on ne peut pas utiliser `max()` (voir les
erreurs fréquentes). On utilise `max_by`, en lui disant comment comparer deux élèves :
`total_cmp` compare deux `f64`.

```rust
struct Eleve {
    nom: String,
    notes: Vec<f64>,
}

impl Eleve {
    fn moyenne(&self) -> f64 {
        if self.notes.is_empty() {
            return 0.0;
        }
        self.notes.iter().sum::<f64>() / self.notes.len() as f64
    }
}

fn main() {
    let eleves = vec![
        Eleve { nom: String::from("Léa"), notes: vec![15.0, 12.5, 17.0] },
        Eleve { nom: String::from("Tom"), notes: vec![9.0, 14.0, 11.0] },
        Eleve { nom: String::from("Inès"), notes: vec![18.0, 16.0, 15.5] },
        Eleve { nom: String::from("Hugo"), notes: vec![12.0, 8.5, 13.0] },
    ];

    let meilleur = eleves
        .iter()
        .max_by(|a, b| a.moyenne().total_cmp(&b.moyenne()));
    if let Some(eleve) = meilleur {
        println!("Meilleure moyenne : {} ({:.1})", eleve.nom, eleve.moyenne());
    }

    let total: f64 = eleves.iter().map(|e| e.moyenne()).sum();
    println!("Moyenne de la classe : {:.1}", total / eleves.len() as f64);

    let au_dessus: Vec<&str> = eleves
        .iter()
        .filter(|e| e.moyenne() >= 12.0)
        .map(|e| e.nom.as_str())
        .collect();
    println!("Au moins 12 : {}", au_dessus.join(", "));
}
```

```sortie
Meilleure moyenne : Inès (16.5)
Moyenne de la classe : 13.5
Au moins 12 : Léa, Inès
```

### Étape 4 : un tableau bien aligné

Dans `println!`, on peut fixer la largeur d'une colonne :

| Écriture | Effet |
|----------|-------|
| `{:<10}` | aligné à gauche, sur 10 caractères |
| `{:>5}` | aligné à droite, sur 5 caractères |
| `{:>5.1}` | aligné à droite, sur 5 caractères, avec 1 chiffre après la virgule |

Voici le programme complet, qui assemble toutes les étapes :

```rust
const CSV: &str = "nom,maths,français,histoire
Léa,15,12.5,17
Tom,9,14,11
Inès,18,16,15.5
Hugo,12,8.5,13";

struct Eleve {
    nom: String,
    notes: Vec<f64>,
}

impl Eleve {
    fn moyenne(&self) -> f64 {
        if self.notes.is_empty() {
            return 0.0;
        }
        self.notes.iter().sum::<f64>() / self.notes.len() as f64
    }
}

fn lire_eleve(ligne: &str) -> Result<Eleve, String> {
    let mut morceaux = ligne.split(',');
    let nom = morceaux.next().ok_or("ligne vide")?.trim().to_string();
    let mut notes = Vec::new();
    for morceau in morceaux {
        let note: f64 = morceau
            .trim()
            .parse()
            .map_err(|_| format!("note invalide pour {nom} : « {morceau} »"))?;
        notes.push(note);
    }
    Ok(Eleve { nom, notes })
}

fn main() {
    let eleves: Vec<Eleve> = CSV
        .lines()
        .skip(1)
        .filter_map(|ligne| lire_eleve(ligne).ok()) // garde seulement les Ok
        .collect();

    println!("{:<10} {:>5} {:>5} {:>5} {:>5}", "Élève", "Maths", "Fran.", "Hist.", "Moy.");
    println!("{}", "-".repeat(34));
    for e in &eleves {
        println!(
            "{:<10} {:>5.1} {:>5.1} {:>5.1} {:>5.1}",
            e.nom, e.notes[0], e.notes[1], e.notes[2], e.moyenne()
        );
    }
    println!("{}", "-".repeat(34));

    // la moyenne de chaque matière (colonne 0, 1, 2)
    let n = eleves.len() as f64;
    let matiere = |i: usize| eleves.iter().map(|e| e.notes[i]).sum::<f64>() / n;
    let classe = eleves.iter().map(|e| e.moyenne()).sum::<f64>() / n;
    println!(
        "{:<10} {:>5.1} {:>5.1} {:>5.1} {:>5.1}",
        "Classe", matiere(0), matiere(1), matiere(2), classe
    );

    if let Some(meilleur) = eleves.iter().max_by(|a, b| a.moyenne().total_cmp(&b.moyenne())) {
        println!("\nBravo à {} !", meilleur.nom);
    }
}
```

```sortie
Élève      Maths Fran. Hist.  Moy.
----------------------------------
Léa         15.0  12.5  17.0  14.8
Tom          9.0  14.0  11.0  11.3
Inès        18.0  16.0  15.5  16.5
Hugo        12.0   8.5  13.0  11.2
----------------------------------
Classe      13.5  12.8  14.1  13.5

Bravo à Inès !
```

> **Astuce :** `filter_map` combine `filter` et `map` : la closure renvoie une `Option`,
> et seuls les `Some` sont gardés. `.ok()` transforme un `Result` en `Option`.

### Lire un vrai fichier CSV

Avec un fichier `notes.csv` à côté du programme, il suffit de remplacer la constante
par une lecture de fichier :

```rust norun
use std::fs;

fn main() -> std::io::Result<()> {
    let csv = fs::read_to_string("notes.csv")?;
    for ligne in csv.lines().skip(1) {
        // … même code qu'avant
    }
    Ok(())
}
```

### Pour aller plus loin

- Lisez les noms des matières dans la première ligne, au lieu de les écrire à la main.
- Triez le tableau par moyenne, de la meilleure à la moins bonne, avec
  `sort_by(|a, b| b.moyenne().total_cmp(&a.moyenne()))`.
- Ajoutez une colonne « Mention » : « Très bien » à partir de 16, « Bien » à partir
  de 14…
- Enregistrez le tableau dans un fichier `bulletin.txt`.

## Erreurs fréquentes

### Chercher le maximum de nombres à virgule avec max

```rust norun
fn main() {
    let moyennes = vec![14.8, 11.3, 16.5];
    let meilleure = moyennes.iter().max();
}
```

```text
error[E0277]: the trait bound `{float}: Ord` is not satisfied
 --> main.rs:3:37
  |
3 |     let meilleure = moyennes.iter().max();
  |                                     ^^^ the trait `Ord` is not implemented for `{float}`
```

Les `f64` ne sont pas toujours comparables (une valeur spéciale, `NaN`, « pas un
nombre », n'est ni plus petite ni plus grande qu'une autre). `max()` les refuse.
Utilisez `max_by` avec `total_cmp` :

```rust
let moyennes: Vec<f64> = vec![14.8, 11.3, 16.5];
let meilleure = moyennes.iter().copied().max_by(|a, b| a.total_cmp(b));
println!("{:?}", meilleure);
```

```sortie
Some(16.5)
```

### Oublier trim avant parse

```rust norun
fn main() {
    let note: f64 = " 12.5".parse().unwrap();
}
```

```text
thread 'main' panicked at main.rs:2:37:
called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }
```

L'espace au début rend le texte invalide. Nettoyez-le d'abord avec `trim()` :

```rust
let note: f64 = " 12.5".trim().parse().unwrap();
println!("{note}");
```

```sortie
12.5
```

### Utiliser ? dans un main qui ne renvoie rien

```rust norun
use std::fs;

fn main() {
    fs::write("taches.txt", "[ ] Acheter du pain\n")?;
}
```

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
help: consider adding return type
  |
3 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
```

Faites renvoyer un `Result` à `main`, et terminez par `Ok(())` :

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("taches.txt", "[ ] Acheter du pain\n")?;
    print!("{}", fs::read_to_string("taches.txt")?);
    Ok(())
}
```

```sortie
[ ] Acheter du pain
```

## Testez-vous

1. Dans le juste nombre, que renvoie `30.cmp(&42)` ?

> **Réponse**
>
> `Ordering::Less` : 30 est plus petit que 42. Le jeu répond donc « C'est plus ! ».

2. Dans la liste de tâches, pourquoi écrit-on `taches[numero - 1]` ?

> **Réponse**
>
> Le joueur compte à partir de 1, mais les positions d'un `Vec` commencent à 0. La
> tâche numéro 1 est donc à la position 0.

3. Qu'affiche `println!("[{:>5.1}]", 9.0);` ?

> **Réponse**
>
> `[  9.0]` : le nombre est écrit avec un chiffre après la virgule (`9.0`, 3
> caractères), puis aligné à droite sur 5 caractères.

## En résumé

- Construisez un programme par petites étapes, en le lançant à chaque fois.
- `cmp` et `match` comparent des nombres ; `trim` puis `parse` lisent une saisie.
- `fs::write` et `fs::read_to_string` enregistrent et relisent une liste simple.
- `lines`, `skip` et `split(',')` découpent un texte CSV en valeurs.
- `{:<10}` et `{:>5.1}` alignent les colonnes d'un tableau.
- Pour le maximum de `f64`, utilisez `max_by` avec `total_cmp`.

Comparez avec [les projets en Ruby](../ruby/projets.md) et
[en Rythme](../rythme/projets.md). Suite : [Exercices corrigés](exercices.md).
