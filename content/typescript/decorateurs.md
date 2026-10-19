# Les décorateurs

Un décorateur est une fonction qui « emballe » une méthode, un champ ou un accesseur d'une classe pour lui ajouter un comportement, sans toucher à son code.

> **Objectifs :**
>
> - vous saurez ce qu'est un [décorateur](../debuter/glossaire.md#decorateur) et lire la
>   syntaxe `@nom` ;
> - vous saurez écrire un décorateur de méthode, par exemple pour afficher chaque appel ;
> - vous saurez décorer un champ et un accesseur créé avec `accessor` ;
> - vous saurez écrire une fabrique de décorateurs qui reçoit des paramètres ;
> - vous saurez lire l'objet contexte que TypeScript donne au décorateur.

Cette page utilise les décorateurs **standard** de TypeScript 5. Il faut connaître les
[classes](types.md) avant de la lire.

## Un décorateur, c'est un papier cadeau

Imaginez un cadeau. Vous ne changez pas l'objet qui est dedans : vous l'emballez. Le
papier ajoute quelque chose **autour** (un ruban, une étiquette), et l'objet reste le
même à l'intérieur.

Un décorateur fait pareil avec une méthode :

- il **reçoit** la méthode d'origine (le cadeau) ;
- il **renvoie** une nouvelle fonction qui appelle l'originale (le cadeau emballé) ;
- la classe utilise ensuite la version emballée.

```text
   sans décorateur              avec @emballer

   ouvrir()                     ouvrir()
     │                            │
     ▼                            ▼
   ┌────────────┐               ┌──────────────────────┐
   │ "Un vélo !"│               │ ***  ┌────────────┐  │
   └────────────┘               │      │ "Un vélo !"│  │
                                │      └────────────┘ ***
                                └──────────────────────┘
```

On écrit `@` suivi du nom de la fonction, juste au-dessus de ce qu'on décore :

```typescript
function emballer(methode: () => string, contexte: ClassMethodDecoratorContext) {
  return function (this: unknown): string {
    return "*** " + methode.call(this) + " ***";
  };
}

class Cadeau {
  constructor(public contenu: string) {}

  @emballer
  ouvrir(): string {
    return `Un ${this.contenu} !`;
  }
}

const cadeau = new Cadeau("vélo");
console.log(cadeau.ouvrir());
```

```sortie
*** Un vélo ! ***
```

Lisons-le doucement :

- `emballer` est une fonction ordinaire. Elle reçoit deux choses : la méthode d'origine
  (`methode`) et un objet `contexte` (on y revient plus bas) ;
- elle renvoie une **nouvelle** fonction. Cette fonction appelle `methode` et ajoute des
  étoiles autour du résultat ;
- `methode.call(this)` appelle la méthode d'origine avec le bon objet : `this` désigne le
  cadeau sur lequel on a appelé `ouvrir()` ;
- `@emballer` dit : « remplace `ouvrir` par ce que renvoie `emballer` ».

Essayez : enlevez la ligne `@emballer`. Le programme affiche `Un vélo !`, sans étoiles.
Le code de `ouvrir` n'a pas changé : seul l'emballage a disparu.

> **À retenir :** un décorateur est une simple fonction. Elle reçoit la méthode d'origine
> et renvoie une fonction qui la remplace.

## Quand le décorateur s'exécute

Il y a deux moments différents, et c'est souvent ce qui trouble au début :

1. la fonction `emballer` s'exécute **une seule fois**, quand la classe est créée ;
2. la fonction qu'elle renvoie s'exécute **à chaque appel** de la méthode.

Ajoutons des messages pour le voir :

```typescript
function emballer(methode: () => string, contexte: ClassMethodDecoratorContext) {
  console.log(`J'emballe ${String(contexte.name)}`);
  return function (this: unknown): string {
    console.log("  on déballe…");
    return "*** " + methode.call(this) + " ***";
  };
}

class Cadeau {
  constructor(public contenu: string) {}

  @emballer
  ouvrir(): string {
    return `Un ${this.contenu} !`;
  }
}

