# JavaScript pour le web

Cette page montre comment JavaScript dialogue avec le web : demander des données avec `fetch`, écrire un petit serveur avec Node ou Express, et choisir un framework pour l'interface.

> **Objectifs :**
>
> - vous saurez ce que sont une requête et une réponse HTTP ;
> - vous saurez demander des données avec `fetch` et lire leur JSON ;
> - vous saurez écrire un serveur minimal avec `node:http` ;
> - vous saurez définir des routes avec Express ;
> - vous saurez situer React, Vue et Svelte.

## Requêtes et réponses

Le web fonctionne par questions et réponses. Un [client](../debuter/glossaire.md#client)
(votre navigateur, ou un programme) envoie une **requête** à un
[serveur](../debuter/glossaire.md#serveur). Le serveur renvoie une **réponse**. Ce
dialogue suit le protocole [HTTP](../debuter/glossaire.md#http). C'est comme au
restaurant : vous passez commande, la cuisine renvoie un plat, ou un « désolé, plus de
dessert ».

Une requête contient :

- une **méthode** : `GET` (lire), `POST` (créer), `PUT` (remplacer), `DELETE` (supprimer) ;
- une [URL](../debuter/glossaire.md#url) : l'adresse de ce que l'on demande ;
- des **en-têtes** (*headers*) : des informations en plus, comme le format ;
- parfois un **corps** (*body*) : les données envoyées.

Une réponse contient un **code de statut**, des en-têtes et un corps.

| Code | Sens |
|------|------|
| `200` | OK |
| `201` | créé |
| `400` | requête incorrecte |
| `404` | introuvable |
| `500` | erreur du serveur |

Pour en savoir plus sur le fonctionnement du web, voir [le web](../debuter/web.md).

## Demander des données avec fetch

`fetch(url)` envoie une requête et renvoie une **promesse** de réponse (voir
[l'asynchrone](asynchrone.md)). Il existe dans le navigateur **et** dans Node (depuis
la version 18). La réponse arrive en deux temps :

1. `await fetch(url)` attend les en-têtes : vous avez le statut ;
2. `await reponse.json()` attend le corps et le lit comme du JSON.

```javascript norun
const reponse = await fetch("https://api.github.com/repos/nodejs/node");
console.log(reponse.status);

const depot = await reponse.json();
console.log(depot.full_name);
```

```text
200
nodejs/node
```

> **Note :** cet exemple a besoin d'Internet, et le résultat dépend du serveur. Il est
> donc montré sans être exécuté.

### Une réponse fabriquée à la main

Sans réseau, on peut quand même s'entraîner : l'objet `Response` (celui que renvoie
`fetch`) se construit directement. On lui donne un corps et des options.

```javascript
const reponse = new Response(JSON.stringify({ nom: "Alice", score: 42 }), {
  status: 200,
  headers: { "Content-Type": "application/json" },
});

console.log(reponse.status, reponse.ok);
console.log(reponse.headers.get("content-type"));

const donnees = await reponse.json();
console.log(donnees.nom, donnees.score);
```

```sortie
200 true
application/json
Alice 42
```

- `ok` vaut `true` si le statut est entre 200 et 299 ;
- `headers.get` lit un en-tête (le nom ne tient pas compte des majuscules) ;
- `json()` lit le corps. Pour du texte simple, il y a `text()`.

### Vérifier le statut

`fetch` ne lève **pas** d'erreur pour un code 404 ou 500 : pour lui, le serveur a bien
répondu. C'est à vous de vérifier `ok`. Pour essayer sans réseau, la fonction
ci-dessous reçoit la fonction de requête en paramètre : on lui passe une fausse version
qui fabrique des réponses.

```javascript
async function chargerJson(url, requete = fetch) {
  const reponse = await requete(url);
  if (!reponse.ok) {
    throw new Error(`Erreur HTTP ${reponse.status} pour ${url}`);
  }
  return reponse.json();
}

async function fauxFetch(url) {
  if (url === "/api/livres") {
    return new Response(JSON.stringify([{ titre: "Le Petit Prince" }]));
  }
  return new Response("introuvable", { status: 404 });
}

console.log(await chargerJson("/api/livres", fauxFetch));
try {
  await chargerJson("/api/films", fauxFetch);
} catch (erreur) {
  console.log(erreur.message);
}
```

```sortie
[ { titre: 'Le Petit Prince' } ]
Erreur HTTP 404 pour /api/films
```

### Envoyer des données

Pour envoyer des données, donnez un deuxième argument à `fetch` : la méthode, les
en-têtes et le corps. On peut d'abord construire un objet `Request` pour voir ce qui
part :

```javascript
const requete = new Request("https://exemple.fr/api/livres", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ titre: "Candide" }),
});

console.log(requete.method, requete.url);
console.log(await requete.json());
```

```sortie
POST https://exemple.fr/api/livres
{ titre: 'Candide' }
```

Avec `fetch`, on passe les mêmes options :

```javascript norun
const reponse = await fetch("https://exemple.fr/api/livres", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ titre: "Candide" }),
});
```

### Construire une URL

Des paramètres dans une adresse (`?q=…&page=2`) doivent être **encodés** : un espace ou
un `&` dans une valeur casserait l'URL. L'objet `URL` s'en occupe.

```javascript
const url = new URL("https://exemple.fr/recherche");
url.searchParams.set("q", "petit prince & renard");
url.searchParams.set("page", "2");
console.log(url.toString());
console.log(url.searchParams.get("q"));
```

```sortie
https://exemple.fr/recherche?q=petit+prince+%26+renard&page=2
petit prince & renard
```

### Dans le navigateur

Dans une page, on combine `fetch` et [le DOM](dom.md) : on charge les données, puis on
les affiche.

```javascript norun
const liste = document.querySelector("#livres");
const reponse = await fetch("/api/livres");
const livres = await reponse.json();

for (const livre of livres) {
  const li = document.createElement("li");
  li.textContent = livre.titre;
  liste.append(li);
}
```

> **Attention :** par sécurité, un navigateur refuse souvent de lire la réponse d'un
> **autre** site que celui de la page, sauf si ce site l'autorise (règle **CORS**).
> Le message apparaît dans la console du navigateur. Node n'a pas cette limite.

## Un serveur minimal avec node:http

Node peut aussi jouer le rôle du serveur. Le module `node:http` reçoit chaque requête
et vous laisse écrire la réponse. Le serveur tourne sans s'arrêter, en attendant les
visites : c'est pourquoi l'exemple n'est pas exécuté ici.

```javascript norun
// serveur.mjs
import { createServer } from "node:http";

const serveur = createServer((req, res) => {
  if (req.url === "/") {
    res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Bonjour depuis Node !\n");
  } else {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Page introuvable\n");
  }
});

serveur.listen(3000, () => {
  console.log("Serveur prêt sur http://localhost:3000");
});
```

- `req` (la requête) contient `url` et `method` ;
- `res` (la réponse) : `writeHead` choisit le statut et les en-têtes, `end` envoie le
  corps et termine.

Lancez-le, puis ouvrez `http://localhost:3000` dans votre navigateur, ou interrogez-le
depuis un autre terminal avec `curl -i` (`-i` montre aussi les en-têtes) :

```sh
node serveur.mjs
curl -i http://localhost:3000/
```

```text
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Date: Sun, 18 Oct 2026 21:05:47 GMT
Connection: keep-alive
Keep-Alive: timeout=5
Transfer-Encoding: chunked

Bonjour depuis Node !
```

`localhost` désigne votre propre ordinateur, et `3000` est le **port** : le numéro de
la porte où le serveur écoute. Arrêtez le serveur avec `Ctrl`+`C`.

## Express : routes et JSON

Avec `node:http`, chaque adresse demande des `if`. **Express**, la
[bibliothèque](../debuter/glossaire.md#bibliotheque) de serveur la plus connue, organise
le code en **routes** : une méthode, un chemin, une fonction. Installez-la dans votre
projet (voir [les modules](modules.md)) :

```sh
npm install express
```

### Bonjour, Express

```javascript norun
// app.mjs
import express from "express";

const app = express();

app.get("/", (req, res) => {
  res.send("Bonjour depuis Express !");
});

app.listen(3000, () => {
  console.log("Express écoute sur http://localhost:3000");
});
```

```sh
curl http://localhost:3000/
```

```text
Bonjour depuis Express !
```

### Une petite API de livres

Une **API** (voir [le glossaire](../debuter/glossaire.md#api)) est un serveur qui répond
en JSON à des programmes, plutôt qu'en pages à des humains.

```javascript norun
import express from "express";

const app = express();
app.use(express.json());

const livres = [
  { id: 1, titre: "Le Petit Prince" },
  { id: 2, titre: "Les Misérables" },
];

app.get("/livres", (req, res) => {
  res.json(livres);
});

app.get("/livres/:id", (req, res) => {
  const livre = livres.find((l) => l.id === Number(req.params.id));
  if (!livre) {
    return res.status(404).json({ erreur: "livre introuvable" });
  }
  res.json(livre);
});

app.post("/livres", (req, res) => {
  const livre = { id: livres.length + 1, titre: req.body.titre };
  livres.push(livre);
  res.status(201).json(livre);
});

app.listen(3000);
```

- `:id` est un **paramètre de route** : sa valeur arrive dans `req.params.id` (en texte) ;
- `express.json()` lit le corps JSON des requêtes dans `req.body` ;
- `res.json` envoie un objet en JSON, `res.status` choisit le code.

Quelques requêtes avec `curl` :

```sh
curl http://localhost:3000/livres/2
curl http://localhost:3000/livres/9
curl -X POST -H "Content-Type: application/json" -d '{"titre":"Candide"}' http://localhost:3000/livres
```

```text
{"id":2,"titre":"Les Misérables"}
{"erreur":"livre introuvable"}
{"id":3,"titre":"Candide"}
```

## Les frameworks d'interface

Construire une grosse interface avec `createElement` et `addEventListener` devient vite
fastidieux : il faut penser à mettre à jour chaque morceau de la page. Un **framework**
d'interface inverse le problème : vous décrivez à quoi la page doit ressembler pour
des données, et il met l'écran à jour tout seul quand les données changent. Tous
s'installent avec npm et s'utilisent avec un outil de construction comme **Vite**.

### React

**React** (créé par Meta) est le plus répandu. Une interface est faite de
**composants** : des fonctions qui renvoient du **JSX**, un mélange de JavaScript et de
balises. `useState` garde une valeur qui, en changeant, redessine le composant.

```javascript norun
import { useState } from "react";

export function Compteur() {
  const [valeur, setValeur] = useState(0);
  return <button onClick={() => setValeur(valeur + 1)}>{valeur} clics</button>;
}
```

### Vue

**Vue** sépare clairement le gabarit HTML, le script et le style dans un fichier
`.vue`. Il est réputé facile à prendre en main, surtout quand on connaît déjà HTML.
`{{ }}` affiche une valeur, `@click` réagit à un clic.

```html norun
<script setup>
import { ref } from "vue";
const valeur = ref(0);
</script>

<template>
  <button @click="valeur++">{{ valeur }} clics</button>
</template>
```

### Svelte

**Svelte** est un **compilateur** : il transforme vos composants en JavaScript simple
avant l'envoi au navigateur. Le code est très court, proche du HTML de départ, et les
pages produites sont légères.

```html norun
<script>
  let valeur = $state(0);
</script>

<button onclick={() => valeur++}>{valeur} clics</button>
```

> **Astuce :** pour débuter, maîtrisez d'abord [le DOM](dom.md) et `fetch`. Les
> frameworks reposent dessus, et vous comprendrez mieux ce qu'ils font pour vous.

## Erreurs fréquentes

### Lire le corps deux fois

```javascript norun
const reponse = new Response('{"a":1}');
console.log(await reponse.json());
console.log(await reponse.json());
```

```text
{ a: 1 }
TypeError: Body is unusable: Body has already been read
```

Le corps d'une réponse se lit **une seule fois**. Rangez le résultat dans une variable
et réutilisez-la.

### Croire que fetch lève une erreur pour un 404

```javascript norun
const reponse = await fetch("https://exemple.fr/api/absent");
const donnees = await reponse.json();
```

```text
SyntaxError: Unexpected token '<', "<!doctype "... is not valid JSON
```

Le serveur a répondu 404 avec une page HTML, et `json()` échoue à la lire. `fetch` ne
lève pas d'erreur pour un code d'échec : testez `reponse.ok` avant de lire le corps.

### Un serveur absent

```javascript norun
await fetch("http://localhost:3999/");
```

```text
TypeError: fetch failed
  [cause]: Error: connect ECONNREFUSED 127.0.0.1:3999
```

Personne n'écoute sur ce port (*connection refused*, « connexion refusée »). Vérifiez
que le serveur est lancé, et que le port est le bon. Pour une vraie erreur réseau,
`fetch` lève bien une erreur : attrapez-la avec `try...catch`.

### Un port déjà utilisé

```sh
node serveur.mjs
```

```text
Error: listen EADDRINUSE: address already in use 0.0.0.0:3000
```

Un autre programme (souvent votre serveur, lancé dans un autre terminal) occupe déjà le
port 3000. Arrêtez-le avec `Ctrl`+`C`, ou choisissez un autre port.

## Testez-vous

**Question 1 :** `fetch` renvoie une réponse de statut 404. Lève-t-il une erreur ?

> **Réponse**
>
> Non. `fetch` ne lève une erreur que si le serveur ne répond pas du tout. Pour un 404,
> `reponse.ok` vaut `false` : c'est à vous de le vérifier.

**Question 2 :** dans Express, où se trouve la valeur de `:id` pour la route
`/livres/:id` ?

> **Réponse**
>
> Dans `req.params.id`, sous forme de texte. Convertissez-la avec `Number` si besoin.

**Question 3 :** fabriquez une `Response` contenant le JSON `[1, 2, 3]`, puis affichez
la somme des nombres.

> **Réponse**
>
> ```javascript
> const reponse = new Response(JSON.stringify([1, 2, 3]));
> const nombres = await reponse.json();
> console.log(nombres.reduce((a, b) => a + b, 0));
> ```
>
> ```sortie
> 6
> ```

## En résumé

- Le web est un dialogue de requêtes (méthode, URL, en-têtes, corps) et de réponses (statut, corps).
- `await fetch(url)` puis `await reponse.json()` ; vérifiez toujours `reponse.ok`.
- `Response`, `Request` et `URL` se construisent à la main, pratique pour tester sans réseau.
- `node:http` écrit un serveur minimal ; Express l'organise en routes.
- React, Vue et Svelte construisent des interfaces qui se mettent à jour toutes seules.

Suite : [Un serveur web avec node:http](node-http.md)
