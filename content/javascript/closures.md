# Portée et closures

Cette page explique où une variable est visible, comment une fonction se souvient de ses variables (une closure), et ce que vaut `this` selon la façon d'écrire une fonction.

> **Objectifs :**
>
> - vous saurez où une variable déclarée avec `let` ou `const` est visible ;
> - vous comprendrez qu'une fonction se souvient des variables qui l'entourent ;
> - vous saurez fabriquer des fonctions avec un état privé ;
> - vous saurez éviter le piège de `var` dans une boucle ;
> - vous saurez ce que vaut `this` dans une fonction fléchée et avec `bind`.

## La portée d'une variable

La [portée](../debuter/glossaire.md#portee) d'une variable, c'est la zone du programme où
elle existe. Avec `let` et `const`, cette zone est le **bloc** : tout ce qui se trouve
entre les accolades `{ }` où la variable est déclarée.

Pensez aux pièces d'une maison : ce qui est rangé dans la cuisine n'est pas visible
depuis le salon. Mais depuis la cuisine, on voit ce qui est dans l'entrée commune.

```javascript
const ville = "Lyon";

if (true) {
  const quartier = "Croix-Rousse";
  console.log(ville, quartier);
}

console.log(ville);
console.log(typeof quartier);
```

```sortie
Lyon Croix-Rousse
Lyon
undefined
```

- `ville` est déclarée tout en haut : elle est visible partout.
- `quartier` est déclarée dans le bloc du `if` : elle disparaît à l'accolade fermante.

> **Note :** `typeof` est le seul moyen de toucher une variable inexistante sans erreur :
> il répond `"undefined"`. Écrire directement `quartier` lèverait une `ReferenceError`.

### Les blocs s'emboîtent

Un bloc intérieur voit les variables des blocs qui l'entourent. L'inverse est faux.
Une fonction a aussi sa propre portée : ses paramètres et ses variables restent à
l'intérieur.

```javascript
const taux = 0.2;

function prixTTC(prixHT) {
  const tva = prixHT * taux;
  return prixHT + tva;
}

console.log(prixTTC(100));
console.log(typeof tva);
```

```sortie
120
undefined
```

`prixTTC` lit `taux`, déclarée dehors. Mais `tva` n'existe que pendant l'appel.

### Le masquage

Une variable intérieure peut porter le même nom qu'une variable extérieure. Elle la
**masque** : dans le bloc, c'est elle qu'on voit.

```javascript
const nom = "extérieur";

{
  const nom = "intérieur";
  console.log(nom);
}

console.log(nom);
```

```sortie
intérieur
extérieur
```

> **Astuce :** évitez de réutiliser les mêmes noms : le code est plus facile à lire.

## Une fonction se souvient de ses variables

Voici l'idée centrale de ce chapitre. Une fonction créée **à l'intérieur** d'une autre
fonction garde l'accès aux variables qui l'entouraient, **même après** que la fonction
extérieure a fini.

Ce couple « fonction + variables dont elle se souvient » s'appelle une
[closure](../debuter/glossaire.md#closure) (« fermeture » en français). Imaginez un sac à
dos : en sortant de la maison, la fonction emporte les variables dont elle a besoin.

```javascript
function creerSalutation(formule) {
  return function (prenom) {
    return `${formule}, ${prenom} !`;
  };
}

const bonjour = creerSalutation("Bonjour");
const salut = creerSalutation("Salut");

console.log(bonjour("Ada"));
console.log(salut("Linus"));
```

```sortie
Bonjour, Ada !
Salut, Linus !
```

Quand on appelle `bonjour("Ada")`, `creerSalutation` a terminé depuis longtemps. Pourtant
la fonction se souvient que `formule` vaut `"Bonjour"`. Chaque appel à `creerSalutation`
crée un nouveau sac à dos : `bonjour` et `salut` ont chacun le leur.

## Une fabrique de compteurs

Le sac à dos ne contient pas une copie figée : la fonction peut **modifier** les
variables dont elle se souvient. On peut ainsi fabriquer un compteur.

```javascript
function creerCompteur() {
  let compte = 0;
  return () => {
    compte += 1;
    return compte;
  };
}

const compteurA = creerCompteur();
const compteurB = creerCompteur();

console.log(compteurA());
console.log(compteurA());
console.log(compteurA());
console.log(compteurB());
```

```sortie
1
2
3
1
```

`compteurA` et `compteurB` ont chacun leur propre variable `compte`. Appeler l'un ne
change pas l'autre.

## Un état privé

Personne, à part la fonction, ne peut toucher à `compte` : il n'existe aucun moyen
d'écrire `compteurA.compte`. La closure donne un **état privé**, comme les champs `#`
d'une [classe](types.md#les-champs-prives).

On peut renvoyer plusieurs fonctions dans un objet ; elles partagent le même sac à dos.

```javascript
function creerTirelire() {
  let solde = 0;

  return {
    deposer(montant) {
      if (montant > 0) {
        solde += montant;
      }
    },
    retirer(montant) {
      if (montant > solde) {
        console.log("Pas assez d'argent.");
        return;
      }
      solde -= montant;
    },
    voirSolde() {
      return solde;
    },
  };
}

const tirelire = creerTirelire();
tirelire.deposer(20);
tirelire.retirer(50);
tirelire.retirer(5);
console.log(tirelire.voirSolde());
console.log(tirelire.solde);
```

```sortie
Pas assez d'argent.
15
undefined
```

`tirelire.solde` vaut `undefined` : le solde est bien caché. Pour le changer, il faut
passer par `deposer` et `retirer`, qui vérifient les montants.

### Se souvenir pour aller plus vite

Une closure peut aussi garder un **cache** : la fonction mémorise les résultats déjà
calculés.

```javascript
function creerCarre() {
  const deja = new Map();
  return (n) => {
    if (deja.has(n)) {
      console.log(`(${n} déjà connu)`);
      return deja.get(n);
    }
    const resultat = n * n;
    deja.set(n, resultat);
    return resultat;
  };
}

const carre = creerCarre();
console.log(carre(9));
console.log(carre(9));
```

```sortie
81
(9 déjà connu)
81
```

## Le piège de var dans une boucle

Avant `let` et `const`, on déclarait les variables avec `var`. Vous en verrez dans du
vieux code. `var` n'a **pas** de portée de bloc : la variable existe dans toute la
fonction. Dans une boucle, cela donne une seule variable partagée par tous les tours.

```javascript
const fonctionsVar = [];
for (var i = 0; i < 3; i++) {
  fonctionsVar.push(() => i);
}
console.log(fonctionsVar.map((f) => f()));

const fonctionsLet = [];
for (let j = 0; j < 3; j++) {
  fonctionsLet.push(() => j);
}
console.log(fonctionsLet.map((f) => f()));
```

```sortie
[ 3, 3, 3 ]
[ 0, 1, 2 ]
```

Avec `var`, les trois fonctions se souviennent de **la même** variable `i`. Quand on les
appelle, la boucle est finie et `i` vaut `3`. Avec `let`, chaque tour de boucle crée une
**nouvelle** variable `j` : chaque fonction garde la sienne.

> **À retenir :** n'utilisez jamais `var`. Écrivez `const` par défaut, et `let` quand la
> valeur doit changer.

## this dans les fonctions

Dans une méthode, `this` désigne l'objet sur lequel on l'appelle. Mais attention : pour
une fonction écrite avec `function`, `this` dépend de **la façon dont elle est
appelée**, pas de l'endroit où elle est écrite.

Une fonction fléchée `=>` n'a pas de `this` à elle : elle utilise celui de l'endroit où
elle est écrite, comme n'importe quelle autre variable de son sac à dos.

```javascript
class Minuteur {
  constructor(nom) {
    this.nom = nom;
  }

  avecFleche() {
    [1, 2].forEach((n) => {
      console.log(`${this.nom} : ${n}`);
    });
  }

  avecFunction() {
    [1, 2].forEach(function (n) {
      console.log(`${this === undefined ? "this perdu" : this.nom} : ${n}`);
    });
  }
}

const m = new Minuteur("Tic");
m.avecFleche();
m.avecFunction();
```

```sortie
Tic : 1
Tic : 2
this perdu : 1
this perdu : 2
```

Dans `avecFunction`, la fonction est appelée par `forEach`, pas par l'objet : son `this`
est `undefined`.

> **Astuce :** pour une fonction passée à une autre (`forEach`, `map`, `setTimeout`…),
> utilisez une fonction fléchée. Pour une méthode d'objet ou de classe, utilisez
> l'écriture normale `methode() { }`.

## Fixer this avec bind

Le même problème arrive quand on **détache** une méthode de son objet, par exemple pour
la ranger dans une variable. `bind(objet)` fabrique une nouvelle fonction dont le `this`
est fixé une fois pour toutes.

```javascript
const chat = {
  nom: "Minou",
  presenter() {
    return `Je suis ${this.nom}`;
  },
};

console.log(chat.presenter());

const detachee = chat.presenter;
console.log(detachee === chat.presenter);

const attachee = chat.presenter.bind(chat);
console.log(attachee());
```

```sortie
Je suis Minou
true
Je suis Minou
```

`detachee()` ferait une erreur : appelée seule, sans `chat.` devant, elle n'a plus de
`this`. `attachee`, elle, se souvient de `chat`.

## Erreurs fréquentes

### Utiliser une variable hors de son bloc

```javascript norun
if (true) {
  let message = "bonjour";
}
console.log(message);
```

```text
file:///home/vous/projet/main.mjs:4
console.log(message);
            ^

ReferenceError: message is not defined
    at file:///home/vous/projet/main.mjs:4:13
```

`message` n'existe que dans le bloc du `if`. Déclarez-la **avant** le bloc
(`let message;`), puis donnez-lui sa valeur dedans.

### Utiliser une variable avant sa déclaration

```javascript norun
console.log(total);
let total = 10;
```

```text
file:///home/vous/projet/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
```

Le message dit : « impossible d'accéder à `total` avant son initialisation ». Déclarez
toujours une variable avant de l'utiliser.

### Perdre this en détachant une méthode

```javascript norun
const chat = { nom: "Minou", presenter() { return "Je suis " + this.nom; } };
const f = chat.presenter;
console.log(f());
```

```text
file:///home/vous/projet/main.mjs:1
const chat = { nom: "Minou", presenter() { return "Je suis " + this.nom; } };
                                                                    ^

TypeError: Cannot read properties of undefined (reading 'nom')
    at presenter (file:///home/vous/projet/main.mjs:1:69)
    at file:///home/vous/projet/main.mjs:3:13
```

`f()` est appelée sans objet devant : `this` vaut `undefined`. Appelez
`chat.presenter()`, ou fixez `this` avec `chat.presenter.bind(chat)`. Le même message
apparaît avec une `function` passée à `forEach` : utilisez une fonction fléchée.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```javascript norun
function creer() {
  let n = 10;
  return () => (n -= 1);
}
const a = creer();
const b = creer();
a();
a();
console.log(a(), b());
```

> **Réponse**
>
> `7 9`. `a` et `b` ont chacun leur propre `n`. Après trois appels, celui de `a` vaut
> `7` ; celui de `b` a été diminué une seule fois : `9`.

**Question 2 :** pourquoi une boucle `for (var i ...)` qui crée des fonctions donne-t-elle
souvent la même valeur partout ?

> **Réponse**
>
> Parce que `var` crée **une seule** variable pour toute la boucle. Toutes les fonctions
> s'en souviennent, et la lisent quand la boucle est finie. Avec `let`, chaque tour a sa
> propre variable.

**Question 3 :** écrivez une fonction `creerAccumulateur()` qui renvoie une fonction :
chaque appel `ajouter(x)` ajoute `x` à un total caché et renvoie le nouveau total.

> **Réponse**
>
> ```javascript
> function creerAccumulateur() {
>   let total = 0;
>   return (x) => {
>     total += x;
>     return total;
>   };
> }
>
> const ajouter = creerAccumulateur();
> ajouter(5);
> ajouter(10);
> console.log(ajouter(1));
> ```
>
> ```sortie
> 16
> ```

## En résumé

- Une variable `let` ou `const` n'existe que dans son bloc `{ }` ; un bloc intérieur voit
  les variables extérieures.
- Une closure est une fonction qui se souvient des variables qui l'entouraient.
- Une fabrique (`creerCompteur`) donne à chaque fonction son propre état privé.
- `var` n'a pas de portée de bloc : ne l'utilisez pas.
- Une fonction fléchée garde le `this` de l'endroit où elle est écrite ; `bind` fixe
  le `this` d'une fonction.

Suite : [itérateurs et générateurs](iterateurs.md). Comparez avec
[les closures en Rust](../rust/fonctions.md).