console.log("Début du programme");
console.log(new Cadeau("vélo").ouvrir());
console.log(new Cadeau("livre").ouvrir());
```

```sortie
J'emballe ouvrir
Début du programme
  on déballe…
*** Un vélo ! ***
  on déballe…
*** Un livre ! ***
```

Suivons le programme pas à pas :

| Étape | Ce qui se passe | Affichage |
|-------|-----------------|-----------|
| 1 | la classe `Cadeau` est créée : `emballer` est appelée pour `ouvrir` | `J'emballe ouvrir` |
| 2 | la ligne `console.log("Début…")` | `Début du programme` |
| 3 | `ouvrir()` sur le vélo : c'est la fonction emballée qui tourne | `  on déballe…` |
| 4 | elle appelle l'originale, qui renvoie `"Un vélo !"`, puis ajoute les étoiles | `*** Un vélo ! ***` |
| 5 | `ouvrir()` sur le livre : même chose, sans rappeler `emballer` | `  on déballe…` puis `*** Un livre ! ***` |

`J'emballe ouvrir` s'affiche **avant** `Début du programme` : le décorateur travaille
dès que la classe est lue, avant même qu'un seul cadeau existe.

Essayez : créez trois cadeaux au lieu de deux. `J'emballe ouvrir` ne s'affiche toujours
qu'une fois ; `on déballe…` s'affiche trois fois.

> **À retenir :** le décorateur emballe une fois, à la création de la classe. La
> fonction emballée tourne à chaque appel.

## Journaliser les appels

Un usage classique : afficher chaque appel d'une méthode, avec ses arguments et son
résultat. C'est **journaliser** (en anglais, *to log*). Pratique pour comprendre un
programme sans ajouter de `console.log` partout.

Pour que ce décorateur marche sur **n'importe quelle** méthode, on le rend
[générique](generiques.md) : `This` est le type de l'objet, `Args` la liste des
paramètres, `R` le type du résultat. TypeScript les devine tout seul.

```typescript
function journaliser<This, Args extends unknown[], R>(
  methode: (this: This, ...args: Args) => R,
  contexte: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  const nom = String(contexte.name);
  return function (this: This, ...args: Args): R {
    console.log(`-> ${nom}(${args.join(", ")})`);
    const resultat = methode.call(this, ...args);
    console.log(`<- ${nom} renvoie ${resultat}`);
    return resultat;
  };
}

class Caisse {
  total = 0;

  @journaliser
  ajouter(prix: number, quantite: number): number {
    this.total += prix * quantite;
    return this.total;
  }
}

const caisse = new Caisse();
caisse.ajouter(3, 2);
caisse.ajouter(10, 1);
console.log(`Total final : ${caisse.total}`);
```

```sortie
-> ajouter(3, 2)
<- ajouter renvoie 6
-> ajouter(10, 1)
<- ajouter renvoie 16
Total final : 16
```

Le dessin d'un appel :

```text
   caisse.ajouter(3, 2)
          │
   ┌──────▼──────────────────────────┐
   │ emballage (journaliser)         │  affiche "-> ajouter(3, 2)"
   │   ┌───────────────────────────┐ │
   │   │ ajouter d'origine         │ │  total = 0 + 3 * 2 = 6
   │   └───────────────────────────┘ │
   │                                 │  affiche "<- ajouter renvoie 6"
   └──────┬──────────────────────────┘
          ▼
          6
```

Et le suivi de `total` :

| Appel | `prix` | `quantite` | `total` avant | `total` après | Résultat |
|-------|--------|------------|---------------|---------------|----------|
| 1 | 3 | 2 | 0 | 6 | 6 |
| 2 | 10 | 1 | 6 | 16 | 16 |

Grâce aux types génériques, la fonction emballée garde exactement la forme de
l'originale : `caisse.ajouter("trois", 2)` reste une erreur de type.

Essayez : ajoutez une méthode `vider(): number` qui remet `total` à 0 et renvoie 0,
décorez-la avec `@journaliser`, et appelez-la. Vous voyez `-> vider()` puis
`<- vider renvoie 0` : le même décorateur sert pour toutes les méthodes.

