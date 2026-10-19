# Les objets

Cette page explique comment regrouper des informations dans un objet, lire et modifier ses propriétés, le parcourir, le copier, et quand choisir plutôt une `Map` ou un `Set`.

> **Objectifs :**
>
> - vous saurez créer un objet et lire ses propriétés avec `.` et `[ ]` ;
> - vous saurez ajouter, modifier et supprimer une propriété ;
> - vous saurez écrire une méthode qui utilise `this` ;
> - vous saurez parcourir un objet avec `Object.keys`, `Object.values` et `Object.entries` ;
> - vous saurez copier un objet et choisir entre un objet, une `Map` et un `Set`.

## Créer un objet

Un [objet](../debuter/glossaire.md#objet) regroupe plusieurs informations sur une même
chose. C'est une fiche d'identité : chaque ligne a une étiquette (la **clé**) et une
valeur. Une paire clé-valeur s'appelle une **propriété**.

```javascript
const chat = {
  nom: "Félix",
  age: 3,
  couleur: "roux",
  vaccine: true,
};

console.log(chat);
```

```sortie
{ nom: 'Félix', age: 3, couleur: 'roux', vaccine: true }
```

- L'objet s'écrit entre accolades `{ }`.
- Chaque propriété s'écrit `clé: valeur`, et les propriétés sont séparées par des
  virgules.
- La virgule après la dernière propriété est permise : elle facilite l'ajout d'une
  ligne.

Un tableau range des valeurs **dans l'ordre**, par numéro. Un objet les range **par
nom**. Ailleurs, on parle de [dictionnaire](../debuter/glossaire.md#dictionnaire).

## Lire une propriété

On lit une propriété avec un point `.` ou avec des crochets `[ ]`.

```javascript
const chat = { nom: "Félix", age: 3 };

console.log(chat.nom);
console.log(chat["age"]);
console.log(chat.poids);

const cle = "nom";
console.log(chat[cle]);
```

```sortie
Félix
3
undefined
Félix
```

- Le point est le plus courant : `chat.nom`.
- Les crochets prennent un **texte** : utile quand la clé est dans une variable
  (`chat[cle]`) ou contient une espace (`objet["date de naissance"]`).
- Une propriété qui n'existe pas donne `undefined`, sans erreur.

## Ajouter, modifier, supprimer

Un objet se modifie après sa création, même s'il est dans une `const` (comme un
tableau : `const` empêche seulement de remplacer l'objet entier).

```javascript
const chat = { nom: "Félix", age: 3 };

chat.age = 4;
chat.couleur = "roux";
delete chat.nom;

console.log(chat);
console.log("couleur" in chat);
console.log("nom" in chat);
```

```sortie
{ age: 4, couleur: 'roux' }
true
false
```

- Affecter une clé qui existe la **modifie** ; une clé nouvelle est **ajoutée**.
- `delete` supprime une propriété.
- `"clé" in objet` vérifie si une propriété existe.

## Les propriétés raccourcies

Quand une variable porte le même nom que la clé, il suffit d'écrire le nom une fois.

```javascript
const nom = "Alice";
const age = 30;

const personne = { nom, age };
console.log(personne);
```

```sortie
{ nom: 'Alice', age: 30 }
```

`{ nom, age }` est un raccourci pour `{ nom: nom, age: age }`.

## Les méthodes et this

Une propriété peut contenir une fonction : on l'appelle alors une
[méthode](../debuter/glossaire.md#methode). À l'intérieur, le mot `this` (« ceci »)
désigne l'objet sur lequel on appelle la méthode.

```javascript
const compte = {
  titulaire: "Alice",
  solde: 100,
  deposer(montant) {
    this.solde += montant;
  },
  resume() {
    return `${this.titulaire} a ${this.solde} €`;
  },
};

compte.deposer(50);
console.log(compte.resume());
```

```sortie
Alice a 150 €
```

`deposer(montant) { ... }` est un raccourci pour `deposer: function (montant) { ... }`.
Dans `compte.deposer(50)`, `this` est l'objet avant le point : `compte`.

> **Attention :** n'écrivez pas une méthode avec une flèche `=>` si elle utilise `this` :
> une fonction fléchée n'a pas son propre `this`. Gardez la forme `nom() { ... }`.

Les objets qui partagent les mêmes méthodes se fabriquent avec des **classes** : ce sera
pour [la page sur l'héritage](heritage.md).

## Parcourir un objet

Trois fonctions transforment un objet en tableau :

- `Object.keys(objet)` : les clés ;
- `Object.values(objet)` : les valeurs ;
- `Object.entries(objet)` : des paires `[clé, valeur]`.

```javascript
const stock = { pommes: 12, poires: 5, kiwis: 0 };

console.log(Object.keys(stock));
console.log(Object.values(stock));
console.log(Object.entries(stock));
```

```sortie
[ 'pommes', 'poires', 'kiwis' ]
[ 12, 5, 0 ]
[ [ 'pommes', 12 ], [ 'poires', 5 ], [ 'kiwis', 0 ] ]
```

Avec `for...of` et une décomposition, on parcourt clés et valeurs d'un coup :

```javascript
const stock = { pommes: 12, poires: 5, kiwis: 0 };

for (const [fruit, quantite] of Object.entries(stock)) {
  console.log(`${fruit} : ${quantite}`);
}

const total = Object.values(stock).reduce((s, n) => s + n, 0);
console.log(`Total : ${total}`);
```

```sortie
pommes : 12
poires : 5
kiwis : 0
Total : 17
```

Les clés sont parcourues dans l'ordre où elles ont été ajoutées (sauf les clés qui sont
des nombres entiers, rangées en premier par ordre croissant).

## La décomposition d'objet

Comme pour les tableaux, on peut ranger plusieurs propriétés dans des variables d'un
coup. Ici, c'est le **nom** qui compte, pas la position.

```javascript
const film = { titre: "Amélie", annee: 2001, duree: 122 };

const { titre, annee } = film;
console.log(titre, annee);

const { duree: minutes, note = "non noté" } = film;
console.log(minutes, note);
```

```sortie
Amélie 2001
122 non noté
```

- `duree: minutes` lit la clé `duree` et la range dans une variable `minutes`.
- `note = "non noté"` donne une valeur par défaut si la clé est absente.

C'est très pratique dans les paramètres d'une fonction :

```javascript
function presenter({ nom, age }) {
  return `${nom}, ${age} ans`;
}

console.log(presenter({ nom: "Alice", age: 30 }));
```

```sortie
Alice, 30 ans
```

## Copier un objet

Comme les tableaux, un objet est partagé par **référence** : `b = a` ne copie rien, les
deux noms désignent le même objet. Pour copier, utilisez le spread `...`.

```javascript
const original = { nom: "Alice", age: 30 };
const copie = { ...original };
copie.age = 31;

const enrichi = { ...original, ville: "Lyon", age: 40 };

console.log(original);
console.log(copie);
console.log(enrichi);
```

```sortie
{ nom: 'Alice', age: 30 }
{ nom: 'Alice', age: 31 }
{ nom: 'Alice', age: 40, ville: 'Lyon' }
```

Dans `enrichi`, les propriétés écrites **après** le spread remplacent celles de
l'original.

### Une copie superficielle

Le spread fait une copie **superficielle** : il copie le premier niveau seulement. Un
objet ou un tableau rangé à l'intérieur reste partagé.

```javascript
const original = { nom: "Alice", loisirs: ["vélo"] };
const copie = { ...original };
copie.loisirs.push("piano");
console.log(original.loisirs);

const profonde = structuredClone(original);
profonde.loisirs.push("chant");
console.log(original.loisirs);
console.log(profonde.loisirs);
```

```sortie
[ 'vélo', 'piano' ]
[ 'vélo', 'piano' ]
[ 'vélo', 'piano', 'chant' ]
```

`structuredClone(objet)` fait une copie **profonde** : tous les niveaux sont copiés.

## Afficher un objet avec JSON.stringify

Dans un gabarit, un objet s'affiche mal : `[object Object]`. `JSON.stringify` le
transforme en texte lisible, au format [JSON](../debuter/glossaire.md#json).

```javascript
const personne = { nom: "Alice", age: 30, langues: ["fr", "en"] };

console.log(`Personne : ${personne}`);
console.log(`Personne : ${JSON.stringify(personne)}`);
console.log(JSON.stringify(personne, null, 2));
```

```sortie
Personne : [object Object]
Personne : {"nom":"Alice","age":30,"langues":["fr","en"]}
{
  "nom": "Alice",
  "age": 30,
  "langues": [
    "fr",
    "en"
  ]
}
```

Le troisième argument (`2`) indente le résultat de 2 espaces. Le JSON sert aussi à
enregistrer des données dans un fichier : voir [les formats de données](formats.md).

## Le chaînage optionnel

Dans des objets imbriqués, une propriété du milieu peut manquer. `?.` s'arrête
proprement et renvoie `undefined` au lieu de planter ; `??` fournit alors une valeur de
secours (voir [les conditions](conditions-boucles.md#les-valeurs-absentes-et)).

```javascript
const clients = [
  { nom: "Alice", adresse: { ville: "Lyon" } },
  { nom: "Bob" },
];

for (const client of clients) {
  const ville = client.adresse?.ville ?? "inconnue";
  console.log(`${client.nom} : ${ville}`);
}
```

```sortie
Alice : Lyon
Bob : inconnue
```

## Map et Set

### Map : un dictionnaire sur mesure

Une `Map` range aussi des paires clé-valeur, avec des méthodes dédiées : `set`, `get`,
`has`, `delete`, et `size` pour le nombre de paires.

```javascript
const compteur = new Map();

for (const mot of ["chat", "chien", "chat", "chat"]) {
  compteur.set(mot, (compteur.get(mot) ?? 0) + 1);
}

console.log(compteur);
console.log(compteur.get("chat"));
console.log(compteur.has("poisson"));
console.log(compteur.size);

for (const [mot, n] of compteur) {
  console.log(mot, n);
}
```

```sortie
Map(2) { 'chat' => 3, 'chien' => 1 }
3
false
2
chat 3
chien 1
```

### Set : un ensemble sans doublons

Un `Set` est un [ensemble](../debuter/glossaire.md#ensemble) : une collection de valeurs
**uniques**. Ajouter une valeur déjà présente ne fait rien.

```javascript
const visiteurs = new Set();
visiteurs.add("Alice");
visiteurs.add("Bob");
visiteurs.add("Alice");

console.log(visiteurs);
console.log(visiteurs.size);
console.log(visiteurs.has("Bob"));

const sansDoublons = [...new Set([3, 1, 3, 2, 1])];
console.log(sansDoublons);
```

```sortie
Set(2) { 'Alice', 'Bob' }
2
true
[ 3, 1, 2 ]
```

`[...new Set(tableau)]` est l'astuce classique pour retirer les doublons d'un tableau.

### Que choisir ?

| Besoin | Choix |
| --- | --- |
| Décrire **une chose** avec des champs connus (nom, âge…) | un objet `{ }` |
| Des clés ajoutées au fil du programme (compter des mots, un cache) | une `Map` |
| Des clés qui ne sont pas des textes (nombres, objets) | une `Map` |
| Une liste de valeurs **sans doublons**, et tester l'appartenance | un `Set` |
| Envoyer ou enregistrer en JSON | un objet (JSON ne connaît pas `Map` ni `Set`) |

## Erreurs fréquentes

### Oublier une virgule entre deux propriétés

```javascript norun
const chat = {
  nom: "Félix",
  age: 3
  couleur: "roux",
};
```

```text
file:///home/alice/js/main.mjs:4
  couleur: "roux",
  ^^^^^^^

SyntaxError: Unexpected identifier 'couleur'

Node.js v22.22.0
```

Il manque la virgule après `age: 3`. JavaScript ne s'attendait pas à trouver `couleur`
juste après (*unexpected identifier*). Ajoutez la virgule.

### Lire dans une propriété qui n'existe pas

```javascript norun
const utilisateur = { nom: "Alice" };
console.log(utilisateur.adresse.ville);
```

```text
file:///home/alice/js/main.mjs:2
console.log(utilisateur.adresse.ville);
                                ^

TypeError: Cannot read properties of undefined (reading 'ville')
    at file:///home/alice/js/main.mjs:2:33

Node.js v22.22.0
```

`utilisateur.adresse` vaut `undefined`, et on ne peut rien lire dans `undefined`.
Utilisez le chaînage optionnel : `utilisateur.adresse?.ville`.

### Utiliser le point avec une clé dans une variable

```javascript norun
const notes = { maths: 15, francais: 12 };
const matiere = "maths";
console.log(notes.matiere);
```

```text
undefined
```

`notes.matiere` cherche une clé qui s'appelle littéralement `"matiere"`. Pour utiliser
le **contenu** de la variable, écrivez `notes[matiere]`.

### Afficher un objet dans un gabarit

```javascript norun
const panier = { pommes: 3 };
console.log(`Panier : ${panier}`);
```

```text
Panier : [object Object]
```

Un gabarit transforme l'objet en texte de la façon la plus pauvre. Écrivez
`${JSON.stringify(panier)}`, ou passez l'objet à part : `console.log("Panier :", panier)`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
const p = { x: 1, y: 2 };
const q = p;
q.x = 10;
console.log(p.x);
```

> **Réponse**
>
> `10`. `q` et `p` désignent le même objet : modifier l'un modifie l'autre. Pour une
> copie, écrivez `const q = { ...p };`.

**Question 2 :** comment afficher chaque clé et chaque valeur de `{ a: 1, b: 2 }` ?

> **Réponse**
>
> ```javascript
> for (const [cle, valeur] of Object.entries({ a: 1, b: 2 })) {
>   console.log(cle, valeur);
> }
> ```
>
> ```sortie
> a 1
> b 2
> ```

**Question 3 :** vous voulez la liste des villes visitées, sans doublons. Objet, `Map` ou
`Set` ?

> **Réponse**
>
> Un `Set` : il garde chaque valeur une seule fois, et `has(ville)` dit si une ville est
> déjà dans la liste.

## En résumé

- Un objet `{ cle: valeur }` regroupe des propriétés ; on les lit avec `objet.cle` ou
  `objet["cle"]`.
- On ajoute ou modifie par affectation, on supprime avec `delete` ; `{ nom, age }` est
  un raccourci.
- Une méthode est une fonction dans un objet ; `this` désigne l'objet avant le point.
- `Object.keys`, `Object.values` et `Object.entries` parcourent un objet ;
  `const { a, b } = objet` le décompose.
- `{ ...objet }` fait une copie superficielle, `structuredClone` une copie profonde ;
  `JSON.stringify` l'affiche lisiblement.
- `Map` pour des clés ajoutées au fil du programme, `Set` pour des valeurs uniques.

Suite : [Les pièges du débutant en JavaScript](pieges.md)
