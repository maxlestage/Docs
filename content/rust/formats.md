# JSON, CSV et TOML avec serde

Pour enregistrer vos données dans un fichier ou les envoyer sur le réseau, vous les transformez en texte dans un format connu (JSON, CSV, TOML) : en Rust, la crate serde fait ce travail presque toute seule.

> **Objectifs :**
>
> - comprendre ce que veut dire « sérialiser » et « désérialiser » ;
> - écrire et lire du JSON avec `serde` et `serde_json` ;
> - renommer des champs et gérer les champs absents avec `Option` ;
> - lire et écrire du CSV, à la main puis avec la crate `csv` ;
> - lire un fichier de configuration TOML.

## Sérialiser, c'est quoi ?

Dans votre programme, un livre est une structure : un titre, un auteur, un nombre de
pages. Mais un fichier ne contient que du **texte**. Pour enregistrer le livre, il faut
le transformer en texte. Pour le relire, il faut faire l'inverse.

- **Sérialiser** : transformer une valeur Rust en texte (ou en octets).
- **Désérialiser** : reconstruire une valeur Rust à partir de ce texte.

C'est comme ranger un meuble en kit dans un carton, avec sa notice : n'importe qui
peut ensuite le remonter à l'identique. Le **format** est la notice : tout le monde
doit lire la même.

Les trois formats les plus courants :

```text
JSON :  {"titre":"Dune","pages":412}
CSV  :  titre,pages
        Dune,412
TOML :  titre = "Dune"
        pages = 412
```

## Un CSV à la main, avec format!

Le CSV (*comma-separated values*) est le format le plus simple : une ligne par
élément, des valeurs séparées par des virgules. La première ligne donne souvent le nom
des colonnes. Pour un petit fichier, `format!` suffit :

```rust
struct Eleve {
    nom: String,
    age: u32,
    note: f64,
}

fn main() {
    let eleves = vec![
        Eleve { nom: String::from("Alice"), age: 15, note: 14.5 },
        Eleve { nom: String::from("Bob"), age: 16, note: 12.0 },
    ];

    let mut csv = String::from("nom,age,note\n");
    for e in &eleves {
        csv.push_str(&format!("{},{},{}\n", e.nom, e.age, e.note));
    }

    std::fs::write("eleves.csv", &csv).unwrap();
    print!("{}", std::fs::read_to_string("eleves.csv").unwrap());
}
```

```sortie
nom,age,note
Alice,15,14.5
Bob,16,12
```

> **Note :** `12.0` s'affiche `12` avec `{}`. Pour garder deux décimales, écrivez
> `{:.2}`.

## Lire un CSV avec split

