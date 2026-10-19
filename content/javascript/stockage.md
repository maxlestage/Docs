# Garder des données dans le navigateur

Cette page montre comment une page web garde des données d'une visite à l'autre avec `localStorage`, `sessionStorage` et IndexedDB, et ce qu'il ne faut jamais y ranger.

> **Objectifs :**
>
> - vous saurez enregistrer, relire et effacer des données avec `localStorage` ;
> - vous saurez ranger des objets et des listes grâce à `JSON.stringify` et `JSON.parse` ;
> - vous saurez choisir entre `localStorage` et `sessionStorage` ;
> - vous connaîtrez les limites du stockage et le rôle d'IndexedDB ;
> - vous saurez ce qu'il ne faut **jamais** stocker dans le navigateur.

## Pourquoi garder des données

Les [variables](../debuter/glossaire.md#variable) d'une page sont écrites sur un
**tableau blanc** : quand vous rechargez la page, tout est effacé. Le compteur revient
à 0, la liste de tâches est vide.

Le navigateur offre aussi un **carnet** : ce que vous y écrivez reste, même après avoir
fermé le navigateur et rallumé l'ordinateur.

```text
  Variables (tableau blanc)          localStorage (carnet)
  ┌──────────────────────┐           ┌──────────────────────┐
  │ compteur = 3         │           │ "compteur" → "3"     │
  └──────────────────────┘           └──────────────────────┘
       rechargement ▼                     rechargement ▼
  ┌──────────────────────┐           ┌──────────────────────┐
  │ compteur = 0 (perdu) │           │ "compteur" → "3"     │
  └──────────────────────┘           └──────────────────────┘
```

Le chapitre [le DOM](dom.md) en a donné un aperçu. Ici, on prend le temps de tout voir.

> **À retenir :** les variables disparaissent au rechargement. Le stockage du
> navigateur, lui, reste.

## localStorage : des casiers étiquetés

`localStorage` ressemble à un mur de casiers. Chaque casier a une **étiquette** (la
clé) et contient **un texte** (la valeur).

```text
  ┌────────────┬────────────┬────────────┐
  │ "theme"    │ "prenom"   │ "visites"  │   ← les clés (étiquettes)
  │  "sombre"  │  "Léa"     │  "4"       │   ← les valeurs (toujours du texte)
  └────────────┴────────────┴────────────┘
```

Quatre méthodes suffisent :

```javascript norun
localStorage.setItem("theme", "sombre");      // ranger (ou remplacer)
const theme = localStorage.getItem("theme");  // relire : "sombre"
localStorage.removeItem("theme");             // vider un casier
localStorage.clear();                         // vider tous les casiers
```

- `getItem` renvoie `null` si le casier n'existe pas ;
- `setItem` sur une clé existante **remplace** l'ancienne valeur ;
- `localStorage.length` donne le nombre de casiers.

Ce code ne tourne que dans un navigateur : Node n'a pas de `localStorage`. Pour essayer
dans le navigateur, ouvrez la console (`F12`, onglet **Console**) et tapez ces lignes
une à une. L'onglet **Application** (Chrome) ou **Stockage** (Firefox) montre les
casiers.

> **À retenir :** `setItem(cle, texte)` range, `getItem(cle)` relit (ou `null`),
> `removeItem(cle)` efface.

## Essayer l'API dans Node

Pour comprendre et tester sans navigateur, fabriquons un **faux** `localStorage` : une
[classe](../debuter/glossaire.md#classe) qui a les mêmes méthodes. Elle range tout dans
une `Map` (voir [les collections](collections.md)).

```javascript
class FauxStockage {
  #casiers = new Map();

  get length() {
    return this.#casiers.size;
  }
  setItem(cle, valeur) {
    this.#casiers.set(String(cle), String(valeur)); // tout devient texte
  }
  getItem(cle) {
    return this.#casiers.has(cle) ? this.#casiers.get(cle) : null;
  }
  removeItem(cle) {
    this.#casiers.delete(cle);
  }
  clear() {
    this.#casiers.clear();
  }
  key(i) {
    return [...this.#casiers.keys()][i] ?? null;
  }
}

const carnet = new FauxStockage();
carnet.setItem("theme", "clair");
carnet.setItem("prenom", "Léa");
carnet.setItem("theme", "sombre");
console.log(carnet.getItem("theme"), carnet.length);
carnet.removeItem("prenom");
console.log(carnet.getItem("prenom"), carnet.length);
console.log(carnet.key(0));
```

```sortie
sombre 2
null 1
theme
```

Suivons le contenu du carnet ligne par ligne :

| Étape | Instruction | Contenu du carnet | length |
|---|---|---|---|
| 1 | `setItem("theme", "clair")` | theme → "clair" | 1 |
| 2 | `setItem("prenom", "Léa")` | theme → "clair", prenom → "Léa" | 2 |
| 3 | `setItem("theme", "sombre")` | theme → "sombre", prenom → "Léa" | 2 |
| 4 | `removeItem("prenom")` | theme → "sombre" | 1 |

À l'étape 3, `length` reste à 2 : la clé `theme` existait, sa valeur est remplacée.

Essayez : ajoutez `carnet.clear();` puis `console.log(carnet.length, carnet.key(0));`.
Vous obtenez `0 null`.

> **À retenir :** le vrai `localStorage` se comporte comme cette petite classe : une
> table de clés et de textes.

## Tout devient du texte

Le carnet ne garde que des [chaînes de caractères](../debuter/glossaire.md#chaine-de-caracteres).
Si vous rangez un nombre, il est converti en texte. Au retour, ce n'est plus un nombre.

```javascript
// même conversion que setItem : String(valeur)
const range = String(5);
console.log(range, typeof range);
console.log(range + 1);          // collage de textes !
console.log(Number(range) + 1);  // le bon calcul

console.log(String(true));
console.log(String([1, 2, 3]));
console.log(String({ nom: "Léa" }));
```

```sortie
5 string
51
6
true
1,2,3
[object Object]
```

| Valeur rangée | Texte gardé | Problème |
|---|---|---|
| `5` | `"5"` | `"5" + 1` donne `"51"` |
| `true` | `"true"` | `"true"` n'est pas `true` |
| `[1, 2, 3]` | `"1,2,3"` | ce n'est plus un tableau |
| `{ nom: "Léa" }` | `"[object Object]"` | les données sont **perdues** |

Pour un nombre, convertissez au retour avec `Number(...)`. Pour tout le reste, utilisez
JSON (section suivante).

> **À retenir :** ce qui entre dans le carnet en ressort sous forme de texte.

## Ranger des objets avec JSON

[JSON](../debuter/glossaire.md#json) est une façon d'écrire un objet ou une liste sous
forme de texte, puis de le relire. C'est comme démonter un meuble pour le mettre dans un
carton, puis le remonter à l'arrivée (on parle de
[sérialisation](../debuter/glossaire.md#serialisation)).

- `JSON.stringify(valeur)` : objet → texte (on met en carton) ;
- `JSON.parse(texte)` : texte → objet (on remonte).

```javascript
const reglages = { theme: "sombre", taille: 18, sons: false };

const texte = JSON.stringify(reglages);
console.log(texte, typeof texte);

const relu = JSON.parse(texte);
console.log(relu.taille + 2, typeof relu.sons);
```

```sortie
{"theme":"sombre","taille":18,"sons":false} string
20 boolean
```

Les nombres et les booléens reviennent intacts. Voir [JSON et CSV](formats.md) pour
plus de détails.

### Deux fonctions sauver et charger

Écrivez une fois deux petites fonctions, et utilisez-les partout. `charger` reçoit une
**valeur par défaut** pour le cas où le casier est vide, ou abîmé.

```javascript
class FauxStockage {
  #casiers = new Map();
  setItem(cle, valeur) { this.#casiers.set(cle, String(valeur)); }
  getItem(cle) { return this.#casiers.get(cle) ?? null; }
}
const carnet = new FauxStockage();

function sauver(cle, valeur) {
  carnet.setItem(cle, JSON.stringify(valeur));
}

function charger(cle, parDefaut) {
  const texte = carnet.getItem(cle);
  if (texte === null) return parDefaut;   // casier vide
  try {
    return JSON.parse(texte);
  } catch {
    return parDefaut;                     // texte abîmé
  }
}

console.log(charger("taches", []));
sauver("taches", ["pain", "lait"]);
console.log(charger("taches", []));
carnet.setItem("taches", "{pas du json");
console.log(charger("taches", []));
```

```sortie
[]
[ 'pain', 'lait' ]
[]
```

| Appel | `getItem` renvoie | Chemin suivi | Résultat |
|---|---|---|---|
| 1er `charger` | `null` | casier vide | `[]` (défaut) |
| 2e `charger` | `'["pain","lait"]'` | `JSON.parse` réussit | `[ 'pain', 'lait' ]` |
| 3e `charger` | `'{pas du json'` | `JSON.parse` échoue, `catch` | `[]` (défaut) |

Dans le navigateur, remplacez simplement `carnet` par `localStorage`.

> **À retenir :** `JSON.stringify` en rangeant, `JSON.parse` en relisant, et toujours
> une valeur par défaut.

## Un exemple : compter les visites

Simulons trois visites d'une page. À chaque visite, les variables repartent de zéro
(nouvel appel de la fonction), mais le carnet, lui, est le même.

```javascript
class FauxStockage {
  #casiers = new Map();
  setItem(cle, valeur) { this.#casiers.set(cle, String(valeur)); }
  getItem(cle) { return this.#casiers.get(cle) ?? null; }
}
const carnet = new FauxStockage();  // survit aux visites

function visite() {
  const avant = Number(carnet.getItem("visites") ?? "0");
  const visites = avant + 1;
  carnet.setItem("visites", visites);
  console.log(visites === 1 ? "Bienvenue !" : `Visite numéro ${visites}`);
}

visite();
visite();
visite();
```

```sortie
Bienvenue !
Visite numéro 2
Visite numéro 3
```

| Visite | `getItem("visites")` | avant | visites | Carnet après |
|---|---|---|---|---|
| 1 | `null` → `"0"` | 0 | 1 | "1" |
| 2 | `"1"` | 1 | 2 | "2" |
| 3 | `"2"` | 2 | 3 | "3" |

Essayez : retirez `Number(...)` (gardez `carnet.getItem("visites") ?? "0"`). La
première visite affiche `Visite numéro 01`, la troisième `Visite numéro 0111` : on
colle des textes au lieu d'ajouter.

> **À retenir :** lisez, calculez, réécrivez. Et convertissez le texte avant de
> calculer.

## sessionStorage : le carnet d'un seul onglet

`sessionStorage` a **exactement** les mêmes méthodes. La différence est sa durée de
vie : il est effacé quand vous fermez l'onglet. C'est un post-it collé sur l'onglet,
alors que `localStorage` est un carnet rangé dans le tiroir.

```javascript norun
sessionStorage.setItem("etape", "2");         // l'étape d'un formulaire en cours
console.log(sessionStorage.getItem("etape")); // "2", même après un rechargement
```

| | `localStorage` | `sessionStorage` |
|---|---|---|
| Survit au rechargement | oui | oui |
| Survit à la fermeture de l'onglet | oui | **non** |
| Partagé entre onglets du même site | oui | non (un carnet par onglet) |
| Usage typique | thème, langue, brouillon | étape d'un formulaire, filtre |

Les deux fonctions `sauver` et `charger` marchent avec l'un comme avec l'autre.

> **À retenir :** même API ; `sessionStorage` vit le temps d'un onglet,
> `localStorage` reste.

## Les limites

Le carnet est pratique, mais petit et simple.

- **La taille** : environ 5 Mo par site, selon le navigateur. Au-delà, `setItem`
  lève une erreur.
- **Seulement du texte**, d'où JSON.
- **Un carnet par site** : un site ne voit pas le carnet d'un autre. « Site » veut dire
  ici le protocole, le nom et le port : `http://localhost:3000` et
  `http://localhost:5173` ont deux carnets différents.
- **L'utilisateur décide** : il peut tout effacer (« effacer les données de
  navigation »), et la navigation privée vide tout à la fermeture.
- **Il est synchrone** : la page attend pendant la lecture. Avec de gros textes, elle
  peut ralentir.

Pour estimer la place prise, mesurez la longueur du texte JSON :

```javascript
const taches = Array.from({ length: 1000 }, (_, i) => ({ texte: `Tâche ${i}`, faite: false }));
const texte = JSON.stringify(taches);
console.log(texte.length, "caractères");
console.log((texte.length / 5_000_000 * 100).toFixed(1), "% de 5 millions");
```

```sortie
35891 caractères
0.7 % de 5 millions
```

Mille tâches tiennent largement. Des photos, non.

Quand le carnet est plein, le navigateur lève une erreur (ici dans Chrome) :

```text
Uncaught DOMException: Failed to execute 'setItem' on 'Storage': Setting the value of 'photos' exceeded the quota.
```

> **À retenir :** `localStorage` convient à de petites données (réglages, brouillons,
> petites listes). Pour beaucoup de données, utilisez IndexedDB.

## IndexedDB : une vraie petite base

IndexedDB est une **base de données** dans le navigateur (voir
[les bases de données](../debuter/bases-de-donnees.md)). Elle range des **objets**
(pas seulement du texte), en grande quantité, et ne bloque pas la page. En échange, son
API est plus longue à écrire. C'est l'entrepôt, à côté du petit carnet.

```text
 base "notes"
 └── magasin "taches"   (comme une table)
      ├── { id: 1, texte: "pain", faite: false }
      └── { id: 2, texte: "lait", faite: true }
```

### Ouvrir la base

On ouvre la base par son nom et sa **version**. La première fois (ou quand la version
augmente), l'événement `upgradeneeded` permet de créer les **magasins**.

```javascript norun
function ouvrir() {
  return new Promise((resolve, reject) => {
    const demande = indexedDB.open("notes", 1);       // nom, version

    demande.onupgradeneeded = () => {
      const base = demande.result;
      base.createObjectStore("taches", { keyPath: "id" }); // la clé est le champ id
    };
    demande.onsuccess = () => resolve(demande.result);
    demande.onerror = () => reject(demande.error);
  });
}
```

IndexedDB prévient par des **événements** (`onsuccess`, `onerror`). On l'enveloppe
dans une [promesse](../debuter/glossaire.md#promesse) pour pouvoir écrire `await`
(voir [l'asynchrone](asynchrone.md)).

### Ajouter et lire

Chaque lecture ou écriture passe par une **transaction** : une petite commande qui
réussit en entier ou pas du tout.

```javascript norun
function enregistrer(base, tache) {
  return new Promise((resolve, reject) => {
    const tx = base.transaction("taches", "readwrite");
    tx.objectStore("taches").put(tache);       // ajoute ou remplace
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

function lire(base, id) {
  return new Promise((resolve, reject) => {
    const demande = base.transaction("taches").objectStore("taches").get(id);
    demande.onsuccess = () => resolve(demande.result);  // undefined si absent
    demande.onerror = () => reject(demande.error);
  });
}

const base = await ouvrir();
await enregistrer(base, { id: 1, texte: "pain", faite: false });
console.log(await lire(base, 1));   // { id: 1, texte: "pain", faite: false }
```

Remarquez : pas de `JSON.stringify` ici. IndexedDB range les objets tels quels.

> **À retenir :** IndexedDB = beaucoup de données, des objets, sans bloquer la page.
> Plus de code : enveloppez-le dans quelques fonctions à promesse.

## Ce qu'il ne faut jamais y ranger

Le stockage du navigateur est une **carte postale**, pas un coffre-fort :

- tout script qui tourne sur la page peut le lire, y compris un script piégé (une
  attaque XSS, voir [la sécurité](../debuter/securite.md)) ;
- toute personne qui utilise l'ordinateur peut l'ouvrir avec `F12` ;
- rien n'y est chiffré.

N'y rangez **jamais** :

- un mot de passe, même « temporairement » ;
- un numéro de carte bancaire ;
- un jeton de connexion qui donne accès à un compte (mieux : un cookie `HttpOnly`,
  posé par le [serveur](../debuter/glossaire.md#serveur), que JavaScript ne peut pas
  lire) ;
- des données personnelles sensibles (santé, papiers d'identité).

Rangez-y sans crainte : le thème, la langue, un brouillon, l'état d'un jeu, une liste
de courses.

> **À retenir :** si vous seriez gêné de le voir écrit sur une carte postale, ne le
> mettez pas dans `localStorage`.

## Erreurs fréquentes

### Relire un casier vide

```javascript norun
const taches = JSON.parse(localStorage.getItem("taches"));
console.log(taches.length);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'length')
```

À la première visite, le casier n'existe pas : `getItem` renvoie `null`, et
`JSON.parse(null)` donne `null`. Prévoyez une valeur par défaut :
`JSON.parse(localStorage.getItem("taches") ?? "[]")`, ou la fonction `charger`.

### Ranger du texte qui n'est pas du JSON

```javascript norun
localStorage.setItem("reglages", "{theme: sombre}");   // écrit à la main
const reglages = JSON.parse(localStorage.getItem("reglages"));
```

```text
Uncaught SyntaxError: Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

En JSON, les clés et les textes sont entre guillemets doubles. Ne fabriquez pas le
texte à la main : passez toujours par `JSON.stringify`, et entourez `JSON.parse` d'un
`try...catch`.

### Calculer avec un nombre relu

```javascript norun
localStorage.setItem("score", 10);
const score = localStorage.getItem("score") + 5;
console.log(score);
```

```text
105
```

Pas d'erreur, mais un faux résultat : `"10" + 5` colle les textes. Écrivez
`Number(localStorage.getItem("score")) + 5`.

## Testez-vous

**Question 1 :** que renvoie `localStorage.getItem("absent")` si ce casier n'a jamais
été rempli ?

> **Réponse**
>
> `null`. Il faut prévoir ce cas, par exemple avec `?? "[]"` avant `JSON.parse`.

**Question 2 :** un utilisateur ferme son onglet puis rouvre le site. Que reste-t-il
dans `localStorage` ? Et dans `sessionStorage` ?

> **Réponse**
>
> `localStorage` a tout gardé. `sessionStorage` est vide : il ne vit que le temps d'un
> onglet.

**Question 3 :** avec le `FauxStockage` (ou une `Map`), rangez la liste
`[3, 1, 2]` sous la clé `"notes"`, relisez-la, et affichez sa somme.

> **Réponse**
>
> ```javascript
> const carnet = new Map();
> carnet.set("notes", JSON.stringify([3, 1, 2]));
>
> const notes = JSON.parse(carnet.get("notes") ?? "[]");
> console.log(notes.reduce((a, b) => a + b, 0));
> ```
>
> ```sortie
> 6
> ```

## En résumé

- `localStorage` garde des textes par clé, même après fermeture : `setItem`, `getItem`, `removeItem`.
- Tout devient texte : `JSON.stringify` pour ranger, `JSON.parse` pour relire, `Number` pour un nombre.
- `getItem` renvoie `null` pour un casier vide : prévoyez une valeur par défaut.
- `sessionStorage` a la même API mais vit le temps d'un onglet.
- Pour beaucoup de données ou des objets, IndexedDB ; jamais de secret dans le navigateur.

Suite : [lire et écrire des fichiers avec Node](fichiers.md)
