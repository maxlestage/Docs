# Exercices corrigés : deuxième série

Cette page propose 12 nouveaux exercices corrigés en JavaScript, du plus facile au plus difficile, avec des tableaux de trace pour suivre les solutions pas à pas.

## Comment utiliser cette page

Ces exercices sont **nouveaux** : ils ne reprennent pas ceux des
[exercices pour bien démarrer](exercices-faciles.md) ni ceux de la
[première série](exercices.md). Chaque exercice a un niveau :

- ★☆☆ : facile, une seule notion ;
- ★★☆ : moyen, deux ou trois notions ensemble ;
- ★★★ : plus dur, il faut d'abord réfléchir sur papier.

Pour chaque exercice :

1. lisez l'énoncé et la sortie attendue ;
2. faites un petit plan en français, sur papier ;
3. écrivez le code dans un fichier `exercice.mjs` et lancez `node exercice.mjs` ;
4. bloqué ? lisez l'indice, puis réessayez ;
5. ouvrez la solution en dernier, et suivez son tableau de trace avec le doigt.

```text
 énoncé → plan sur papier → code → lancer → comparer
                              ▲                │
                              └── différent ? ─┘
```

> **Astuce :** pour remplir un tableau de trace, relisez
> [suivre un programme pas à pas](pas-a-pas.md).

## Premier niveau

### Exercice 1 : Le ticket de parking

Niveau : ★☆☆

Un parking coûte 2 € par heure, mais jamais plus de 10 €. Écrivez une fonction
`prixParking(heures)` qui renvoie le prix. Affichez le prix pour 1, 3 et 8 heures.

Sortie attendue :

```text
1 h : 2 €
3 h : 6 €
8 h : 10 €
```

> **Indice :** calculez d'abord `heures * 2`. Puis, avec un `if`, renvoyez `10` si ce
> prix dépasse 10 (voir [les fonctions](fonctions.md)).

> **Solution**
>
> ```javascript
> function prixParking(heures) {
>   const prix = heures * 2;
>   if (prix > 10) {
>     return 10;
>   }
>   return prix;
> }
>
> for (const h of [1, 3, 8]) {
>   console.log(`${h} h : ${prixParking(h)} €`);
> }
> ```
>
> ```sortie
> 1 h : 2 €
> 3 h : 6 €
> 8 h : 10 €
> ```
>
> Pourquoi ça marche : `return` arrête la fonction tout de suite. Si le prix dépasse
> 10, on renvoie 10 et la dernière ligne n'est jamais lue. `Math.min(heures * 2, 10)`
> fait la même chose en une ligne.

### Exercice 2 : Les jours de gel

Niveau : ★☆☆

Voici les températures d'une semaine : `[3, -2, 5, -7, 0, 4]`. Affichez chaque jour de
gel (température sous 0), avec son numéro (le premier jour est le jour 1), puis le
nombre de jours de gel.

Sortie attendue :

```text
Gel le jour 2 : -2 °C
Gel le jour 4 : -7 °C
Jours de gel : 2
```

> **Indice :** il faut le numéro du jour : utilisez une boucle avec un indice,
> `for (let i = 0; i < temperatures.length; i++)`. Le numéro du jour est `i + 1`.

> **Solution**
>
> ```javascript
> const temperatures = [3, -2, 5, -7, 0, 4];
> let joursDeGel = 0;
> for (let i = 0; i < temperatures.length; i++) {
>   if (temperatures[i] < 0) {
>     console.log(`Gel le jour ${i + 1} : ${temperatures[i]} °C`);
>     joursDeGel = joursDeGel + 1;
>   }
> }
> console.log(`Jours de gel : ${joursDeGel}`);
> ```
>
> ```sortie
> Gel le jour 2 : -2 °C
> Gel le jour 4 : -7 °C
> Jours de gel : 2
> ```
>
> Pourquoi ça marche : les indices commencent à 0, les jours à 1, d'où `i + 1`. Le
> jour 5 vaut `0` : `0 < 0` est faux, ce n'est pas du gel.

### Exercice 3 : Le mot le plus long

Niveau : ★☆☆

