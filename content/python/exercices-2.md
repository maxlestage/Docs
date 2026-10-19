# Exercices corrigés : deuxième série

Cette page propose 12 nouveaux exercices corrigés, du plus facile au plus difficile, avec des tableaux qui suivent les variables pas à pas dans les solutions.

## Comment utiliser cette page

Ces exercices sont **nouveaux** : ils ne reprennent pas ceux des
[exercices pour bien démarrer](exercices-faciles.md) ni des
[exercices corrigés](exercices.md). Chacun a un niveau :

- ★☆☆ : facile, une seule idée ;
- ★★☆ : moyen, deux ou trois idées ensemble ;
- ★★★ : plus dur, il faut réfléchir avant d'écrire.

Pour chaque exercice :

1. Lisez l'énoncé et la sortie attendue.
2. Écrivez les étapes en français sur un papier.
3. Écrivez le code dans un fichier `.py` et lancez-le avec `python3`.
4. Bloqué ? Lisez l'indice. Toujours bloqué ? Ouvrez la solution.
5. Lisez « Pourquoi ça marche » même si vous avez réussi.

Beaucoup de solutions contiennent un **tableau de suivi** : une colonne par variable,
une ligne par tour de boucle. Si vous ne connaissez pas encore cet outil, lisez d'abord
[Suivre un programme pas à pas](pas-a-pas.md).

> **À retenir :** un exercice raté puis compris vaut mieux qu'un exercice réussi par
> chance. Comparez toujours votre solution avec celle de la page.

## Niveau ★☆☆ : pour s'échauffer

### Exercice 1 : Le ticket de caisse

Niveau : ★☆☆

Vous avez une liste d'achats : `[("pain", 2), ("lait", 1), ("fromage", 6)]`. Chaque
achat est un tuple (nom, prix). Affichez une ligne par achat, puis le total.

```text
pain ... 2 €
lait ... 1 €
fromage ... 6 €
TOTAL ... 9 €
```

> **Indice :** `for nom, prix in articles:` découpe chaque tuple en deux variables.
> Gardez une tirelire `total = 0` avant la boucle.

> **Solution**
>
> ```python
> articles = [("pain", 2), ("lait", 1), ("fromage", 6)]
> total = 0
> for nom, prix in articles:
>     print(f"{nom} ... {prix} €")
>     total = total + prix
> print(f"TOTAL ... {total} €")
> ```
>
> ```sortie
> pain ... 2 €
> lait ... 1 €
> fromage ... 6 €
> TOTAL ... 9 €
> ```
>
> Pourquoi ça marche : comme une caissière, la boucle passe chaque article, l'affiche, et
> ajoute son prix au total.
>
> | Tour | `nom` | `prix` | `total` après |
> |---|---|---|---|
> | 1 | pain | 2 | 2 |
> | 2 | lait | 1 | 3 |
> | 3 | fromage | 6 | 9 |
>
> Le dernier `print` n'est pas indenté : il s'exécute une seule fois, après la boucle.

### Exercice 2 : Des degrés Celsius aux degrés Fahrenheit

Niveau : ★☆☆

Écrivez une fonction `fahrenheit(celsius)` qui renvoie la température en degrés
Fahrenheit. La formule est : `celsius × 9 / 5 + 32`. Affichez la conversion de 0, 20,
37 et 100.

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Indice :** la fonction tient en une ligne, avec `return`. Appelez-la ensuite dans
> une boucle sur la liste `[0, 20, 37, 100]`.

