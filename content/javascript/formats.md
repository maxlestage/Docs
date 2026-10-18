# JSON et CSV

Cette page montre comment transformer des données JavaScript en texte JSON ou CSV, et comment relire ce texte pour retrouver les données.

> **Objectifs :**
>
> - vous saurez écrire du JSON lisible avec `JSON.stringify` ;
> - vous saurez relire du JSON avec `JSON.parse` et réagir à un texte invalide ;
> - vous saurez retrouver des dates avec un *reviver* ;
> - vous saurez lire et écrire un CSV simple à la main ;
> - vous saurez choisir le bon format selon la situation.

## Pourquoi un format ?

Un objet JavaScript vit dans la mémoire du programme. Pour l'enregistrer dans un
[fichier](fichiers.md) ou l'envoyer sur le réseau, il faut le transformer en **texte**,
puis savoir le relire. Cette opération s'appelle la
[sérialisation](../debuter/glossaire.md#serialisation). C'est comme démonter un meuble
pour le transporter : la notice de montage, c'est le format.

Deux formats texte reviennent sans cesse :

- **JSON** pour des données structurées (objets dans des objets, listes) ;
- **CSV** pour des tableaux (des lignes et des colonnes).

## JSON en deux mots

[JSON](../debuter/glossaire.md#json) (*JavaScript Object Notation*) ressemble beaucoup
aux objets et aux tableaux de JavaScript, avec des règles plus strictes :

- les clés sont **toujours** entre guillemets doubles : `"nom"` ;
- les textes aussi : `"Alice"` (jamais d'apostrophes `'…'`) ;
- pas de virgule après le dernier élément ;
- seulement des textes, nombres, `true`, `false`, `null`, tableaux et objets.

```text
{
  "nom": "Alice",
  "age": 30,
  "langages": ["JavaScript", "Python"],
  "adresse": { "ville": "Lyon" },
  "chat": null
}
```

## Écrire du JSON avec JSON.stringify

`JSON.stringify` transforme une valeur en texte JSON, sur une seule ligne.

```javascript
const joueur = { nom: "Alice", score: 42, niveaux: [1, 2, 3] };

const texte = JSON.stringify(joueur);
console.log(texte);
console.log(typeof texte);
```

```sortie
{"nom":"Alice","score":42,"niveaux":[1,2,3]}
string
```

### Indenter pour un humain

Le troisième argument indique l'**indentation** : le nombre d'espaces au début de
chaque niveau. Le texte devient lisible, idéal pour un fichier qu'on ouvrira à la main.

```javascript
const joueur = { nom: "Alice", score: 42, niveaux: [1, 2] };
console.log(JSON.stringify(joueur, null, 2));
```

```sortie
{
  "nom": "Alice",
  "score": 42,
  "niveaux": [
    1,
    2
  ]
}
```

### Choisir les clés avec un replacer

Le deuxième argument est le **replacer** (« remplaçant »). Avec un tableau de noms, seules
ces clés sont gardées. C'est pratique pour ne pas enregistrer un mot de passe.

```javascript
const compte = { nom: "Alice", email: "alice@exemple.fr", motDePasse: "secret" };
console.log(JSON.stringify(compte, ["nom", "email"]));
```

```sortie
{"nom":"Alice","email":"alice@exemple.fr"}
```

Le replacer peut aussi être une fonction `(cle, valeur) => …`. Elle est appelée pour
chaque valeur. Ce qu'elle renvoie est écrit à la place ; `undefined` supprime la clé.

```javascript
const produit = { nom: "Stylo", prix: 1.5, stock: 120 };

const texte = JSON.stringify(produit, (cle, valeur) => {
  if (cle === "stock") return undefined;
  if (cle === "prix") return `${valeur.toFixed(2)} €`;
  return valeur;
});
console.log(texte);
```

```sortie
{"nom":"Stylo","prix":"1.50 €"}
```

### Ce que JSON ne sait pas écrire

Certaines valeurs n'ont pas d'équivalent en JSON :

```javascript
const donnees = {
  fonction() {},
  absent: undefined,
  ensemble: new Set([1, 2]),
  date: new Date(Date.UTC(2024, 0, 15)),
};
console.log(JSON.stringify(donnees));
```

```sortie
{"ensemble":{},"date":"2024-01-15T00:00:00.000Z"}
```

- les fonctions et `undefined` disparaissent ;
- un `Set` ou une `Map` devient un objet vide `{}` : convertissez-les en tableau avant
  (`[...ensemble]`) ;
- une date devient un **texte** au format ISO.

## Relire du JSON avec JSON.parse

`JSON.parse` fait l'inverse : il lit un texte JSON et renvoie la valeur.

```javascript
const texte = '{"nom":"Alice","score":42,"niveaux":[1,2,3]}';
const joueur = JSON.parse(texte);

console.log(joueur.nom);
console.log(joueur.score + 1);
console.log(joueur.niveaux.length);
```

```sortie
Alice
43
3
```

### Un texte invalide

Si le texte n'est pas du JSON correct, `JSON.parse` lève une `SyntaxError`. Un fichier
modifié à la main, ou une réponse de serveur inattendue, suffit à la provoquer.
Attrapez-la avec `try...catch` (voir [les erreurs](erreurs.md)).

```javascript
function lireJson(texte, parDefaut) {
  try {
    return JSON.parse(texte);
  } catch (erreur) {
    console.log(`JSON invalide : ${erreur.message}`);
    return parDefaut;
  }
}

console.log(lireJson('{"a": 1}', {}));
console.log(lireJson("{'a': 1}", {}));
console.log(lireJson("", []));
```

```sortie
{ a: 1 }
JSON invalide : Expected property name or '}' in JSON at position 1 (line 1 column 2)
{}
JSON invalide : Unexpected end of JSON input
[]
```

Le message donne la **position** du problème : le numéro du caractère fautif, à partir
de 0.

### Retrouver des dates avec un reviver

Après un aller-retour en JSON, une date est devenue un texte. Le deuxième argument de
`JSON.parse`, le **reviver** (« celui qui ranime »), est une fonction appelée sur
chaque valeur lue. Elle peut la transformer.

```javascript
const texte = '{"titre":"Réunion","le":"2024-03-05T14:30:00.000Z"}';

const sansReviver = JSON.parse(texte);
console.log(typeof sansReviver.le);

const evenement = JSON.parse(texte, (cle, valeur) => {
  if (cle === "le") return new Date(valeur);
  return valeur;
});
console.log(evenement.le instanceof Date);
console.log(evenement.le.getUTCHours());
```

```sortie
string
true
14
```

> **Astuce :** pour les dates, le plus simple est de choisir des noms de clés
> reconnaissables (`le`, `creeLe`…) et de les convertir dans le reviver. Voir
> [dates et heures](dates.md).

## JSON et fichiers ensemble

Écrire un objet dans un fichier, puis le relire, se fait en deux étapes à chaque fois.

```javascript
import { readFile, writeFile } from "node:fs/promises";

const scores = { alice: 42, bob: 17 };
await writeFile("scores.json", JSON.stringify(scores, null, 2) + "\n");

const relu = JSON.parse(await readFile("scores.json", "utf8"));
relu.bob += 10;
console.log(relu);
```

```sortie
{ alice: 42, bob: 27 }
```

## CSV à la main

Un **CSV** (*Comma-Separated Values*, « valeurs séparées par des virgules ») est un
tableau écrit en texte : une ligne par rangée, une virgule entre les colonnes. La
première ligne donne souvent les titres.

```text
nom,ville,note
Alice,Lyon,15
Bob,Paris,12
```

### Lire un CSV simple

On découpe en lignes, puis chaque ligne en colonnes, avec `split`. La ligne de titres
sert à fabriquer un objet par rangée.

```javascript
const csv = "nom,ville,note\nAlice,Lyon,15\nBob,Paris,12\n";

const [entete, ...lignes] = csv.trim().split("\n");
const colonnes = entete.split(",");

const eleves = lignes.map((ligne) => {
  const valeurs = ligne.split(",");
  return Object.fromEntries(colonnes.map((col, i) => [col, valeurs[i]]));
});

console.log(eleves);
const moyenne = eleves.reduce((s, e) => s + Number(e.note), 0) / eleves.length;
console.log(`Moyenne : ${moyenne}`);
```

```sortie
[
  { nom: 'Alice', ville: 'Lyon', note: '15' },
  { nom: 'Bob', ville: 'Paris', note: '12' }
]
Moyenne : 13.5
```

> **Attention :** en CSV, tout est du texte. `note` vaut `"15"`, pas `15` : convertissez
> avec `Number` avant de calculer.

### Écrire un CSV

Pour écrire, on fait l'inverse : `join(",")` pour les colonnes, `join("\n")` pour les
lignes.

```javascript
const eleves = [
  { nom: "Alice", note: 15 },
  { nom: "Bob", note: 12 },
];

const lignes = [
  "nom,note",
  ...eleves.map((e) => [e.nom, e.note].join(",")),
];
console.log(lignes.join("\n"));
```

```sortie
nom,note
Alice,15
Bob,12
```

### Pourquoi une bibliothèque vaut mieux

Le CSV a des pièges. Une valeur peut contenir une virgule : elle est alors écrite entre
guillemets. `split(",")` la coupe en deux.

```javascript
const ligne = '"Dupont, Jean",Lyon,14';
console.log(ligne.split(","));
```

```sortie
[ '"Dupont', ' Jean"', 'Lyon', '14' ]
```

Il y a d'autres cas : des guillemets doublés (`""`) dans une valeur, des retours à la
ligne dans une case, un séparateur `;` dans les fichiers Excel français… Pour de vrais
fichiers, utilisez une [bibliothèque](../debuter/glossaire.md#bibliotheque) qui gère
tout cela, installée avec `npm` (voir [les modules](modules.md)).

Dans le navigateur ou dans Node, **Papa Parse** est très répandu :

```javascript norun
// npm install papaparse
import Papa from "papaparse";

const resultat = Papa.parse('nom,ville\n"Dupont, Jean",Lyon\n', { header: true, skipEmptyLines: true });
console.log(resultat.data);
```

```text
[ { nom: 'Dupont, Jean', ville: 'Lyon' } ]
```

Dans Node, **csv-parse** est une autre option solide :

```javascript norun
// npm install csv-parse
import { parse } from "csv-parse/sync";

const lignes = parse('nom,ville\n"Dupont, Jean",Lyon\n', { columns: true });
console.log(lignes);
```

```text
[ { nom: 'Dupont, Jean', ville: 'Lyon' } ]
```

## Quel format choisir ?

| Besoin | Format | Pourquoi |
|--------|--------|----------|
| Sauver l'état d'un programme | JSON | garde les objets imbriqués et les listes |
| Échanger avec un serveur web | JSON | le format des API web |
| Un tableau pour Excel ou un tableur | CSV | s'ouvre partout, une ligne par rangée |
| De grosses listes plates (mesures, ventes) | CSV | compact, se lit ligne à ligne |
| Un fichier de configuration écrit à la main | JSON (ou YAML, TOML avec une bibliothèque) | lisible et structuré |
| Des données dans `localStorage` | JSON | `localStorage` ne range que du texte |

## Erreurs fréquentes

### Une virgule en trop

```javascript norun
JSON.parse('{"nom": "Alice",}');
```

```text
SyntaxError: Expected double-quoted property name in JSON at position 16 (line 1 column 17)
```

JavaScript accepte une virgule finale dans un objet, pas JSON. Retirez la virgule après
la dernière valeur.

### Des apostrophes au lieu de guillemets

```javascript norun
JSON.parse("{'nom': 'Alice'}");
```

```text
SyntaxError: Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

En JSON, clés et textes prennent des guillemets doubles. Le plus sûr : ne pas écrire le
JSON à la main, mais le produire avec `JSON.stringify`.

### Analyser un objet au lieu d'un texte

```javascript norun
const joueur = { nom: "Alice" };
JSON.parse(joueur);
```

```text
SyntaxError: "[object Object]" is not valid JSON
```

`JSON.parse` attend un **texte**. Ici, `joueur` est déjà un objet : il n'y a rien à
analyser. Utilisez `JSON.parse` sur ce que vous lisez d'un fichier ou du réseau, et
`JSON.stringify` pour aller dans l'autre sens.

## Testez-vous

**Question 1 :** que renvoie `JSON.stringify({ a: 1, b: undefined })` ?

> **Réponse**
>
> `'{"a":1}'` : une clé dont la valeur est `undefined` disparaît.

**Question 2 :** après `JSON.parse`, une date enregistrée est-elle un objet `Date` ?

> **Réponse**
>
> Non, c'est un texte comme `"2024-03-05T14:30:00.000Z"`. Utilisez un *reviver* ou
> `new Date(texte)` pour retrouver une date.

**Question 3 :** transformez le CSV `"fruit,prix\npomme,2\npoire,3"` en total des prix.

> **Réponse**
>
> ```javascript
> const csv = "fruit,prix\npomme,2\npoire,3";
> const total = csv
>   .split("\n")
>   .slice(1)
>   .map((ligne) => Number(ligne.split(",")[1]))
>   .reduce((a, b) => a + b, 0);
> console.log(total);
> ```
>
> ```sortie
> 5
> ```

## En résumé

- `JSON.stringify(valeur, null, 2)` écrit du JSON lisible ; un replacer choisit ou transforme les clés.
- `JSON.parse(texte)` relit du JSON ; un texte invalide lève une `SyntaxError`.
- Les dates deviennent des textes en JSON : un reviver les retransforme en `Date`.
- Un CSV simple se lit avec `split` et s'écrit avec `join` ; tout y est du texte.
- Pour de vrais fichiers CSV (guillemets, virgules dans les cases), prenez une bibliothèque.

Suite : [les expressions régulières](regex.md)
