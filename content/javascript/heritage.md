# Héritage et prototypes

Cette page montre comment une classe peut réutiliser une autre classe avec `extends` et `super`, ce qu'est un prototype, et quand préférer la composition.

> **Objectifs :**
>
> - vous saurez créer une sous-classe avec `extends` ;
> - vous saurez appeler le parent avec `super` ;
> - vous saurez redéfinir une méthode et profiter du polymorphisme ;
> - vous comprendrez simplement ce qu'est un prototype ;
> - vous saurez choisir entre héritage, composition et mixin.

## Créer une sous-classe avec extends

L'[héritage](../debuter/glossaire.md#heritage) permet de créer une classe à partir d'une
autre. La nouvelle classe, la **sous-classe** (ou classe enfant), reçoit tout ce que
possède la classe **parente** : ses propriétés et ses méthodes.

Pensez à une famille de véhicules : une voiture *est un* véhicule, un vélo *est un*
véhicule. Tous roulent ; chacun ajoute ses particularités.

```javascript
class Animal {
  constructor(nom) {
    this.nom = nom;
  }

  manger() {
    console.log(`${this.nom} mange.`);
  }
}

class Chien extends Animal {
  aboyer() {
    console.log(`${this.nom} aboie : Wouf !`);
  }
}

const rex = new Chien("Rex");
rex.manger();
rex.aboyer();
console.log(rex instanceof Chien, rex instanceof Animal);
```

```sortie
Rex mange.
Rex aboie : Wouf !
true true
```

`Chien` n'a pas écrit de `constructor` ni de `manger` : il les hérite d'`Animal`. Un
chien est aussi un animal, donc `instanceof Animal` répond `true`.

## Le constructeur et super

Si la sous-classe a besoin de son propre constructeur, elle doit d'abord appeler celui du
parent avec `super(...)`. C'est le parent qui prépare la base de l'objet ; l'enfant
complète ensuite.

```javascript
class Animal {
  constructor(nom) {
    this.nom = nom;
  }
}

class Chien extends Animal {
  constructor(nom, race) {
    super(nom);
    this.race = race;
  }
}

const rex = new Chien("Rex", "labrador");
console.log(rex);
```

```sortie
Chien { nom: 'Rex', race: 'labrador' }
```

> **À retenir :** dans le constructeur d'une sous-classe, appelez `super(...)` **avant**
> d'utiliser `this`.

## Redéfinir une méthode

Une sous-classe peut **redéfinir** une méthode du parent : elle écrit une méthode du même
nom, qui remplace celle du parent pour ses objets.

```javascript
class Animal {
  constructor(nom) {
    this.nom = nom;
  }

  parler() {
    return "...";
  }
}

class Chat extends Animal {
  parler() {
    return "Miaou";
  }
}

console.log(new Animal("Bête").parler());
console.log(new Chat("Minou").parler());
```

```sortie
...
Miaou
```

### Compléter la méthode du parent avec super

Parfois, on ne veut pas remplacer la méthode du parent, mais l'**enrichir**.
`super.methode()` appelle la version du parent.

```javascript
class Employe {
  constructor(nom, salaire) {
    this.nom = nom;
    this.salaire = salaire;
  }

  decrire() {
    return `${this.nom}, salaire ${this.salaire} €`;
  }
}

class Chef extends Employe {
  constructor(nom, salaire, equipe) {
    super(nom, salaire);
    this.equipe = equipe;
  }

  decrire() {
    return `${super.decrire()}, chef de ${this.equipe.length} personnes`;
  }
}

console.log(new Employe("Léa", 2000).decrire());
console.log(new Chef("Sam", 3000, ["Léa", "Tom"]).decrire());
```

```sortie
Léa, salaire 2000 €
Sam, salaire 3000 €, chef de 2 personnes
```

## Le polymorphisme

« Polymorphisme » veut dire « plusieurs formes ». Des objets de classes différentes
répondent au **même appel** de méthode, chacun à sa façon. Le code qui les utilise n'a
pas besoin de savoir qui est qui.

Voici une liste de formes. Chacune sait calculer son aire ; la boucle appelle simplement
`aire()`.

```javascript
class Forme {
  constructor(nom) {
    this.nom = nom;
  }

  aire() {
    throw new Error("aire() doit être redéfinie");
  }

  toString() {
    return `${this.nom} : ${this.aire().toFixed(2)}`;
  }
}

class Rectangle extends Forme {
  constructor(largeur, hauteur) {
    super("Rectangle");
    this.largeur = largeur;
    this.hauteur = hauteur;
  }

  aire() {
    return this.largeur * this.hauteur;
  }
}

class Carre extends Rectangle {
  constructor(cote) {
    super(cote, cote);
    this.nom = "Carré";
  }
}

class Cercle extends Forme {
  constructor(rayon) {
    super("Cercle");
    this.rayon = rayon;
  }

  aire() {
    return Math.PI * this.rayon ** 2;
  }
}

const formes = [new Rectangle(2, 3), new Carre(4), new Cercle(1)];
for (const forme of formes) {
  console.log(String(forme));
}

const total = formes.reduce((somme, f) => somme + f.aire(), 0);
console.log(`Aire totale : ${total.toFixed(2)}`);
```

