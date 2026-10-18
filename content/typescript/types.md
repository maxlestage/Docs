# Les classes

Cette page explique comment créer vos propres types d'objets avec `class`, protéger leurs données et les relier à des interfaces.

> **Objectifs :**
>
> - vous saurez écrire une classe avec des champs typés et un constructeur ;
> - vous saurez raccourcir un constructeur avec les propriétés de paramètre ;
> - vous saurez protéger des données avec `private`, `protected`, `readonly` et `#` ;
> - vous saurez qu'une classe respecte une interface avec `implements` ;
> - vous saurez utiliser `abstract`, `static` et les accesseurs `get` / `set`.

## Une première classe

Une [classe](../debuter/glossaire.md#classe) est un moule à objets. Le moule décrit la
forme ; chaque objet fabriqué avec ce moule est une **instance**. C'est comme un
emporte-pièce à biscuits : un seul outil, autant de biscuits que vous voulez.

```typescript
class Chat {
  nom: string;
  age: number;

  constructor(nom: string, age: number) {
    this.nom = nom;
    this.age = age;
  }

  presenter(): string {
    return `Je suis ${this.nom}, j'ai ${this.age} ans.`;
  }
}

const minou = new Chat("Minou", 3);
const tigre = new Chat("Tigre", 5);
console.log(minou.presenter());
console.log(tigre.presenter());
```

```sortie
Je suis Minou, j'ai 3 ans.
Je suis Tigre, j'ai 5 ans.
```

Décortiquons :

- `nom: string;` déclare un **champ** : une donnée que chaque chat possède, avec son
  type ;
- `constructor` est la fonction appelée par `new`. Elle remplit les champs ;
- `this` désigne l'objet en cours de construction (ou celui sur lequel on appelle la
  méthode) ;
- `presenter()` est une [méthode](../debuter/glossaire.md#methode) : une fonction
  attachée à l'objet.

Une classe est aussi un type : vous pouvez écrire `const c: Chat = ...`.

### Une valeur de départ

Un champ peut recevoir sa valeur directement, sans passer par le constructeur.
TypeScript devine alors son type.

```typescript
class Compteur {
  valeur = 0;