> **Solution**
>
> ```python
> def fahrenheit(celsius):
>     return celsius * 9 / 5 + 32
>
> for c in [0, 20, 37, 100]:
>     print(f"{c} °C = {fahrenheit(c)} °F")
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Pourquoi ça marche : la fonction est une petite machine à convertir ; la boucle lui
> donne chaque température à tour de rôle. Le résultat a un `.0` parce que `/` donne
> toujours un nombre à virgule (voir [les nombres](nombres.md)).
>
> Essayez : ajoutez `-40` à la liste. Vous obtenez `-40 °C = -40.0 °F` : c'est la seule
> température identique dans les deux unités.

### Exercice 3 : Le mot le plus long

Niveau : ★☆☆

Dans la phrase `"le chat mange une souris"`, trouvez le mot le plus long, avec une
boucle, sans utiliser `max`.

```text
souris
```

> **Indice :** `phrase.split()` découpe la phrase en liste de mots. Gardez dans
> `plus_long` le plus long mot vu jusqu'ici ; au départ, un texte vide `""`.

> **Solution**
>
> ```python
> phrase = "le chat mange une souris"
> plus_long = ""
> for mot in phrase.split():
>     if len(mot) > len(plus_long):
>         plus_long = mot
> print(plus_long)
> ```
>
> ```sortie
> souris
> ```
>
> Pourquoi ça marche : c'est un concours. Chaque mot affronte le champion actuel ; s'il
> est plus long, il prend sa place.
>
> | `mot` | `len(mot)` | `len(plus_long)` avant | Plus long ? | `plus_long` après |
> |---|---|---|---|---|
> | le | 2 | 0 | oui | le |
> | chat | 4 | 2 | oui | chat |
> | mange | 5 | 4 | oui | mange |
> | une | 3 | 5 | non | mange |
> | souris | 6 | 5 | oui | souris |
>
> Essayez : remplacez `souris` par `rat`. Le programme affiche `mange`. En cas
> d'égalité, le premier mot garde sa place, car `>` est strict.

### Exercice 4 : Retourner une liste

Niveau : ★☆☆

Partez de la liste `["a", "b", "c", "d"]`. Construisez une **nouvelle** liste à l'envers,
avec une boucle, sans utiliser `reversed` ni `[::-1]`.

```text
['d', 'c', 'b', 'a']
```

> **Indice :** `liste.insert(0, x)` ajoute `x` **au début** de la liste, devant tous les
> autres (voir [les collections](collections.md)).

> **Solution**
>
> ```python
> lettres = ["a", "b", "c", "d"]
> envers = []
> for lettre in lettres:
>     envers.insert(0, lettre)
> print(envers)
> ```
>
> ```sortie
> ['d', 'c', 'b', 'a']
> ```
>
> Pourquoi ça marche : pensez à une pile d'assiettes. Chaque nouvelle assiette se pose
> **devant** les autres ; la dernière posée se retrouve en premier.
>
> | `lettre` | `envers` après |
> |---|---|
> | a | `['a']` |
> | b | `['b', 'a']` |
> | c | `['c', 'b', 'a']` |
> | d | `['d', 'c', 'b', 'a']` |

## Niveau ★★☆ : combiner plusieurs idées

> **À retenir :** à ce niveau, découpez le problème. Écrivez d'abord une petite fonction
> qui fait une seule chose, testez-la, puis assemblez.

### Exercice 5 : L'année bissextile

Niveau : ★★☆

Une année est bissextile (elle a un 29 février) si elle est divisible par 4, **sauf**
si elle est divisible par 100, **sauf** si elle est divisible par 400. Écrivez
`est_bissextile(annee)` et testez 2024, 2023, 1900 et 2000.

```text
2024 True
2023 False
1900 False
2000 True
```

> **Indice :** `annee % 4 == 0` teste « divisible par 4 ». Commencez par le cas le plus
> rare : 400, puis 100, puis 4.

> **Solution**
>
> ```python
> def est_bissextile(annee):
>     if annee % 400 == 0:
>         return True
>     if annee % 100 == 0:
>         return False
>     return annee % 4 == 0
>
> for annee in [2024, 2023, 1900, 2000]:
>     print(annee, est_bissextile(annee))
> ```
>
> ```sortie
> 2024 True
> 2023 False
> 1900 False
> 2000 True
> ```
>
> Pourquoi ça marche : le premier `return` atteint arrête la fonction. On teste donc
> l'exception la plus forte en premier.
>
> | `annee` | `% 400 == 0` ? | `% 100 == 0` ? | `% 4 == 0` ? | Résultat |
> |---|---|---|---|---|
> | 2024 | non | non | oui | True |
> | 2023 | non | non | non | False |
> | 1900 | non | oui | (pas testé) | False |
> | 2000 | oui | (pas testé) | (pas testé) | True |

### Exercice 6 : Le distributeur de billets

Niveau : ★★☆

Un distributeur a des billets de 50, 20 et 10 €. Il donne toujours le moins de billets
possible : d'abord les plus gros. Écrivez `distribuer(montant)` et testez avec 180 €.

```text
3 x 50 €
1 x 20 €
1 x 10 €
```

> **Indice :** pour chaque billet, `montant // billet` donne combien on peut en donner,
> et `montant % billet` ce qui reste à payer.