```sortie
Rectangle : 6.00
Carré : 16.00
Cercle : 3.14
Aire totale : 25.14
```

Trois idées dans cet exemple :

- `Forme.aire` lève une erreur : c'est une façon de dire « chaque sous-classe **doit**
  écrire sa propre version » ;
- `toString` est écrite une seule fois, dans `Forme`, et appelle `this.aire()` : la
  bonne version est choisie selon l'objet ;
- `Carre` hérite de `Rectangle`, qui hérite de `Forme` : l'héritage peut avoir plusieurs
  étages.

Pour ajouter un triangle, il suffit d'écrire une classe `Triangle` : la boucle ne change
pas.

## Les prototypes, simplement

Sous le capot, JavaScript n'a pas vraiment de « classes » comme d'autres langages. Il a
des **prototypes**. Le mot `class` est une écriture plus agréable de ce mécanisme.

Chaque objet a un lien caché vers un autre objet : son **prototype**. Quand vous
demandez une propriété, JavaScript la cherche d'abord dans l'objet. S'il ne la trouve
pas, il la cherche dans le prototype, puis dans le prototype du prototype, etc. C'est la
**chaîne des prototypes**.

Pensez à un enfant qui ne sait pas répondre : il demande à son parent, qui demande au
grand-parent, et ainsi de suite.

`Object.getPrototypeOf(objet)` montre ce lien.

```javascript
class Animal {
  constructor(nom) {
    this.nom = nom;
  }

  parler() {
    return "...";
  }
}

class Chien extends Animal {}

const rex = new Chien("Rex");

console.log(Object.getPrototypeOf(rex) === Chien.prototype);
console.log(Object.getPrototypeOf(Chien.prototype) === Animal.prototype);
console.log(Object.hasOwn(rex, "nom"));
console.log(Object.hasOwn(rex, "parler"));
console.log(Object.getOwnPropertyNames(Animal.prototype));
```

```sortie
true
true
true
false
[ 'constructor', 'parler' ]
```

Que se passe-t-il quand on écrit `rex.parler()` ?

```text
rex                 → nom: "Rex"             pas de parler
  ↓ prototype
Chien.prototype     → (rien de plus)         pas de parler
  ↓ prototype
Animal.prototype    → parler()               trouvé !
  ↓ prototype
Object.prototype    → toString(), hasOwnProperty()…
  ↓ prototype
null                → fin de la chaîne
```

- Les **données** (`nom`) sont rangées dans chaque objet : `Object.hasOwn(rex, "nom")`
  répond `true`.
- Les **méthodes** sont rangées une seule fois, dans le prototype de la classe : tous les
  objets les partagent.

C'est aussi pour cela que tout objet possède `toString` : la chaîne finit toujours par
`Object.prototype`.

> **Note :** vous n'avez presque jamais besoin de manipuler les prototypes à la main.
> Écrivez des classes ; retenez seulement que les méthodes sont partagées par la chaîne
> des prototypes.

## Préférer la composition

L'héritage exprime une relation « **est un** » : un chien *est un* animal. Mais souvent,
la relation est « **a un** » : une voiture *a un* moteur. Une voiture n'est pas un
moteur ! Dans ce cas, on range l'objet dans une propriété : c'est la **composition**.

```javascript
class Moteur {
  constructor(chevaux) {
    this.chevaux = chevaux;
  }

  demarrer() {
    return `Vroum (${this.chevaux} ch)`;
  }
}

class Voiture {
  constructor(marque, moteur) {
    this.marque = marque;
    this.moteur = moteur;
  }

  demarrer() {
    console.log(`${this.marque} : ${this.moteur.demarrer()}`);
  }
}

const petite = new Voiture("Citadine", new Moteur(70));
const grosse = new Voiture("Camion", new Moteur(400));
petite.demarrer();
grosse.demarrer();
```

```sortie
Citadine : Vroum (70 ch)
Camion : Vroum (400 ch)
```

La composition est plus souple : on change de moteur sans toucher à `Voiture`. Les
longues chaînes d'héritage (`A` → `B` → `C` → `D`) deviennent vite difficiles à
comprendre.

> **Astuce :** en cas de doute, demandez-vous « est-ce que X **est un** Y ? ». Si la
> phrase sonne faux, choisissez la composition.

## Les mixins avec Object.assign