Dans le tableau `["chat", "éléphant", "souris", "ours"]`, trouvez le mot le plus long
et affichez-le avec son nombre de lettres.

Sortie attendue :

```text
Le mot le plus long : éléphant (8 lettres)
```

> **Indice :** gardez le meilleur mot trouvé dans une boîte `plusLong`, qui commence
> avec le premier mot. À chaque tour, comparez les `.length` (voir [le texte](texte.md)).

> **Solution**
>
> ```javascript
> const mots = ["chat", "éléphant", "souris", "ours"];
> let plusLong = mots[0];
> for (const mot of mots) {
>   if (mot.length > plusLong.length) {
>     plusLong = mot;
>   }
> }
> console.log(`Le mot le plus long : ${plusLong} (${plusLong.length} lettres)`);
> ```
>
> ```sortie
> Le mot le plus long : éléphant (8 lettres)
> ```
>
> Pourquoi ça marche : c'est le principe du champion. On garde le meilleur, et chaque
> nouveau mot doit le battre pour prendre sa place.
>
> | Tour | mot | longueur | plus long que plusLong ? | plusLong |
> | --- | --- | --- | --- | --- |
> | départ | – | – | – | chat |
> | 1 | chat | 4 | 4 > 4 → non | chat |
> | 2 | éléphant | 8 | 8 > 4 → oui | éléphant |
> | 3 | souris | 6 | 6 > 8 → non | éléphant |
> | 4 | ours | 4 | 4 > 8 → non | éléphant |

### Exercice 4 : Le triangle d'étoiles

Niveau : ★☆☆

Affichez un triangle de 4 lignes : une étoile sur la première ligne, deux sur la
deuxième, et ainsi de suite.

Sortie attendue :

```text
*
**
***
****
```

> **Indice :** `"*".repeat(3)` donne `"***"`. Faites une boucle de 1 à 4 et répétez
> l'étoile autant de fois que le numéro de la ligne.

