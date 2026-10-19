# Travailler avec du texte

Cette page montre comment manipuler du texte en JavaScript : lire ses caractères, le découper, le transformer, le chercher et le trier dans l'ordre alphabétique français.

> **Objectifs :**
>
> - vous saurez lire la longueur d'un texte et un caractère à une position donnée ;
> - vous saurez extraire un morceau avec `slice` et nettoyer un texte avec `trim` ;
> - vous saurez découper avec `split` et recoller avec `join` ;
> - vous saurez chercher et remplacer dans un texte ;
> - vous saurez trier des mots français correctement avec `localeCompare`.

## Les chaînes de caractères

Presque tous les programmes manipulent du texte : un prénom, un message, une adresse
e-mail, une ligne lue dans un fichier. Avant de le transformer, il faut savoir l'écrire.

Un texte s'appelle une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres)
(*string*) : une suite de lettres, de chiffres, d'espaces et de symboles, comme des
perles sur un fil. On l'écrit entre guillemets doubles, simples ou accents graves.

```javascript
const a = "Bonjour";
const b = 'Salut';
const c = `Coucou`;
const citation = "Il a dit : 'oui'";

console.log(a, b, c);
console.log(citation);
console.log("L'été");
```

```sortie
Bonjour Salut Coucou
Il a dit : 'oui'
L'été
```

Ligne par ligne :

- les trois premières lignes rangent un texte dans `a`, `b` et `c`, avec trois sortes
  de guillemets ;
- `citation` contient des guillemets simples : on l'entoure donc de guillemets doubles ;
- `console.log(a, b, c)` affiche les trois textes sur une ligne, séparés par une espace ;
- les guillemets qui entourent un texte ne sont jamais affichés : ils marquent seulement
  son début et sa fin.

Choisissez les guillemets qui ne sont pas dans le texte : `"L'été"` contient une
apostrophe, on l'entoure donc de guillemets doubles.

### Les caractères spéciaux

Une barre oblique inverse `\` introduit un caractère spécial : `\n` pour un retour à la
ligne, `\t` pour une tabulation, `\"` pour un guillemet dans un texte entre guillemets.

```javascript
console.log("Ligne 1\nLigne 2");
console.log("Elle dit \"bonjour\".");
```

```sortie
Ligne 1
Ligne 2
Elle dit "bonjour".
```

## Longueur et position

Vérifier qu'un mot de passe est assez long, prendre la première lettre d'un prénom :
pour cela, il faut connaître la taille d'un texte et savoir désigner une lettre précise,
comme on désigne une maison par son numéro dans une rue.