Pour relire le fichier, on découpe le texte en lignes avec `lines()`, puis chaque ligne
en morceaux avec `split(',')`. On saute la première ligne (les titres) avec `skip(1)`,
et on convertit les nombres avec `parse` (voir [Découper avec split](texte.md#decouper-avec-split)) :

```rust
fn main() {
    let texte = "nom,age,note\nAlice,15,14.5\nBob,16,12\n";

    for ligne in texte.lines().skip(1) {
        let morceaux: Vec<&str> = ligne.split(',').collect();
        let nom = morceaux[0];
        let age: u32 = morceaux[1].parse().unwrap();
        let note: f64 = morceaux[2].parse().unwrap();
        println!("{nom} a {age} ans et {note} de moyenne");
    }
}
```

```sortie
Alice a 15 ans et 14.5 de moyenne
Bob a 16 ans et 12 de moyenne
```

Une ligne mal écrite ferait planter `unwrap`. Une version plus solide renvoie un
`Result` et utilise `?` (voir [Gérer les erreurs](erreurs.md)) :

```rust
fn lire_ligne(ligne: &str) -> Result<(String, u32), String> {
    let (nom, age) = ligne
        .split_once(',')
        .ok_or(format!("pas de virgule dans « {ligne} »"))?;
    let age: u32 = age.trim().parse().map_err(|_| format!("âge invalide : {age}"))?;
    Ok((nom.to_string(), age))
}

fn main() {
    println!("{:?}", lire_ligne("Alice,15"));
    println!("{:?}", lire_ligne("Bob,seize"));
    println!("{:?}", lire_ligne("Chloé"));
}
```

```sortie
Ok(("Alice", 15))
Err("âge invalide : seize")
Err("pas de virgule dans « Chloé »")
```

> **Attention :** le découpage à la main casse dès qu'une valeur contient une virgule
> (`"Dupont, Léo"`) ou un retour à la ligne. Pour de vrais fichiers, utilisez la crate
> `csv`, présentée plus bas.

## Installer serde et serde_json

`serde` (de *SERialize* et *DEserialize*) est la [crate](../debuter/glossaire.md#crate)
de référence. Elle ne connaît aucun format : elle décrit vos données. Chaque format a
sa propre crate : `serde_json` pour JSON, `csv`, `toml`… Ajoutez-les à votre projet :

```sh
cargo add serde --features derive
cargo add serde_json
```

Votre `Cargo.toml` contient alors :

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

> **Note :** la fonctionnalité `derive` active `#[derive(Serialize, Deserialize)]`.
> Sans elle, le compilateur ne trouve pas ces macros.

Les exemples qui suivent utilisent ces crates : ils ne peuvent pas tourner sur cette
page, mais leur sortie a été obtenue avec `cargo run`.

## Écrire du JSON : to_string et to_string_pretty

On ajoute `Serialize` et `Deserialize` à la liste des `derive` de la structure. serde
écrit alors tout le code de conversion pour vous :

```rust norun
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Livre {
    titre: String,
    auteur: String,
    pages: u32,
    lu: bool,
}

fn main() -> Result<(), serde_json::Error> {
    let livre = Livre {
        titre: String::from("Dune"),
        auteur: String::from("Frank Herbert"),
        pages: 412,
        lu: true,
    };

    let compact = serde_json::to_string(&livre)?;
    println!("{compact}");

    let joli = serde_json::to_string_pretty(&livre)?;
    println!("{joli}");
    Ok(())
}
```

```text
{"titre":"Dune","auteur":"Frank Herbert","pages":412,"lu":true}
{
  "titre": "Dune",
  "auteur": "Frank Herbert",
  "pages": 412,
  "lu": true
}
```

- `to_string` donne un JSON compact, sur une ligne : idéal pour le réseau ;
- `to_string_pretty` l'indente : idéal pour un fichier que des humains liront.

Pour l'enregistrer, il suffit ensuite d'un `std::fs::write("livre.json", joli)`.

## Lire du JSON : from_str

`serde_json::from_str` fait l'inverse. Il faut lui dire **quel type** construire, avec
une annotation (`let livre: Livre`) :

```rust norun
let texte = r#"
    { "titre": "Le Petit Prince", "auteur": "Saint-Exupéry",
      "pages": 96, "lu": false }
"#;

let livre: Livre = serde_json::from_str(texte)?;
println!("{livre:?}");
println!("{} fait {} pages", livre.titre, livre.pages);

let liste: Vec<u32> = serde_json::from_str("[3, 1, 4]")?;
println!("{liste:?}");
```

```text
Livre { titre: "Le Petit Prince", auteur: "Saint-Exupéry", pages: 96, lu: false }
Le Petit Prince fait 96 pages
[3, 1, 4]
```

Si le texte ne correspond pas au type, `from_str` renvoie une erreur claire, avec la
position du problème :

```text
missing field `pages` at line 1 column 19
invalid type: string "412", expected u32 at line 1 column 33
```

> **À retenir :** le type Rust sert de **modèle**. serde vérifie que le JSON a les bons
> champs, avec les bons types. Vous n'avez rien à vérifier à la main.

## Renommer les champs

En Rust, les champs s'écrivent en `snake_case`. Beaucoup d'API JSON utilisent le
`camelCase`, ou des noms impossibles en Rust (`e-mail`). Les **attributs** `#[serde(…)]`
font le lien :

- `#[serde(rename_all = "camelCase")]` sur la structure renomme tous les champs ;
- `#[serde(rename = "…")]` sur un champ lui donne un nom précis.

```rust norun
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Utilisateur {
    nom_complet: String,
    #[serde(rename = "e-mail")]
    email: String,
    date_naissance: Option<String>,
}

let ada = Utilisateur {
    nom_complet: String::from("Ada Lovelace"),
    email: String::from("ada@exemple.fr"),
    date_naissance: None,
};
println!("{}", serde_json::to_string_pretty(&ada)?);
```

```text
{
  "nomComplet": "Ada Lovelace",
  "e-mail": "ada@exemple.fr",
  "dateNaissance": null
}
```

## Les champs facultatifs : Option

Un champ de type `Option<T>` peut manquer dans le JSON : il vaut alors `None`. Quand on
écrit, `None` devient `null`. Deux attributs affinent ce comportement :

- `#[serde(skip_serializing_if = "Option::is_none")]` : n'écrit pas le champ s'il
  vaut `None` ;
- `#[serde(default)]` : si le champ manque, prend la valeur par défaut du type
  (`0` pour un nombre, `""` pour un texte…).

```rust norun
#[derive(Debug, Serialize, Deserialize)]
struct Produit {
    nom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    promo: Option<u32>,
    #[serde(default)]
    stock: u32,
}

let p: Produit = serde_json::from_str(r#"{ "nom": "Stylo" }"#)?;
println!("{p:?}");
println!("{}", serde_json::to_string(&p)?);
```

```text
Produit { nom: "Stylo", promo: None, stock: 0 }
{"nom":"Stylo","stock":0}
```

## Le CSV avec la crate csv

La crate `csv` gère les cas difficiles (virgules dans une valeur, guillemets…) et
s'appuie sur serde. Ajoutez-la avec `cargo add csv`.

```rust norun
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Eleve {
    nom: String,
    age: u32,
    note: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Lire : chaque ligne devient un Eleve.
    let donnees = "nom,age,note\nAlice,15,14.5\nBob,16,12\n";
    let mut lecteur = csv::Reader::from_reader(donnees.as_bytes());
    for ligne in lecteur.deserialize() {
        let eleve: Eleve = ligne?;
        println!("{} a {} ans et {} de moyenne", eleve.nom, eleve.age, eleve.note);
    }

    // Écrire : la ligne de titres est ajoutée toute seule.
    let mut ecrivain = csv::Writer::from_writer(vec![]);
    ecrivain.serialize(Eleve { nom: "Chloé".into(), age: 15, note: 16.0 })?;
    ecrivain.serialize(Eleve { nom: "Dupont, Léo".into(), age: 17, note: 11.5 })?;
    let texte = String::from_utf8(ecrivain.into_inner()?)?;
    print!("{texte}");
    Ok(())
}
```

```text
Alice a 15 ans et 14.5 de moyenne
Bob a 16 ans et 12 de moyenne
nom,age,note
Chloé,15,16.0
"Dupont, Léo",17,11.5
```

Remarquez les guillemets autour de `"Dupont, Léo"` : la crate les a ajoutés pour que
la virgule ne coupe pas la valeur en deux.

> **Astuce :** pour lire un vrai fichier, remplacez `from_reader(…)` par
> `csv::Reader::from_path("eleves.csv")?`.

## Un fichier de configuration en TOML

Le TOML est fait pour les fichiers de **configuration** écrits à la main. Vous le
connaissez déjà : `Cargo.toml` est en TOML. Des `clé = valeur`, et des sections entre
crochets. Ajoutez la crate avec `cargo add toml`.

```rust norun
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    titre: String,
    serveur: Serveur,
}

#[derive(Debug, Serialize, Deserialize)]
struct Serveur {
    adresse: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let texte = r#"
titre = "Mon site"

[serveur]
adresse = "127.0.0.1"
port = 8080
debug = true
"#;

    let config: Config = toml::from_str(texte)?;
    println!("{} écoute sur {}:{}", config.titre, config.serveur.adresse, config.serveur.port);
    Ok(())
}
```

```text
Mon site écoute sur 127.0.0.1:8080
```

La section `[serveur]` correspond au champ `serveur`, de type `Serveur`. Pour un vrai
fichier : `let texte = std::fs::read_to_string("config.toml")?;`.

## Quel format choisir ?

| Format | Lisible par un humain | Idéal pour | Crate |
|--------|----------------------|------------|-------|
| JSON | oui | échanger avec une API web, enregistrer des données imbriquées | `serde_json` |
| CSV | oui | des tableaux : une ligne par élément, à ouvrir dans un tableur | `csv` |
| TOML | très | les fichiers de configuration écrits à la main | `toml` |
| Binaire (`bincode`…) | non | stocker beaucoup de données, vite et en peu de place | `bincode`, `postcard` |

Le grand avantage de serde : la **même** structure, avec le même `derive`, passe d'un
format à l'autre. Seule la crate du format change.

## Erreurs fréquentes

### Oublier le derive

```rust norun
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let json = serde_json::to_string(&p).unwrap();
    println!("{json}");
}
```

```text
error[E0277]: the trait bound `Point: serde::Serialize` is not satisfied
    --> src/main.rs:9:38
     |
   9 |     let json = serde_json::to_string(&p).unwrap();
     |                --------------------- ^^ unsatisfied trait bound
     |                |
     |                required by a bound introduced by this call
     |
     = note: for local types consider adding `#[derive(serde::Serialize)]` to your `Point` type
```

serde ne sait pas convertir `Point` : il manque le [trait](../debuter/glossaire.md#trait).
Ajoutez `use serde::{Deserialize, Serialize};` et
`#[derive(Debug, Serialize, Deserialize)]`.

### Un champ absent du JSON

```rust norun
#[derive(Debug, Deserialize)]
struct Livre {
    titre: String,
    pages: u32,
}

let livre: Livre = serde_json::from_str(r#"{ "titre": "Dune" }"#).unwrap();
```

```text
called `Result::unwrap()` on an `Err` value: Error("missing field `pages`", line: 1, column: 19)
```

Tous les champs sont obligatoires par défaut. Si `pages` peut manquer, déclarez-le
`Option<u32>`, ou ajoutez `#[serde(default)]`.

### Oublier trim avant parse

```rust norun
let ligne = "Alice,15\n";
let morceaux: Vec<&str> = ligne.split(',').collect();
let age: u32 = morceaux[1].parse().unwrap();
```

```text
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

Le dernier morceau vaut `"15\n"` : le retour à la ligne n'est pas un chiffre. Écrivez
`morceaux[1].trim().parse()`, ou découpez avec `lines()`, qui enlève les retours à la
ligne.

## Testez-vous

1. Quelle est la différence entre sérialiser et désérialiser ?

> **Réponse**
>
> Sérialiser transforme une valeur Rust en texte (ou en octets). Désérialiser
> reconstruit la valeur Rust à partir de ce texte.

2. Comment écrire un champ `nom_complet` sous le nom `nomComplet` dans le JSON ?

> **Réponse**
>
> Avec `#[serde(rename = "nomComplet")]` sur le champ, ou
> `#[serde(rename_all = "camelCase")]` sur toute la structure.

3. Quel format choisir pour le fichier de réglages d'un programme, écrit à la main ?

> **Réponse**
>
> Le TOML : il est très lisible, accepte des sections, et c'est le format de
> `Cargo.toml`.

## En résumé

- Sérialiser, c'est transformer des données en texte ; désérialiser, l'inverse.
- Pour un petit CSV, `format!`, `lines()` et `split(',')` suffisent.
- `#[derive(Serialize, Deserialize)]` + `serde_json::to_string_pretty` et
  `serde_json::from_str` : le JSON en deux lignes.
- `#[serde(rename…)]`, `Option` et `#[serde(default)]` adaptent la structure au format.
- La même structure sert pour JSON, CSV (crate `csv`) et TOML (crate `toml`).

Suite : [Rust pour le web](web.md)
