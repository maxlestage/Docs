# Rust pour le web

Un site web est un programme qui répond à des requêtes : cette page montre ce qui circule entre le navigateur et le serveur, comment l'écrire avec la seule bibliothèque standard, puis comment aller plus vite avec un framework.

> **Objectifs :**
>
> - comprendre en quelques lignes une requête et une réponse HTTP ;
> - fabriquer une réponse HTTP et écrire un mini serveur avec `TcpListener` ;
> - découvrir trois frameworks : Paquets, axum et actix-web ;
> - appeler une API web avec `reqwest` ;
> - savoir ce qu'est WebAssembly.

## Le web en quelques lignes

Quand vous tapez une adresse dans votre navigateur, il se passe toujours la même chose :

1. le navigateur (le **client**) envoie une **requête** au **serveur** : « donne-moi la
   page `/bonjour` » ;
2. le serveur la lit, prépare une **réponse** (souvent du HTML, ou du JSON) et la
   renvoie ;
3. le navigateur affiche la réponse.

C'est comme au restaurant : vous passez commande (la requête), la cuisine prépare le
plat (le serveur), le serveur vous l'apporte (la réponse). Requête et réponse suivent
une langue commune : le protocole **HTTP**. Et c'est simplement du texte.

Une requête ressemble à ceci :

```text
GET /bonjour HTTP/1.1
Host: localhost:7878
User-Agent: curl/8.5.0
```

La première ligne contient la **méthode** (`GET` pour lire, `POST` pour envoyer des
données), le **chemin** demandé et la version de HTTP. Les lignes suivantes sont les
**en-têtes** : des informations en plus, sous la forme `Nom: valeur`.

## Fabriquer une réponse HTTP

La réponse a la même forme :

```text
HTTP/1.1 200 OK                          ← la ligne de statut
Content-Type: text/html; charset=utf-8   ← les en-têtes
Content-Length: 30
                                         ← une ligne vide
<h1>Bonjour depuis Rust !</h1>           ← le corps
```

- le **code de statut** dit comment ça s'est passé : `200` (tout va bien), `404` (page
  introuvable), `500` (erreur du serveur) ;
- `Content-Type` dit ce que contient le corps : du HTML, du JSON, du texte… ;
- `Content-Length` donne la taille du corps, **en octets** ;
- une **ligne vide** sépare les en-têtes du corps.

En HTTP, chaque ligne se termine par `\r\n` (retour chariot puis saut de ligne). Une
fonction peut fabriquer cette réponse avec `format!` :

```rust
fn reponse_http(statut: &str, type_contenu: &str, corps: &str) -> String {
    format!(
        "HTTP/1.1 {statut}\r\nContent-Type: {type_contenu}\r\nContent-Length: {}\r\n\r\n{corps}",
        corps.len()
    )
}

fn main() {
    let reponse = reponse_http("200 OK", "text/html; charset=utf-8", "<h1>Bonjour depuis Rust !</h1>");
    // On affiche chaque ligne pour voir la structure.
    for ligne in reponse.split("\r\n") {
        println!("|{ligne}");
    }
}
```

```sortie
|HTTP/1.1 200 OK
|Content-Type: text/html; charset=utf-8
|Content-Length: 30
|
|<h1>Bonjour depuis Rust !</h1>
```

Le `|` du début montre où commence chaque ligne : la ligne vide est bien là.

## Choisir la réponse selon le chemin

Un serveur répond différemment selon le chemin demandé : c'est le **routage**. On lit
la première ligne de la requête avec `split_whitespace`, puis un `match` choisit la
réponse :

```rust
fn repondre(requete: &str) -> (u16, String) {
    let premiere_ligne = requete.lines().next().unwrap_or("");
    let mut morceaux = premiere_ligne.split_whitespace();
    let methode = morceaux.next().unwrap_or("");
    let chemin = morceaux.next().unwrap_or("");

    match (methode, chemin) {
        ("GET", "/") => (200, String::from("Accueil")),
        ("GET", "/contact") => (200, String::from("Écrivez-nous !")),
        ("GET", c) if c.starts_with("/bonjour/") => {
            let nom = &c["/bonjour/".len()..];
            (200, format!("Bonjour {nom} !"))
        }
        _ => (404, String::from("Page introuvable")),
    }
}

fn main() {
    for requete in ["GET / HTTP/1.1", "GET /bonjour/Ada HTTP/1.1", "GET /chat HTTP/1.1"] {
        let (code, corps) = repondre(requete);
        println!("{requete} → {code} {corps}");
    }
}
```

```sortie
GET / HTTP/1.1 → 200 Accueil
GET /bonjour/Ada HTTP/1.1 → 200 Bonjour Ada !
GET /chat HTTP/1.1 → 404 Page introuvable
```

Ce `match` est exactement ce que font les frameworks, en plus complet.