> **À retenir :** un seul décorateur générique peut emballer toutes les méthodes. Il
> ajoute un comportement commun sans recopier de code.

## L'objet contexte

Le deuxième paramètre d'un décorateur est le **contexte** : une fiche qui décrit ce qui
est décoré. C'est comme l'étiquette collée sur le colis : elle dit ce qu'il y a dedans,
sans l'ouvrir.

| Champ | Ce qu'il contient |
|-------|-------------------|
| `kind` | la sorte d'élément : `"method"`, `"field"`, `"accessor"`, `"getter"`, `"setter"` ou `"class"` |
| `name` | le nom de l'élément (`"ajouter"`) |
| `static` | `true` si l'élément est `static` |
| `private` | `true` si son nom commence par `#` |
| `addInitializer` | une fonction pour lancer du code à la création de chaque objet |

Un décorateur peut décorer n'importe quoi, s'il accepte n'importe quel contexte. Voici un
« espion » qui affiche juste la fiche :

```typescript
function espion(_valeur: unknown, contexte: DecoratorContext) {
  const details = "static" in contexte ? ` static=${contexte.static} private=${contexte.private}` : "";
  console.log(`${contexte.kind} ${String(contexte.name)}${details}`);
}

@espion
class Compte {
  @espion solde = 0;
  @espion static banque = "Banque du Coin";
  @espion #code = 1234;

  @espion
  deposer(montant: number) {
    this.solde += montant;
  }
}

console.log(new Compte().solde);
```

```sortie
method deposer static=false private=false
field solde static=false private=false
field banque static=true private=false
field #code static=false private=true
class Compte
0
```

Remarquez l'ordre : les méthodes d'abord, puis les champs, et la classe en dernier.
`"static" in contexte` vérifie que la fiche a un champ `static` : une fiche de classe
n'en a pas, et TypeScript le sait (c'est un
[rétrécissement](../debuter/glossaire.md#retrecissement)).

### addInitializer : agir sur chaque objet

`addInitializer` enregistre une fonction qui s'exécute **à la création de chaque
objet**. Un exemple utile : « lier » une méthode à son objet.

D'abord, le problème. Quand on sort une méthode de son objet (pour la passer à
quelqu'un d'autre), elle perd son `this` :

```typescript
class Sonnette {
  constructor(public son: string) {}

  sonner() {
    return this?.son ?? "(this a été perdu)";
  }
}

const s = new Sonnette("Ding dong");
const appuyer = s.sonner;
console.log(s.sonner());
console.log(appuyer());
```

```sortie
Ding dong
(this a été perdu)
```

Le décorateur `lier` répare cela : pour chaque nouvel objet, il range dans l'objet une
copie de la méthode attachée pour toujours à cet objet (`bind`).

```typescript
function lier<This, F extends (this: This, ...args: never[]) => unknown>(
  methode: F,
  contexte: ClassMethodDecoratorContext<This, F>,
) {
  contexte.addInitializer(function (this: This) {
    const nom = contexte.name as keyof This;
    this[nom] = methode.bind(this) as This[keyof This];
  });
}

class Sonnette {
  constructor(public son: string) {}

  @lier
  sonner() {
    return this?.son ?? "(this a été perdu)";
  }
}

const s = new Sonnette("Ding dong");
const appuyer = s.sonner;
console.log(appuyer());
```

```sortie
Ding dong
```

> **À retenir :** le contexte décrit l'élément décoré (`kind`, `name`, `static`,
> `private`). `addInitializer` lance du code pour chaque objet créé.

## Décorer un champ

Un **champ** n'a pas de fonction à emballer : il a une **valeur de départ**. Un
décorateur de champ reçoit donc `undefined` comme premier paramètre, et il peut renvoyer
une fonction qui **transforme la valeur de départ**.

C'est comme un tampon à l'entrée d'un bureau : chaque dossier qui arrive reçoit le tampon,
une fois, avant d'être rangé.

