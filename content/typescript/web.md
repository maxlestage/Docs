# TypeScript pour le web

Cette page montre comment TypeScript sécurise le code web : les réponses de `fetch`, la manipulation de la page (le DOM), un serveur Express et un composant React.

> **Objectifs :**
>
> - typer le résultat d'un appel `fetch` et vérifier la réponse ;
> - manipuler les éléments d'une page avec les bons types (`HTMLInputElement`…) ;
> - gérer les éléments absents (`null`) que renvoie `querySelector` ;
> - écrire un petit serveur Express typé ;
> - lire un composant React écrit en TypeScript (TSX).

## Le web en deux mots

Sur le web, un [client](../debuter/glossaire.md#client) (le navigateur, ou un programme)
envoie une requête [HTTP](../debuter/glossaire.md#http) à un
[serveur](../debuter/glossaire.md#serveur), qui renvoie une réponse : une page
[HTML](../debuter/glossaire.md#html), ou des données en JSON. Si ces notions sont
nouvelles, lisez d'abord [le web pour débuter](../debuter/web.md).

TypeScript est partout dans ce monde : côté navigateur, côté serveur avec Node, et dans
les outils modernes (React, Vue, Angular…). Partout, il apporte la même chose : savoir
**à l'avance** quelles données circulent.

## Typer les résultats de fetch

`fetch(url)` envoie une requête et renvoie une `Promise<Response>`. L'objet `Response`
contient :

- `status` : le code HTTP (200 = OK, 404 = introuvable…) ;
- `ok` : `true` si le code est entre 200 et 299 ;
- `json()` : une promesse du corps, lu comme du JSON.

Ici, pas de réseau : nous fabriquons les réponses nous-mêmes avec `new Response(...)`,
qui existe dans Node 22 comme dans le navigateur. Le reste du code est exactement celui
que vous écririez avec un vrai `fetch`.

```typescript
const reponse = new Response('{"id": 1, "nom": "Ada"}', {
  status: 200,
  headers: { "Content-Type": "application/json" },
});

console.log(reponse.status, reponse.ok);
console.log(reponse.headers.get("Content-Type"));
const donnees: unknown = await reponse.json();
console.log(donnees);
```

```sortie
200 true
application/json
{ id: 1, nom: 'Ada' }
```

> **Attention :** `json()` renvoie un `Promise<any>`. Comme pour `JSON.parse` (voir
> [JSON et validation des données](formats.md)), rangez le résultat dans `unknown` et
> vérifiez-le : un serveur peut changer, ou renvoyer une erreur.

### Une fonction de chargement bien typée

Voici le schéma complet : vérifier le code HTTP, lire le JSON, valider sa forme, puis
renvoyer un objet du bon type. La fonction `faussesRequetes` remplace le réseau ; dans
un vrai programme, vous écririez `await fetch(url)` à la place.

```typescript
interface Utilisateur {
  id: number;
  nom: string;
}

function estUtilisateur(v: unknown): v is Utilisateur {
  return (
    typeof v === "object" && v !== null &&
    "id" in v && typeof v.id === "number" &&
    "nom" in v && typeof v.nom === "string"
  );
}

// Remplace fetch(url) pour l'exemple : aucune connexion réseau.
async function faussesRequetes(url: string): Promise<Response> {
  if (url === "/api/utilisateurs/1") {
    return new Response('{"id": 1, "nom": "Ada"}', { status: 200 });
  }
  if (url === "/api/utilisateurs/2") {
    return new Response('{"id": "deux"}', { status: 200 });
  }
  return new Response("Introuvable", { status: 404 });
}

async function chargerUtilisateur(id: number): Promise<Utilisateur> {
  const reponse = await faussesRequetes(`/api/utilisateurs/${id}`);
  if (!reponse.ok) {
    throw new Error(`HTTP ${reponse.status}`);
  }
  const donnees: unknown = await reponse.json();
  if (!estUtilisateur(donnees)) {
    throw new Error("réponse inattendue du serveur");
  }
  return donnees;
}

for (const id of [1, 2, 3]) {
  try {
    const u = await chargerUtilisateur(id);
    console.log(`Utilisateur ${u.id} : ${u.nom}`);
  } catch (e) {
    if (e instanceof Error) {
      console.log(`Échec pour ${id} : ${e.message}`);
    }
  }
}
```

```sortie
Utilisateur 1 : Ada
Échec pour 2 : réponse inattendue du serveur
Échec pour 3 : HTTP 404
```

Le reste du programme ne manipule que des `Utilisateur` vérifiés. Les trois cas
d'échec possibles (code HTTP, JSON invalide, mauvaise forme) sont traités au même endroit.

> **Note :** `fetch` ne lève **pas** d'erreur pour un code 404 ou 500 : la requête a
> bien eu lieu. Il faut tester `reponse.ok` vous-même. `fetch` ne lève une erreur que si
> le réseau est injoignable.

### Envoyer des données

Pour envoyer du JSON, on précise la méthode, l'en-tête et le corps :

```typescript norun
interface NouvelleTache {
  titre: string;
  faite: boolean;
}

const tache: NouvelleTache = { titre: "Réviser", faite: false };

const reponse = await fetch("https://exemple.fr/api/taches", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(tache),
});
console.log(reponse.status);
```

```text
201
```

## Le DOM avec des types

Dans le navigateur, la page est représentée par le **DOM** (*Document Object Model*) :
un arbre d'objets, un par balise. TypeScript connaît tous ces objets. Chaque sorte de
balise a son type :

| Balise | Type TypeScript | Propriétés utiles |
|--------|-----------------|-------------------|
| `input` | `HTMLInputElement` | `value`, `checked` |
| `button` | `HTMLButtonElement` | `disabled` |
| `a` | `HTMLAnchorElement` | `href` |
| `img` | `HTMLImageElement` | `src`, `alt` |
| n'importe laquelle | `HTMLElement` | `textContent`, `style`, `classList` |

Prenons cette page :

```html norun
<input id="nom" placeholder="Votre nom">
<button>Saluer</button>
<p id="message"></p>
```

### querySelector peut renvoyer null

`document.querySelector("button")` cherche le premier élément qui correspond. Mais il
peut ne rien trouver : son type de retour est donc `HTMLButtonElement | null`. TypeScript
vous oblige à traiter le cas `null`.

Avec un nom de balise (`"button"`), TypeScript devine le type exact. Avec un sélecteur
CSS comme `"#nom"`, il ne peut pas deviner : il donne un simple `Element`. Précisez alors
le type entre chevrons, comme pour un [générique](generiques.md) :
`querySelector<HTMLInputElement>("#nom")`.

```typescript norun
const champ = document.querySelector<HTMLInputElement>("#nom");
const bouton = document.querySelector("button");
const sortie = document.getElementById("message");

if (champ === null || bouton === null || sortie === null) {
  throw new Error("Élément manquant dans la page");
}

bouton.addEventListener("click", () => {
  const nom = champ.value.trim();
  sortie.textContent = nom === "" ? "Entrez un nom." : `Bonjour ${nom} !`;
});
```

Après le `if`, TypeScript sait que les trois variables ne sont plus `null`. Dans la
fonction du clic, `champ.value` est un `string` : l'autocomplétion le propose.

Quand on clique avec « Ada » dans le champ, le paragraphe affiche :

```text
Bonjour Ada !
```

### Vérifier le type avec instanceof

Le chevron `<HTMLInputElement>` est une promesse que **vous** faites à TypeScript, sans
vérification. Pour être sûr, testez avec `instanceof` :

```typescript norun
const zone = document.querySelector("#commentaire");
if (zone instanceof HTMLTextAreaElement) {
  console.log(`${zone.value.length} caractères`);
}
```

### Les événements

Le paramètre d'un écouteur d'événement est typé automatiquement : `MouseEvent` pour
`"click"`, `KeyboardEvent` pour `"keydown"`… Inutile de l'écrire.

```typescript norun
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") {
    console.log("Fermeture de la fenêtre");
  }
});
```

> **Astuce :** pour que TypeScript connaisse le DOM, la liste `lib` du `tsconfig.json`
> doit contenir `"dom"` (voir [configurer TypeScript](configuration.md)). C'est le cas par
> défaut quand vous ne précisez pas `lib`.

## Un serveur Express typé

**Express** est la bibliothèque la plus connue pour écrire un serveur web avec Node.
Installez-la avec ses types :

```sh
npm install express
npm install --save-dev @types/express
```

Le paquet `@types/express` décrit les types d'Express : `Request` (la requête reçue)
et `Response` (la réponse à envoyer). Les paramètres de type précisent ce que contient
la requête.

```typescript norun
import express, { type Request, type Response } from "express";

interface Livre {
  id: number;
  titre: string;
}

const livres: Livre[] = [
  { id: 1, titre: "Le Petit Prince" },
  { id: 2, titre: "Les Misérables" },
];

const app = express();
app.use(express.json());

app.get("/livres", (req: Request, res: Response<Livre[]>) => {
  res.json(livres);
});

app.get("/livres/:id", (req: Request<{ id: string }>, res: Response) => {
  const livre = livres.find((l) => l.id === Number(req.params.id));
  if (livre === undefined) {
    res.status(404).json({ erreur: "livre introuvable" });
    return;
  }
  res.json(livre);
});

app.listen(3000, () => {
  console.log("Serveur prêt sur http://localhost:3000");
});
```

Le serveur lancé, on l'interroge depuis un autre terminal :

```sh
curl http://localhost:3000/livres/1
curl http://localhost:3000/livres/9
```

```text
{"id":1,"titre":"Le Petit Prince"}
{"erreur":"livre introuvable"}
```

- `Response<Livre[]>` oblige `res.json(...)` à recevoir une liste de livres ;
- `Request<{ id: string }>` déclare le paramètre `:id` de l'adresse. C'est toujours un
  texte : convertissez-le avec `Number`.

> **Attention :** `req.body` (les données envoyées par le client) n'est **pas** vérifié
> par TypeScript. Validez-le comme n'importe quelle donnée extérieure, par exemple avec zod.

## Les frameworks front-end

Les frameworks modernes (React, Vue, Angular, Svelte) prennent tous en charge
TypeScript. Avec **React**, les composants s'écrivent dans des fichiers `.tsx` : du
TypeScript qui peut contenir des balises (le JSX).

Le principal apport : les **props** (les paramètres d'un composant) sont décrites par
une `interface`. Un oubli ou une faute de frappe est signalé dès l'écriture.

```tsx norun
import { useState } from "react";

interface CompteurProps {
  titre: string;
  depart?: number;
}

export function Compteur({ titre, depart = 0 }: CompteurProps) {
  const [valeur, setValeur] = useState<number>(depart);

  return (
    <div>
      <h2>{titre}</h2>
      <p>Valeur : {valeur}</p>
      <button onClick={() => setValeur(valeur + 1)}>+1</button>
    </div>
  );
}

// Utilisation ailleurs :
// <Compteur titre="Visiteurs" depart={10} />   correct
// <Compteur depart="10" />                     refusé : titre manque, depart n'est pas un nombre
```

- `CompteurProps` décrit les props ; `depart?` est facultatif.
- `useState<number>` dit que l'état est un nombre : `setValeur("dix")` serait refusé.

Pour démarrer un projet React en TypeScript, l'outil Vite crée tout le nécessaire :

```sh
npm create vite@latest mon-app -- --template react-ts
```

## Erreurs fréquentes

### Oublier que querySelector peut renvoyer null

```typescript norun
const champ = document.querySelector("#nom");
console.log(champ.value);
```

```text
main.ts(2,13): error TS18047: 'champ' is possibly 'null'.
main.ts(2,19): error TS2339: Property 'value' does not exist on type 'Element'.
```

Deux problèmes : l'élément peut être absent, et un `Element` n'a pas de `value`.
Écrivez `document.querySelector<HTMLInputElement>("#nom")`, puis vérifiez
`if (champ !== null)` avant de lire `champ.value`.

### Vérifier null sans préciser le type

```typescript norun
const champ = document.querySelector("#nom");
if (champ !== null) {
  console.log(champ.value);
}
```

```text
main.ts(3,21): error TS2339: Property 'value' does not exist on type 'Element'.
```

Le `null` est réglé, mais `"#nom"` peut désigner n'importe quelle balise. Indiquez le
type avec `<HTMLInputElement>` ou testez `champ instanceof HTMLInputElement`.

### Oublier await devant json()

```typescript norun
interface Utilisateur {
  id: number;
  nom: string;
}
const reponse = new Response('{"id": 1, "nom": "Ada"}');
const u: Utilisateur = reponse.json();
console.log(u.nom);
```

```text
main.ts(6,7): error TS2739: Type 'Promise<any>' is missing the following properties from type 'Utilisateur': id, nom
```

`json()` renvoie une promesse : écrivez `await reponse.json()`. Et rangez plutôt le
résultat dans `unknown`, pour le valider.

## Testez-vous

**Question 1 :** quel est le type de `document.querySelector("img")` ?

> **Réponse**
>
> `HTMLImageElement | null` : TypeScript reconnaît le nom de balise, mais l'image peut
> être absente de la page.

**Question 2 :** un serveur répond avec le code 500. Est-ce que `await fetch(url)` lève
une erreur ?

> **Réponse**
>
> Non. La promesse est tenue, avec `reponse.ok` à `false` et `reponse.status` à `500`.
> C'est à vous de tester `reponse.ok`.

**Question 3 :** écrivez une fonction `lireTotal(reponse: Response): Promise<number>` qui
lit un JSON `{ "total": 42 }` et renvoie le nombre, ou lève une erreur si la forme est
mauvaise.

> **Réponse**
>
> ```typescript
> async function lireTotal(reponse: Response): Promise<number> {
>   const donnees: unknown = await reponse.json();
>   if (
>     typeof donnees === "object" && donnees !== null &&
>     "total" in donnees && typeof donnees.total === "number"
>   ) {
>     return donnees.total;
>   }
>   throw new Error("forme inattendue");
> }
>
> console.log(await lireTotal(new Response('{ "total": 42 }')));
> ```
>
> ```sortie
> 42
> ```

## En résumé

- `fetch` renvoie une `Promise<Response>` ; testez `reponse.ok`, puis validez `json()`
  rangé dans `unknown`.
- Chaque balise a son type DOM : `HTMLInputElement`, `HTMLButtonElement`…
- `querySelector` peut renvoyer `null` : vérifiez avant d'utiliser l'élément.
- `querySelector<HTMLInputElement>(...)` précise le type ; `instanceof` le vérifie.
- Express (avec `@types/express`) et React (fichiers `.tsx`) s'utilisent pleinement en
  TypeScript.

Suite : [tester son code](tests.md). Comparez avec
[le web en Ruby](../ruby/web.md).