`length` donne le nombre de caractères. Chaque caractère a une position, son
[indice](../debuter/glossaire.md#indice), qui **commence à 0**.

```javascript
const mot = "Bonjour";

console.log(mot.length);
console.log(mot[0]);
console.log(mot[3]);
console.log(mot[mot.length - 1]);
console.log(mot[10]);
```

```sortie
7
B
j
r
undefined
```

Ligne par ligne :

- `mot.length` vaut 7 : « Bonjour » a sept lettres ;
- `mot[0]` est la première lettre, `"B"`, car on compte à partir de 0 ;
- `mot[3]` est donc la quatrième lettre, `"j"` ;
- `mot.length - 1` vaut 6 : `mot[6]` est la dernière lettre, `"r"` ;
- `mot[10]` désigne une case qui n'existe pas : le résultat est `undefined`.

| Caractère | B | o | n | j | o | u | r |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Indice | 0 | 1 | 2 | 3 | 4 | 5 | 6 |

Le dernier caractère est à l'indice `length - 1`. Une position qui n'existe pas donne
`undefined`.

### Compter depuis la fin avec at

`at(i)` fait comme `[i]`, mais accepte un nombre négatif : `-1` est le dernier
caractère, `-2` l'avant-dernier.

```javascript
const mot = "Bonjour";
console.log(mot.at(0));
console.log(mot.at(-1));
console.log(mot.at(-2));
```

```sortie
B
r
u
```

**À vous :** écrivez un programme qui range `"Camille"` dans une variable `prenom`,
puis affiche sa longueur, sa première lettre et sa dernière lettre.

> **Solution**
>
> ```javascript
> const prenom = "Camille";
> console.log(prenom.length);
> console.log(prenom[0]);
> console.log(prenom.at(-1));
> ```
>
> ```sortie
> 7
> C
> e
> ```

## Les textes ne changent jamais

Cette règle est à connaître tôt, car elle cause beaucoup d'erreurs chez les débutants :
on croit avoir changé un texte, alors qu'on en a fabriqué un nouveau, puis oublié.

Une chaîne est [immuable](../debuter/glossaire.md#immuable) : on ne peut pas modifier
un de ses caractères. Toutes les méthodes de cette page **renvoient un nouveau texte**
et laissent l'original intact.

```javascript
const nom = "alice";
const majuscules = nom.toUpperCase();

console.log(nom);
console.log(majuscules);
```

```sortie
alice
ALICE
```

Pour garder le résultat, rangez-le dans une variable (avec `let`, vous pouvez réutiliser
la même : `texte = texte.trim()`).

## Majuscules et espaces

Ce qu'un utilisateur tape est rarement propre : des espaces en trop, des majuscules au
hasard. Ces méthodes remettent le texte en ordre avant de s'en servir, comme on range un
tiroir avant d'y chercher quelque chose.

```javascript
const saisie = "   Marie Curie   ";

console.log(`[${saisie.toUpperCase()}]`);
console.log(`[${saisie.toLowerCase()}]`);
console.log(`[${saisie.trim()}]`);
console.log(`[${saisie.trimStart()}]`);
console.log(`[${saisie.trimEnd()}]`);
```

```sortie
[   MARIE CURIE   ]
[   marie curie   ]
[Marie Curie]
[Marie Curie   ]
[   Marie Curie]
```

Ligne par ligne :

- `saisie` contient « Marie Curie » entouré de trois espaces de chaque côté ;
- les crochets `[` et `]` ne servent qu'à voir où commencent et finissent les espaces ;
- chaque ligne appelle une méthode sur `saisie` et affiche le nouveau texte obtenu ;
- `saisie` ne change jamais : chaque ligne repart du texte d'origine, avec ses espaces.

- `toUpperCase()` et `toLowerCase()` changent la casse, accents compris
  (`"été"` devient `"ÉTÉ"`).
- `trim()` enlève les espaces au début et à la fin ; `trimStart()` et `trimEnd()` d'un
  seul côté.

> **Astuce :** appelez `trim()` sur tout ce qu'un utilisateur tape : une espace oubliée
> en fin de mot de passe ou d'adresse e-mail est une source d'erreurs classique.

## Extraire un morceau

Souvent, seule une partie d'un texte vous intéresse : l'année dans une date, les
premières lettres d'un mot. `slice` découpe ce morceau, comme une paire de ciseaux.

`slice(début, fin)` renvoie le morceau qui va de `début` (compris) à `fin` (non
compris). Sans `fin`, il va jusqu'au bout. Les nombres négatifs comptent depuis la fin.

```javascript
const mot = "Bonjour";

console.log(mot.slice(0, 3));
console.log(mot.slice(3));
console.log(mot.slice(-4));
console.log(mot.slice(1, -1));
```

```sortie
Bon
jour
jour
onjou
```

Ligne par ligne :

- `slice(0, 3)` prend les indices 0, 1 et 2 : `"Bon"` ;
- `slice(3)` part de l'indice 3 et va jusqu'au bout : `"jour"` ;
- `slice(-4)` prend les quatre derniers caractères : encore `"jour"` ;
- `slice(1, -1)` enlève la première et la dernière lettre : `"onjou"`.

Pensez aux indices comme à des **coupures entre les lettres** : `slice(0, 3)` coupe
avant la lettre 0 et avant la lettre 3.

`substring(début, fin)` fait presque pareil, mais ne comprend pas les nombres négatifs.
Retenez `slice`, qui marche aussi sur les tableaux.

**À vous :** écrivez un programme qui range `"2026-10-19"` dans une variable `date`,
puis affiche l'année (les quatre premiers caractères) et le jour (les deux derniers).

> **Solution**
>
> ```javascript
> const date = "2026-10-19";
> console.log(date.slice(0, 4));
> console.log(date.slice(-2));
> ```
>
> ```sortie
> 2026
> 19
> ```

## Découper et recoller

Un texte contient souvent une liste cachée : des fruits séparés par des virgules, des
mots séparés par des espaces. `split` sort cette liste du texte, pour traiter les
morceaux un par un.

`split(séparateur)` découpe un texte en un tableau de morceaux. `join(séparateur)` fait
l'inverse : il recolle les morceaux d'un tableau.

```javascript
const ligne = "pomme,poire,kiwi";
const fruits = ligne.split(",");
console.log(fruits);
console.log(fruits.length);

console.log(fruits.join(" - "));
console.log("Bonjour à tous".split(" "));
console.log("abc".split(""));
```

```sortie
[ 'pomme', 'poire', 'kiwi' ]
3
pomme - poire - kiwi
[ 'Bonjour', 'à', 'tous' ]
[ 'a', 'b', 'c' ]
```

Ligne par ligne :

- `ligne.split(",")` coupe le texte à chaque virgule : `fruits` est un tableau de trois
  textes ;
- `fruits.length` vaut 3 : le tableau a trois cases ;
- `fruits.join(" - ")` recolle les trois morceaux en mettant `" - "` entre chacun ;
- `split(" ")` coupe à chaque espace : on obtient les mots de la phrase.

`split("")` découpe en caractères. Les tableaux sont expliqués dans
[la page suivante](collections.md).

**À vous :** écrivez un programme qui compte les mots de la phrase
`"Le chat dort sur le canapé"` et affiche ce nombre.

> **Solution**
>
> ```javascript
> const phrase = "Le chat dort sur le canapé";
> const mots = phrase.split(" ");
> console.log(mots.length);
> ```
>
> ```sortie
> 6
> ```

## Chercher dans un texte

Un programme a souvent besoin de vérifier un texte : une adresse contient-elle un `@` ?
Un nom de fichier finit-il par `.jpg` ? Ces méthodes répondent par `true` (vrai) ou
`false` (faux), ou donnent une position.

```javascript
const phrase = "Le chat dort sur le canapé";

console.log(phrase.includes("chat"));
console.log(phrase.includes("Chat"));
console.log(phrase.startsWith("Le"));
console.log(phrase.endsWith("canapé"));
console.log(phrase.indexOf("dort"));
console.log(phrase.indexOf("chien"));
```

```sortie
true
false
true
true
8
-1
```

Ligne par ligne :

- `includes("chat")` donne `true`, mais `includes("Chat")` donne `false` : la majuscule
  compte ;
- `startsWith("Le")` et `endsWith("canapé")` donnent `true` : la phrase commence et
  finit bien ainsi ;
- `indexOf("dort")` donne 8 : « dort » commence à l'indice 8, espaces comprises ;
- `indexOf("chien")` donne `-1` : « chien » n'est pas dans la phrase.

- `includes(x)` : le texte contient-il `x` ?
- `startsWith(x)` et `endsWith(x)` : commence-t-il ou finit-il par `x` ?
- `indexOf(x)` : la position de la première apparition de `x`, ou `-1` s'il n'y est pas.

Toutes ces recherches font la différence entre majuscules et minuscules. Pour l'ignorer,
mettez d'abord les deux en minuscules :

```javascript
const phrase = "Le Chat dort";
console.log(phrase.toLowerCase().includes("chat"));
```

```sortie
true
```

## Remplacer

Corriger un mot partout dans un texte, enlever les espaces d'un numéro de téléphone :
c'est le « rechercher et remplacer » d'un traitement de texte.

`replace(ancien, nouveau)` remplace la **première** apparition. `replaceAll` les
remplace **toutes**.

```javascript
const phrase = "le chat et le chien";

console.log(phrase.replace("le", "un"));
console.log(phrase.replaceAll("le", "un"));
console.log("06 12 34 56 78".replaceAll(" ", ""));
```

```sortie
un chat et le chien
un chat et un chien
0612345678
```

Ligne par ligne :

- `replace("le", "un")` ne change que le premier « le » ;
- `replaceAll("le", "un")` change les deux ;
- la dernière ligne remplace chaque espace par un texte vide `""` : les espaces
  disparaissent.

Pour des recherches plus fines (« tous les chiffres », « un mot en majuscule »), il
existe les [expressions régulières](regex.md).

**À vous :** écrivez un programme qui range `"19/10/2026"` dans une variable, puis
l'affiche avec des tirets à la place des barres obliques.

> **Solution**
>
> ```javascript
> const date = "19/10/2026";
> console.log(date.replaceAll("/", "-"));
> ```
>
> ```sortie
> 19-10-2026
> ```

## Aligner et répéter

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`padStart(longueur, remplissage)` complète un texte au début jusqu'à la longueur
voulue ; `padEnd` complète à la fin. `repeat(n)` répète un texte `n` fois.

```javascript
console.log("7".padStart(3, "0"));
console.log(String(42).padStart(5, " "));

const lignes = [["Café", 2.5], ["Croissant", 1.2]];
for (const [article, prix] of lignes) {
  console.log(article.padEnd(12, ".") + prix.toFixed(2).padStart(6));
}
console.log("=".repeat(18));
```

```sortie
007
   42
Café........  2.50
Croissant...  1.20
==================
```

`padStart(6)` sans second argument complète avec des espaces. Sur un nombre, il faut
d'abord le transformer en texte : `String(42)`.
Ces méthodes sont parfaites pour dessiner des tableaux alignés dans le terminal.

## Les gabarits sur plusieurs lignes

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un gabarit entre accents graves peut s'étendre sur plusieurs lignes : les retours à la
ligne font partie du texte.

```javascript
const nom = "Alice";
const total = 42.5;

const ticket = `Client : ${nom}
Total  : ${total.toFixed(2)} €
Merci de votre visite !`;

console.log(ticket);
```

```sortie
Client : Alice
Total  : 42.50 €
Merci de votre visite !
```

> **Attention :** les espaces d'indentation font aussi partie du texte. Si vous décalez
> les lignes 2 et 3 dans votre code, elles seront décalées à l'affichage.

## Trier des mots français

Ranger des noms dans l'ordre alphabétique, comme dans un annuaire, semble simple. Mais
l'ordinateur ne connaît pas l'alphabet français : il range les lettres selon des numéros.

Comparer deux textes avec `<` ou trier avec `sort()` suit l'ordre des **codes des
caractères** (le standard [Unicode](../debuter/glossaire.md#unicode)). Les majuscules
passent avant les minuscules, et les lettres accentuées à la fin.

```javascript
const mots = ["zèbre", "Éléphant", "abricot", "étoile", "Banane"];

console.log([...mots].sort());
console.log([...mots].sort((a, b) => a.localeCompare(b, "fr")));
```

```sortie
[ 'Banane', 'abricot', 'zèbre', 'Éléphant', 'étoile' ]
[ 'abricot', 'Banane', 'Éléphant', 'étoile', 'zèbre' ]
```

Ligne par ligne :

- `mots` contient cinq mots, avec des majuscules et des accents ;
- `[...mots].sort()` trie une copie selon les codes des caractères : `"Banane"` passe
  avant `"abricot"`, et les mots qui commencent par `É` ou `é` finissent au bout ;
- la ligne suivante trie une autre copie avec `localeCompare` : l'ordre est celui du
  dictionnaire.

`a.localeCompare(b, "fr")` compare deux textes selon les règles du français : il
renvoie un nombre négatif si `a` vient avant `b`, positif si après, `0` si égaux.
`sort` s'en sert pour ranger le tableau. `[...mots]` crée une copie, pour ne pas
modifier l'original (voir [les tableaux](collections.md)).

> **À retenir :** pour trier des mots qu'un humain va lire, utilisez toujours
> `localeCompare` avec `"fr"`.

## Erreurs fréquentes

### Modifier un caractère

```javascript norun
const mot = "chat";
mot[0] = "C";
```

```text
file:///home/alice/js/main.mjs:2
mot[0] = "C";
       ^

TypeError: Cannot assign to read only property '0' of string 'chat'
    at file:///home/alice/js/main.mjs:2:8

Node.js v22.22.0
```

Un texte est immuable : ses caractères sont en lecture seule (*read only*). Fabriquez
un nouveau texte : `const nouveau = "C" + mot.slice(1);`.

### Écrire length avec des parenthèses

```javascript norun
const nom = "Alice";
console.log(nom.length());
```

```text
file:///home/alice/js/main.mjs:2
console.log(nom.length());
                ^

TypeError: nom.length is not a function
    at file:///home/alice/js/main.mjs:2:17

Node.js v22.22.0
```

`length` est une **propriété** (une valeur), pas une méthode : on ne l'appelle pas.
Écrivez `nom.length`.

### Se tromper de majuscule dans un nom de méthode

```javascript norun
const nom = "alice";
console.log(nom.toUppercase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(nom.toUppercase());
                ^

TypeError: nom.toUppercase is not a function
    at file:///home/alice/js/main.mjs:2:17

Node.js v22.22.0
```

La méthode s'appelle `toUpperCase`, avec un `C` majuscule. Les noms de JavaScript sont
en camelCase : chaque nouveau mot prend une majuscule.

### Oublier de garder le résultat

```javascript norun
let saisie = "  bonjour  ";
saisie.trim();
console.log(`[${saisie}]`);
```

```text
[  bonjour  ]
```

`trim()` ne modifie pas `saisie` : il renvoie un nouveau texte, perdu ici. Écrivez
`saisie = saisie.trim();`.

## Testez-vous

**Question 1 :** que donne `"JavaScript".slice(0, 4)` ?

> **Réponse**
>
> ```javascript
> console.log("JavaScript".slice(0, 4));
> ```
>
> ```sortie
> Java
> ```
>
> Les indices 0, 1, 2 et 3 : la fin (4) n'est pas comprise.

**Question 2 :** comment transformer `"a;b;c"` en `"a, b, c"` ?

> **Réponse**
>
> ```javascript
> console.log("a;b;c".split(";").join(", "));
> ```
>
> ```sortie
> a, b, c
> ```
>
> `replaceAll(";", ", ")` marche aussi.

**Question 3 :** pourquoi trier avec `localeCompare(b, "fr")` plutôt qu'avec `sort()`
seul ?

> **Réponse**
>
> `sort()` seul range selon les codes des caractères : les majuscules avant les
> minuscules et `"é"` après `"z"`. `localeCompare` suit l'ordre alphabétique français.

## En résumé

- Un texte s'écrit entre `"..."`, `'...'` ou `` `...` `` ; il est immuable, chaque
  méthode renvoie un nouveau texte.
- `length` donne la longueur ; `texte[i]` et `texte.at(i)` un caractère, à partir de 0.
- `slice`, `trim`, `toUpperCase` et `toLowerCase` extraient et nettoient.
- `split` découpe en tableau, `join` recolle ; `includes`, `startsWith` et `indexOf`
  cherchent ; `replace` et `replaceAll` remplacent.
- `padStart`, `padEnd` et `repeat` alignent ; un gabarit peut faire plusieurs lignes.
- Triez des mots français avec `localeCompare(b, "fr")`.

Suite : [les tableaux](collections.md).
