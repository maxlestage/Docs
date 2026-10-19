# Travailler avec du texte

Cette page montre comment manipuler du texte en TypeScript : mesurer, chercher, découper, remplacer, formater des nombres et aligner des colonnes, avec des types qui protègent chaque étape.

> **Objectifs :**
>
> - vous saurez utiliser les méthodes courantes des chaînes : `toUpperCase`, `includes`,
>   `slice`, `split`, `replace`… ;
> - vous saurez lire le type renvoyé par chaque méthode ;
> - vous saurez convertir un texte en nombre et l'inverse ;
> - vous saurez limiter un texte à quelques valeurs avec un type comme `"oui" | "non"` ;
> - vous saurez formater des nombres avec `Intl` et aligner des colonnes avec
>   `padStart`.

## Les chaînes de caractères

Un programme manipule du texte en permanence : un prénom, un message, une adresse
e-mail. Savoir le mesurer et lire ses lettres une à une est la première étape.

Un texte est une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres),
de type `string`. Chaque caractère a une position, son
[indice](../debuter/glossaire.md#indice), qui commence à **0**.

```typescript
const mot: string = "Bonjour";

console.log(mot.length);
console.log(mot[0]);
console.log(mot[3]);
console.log(mot.at(-1));
```

```sortie
7
B
j
r
```

- `length` donne le nombre de caractères. C'est une **propriété** : pas de
  parenthèses.
- `mot[0]` donne le premier caractère, `mot[3]` le quatrième.
- `at(-1)` compte depuis la fin : `-1` est le dernier caractère.

Une [méthode](../debuter/glossaire.md#methode) est une action qu'une valeur sait faire.
On l'appelle avec un point et des parenthèses : `mot.toUpperCase()`. Votre éditeur
connaît toutes les méthodes d'un `string` : tapez `mot.` et il vous les propose, avec
leur type.

## Changer la casse et nettoyer

Un texte tapé par quelqu'un est rarement propre : des espaces en trop, des majuscules
au hasard. Ces méthodes le remettent en ordre, comme on range un tiroir avant de s'en
servir.

```typescript
const saisie = "  Marie Curie  ";

console.log(`[${saisie.toUpperCase()}]`);
console.log(`[${saisie.toLowerCase()}]`);
console.log(`[${saisie.trim()}]`);
console.log(`[${saisie.trimStart()}]`);
```

```sortie
[  MARIE CURIE  ]
[  marie curie  ]
[Marie Curie]
[Marie Curie  ]
```

- `toUpperCase()` met en majuscules, `toLowerCase()` en minuscules ;
- `trim()` enlève les espaces au début et à la fin ; `trimStart()` seulement au début.

Ces méthodes ne modifient pas `saisie` : elles **renvoient un nouveau texte**. Une
chaîne est [immuable](../debuter/glossaire.md#immuable) : on ne peut jamais la changer
sur place. TypeScript l'interdit d'ailleurs : `mot[0] = "b"` est refusé (voir les
[erreurs fréquentes](#erreurs-frequentes)).

> **Astuce :** appliquez `trim()` à tout texte tapé par un utilisateur : les espaces en
> trop sont une source classique de comparaisons ratées.

**À vous :** écrivez un programme qui range `"  bonjour  "` dans une variable,
enlève les espaces autour, puis affiche le résultat en majuscules.

> **Solution**
>
> ```typescript
> const saisie = "  bonjour  ";
> const propre = saisie.trim();
> console.log(propre.toUpperCase());
> ```
>
> ```sortie
> BONJOUR
> ```

## Chercher dans un texte

Avant d'agir, un programme a souvent besoin de vérifier quelque chose : une adresse
contient-elle un `@` ? Un nom de fichier finit-il par `.pdf` ?

Ces méthodes répondent à une question. Leur type de retour vous dit quelle forme a la
réponse :

| Méthode | Question | Renvoie |
|---------|----------|---------|
| `includes(x)` | contient `x` ? | `boolean` |
| `startsWith(x)` | commence par `x` ? | `boolean` |
| `endsWith(x)` | finit par `x` ? | `boolean` |
| `indexOf(x)` | à quelle position est `x` ? | `number` (`-1` si absent) |

```typescript
const phrase = "Le chat dort sur le canapé.";

console.log(phrase.includes("chat"));
console.log(phrase.startsWith("Le"));
console.log(phrase.endsWith("!"));
console.log(phrase.indexOf("dort"));
console.log(phrase.indexOf("chien"));
```

```sortie
true
true
false
8
-1
```

Ligne par ligne :

- `phrase` contient le texte dans lequel on cherche ;
- `includes("chat")` répond `true` : le mot « chat » est bien dans la phrase ;
- `endsWith("!")` répond `false` : la phrase finit par un point, pas par `!` ;
- `indexOf("dort")` donne `8` : en comptant depuis 0, le « d » de « dort » est le
  neuvième caractère ;
- `indexOf("chien")` donne `-1` : « chien » n'apparaît nulle part.

Toutes ces recherches distinguent majuscules et minuscules : `"Chat"` n'est pas
`"chat"`. Pour chercher sans tenir compte de la casse, passez tout en minuscules
d'abord.

```typescript
const titre = "Apprendre TypeScript";
console.log(titre.toLowerCase().includes("typescript"));
```

```sortie
true
```

## Extraire un morceau avec slice

Souvent, seule une partie d'un texte vous intéresse : l'année dans un nom de fichier,
l'extension à la fin. `slice` découpe ce morceau, comme une paire de ciseaux.

`slice(début, fin)` découpe une partie du texte, du caractère `début` **inclus**
jusqu'au caractère `fin` **exclu**. Sans `fin`, elle va jusqu'au bout. Un nombre
négatif compte depuis la fin.

```typescript
const fichier = "rapport-2024.pdf";

console.log(fichier.slice(0, 7));
console.log(fichier.slice(8, 12));
console.log(fichier.slice(-3));
```

```sortie
rapport
2024
pdf
```

Ligne par ligne :

- `fichier` contient `"rapport-2024.pdf"` : le « r » est en position 0, le tiret en
  position 7 ;
- `slice(0, 7)` prend les positions 0 à 6 : `rapport` ;
- `slice(8, 12)` prend les positions 8 à 11, juste après le tiret : `2024` ;
- `slice(-3)` prend les trois derniers caractères : `pdf`.

Pensez aux positions comme aux traits d'une règle : `slice(0, 7)` prend tout ce qui
se trouve entre le trait 0 et le trait 7.

**À vous :** écrivez un programme qui range la date `"2026-10-19"` dans une variable,
puis affiche l'année sur une ligne et le mois sur la suivante, avec `slice`.

> **Solution**
>
> ```typescript
> const date = "2026-10-19";
> console.log(date.slice(0, 4));
> console.log(date.slice(5, 7));
> ```
>
> ```sortie
> 2026
> 10
> ```

## Découper et recoller

Des données arrivent souvent en une seule ligne : une liste de courses séparée par des
virgules, une ligne de tableur séparée par des `;`. Pour traiter chaque élément, il
faut d'abord couper cette ligne.

`split(séparateur)` coupe un texte en morceaux. Elle renvoie un **tableau de textes**,
de type `string[]`. `join(séparateur)` fait l'inverse : elle recolle un tableau en un
seul texte.

```typescript
const ligne = "pomme;poire;kiwi";
const fruits: string[] = ligne.split(";");

console.log(fruits);
console.log(fruits.length);
console.log(fruits.join(", "));
console.log("abc".split(""));
```

```sortie
[ 'pomme', 'poire', 'kiwi' ]
3
pomme, poire, kiwi
[ 'a', 'b', 'c' ]
```

Ligne par ligne :

- `ligne` est un seul texte, avec des `;` entre les fruits ;
- `split(";")` coupe à chaque `;` et range les trois morceaux dans le tableau
  `fruits` ;
- `fruits.length` compte les morceaux : 3 ;
- `join(", ")` recolle les morceaux, en mettant `", "` entre chacun.

`split("")` coupe entre chaque caractère. Les tableaux sont détaillés dans
[le chapitre sur les tableaux](collections.md).

## Remplacer

Corriger un mot, retirer les espaces d'un numéro de téléphone : c'est la fonction
« rechercher et remplacer » d'un traitement de texte. En code, ce sont ces deux
méthodes.

`replace(a, b)` remplace la **première** apparition de `a` par `b`. `replaceAll`
remplace toutes les apparitions.

```typescript
const texte = "Le chat voit le chat.";

console.log(texte.replace("chat", "chien"));
console.log(texte.replaceAll("chat", "chien"));
console.log("06 12 34 56 78".replaceAll(" ", ""));
```

```sortie
Le chien voit le chat.
Le chien voit le chien.
0612345678
```

## Convertir entre texte et nombre

Tout ce qu'un utilisateur tape au clavier arrive sous forme de texte, même un âge ou
un prix. Pour calculer avec, il faut d'abord le convertir.

Un texte qui contient des chiffres reste un texte : `"42"` n'est pas `42`. TypeScript
vous empêche de les confondre. Pour convertir, utilisez :

- `Number(texte)` : texte → nombre (donne `NaN`, « pas un nombre », si c'est
  impossible) ;
- `String(nombre)` ou `nombre.toString()` : nombre → texte.

```typescript
const saisie = "42";
const nombre: number = Number(saisie);

console.log(nombre + 1);
console.log(saisie + 1);
console.log(Number("3,5"), Number("3.5"));
console.log(String(7) + String(7));
```

```sortie
43
421
NaN 3.5
77
```

- `nombre + 1` additionne : `43`.
- `saisie + 1` colle les textes : `"421"`.
- `Number("3,5")` échoue : la virgule décimale s'écrit avec un point en code.

`NaN` est de type `number`, mais ce n'est pas un vrai nombre. Testez-le avec
`Number.isNaN` :

```typescript
const essais = ["12", "douze"];

for (const essai of essais) {
  const n = Number(essai);
  if (Number.isNaN(n)) {
    console.log(`"${essai}" n'est pas un nombre.`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"douze" n'est pas un nombre.
```

**À vous :** écrivez un programme qui range les textes `"20"` et `"22"` dans deux
variables, les convertit en nombres, puis affiche leur somme.

> **Solution**
>
> ```typescript
> const premier = "20";
> const second = "22";
> console.log(Number(premier) + Number(second));
> ```
>
> ```sortie
> 42
> ```

## Les types littéraux : "oui" | "non"

Voici un atout propre à TypeScript. Un type peut accepter **seulement certains
textes**. On liste les valeurs permises, séparées par `|` (« ou ») :

```typescript
type Reponse = "oui" | "non";

function repondre(r: Reponse): string {
  return r === "oui" ? "Parfait !" : "Dommage.";
}

console.log(repondre("oui"));
console.log(repondre("non"));
```

```sortie
Parfait !
Dommage.
```

Ligne par ligne :

- `type Reponse = "oui" | "non";` crée un nouveau type, qui n'accepte que ces deux
  textes ;
- la fonction `repondre` reçoit `r`, de type `Reponse` ;
- `r === "oui" ? "Parfait !" : "Dommage."` choisit : si `r` vaut `"oui"`, le résultat
  est `"Parfait !"`, sinon `"Dommage."` ;
- les deux appels affichent donc une réponse chacun.

`"oui"` et `"non"` sont des **types littéraux** : chacun n'accepte qu'un seul texte.
Leur union `"oui" | "non"` accepte l'un ou l'autre, et rien d'autre. C'est comme un
formulaire à cases à cocher plutôt qu'un champ libre : impossible de répondre « bof ».

- Votre éditeur propose `"oui"` et `"non"` quand vous tapez l'argument.
- Une faute de frappe (`"ouii"`) est refusée avant l'exécution.
- Un `switch` sur ce type peut vérifier qu'aucun cas n'est oublié (voir
  [conditions et boucles](conditions-boucles.md#un-switch-qui-n-oublie-aucun-cas)).

```typescript
type Couleur = "rouge" | "vert" | "bleu";

const preferee: Couleur = "vert";
const toutes: Couleur[] = ["rouge", "vert", "bleu"];

console.log(preferee, toutes.length);
```

```sortie
vert 3
```

> **À retenir :** quand un texte ne peut prendre que quelques valeurs (un statut, une
> direction, une taille), utilisez une union de types littéraux plutôt que `string`.

### Les types gabarits, en bref

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

TypeScript peut même décrire la **forme** d'un texte, avec la syntaxe des gabarits.
Ce sont les *template literal types* :

```typescript
type Pixels = `${number}px`;

const largeur: Pixels = "120px";
const hauteur: Pixels = "80px";

console.log(largeur, hauteur);
```

```sortie
120px 80px
```

`` `${number}px` `` veut dire « un nombre suivi de `px` ». `"120"` ou `"120 px"` seraient
refusés. Vous reverrez cette idée dans [les types littéraux](litteraux.md).

## Formater des nombres avec Intl

Pour afficher un nombre « à la française » (espace entre les milliers, virgule
décimale, symbole €), utilisez `Intl.NumberFormat`. `Intl` (*internationalisation*)
connaît les habitudes de chaque pays.

```typescript
const montant = 1234567.891;

const nombreFr = new Intl.NumberFormat("fr-FR");
const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
const dollars = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
const pourcent = new Intl.NumberFormat("fr-FR", { style: "percent" });

console.log(nombreFr.format(montant));
console.log(euros.format(montant));
console.log(dollars.format(montant));
console.log(pourcent.format(0.256));
```

```sortie
1 234 567,891
1 234 567,89 €
$1,234,567.89
26 %
```

- `"fr-FR"` veut dire « français de France », `"en-US"` « anglais des États-Unis ».
- `style: "currency"` affiche une monnaie ; `currency` donne laquelle.
- `style: "percent"` multiplie par 100 et ajoute `%`.

> **Note :** les espaces du résultat français sont des **espaces insécables** : elles
> empêchent un retour à la ligne au milieu d'un nombre. Elles ressemblent à des espaces
> normales, mais ce sont d'autres caractères. Ne comparez donc pas ce résultat à un
> texte tapé à la main.

Les options sont typées : `{ style: "monnaie" }` serait refusé, car `style` accepte
seulement quelques textes précis (`"decimal"`, `"currency"`, `"percent"`, `"unit"`).
Encore une union de types littéraux !

Pour choisir le nombre de décimales, utilisez `maximumFractionDigits` :

```typescript
const court = new Intl.NumberFormat("fr-FR", { maximumFractionDigits: 1 });
console.log(court.format(3.14159));
console.log(court.format(2));
```

```sortie
3,1
2
```

## Aligner des colonnes avec padStart et padEnd

Quand vous affichez une liste de prix, les chiffres doivent tomber les uns sous les
autres, sinon on lit mal. Ces méthodes ajoutent juste ce qu'il faut de remplissage.

`padStart(longueur, remplissage)` complète un texte **au début** jusqu'à la longueur
voulue. `padEnd` complète **à la fin**. Sans remplissage, ce sont des espaces.

```typescript
console.log("7".padStart(3, "0"));
console.log("42".padStart(5) + "|");
console.log("ok".padEnd(5, ".") + "|");
```

```sortie
007
   42|
ok...|
```

Ligne par ligne :

- `"7".padStart(3, "0")` ajoute des `0` au début jusqu'à 3 caractères : `007` ;
- `"42".padStart(5)` ajoute trois espaces devant `42` ; le `|` montre où le texte
  s'arrête ;
- `"ok".padEnd(5, ".")` ajoute trois points après `ok`, pour arriver à 5 caractères.

C'est parfait pour afficher un tableau bien aligné : les textes à gauche avec `padEnd`,
les nombres à droite avec `padStart`.

```typescript
function ligne(article: string, prix: number): string {
  return article.padEnd(12, ".") + prix.toFixed(2).padStart(8);
}

console.log(ligne("Pain", 1.2));
console.log(ligne("Fromage", 12.5));
console.log(ligne("Café", 104));
console.log("-".repeat(20));
console.log("Total".padEnd(12) + (1.2 + 12.5 + 104).toFixed(2).padStart(8));
```

```sortie
Pain........    1.20
Fromage.....   12.50
Café........  104.00
--------------------
Total         117.70
```

`repeat(20)` répète un texte 20 fois. Les points décimaux des prix sont alignés, comme sur un
ticket de caisse.

**À vous :** écrivez un programme qui affiche les numéros `"1"`, `"25"` et `"300"` sur
trois chiffres, en complétant avec des zéros au début.

> **Solution**
>
> ```typescript
> console.log("1".padStart(3, "0"));
> console.log("25".padStart(3, "0"));
> console.log("300".padStart(3, "0"));
> ```
>
> ```sortie
> 001
> 025
> 300
> ```

## Erreurs fréquentes

### Modifier un caractère d'une chaîne

```typescript norun
let mot = "chat";
mot[0] = "C";
```

```text
main.ts:2:1 - error TS2542: Index signature in type 'String' only permits reading.

2 mot[0] = "C";
  ~~~~~~


Found 1 error in main.ts:2
```

« Les positions d'un `String` sont en lecture seule. » Une chaîne est immuable.
Construisez un nouveau texte : `mot = "C" + mot.slice(1);`.

### Appeler length comme une méthode

```typescript norun
const prenom = "Alice";
console.log(prenom.length());
```

```text
main.ts:2:20 - error TS2349: This expression is not callable.
  Type 'Number' has no call signatures.

2 console.log(prenom.length());
                     ~~~~~~


Found 1 error in main.ts:2
```

« Cette expression ne peut pas être appelée : c'est un nombre. » `length` est une
propriété, pas une méthode. Écrivez `prenom.length`, sans parenthèses.

### Additionner un texte et un nombre

```typescript norun
const age = "30";
const dansDixAns: number = age + 10;
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const dansDixAns: number = age + 10;
        ~~~~~~~~~~


Found 1 error in main.ts:2
```

`age` est un texte, donc `age + 10` colle `"30"` et `"10"` : le résultat est le texte
`"3010"`. Convertissez d'abord : `Number(age) + 10`.

### Donner une valeur hors de la liste

```typescript norun
type Reponse = "oui" | "non";
function repondre(r: Reponse): void {
  console.log(`Vous avez dit ${r}.`);
}
repondre("peut-être");
```

```text
main.ts:5:10 - error TS2345: Argument of type '"peut-être"' is not assignable to parameter of type 'Reponse'.

5 repondre("peut-être");
           ~~~~~~~~~~~


Found 1 error in main.ts:5
```

`Reponse` n'accepte que `"oui"` ou `"non"`. Utilisez une valeur permise, ou ajoutez
`"peut-être"` au type si c'est une vraie réponse possible.

## Testez-vous

**Question 1 :** qu'affiche `"TypeScript".slice(4, 10).toLowerCase()` ?

> **Réponse**
>
> `script`. `slice(4, 10)` prend les caractères des positions 4 à 9 (`Script`), puis
> `toLowerCase()` les met en minuscules.

**Question 2 :** quel est le type de `"a-b-c".split("-")` ?

> **Réponse**
>
> `string[]`, un tableau de textes : `["a", "b", "c"]`.

**Question 3 :** écrivez un type `Direction` qui accepte seulement `"haut"`, `"bas"`,
`"gauche"` et `"droite"`, puis une variable de ce type.

> **Réponse**
>
> ```typescript
> type Direction = "haut" | "bas" | "gauche" | "droite";
>
> const d: Direction = "gauche";
> console.log(d);
> ```
>
> ```sortie
> gauche
> ```

## En résumé

- Un `string` est immuable : ses méthodes renvoient toujours un nouveau texte.
- `length` mesure, `includes` / `startsWith` / `indexOf` cherchent, `slice` extrait,
  `split` découpe en `string[]`, `replaceAll` remplace.
- `Number(texte)` convertit en nombre (`NaN` si impossible), `String(n)` en texte.
- Une union de types littéraux comme `"oui" | "non"` limite un texte à quelques
  valeurs, vérifiées avant l'exécution.
- `Intl.NumberFormat("fr-FR", ...)` formate nombres, monnaies et pourcentages ;
  `padStart` et `padEnd` alignent des colonnes.

Suite : [tableaux et tuples](collections.md).