  incrementer() {
    this.valeur++;
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

> **À retenir :** en mode strict, chaque champ doit recevoir une valeur, soit dans sa
> déclaration, soit dans le constructeur. Sinon, TypeScript refuse (voir les
> [erreurs fréquentes](#erreurs-frequentes)).

## Les propriétés de paramètre

Écrire chaque champ trois fois (déclaration, paramètre, `this.x = x`) est fastidieux.
TypeScript propose un raccourci : placez `public` (ou `private`, `readonly`…) devant un
paramètre du constructeur. Le champ est déclaré et rempli automatiquement.

```typescript
class Point {
  constructor(public x: number, public y: number) {}

  distanceOrigine(): number {
    return Math.sqrt(this.x ** 2 + this.y ** 2);
  }
}

const p = new Point(3, 4);
console.log(p.x, p.y);
console.log(p.distanceOrigine());
```

```sortie
3 4
5
```

Le corps du constructeur est vide `{}` : tout le travail est fait par les mots devant
les paramètres.

## Protéger les données

Par défaut, tout est **public** : n'importe quel code peut lire et modifier les champs.
Certains champs ne devraient pas être touchés de l'extérieur. Un compte en banque ne
doit pas voir son solde modifié n'importe comment.

| Mot | Qui peut y accéder ? |
|-----|----------------------|
| `public` | tout le monde (c'est le défaut) |
| `private` | seulement le code de la classe |
| `protected` | la classe et ses classes filles |
| `readonly` | tout le monde peut lire, personne ne modifie après le constructeur |

```typescript
class Compte {
  private solde = 0;
  readonly titulaire: string;

  constructor(titulaire: string) {
    this.titulaire = titulaire;
  }

  deposer(montant: number) {
    if (montant <= 0) {
      console.log("Montant refusé");
      return;
    }
    this.solde += montant;
  }

  afficher() {
    console.log(`${this.titulaire} : ${this.solde} €`);
  }
}

const compte = new Compte("Alice");
compte.deposer(50);
compte.deposer(-10);
compte.afficher();
```

```sortie
Montant refusé
Alice : 50 €
```

Le seul moyen de changer le solde est `deposer`, qui vérifie le montant. Écrire
`compte.solde = 1000` dehors serait refusé par TypeScript.

### Les champs vraiment privés avec #

`private` est une protection de TypeScript seulement : une fois le code compilé en
JavaScript, elle disparaît. JavaScript a sa propre protection : un nom qui commence
par `#`. Celle-ci reste active à l'exécution.

```typescript
class Coffre {
  #code: string;

  constructor(code: string) {
    this.#code = code;
  }

  ouvrir(essai: string): string {
    return essai === this.#code ? "Ouvert !" : "Mauvais code";
  }
}

const coffre = new Coffre("1234");
console.log(coffre.ouvrir("0000"));
console.log(coffre.ouvrir("1234"));
```

```sortie
Mauvais code
Ouvert !
```

> **Note :** les deux se valent pour débuter. `private` est plus courant dans le code
> TypeScript ; `#` est utile quand la protection doit tenir aussi à l'exécution.

## Implémenter une interface

Une interface décrit une forme (voir [les objets](objets.md)). Avec `implements`, une
classe **promet** de respecter cette forme. Si elle oublie un champ ou une méthode,
TypeScript le signale.

```typescript
interface Animal {
  nom: string;
  crier(): string;
}

class Chien implements Animal {
  constructor(public nom: string) {}
  crier() {
    return "Ouaf";
  }
}

class Vache implements Animal {
  constructor(public nom: string) {}
  crier() {
    return "Meuh";
  }
}

const ferme: Animal[] = [new Chien("Rex"), new Vache("Marguerite")];
for (const a of ferme) {
  console.log(`${a.nom} fait ${a.crier()}`);
}
```

```sortie
Rex fait Ouaf
Marguerite fait Meuh
```

Le tableau a le type `Animal[]` : il accepte n'importe quel objet qui a la bonne forme.

## Hériter d'une classe

`extends` crée une classe **fille** à partir d'une classe **mère** : elle reçoit tous
ses champs et méthodes (voir l'[héritage](../debuter/glossaire.md#heritage)). Dans le
constructeur, `super(...)` appelle d'abord le constructeur de la mère.

```typescript
class Vehicule {
  protected vitesse = 0;

  constructor(public nom: string) {}

  accelerer(delta: number) {
    this.vitesse += delta;
  }
}

class Velo extends Vehicule {
  constructor() {
    super("Vélo");
  }

  decrire(): string {
    return `${this.nom} à ${this.vitesse} km/h`;
  }
}

const v = new Velo();
v.accelerer(15);
console.log(v.decrire());
```

```sortie
Vélo à 15 km/h
```

`vitesse` est `protected` : `Velo`, la classe fille, peut la lire ; le code extérieur
non.

## Les classes abstraites

Une classe **abstraite** est un moule incomplet : on ne peut pas créer d'objet avec.
Elle sert de base commune. Elle déclare des méthodes `abstract` sans corps, que chaque
classe fille doit écrire.

```typescript
abstract class Forme {
  abstract aire(): number;

  decrire(): string {
    return `Aire : ${this.aire().toFixed(2)}`;
  }
}

class Carre extends Forme {
  constructor(private cote: number) {
    super();
  }
  aire() {
    return this.cote ** 2;
  }
}

class Disque extends Forme {
  constructor(private rayon: number) {
    super();
  }
  aire() {
    return Math.PI * this.rayon ** 2;
  }
}

const formes: Forme[] = [new Carre(2), new Disque(1)];
for (const f of formes) {
  console.log(f.decrire());
}
```

```sortie
Aire : 4.00
Aire : 3.14
```

`decrire` est écrite une seule fois, dans `Forme`. Elle appelle `aire()`, qui est
différente pour chaque forme.

> **Astuce :** interface ou classe abstraite ? Une interface ne contient aucun code :
> c'est un contrat. Une classe abstraite peut partager du code (ici, `decrire`). Dans
> le doute, commencez par une interface.

## Les membres static

Un membre `static` appartient à la **classe** elle-même, pas à chaque objet. On
l'utilise avec le nom de la classe : `Temperature.depuisFahrenheit(...)`. C'est
pratique pour des constantes ou des fonctions de fabrication.

```typescript
class Temperature {
  static readonly ZERO_ABSOLU = -273.15;
  static creees = 0;

  constructor(public celsius: number) {
    Temperature.creees++;
  }

  static depuisFahrenheit(f: number): Temperature {
    return new Temperature(((f - 32) * 5) / 9);
  }
}

const t = Temperature.depuisFahrenheit(212);
console.log(t.celsius);
console.log(Temperature.ZERO_ABSOLU);
console.log(`Objets créés : ${Temperature.creees}`);
```

```sortie
100
-273.15
Objets créés : 1
```

## Les accesseurs get et set

Un **accesseur** ressemble à un champ, mais c'est une méthode déguisée. `get` calcule
une valeur à la lecture ; `set` contrôle une écriture. On les utilise sans
parenthèses.

```typescript
class Rectangle {
  constructor(private _largeur: number, private hauteur: number) {}

  get aire(): number {
    return this._largeur * this.hauteur;
  }

  get largeur(): number {
    return this._largeur;
  }

  set largeur(valeur: number) {
    if (valeur <= 0) {
      console.log("Largeur refusée");
      return;
    }
    this._largeur = valeur;
  }
}

const r = new Rectangle(2, 3);
console.log(r.aire);
r.largeur = 10;
r.largeur = -1;
console.log(r.aire);
```

```sortie
6
Largeur refusée
30
```

`r.aire` est toujours à jour : il est recalculé à chaque lecture. Un `get` sans `set`
est en lecture seule.

## Erreurs fréquentes

### Oublier d'initialiser un champ

```typescript norun
class Chat {
  nom: string;
}
```

```text
main.ts(2,3): error TS2564: Property 'nom' has no initializer and is not definitely assigned in the constructor.
```

Le champ `nom` n'a jamais de valeur. Donnez-lui une valeur de départ (`nom = ""`), ou
remplissez-le dans le constructeur :
`constructor(public nom: string) {}`.

### Oublier this

```typescript norun
class Compteur {
  valeur = 0;
  incrementer() {
    valeur++;
  }
}
```

```text
main.ts(4,5): error TS2663: Cannot find name 'valeur'. Did you mean the instance member 'this.valeur'?
```

Dans une méthode, un champ s'écrit toujours avec `this.` devant : `this.valeur++`.
Le message vous le suggère d'ailleurs.

### Toucher à un champ privé depuis l'extérieur

```typescript norun
class Compte {
  private solde = 0;
}
const c = new Compte();
console.log(c.solde);
```

```text
main.ts(5,15): error TS2341: Property 'solde' is private and only accessible within class 'Compte'.
```

C'est voulu : le champ est protégé. Ajoutez une méthode publique pour le lire, par
exemple `get montant() { return this.solde; }`.

### Oublier une méthode de l'interface

```typescript norun
interface Animal {
  nom: string;
  crier(): string;
}
class Chien implements Animal {
  constructor(public nom: string) {}
}
```

```text
main.ts(5,7): error TS2420: Class 'Chien' incorrectly implements interface 'Animal'.
  Property 'crier' is missing in type 'Chien' but required in type 'Animal'.
```

La promesse `implements Animal` n'est pas tenue. Ajoutez la méthode manquante :
`crier() { return "Ouaf"; }`.

## Testez-vous

**Question 1 :** que fait ce constructeur ?

```typescript norun
class Livre {
  constructor(public titre: string, private pages: number) {}
}
```

> **Réponse**
>
> Il déclare deux champs et les remplit : `titre`, public, et `pages`, privé. C'est
> la version courte de deux déclarations de champs et de deux lignes
> `this.titre = titre;` et `this.pages = pages;`.

**Question 2 :** peut-on écrire `new Forme()` si `Forme` est une classe abstraite ?

> **Réponse**
>
> Non. TypeScript répond « Cannot create an instance of an abstract class ». Il faut
> créer une classe fille qui écrit les méthodes abstraites, puis faire `new` sur
> cette classe fille.

**Question 3 :** écrivez une classe `Personne` avec un champ `readonly nom` et un
accesseur `get initiale` qui renvoie la première lettre du nom.

> **Réponse**
>
> ```typescript
> class Personne {
>   constructor(readonly nom: string) {}
>
>   get initiale(): string {
>     return this.nom[0] ?? "?";
>   }
> }
>
> const p = new Personne("Zoé");
> console.log(p.initiale);
> ```
>
> ```sortie
> Z
> ```

## En résumé

- Une classe est un moule à objets ; `new` fabrique une instance, `constructor` la
  remplit.
- `constructor(public x: number)` déclare et remplit un champ en une fois.
- `private`, `protected` et `readonly` protègent les champs ; `#` protège aussi à
  l'exécution.
- `implements` vérifie qu'une classe respecte une interface ; `extends` hérite d'une
  classe.
- Une classe `abstract` sert de base et ne s'instancie pas ; `static` appartient à la
  classe ; `get` et `set` déguisent des méthodes en champs.

Suite : [les génériques](generiques.md)