## Un mini serveur avec TcpListener

Il reste à recevoir les requêtes du réseau. `std::net::TcpListener` **écoute** sur une
adresse et un **port** (un numéro de porte : ici 7878). Chaque connexion est un flux
dans lequel on lit la requête et on écrit la réponse :

```rust norun
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let ecouteur = TcpListener::bind("127.0.0.1:7878")?;
    println!("Serveur prêt : http://127.0.0.1:7878");

    for connexion in ecouteur.incoming() {
        let mut flux = connexion?;

        // La première ligne de la requête : « GET / HTTP/1.1 »
        let mut premiere_ligne = String::new();
        BufReader::new(&flux).read_line(&mut premiere_ligne)?;
        println!("Requête : {}", premiere_ligne.trim());

        let corps = "<h1>Bonjour depuis Rust !</h1>";
        let reponse = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            corps.len(),
            corps
        );
        flux.write_all(reponse.as_bytes())?;
    }
    Ok(())
}
```

Ce programme ne s'arrête jamais : la boucle `for` attend la connexion suivante. C'est
pourquoi il n'est pas exécuté sur cette page. Lancez-le avec `cargo run`, ouvrez
http://127.0.0.1:7878 dans votre navigateur, puis arrêtez-le avec `Ctrl+C`. Le
[terminal](../debuter/glossaire.md#terminal) affiche :

```text
Serveur prêt : http://127.0.0.1:7878
Requête : GET / HTTP/1.1
Requête : GET /favicon.ico HTTP/1.1
```

Le navigateur demande aussi `/favicon.ico`, la petite icône de l'onglet.

> **Note :** ce serveur traite une requête à la fois. Un vrai serveur en traite des
> milliers en même temps, gère les erreurs, la sécurité, les fichiers… C'est le
> travail d'un framework.

## Les frameworks web

Un **framework** est une [bibliothèque](../debuter/glossaire.md#bibliotheque) qui
fournit tout le squelette d'une application web : le serveur, le routage, la lecture
des paramètres, le JSON. Vous n'écrivez plus que vos **handlers** : les fonctions qui
répondent à chaque route.

La plupart sont **asynchrones** (`async fn`) : ils servent beaucoup de clients en même
temps sans créer un thread par client (voir [Programmation asynchrone](async.md)).

Les exemples de cette section utilisent des crates externes : ils ne tournent pas sur
cette page, mais ils ont été lancés pour vérifier les sorties.

### Paquets : le plus simple

[Paquets](https://github.com/maxlestage/Paquets) se présente comme le framework
full-stack le plus simple pour Rust : un seul import, un `App`, et c'est parti. Il
apporte aussi la base de données, la sécurité et des pages HTML prêtes pour les moteurs
de recherche. Dans `Cargo.toml` :

```toml
[dependencies]
paquets = { git = "https://github.com/maxlestage/Paquets" }
```

```rust norun
use paquets::prelude::*;

/// La page d'accueil, fabriquée sur le serveur.
async fn accueil(_req: Request) -> Page {
    Page::new("Accueil")
        .description("Mon premier site en Rust.")
        .body(h1().text("Bonjour depuis Paquets !"))
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.get("/", accueil);
    app.get("/bonjour/:nom", |req: Request| async move {
        format!("Bonjour {} !", req.param("nom").unwrap_or("toi"))
    });
    app.run() // http://localhost:3000
}
```

Au lancement, le serveur affiche :

```text
📦 Paquets · http://localhost:3000
   database: memory · security: on (10 guards) · islands: none
```

- `App::new()` crée l'application ;
- `app.get("/", accueil)` : pour une requête `GET` sur `/`, appeler `accueil` ;
- `:nom` est un paramètre du chemin, lu avec `req.param("nom")` ;
- `h1().text(…)` fabrique la balise `<h1>…</h1>` ;
- `app.run()` lance le serveur et bloque jusqu'à `Ctrl+C`.

Une structure marquée `#[model]` devient une table de base de données, et une seule
ligne, `app.api::<Note>("/api/notes");`, crée toute une API JSON pour elle. Le tutoriel
du projet vous guide pas à pas.

### axum

axum, de l'équipe de tokio, est très utilisé. Les routes sont rangées dans un `Router` :

```rust norun
use axum::{Router, extract::Path, routing::get};

async fn accueil() -> &'static str {
    "Bonjour depuis axum !"
}

async fn saluer(Path(nom): Path<String>) -> String {
    format!("Bonjour {nom} !")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(accueil))
        .route("/bonjour/{nom}", get(saluer));

    let ecouteur = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(ecouteur, app).await.unwrap();
}
```

Dépendances : `cargo add axum` et `cargo add tokio --features full`. La page
http://127.0.0.1:3000/bonjour/Ada affiche `Bonjour Ada !`.

### actix-web

actix-web est l'un des plus anciens et des plus rapides. Les routes s'écrivent avec
des attributs au-dessus des fonctions :

```rust norun
use actix_web::{App, HttpServer, get, web};

#[get("/")]
async fn accueil() -> &'static str {
    "Bonjour depuis actix-web !"
}

#[get("/bonjour/{nom}")]
async fn saluer(nom: web::Path<String>) -> String {
    format!("Bonjour {nom} !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(accueil).service(saluer))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
```

Dépendance : `cargo add actix-web`.

| Framework | Points forts | Pour qui ? |
|-----------|--------------|-----------|
| Paquets | tout en un : pages, base de données, sécurité, API | débuter, un site complet vite |
| axum | simple, modulaire, très répandu | une API sur mesure |
| actix-web | mûr, très rapide | une API qui doit encaisser beaucoup de trafic |

## Appeler une API avec reqwest

Votre programme peut aussi être le **client** : il envoie une requête à un autre
serveur et lit la réponse, souvent du JSON. La crate `reqwest` s'en charge, et serde
transforme le JSON en structure (voir [JSON, CSV et TOML avec serde](formats.md)) :

```rust norun
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Depot {
    name: String,
    stargazers_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let depot: Depot = client
        .get("https://api.github.com/repos/rust-lang/rust")
        .header("User-Agent", "mon-programme")
        .send()
        .await?
        .json()
        .await?;
    println!("{} a {} étoiles", depot.name, depot.stargazers_count);
    Ok(())
}
```

Dépendances : `cargo add reqwest --features json`, `cargo add tokio --features full`
et `cargo add serde --features derive`. Le programme affiche une ligne comme celle-ci
(le nombre change chaque jour) :

```text
rust a 109000 étoiles
```

Seuls les champs déclarés dans `Depot` sont lus : serde ignore tous les autres.

## WebAssembly : Rust dans le navigateur

Le navigateur n'exécute normalement que du JavaScript. **WebAssembly** (*Wasm*) est un
second langage qu'il comprend : un code compact et rapide, dans lequel Rust peut être
compilé (`rustup target add wasm32-unknown-unknown`). On l'utilise pour les parties
lourdes d'une page (un jeu, un éditeur d'images, des calculs) ou pour écrire toute
l'interface en Rust, avec des outils comme `wasm-bindgen`, Leptos, Yew ou les « îlots »
de Paquets.

## Erreurs fréquentes

### Un main async sans runtime

```rust norun
async fn main() {
    println!("Bonjour");
}
```

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:1:1
  |
1 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

Une fonction `async` a besoin d'un **runtime** pour avancer. Ajoutez `#[tokio::main]`
(ou `#[actix_web::main]`) au-dessus de `main`, et la crate `tokio` au projet.

### Le port est déjà utilisé

Vous lancez le serveur une deuxième fois, alors que le premier tourne encore :

```text
Error: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
```

Un seul programme peut écouter sur un port. Arrêtez l'ancien serveur avec `Ctrl+C`, ou
choisissez un autre port (`127.0.0.1:7879`).

### Compter les lettres au lieu des octets

`Content-Length` est une taille **en octets**. Avec des accents, le nombre de
caractères est plus petit :

```rust
let corps = "Café crème";
println!("caractères : {}", corps.chars().count());
println!("octets     : {}", corps.len());
```

```sortie
caractères : 10
octets     : 12
```

Avec `chars().count()`, le navigateur lit 10 octets et coupe la fin de la page
(`Café crè`). Utilisez toujours `corps.len()`.

## Testez-vous

1. Que sépare la ligne vide dans une réponse HTTP ?

> **Réponse**
>
> Les en-têtes (au-dessus) et le corps de la réponse (en dessous).

2. Que signifie le code de statut `404` ?

> **Réponse**
>
> La page demandée est introuvable. `200` veut dire que tout s'est bien passé, `500`
> qu'une erreur s'est produite sur le serveur.

3. Avec Paquets, comment faire répondre la fonction `accueil` au chemin `/` ?

> **Réponse**
>
> Avec `app.get("/", accueil);`, avant `app.run()`.

## En résumé

- Le web, c'est des requêtes et des réponses HTTP : du texte, avec une première ligne,
  des en-têtes, une ligne vide et un corps.
- `TcpListener` suffit pour un mini serveur, mais un framework fait le vrai travail.
- Paquets (le plus simple, tout en un), axum et actix-web : on déclare des routes et des
  handlers `async`.
- `reqwest` et serde appellent une API et lisent son JSON.
- WebAssembly fait tourner du Rust dans le navigateur.

Pour revoir les bases du web, lisez [le web pour débuter](../debuter/web.md). Suite :
[Écrire du code rapide](performance.md)

Suite : [Rust dans le navigateur avec WebAssembly](wasm.md)