```typescript
function enMajuscules(_valeur: undefined, contexte: ClassFieldDecoratorContext<unknown, string>) {
  return function (valeurDeDepart: string): string {
    console.log(`  ${String(contexte.name)} : "${valeurDeDepart}" devient "${valeurDeDepart.toUpperCase()}"`);
    return valeurDeDepart.toUpperCase();
  };
}

class Ville {
  @enMajuscules nom: string;
  @enMajuscules pays = "france";

  constructor(nom: string) {
    this.nom = nom;
  }
}

console.log("Création de Lyon");
const lyon = new Ville("lyon");
console.log(lyon.nom, lyon.pays);
```

```sortie
Création de Lyon
  nom : "undefined" devient "UNDEFINED"
```

Attention, cet exemple **plante** à moitié : regardez la sortie. Le champ `nom` n'a pas
de valeur de départ (il est rempli plus tard, dans le constructeur). Le décorateur reçoit
donc `undefined`… et `toUpperCase` échoue.

Retenez cette règle : un décorateur de champ ne voit **que la valeur de départ**, pas
les valeurs données ensuite. Décorez donc un champ qui a une valeur écrite après `=` :

```typescript
function enMajuscules(_valeur: undefined, contexte: ClassFieldDecoratorContext<unknown, string>) {
  return function (valeurDeDepart: string): string {
    console.log(`  ${String(contexte.name)} : "${valeurDeDepart}" devient "${valeurDeDepart.toUpperCase()}"`);
    return valeurDeDepart.toUpperCase();
  };
}

class Ville {
  @enMajuscules pays = "france";
  @enMajuscules continent = "europe";
  nom = "lyon";
}

console.log("Création");
const ville = new Ville();
console.log(ville.nom, ville.pays, ville.continent);
ville.pays = "italie";
console.log(ville.pays);
```

```sortie
Création
  pays : "france" devient "FRANCE"
  continent : "europe" devient "EUROPE"
lyon FRANCE EUROPE
italie
```

| Étape | `pays` | `continent` | `nom` |
|-------|--------|-------------|-------|
| `new Ville()` commence | (vide) | (vide) | (vide) |
| champ `pays` : `"france"` passe par le décorateur | `"FRANCE"` | (vide) | (vide) |
| champ `continent` : `"europe"` passe par le décorateur | `"FRANCE"` | `"EUROPE"` | (vide) |
| champ `nom` : pas de décorateur | `"FRANCE"` | `"EUROPE"` | `"lyon"` |
| `ville.pays = "italie"` | `"italie"` | `"EUROPE"` | `"lyon"` |

La dernière ligne montre la limite : `"italie"` n'est **pas** mis en majuscules. Pour
surveiller chaque modification, il faut un accesseur (section suivante).

> **À retenir :** un décorateur de champ transforme seulement la valeur de départ, une
> fois par objet.

## Décorer un accesseur avec accessor

Le mot-clé `accessor` devant un champ crée automatiquement un couple `get` / `set`
caché (voir les [accesseurs](types.md)). De l'extérieur, on l'utilise comme un champ
normal. Mais un décorateur peut alors **intercepter chaque lecture et chaque écriture**.

C'est comme un guichet devant un coffre : on ne touche plus le coffre directement, on
passe par le guichetier, qui peut vérifier ce qu'on dépose.

```text
   joueur.vie = 150
          │
   ┌──────▼──────────────┐
   │ guichet (set)       │  150 est trop grand : on garde 100
   └──────┬──────────────┘
          ▼
   ┌─────────────────────┐
   │ coffre caché : 100  │
   └─────────────────────┘
```

Le décorateur reçoit `cible`, qui contient le `get` et le `set` d'origine. Il renvoie un
objet avec les nouvelles versions (`get`, `set`, et `init` pour la valeur de départ ;
chacune est facultative).

