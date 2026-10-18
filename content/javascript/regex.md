# Les expressions régulières

Cette page montre comment décrire un motif de texte avec une expression régulière, pour chercher, extraire, remplacer ou valider du texte.

> **Objectifs :**
>
> - vous saurez écrire une expression régulière `/…/` et la tester avec `test` ;
> - vous saurez extraire des morceaux avec `match`, `matchAll` et les groupes ;
> - vous saurez remplacer avec `replace`, par `$1` ou par une fonction ;
> - vous saurez utiliser les drapeaux `g`, `i`, `m` et `u` ;
> - vous saurez valider des formats simples.

## Qu'est-ce qu'une expression régulière ?

Une [expression régulière](../debuter/glossaire.md#expression-reguliere) (ou **regex**)
décrit un **motif** de texte : « trois chiffres », « un mot qui commence par une
majuscule », « quelque chose qui ressemble à un e-mail ». C'est un avis de recherche :
on ne donne pas le nom exact, on décrit à quoi ressemble ce qu'on cherche.

En JavaScript, une regex s'écrit entre deux barres obliques : `/chat/`. Après la
dernière barre viennent d'éventuels **drapeaux** (*flags*) : `/chat/gi`.

```javascript
const motif = /chat/;
console.log(motif.test("Le chat dort."));
console.log(motif.test("Le chien dort."));
```

```sortie
true
false
```

`test` répond `true` si le motif apparaît **quelque part** dans le texte.

> **Note :** on peut aussi construire une regex à partir d'un texte avec
> `new RegExp("chat", "gi")`. C'est utile quand le motif vient d'une variable.

## Les briques de base

Une regex mélange des caractères normaux (`c`, `h`, `a`, `t`) et des caractères
spéciaux qui décrivent des familles.

```javascript
console.log(/\d/.test("Il a 3 chats"));
console.log(/^Bonjour/.test("Bonjour Alice"));
console.log(/^Bonjour/.test("Alice, Bonjour"));
console.log(/a.c/.test("abc"));
console.log(/colou?r/.test("color"));
console.log(/^\d{5}$/.test("75001"));
console.log(/^\d{5}$/.test("750012"));
```

```sortie
true
true
false
true
true
true
false
```

- `\d` : un chiffre ;
- `^` : le début du texte, `$` : la fin ;
- `.` : n'importe quel caractère ;
- `u?` : un `u` facultatif ;
- `\d{5}` : exactement cinq chiffres.

Le tableau complet est plus bas, dans l'aide-mémoire.

## Trouver avec match

`texte.match(regex)` renvoie ce qui correspond, ou `null` si rien ne correspond.

Sans le drapeau `g`, vous obtenez la **première** correspondance, avec des détails :

```javascript
const texte = "Commande 42, puis commande 17.";
const resultat = texte.match(/\d+/);
console.log(resultat[0]);
console.log(resultat.index);
```

```sortie
42
9
```

`resultat[0]` est le texte trouvé, `index` sa position. Avec le drapeau `g` (*global*),
`match` renvoie un simple tableau de **toutes** les correspondances :

```javascript
const texte = "Commande 42, puis commande 17.";
console.log(texte.match(/\d+/g));
console.log("Pas de chiffre".match(/\d+/g));
```

```sortie
[ '42', '17' ]
null
```

> **Attention :** quand rien ne correspond, `match` renvoie `null`, pas un tableau vide.
> Écrivez `texte.match(/\d+/g) ?? []` pour toujours avoir un tableau.

## Les groupes

Des parenthèses `( )` forment un **groupe** : un morceau du motif que l'on veut
récupérer à part. Les groupes sont numérotés de gauche à droite, à partir de 1.

```javascript
const date = "2024-03-05";
const m = date.match(/(\d{4})-(\d{2})-(\d{2})/);
console.log(m[0]);
console.log(m[1], m[2], m[3]);
```

```sortie
2024-03-05
2024 03 05
```

### Les groupes nommés

Un numéro, c'est vite confus. `(?<nom>…)` donne un nom au groupe. On le retrouve dans
`groups`.

```javascript
const m = "2024-03-05".match(/(?<annee>\d{4})-(?<mois>\d{2})-(?<jour>\d{2})/);
console.log(m.groups.jour, m.groups.mois, m.groups.annee);

const { annee, mois } = m.groups;
console.log(`${mois}/${annee}`);
```

```sortie
05 03 2024
03/2024
```

## Toutes les correspondances avec matchAll

`match` avec `g` perd les groupes. `matchAll` renvoie **chaque** correspondance avec ses
groupes. On le parcourt avec `for...of`. Le drapeau `g` est obligatoire.

```javascript
const texte = "Alice: 15, Bob: 12, Chloé: 18";

for (const m of texte.matchAll(/(?<nom>\p{L}+): (?<note>\d+)/gu)) {
  console.log(`${m.groups.nom} a ${m.groups.note}`);
}
```

```sortie
Alice a 15
Bob a 12
Chloé a 18
```

`\p{L}` veut dire « une lettre de n'importe quelle langue », accents compris. Il
demande le drapeau `u`.

## Remplacer avec replace

`texte.replace(regex, remplacement)` renvoie un nouveau texte. Sans `g`, seule la
première correspondance est remplacée ; avec `g`, toutes.

```javascript
const texte = "chat, chat, chat";
console.log(texte.replace(/chat/, "chien"));
console.log(texte.replace(/chat/g, "chien"));
```

```sortie
chien, chat, chat
chien, chien, chien
```

### Réutiliser un groupe avec $1

Dans le texte de remplacement, `$1`, `$2`… désignent les groupes, et `$<nom>` un groupe
nommé. Ici, on passe d'une date anglaise à une date française :

```javascript
const iso = "2024-03-05";
console.log(iso.replace(/(\d{4})-(\d{2})-(\d{2})/, "$3/$2/$1"));
console.log(iso.replace(/(?<a>\d{4})-(?<m>\d{2})-(?<j>\d{2})/, "$<j>.$<m>.$<a>"));
```

```sortie
05/03/2024
05.03.2024
```

### Remplacer avec une fonction

Quand le remplacement demande un calcul, donnez une fonction. Elle reçoit le texte
trouvé, puis les groupes, et renvoie le texte à mettre à la place.

```javascript
const recette = "2 œufs, 100 g de farine, 3 cuillères";
const double = recette.replace(/\d+/g, (nombre) => String(Number(nombre) * 2));
console.log(double);

const titre = "le petit prince";
console.log(titre.replace(/\b\w/g, (lettre) => lettre.toUpperCase()));
```

```sortie
4 œufs, 200 g de farine, 6 cuillères
Le Petit Prince
```

`\b` est une **frontière de mot** : l'endroit où un mot commence ou finit.

## Découper avec split

`split` accepte aussi une regex. Pratique quand les séparateurs varient.

```javascript
const liste = "pomme, poire;banane  ;  kiwi";
console.log(liste.split(/\s*[,;]\s*/));

const phrase = "Un   texte  avec   trop d'espaces";
console.log(phrase.split(/\s+/));
```

```sortie
[ 'pomme', 'poire', 'banane', 'kiwi' ]
[ 'Un', 'texte', 'avec', 'trop', "d'espaces" ]
```

- `[,;]` : une virgule **ou** un point-virgule (une classe de caractères) ;
- `\s*` : zéro ou plusieurs espaces ;
- `\s+` : au moins un espace.

## Les drapeaux

| Drapeau | Nom | Effet |
|---------|-----|-------|
| `g` | *global* | toutes les correspondances, pas seulement la première |
| `i` | *ignore case* | ignore majuscules et minuscules |
| `m` | *multiline* | `^` et `$` marchent au début et à la fin de **chaque ligne** |
| `u` | *unicode* | lit correctement les accents et les émojis, active `\p{…}` |

```javascript
console.log(/bonjour/i.test("BONJOUR"));

const texte = "pain\nlait\npâtes";
console.log(texte.match(/^p.*/g));
console.log(texte.match(/^p.*/gm));
```

```sortie
true
[ 'pain' ]
[ 'pain', 'pâtes' ]
```

Sans `m`, `^` ne marque que le tout début du texte. Avec `m`, chaque début de ligne
compte.

## Aide-mémoire

| Motif | Signifie | Exemple qui correspond |
|-------|----------|------------------------|
| `abc` | le texte `abc` | `abc` |
| `.` | n'importe quel caractère (sauf retour à la ligne) | `a`, `7`, `!` |
| `\d` / `\D` | un chiffre / tout sauf un chiffre | `5` / `x` |
| `\w` / `\W` | lettre (sans accent), chiffre ou `_` / l'inverse | `a`, `_` |
| `\s` / `\S` | un espace (espace, tabulation, retour) / l'inverse | ` ` |
| `\p{L}` | une lettre de n'importe quelle langue (avec `u`) | `é`, `ß` |
| `[aeiou]` | un caractère de la liste | `e` |
| `[^0-9]` | un caractère **hors** de la liste | `a` |
| `[a-z]` | un caractère de l'intervalle | `m` |
| `x?` | zéro ou un `x` | (rien), `x` |
| `x*` | zéro, un ou plusieurs `x` | (rien), `xxx` |
| `x+` | au moins un `x` | `x`, `xx` |
| `x{3}` / `x{2,4}` | exactement 3 / entre 2 et 4 | `xxx` |
| `^` / `$` | début / fin du texte (ou de la ligne avec `m`) | |
| `\b` | frontière de mot | |
| `a\|b` | `a` ou `b` | `a` |
| `( )` | un groupe numéroté | |
| `(?<nom> )` | un groupe nommé | |
| `\.` `\(` `\?` | le caractère lui-même (échappé) | `.` |

> **Astuce :** pour tester une regex pas à pas, un site comme regex101 (choisissez
> « ECMAScript ») colore chaque morceau et explique le motif.

## Valider des formats simples

Pour valider tout un texte, encadrez le motif par `^` et `$`. Sinon, il suffit qu'un
morceau corresponde.

```javascript
const formats = {
  codePostal: /^\d{5}$/,
  telephone: /^0\d( ?\d{2}){4}$/,
  heure: /^([01]\d|2[0-3]):[0-5]\d$/,
  email: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
};

const essais = [
  ["codePostal", "69001"],
  ["codePostal", "6900"],
  ["telephone", "06 12 34 56 78"],
  ["telephone", "0612345678"],
  ["heure", "23:59"],
  ["heure", "24:00"],
  ["email", "alice@exemple.fr"],
  ["email", "alice@exemple"],
];

for (const [format, valeur] of essais) {
  const ok = formats[format].test(valeur);
  console.log(`${format.padEnd(10)} ${valeur.padEnd(16)} ${ok ? "valide" : "invalide"}`);
}
```

```sortie
codePostal 69001            valide
codePostal 6900             invalide
telephone  06 12 34 56 78   valide
telephone  0612345678       valide
heure      23:59            valide
heure      24:00            invalide
email      alice@exemple.fr valide
email      alice@exemple    invalide
```

> **À retenir :** une regex d'e-mail ne vérifie que la **forme**. Seul un e-mail de
> confirmation prouve que l'adresse existe. Ne cherchez pas la regex parfaite : restez
> simple et lisible.

## Erreurs fréquentes

### matchAll sans le drapeau g

```javascript norun
const notes = "a1b2".matchAll(/\d/);
```

```text
TypeError: String.prototype.matchAll called with a non-global RegExp argument
```

`matchAll` exige le drapeau `g`. Écrivez `/\d/g`. La même erreur arrive avec
`replaceAll` et une regex sans `g`.

### Oublier g dans replace

```javascript norun
console.log("a-b-c".replace(/-/, "+"));
```

```text
a+b-c
```

Pas de message d'erreur, mais un seul tiret est remplacé. Ajoutez `g` : `/-/g`.

### Oublier d'échapper un caractère spécial

```javascript norun
console.log(/3.14/.test("3514"));
```

```text
true
```

Le `.` veut dire « n'importe quel caractère » : `3514` correspond. Pour un vrai point,
échappez-le : `/3\.14/`. De même pour `(`, `)`, `?`, `+`, `*`, `[`, `$`, `^` et `/`.

### Une parenthèse non fermée

```javascript norun
const motif = new RegExp("(abc");
```

```text
SyntaxError: Invalid regular expression: /(abc/: Unterminated group
```

Chaque `(` doit avoir sa `)`. Pour chercher une parenthèse, échappez-la : `\(`. Avec
`new RegExp`, le texte demande deux barres : `new RegExp("\\(abc")`.

## Testez-vous

**Question 1 :** que renvoie `"abc".match(/\d+/g)` ?

> **Réponse**
>
> `null` : il n'y a aucun chiffre. Ce n'est pas un tableau vide.

**Question 2 :** que fait le drapeau `i` ?

> **Réponse**
>
> Il ignore la casse : `/chat/i` trouve `chat`, `Chat` et `CHAT`.

**Question 3 :** écrivez un code qui transforme `"Dupont Jean"` en `"Jean DUPONT"` avec
`replace` et deux groupes.

> **Réponse**
>
> ```javascript
> const nom = "Dupont Jean";
> const resultat = nom.replace(/(\w+) (\w+)/, (_, famille, prenom) => `${prenom} ${famille.toUpperCase()}`);
> console.log(resultat);
> ```
>
> ```sortie
> Jean DUPONT
> ```

## En résumé

- Une regex s'écrit `/motif/drapeaux` ; `test` répond `true` ou `false`.
- `match` renvoie la première correspondance (ou toutes avec `g`), et `null` si rien.
- Les groupes `( )` et `(?<nom> )` récupèrent des morceaux ; `matchAll` les donne tous.
- `replace` remplace par un texte (`$1`, `$<nom>`) ou par le résultat d'une fonction.
- `g` = toutes, `i` = sans casse, `m` = par ligne, `u` = Unicode.
- Pour valider, encadrez le motif par `^` et `$`, et restez simple.

Suite : [dates et heures](dates.md)
