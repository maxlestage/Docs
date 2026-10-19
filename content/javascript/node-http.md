# Un serveur web avec node:http

Cette page montre comment écrire un petit serveur web avec le module `node:http` de Node : répondre selon la méthode et l'adresse, envoyer du JSON, choisir le bon code de statut et lire le corps d'une requête.

> **Objectifs :**
>
> - vous saurez créer un serveur avec `createServer` et le tester depuis le même script ;
> - vous saurez lire la méthode, l'adresse et les paramètres d'une requête ;
> - vous saurez organiser des routes selon la méthode et le chemin ;
> - vous saurez répondre en JSON avec le bon code de statut ;
> - vous saurez lire le corps d'une requête `POST`.

## Un serveur, c'est un guichet

Un [serveur](../debuter/glossaire.md#serveur) est un programme qui attend des
**requêtes** et renvoie des **réponses**, comme un guichet de gare. Le
[client](../debuter/glossaire.md#client) (un navigateur, `fetch`, `curl`) se présente
au guichet et demande quelque chose. Le guichetier lit la demande, puis répond.

```text
   client                                  serveur (votre code)
 ┌─────────┐   requête : GET /bonjour    ┌──────────────────────┐
 │  fetch  │ ──────────────────────────▶ │ (req, res) => { … }  │
 │         │ ◀────────────────────────── │                      │
 └─────────┘   réponse : 200 + "Salut"   └──────────────────────┘
```

Le chapitre [JavaScript pour le web](web.md) a présenté [HTTP](../debuter/glossaire.md#http),
`fetch` et un serveur minimal. Ici, on construit un vrai petit serveur, pas à pas.

> **À retenir :** une requête entre, votre fonction la lit, et écrit une réponse.

## Le premier serveur

Un serveur normal tourne **sans fin**, en attendant des visites. Pour que les exemples
de cette page s'arrêtent tout seuls, chacun suit le même plan :

1. créer le serveur ;
2. le lancer sur le **port 0** : le système choisit un port libre ;
3. l'appeler avec `fetch` depuis le même script ;
4. afficher les réponses ;
5. le fermer avec `close()`.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const serveur = createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
  res.end("Bonjour depuis Node !");
});

serveur.listen(0);                  // 0 : un port libre, au hasard
await once(serveur, "listening");   // attendre que le serveur soit prêt
const adresse = `http://localhost:${serveur.address().port}`;

const reponse = await fetch(adresse);
console.log(reponse.status);
console.log(await reponse.text());

serveur.close();
```

```sortie
200
Bonjour depuis Node !
```

Le port change à chaque lancement : c'est pourquoi on ne l'affiche jamais.

| Étape | Qui agit | Ce qui se passe |
|---|---|---|
| 1 | script | `createServer` prépare le guichet (rien n'écoute encore) |
| 2 | script | `listen(0)` ouvre le guichet sur un port libre |
| 3 | script | `await once(...)` attend que le guichet soit ouvert |
| 4 | client | `fetch(adresse)` envoie `GET /` |
| 5 | serveur | la fonction `(req, res)` est appelée avec la requête |
| 6 | serveur | `writeHead(200, …)` choisit le statut, `end(…)` envoie le texte |
| 7 | client | `reponse.status` vaut `200`, `reponse.text()` lit le corps |
| 8 | script | `close()` ferme le guichet ; le programme se termine |

La fonction donnée à `createServer` est appelée **une fois par requête**. Elle reçoit
deux objets :

- `req` (*request*, la requête) : ce que le client demande ;
- `res` (*response*, la réponse) : le formulaire de réponse à remplir.

Essayez : remplacez `200` par `418` dans `writeHead`. `reponse.status` affiche `418`
(« je suis une théière », un code pour rire, mais réel).

> **Note :** pour un vrai serveur, écrivez `serveur.listen(3000)` sans `close()`, puis
> ouvrez `http://localhost:3000` dans le navigateur. Arrêtez-le avec `Ctrl`+`C`.

> **À retenir :** `createServer(fonction)` crée le serveur, `listen` l'ouvre, `res.end`
> envoie la réponse. Sans `res.end`, le client attend pour toujours.

## Lire la requête

L'objet `req` contient :

- `req.method` : la méthode (`"GET"`, `"POST"`…) ;
- `req.url` : le chemin et les paramètres, par exemple `"/livres?page=2"` ;
- `req.headers` : les en-têtes, avec des noms en minuscules.

Pour découper `req.url`, on la donne à `new URL` (voir [le web](web.md#construire-une-url)),
avec une adresse de base quelconque :

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const serveur = createServer((req, res) => {
  const url = new URL(req.url, "http://localhost");
  const nom = url.searchParams.get("nom") ?? "inconnu";
  res.end(`${req.method} ${url.pathname} nom=${nom}`);
});

serveur.listen(0);
await once(serveur, "listening");
const adresse = `http://localhost:${serveur.address().port}`;

console.log(await (await fetch(`${adresse}/salut?nom=Léa`)).text());
console.log(await (await fetch(`${adresse}/salut`)).text());
console.log(await (await fetch(`${adresse}/`, { method: "DELETE" })).text());

serveur.close();
```

```sortie
GET /salut nom=Léa
GET /salut nom=inconnu
DELETE / nom=inconnu
```

| Requête envoyée | `req.method` | `url.pathname` | `searchParams.get("nom")` |
|---|---|---|---|
| `/salut?nom=Léa` | `GET` | `/salut` | `"Léa"` |
| `/salut` | `GET` | `/salut` | `null` → `"inconnu"` |
| `/` avec `DELETE` | `DELETE` | `/` | `null` → `"inconnu"` |

`fetch` envoie `GET` par défaut. L'option `method` en choisit une autre.

> **À retenir :** `req.method` dit **quoi faire**, `url.pathname` dit **sur quoi**,
> `url.searchParams` donne les petits réglages après le `?`.

## Des routes : méthode + chemin

Une **route**, c'est une méthode et un chemin, reliés à une fonction. Comme les
panneaux d'une administration : « Dépôt de dossier → guichet 1 », « Retrait → guichet
2 ». Le plus simple : coller les deux dans une seule clé, puis chercher dans un objet.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const routes = {
  "GET /": () => "Accueil",
  "GET /contact": () => "Écrivez-nous",
  "POST /contact": () => "Message reçu",
};

const serveur = createServer((req, res) => {
  const { pathname } = new URL(req.url, "http://localhost");
  const cle = `${req.method} ${pathname}`;
  const action = routes[cle];
  if (action) {
    res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
    res.end(action());
  } else {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Introuvable");
  }
});

serveur.listen(0);
await once(serveur, "listening");
const adresse = `http://localhost:${serveur.address().port}`;

for (const [methode, chemin] of [["GET", "/"], ["GET", "/contact"], ["POST", "/contact"], ["GET", "/prix"]]) {
  const reponse = await fetch(adresse + chemin, { method: methode });
  console.log(methode, chemin, "→", reponse.status, await reponse.text());
}

serveur.close();
```

```sortie
GET / → 200 Accueil
GET /contact → 200 Écrivez-nous
POST /contact → 200 Message reçu
GET /prix → 404 Introuvable
```

| Requête | `cle` | `routes[cle]` | Statut |
|---|---|---|---|
| 1 | `"GET /"` | trouvée | 200 |
| 2 | `"GET /contact"` | trouvée | 200 |
| 3 | `"POST /contact"` | trouvée (autre fonction !) | 200 |
| 4 | `"GET /prix"` | `undefined` | 404 |

Le même chemin `/contact` mène à deux fonctions : c'est la méthode qui fait la
différence.

Essayez : ajoutez la route `"GET /prix": () => "Gratuit"`. La dernière ligne devient
`GET /prix → 200 Gratuit`.

> **À retenir :** une route = méthode + chemin. Si aucune route ne correspond,
> répondez 404.

## Répondre en JSON

Une API répond presque toujours en [JSON](../debuter/glossaire.md#json). `res.end`
n'accepte que du texte : on convertit avec `JSON.stringify`, et on prévient le client
avec l'en-tête `Content-Type: application/json`. Une petite fonction évite de le
répéter :

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

function envoyerJson(res, statut, donnees) {
  res.writeHead(statut, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(donnees));
}

const fruits = ["pomme", "poire", "kiwi"];

const serveur = createServer((req, res) => {
  envoyerJson(res, 200, { total: fruits.length, fruits });
});

serveur.listen(0);
await once(serveur, "listening");

const reponse = await fetch(`http://localhost:${serveur.address().port}/fruits`);
console.log(reponse.headers.get("content-type"));
const donnees = await reponse.json();
console.log(donnees.total, donnees.fruits[2]);

serveur.close();
```

```sortie
application/json; charset=utf-8
3 kiwi
```

```text
 serveur : { total: 3, fruits: [...] }   (objet)
              │ JSON.stringify
              ▼
 réseau  : '{"total":3,"fruits":["pomme","poire","kiwi"]}'   (texte)
              │ reponse.json()
              ▼
 client  : { total: 3, fruits: [...] }   (objet)
```

> **À retenir :** objet → `JSON.stringify` → réseau → `reponse.json()` → objet. Et
> l'en-tête `Content-Type: application/json`.

## Les codes de statut

Le code de statut est la **première chose** que lit le client. C'est un nombre à trois
chiffres. Le premier chiffre donne la famille :

| Code | Nom | Quand l'utiliser |
|---|---|---|
| 200 | OK | tout va bien, voici la réponse |
| 201 | Created | une ressource a été créée (après un `POST`) |
| 204 | No Content | c'est fait, rien à renvoyer (après un `DELETE`) |
| 400 | Bad Request | la requête est mal formée (JSON invalide, champ manquant) |
| 404 | Not Found | cette adresse n'existe pas |
| 405 | Method Not Allowed | le chemin existe, pas avec cette méthode |
| 500 | Internal Server Error | le serveur a planté |

- `2xx` : succès ;
- `4xx` : la faute du **client** (il a mal demandé) ;
- `5xx` : la faute du **serveur** (votre code).

> **À retenir :** 2xx = réussi, 4xx = le client s'est trompé, 5xx = le serveur s'est
> trompé. Choisir le bon code aide celui qui utilise votre API.

## Lire le corps d'une requête

Avec `POST`, le client envoie des données dans le **corps** de la requête. Elles
n'arrivent pas d'un bloc : elles arrivent en **morceaux**, comme une longue lettre
envoyée en plusieurs enveloppes. `req` est un flux qu'on parcourt avec `for await`
(voir [l'asynchrone](asynchrone.md)), on range chaque morceau dans un tableau, et on
les recolle à la fin avec `Buffer.concat`.

### Les morceaux pas à pas

Un petit corps arrive souvent en un seul morceau. Pour voir le recollage, donnons à
la fonction `lireCorps` un faux flux de trois morceaux, fabriqué avec `Readable.from` :

```javascript
import { Readable } from "node:stream";

async function lireCorps(req) {
  const morceaux = [];
  for await (const morceau of req) {
    morceaux.push(morceau);
    console.log("reçu :", morceau.toString(), "| morceaux :", morceaux.length);
  }
  return Buffer.concat(morceaux).toString("utf8");
}

const fauxReq = Readable.from([Buffer.from('{"tex'), Buffer.from('te":"pa'), Buffer.from('in"}')]);
const texte = await lireCorps(fauxReq);
console.log(texte);
console.log(JSON.parse(texte).texte);
```

```sortie
reçu : {"tex | morceaux : 1
reçu : te":"pa | morceaux : 2
reçu : in"} | morceaux : 3
{"texte":"pain"}
pain
```

| Tour | Morceau reçu | Ce qu'on a recollé jusque-là |
|---|---|---|
| 1 | `{"tex` | `{"tex` |
| 2 | `te":"pa` | `{"texte":"pa` |
| 3 | `in"}` | `{"texte":"pain"}` |

Aucun morceau seul n'est du JSON valide : il faut **tout** attendre avant
`JSON.parse`.

### Un écho en JSON

Le serveur lit le corps, le convertit, et renvoie ce qu'il a compris. Si le JSON est
invalide, c'est la faute du client : 400.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function lireCorps(req) {
  const morceaux = [];
  for await (const morceau of req) morceaux.push(morceau);
  return Buffer.concat(morceaux).toString("utf8");
}

function envoyerJson(res, statut, donnees) {
  res.writeHead(statut, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(donnees));
}

const serveur = createServer(async (req, res) => {
  let donnees;
  try {
    donnees = JSON.parse(await lireCorps(req));
  } catch {
    return envoyerJson(res, 400, { erreur: "JSON invalide" });
  }
  envoyerJson(res, 200, { recu: donnees });
});

serveur.listen(0);
await once(serveur, "listening");
const adresse = `http://localhost:${serveur.address().port}`;

for (const corps of ['{"age": 12}', "{age: 12}"]) {
  const reponse = await fetch(adresse, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: corps,
  });
  console.log(reponse.status, await reponse.text());
}

serveur.close();
```

```sortie
200 {"recu":{"age":12}}
400 {"erreur":"JSON invalide"}
```

La fonction du serveur est maintenant `async` : elle attend la fin du corps avec
`await`.

Essayez : envoyez `body: ""` (un corps vide). `JSON.parse("")` échoue aussi : la
réponse est `400 {"erreur":"JSON invalide"}`.

> **À retenir :** le corps arrive en morceaux. Recollez-les, **puis** `JSON.parse`
> dans un `try...catch`, et répondez 400 si ça échoue.

## Une petite API de tâches

Assemblons tout : une liste de tâches gardée en mémoire, avec trois routes.

| Route | Rôle | Statuts possibles |
|---|---|---|
| `GET /taches` | la liste | 200 |
| `GET /taches/:id` | une tâche | 200, 404 |
| `POST /taches` | ajouter une tâche | 201, 400 |

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function lireCorps(req) {
  const morceaux = [];
  for await (const morceau of req) morceaux.push(morceau);
  return Buffer.concat(morceaux).toString("utf8");
}

function envoyerJson(res, statut, donnees) {
  res.writeHead(statut, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(donnees));
}

const taches = [{ id: 1, texte: "pain" }];
let prochainId = 2;

async function repondre(req, res) {
  const { pathname } = new URL(req.url, "http://localhost");
  const [, ressource, id] = pathname.split("/");   // "/taches/1" → ["", "taches", "1"]

  if (ressource !== "taches") return envoyerJson(res, 404, { erreur: "introuvable" });

  if (req.method === "GET" && id === undefined) {
    return envoyerJson(res, 200, taches);
  }
  if (req.method === "GET") {
    const tache = taches.find((t) => t.id === Number(id));
    return tache ? envoyerJson(res, 200, tache) : envoyerJson(res, 404, { erreur: "introuvable" });
  }
  if (req.method === "POST" && id === undefined) {
    let corps;
    try {
      corps = JSON.parse(await lireCorps(req));
    } catch {
      return envoyerJson(res, 400, { erreur: "JSON invalide" });
    }
    if (typeof corps.texte !== "string" || corps.texte.trim() === "") {
      return envoyerJson(res, 400, { erreur: "texte manquant" });
    }
    const tache = { id: prochainId++, texte: corps.texte.trim() };
    taches.push(tache);
    return envoyerJson(res, 201, tache);
  }
  envoyerJson(res, 405, { erreur: "méthode non permise" });
}

const serveur = createServer(repondre);
serveur.listen(0);
await once(serveur, "listening");
const adresse = `http://localhost:${serveur.address().port}`;

async function appeler(methode, chemin, corps) {
  const options = { method: methode };
  if (corps !== undefined) options.body = JSON.stringify(corps);
  const reponse = await fetch(adresse + chemin, options);
  console.log(methode, chemin, "→", reponse.status, await reponse.text());
}

await appeler("GET", "/taches");
await appeler("POST", "/taches", { texte: " lait " });
await appeler("POST", "/taches", { titre: "oubli" });
await appeler("GET", "/taches/2");
await appeler("GET", "/taches/9");
await appeler("DELETE", "/taches/1");
await appeler("GET", "/taches");

serveur.close();
```

```sortie
GET /taches → 200 [{"id":1,"texte":"pain"}]
POST /taches → 201 {"id":2,"texte":"lait"}
POST /taches → 400 {"erreur":"texte manquant"}
GET /taches/2 → 200 {"id":2,"texte":"lait"}
GET /taches/9 → 404 {"erreur":"introuvable"}
DELETE /taches/1 → 405 {"erreur":"méthode non permise"}
GET /taches → 200 [{"id":1,"texte":"pain"},{"id":2,"texte":"lait"}]
```

Suivons l'état du serveur après chaque appel :

| Appel | `id` | Branche suivie | Statut | `taches` après | `prochainId` |
|---|---|---|---|---|---|
| `GET /taches` | `undefined` | la liste | 200 | pain | 2 |
| `POST` « lait » | `undefined` | ajout | 201 | pain, lait | 3 |
| `POST` sans texte | `undefined` | `texte` manquant | 400 | pain, lait | 3 |
| `GET /taches/2` | `"2"` | une tâche, trouvée | 200 | pain, lait | 3 |
| `GET /taches/9` | `"9"` | une tâche, absente | 404 | pain, lait | 3 |
| `DELETE /taches/1` | `"1"` | aucune : la fin | 405 | pain, lait | 3 |

Attention : `id` vient de l'adresse, c'est donc un **texte** (`"2"`). On le convertit
avec `Number(id)` avant de comparer avec `===`.

Les données sont dans une variable : elles disparaissent quand le serveur s'arrête.
Pour les garder, il faut un fichier (voir [les fichiers](fichiers.md)) ou une base de
données.

Essayez : ajoutez une branche pour `DELETE` avec un `id` qui retire la tâche
(`taches.splice(index, 1)`) et répond `204` avec `res.writeHead(204); res.end();`.

> **À retenir :** une API, c'est des routes, du JSON, et un code de statut honnête
> pour chaque cas, y compris les erreurs.

## Erreurs fréquentes

### Oublier res.end

```javascript norun
const serveur = createServer((req, res) => {
  res.writeHead(200);
  // pas de res.end()
});
// côté client :
await fetch(adresse, { signal: AbortSignal.timeout(300) });
```

```text
TimeoutError: The operation was aborted due to timeout
```

Le serveur n'a jamais terminé sa réponse : le client attend. Sans le `timeout`, il
attendrait très longtemps. Chaque chemin de votre fonction doit finir par `res.end`
(un `return` aide à ne pas l'oublier).

### Envoyer un objet sans JSON.stringify

```javascript norun
res.writeHead(200, { "Content-Type": "application/json" });
res.end({ message: "Bonjour" });
```

```text
TypeError [ERR_INVALID_ARG_TYPE]: The "chunk" argument must be of type string or an instance of Buffer or Uint8Array. Received an instance of Object
    at write_ (node:_http_outgoing:950:11)
    at ServerResponse.end (node:_http_outgoing:1107:5)
```

`res.end` n'accepte que du texte (ou des octets). Écrivez
`res.end(JSON.stringify({ message: "Bonjour" }))`, ou utilisez `envoyerJson`.

### Répondre deux fois

```javascript norun
const serveur = createServer((req, res) => {
  if (req.url === "/") {
    res.end("Accueil");
  }
  res.writeHead(404);
  res.end("Introuvable");
});
```

```text
Error [ERR_HTTP_HEADERS_SENT]: Cannot write headers after they are sent to the client
    at ServerResponse.writeHead (node:_http_server:354:11)
```

Après `res.end("Accueil")`, la fonction **continue** et tente une seconde réponse. Le
serveur plante. Écrivez `return res.end("Accueil");`, ou utilisez `else`.

## Testez-vous

**Question 1 :** pourquoi les exemples de cette page utilisent-ils `listen(0)` ?

> **Réponse**
>
> Le port `0` demande au système un port libre. Le script ne risque pas l'erreur
> `EADDRINUSE` (port déjà pris), et retrouve le vrai numéro avec
> `serveur.address().port`.

**Question 2 :** un client envoie `POST /taches` avec le corps `{texte: pain}`. Quel
code de statut faut-il renvoyer ?

> **Réponse**
>
> `400 Bad Request` : le corps n'est pas du JSON valide (il manque les guillemets),
> c'est la faute du client.

**Question 3 :** écrivez un serveur qui répond au chemin `/double?n=21` par le JSON
`{"resultat":42}`, appelez-le une fois, affichez la réponse et fermez-le.

> **Réponse**
>
> ```javascript
> import { createServer } from "node:http";
> import { once } from "node:events";
>
> const serveur = createServer((req, res) => {
>   const url = new URL(req.url, "http://localhost");
>   const n = Number(url.searchParams.get("n"));
>   res.writeHead(200, { "Content-Type": "application/json" });
>   res.end(JSON.stringify({ resultat: n * 2 }));
> });
>
> serveur.listen(0);
> await once(serveur, "listening");
> const reponse = await fetch(`http://localhost:${serveur.address().port}/double?n=21`);
> console.log(await reponse.text());
> serveur.close();
> ```
>
> ```sortie
> {"resultat":42}
> ```

## En résumé

- `createServer((req, res) => …)` appelle votre fonction pour chaque requête ; `listen` l'ouvre, `close` la ferme.
- `req.method` et `new URL(req.url, "http://localhost")` disent ce que veut le client.
- Une route = méthode + chemin ; sinon 404 (ou 405 pour une mauvaise méthode).
- Répondez en JSON avec `JSON.stringify` et `Content-Type: application/json`, et un code de statut juste.
- Le corps d'un `POST` arrive en morceaux : recollez-les, puis `JSON.parse` dans un `try...catch` (400 si invalide).

Suite : [tester son code](tests.md)