Une classe n'a qu'**un seul** parent. Comment donner la même capacité (par exemple
« savoir nager ») à des classes sans lien entre elles ? Avec un
[mixin](../debuter/glossaire.md#mixin) : un simple objet qui contient des méthodes, que
l'on copie dans le prototype d'une classe.

`Object.assign(cible, source)` copie les propriétés de `source` dans `cible`.

```javascript
const Nageur = {
  nager() {
    return `${this.nom} nage.`;
  },
};

const Volant = {
  voler() {
    return `${this.nom} vole.`;
  },
};

class Canard {
  constructor(nom) {
    this.nom = nom;
  }
}

class Poisson {
  constructor(nom) {
    this.nom = nom;
  }
}

Object.assign(Canard.prototype, Nageur, Volant);
Object.assign(Poisson.prototype, Nageur);

const donald = new Canard("Donald");
const nemo = new Poisson("Nemo");
console.log(donald.nager());
console.log(donald.voler());
console.log(nemo.nager());
console.log(typeof nemo.voler);
```

```sortie
Donald nage.
Donald vole.
Nemo nage.
undefined
```

Le canard sait nager et voler ; le poisson sait seulement nager. Les méthodes du mixin
utilisent `this.nom` : elles fonctionnent avec n'importe quel objet qui a un `nom`.

## Erreurs fréquentes

### Utiliser this avant super

```javascript norun
class Animal {
  constructor(nom) { this.nom = nom; }
}
class Chien extends Animal {
  constructor(nom, race) {
    this.race = race;
    super(nom);
  }
}
new Chien("Rex", "labrador");
```

```text
file:///home/vous/projet/main.mjs:6
    this.race = race;
    ^

ReferenceError: Must call super constructor in derived class before accessing 'this' or returning from derived constructor
    at new Chien (file:///home/vous/projet/main.mjs:6:5)
    at file:///home/vous/projet/main.mjs:10:1
```

Le message dit : « appelez le constructeur parent avant d'utiliser `this` ». Placez
`super(nom);` en **première** ligne du constructeur. Le même message apparaît si vous
oubliez complètement `super`.

### Oublier super. en complétant une méthode

```javascript norun
class Animal {
  decrire() { return "un animal"; }
}
class Chien extends Animal {
  decrire() { return this.decrire() + " qui aboie"; }
}
console.log(new Chien().decrire());
```

```text
file:///home/vous/projet/main.mjs:5
  decrire() { return this.decrire() + " qui aboie"; }
                          ^

RangeError: Maximum call stack size exceeded
    at Chien.decrire (file:///home/vous/projet/main.mjs:5:27)
    at Chien.decrire (file:///home/vous/projet/main.mjs:5:27)
    at Chien.decrire (file:///home/vous/projet/main.mjs:5:27)
```

`this.decrire()` appelle la méthode du **chien**, c'est-à-dire elle-même, à l'infini.
La pile d'appels déborde. Pour appeler la version du parent, écrivez `super.decrire()`.

### Mal orthographier une méthode redéfinie

```javascript norun
class Forme { aire() { throw new Error("aire() doit être redéfinie"); } }
class Carre extends Forme { constructor(c){super(); this.c=c;} air(){ return this.c**2; } }
console.log(new Carre(2).aire());
```

```text
file:///home/vous/projet/main.mjs:1
class Forme { aire() { throw new Error("aire() doit être redéfinie"); } }
                             ^

Error: aire() doit être redéfinie
    at Carre.aire (file:///home/vous/projet/main.mjs:1:30)
    at file:///home/vous/projet/main.mjs:3:26
```

`air` n'est pas `aire` : la méthode n'est pas redéfinie, c'est celle du parent qui
s'exécute. Vérifiez que le nom est **exactement** le même que dans la classe parente.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
class A { nom() { return "A"; } }
class B extends A { nom() { return "B" + super.nom(); } }
class C extends B {}
console.log(new C().nom());
```

> **Réponse**
>
> `BA`. `C` n'a pas de méthode `nom` : JavaScript la trouve dans `B`, qui ajoute `"B"`
> devant le résultat de la version de `A`.

**Question 2 :** une `Maison` a des `Piece`s. Faut-il écrire `class Maison extends
Piece` ?

> **Réponse**
>
> Non. Une maison **a des** pièces, elle n'**est pas** une pièce. Utilisez la
> composition : `this.pieces = [new Piece(...), ...]`.

**Question 3 :** où est rangée la méthode `parler` d'un objet `new Chat()` : dans
l'objet lui-même, ou ailleurs ?

> **Réponse**
>
> Dans `Chat.prototype` (ou plus haut dans la chaîne). L'objet ne contient que ses
> données ; `Object.hasOwn(objet, "parler")` répond `false`. Toutes les instances
> partagent la même méthode.

## En résumé

- `class B extends A` : `B` hérite des propriétés et méthodes de `A`.
- Dans le constructeur enfant, `super(...)` vient avant tout `this`.
- Une méthode du même nom redéfinit celle du parent ; `super.methode()` appelle le parent.
- Le polymorphisme : des objets différents répondent au même appel, chacun à sa façon.
- Les méthodes vivent dans les prototypes, reliés en chaîne jusqu'à `Object.prototype`.
- « Est un » → héritage ; « a un » → composition ; capacité partagée → mixin.

Suite : [portée et closures](closures.md). Comparez avec
[l'héritage en Ruby](../ruby/heritage.md).
