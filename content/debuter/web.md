# Comment marche le web

Quand vous ouvrez une page, votre navigateur envoie une demande à un ordinateur lointain qui lui répond : cette page explique ce dialogue, les adresses, les codes de réponse, HTML, CSS, JavaScript, les API, et la place de Ruby, Rust et Rythme.

> **Objectifs :**
>
> - comprendre le rôle du client et du serveur ;
> - lire une URL morceau par morceau ;
> - lire une requête et une réponse HTTP, et ses codes (200, 404…) ;
> - savoir à quoi servent HTML, CSS, JavaScript, les API et JSON ;
> - créer votre première page web et l'ouvrir dans le navigateur.

## Le client et le serveur

Le web, c'est une conversation entre deux ordinateurs :

- le [client](glossaire.md#client) **demande** : c'est votre navigateur
  (Firefox, Chrome, Safari…) ;
- le [serveur](glossaire.md#serveur) **répond** : c'est un ordinateur,
  souvent dans un grand centre de données, qui attend les demandes jour et
  nuit.

Pensez à un restaurant. Vous (le client) passez commande, la cuisine (le
serveur) prépare le plat et vous l'apporte. Vous ne voyez jamais la cuisine :
vous ne voyez que le plat.

Un même serveur répond à des milliers de clients à la fois. Et un serveur
n'est qu'un programme qui tourne sur un ordinateur : vous pouvez en lancer
un sur le vôtre.

## L'adresse : l'URL

Pour demander une page, il faut son adresse : l'[URL](glossaire.md#url).
Décomposons-en une :

```text
https://www.exemple.fr:443/recettes/crepes?portions=4#ingredients
└─┬─┘   └─────┬──────┘ └┬┘└───────┬──────┘ └───┬────┘ └────┬────┘
protocole   machine    port    chemin       paramètres     ancre
```

- le **protocole** dit comment parler : `https` (chiffré) ou `http` ;
- la **machine** (le nom de domaine) dit à quel serveur s'adresser ;
- le **port** est la « porte » du serveur ; on l'omet presque toujours
  (443 pour `https`, 80 pour `http`) ;
- le **chemin** dit quelle page on veut, comme un
  [chemin de fichier](ordinateur.md#les-chemins) ;
- les **paramètres**, après `?`, précisent la demande : `nom=valeur`,
  séparés par `&` ;
- l'**ancre**, après `#`, désigne un endroit dans la page.

Ruby sait découper une URL :

```ruby
require "uri"

adresse = URI("https://www.exemple.fr:443/recettes/crepes?portions=4#ingredients")
puts adresse.scheme
puts adresse.host
puts adresse.port
puts adresse.path
puts adresse.query
puts adresse.fragment
```

```sortie
https
www.exemple.fr
443
/recettes/crepes
portions=4
ingredients
```

En Rust, on peut le faire à la main, avec `split_once` :

```rust
let url = "https://www.exemple.fr/recettes/crepes?portions=4";
let (protocole, reste) = url.split_once("://").unwrap();
let (machine, chemin) = reste.split_once('/').unwrap();
println!("protocole : {protocole}");
println!("machine : {machine}");
println!("chemin : /{chemin}");
```

```sortie
protocole : https
machine : www.exemple.fr
chemin : /recettes/crepes?portions=4
```

## La requête et la réponse HTTP

Le client et le serveur parlent une langue commune : [HTTP](glossaire.md#http).
C'est du **texte**, ligne par ligne.

### La requête

Quand vous tapez `https://example.com/` dans le navigateur, il envoie à peu
près ceci :

```text
GET / HTTP/1.1
Host: example.com
User-Agent: Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0
Accept: text/html
```

- `GET` est la **méthode** : « donne-moi ». `POST` veut dire « voici des
  données » (un formulaire envoyé, par exemple) ;
- `/` est le chemin demandé ;
- les lignes suivantes sont les **en-têtes** : des informations en plus,
  sous la forme `Nom: valeur`.

### La réponse

Le serveur répond avec une ligne d'état, des en-têtes, une ligne vide, puis
le **corps** : la page elle-même.

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 1256
Date: Sun, 18 Oct 2026 20:14:17 GMT

<!doctype html>
<html>
<head><title>Example Domain</title></head>
<body><h1>Example Domain</h1> ...
```

Le navigateur lit ce corps et dessine la page à l'écran.

> **Astuce :** dans votre navigateur, appuyez sur **F12** et ouvrez l'onglet
> **Réseau** (*Network*). Rechargez la page : vous voyez toutes les
> requêtes et toutes les réponses.

Comme HTTP est du texte, un programme peut le lire facilement :

```ruby
requete = "GET /recettes/crepes HTTP/1.1\r\nHost: www.exemple.fr\r\n\r\n"
premiere_ligne = requete.lines.first.strip
methode, chemin, version = premiere_ligne.split(" ")
puts "Méthode : #{methode}"
puts "Chemin : #{chemin}"
puts "Version : #{version}"
```

```sortie
Méthode : GET
Chemin : /recettes/crepes
Version : HTTP/1.1
```

## Les codes d'état

Le nombre de la première ligne de la réponse dit comment cela s'est passé.
Le premier chiffre donne la famille : 2 = succès, 3 = ailleurs, 4 = erreur
du client, 5 = erreur du serveur.

| Code | Nom                   | Sens                                        |
|------|-----------------------|---------------------------------------------|
| 200  | OK                    | tout va bien, voici la page                 |
| 301  | Moved Permanently     | la page a déménagé, voici sa nouvelle adresse |
| 404  | Not Found             | cette page n'existe pas                     |
| 500  | Internal Server Error | le programme du serveur a planté            |

Un `match` en Rust lit un code facilement :

```rust
let reponse = "HTTP/1.1 404 Not Found";
let code: u16 = reponse.split(' ').nth(1).unwrap().parse().unwrap();
let sens = match code {
    200 => "tout va bien",
    301 => "la page a déménagé",
    404 => "page introuvable",
    500 => "le serveur a planté",
    _ => "autre réponse",
};
println!("{code} : {sens}");
```

```sortie
404 : page introuvable
```

Et en Rythme, avec une fonction par cas :

```rythme
fn sens(200) = "tout va bien"
fn sens(404) = "page introuvable"
fn sens(_) = "autre réponse"

each([200, 404, 418], fn(code) = print("{code} : {sens(code)}"))
```

```sortie
200 : tout va bien
404 : page introuvable
418 : autre réponse
```

## HTML, CSS et JavaScript

Une page web est faite de trois langages, chacun avec son rôle. Pensez à
une maison :

- [HTML](glossaire.md#html) est la **structure** : les murs, les pièces.
  Il dit « ceci est un titre, ceci est un paragraphe, ceci est un lien » ;
- **CSS** est la **décoration** : les couleurs, les polices, la place de
  chaque élément ;
- **JavaScript** est l'**électricité** : il rend la page vivante (un bouton
  qui réagit, un menu qui s'ouvre).

Le HTML s'écrit avec des **balises** : `<h1>` ouvre un titre, `</h1>` le
ferme. Le CSS dit comment les afficher :

```text
h1 {
  color: darkblue;
  font-family: sans-serif;
}
```

Ces trois langages tournent **dans le navigateur**, chez le client. Ruby,
Rust et Rythme, eux, tournent en général **sur le serveur**, ou sur votre
ordinateur.

## Les API et JSON

Une page HTML est faite pour les humains. Mais souvent, un programme veut
parler à un autre programme : une application météo demande la température
à un serveur, par exemple. Pour cela, le serveur offre une
[API](glossaire.md#api) : des adresses qui répondent avec des **données**,
pas avec une page.

Ces données sont presque toujours en [JSON](glossaire.md#json), un format
texte simple :

```text
GET /api/meteo?ville=Lyon HTTP/1.1
Host: api.exemple.fr

HTTP/1.1 200 OK
Content-Type: application/json

{"ville": "Lyon", "temperature": 18, "pluie": false}
```

Transformer des données en texte JSON s'appelle la
[sérialisation](glossaire.md#serialisation). En Ruby :

```ruby
require "json"

recette = { "nom" => "Crêpes", "portions" => 4, "vegetarien" => true }
texte = JSON.generate(recette)
puts texte

relu = JSON.parse(texte)
puts relu["portions"] + 1
```

```sortie
{"nom":"Crêpes","portions":4,"vegetarien":true}
5
```

En Rythme, `to_json` et `parse_json` sont déjà là :

```rythme
recette = {nom: "Crêpes", portions: 4, vegetarien: true}
texte = to_json(recette)
print(texte)
match parse_json(texte)
  Ok(r) -> print(r.portions + 1)
  Err(e) -> print("JSON invalide :", e)
end
```

```sortie
{"nom": "Crêpes", "portions": 4, "vegetarien": true}
5
```

En Rust, on utilise le [paquet](glossaire.md#paquet) `serde_json`, à
ajouter au projet : voyez [les formats en Rust](../rust/formats.md).

## Où se placent Ruby, Rust et Rythme

### Ruby et Rails

Ruby est célèbre pour le web grâce à **Ruby on Rails**, un cadriciel (en
anglais *framework*) : une boîte à outils qui fournit déjà le serveur, les
pages, la base de données. GitHub, Shopify et bien d'autres sites l'ont
utilisé.

```sh
gem install rails
rails new mon_site
cd mon_site
bin/rails server
```

Voyez [Ruby et le web](../ruby/web.md).

### Rust et ses paquets : axum

Rust n'a pas de serveur web dans sa bibliothèque standard : on ajoute des
paquets (des *crates*), comme **axum** pour le serveur et **tokio** pour
l'[asynchrone](glossaire.md#async). Le résultat est très rapide et
consomme peu de mémoire. Ce code a besoin de ces paquets, il n'est donc pas
exécuté ici :

```rust norun
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Bonjour depuis Rust !" }));
    let ecoute = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(ecoute, app).await.unwrap();
}
```

Voyez [Rust et le web](../rust/web.md).

### Rythme

Rythme n'est pas fait pour écrire un serveur. Mais il sait lire et écrire
du JSON, écrire des fichiers HTML, et il tourne aussi dans le navigateur
grâce à son *playground*. Voyez [les formats en Rythme](../rythme/formats.md).

## Votre première page web

Pas besoin de serveur pour commencer : un fichier HTML s'ouvre directement
dans le navigateur.

1. Dans votre [éditeur](editeur.md), créez un fichier `index.html`.
2. Tapez ce contenu :

```text
<!doctype html>
<html lang="fr">
  <head>
    <meta charset="utf-8">
    <title>Ma première page</title>
    <style>
      body { font-family: sans-serif; }
      h1 { color: darkblue; }
    </style>
  </head>
  <body>
    <h1>Bonjour le web !</h1>
    <p>Cette page est un simple fichier sur mon ordinateur.</p>
    <a href="https://www.ruby-lang.org/fr/">Le site de Ruby</a>
  </body>
</html>
```

3. Enregistrez, puis double-cliquez sur le fichier (ou glissez-le dans le
   navigateur). L'adresse commence par `file://` : c'est votre disque, pas
   un serveur.

Un programme peut aussi **fabriquer** une page. En Rythme :

```rythme
plats = ["Crêpes", "Gaufres", "Tarte aux pommes"]
lignes = map(plats, fn(p) = "  <li>{p}</li>")
page = "<h1>Mes recettes</h1>\n<ul>\n" + join(lignes, "\n") + "\n</ul>"
write_file("recettes.html", page)
print(unwrap(read_file("recettes.html")))
```

```sortie
<h1>Mes recettes</h1>
<ul>
  <li>Crêpes</li>
  <li>Gaufres</li>
  <li>Tarte aux pommes</li>
</ul>
```

Ouvrez `recettes.html` dans le navigateur : votre liste s'affiche. C'est le
principe de tous les sites : un programme écrit du HTML.

## Erreurs fréquentes

### Mettre une espace dans une URL

```ruby norun
require "uri"
adresse = URI("https://exemple.fr/mes photos")
puts adresse.host
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/uri/rfc3986_parser.rb:130:in `split': bad URI(is not URI?): "https://exemple.fr/mes photos" (URI::InvalidURIError)
	from main.rb:2:in `<main>'
```

Une URL ne contient pas d'espace. On la remplace par `%20` :
`https://exemple.fr/mes%20photos`. Mieux : évitez les espaces dans les
noms de vos fichiers web (`mes-photos`).

### Écrire du JSON avec des apostrophes

```ruby norun
require "json"
donnees = JSON.parse("{'nom': 'Ada'}")
```

```text
/opt/rbenv/versions/3.3.6/lib/ruby/3.3.0/json/common.rb:220:in `parse': unexpected token at '{'nom': 'Ada'}' (JSON::ParserError)
	from main.rb:2:in `<main>'
```

JSON n'accepte que les guillemets droits doubles : `{"nom": "Ada"}`.

### Oublier les guillemets autour des clés JSON

```rythme norun
print(parse_json("\{nom: \"Ada\"\}"))
```

```text
Err("JSON, ligne 1, colonne 2 : on attendait un nom entre guillemets, mais on trouve `n`")
```

En Rythme, `{nom: "Ada"}` est un enregistrement valide, mais en JSON, les
clés sont **toujours** entre guillemets : `{"nom": "Ada"}`. Le plus sûr est
de laisser `to_json` écrire le JSON pour vous.

## Testez-vous

**1.** Dans `https://boutique.fr/panier?code=PROMO`, quel est le chemin, et
quel est le paramètre ?

> **Réponse**
>
> Le chemin est `/panier`, le paramètre est `code=PROMO`.

**2.** Le serveur répond `404`. Qui s'est trompé, et sur quoi ?

> **Réponse**
>
> Le client : il a demandé une page qui n'existe pas (souvent une faute dans
> l'adresse). Une erreur du serveur commencerait par 5, comme `500`.

**3.** Quel langage décide de la couleur d'un titre : HTML, CSS ou
JavaScript ?

> **Réponse**
>
> CSS, la décoration. HTML dit que c'est un titre, JavaScript le rend
> interactif.

## En résumé

- Le client (le navigateur) demande, le serveur répond.
- Une URL se lit en morceaux : protocole, machine, chemin, paramètres, ancre.
- HTTP est du texte : une requête (`GET /…`), une réponse avec un code (200, 301, 404, 500).
- HTML structure, CSS décore, JavaScript anime ; une API renvoie des données en JSON.
- Ruby (Rails) et Rust (axum) écrivent des serveurs ; Rythme lit et écrit JSON et HTML.

Suite : [Qu'est-ce qu'une base de données ?](bases-de-donnees.md)
