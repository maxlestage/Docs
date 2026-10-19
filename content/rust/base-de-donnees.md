# Une base de données en Rust

Une base de données garde vos informations quand le programme s'arrête : en Rust, on lui parle avec la crate `rusqlite`, et on cache ce détail derrière un trait pour que le reste du programme ne dépende pas d'elle.

> **Objectifs :**
>
> - comprendre à quoi sert une base de données ;
> - créer une table, ajouter et lire des lignes avec `rusqlite` ;
> - écrire un trait `Depot` avec des méthodes qui renvoient un `Result` ;
> - le faire marcher avec une simple `HashMap`, puis avec SQLite ;
> - tester tout le programme sans base de données.

## Pourquoi une base de données ?

Vos variables vivent dans la [mémoire vive](../debuter/glossaire.md#memoire-vive).
Quand le programme s'arrête, elles disparaissent. Pour garder des informations, il faut
les écrire sur le disque.

Vous pourriez écrire un fichier texte (voir [JSON, CSV et TOML](formats.md)). Mais pour
retrouver **un** livre parmi 10 000, il faudrait relire tout le fichier. Une **base de
données** est un fichier rangé comme une armoire de bureau : des tiroirs étiquetés, des
fiches numérotées, et un employé qui retrouve la bonne fiche tout de suite.

```text
  Programme Rust                 Disque
 ┌──────────────┐  demande   ┌──────────────────────┐
 │ let livres = │ ─────────► │ bibliotheque.db      │
 │   ...        │            │  table « livres »    │
 │              │ ◄───────── │  id | titre | pages  │
 └──────────────┘  réponse   └──────────────────────┘
   (s'efface à la fin)          (reste après la fin)
```

Une **table** ressemble à un tableau de tableur : des colonnes (`id`, `titre`,
`pages`) et une ligne par élément. On lui parle dans un petit langage, le **SQL**
(voir [Les bases de données](../debuter/bases-de-donnees.md)).

| id | titre | pages |
|----|-------|-------|
| 1 | Dune | 412 |
| 2 | Le Petit Prince | 96 |

> **À retenir :** la mémoire s'efface quand le programme s'arrête ; une base de données
> garde les informations sur le disque et retrouve vite celle qu'on demande.

## SQLite et la crate rusqlite

**SQLite** est la base de données la plus simple : pas de serveur à installer, toute
la base tient dans **un seul fichier**. La [crate](../debuter/glossaire.md#crate)
`rusqlite` permet de l'utiliser depuis Rust :

```sh
cargo add rusqlite --features bundled
```

Votre `Cargo.toml` contient alors :

```toml
[dependencies]
rusqlite = { version = "0.40", features = ["bundled"] }
```

> **Note :** `bundled` embarque SQLite dans votre programme. Vous n'avez rien d'autre
> à installer sur l'ordinateur.

Les exemples avec `rusqlite` ne peuvent pas tourner sur cette page (il faut Cargo). Leur
sortie a été obtenue avec `cargo run`.

## Créer une table, ajouter et lire

Voici un programme complet. Il ouvre (ou crée) le fichier, crée la table, ajoute deux
livres et les relit :

```rust norun
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Livre {
    id: i64,
    titre: String,
    pages: u32,
}

fn main() -> Result<()> {
    let conn = Connection::open("bibliotheque.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS livres (
            id     INTEGER PRIMARY KEY,
            titre  TEXT NOT NULL,
            pages  INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute("DELETE FROM livres", [])?;

    conn.execute("INSERT INTO livres (titre, pages) VALUES (?1, ?2)", params!["Dune", 412])?;
    conn.execute("INSERT INTO livres (titre, pages) VALUES (?1, ?2)", params!["Le Petit Prince", 96])?;
    println!("Dernier id : {}", conn.last_insert_rowid());

    let mut requete = conn.prepare("SELECT id, titre, pages FROM livres ORDER BY id")?;
    let livres = requete.query_map([], |ligne| {
        Ok(Livre { id: ligne.get(0)?, titre: ligne.get(1)?, pages: ligne.get(2)? })
    })?;
    for livre in livres {
        println!("{:?}", livre?);
    }
    Ok(())
}
```

```text
Dernier id : 2
Livre { id: 1, titre: "Dune", pages: 412 }
Livre { id: 2, titre: "Le Petit Prince", pages: 96 }
```

Pas à pas :

| Étape | Ligne | Ce qui se passe | Table `livres` |
|-------|-------|-----------------|----------------|
| 1 | `Connection::open` | ouvre le fichier (le crée s'il n'existe pas) | n'existe pas encore |
| 2 | `CREATE TABLE IF NOT EXISTS` | crée la table, seulement la première fois | vide |
| 3 | `DELETE FROM livres` | vide la table (pour cet exemple) | vide |
| 4 | 1er `INSERT` | SQLite choisit l'id 1 | (1, Dune, 412) |
| 5 | 2e `INSERT` | SQLite choisit l'id 2 | + (2, Le Petit Prince, 96) |
| 6 | `last_insert_rowid` | renvoie l'id du dernier ajout | 2 |
| 7 | `query_map` | lit chaque ligne et fabrique un `Livre` | inchangée |

- `?1`, `?2` sont des **trous** remplis par `params![…]`, dans l'ordre.
- `ligne.get(0)?` lit la colonne n° 0 de la ligne (on compte à partir de 0).
- Chaque appel renvoie un [`Result`](../debuter/glossaire.md#result) : `?` arrête la
  fonction à la première erreur (voir [Gérer les erreurs](erreurs.md)).

> **Attention :** n'écrivez jamais `format!("… VALUES ('{titre}')")` pour mettre une
> valeur dans une requête. Un titre comme `l'Étranger` casse la requête, et un
> utilisateur malveillant peut y glisser ses propres ordres SQL (une « injection
> SQL »). Les trous `?1` sont toujours sûrs.

Essayez : enlevez la ligne `DELETE FROM livres` et lancez deux fois. La deuxième fois,
vous verrez quatre livres et `Dernier id : 4` : le fichier a gardé ceux de la première
fois.

> **À retenir :** `execute` pour modifier, `prepare` + `query_map` pour lire, et des
> trous `?1` pour les valeurs.

## Le problème : SQL partout

Imaginez maintenant un programme plus gros. Si chaque fonction appelle `conn.execute`
directement, le SQL est éparpillé partout :

- pour tester une fonction, il faut une vraie base ;
- pour changer de base (PostgreSQL au lieu de SQLite), il faut tout relire.

La solution : un **guichet**. Au guichet d'une bibliothèque, vous dites « je voudrais
le livre n° 2 ». Vous ne savez pas si l'employé va le chercher dans un tiroir, dans une
cave ou dans un entrepôt. Vous savez seulement ce que vous pouvez demander.

En Rust, ce guichet est un [trait](../debuter/glossaire.md#trait) (voir
[Traits et objets](traits-objets.md)). On l'appelle souvent `Depot` (*repository* en
anglais).

```text
           le reste du programme
                    │
                    ▼
        ┌───────────────────────┐
        │     trait Depot       │   ajouter, trouver, tous, supprimer
        └───────────────────────┘
             ▲               ▲
             │               │
   ┌──────────────┐   ┌──────────────┐
   │ DepotMemoire │   │ DepotSqlite  │
   │  (HashMap)   │   │ (rusqlite)   │
   └──────────────┘   └──────────────┘
     pour essayer       pour de vrai
     et tester
```

> **À retenir :** le reste du programme ne parle qu'au trait. Derrière, on peut
> brancher une `HashMap` ou une vraie base, sans rien changer d'autre.

## Le trait Depot

Le guichet propose quatre demandes. Chacune peut échouer, donc chacune renvoie un
`Result` :

```rust
#[derive(Debug, Clone, PartialEq)]
struct Livre {
    id: u32,
    titre: String,
    pages: u32,
}

#[derive(Debug, PartialEq)]
enum ErreurDepot {
    Introuvable(u32),
}

trait Depot {
    fn ajouter(&mut self, titre: &str, pages: u32) -> Result<u32, ErreurDepot>;
    fn trouver(&self, id: u32) -> Result<Livre, ErreurDepot>;
    fn tous(&self) -> Result<Vec<Livre>, ErreurDepot>;
    fn supprimer(&mut self, id: u32) -> Result<(), ErreurDepot>;
}
```

Ce bloc ne fait rien : il **décrit** seulement. Lisons chaque ligne :

| Méthode | Reçoit | Renvoie si tout va bien | Peut échouer si… |
|---------|--------|-------------------------|------------------|
| `ajouter` | un titre, un nombre de pages | le nouvel id (`u32`) | la base a un problème |
| `trouver` | un id | une copie du `Livre` | aucun livre n'a cet id |
| `tous` | rien | la liste des livres | la base a un problème |
| `supprimer` | un id | rien (`()`) | aucun livre n'a cet id |

- `&mut self` : la méthode **modifie** le dépôt (ajouter, supprimer).
- `&self` : la méthode **regarde** seulement (trouver, tous).
- `ErreurDepot` est notre propre type d'erreur (voir
  [Votre propre type d'erreur](erreurs.md#votre-propre-type-d-erreur)).

> **À retenir :** un trait est une liste de promesses. Chaque type qui l'implémente
> doit tenir les quatre.

## Un dépôt en mémoire avec une HashMap

Première implémentation : une [`HashMap`](hashmap.md) qui range chaque livre sous son
id, et un compteur `prochain_id` qui donne les numéros, comme le ticket d'une file
d'attente.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Livre {
    id: u32,
    titre: String,
    pages: u32,
}

#[derive(Debug, PartialEq)]
enum ErreurDepot {
    Introuvable(u32),
}

trait Depot {
    fn ajouter(&mut self, titre: &str, pages: u32) -> Result<u32, ErreurDepot>;
    fn trouver(&self, id: u32) -> Result<Livre, ErreurDepot>;
    fn tous(&self) -> Result<Vec<Livre>, ErreurDepot>;
    fn supprimer(&mut self, id: u32) -> Result<(), ErreurDepot>;
}

struct DepotMemoire {
    livres: HashMap<u32, Livre>,
    prochain_id: u32,
}

impl DepotMemoire {
    fn new() -> DepotMemoire {
        DepotMemoire { livres: HashMap::new(), prochain_id: 1 }
    }
}

impl Depot for DepotMemoire {
    fn ajouter(&mut self, titre: &str, pages: u32) -> Result<u32, ErreurDepot> {
        let id = self.prochain_id;
        self.prochain_id += 1;
        self.livres.insert(id, Livre { id, titre: titre.to_string(), pages });
        Ok(id)
    }

    fn trouver(&self, id: u32) -> Result<Livre, ErreurDepot> {
        self.livres.get(&id).cloned().ok_or(ErreurDepot::Introuvable(id))
    }

    fn tous(&self) -> Result<Vec<Livre>, ErreurDepot> {
        let mut liste: Vec<Livre> = self.livres.values().cloned().collect();
        liste.sort_by_key(|livre| livre.id);
        Ok(liste)
    }

    fn supprimer(&mut self, id: u32) -> Result<(), ErreurDepot> {
        match self.livres.remove(&id) {
            Some(_) => Ok(()),
            None => Err(ErreurDepot::Introuvable(id)),
        }
    }
}

fn main() {
    let mut depot = DepotMemoire::new();
    let a = depot.ajouter("Dune", 412).unwrap();
    let b = depot.ajouter("Le Petit Prince", 96).unwrap();
    let c = depot.ajouter("Bilbo", 310).unwrap();
    println!("ids : {a}, {b}, {c}");

    depot.supprimer(b).unwrap();
    println!("{:?}", depot.trouver(b));
    println!("{:?}", depot.trouver(c));

    for livre in depot.tous().unwrap() {
        println!("{} - {} ({} pages)", livre.id, livre.titre, livre.pages);
    }
}
```

```sortie
ids : 1, 2, 3
Err(Introuvable(2))
Ok(Livre { id: 3, titre: "Bilbo", pages: 310 })
1 - Dune (412 pages)
3 - Bilbo (310 pages)
```

### Pas à pas

Suivons le dépôt pendant `main` :

| Étape | Appel | `prochain_id` après | Clés dans `livres` | Résultat |
|-------|-------|--------------------:|--------------------|----------|
| 1 | `DepotMemoire::new()` | 1 | (aucune) | un dépôt vide |
| 2 | `ajouter("Dune", 412)` | 2 | 1 | `Ok(1)` |
| 3 | `ajouter("Le Petit Prince", 96)` | 3 | 1, 2 | `Ok(2)` |
| 4 | `ajouter("Bilbo", 310)` | 4 | 1, 2, 3 | `Ok(3)` |
| 5 | `supprimer(2)` | 4 | 1, 3 | `Ok(())` |
| 6 | `trouver(2)` | 4 | 1, 3 | `Err(Introuvable(2))` |
| 7 | `trouver(3)` | 4 | 1, 3 | `Ok(Livre { id: 3, … })` |
| 8 | `tous()` | 4 | 1, 3 | `Ok(vec![Dune, Bilbo])` |

Remarquez l'étape 5 : l'id 2 a disparu, mais `prochain_id` ne recule pas. Le prochain
livre aura l'id 4. Une vraie base fait pareil : un numéro donné n'est jamais redonné.

### Trois détails importants

- `get(&id)` renvoie une **référence** (`Option<&Livre>`). `.cloned()` en fait une copie
  (`Option<Livre>`), et `.ok_or(…)` transforme `None` en erreur.
- `remove` renvoie `Some(livre)` s'il y avait un livre, `None` sinon : c'est exactement
  ce qu'il faut pour savoir si la suppression a marché.
- Une `HashMap` rend ses valeurs **dans le désordre**, et cet ordre change d'une
  exécution à l'autre. `sort_by_key` les remet dans l'ordre des ids, comme le
  `ORDER BY id` du SQL.

Essayez : enlevez la ligne `liste.sort_by_key(…)` et lancez plusieurs fois. Dune et
Bilbo s'affichent parfois dans un ordre, parfois dans l'autre.

Essayez : ajoutez `println!("{:?}", depot.supprimer(2));` à la fin de `main`. Le
programme affiche `Err(Introuvable(2))` : le livre était déjà supprimé.

> **À retenir :** une `HashMap<u32, Livre>` et un compteur suffisent pour imiter une
> table : l'id est la clé, le livre est la valeur.

## Afficher une erreur proprement

`Err(Introuvable(2))` est fait pour le développeur. Pour l'utilisateur, on écrit une
phrase avec le trait `Display`, puis on traite chaque cas avec `match` :

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum ErreurDepot {
    Introuvable(u32),
    Base(String),
}

impl fmt::Display for ErreurDepot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErreurDepot::Introuvable(id) => write!(f, "aucun livre avec l'id {id}"),
            ErreurDepot::Base(message) => write!(f, "erreur de la base : {message}"),
        }
    }
}

fn chercher(id: u32) -> Result<String, ErreurDepot> {
    match id {
        1 => Ok(String::from("Dune")),
        99 => Err(ErreurDepot::Base(String::from("disque plein"))),
        _ => Err(ErreurDepot::Introuvable(id)),
    }
}

fn main() {
    for id in [1, 2, 99] {
        match chercher(id) {
            Ok(titre) => println!("{id} : {titre}"),
            Err(e) => println!("{id} : Erreur : {e}"),
        }
    }
}
```

```sortie
1 : Dune
2 : Erreur : aucun livre avec l'id 2
99 : Erreur : erreur de la base : disque plein
```

La variante `Base` servira à la version SQLite : quand le disque ou la requête pose
problème, on range le message de `rusqlite` dedans.

| `id` | `chercher(id)` | Branche du `match` | Ligne affichée |
|-----:|----------------|--------------------|----------------|
| 1 | `Ok("Dune")` | `Ok(titre)` | `1 : Dune` |
| 2 | `Err(Introuvable(2))` | `Err(e)` | `2 : Erreur : aucun livre…` |
| 99 | `Err(Base("disque plein"))` | `Err(e)` | `99 : Erreur : erreur de la base…` |

> **À retenir :** `Debug` (`{:?}`) pour vous, `Display` (`{}`) pour l'utilisateur.

## Écrire le reste du programme une seule fois

Voici une fonction qui travaille avec **n'importe quel** dépôt. Elle reçoit
`&impl Depot` : « quelque chose qui tient les promesses de `Depot` » (voir
[Les génériques](generiques.md)). Elle ne sait pas si c'est une `HashMap` ou SQLite.

Ajoutez-la au programme de la section [Un dépôt en mémoire avec une
HashMap](#un-depot-en-memoire-avec-une-hashmap), et ajoutez les lignes de test à la fin
de `main` :

```rust norun
// Ne connaît que le trait : marche avec tous les dépôts.
fn pages_totales(depot: &impl Depot) -> Result<u32, ErreurDepot> {
    let mut total = 0;
    for livre in depot.tous()? {
        total += livre.pages;
    }
    Ok(total)
}

fn main() {
    // ... le début de main, comme plus haut ...
    println!("Pages à lire : {}", pages_totales(&depot).unwrap());

    // De petits tests : le dépôt doit refuser un id inconnu.
    assert_eq!(depot.supprimer(7), Err(ErreurDepot::Introuvable(7)));
    assert_eq!(depot.tous().unwrap().len(), 2);
    println!("Tests réussis");
}
```

Les nouvelles lignes affichées à la fin :

```text
Pages à lire : 722
Tests réussis
```

La boucle de `pages_totales`, pas à pas (le livre 2 a été supprimé) :

| Tour | `livre.titre` | `livre.pages` | `total` après |
|-----:|---------------|--------------:|--------------:|
| début | | | 0 |
| 1 | Dune | 412 | 412 |
| 2 | Bilbo | 310 | 722 |

Les deux `assert_eq!` sont de petits tests : si l'un est faux, le programme s'arrête
avec un message (voir [Tester son code](tests.md)). Grâce au dépôt en mémoire, ces tests
tournent en une fraction de seconde, sans fichier ni base.

Essayez : remplacez `supprimer(7)` par `supprimer(1)`. Le livre 1 existe, la méthode
renvoie `Ok(())`, et le programme s'arrête sur `assertion `left == right` failed`.

> **À retenir :** écrivez vos fonctions avec `impl Depot` ; testez-les avec le dépôt en
> mémoire, utilisez-les avec la vraie base.

## Le même trait avec rusqlite

Il reste à brancher la vraie base. `DepotSqlite` garde une connexion, et chaque méthode
du trait devient une requête SQL. Les types `Livre`, `ErreurDepot` (avec sa variante
`Base`) et le trait `Depot` sont **exactement** ceux de plus haut :

```rust norun
use rusqlite::{params, Connection, OptionalExtension};

// Transforme une erreur de rusqlite en ErreurDepot.
fn base(e: rusqlite::Error) -> ErreurDepot {
    ErreurDepot::Base(e.to_string())
}

struct DepotSqlite {
    conn: Connection,
}

impl DepotSqlite {
    fn ouvrir(chemin: &str) -> Result<DepotSqlite, ErreurDepot> {
        let conn = Connection::open(chemin).map_err(base)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS livres (
                id    INTEGER PRIMARY KEY,
                titre TEXT NOT NULL,
                pages INTEGER NOT NULL
            )",
            [],
        )
        .map_err(base)?;
        Ok(DepotSqlite { conn })
    }
}

impl Depot for DepotSqlite {
    fn ajouter(&mut self, titre: &str, pages: u32) -> Result<u32, ErreurDepot> {
        self.conn
            .execute(
                "INSERT INTO livres (titre, pages) VALUES (?1, ?2)",
                params![titre, pages],
            )
            .map_err(base)?;
        Ok(self.conn.last_insert_rowid() as u32)
    }

    fn trouver(&self, id: u32) -> Result<Livre, ErreurDepot> {
        self.conn
            .query_row(
                "SELECT id, titre, pages FROM livres WHERE id = ?1",
                [id],
                |ligne| {
                    Ok(Livre {
                        id: ligne.get(0)?,
                        titre: ligne.get(1)?,
                        pages: ligne.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(base)?
            .ok_or(ErreurDepot::Introuvable(id))
    }

    fn tous(&self) -> Result<Vec<Livre>, ErreurDepot> {
        let mut requete = self
            .conn
            .prepare("SELECT id, titre, pages FROM livres ORDER BY id")
            .map_err(base)?;
        let lignes = requete
            .query_map([], |ligne| {
                Ok(Livre {
                    id: ligne.get(0)?,
                    titre: ligne.get(1)?,
                    pages: ligne.get(2)?,
                })
            })
            .map_err(base)?;
        lignes.collect::<Result<Vec<Livre>, _>>().map_err(base)
    }

    fn supprimer(&mut self, id: u32) -> Result<(), ErreurDepot> {
        let nombre = self
            .conn
            .execute("DELETE FROM livres WHERE id = ?1", [id])
            .map_err(base)?;
        if nombre == 0 {
            Err(ErreurDepot::Introuvable(id))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut depot = DepotSqlite::ouvrir("bibliotheque.db").unwrap();
    let a = depot.ajouter("Dune", 412).unwrap();
    let b = depot.ajouter("Le Petit Prince", 96).unwrap();
    println!("ids : {a} et {b}");
    println!("{:?}", depot.trouver(a));
    depot.supprimer(a).unwrap();
    for livre in depot.tous().unwrap() {
        println!("{} - {} ({} pages)", livre.id, livre.titre, livre.pages);
    }
    match depot.trouver(a) {
        Ok(livre) => println!("trouvé : {}", livre.titre),
        Err(e) => println!("Erreur : {e}"),
    }
}
```

Premier lancement (le fichier n'existe pas encore) :

```text
ids : 1 et 2
Ok(Livre { id: 1, titre: "Dune", pages: 412 })
2 - Le Petit Prince (96 pages)
Erreur : aucun livre avec l'id 1
```

Deuxième lancement : le fichier `bibliotheque.db` a gardé le Petit Prince n° 2.

```text
ids : 3 et 4
Ok(Livre { id: 3, titre: "Dune", pages: 412 })
2 - Le Petit Prince (96 pages)
4 - Le Petit Prince (96 pages)
Erreur : aucun livre avec l'id 3
```

Méthode par méthode, la traduction est simple :

| Méthode du trait | `DepotMemoire` (HashMap) | `DepotSqlite` (SQL) |
|------------------|--------------------------|---------------------|
| `ajouter` | `insert` + compteur | `INSERT` + `last_insert_rowid` |
| `trouver` | `get(&id)` | `SELECT … WHERE id = ?1` |
| `tous` | `values()` + `sort_by_key` | `SELECT … ORDER BY id` |
| `supprimer` | `remove(&id)` → `None` = introuvable | `DELETE` → 0 ligne = introuvable |
| erreur de la base | jamais | `map_err(base)` → `ErreurDepot::Base` |

- `.optional()` transforme « aucune ligne » en `Ok(None)`, au lieu d'une erreur.
- `execute` renvoie le **nombre de lignes** touchées : 0 veut dire que l'id n'existait pas.
- `map_err(base)` range toute erreur de `rusqlite` dans notre `ErreurDepot::Base`.
  Ainsi le trait ne parle jamais de `rusqlite`.

Et la fonction `pages_totales` de la section précédente ? Elle marche telle quelle :
`pages_totales(&depot)` avec un `DepotSqlite`. Pas une ligne à changer.

> **À retenir :** pour passer de la mémoire à SQLite, on écrit un nouveau
> `impl Depot for …`. Le reste du programme ne bouge pas.

## Erreurs fréquentes

### Oublier &mut self pour modifier

```rust norun
use std::collections::HashMap;

struct DepotMemoire {
    livres: HashMap<u32, String>,
}

impl DepotMemoire {
    fn ajouter(&self, id: u32, titre: &str) {
        self.livres.insert(id, titre.to_string());
    }
}
```

```text
error[E0596]: cannot borrow `self.livres` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:9
  |
9 |         self.livres.insert(id, titre.to_string());
  |         ^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
8 |     fn ajouter(&mut self, id: u32, titre: &str) {
  |                 +++
```

`&self` permet seulement de regarder. Pour ajouter ou supprimer, écrivez `&mut self`,
dans le trait **et** dans l'`impl`.

### Oublier une méthode du trait

```rust norun
trait Depot {
    fn ajouter(&mut self, titre: &str) -> Result<u32, String>;
    fn supprimer(&mut self, id: u32) -> Result<(), String>;
}

struct DepotMemoire;

impl Depot for DepotMemoire {
    fn ajouter(&mut self, _titre: &str) -> Result<u32, String> {
        Ok(1)
    }
}
```

```text
error[E0046]: not all trait items implemented, missing: `supprimer`
 --> src/main.rs:8:1
  |
3 |     fn supprimer(&mut self, id: u32) -> Result<(), String>;
  |     ------------------------------------------------------- `supprimer` from trait
...
8 | impl Depot for DepotMemoire {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `supprimer` in implementation
```

Un trait est une promesse complète : écrivez toutes ses méthodes.

### Utiliser une table qui n'existe pas

```rust norun
let conn = Connection::open_in_memory().unwrap();
conn.execute("INSERT INTO livres (titre, pages) VALUES ('Dune', 412)", [])
    .unwrap();
```

```text
called `Result::unwrap()` on an `Err` value: SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("no such table: livres"))
```

Le programme compile : le SQL n'est vérifié qu'à l'exécution. Créez la table d'abord,
avec `CREATE TABLE IF NOT EXISTS`, au moment d'ouvrir la base.

## Testez-vous

1. Pourquoi `tous` trie-t-il la liste dans `DepotMemoire` ?

> **Réponse**
>
> Parce qu'une `HashMap` rend ses valeurs dans un ordre qui change d'une exécution à
> l'autre. `sort_by_key(|livre| livre.id)` donne toujours le même ordre, comme
> `ORDER BY id` en SQL.

2. Pourquoi `trouver` prend-il `&self` alors qu'`ajouter` prend `&mut self` ?

> **Réponse**
>
> `trouver` regarde seulement le dépôt, il ne change rien. `ajouter` modifie le
> dépôt (il insère un livre et augmente le compteur), donc il a besoin de `&mut self`.

3. Quel est l'intérêt du trait `Depot` si on n'utilise que SQLite au final ?

> **Réponse**
>
> On peut tester tout le programme avec `DepotMemoire`, sans fichier ni base, très
> vite. Et le SQL reste rangé à un seul endroit : si on change de base, on n'écrit
> qu'un nouvel `impl Depot`.

## En résumé

- Une base de données garde les informations sur le disque ; SQLite tient dans un seul
  fichier.
- Avec `rusqlite` : `execute` pour modifier, `prepare` + `query_map` pour lire, des
  trous `?1` pour les valeurs.
- Un trait `Depot` (ajouter, trouver, tous, supprimer) cache la base au reste du
  programme ; chaque méthode renvoie un `Result`.
- `DepotMemoire` (une `HashMap` et un compteur) sert à essayer et à tester.
- `DepotSqlite` implémente le même trait avec du SQL : rien d'autre ne change.

Suite : [Rust pour le web](web.md)
