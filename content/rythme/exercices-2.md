# Exercices corrigés : deuxième série

Douze nouveaux exercices pour continuer à vous entraîner en Rythme, du plus facile au plus difficile, chacun avec un indice, une solution vérifiée et une explication pas à pas.

## Comment faire ces exercices

Cette série complète [les exercices pour bien démarrer](exercices-faciles.md)
et [la première série d'exercices corrigés](exercices.md). Aucun exercice
ne se répète. Chaque exercice a un niveau :

- ★☆☆ facile : quelques lignes, une seule idée ;
- ★★☆ moyen : il faut combiner deux ou trois idées ;
- ★★★ plus dur : prenez votre temps, et un crayon.

Pour chaque exercice, suivez la même méthode :

```text
1. Lisez l'énoncé et le résultat attendu.
2. Sur papier : quelles valeurs entrent ? quelle valeur doit sortir ?
3. Écrivez le code dans exo.ry, puis :  rythme run exo.ry
4. Comparez avec le résultat attendu.
5. Bloqué ? Lisez l'indice. Toujours bloqué ? Ouvrez la solution.
6. Lisez le tableau pas à pas de la solution, même si vous avez réussi.
```

Les tableaux pas à pas suivent la valeur de chaque nom, étape par étape.
C'est comme regarder un film image par image : on voit exactement à quel
moment les choses changent. La page [rythme etapes](etapes.md) montre comment
Rythme peut faire ce travail pour vous.

> **À retenir :** essayez d'abord, même cinq minutes. Une solution lue sans
> avoir cherché s'oublie vite ; une solution trouvée après avoir cherché
> reste.

## Niveau ★☆☆ : une seule idée

### Exercice 1 : Le thermomètre américain

Niveau : ★☆☆

Aux États-Unis, la température se donne en degrés Fahrenheit. Pour passer
des degrés Celsius aux degrés Fahrenheit, on multiplie par 9, on divise
par 5, puis on ajoute 32. Écrivez `fahrenheit(c)`, puis convertissez toute
la liste `[0, 20, 37, 100]` d'un coup :

```text
[32.0, 68.0, 98.6, 212.0]
```

> **Indice :** commencez par la fonction pour **un seul** nombre :
> `fn fahrenheit(c) = ...`. Ensuite, `map` l'applique à chaque élément de la
> liste. Voir [penser fonctionnel](fonctionnel.md).

> **Solution**
>
> ```rythme
> fn fahrenheit(c) = c * 9 / 5 + 32
>
> print([0, 20, 37, 100] |> map(fahrenheit))
> ```
>
> ```sortie
> [32.0, 68.0, 98.6, 212.0]
> ```
>
> Pourquoi ça marche : `map` est comme une machine à tampon. Chaque nombre
> passe dessous, ressort transformé, et la nouvelle liste garde le même
> ordre. Le `/` donne toujours un nombre à virgule, d'où le `.0`.
>
> | Étape | c | c * 9 | / 5 | + 32 |
> |-------|---|-------|-----|------|
> | 1 | 0 | 0 | 0.0 | 32.0 |
> | 2 | 20 | 180 | 36.0 | 68.0 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180.0 | 212.0 |

Essayez : remplacez `/` par `//` (la division entière). Vous obtenez
`[32, 68, 98, 212]` : le `.6` de 98.6 a disparu, car `333 // 5` vaut 66.

### Exercice 2 : Les initiales

Niveau : ★☆☆

Écrivez `initiales(nom)` qui donne les initiales d'un nom, en majuscules,
chacune suivie d'un point :

```text
A. L.
J. P. S.
```

pour `"ada lovelace"` et `"Jean Paul Sartre"`.

> **Indice :** trois étapes à enchaîner avec `|>` : `split` coupe le nom
> en mots ; `map` transforme chaque mot en `"A."` ; `join(" ")` recolle les
> morceaux. La première lettre d'un mot `m` s'écrit `m[0]`.

> **Solution**
>
> ```rythme
> fn initiale(mot) = upper(mot[0]) + "."
>
> fn initiales(nom) = split(nom) |> map(initiale) |> join(" ")
>
> print(initiales("ada lovelace"))
> print(initiales("Jean Paul Sartre"))
> ```
>
> ```sortie
> A. L.
> J. P. S.
> ```
>
> Pourquoi ça marche : chaque `|>` passe le résultat à l'étape suivante,
> comme sur une chaîne de montage.
>
> ```text
> "ada lovelace"
>    │ split
>    ▼
> ["ada", "lovelace"]
>    │ map(initiale)
>    ▼
> ["A.", "L."]
>    │ join(" ")
>    ▼
> "A. L."
> ```

Essayez : changez `join(" ")` en `join("")`. Vous obtenez `A.L.`, sans
espace.

### Exercice 3 : Compter les voyelles

Niveau : ★☆☆

Combien de voyelles (a, e, i, o, u, y) y a-t-il dans le mot
`anticonstitutionnellement` ? Résultat attendu :

```text
10
```

> **Indice :** `chars(mot)` donne la liste des lettres. `count(liste, f)`
> compte les éléments pour lesquels `f` répond `true`. Pour savoir si une
> lettre est une voyelle, cherchez-la dans le texte `"aeiouy"` avec
> `contains`.

> **Solution**
>
> ```rythme
> fn voyelle?(lettre) = contains("aeiouy", lettre)
>
> mot = "anticonstitutionnellement"
> print(count(chars(mot), voyelle?))
> ```
>
> ```sortie
> 10
> ```
>
> Pourquoi ça marche : `voyelle?` répond à une question par oui ou non,
> d'où le `?` à la fin de son nom. `count` pose la question à chaque
> lettre et compte les « oui ». C'est comme un videur qui compte les
> personnes avec un badge.
>
> | Lettres | a | n | t | i | c | o | n | s | t | i | ... |
> |---------|---|---|---|---|---|---|---|---|---|---|-----|
> | voyelle? | oui | non | non | oui | non | oui | non | non | non | oui | ... |
> | compte | 1 | 1 | 1 | 2 | 2 | 3 | 3 | 3 | 3 | 4 | ... |

Essayez : remplacez le mot par `"Rythme"`. Vous obtenez `2` : le `y` et
le `e`. Avec `"AVION"`, vous obtenez `0`, car `"aeiouy"` ne contient que
des minuscules. Ajoutez `lower` pour corriger : `chars(lower(mot))`.

### Exercice 4 : Le panier

Niveau : ★☆☆

Un panier contient des articles, chacun rangé dans un enregistrement avec
un nom et un prix. Affichez le nombre d'articles et le total à payer :

```text
3 articles, total : 12 €
```

> **Indice :** `len` compte les articles. `sum(liste, f)` additionne les
> valeurs que `f` donne pour chaque élément : ici, le champ `prix`.

> **Solution**
>
> ```rythme
> panier = [
>   {article: "pain", prix: 2},
>   {article: "fromage", prix: 7},
>   {article: "jus", prix: 3}
> ]
> total = sum(panier, fn(a) = a.prix)
> print("{len(panier)} articles, total : {total} €")
> ```
>
> ```sortie
> 3 articles, total : 12 €
> ```
>
> Pourquoi ça marche : `fn(a) = a.prix` est une petite fonction sans nom
> qui « ouvre » chaque enregistrement et en sort le prix. `sum` additionne
> ces prix un par un.
>
> | Étape | a.article | a.prix | total en cours |
> |-------|-----------|--------|----------------|
> | départ | | | 0 |
> | 1 | pain | 2 | 2 |
> | 2 | fromage | 7 | 9 |
> | 3 | jus | 3 | 12 |

Essayez : ajoutez `{article: "pommes", prix: 4}` au panier. Le message
devient `4 articles, total : 16 €`, sans rien changer d'autre.

> **À retenir :** pour une liste, pensez d'abord « que faire à **un**
> élément ? ». Écrivez cette petite fonction, puis confiez la liste à
> `map`, `count` ou `sum`.

## Niveau ★★☆ : combiner deux ou trois idées

### Exercice 5 : FizzBuzz

Niveau : ★★☆

C'est un jeu d'école : on compte à voix haute, mais on dit « Fizz » pour un
multiple de 3, « Buzz » pour un multiple de 5, et « FizzBuzz » pour un
multiple des deux. Écrivez `fizzbuzz(n)` avec des gardes, puis affichez les
réponses de 1 à 15 sur une ligne :

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
```

> **Indice :** un nombre est multiple de 3 si `n % 3 == 0`. Écrivez une
> définition par cas, avec une garde par cas (voir
> [les gardes](fonctions.md#les-gardes)). Le dernier cas donne le nombre en
> texte : `text(n)`.

> **Indice :** l'ordre des cas compte. Rythme essaie les définitions de
> haut en bas : le cas « multiple de 15 » doit venir en premier.

> **Solution**
>
> ```rythme
> fn fizzbuzz(n) if n % 15 == 0 = "FizzBuzz"
> fn fizzbuzz(n) if n % 3 == 0 = "Fizz"
> fn fizzbuzz(n) if n % 5 == 0 = "Buzz"
> fn fizzbuzz(n) = text(n)
>
> print(1..15 |> map(fizzbuzz) |> join(" "))
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> Pourquoi ça marche : un nombre multiple de 3 **et** de 5 est un multiple
> de 15. On teste donc 15 d'abord. Pour 15, voici les cas essayés :
>
> | Cas essayé | Garde | Résultat |
> |------------|-------|----------|
> | 1 | `15 % 15 == 0` → `true` | `"FizzBuzz"`, on s'arrête |
>
> Pour 9 : `9 % 15` vaut 9 (faux), puis `9 % 3` vaut 0 (vrai) : `"Fizz"`.
> Pour 7, aucune garde n'est vraie : le dernier cas donne `"7"`.

Essayez : mettez le cas `n % 15` en dernier, juste avant `text(n)`. Pour
15, vous obtenez `Fizz` : le cas du 3 a répondu le premier.

### Exercice 6 : Le relevé de compte

Niveau : ★★☆

Un compte en banque contient 100 €. Voici les opérations du mois, chacune
avec un symbole `:depot` ou `:retrait` (voir [les symboles](symboles.md)) :

```text
dépôt 50, retrait 20, dépôt 15, retrait 30
```

Affichez le solde après chaque opération, puis le solde final :

```text
[150, 130, 145, 115]
115
```

> **Indice :** écrivez `appliquer(solde, operation)` qui donne le nouveau
> solde, avec un `match` sur l'enregistrement. Ensuite, `fold` passe le
> solde d'une opération à la suivante, et `scan` montre chaque étape.

> **Solution**
>
> ```rythme
> fn appliquer(solde, operation) = match operation
>   {type: :depot, montant: m} -> solde + m
>   {type: :retrait, montant: m} -> solde - m
> end
>
> operations = [
>   {type: :depot, montant: 50},
>   {type: :retrait, montant: 20},
>   {type: :depot, montant: 15},
>   {type: :retrait, montant: 30}
> ]
> print(scan(operations, 100, appliquer))
> print(fold(operations, 100, appliquer))
> ```
>
> ```sortie
> [150, 130, 145, 115]
> 115
> ```
>
> Pourquoi ça marche : `fold` est comme le carnet d'un caissier. Il part
> de 100, et chaque ligne du carnet utilise le solde de la ligne
> précédente. Rien n'est modifié : chaque étape fabrique un nouveau solde.
>
> | Étape | solde avant | type | montant | solde après |
> |-------|-------------|------|---------|-------------|
> | 1 | 100 | `:depot` | 50 | 150 |
> | 2 | 150 | `:retrait` | 20 | 130 |
> | 3 | 130 | `:depot` | 15 | 145 |
> | 4 | 145 | `:retrait` | 30 | 115 |

Essayez : remplacez le 100 de départ par 0. Le solde final devient `15`,
et le deuxième élément de `scan` devient `30`.

### Exercice 7 : Le code de César

Niveau : ★★☆

Jules César cachait ses messages en décalant chaque lettre de 3 places dans
l'alphabet : `a` devient `d`, `b` devient `e`… et `z` revient au début pour
devenir `c`. Écrivez `cesar(texte, decalage)` pour un texte en minuscules
sans accents. Les espaces ne changent pas. Vérifiez qu'un second décalage
de 23 (3 + 23 = 26, un tour complet) rend le message d'origine :

```text
erqmrxu crh
bonjour zoe
```

> **Indice :** chaque caractère a un numéro : `char_code("a")` vaut 97,
> et `char(97)` redonne `"a"`. La position d'une lettre dans l'alphabet est
> donc `char_code(c) - 97` (de 0 pour `a` à 25 pour `z`).

> **Indice :** pour revenir au début après `z`, prenez le reste de la
> division par 26 : `(position + decalage) % 26`.

> **Solution**
>
> ```rythme
> alphabet = "abcdefghijklmnopqrstuvwxyz"
>
> fn decaler(c, k) =
>   if contains(alphabet, c) then char((char_code(c) - 97 + k) % 26 + 97) else c
>
> fn cesar(texte, k) = chars(texte) |> map(fn(c) = decaler(c, k)) |> join("")
>
> secret = cesar("bonjour zoe", 3)
> print(secret)
> print(cesar(secret, 23))
> ```
>
> ```sortie
> erqmrxu crh
> bonjour zoe
> ```
>
> Pourquoi ça marche : l'alphabet est rangé en cercle, comme les heures
> d'une horloge. Le `% 26` fait le tour du cadran.
>
> ```text
>   ... w  x  y  z  a  b  c  d ...
>                └──────────►c   z avance de 3 : 25 + 3 = 28, 28 % 26 = 2 → c
> ```
>
> | c | char_code(c) - 97 | + 3 | % 26 | lettre |
> |---|-------------------|-----|------|--------|
> | `b` | 1 | 4 | 4 | `e` |
> | `o` | 14 | 17 | 17 | `r` |
> | `z` | 25 | 28 | 2 | `c` |
> | espace | | | | espace (inchangé) |

Essayez : `cesar("abc", 1)` donne `bcd`, et `cesar("abc", 26)` redonne
`abc` : un tour complet ne change rien.

### Exercice 8 : La somme des chiffres

Niveau : ★★☆

Écrivez une fonction **récursive** `somme_chiffres(n)` qui additionne les
chiffres d'un nombre entier positif : pour 1234, c'est 1 + 2 + 3 + 4 = 10.
N'utilisez pas de texte, seulement des calculs :

```text
10 9 18
```

pour 1234, 9 et 99.

> **Indice :** `n % 10` donne le dernier chiffre (1234 % 10 vaut 4), et
> `n // 10` enlève ce dernier chiffre (1234 // 10 vaut 123).

> **Indice :** le cas de base est 0 : il n'y a plus de chiffre, la somme
> vaut 0. Voir [la récursion](recursion.md).

> **Solution**
>
> ```rythme
> fn somme_chiffres(0) = 0
> fn somme_chiffres(n) = n % 10 + somme_chiffres(n // 10)
>
> print(somme_chiffres(1234), somme_chiffres(9), somme_chiffres(99))
> ```
>
> ```sortie
> 10 9 18
> ```
>
> Pourquoi ça marche : à chaque appel, on « arrache » le dernier chiffre,
> comme on détache les tickets d'un carnet, jusqu'à ce qu'il ne reste rien.
>
> | Appel | n | n % 10 | n // 10 | en attente |
> |-------|---|--------|---------|------------|
> | 1 | 1234 | 4 | 123 | `4 + ...` |
> | 2 | 123 | 3 | 12 | `4 + 3 + ...` |
> | 3 | 12 | 2 | 1 | `4 + 3 + 2 + ...` |
> | 4 | 1 | 1 | 0 | `4 + 3 + 2 + 1 + ...` |
> | 5 | 0 | cas de base | | `4 + 3 + 2 + 1 + 0` = 10 |

Essayez : enlevez la ligne `fn somme_chiffres(0) = 0`. La fonction ne
s'arrête plus jamais, car `0 // 10` vaut encore 0. Rythme finit par
s'arrêter avec l'erreur `E014` : « trop d'appels imbriqués ».

### Exercice 9 : Ranger par initiale

Niveau : ★★☆

Voici une liste de fruits. Rangez-les par première lettre, puis affichez
les fruits qui commencent par `p` :

```text
{p: ["pomme", "poire", "prune"], b: ["banane", "brugnon"], a: ["abricot"]}
["pomme", "poire", "prune"]
```

> **Indice :** `group_by(liste, f)` range les éléments dans des tiroirs :
> le nom du tiroir est ce que `f` répond pour chaque élément. Ici, `f`
> donne la première lettre.

> **Solution**
>
> ```rythme
> fruits = ["pomme", "poire", "banane", "abricot", "brugnon", "prune"]
> rangement = group_by(fruits, fn(f) = f[0])
> print(rangement)
> print(rangement.p)
> ```
>
> ```sortie
> {p: ["pomme", "poire", "prune"], b: ["banane", "brugnon"], a: ["abricot"]}
> ["pomme", "poire", "prune"]
> ```
>
> Pourquoi ça marche : `group_by` fait comme quelqu'un qui trie le
> courrier. Pour chaque lettre, il lit l'étiquette et la pose dans le bon
> casier. Un nouveau casier s'ouvre la première fois qu'une étiquette
> apparaît, d'où l'ordre `p`, `b`, `a`.
>
> ```text
> ┌───────────────────────────┐ ┌───────────────────┐ ┌───────────┐
> │ p                         │ │ b                 │ │ a         │
> │ pomme  poire  prune       │ │ banane  brugnon   │ │ abricot   │
> └───────────────────────────┘ └───────────────────┘ └───────────┘
> ```

Essayez : rangez par longueur avec `fn(f) = len(f)`. Les tiroirs
s'appellent alors `5`, `6` et `7`.

> **À retenir :** un exercice moyen se découpe en petites fonctions d'une
> ligne (`appliquer`, `decaler`, `initiale`…). Testez chacune seule avant
> de les assembler.

## Niveau ★★★ : prenez votre temps

### Exercice 10 : Compresser un texte

Niveau : ★★★

Pour gagner de la place, on peut écrire `aaabcc` sous la forme `a3b1c2` :
chaque lettre suivie du nombre de fois où elle se répète **d'affilée**.
Écrivez `compresser(texte)` :

```text
[["a", 3], ["b", 1], ["c", 2]]
a3b1c2 z10
```

La première ligne montre les paquets de `"aaabcc"`, la seconde les textes
compressés de `"aaabcc"` et de `"zzzzzzzzzz"`.

> **Indice :** écrivez d'abord une fonction récursive `paquets(lettres)`
> sur la liste des lettres. Pour la liste `[c, ..reste]`, `take_while`
> prend dans `reste` les lettres égales à `c`, et `drop` enlève ces
> lettres-là.

> **Indice :** le cas de base est la liste vide, qui donne `[]`. Chaque
> paquet est une petite liste `[lettre, nombre]`.

> **Solution**
>
> ```rythme
> fn paquets([]) = []
> fn paquets([c, ..reste])
>   pareilles = take_while(reste, fn(x) = x == c)
>   suite = drop(reste, len(pareilles))
>   [[c, 1 + len(pareilles)], ..paquets(suite)]
> end
>
> fn compresser(texte) =
>   paquets(chars(texte)) |> map(fn(p) = "{p[0]}{p[1]}") |> join("")
>
> print(paquets(chars("aaabcc")))
> print(compresser("aaabcc"), compresser("zzzzzzzzzz"))
> ```
>
> ```sortie
> [["a", 3], ["b", 1], ["c", 2]]
> a3b1c2 z10
> ```
>
> Pourquoi ça marche : c'est comme ranger des perles de couleur. On prend
> la première perle, on attrape toutes celles de la même couleur qui
> suivent, on note « couleur, nombre », puis on recommence avec ce qui
> reste.
>
> | Appel | c | reste | pareilles | suite | paquet |
> |-------|---|-------|-----------|-------|--------|
> | 1 | `a` | `a a b c c` | `a a` | `b c c` | `["a", 3]` |
> | 2 | `b` | `c c` | (vide) | `c c` | `["b", 1]` |
> | 3 | `c` | `c` | `c` | (vide) | `["c", 2]` |
> | 4 | liste vide | | | | cas de base : `[]` |

Essayez : `compresser("abab")` donne `a1b1a1b1`. Les lettres doivent se
suivre pour former un paquet : ce texte ne se compresse pas bien du tout.

### Exercice 11 : Le juste prix

Niveau : ★★★

Un nombre secret est entre 1 et 100. La meilleure stratégie : proposer le
nombre du **milieu**. Si c'est trop petit, on cherche dans la moitié haute ;
si c'est trop grand, dans la moitié basse. Écrivez
`deviner(secret, bas, haut, essais)` qui compte les essais nécessaires.
Pour les secrets 37, 50, 1 et 100 :

```text
3 1 6 7
```

> **Indice :** le milieu se calcule avec la division entière :
> `(bas + haut) // 2`. Trois cas : trouvé, trop petit, trop grand.

> **Indice :** « trop petit » veut dire que le secret est au-dessus du
> milieu : on recommence avec `milieu + 1` comme nouveau `bas`, et un essai
> de plus.

> **Solution**
>
> ```rythme
> fn deviner(secret, bas, haut, essais)
>   milieu = (bas + haut) // 2
>   if milieu == secret then essais
>   else if milieu < secret then deviner(secret, milieu + 1, haut, essais + 1)
>   else deviner(secret, bas, milieu - 1, essais + 1)
> end
>
> print(deviner(37, 1, 100, 1), deviner(50, 1, 100, 1), deviner(1, 1, 100, 1), deviner(100, 1, 100, 1))
> ```
>
> ```sortie
> 3 1 6 7
> ```
>
> Pourquoi ça marche : à chaque essai, la zone de recherche est coupée en
> deux, comme quand on ouvre un dictionnaire au milieu pour trouver un mot.
> Sept essais suffisent toujours pour 100 nombres. Voici la partie pour le
> secret 37 :
>
> | Essai | bas | haut | milieu | réponse |
> |-------|-----|------|--------|---------|
> | 1 | 1 | 100 | 50 | trop grand : `haut` devient 49 |
> | 2 | 1 | 49 | 25 | trop petit : `bas` devient 26 |
> | 3 | 26 | 49 | 37 | trouvé : la fonction donne 3 |
>
> ```text
> 1 ─────────────────────────────── 100     essai 1 : 50 ?
> 1 ─────────────── 49                       essai 2 : 25 ?
>             26 ── 49                       essai 3 : 37 !
> ```

Essayez : ajoutez `print(bas, haut, milieu)` juste après la ligne
`milieu = ...`. Le programme affiche alors chaque essai, et vous pouvez
comparer avec votre tableau.

### Exercice 12 : La calculatrice polonaise

Niveau : ★★★

Dans la notation « polonaise inverse », on écrit les nombres **avant**
l'opération : `3 4 +` veut dire 3 + 4, et `3 4 + 2 *` veut dire
(3 + 4) × 2. Les calculatrices de certains ingénieurs marchent ainsi.
Écrivez `rpn(texte)` qui donne un `Result` : `Ok(résultat)`, ou `Err(...)`
si le calcul est impossible.

```text
Ok(7)
Ok(14)
Ok(4)
Err("il manque un nombre avant +")
Err("il reste plusieurs nombres")
```

pour `"3 4 +"`, `"3 4 + 2 *"`, `"10 2 3 * -"`, `"3 +"` et `"3 4"`.

> **Indice :** utilisez une **pile**, comme une pile d'assiettes : un
> nombre se pose sur le dessus ; une opération prend les deux assiettes du
> dessus, calcule, et repose le résultat. Avec une liste, le dessus est le
> premier élément : `[n, ..pile]` pose `n`.

> **Indice :** écrivez `etape(pile, mot)` qui donne `Ok(nouvelle_pile)` ou
> `Err(message)`, puis une fonction récursive `evaluer(mots, pile)` qui
> utilise `?` pour s'arrêter à la première erreur (voir
> [gérer les erreurs](erreurs.md)).

> **Solution**
>
> ```rythme
> operations = ["+", "-", "*"]
>
> fn calculer(op, a, b) = match op
>   "+" -> a + b
>   "-" -> a - b
>   _ -> a * b
> end
>
> fn etape([b, a, ..reste], op) if contains(operations, op) =
>   Ok([calculer(op, a, b), ..reste])
> fn etape(_, op) if contains(operations, op) = Err("il manque un nombre avant {op}")
> fn etape(pile, mot)
>   n = parse_int(mot)?
>   Ok([n, ..pile])
> end
>
> fn evaluer([], [resultat]) = Ok(resultat)
> fn evaluer([], _) = Err("il reste plusieurs nombres")
> fn evaluer([mot, ..reste], pile)
>   nouvelle = etape(pile, mot)?
>   evaluer(reste, nouvelle)
> end
>
> fn rpn(texte) = evaluer(split(texte), [])
>
> print(rpn("3 4 +"))
> print(rpn("3 4 + 2 *"))
> print(rpn("10 2 3 * -"))
> print(rpn("3 +"))
> print(rpn("3 4"))
> ```
>
> ```sortie
> Ok(7)
> Ok(14)
> Ok(4)
> Err("il manque un nombre avant +")
> Err("il reste plusieurs nombres")
> ```
>
> Pourquoi ça marche : la pile retient les nombres en attente. Attention à
> l'ordre : le dessus de la pile est le **deuxième** nombre (`b`), d'où
> `calculer(op, a, b)` et pas `(op, b, a)`. Pour `10 2 3 * -` :
>
> | Étape | mot | pile avant | pile après |
> |-------|-----|------------|------------|
> | 1 | `10` | `[]` | `[10]` |
> | 2 | `2` | `[10]` | `[2, 10]` |
> | 3 | `3` | `[2, 10]` | `[3, 2, 10]` |
> | 4 | `*` | `[3, 2, 10]` | `[6, 10]` (2 × 3) |
> | 5 | `-` | `[6, 10]` | `[4]` (10 − 6) |
> | fin | | `[4]` | `Ok(4)` |
>
> ```text
>         ┌───┐
>         │ 3 │          ┌───┐
>         │ 2 │   *      │ 6 │   -      ┌───┐
>         │10 │  ───►    │10 │  ───►    │ 4 │
>         └───┘          └───┘          └───┘
> ```

Essayez : `print(rpn("3 x +"))`. Le mot `x` n'est pas un nombre :
`parse_int` donne une erreur, et `?` la fait remonter jusqu'au bout.

> **À retenir :** pour un exercice difficile, faites le tableau pas à pas
> **avant** d'écrire le code. Si vous savez le faire à la main, vous savez
> quoi demander à Rythme.

## Et ensuite ?

Bravo ! Vous avez fini les trois séries d'exercices de ce parcours. Pour
continuer :

- `rythme exercise list` propose 30 autres exercices, vérifiés par des
  tests ;
- [les projets guidés](projets.md) assemblent tout cela dans de vrais
  programmes ;
- [l'aide-mémoire](aide-memoire.md) résume toute la syntaxe sur une page.

Les autres parcours ont aussi leur deuxième série, par exemple
[en Ruby](../ruby/exercices-2.md) et [en Python](../python/exercices-2.md).

Suite : [Aide-mémoire](aide-memoire.md)
