# Les classes

Cette page montre comment créer vos propres objets avec `class` : constructeur, champs, méthodes, `this`, accesseurs, membres statiques et champs privés.

> **Objectifs :**
>
> - vous saurez écrire une classe et créer des objets avec `new` ;
> - vous saurez ranger des données dans un objet avec `constructor` et `this` ;
> - vous saurez ajouter des méthodes, des getters et des setters ;
> - vous saurez cacher une donnée avec un champ privé `#` ;
> - vous saurez vérifier la classe d'un objet avec `instanceof`.

## Pourquoi des classes ?

Vous savez déjà créer un objet avec des accolades (voir [les objets](objets.md)) :

```javascript
const minou = { nom: "Minou", age: 3 };
const felix = { nom: "Félix", age: 5 };
console.log(minou.nom, felix.nom);
```

```sortie
Minou Félix
```

C'est pratique pour un ou deux objets. Mais si vous avez cent chats, vous recopiez cent
fois la même forme, et une faute de frappe (`nmo` au lieu de `nom`) passe inaperçue.

Une [classe](../debuter/glossaire.md#classe) est un **modèle** pour fabriquer des objets
qui ont tous la même forme. Pensez à un moule à gâteaux : le moule est la classe, chaque
gâteau est un [objet](../debuter/glossaire.md#objet). Tous ont la même forme, mais chacun
a son parfum.

## Définir une classe

Le mot-clé `class` définit une classe. Par convention, son nom commence par une
majuscule. `new` fabrique un nouvel objet à partir de la classe : une **instance**.

```javascript
class Chat {
  miauler() {
    console.log("Miaou !");
  }
}

const minou = new Chat();
minou.miauler();
```

```sortie
Miaou !
```

`miauler` est une [méthode](../debuter/glossaire.md#methode) : une fonction qui
appartient aux objets de la classe. Remarquez l'écriture : pas de mot `function`, et pas
de virgule entre les méthodes.

## Le constructeur et this

`constructor` est une méthode spéciale, appelée automatiquement par `new`. Elle prépare
l'objet, comme on remplit une fiche à l'inscription. Les arguments de `new` lui sont
passés.

Dans une méthode, `this` désigne **l'objet en cours** : « moi-même ». `this.nom = nom`
veut dire : « range la valeur `nom` dans ma propriété `nom` ».

```javascript
class Chat {
  constructor(nom, age) {
    this.nom = nom;
    this.age = age;
  }

  presenter() {
    console.log(`Je m'appelle ${this.nom} et j'ai ${this.age} ans.`);
  }
}

const minou = new Chat("Minou", 3);
const felix = new Chat("Félix", 5);
minou.presenter();
felix.presenter();
console.log(minou);
```

```sortie
Je m'appelle Minou et j'ai 3 ans.
Je m'appelle Félix et j'ai 5 ans.
Chat { nom: 'Minou', age: 3 }
```

Chaque objet a ses propres données : `minou.nom` et `felix.nom` sont différents. La
méthode, elle, est la même pour tous ; `this` lui dit sur quel objet travailler.

`console.log` affiche le nom de la classe, puis les propriétés de l'objet.

## Les champs

Un **champ** est une propriété déclarée directement dans la classe, avec une valeur de
départ. C'est plus lisible quand la valeur ne dépend pas des arguments.

```javascript
class Compteur {
  valeur = 0;

  incrementer() {
    this.valeur += 1;
  }
}

const c = new Compteur();
c.incrementer();
c.incrementer();
console.log(c.valeur);
```

```sortie
2
```

Les champs sont créés **avant** que le constructeur s'exécute. Chaque objet reçoit son
propre champ : deux compteurs ne partagent pas leur `valeur`.

## Des méthodes qui calculent

Une méthode peut renvoyer une valeur avec `return`, et modifier l'objet. Elle peut aussi
appeler une autre méthode du même objet avec `this.`.

```javascript
class Panier {
  articles = [];

  ajouter(nom, prix) {
    this.articles.push({ nom, prix });
    return this;
  }

  total() {
    return this.articles.reduce((somme, a) => somme + a.prix, 0);
  }

  afficher() {
    console.log(`${this.articles.length} articles, total ${this.total()} €`);
  }
}

const panier = new Panier();
panier.ajouter("pain", 2).ajouter("lait", 1.5);
panier.afficher();
```

```sortie
2 articles, total 3.5 €
```

> **Astuce :** `ajouter` renvoie `this`, l'objet lui-même. On peut donc enchaîner les
> appels : `panier.ajouter(...).ajouter(...)`.

## Getters et setters

Un **getter** (`get`) est une méthode qui se lit comme une propriété, sans parenthèses.
Il sert pour une valeur calculée.

Un **setter** (`set`) est appelé quand on affecte une valeur avec `=`. Il sert à
vérifier la valeur avant de la ranger.

```javascript
class Rectangle {
  constructor(largeur, hauteur) {
    this.largeur = largeur;
    this.hauteur = hauteur;
  }

  get aire() {
    return this.largeur * this.hauteur;
  }

  set cote(valeur) {
    this.largeur = valeur;
    this.hauteur = valeur;
  }
}

const r = new Rectangle(3, 4);
console.log(r.aire);
r.cote = 5;
console.log(r.largeur, r.hauteur, r.aire);
```

```sortie
12
5 5 25
```

`r.aire` s'écrit sans parenthèses, mais c'est bien du code qui s'exécute à chaque
lecture : l'aire est toujours à jour.

## Les membres statiques

Un membre `static` appartient à la **classe elle-même**, pas à ses objets. On l'appelle
avec le nom de la classe : `Temperature.depuisFahrenheit(...)`. C'est utile pour une
constante commune ou pour une autre façon de fabriquer un objet.

```javascript
class Temperature {
  static ZERO_ABSOLU = -273.15;

  constructor(celsius) {
    this.celsius = celsius;
  }

  static depuisFahrenheit(f) {
    return new Temperature(((f - 32) * 5) / 9);
  }
}

console.log(Temperature.ZERO_ABSOLU);
const t = Temperature.depuisFahrenheit(212);
console.log(t.celsius);
```

```sortie
-273.15
100
```

Vous connaissez déjà des méthodes statiques : `Math.max`, `Number.parseInt`,
`Array.isArray`, `Object.keys`…

## Les champs privés

Par défaut, tout le monde peut lire et modifier les propriétés d'un objet. Pour un
compte bancaire, c'est gênant : n'importe qui pourrait écrire `compte.solde = 1000000`.

Un champ dont le nom commence par `#` est **privé** : seul le code écrit à l'intérieur de
la classe peut y accéder. C'est comme le coffre d'une banque : on passe par le guichet
(les méthodes), jamais directement par le coffre.

```javascript
class Compte {
  #solde = 0;

  deposer(montant) {
    if (montant <= 0) {
      console.log("Montant refusé.");
      return;
    }
    this.#solde += montant;
  }

  get solde() {
    return this.#solde;
  }
}

const compte = new Compte();
compte.deposer(50);
compte.deposer(-10);
console.log(compte.solde);
try {
  compte.solde = 1000000;
} catch (erreur) {
  console.log(erreur.message);
}
```

```sortie
Montant refusé.
50
Cannot set property solde of #<Compte> which has only a getter
```

Le getter `solde` permet de **lire** le solde, mais il n'y a pas de setter :
l'affectation `compte.solde = 1000000` lève une erreur (« impossible de modifier `solde`,
qui n'a qu'un getter »). Seule `deposer` peut changer `#solde`, et elle vérifie le
montant. `try` et `catch` attrapent l'erreur ; vous les verrez en détail dans
[gérer les erreurs](erreurs.md).

Une méthode peut aussi être privée : `#calculerInterets() { ... }`. On l'appelle avec
`this.#calculerInterets()`, et seulement depuis la classe.

## Afficher un objet avec toString

Quand JavaScript doit transformer un objet en texte (dans un gabarit `${}` ou avec
`String(...)`), il appelle sa méthode `toString`. Par défaut, elle donne un texte peu
utile : `[object Object]`. Écrivez la vôtre.

```javascript
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  toString() {
    return `(${this.x}, ${this.y})`;
  }
}

const p = new Point(2, 5);
console.log(`Le point est en ${p}`);
console.log(String(p));
```

```sortie
Le point est en (2, 5)
(2, 5)
```

> **Note :** `console.log(p)` n'utilise pas `toString` : il montre la structure de
> l'objet, `Point { x: 2, y: 5 }`. C'est pratique pour déboguer.

## Vérifier la classe avec instanceof

`instanceof` répond à la question : « cet objet a-t-il été fabriqué avec cette
classe ? ». Le résultat est un [booléen](../debuter/glossaire.md#booleen).

```javascript
class Chat {}
class Chien {}

const minou = new Chat();
console.log(minou instanceof Chat);
console.log(minou instanceof Chien);
console.log([1, 2] instanceof Array);
console.log(minou.constructor.name);
console.log(typeof Chat);
```

```sortie
true
false
true
Chat
function
```

`constructor.name` donne le nom de la classe d'un objet. Et `typeof Chat` révèle un
secret : en JavaScript, une classe est une sorte de fonction, celle qui fabrique les
objets. Le chapitre sur [l'héritage](heritage.md) explique ce qui se cache derrière.

## Erreurs fréquentes

### Oublier new

```javascript norun
class Chat {
  constructor(nom) { this.nom = nom; }
}
const c = Chat("Minou");
```

```text
file:///home/vous/projet/main.mjs:4
const c = Chat("Minou");
          ^

TypeError: Class constructor Chat cannot be invoked without 'new'
    at file:///home/vous/projet/main.mjs:4:11
```

Une classe ne s'appelle pas comme une fonction. Écrivez `new Chat("Minou")`.

### Oublier this dans une méthode

```javascript norun
class Chat {
  constructor(nom) { this.nom = nom; }
  presenter() { console.log("Je suis " + nom); }
}
new Chat("Minou").presenter();
```

```text
file:///home/vous/projet/main.mjs:3
  presenter() { console.log("Je suis " + nom); }
                                         ^

ReferenceError: nom is not defined
    at Chat.presenter (file:///home/vous/projet/main.mjs:3:42)
    at file:///home/vous/projet/main.mjs:5:19
```

Dans `presenter`, `nom` seul est une variable qui n'existe pas. La propriété de l'objet
s'écrit `this.nom`.

### Lire un champ privé depuis l'extérieur

```javascript norun
class Compte {
  #solde = 0;
}
const c = new Compte();
console.log(c.#solde);
```

```text
file:///home/vous/projet/main.mjs:5
console.log(c.#solde);
             ^

SyntaxError: Private field '#solde' must be declared in an enclosing class
```

Le programme ne démarre même pas : c'est une erreur de
[syntaxe](../debuter/glossaire.md#syntaxe). Ajoutez une méthode ou un getter dans la
classe (`get solde() { return this.#solde; }`), puis lisez `c.solde`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
class Boite {
  contenu = [];
}
const a = new Boite();
const b = new Boite();
a.contenu.push("pomme");
console.log(b.contenu.length);
```

> **Réponse**
>
> `0`. Chaque objet reçoit son propre champ `contenu` : remplir celui de `a` ne change
> pas celui de `b`.

**Question 2 :** quelle est la différence entre `static` et un champ normal ?

> **Réponse**
>
> Un champ normal existe dans **chaque objet** (`minou.nom`). Un membre `static` existe
> **une seule fois**, sur la classe (`Temperature.ZERO_ABSOLU`).

**Question 3 :** écrivez une classe `Personne` avec un `prenom` et un `nom`, et un getter
`nomComplet` qui renvoie les deux séparés par une espace.

> **Réponse**
>
> ```javascript
> class Personne {
>   constructor(prenom, nom) {
>     this.prenom = prenom;
>     this.nom = nom;
>   }
>
>   get nomComplet() {
>     return `${this.prenom} ${this.nom}`;
>   }
> }
>
> console.log(new Personne("Ada", "Lovelace").nomComplet);
> ```
>
> ```sortie
> Ada Lovelace
> ```

## En résumé

- Une classe est un modèle ; `new` fabrique un objet, et appelle `constructor`.
- `this` désigne l'objet en cours ; ses propriétés s'écrivent `this.nom`.
- `get` et `set` donnent des propriétés calculées ou vérifiées.
- `static` appartient à la classe ; `#champ` est privé, invisible hors de la classe.
- `toString` choisit le texte d'un objet ; `instanceof` vérifie sa classe.

Suite : [héritage et prototypes](heritage.md). Comparez avec
[les classes en Ruby](../ruby/types.md).