```typescript
function entre0et100<This>(
  cible: ClassAccessorDecoratorTarget<This, number>,
  contexte: ClassAccessorDecoratorContext<This, number>,
): ClassAccessorDecoratorResult<This, number> {
  const ramener = (v: number) => Math.min(100, Math.max(0, v));
  return {
    init(depart: number): number {
      return ramener(depart);
    },
    set(this: This, v: number) {
      console.log(`  ${String(contexte.name)} : on demande ${v}, on garde ${ramener(v)}`);
      cible.set.call(this, ramener(v));
    },
  };
}

class Joueur {
  @entre0et100 accessor vie = 150;
}

const joueur = new Joueur();
console.log(joueur.vie);
joueur.vie = 40;
joueur.vie = -10;
console.log(joueur.vie);
```

```sortie
100
  vie : on demande 40, on garde 40
  vie : on demande -10, on garde 0
0
```

| Étape | On écrit | Le guichet garde | `vie` |
|-------|----------|------------------|-------|
| création (`init`) | 150 | 100 | 100 |
| `joueur.vie = 40` | 40 | 40 | 40 |
| `joueur.vie = -10` | -10 | 0 | 0 |

Essayez : enlevez le mot `accessor`. TypeScript refuse le programme : un décorateur
d'accesseur ne s'applique pas à un champ simple (voir les
[erreurs fréquentes](#erreurs-frequentes)).

> **À retenir :** `accessor` transforme un champ en couple `get` / `set`. Son décorateur
> voit chaque écriture, pas seulement la valeur de départ.

## Les fabriques de décorateurs

`entre0et100` est figé sur 0 et 100. Et si on veut 0 à 10 pour un autre champ ? On écrit
une **fabrique** : une fonction qui reçoit des paramètres et **renvoie** un décorateur.

C'est comme une machine à tampons réglable : vous choisissez d'abord le texte du tampon
(`"URGENT"`, `"PAYÉ"`), puis vous tamponnez.

```text
   prefixer("[caisse]")      ->  un décorateur réglé sur "[caisse]"
   @prefixer("[caisse]")     ->  ce décorateur emballe la méthode
```

```typescript
function prefixer(etiquette: string) {
  return function <This, Args extends unknown[]>(
    methode: (this: This, ...args: Args) => string,
    _contexte: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => string>,
  ) {
    return function (this: This, ...args: Args): string {
      return `${etiquette} ${methode.call(this, ...args)}`;
    };
  };
}

class Magasin {
  @prefixer("[caisse]")
  payer(montant: number): string {
    return `${montant} € payés`;
  }

  @prefixer("[stock]")
  ranger(article: string): string {
    return `${article} rangé`;
  }
}

const magasin = new Magasin();
console.log(magasin.payer(12));
console.log(magasin.ranger("savon"));
```

```sortie
[caisse] 12 € payés
[stock] savon rangé
```

Remarquez les **parenthèses** : `@prefixer("[caisse]")` appelle d'abord `prefixer`, qui
renvoie le vrai décorateur. Sans parenthèses, TypeScript croirait que `prefixer`
lui-même est le décorateur (voir les [erreurs fréquentes](#erreurs-frequentes)).

### Une fabrique pour un accesseur

La même idée rend le guichet réglable :

```typescript
function borne(min: number, max: number) {
  return function <This>(
    cible: ClassAccessorDecoratorTarget<This, number>,
    _contexte: ClassAccessorDecoratorContext<This, number>,
  ): ClassAccessorDecoratorResult<This, number> {
    const ramener = (v: number) => Math.min(max, Math.max(min, v));
    return {
      init: ramener,
      set(this: This, v: number) {
        cible.set.call(this, ramener(v));
      },
    };
  };
}

class Reglages {
  @borne(0, 100) accessor volume = 50;
  @borne(1, 5) accessor etoiles = 9;
}

const r = new Reglages();
r.volume = 120;
console.log(r.volume, r.etoiles);
```

```sortie
100 5
```

Essayez : remplacez `@borne(1, 5)` par `@borne(1, 10)`. La note de départ 9 est
gardée telle quelle, et le programme affiche `100 9`.

### Plusieurs décorateurs

On peut empiler les décorateurs, comme plusieurs couches de papier. La couche la plus
**proche** de la méthode est posée en premier ; la plus haute est donc à l'extérieur.

```typescript
function prefixer(etiquette: string) {
  return function <This, Args extends unknown[]>(
    methode: (this: This, ...args: Args) => string,
    _contexte: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => string>,
  ) {
    console.log(`pose de ${etiquette}`);
    return function (this: This, ...args: Args): string {
      return `${etiquette} ${methode.call(this, ...args)}`;
    };
  };
}

class Colis {
  @prefixer("(dessus)")
  @prefixer("(dessous)")
  decrire(): string {
    return "un colis";
  }
}

console.log(new Colis().decrire());
```

```sortie
pose de (dessus)
pose de (dessous)
(dessus) (dessous) un colis
```

Les fabriques sont appelées de haut en bas (d'où l'ordre des messages `pose de`), puis
les couches sont posées de bas en haut. À l'appel, on traverse les couches de
l'extérieur vers l'intérieur :

```text
   ┌─ (dessus) ─────────────────────┐
   │   ┌─ (dessous) ──────────────┐ │
   │   │   decrire() : "un colis" │ │
   │   └──────────────────────────┘ │
   └────────────────────────────────┘
```

> **À retenir :** une fabrique est une fonction qui renvoie un décorateur. On l'écrit
> avec des parenthèses : `@borne(0, 100)`.

## Ce que fait le compilateur

Node 22 ne connaît pas encore la syntaxe `@`. Avec la cible `es2022` (celle des
exemples de ce site), `tsc` **réécrit** donc chaque décorateur en JavaScript ordinaire :
des fonctions d'aide nommées `__esDecorate` et `__runInitializers` appellent vos
décorateurs au bon moment. C'est pour cela que tous les exemples de cette page tournent
dans Node 22.

Vous n'avez rien à régler : sans option particulière, TypeScript 5 utilise ces
décorateurs **standard**.

> **Attention :** beaucoup d'articles et de bibliothèques (Angular, NestJS, TypeORM)
> utilisent les **anciens** décorateurs, activés par l'option
> `"experimentalDecorators": true` dans `tsconfig.json`. Ils reçoivent d'autres
> paramètres (`target, key, descriptor`) et ne se mélangent pas avec ceux de cette page.
> Si un projet utilise déjà cette option, suivez la documentation de sa bibliothèque.

> **À retenir :** les décorateurs standard marchent sans option ; `tsc` les traduit en
> fonctions ordinaires pour Node.

## Erreurs fréquentes

### Écrire un décorateur à l'ancienne

Un exemple copié d'un vieux tutoriel attend trois paramètres :

```typescript norun
function journaliser(cible: any, cle: string, descripteur: PropertyDescriptor) {
  return descripteur;
}

class Calc {
  @journaliser
  ajouter(a: number, b: number): number {
    return a + b;
  }
}
```

```text
main.ts(6,3): error TS1241: Unable to resolve signature of method decorator when called as an expression.
  The runtime will invoke the decorator with 2 arguments, but the decorator expects 3.
main.ts(6,4): error TS1270: Decorator function return type 'PropertyDescriptor' is not assignable to type 'void | ((a: number, b: number) => number)'.
```

Un décorateur standard reçoit **deux** paramètres : la méthode et le contexte. Il renvoie
une fonction (ou rien). Reprenez le modèle de `journaliser` vu plus haut.

### Oublier les parenthèses d'une fabrique

```typescript norun
function borne(min: number, max: number) {
  return function (
    cible: ClassAccessorDecoratorTarget<unknown, number>,
    contexte: ClassAccessorDecoratorContext<unknown, number>,
  ) {
    return cible;
  };
}

class Joueur {
  @borne accessor vie = 100;
}
```

```text
main.ts(11,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'ClassAccessorDecoratorTarget<Joueur, number>' is not assignable to parameter of type 'number'.
main.ts(11,4): error TS1270: Decorator function return type '(cible: ClassAccessorDecoratorTarget<unknown, number>, contexte: ClassAccessorDecoratorContext<unknown, number>) => ClassAccessorDecoratorTarget<unknown, number>' is not assignable to type 'void | ClassAccessorDecoratorResult<Joueur, number>'.
```

`borne` attend `min` et `max`, pas la cible. Appelez la fabrique :
`@borne(0, 100) accessor vie = 100;`.

### Oublier le mot accessor

```typescript norun
function borne<This>(
  cible: ClassAccessorDecoratorTarget<This, number>,
  contexte: ClassAccessorDecoratorContext<This, number>,
) {
  return cible;
}

class Joueur {
  @borne vie = 100;
}
```

```text
main.ts(9,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'undefined' is not assignable to parameter of type 'ClassAccessorDecoratorTarget<Joueur, number>'.
main.ts(9,4): error TS1270: Decorator function return type 'ClassAccessorDecoratorTarget<Joueur, number>' is not assignable to type 'void | ((this: Joueur, value: number) => number)'.
```

Sans `accessor`, `vie` est un champ simple : le décorateur reçoit `undefined`, pas une
cible. Écrivez `@borne accessor vie = 100;`.

### Décorer une fonction hors d'une classe

```typescript norun
function journaliser(methode: Function, contexte: ClassMethodDecoratorContext) {
  return methode;
}

@journaliser
function saluer() {
  console.log("Bonjour");
}
```

```text
main.ts(5,1): error TS1206: Decorators are not valid here.
```

Les décorateurs ne marchent que sur une classe et ses membres. Pour une fonction seule,
emballez-la à la main : `const saluerJournalise = emballer(saluer);`.

## Testez-vous

**Question 1 :** dans quel ordre s'affichent `A` et `B` ?

```typescript
function marquer(_m: () => void, _c: ClassMethodDecoratorContext) {
  console.log("A");
}

class Porte {
  @marquer
  ouvrir() {}
}

console.log("B");
new Porte().ouvrir();
```

> **Réponse**
>
> `A` puis `B`. Le décorateur s'exécute une fois, quand la classe est créée, avant la
> ligne `console.log("B")`. Comme il ne renvoie rien, `ouvrir` reste la méthode
> d'origine.
>
> ```sortie
> A
> B
> ```

**Question 2 :** un décorateur de champ transforme `"paris"` en `"PARIS"`. Que vaut le
champ après `ville.nom = "nice"` ?

> **Réponse**
>
> `"nice"`, en minuscules. Un décorateur de champ ne transforme que la valeur de départ.
> Pour surveiller chaque écriture, utilisez `accessor` et un décorateur d'accesseur.

**Question 3 :** écrivez une fabrique `repeter(n)` qui emballe une méthode renvoyant un
texte et renvoie ce texte répété `n` fois.

> **Réponse**
>
> ```typescript
> function repeter(n: number) {
>   return function <This>(
>     methode: (this: This) => string,
>     _contexte: ClassMethodDecoratorContext<This, (this: This) => string>,
>   ) {
>     return function (this: This): string {
>       return methode.call(this).repeat(n);
>     };
>   };
> }
>
> class Echo {
>   @repeter(3)
>   crier(): string {
>     return "ho ! ";
>   }
> }
>
> console.log(new Echo().crier());
> ```
>
> ```sortie
> ho ! ho ! ho !
> ```

## En résumé

- Un décorateur est une fonction placée avec `@nom` au-dessus d'un membre de classe : il
  l'emballe pour lui ajouter un comportement.
- Un décorateur de méthode reçoit la méthode et le contexte, et renvoie la fonction qui
  la remplace.
- Il s'exécute une fois à la création de la classe ; la fonction emballée, à chaque
  appel.
- Un décorateur de champ ne transforme que la valeur de départ ; avec `accessor`, on
  intercepte chaque lecture et écriture.
- Une fabrique reçoit des paramètres et renvoie un décorateur : `@borne(0, 100)`.
- Les décorateurs standard marchent sans option ; `experimentalDecorators` est l'ancien
  système, différent.

Suite : [Gérer les erreurs](erreurs.md).