> **Solution**
>
> ```python
> def distribuer(montant):
>     for billet in [50, 20, 10]:
>         nombre = montant // billet
>         montant = montant % billet
>         if nombre > 0:
>             print(f"{nombre} x {billet} €")
>
> distribuer(180)
> ```
>
> ```sortie
> 3 x 50 €
> 1 x 20 €
> 1 x 10 €
> ```
>
> Pourquoi ça marche : à chaque tour, on prend le plus possible du billet en cours,
> puis on continue avec le reste.
>
> | `billet` | `montant` avant | `nombre` | `montant` après |
> |---|---|---|---|
> | 50 | 180 | 3 | 30 |
> | 20 | 30 | 1 | 10 |
> | 10 | 10 | 1 | 0 |
>
> ```text
> 180 €  →  [50][50][50]  reste 30  →  [20]  reste 10  →  [10]  reste 0
> ```
>
> Essayez : `distribuer(90)`. Il n'y a pas de ligne pour un billet donné 0 fois, grâce
> au `if nombre > 0`.

### Exercice 7 : Enlever les doublons

Niveau : ★★☆

Écrivez `sans_doublons(elements)` qui renvoie une liste sans doublons, **en gardant
l'ordre** de la première apparition. Testez avec `[3, 1, 3, 2, 1, 4]` et
`["rouge", "vert", "rouge"]`.

```text
[3, 1, 2, 4]
['rouge', 'vert']
```