> **Solution**
>
> ```javascript
> for (let ligne = 1; ligne <= 4; ligne++) {
>   console.log("*".repeat(ligne));
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Pourquoi ça marche : la variable de la boucle sert deux fois, pour compter les
> lignes et pour dire combien d'étoiles mettre.
>
> | Tour | ligne | ligne <= 4 ? | affiché |
> | --- | --- | --- | --- |
> | 1 | 1 | oui | `*` |
> | 2 | 2 | oui | `**` |
> | 3 | 3 | oui | `***` |
> | 4 | 4 | oui | `****` |
> | fin | 5 | non | (la boucle s'arrête) |

## Deuxième niveau

### Exercice 5 : Rendre la monnaie

Niveau : ★★☆

Une caisse doit rendre 470 centimes avec le moins de pièces possible. Les pièces
valent 200, 100, 50, 20 et 10 centimes. Écrivez `rendreMonnaie(montant)` qui affiche
combien de pièces de chaque sorte il faut (seulement celles utilisées).

Sortie attendue :

```text
2 × 200 c
1 × 50 c
1 × 20 c
```

> **Indice :** commencez par la plus grosse pièce. Tant que le reste est assez grand,
> donnez cette pièce et retirez sa valeur du reste. Puis passez à la pièce suivante.

> **Indice :** il faut deux boucles : un `for…of` sur les pièces, et dedans un
> `while (reste >= piece)`. Travaillez en centimes pour éviter les virgules
> (voir [les nombres](nombres.md)).

> **Solution**
>
> ```javascript
> function rendreMonnaie(montant) {
>   const pieces = [200, 100, 50, 20, 10];
>   let reste = montant;
>   for (const piece of pieces) {
>     let nombre = 0;
>     while (reste >= piece) {
>       reste = reste - piece;
>       nombre = nombre + 1;
>     }
>     if (nombre > 0) {
>       console.log(`${nombre} × ${piece} c`);
>     }
>   }
> }
>
> rendreMonnaie(470);
> ```
>
> ```sortie
> 2 × 200 c
> 1 × 50 c
> 1 × 20 c
> ```
>
> Pourquoi ça marche : on vide le reste en commençant par les grosses pièces, comme un
> caissier qui pioche d'abord dans la case des 2 €.
>
> | Étape | piece | reste >= piece ? | reste | nombre |
> | --- | --- | --- | --- | --- |
> | départ | – | – | 470 | – |
> | 1 | 200 | 470 >= 200 → oui | 270 | 1 |
> | 2 | 200 | 270 >= 200 → oui | 70 | 2 |
> | 3 | 200 | 70 >= 200 → non | 70 | 2 (affiché) |
> | 4 | 100 | 70 >= 100 → non | 70 | 0 |
> | 5 | 50 | 70 >= 50 → oui | 20 | 1 |
> | 6 | 50 | 20 >= 50 → non | 20 | 1 (affiché) |
> | 7 | 20 | 20 >= 20 → oui | 0 | 1 |
> | 8 | 20 | 0 >= 20 → non | 0 | 1 (affiché) |
> | 9 | 10 | 0 >= 10 → non | 0 | 0 |

### Exercice 6 : Retirer les doublons

Niveau : ★★☆

Écrivez `sansDoublons(valeurs)` qui renvoie un **nouveau** tableau sans les valeurs
répétées, dans l'ordre de leur première apparition. Testez avec `[3, 1, 3, 2, 1]`.

Sortie attendue :

```text
[ 3, 1, 2 ]
[ 'a', 'b' ]
```

> **Indice :** créez un tableau vide `resultat`. Pour chaque valeur, ajoutez-la
> seulement si `resultat.includes(v)` est faux (voir [les collections](collections.md)).

> **Solution**
>
> ```javascript
> function sansDoublons(valeurs) {
>   const resultat = [];
>   for (const v of valeurs) {
>     if (!resultat.includes(v)) {
>       resultat.push(v);
>     }
>   }
>   return resultat;
> }
>
> console.log(sansDoublons([3, 1, 3, 2, 1]));
> console.log(sansDoublons(["a", "b", "a"]));
> ```
>
> ```sortie
> [ 3, 1, 2 ]
> [ 'a', 'b' ]
> ```
>
> Pourquoi ça marche : `!` inverse la réponse de `includes`. On ajoute donc une valeur
> seulement quand elle n'est pas encore dans `resultat`.
>
> | Tour | v | déjà dans resultat ? | resultat |
> | --- | --- | --- | --- |
> | départ | – | – | `[]` |
> | 1 | 3 | non | `[3]` |
> | 2 | 1 | non | `[3, 1]` |
> | 3 | 3 | oui | `[3, 1]` |
> | 4 | 2 | non | `[3, 1, 2]` |
> | 5 | 1 | oui | `[3, 1, 2]` |
>
> Plus tard, vous écrirez simplement `[...new Set(valeurs)]` : un `Set` ne garde
> jamais deux fois la même valeur (voir [les objets](objets.md)).

### Exercice 7 : Ranger par initiale

Niveau : ★★☆

Rangez les prénoms `["Alice", "Bob", "Anne", "Chloé", "Bruno"]` par première lettre,
dans un objet. Affichez ensuite une ligne par lettre.

Sortie attendue :

```text
A : Alice, Anne
B : Bob, Bruno
C : Chloé
```

> **Indice :** l'objet `groupes` a une clé par lettre, et pour valeur un tableau de
> prénoms. `prenom[0]` donne la première lettre.

> **Indice :** si `groupes[initiale]` vaut `undefined`, créez d'abord le tiroir vide
> `groupes[initiale] = []`, puis faites `push`. Pour l'affichage, utilisez
> `Object.entries` (voir [les objets](objets.md)).

> **Solution**
>
> ```javascript
> const prenoms = ["Alice", "Bob", "Anne", "Chloé", "Bruno"];
> const groupes = {};
> for (const prenom of prenoms) {
>   const initiale = prenom[0];
>   if (groupes[initiale] === undefined) {
>     groupes[initiale] = [];
>   }
>   groupes[initiale].push(prenom);
> }
>
> for (const [lettre, liste] of Object.entries(groupes)) {
>   console.log(`${lettre} : ${liste.join(", ")}`);
> }
> ```
>
> ```sortie
> A : Alice, Anne
> B : Bob, Bruno
> C : Chloé
> ```
>
> Pourquoi ça marche : c'est un meuble à tiroirs. On ouvre le tiroir de la lettre ; s'il
> n'existe pas, on le crée vide ; puis on y range le prénom.

### Exercice 8 : Vérifier un mot de passe

Niveau : ★★☆

Écrivez `problemesMotDePasse(mdp)` qui renvoie la liste des problèmes d'un mot de
passe : moins de 8 caractères, aucun chiffre, aucune majuscule. Testez avec `"chat"`,
`"soleil2024"` et `"Soleil2024"`.

Sortie attendue :

```text
chat : moins de 8 caractères, aucun chiffre, aucune majuscule
soleil2024 : aucune majuscule
Soleil2024 : correct
```

> **Indice :** trois `if` séparés (pas de `else`), car plusieurs problèmes peuvent
> exister en même temps. Chaque `if` ajoute un texte dans un tableau `problemes`.

> **Indice :** un texte sans majuscule ne change pas avec `toLowerCase()`. Pour les
> chiffres, cherchez si un caractère est dans `"0123456789"`.

> **Solution**
>
> ```javascript
> function problemesMotDePasse(mdp) {
>   const problemes = [];
>   if (mdp.length < 8) {
>     problemes.push("moins de 8 caractères");
>   }
>   if (![...mdp].some((c) => "0123456789".includes(c))) {
>     problemes.push("aucun chiffre");
>   }
>   if (mdp === mdp.toLowerCase()) {
>     problemes.push("aucune majuscule");
>   }
>   return problemes;
> }
>
> for (const mdp of ["chat", "soleil2024", "Soleil2024"]) {
>   const problemes = problemesMotDePasse(mdp);
>   if (problemes.length === 0) {
>     console.log(`${mdp} : correct`);
>   } else {
>     console.log(`${mdp} : ${problemes.join(", ")}`);
>   }
> }
> ```
>
> ```sortie
> chat : moins de 8 caractères, aucun chiffre, aucune majuscule
> soleil2024 : aucune majuscule
> Soleil2024 : correct
> ```
>
> Pourquoi ça marche : avec `else if`, le programme s'arrêterait au premier problème.
> Trois `if` indépendants testent tout, comme une liste de contrôle où l'on coche
> chaque case.

### Exercice 9 : Le code de César

Niveau : ★★☆

Jules César cachait ses messages en décalant chaque lettre de 3 dans l'alphabet : `a`
devient `d`, `z` devient `c`. Écrivez `cesar(texte, decalage)` pour des textes en
minuscules sans accents ; les autres caractères ne changent pas. Codez
`"bonjour zoe"`, puis décodez le résultat avec un décalage de 23.

Sortie attendue :

```text
erqmrxu crh
bonjour zoe
```

> **Indice :** `ALPHABET.indexOf(c)` donne la position d'une lettre (`-1` si ce n'est
> pas une lettre). La nouvelle position est `(position + decalage) % 26`.

> **Solution**
>
> ```javascript
> const ALPHABET = "abcdefghijklmnopqrstuvwxyz";
>
> function cesar(texte, decalage) {
>   let resultat = "";
>   for (const c of texte) {
>     const position = ALPHABET.indexOf(c);
>     if (position === -1) {
>       resultat += c;
>     } else {
>       resultat += ALPHABET[(position + decalage) % 26];
>     }
>   }
>   return resultat;
> }
>
> const secret = cesar("bonjour zoe", 3);
> console.log(secret);
> console.log(cesar(secret, 23));
> ```
>
> ```sortie
> erqmrxu crh
> bonjour zoe
> ```
>
> Pourquoi ça marche : `% 26` fait tourner l'alphabet en rond, comme les aiguilles
> d'une horloge. Pour `z` : position 25, 25 + 3 = 28, 28 % 26 = 2, c'est `c`. Décaler
> de 3 puis de 23 fait 26 : un tour complet, on revient au départ.
>
> ```text
>  a b c d e … x y z
>  ▲               │
>  └───────────────┘
>   après z, on repart à a
> ```

## Troisième niveau

### Exercice 10 : Le tri à bulles

Niveau : ★★★

Triez `[5, 2, 4, 1]` du plus petit au plus grand **sans** utiliser `sort`. Méthode :
parcourez le tableau et échangez deux voisins s'ils sont dans le mauvais ordre.
Recommencez jusqu'à ce que tout soit trié. Affichez le tableau après chaque passe. Le
tableau de départ ne doit pas changer.

Sortie attendue :

```text
Après la passe 1 : 2 4 1 5
Après la passe 2 : 2 1 4 5
Après la passe 3 : 1 2 4 5
Trié : 1 2 4 5, original : 5 2 4 1
```

> **Indice :** travaillez sur une copie : `const t = [...tableau];`. Pour échanger
> `t[i]` et `t[i + 1]`, gardez l'un dans une boîte `temp` (voir
> [suivre un programme pas à pas](pas-a-pas.md)).

> **Indice :** après chaque passe, le plus grand nombre restant est arrivé au bout,
> comme une bulle qui remonte. Pour un tableau de 4 nombres, il faut 3 passes, et la
> passe numéro `p` s'arrête avant les `p` dernières cases.

> **Solution**
>
> ```javascript
> function triABulles(tableau) {
>   const t = [...tableau];
>   for (let passe = 1; passe < t.length; passe++) {
>     for (let i = 0; i < t.length - passe; i++) {
>       if (t[i] > t[i + 1]) {
>         const temp = t[i];
>         t[i] = t[i + 1];
>         t[i + 1] = temp;
>       }
>     }
>     console.log(`Après la passe ${passe} : ${t.join(" ")}`);
>   }
>   return t;
> }
>
> const nombres = [5, 2, 4, 1];
> const tries = triABulles(nombres);
> console.log(`Trié : ${tries.join(" ")}, original : ${nombres.join(" ")}`);
> ```
>
> ```sortie
> Après la passe 1 : 2 4 1 5
> Après la passe 2 : 2 1 4 5
> Après la passe 3 : 1 2 4 5
> Trié : 1 2 4 5, original : 5 2 4 1
> ```
>
> Pourquoi ça marche : chaque passe pousse le plus grand nombre restant vers la
> droite. Voici la première passe en détail :
>
> | passe | i | compare | échange ? | t |
> | --- | --- | --- | --- | --- |
> | 1 | 0 | 5 > 2 | oui | `[2, 5, 4, 1]` |
> | 1 | 1 | 5 > 4 | oui | `[2, 4, 5, 1]` |
> | 1 | 2 | 5 > 1 | oui | `[2, 4, 1, 5]` |
>
> Le 5 est au bout : il est à sa place. La passe 2 ne regarde plus que les 3 premières
> cases, la passe 3 les 2 premières.

### Exercice 11 : Deviner un nombre en peu d'essais

Niveau : ★★★

Un nombre secret est entre 1 et 100. À chaque essai, on vous dit « trop petit » ou
« trop grand ». Écrivez `deviner(secret)` qui essaie toujours le **milieu** de
l'intervalle encore possible, et affiche chaque essai. Testez avec `71`.

Sortie attendue :

```text
50 : trop petit
75 : trop grand
62 : trop petit
68 : trop petit
71 : trouvé en 5 essais
```

> **Indice :** gardez deux bornes, `bas = 1` et `haut = 100`. Le milieu est
> `Math.floor((bas + haut) / 2)`.

> **Indice :** si le milieu est trop petit, le secret est au-dessus : `bas = milieu + 1`.
> S'il est trop grand : `haut = milieu - 1`. Répétez avec `while (bas <= haut)`.

> **Solution**
>
> ```javascript
> function deviner(secret) {
>   let bas = 1;
>   let haut = 100;
>   let essais = 0;
>   while (bas <= haut) {
>     const milieu = Math.floor((bas + haut) / 2);
>     essais = essais + 1;
>     if (milieu === secret) {
>       console.log(`${milieu} : trouvé en ${essais} essais`);
>       return essais;
>     } else if (milieu < secret) {
>       console.log(`${milieu} : trop petit`);
>       bas = milieu + 1;
>     } else {
>       console.log(`${milieu} : trop grand`);
>       haut = milieu - 1;
>     }
>   }
>   return -1;
> }
>
> deviner(71);
> ```
>
> ```sortie
> 50 : trop petit
> 75 : trop grand
> 62 : trop petit
> 68 : trop petit
> 71 : trouvé en 5 essais
> ```
>
> Pourquoi ça marche : chaque essai coupe l'intervalle en deux, comme quand on ouvre
> un dictionnaire au milieu pour chercher un mot. Il ne faut jamais plus de 7 essais
> pour 100 nombres.
>
> | Essai | bas | haut | milieu | réponse |
> | --- | --- | --- | --- | --- |
> | 1 | 1 | 100 | 50 | trop petit |
> | 2 | 51 | 100 | 75 | trop grand |
> | 3 | 51 | 74 | 62 | trop petit |
> | 4 | 63 | 74 | 68 | trop petit |
> | 5 | 69 | 74 | 71 | trouvé |

### Exercice 12 : Les parenthèses équilibrées

Niveau : ★★★

Écrivez `equilibre(texte)` qui dit si les parenthèses `()`, crochets `[]` et
accolades `{}` d'un texte sont bien fermées, dans le bon ordre. Les autres caractères
sont ignorés. Testez avec `"(a[b]{c})"`, `"(]"`, `"((a)"` et `"f(x[0])"`.

Sortie attendue :

```text
(a[b]{c}) : true
(] : false
((a) : false
f(x[0]) : true
```

> **Indice :** utilisez un tableau comme une **pile** d'assiettes : `push` pose une
> ouvrante sur le dessus, `pop` reprend celle du dessus.

> **Indice :** à chaque fermante, la dernière ouvrante posée doit être sa partenaire.
> Un objet `{ ")": "(", "]": "[", "}": "{" }` donne la partenaire. À la fin, la pile
> doit être vide.

> **Solution**
>
> ```javascript
> function equilibre(texte) {
>   const ouvrantes = { ")": "(", "]": "[", "}": "{" };
>   const pile = [];
>   for (const c of texte) {
>     if (c === "(" || c === "[" || c === "{") {
>       pile.push(c);
>     } else if (c in ouvrantes) {
>       if (pile.pop() !== ouvrantes[c]) {
>         return false;
>       }
>     }
>   }
>   return pile.length === 0;
> }
>
> for (const texte of ["(a[b]{c})", "(]", "((a)", "f(x[0])"]) {
>   console.log(`${texte} : ${equilibre(texte)}`);
> }
> ```
>
> ```sortie
> (a[b]{c}) : true
> (] : false
> ((a) : false
> f(x[0]) : true
> ```
>
> Pourquoi ça marche : la dernière ouvrante posée est la première à fermer, comme la
> dernière assiette posée est la première reprise. Voici la trace de `"(a[b]{c})"` :
>
> | Tour | c | action | pile |
> | --- | --- | --- | --- |
> | 1 | `(` | poser | `["("]` |
> | 2 | `a` | ignorer | `["("]` |
> | 3 | `[` | poser | `["(", "["]` |
> | 4 | `b` | ignorer | `["(", "["]` |
> | 5 | `]` | reprendre `[` : partenaire, ok | `["("]` |
> | 6 | `{` | poser | `["(", "{"]` |
> | 7 | `c` | ignorer | `["(", "{"]` |
> | 8 | `}` | reprendre `{` : partenaire, ok | `["("]` |
> | 9 | `)` | reprendre `(` : partenaire, ok | `[]` |
>
> La pile est vide à la fin : `true`. Pour `"((a)"`, il reste un `(` dans la pile :
> `false`. Pour `"(]"`, on reprend `(` alors qu'il fallait `[` : `false` tout de suite.

## En résumé

- Les exercices 1 à 4 travaillent une notion à la fois : fonction, indice, champion,
  répétition.
- Les exercices 5 à 9 combinent boucles, conditions, tableaux, objets et texte.
- Les exercices 10 à 12 sont de vrais petits algorithmes : trier, chercher, vérifier.
- Quand une solution vous échappe, remplissez son tableau de trace avec un crayon.

Suite : gardez [l'aide-mémoire](aide-memoire.md) sous la main.