> **Indice :** gardez un [ensemble](../debuter/glossaire.md#ensemble) `deja_vus` des
> éléments déjà rencontrés. Un ensemble répond très vite à la question `x in deja_vus`.

> **Solution**
>
> ```python
> def sans_doublons(elements):
>     deja_vus = set()
>     resultat = []
>     for e in elements:
>         if e not in deja_vus:
>             deja_vus.add(e)
>             resultat.append(e)
>     return resultat
>
> print(sans_doublons([3, 1, 3, 2, 1, 4]))
> print(sans_doublons(["rouge", "vert", "rouge"]))
> ```
>
> ```sortie
> [3, 1, 2, 4]
> ['rouge', 'vert']
> ```
>
> Pourquoi ça marche : c'est le videur d'une soirée avec sa liste d'invités déjà entrés.
> Quelqu'un qui est déjà entré ne rentre pas une deuxième fois.
>
> | `e` | Déjà vu ? | `resultat` après |
> |---|---|---|
> | 3 | non | `[3]` |
> | 1 | non | `[3, 1]` |
> | 3 | oui | `[3, 1]` |
> | 2 | non | `[3, 1, 2]` |
> | 1 | oui | `[3, 1, 2]` |
> | 4 | non | `[3, 1, 2, 4]` |
>
> `list(set(elements))` enlève aussi les doublons, mais ne garantit pas l'ordre.

### Exercice 8 : Le code de César

Niveau : ★★☆

Jules César cachait ses messages en décalant chaque lettre de 3 dans l'alphabet : `a`
devient `d`, `b` devient `e`… et `z` revient à `c`. Écrivez `cesar(texte, decalage)`
pour les lettres minuscules ; les autres caractères restent tels quels. Codez
`"bonjour zoe"`, puis décodez le résultat.

```text
erqmrxu crh
bonjour zoe
```

> **Indice :** `alphabet.index(lettre)` donne la position d'une lettre (0 pour `a`).
> Pour revenir au début après `z`, prenez le reste : `(position + decalage) % 26`.

> **Solution**
>
> ```python
> def cesar(texte, decalage):
>     alphabet = "abcdefghijklmnopqrstuvwxyz"
>     resultat = ""
>     for lettre in texte:
>         if lettre in alphabet:
>             position = alphabet.index(lettre)
>             resultat += alphabet[(position + decalage) % 26]
>         else:
>             resultat += lettre
>     return resultat
>
> secret = cesar("bonjour zoe", 3)
> print(secret)
> print(cesar(secret, -3))
> ```
>
> ```sortie
> erqmrxu crh
> bonjour zoe
> ```
>
> Pourquoi ça marche : l'alphabet est comme le cadran d'une horloge. Après la position
> 25 (`z`), on revient à 0 (`a`). Pour `z` : `(25 + 3) % 26` vaut `2`, c'est `c`. Pour
> décoder, on décale dans l'autre sens, de `-3`.
>
> ```text
> a b c d e f ... w x y z
> 0 1 2 3 4 5 ... 22 23 24 25  →  après 25, on revient à 0
> ```

### Exercice 9 : Ranger les mots par initiale

Niveau : ★★☆

Rangez les mots `["pomme", "poire", "banane", "kiwi", "brocoli", "prune"]` dans un
dictionnaire, selon leur première lettre. Affichez ensuite chaque groupe.

```text
p : pomme, poire, prune
b : banane, brocoli
k : kiwi
```

> **Indice :** la clé est `mot[0]`, la valeur est une liste. Si la clé n'existe pas
> encore, créez d'abord une liste vide (voir [les dictionnaires](dictionnaires.md)).

> **Solution**
>
> ```python
> mots = ["pomme", "poire", "banane", "kiwi", "brocoli", "prune"]
> groupes = {}
> for mot in mots:
>     initiale = mot[0]
>     if initiale not in groupes:
>         groupes[initiale] = []
>     groupes[initiale].append(mot)
>
> for initiale, liste in groupes.items():
>     print(f"{initiale} : {', '.join(liste)}")
> ```
>
> ```sortie
> p : pomme, poire, prune
> b : banane, brocoli
> k : kiwi
> ```
>
> Pourquoi ça marche : comme un classeur avec des intercalaires A, B, C… On crée un
> intercalaire la première fois qu'on en a besoin, puis on range chaque mot derrière.
>
> `", ".join(liste)` colle les mots de la liste avec une virgule et un espace entre
> eux.

## Niveau ★★★ : pour aller plus loin

> **À retenir :** pour un exercice difficile, faites le tableau de suivi **avant**
> d'écrire le code, sur un petit exemple. Le code vient ensuite tout seul.

### Exercice 10 : Chercher dans une liste triée

Niveau : ★★★

Pour trouver un mot dans un dictionnaire papier, vous ne lisez pas toutes les pages :
vous l'ouvrez au milieu, puis vous gardez la bonne moitié. Écrivez
`chercher(liste, cible)` qui fait pareil sur une liste **triée**, et renvoie la position
de `cible`, ou `None`. Affichez chaque étape.

```text
bas=0 haut=7 milieu=3 -> 19
bas=4 haut=7 milieu=5 -> 31
position : 5
bas=0 haut=7 milieu=3 -> 19
bas=0 haut=2 milieu=1 -> 8
bas=0 haut=0 milieu=0 -> 3
position : None
```

> **Indice :** gardez deux bornes, `bas` et `haut`, qui encadrent la zone où la cible
> peut encore être. Le milieu est `(bas + haut) // 2`. Bouclez tant que `bas <= haut`.

> **Solution**
>
> ```python
> def chercher(liste, cible):
>     bas = 0
>     haut = len(liste) - 1
>     while bas <= haut:
>         milieu = (bas + haut) // 2
>         print(f"bas={bas} haut={haut} milieu={milieu} -> {liste[milieu]}")
>         if liste[milieu] == cible:
>             return milieu
>         if liste[milieu] < cible:
>             bas = milieu + 1
>         else:
>             haut = milieu - 1
>     return None
>
> nombres = [3, 8, 12, 19, 25, 31, 40, 47]
> print("position :", chercher(nombres, 31))
> print("position :", chercher(nombres, 5))
> ```
>
> ```sortie
> bas=0 haut=7 milieu=3 -> 19
> bas=4 haut=7 milieu=5 -> 31
> position : 5
> bas=0 haut=7 milieu=3 -> 19
> bas=0 haut=2 milieu=1 -> 8
> bas=0 haut=0 milieu=0 -> 3
> position : None
> ```
>
> Pourquoi ça marche : à chaque tour, la zone de recherche est coupée en deux. Pour
> chercher 5 :
>
> | Tour | `bas` | `haut` | `milieu` | `liste[milieu]` | Décision |
> |---|---|---|---|---|---|
> | 1 | 0 | 7 | 3 | 19 | 19 > 5 : `haut = 2` |
> | 2 | 0 | 2 | 1 | 8 | 8 > 5 : `haut = 0` |
> | 3 | 0 | 0 | 0 | 3 | 3 < 5 : `bas = 1` |
> | — | 1 | 0 | | | `1 <= 0` faux : fin, `None` |
>
> ```text
> [3, 8, 12, 19, 25, 31, 40, 47]
>  ^           ^              ^
> bas        milieu          haut     tour 1 : 19 trop grand, on garde la gauche
> ```
>
> Avec 1 000 nombres, il faut au plus 10 tours, au lieu de 1 000 en lisant tout.

### Exercice 11 : Compresser un texte

Niveau : ★★★

Un texte comme `"aaabccdddd"` contient des lettres répétées. Compressez-le en écrivant
chaque lettre suivie du nombre de répétitions : `"a3b1c2d4"`. Écrivez `compresser` et
`decompresser`, qui fait l'inverse. On suppose qu'une lettre se répète au plus 9 fois.

```text
a3b1c2d4
aaabccdddd
```

> **Indice :** gardez la lettre `courant` et son `compte`. Quand la lettre change,
> écrivez `courant` et `compte` dans le résultat, puis repartez à 1. N'oubliez pas le
> dernier groupe après la boucle.

> **Indice :** pour décompresser, lisez le code deux caractères à la fois :
> `range(0, len(code), 2)`. `"d" * 4` donne `"dddd"`.

> **Solution**
>
> ```python
> def compresser(texte):
>     if texte == "":
>         return ""
>     resultat = ""
>     courant = texte[0]
>     compte = 0
>     for lettre in texte:
>         if lettre == courant:
>             compte += 1
>         else:
>             resultat += f"{courant}{compte}"
>             courant = lettre
>             compte = 1
>     resultat += f"{courant}{compte}"
>     return resultat
>
> def decompresser(code):
>     resultat = ""
>     for i in range(0, len(code), 2):
>         resultat += code[i] * int(code[i + 1])
>     return resultat
>
> c = compresser("aaabccdddd")
> print(c)
> print(decompresser(c))
> ```
>
> ```sortie
> a3b1c2d4
> aaabccdddd
> ```
>
> Pourquoi ça marche : on compte les lettres identiques qui se suivent, comme on compte
> des wagons de la même couleur dans un train.
>
> | `lettre` | Même que `courant` ? | `courant` | `compte` | `resultat` |
> |---|---|---|---|---|
> | a | oui | a | 1 | `""` |
> | a | oui | a | 2 | `""` |
> | a | oui | a | 3 | `""` |
> | b | non | b | 1 | `"a3"` |
> | c | non | c | 1 | `"a3b1"` |
> | c | oui | c | 2 | `"a3b1"` |
> | d | non | d | 1 | `"a3b1c2"` |
> | d, d, d | oui | d | 4 | `"a3b1c2"` |
> | après la boucle | | | | `"a3b1c2d4"` |
>
> La dernière ligne du tableau montre pourquoi il faut la ligne après la boucle : sans
> elle, le groupe `d4` serait perdu.

### Exercice 12 : Les parenthèses équilibrées

Niveau : ★★★

Écrivez `equilibre(texte)` qui dit si les signes `()`, `[]` et `{}` d'un texte sont
bien fermés, dans le bon ordre. Testez quatre textes.

```text
(a[b]{c})    True
(]           False
((x)         False
f(x) + [y]   True
```

> **Indice :** utilisez une liste comme une **pile** d'assiettes : `append` pose une
> ouverture sur le dessus, `pop()` reprend celle du dessus.

> **Indice :** pour une fermeture, la pile doit être non vide et son dessus doit être
> l'ouverture correspondante. À la fin, la pile doit être vide.

> **Solution**
>
> ```python
> def equilibre(texte):
>     paires = {")": "(", "]": "[", "}": "{"}
>     pile = []
>     for signe in texte:
>         if signe in "([{":
>             pile.append(signe)
>         elif signe in paires:
>             if not pile or pile.pop() != paires[signe]:
>                 return False
>     return pile == []
>
> for essai in ["(a[b]{c})", "(]", "((x)", "f(x) + [y]"]:
>     print(f"{essai:<12} {equilibre(essai)}")
> ```
>
> ```sortie
> (a[b]{c})    True
> (]           False
> ((x)         False
> f(x) + [y]   True
> ```
>
> Pourquoi ça marche : la dernière parenthèse ouverte doit être la première fermée,
> comme des boîtes rangées les unes dans les autres. Pour `"(a[b]{c})"` :
>
> | `signe` | Action | `pile` après |
> |---|---|---|
> | ( | on pose | `['(']` |
> | a | rien | `['(']` |
> | [ | on pose | `['(', '[']` |
> | b | rien | `['(', '[']` |
> | ] | on reprend `[` : ça correspond | `['(']` |
> | { | on pose | `['(', '{']` |
> | c | rien | `['(', '{']` |
> | } | on reprend `{` : ça correspond | `['(']` |
> | ) | on reprend `(` : ça correspond | `[]` |
>
> La pile est vide à la fin : `True`. Pour `"((x)"`, il reste un `(` dans la pile, donc
> `False`.

## En résumé

- Les exercices 1 à 4 reprennent les boucles, les fonctions et les listes.
- Les exercices 5 à 9 combinent conditions, divisions entières, ensembles, texte et
  dictionnaires.
- Les exercices 10 à 12 demandent de faire un tableau de suivi avant d'écrire le code.
- Quand une solution vous surprend, recopiez son tableau à la main : c'est là qu'on
  comprend.

Suite : gardez [l'aide-mémoire](aide-memoire.md) sous la main.
