# Exercices corrigés : deuxième série

Cette page propose 12 nouveaux exercices corrigés en Ruby, du plus facile au plus difficile, avec des tableaux qui suivent les variables pas à pas.

## Comment utiliser cette page

Ces exercices sont **différents** de ceux des
[exercices pour bien démarrer](exercices-faciles.md) et des
[exercices corrigés](exercices.md). Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Pour chaque exercice :

1. Lisez l'énoncé et la sortie attendue.
2. Cherchez sur papier, avec un tableau de suivi (voir
   [suivre un programme pas à pas](pas-a-pas.md)).
3. Écrivez votre programme dans un fichier `.rb` et lancez-le.
4. Bloqué ? Lisez l'indice. Ouvrez la solution en dernier.

> **Astuce :** si votre programme affiche un résultat faux, ajoutez un `puts` avec des
> étiquettes dans la boucle. Vous verrez à quel tour les choses dérapent.

## Échauffement

### Exercice 1 : Celsius en Fahrenheit

Niveau : ★☆☆

Pour chaque température du tableau `[0, 20, 37, 100]` (en degrés Celsius), affichez sa
valeur en degrés Fahrenheit. La formule : `f = c * 9 / 5 + 32`.

Sortie attendue :

```text
0 °C = 32.0 °F
20 °C = 68.0 °F
37 °C = 98.6 °F
100 °C = 212.0 °F
```

> **Indice :** `37 * 9 / 5` donne un nombre entier (66), sans virgule. Écrivez `9.0`
> au lieu de `9` pour obtenir un nombre à virgule.

> **Solution**
>
> ```ruby
> [0, 20, 37, 100].each do |c|
>   f = c * 9.0 / 5 + 32
>   puts "#{c} °C = #{f} °F"
> end
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 20 °C = 68.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```
>
> Pourquoi ça marche : dès qu'un des nombres est à virgule (`9.0`), tout le calcul se
> fait à virgule. Avec `9`, Ruby ferait une division entière et afficherait `98` pour
> 37 °C (voir [les nombres](nombres.md)).

### Exercice 2 : Le plus petit, à la main

Niveau : ★☆☆

Trouvez le plus petit nombre du tableau `[8, 3, 11, 5]` **sans** utiliser `min`.
Utilisez une boucle.

Sortie attendue :

```text
Le plus petit : 3
```

> **Indice :** gardez le « record » dans une variable `plus_petit`. Commencez avec le
> premier élément, `nombres[0]`.

> **Indice :** à chaque tour, si le nombre est plus petit que le record, il devient le
> nouveau record.

> **Solution**
>
> ```ruby
> nombres = [8, 3, 11, 5]
> plus_petit = nombres[0]
> nombres.each do |n|
>   if n < plus_petit
>     plus_petit = n
>   end
> end
> puts "Le plus petit : #{plus_petit}"
> ```
>
> ```sortie
> Le plus petit : 3
> ```
>
> Pourquoi ça marche : comme un champion qui garde sa coupe tant que personne ne fait
> mieux.
>
> | Tour | n | `n < plus_petit` ? | plus_petit |
> |---|---|---|---|
> | avant | – | | 8 |
> | 1 | 8 | `8 < 8` non | 8 |
> | 2 | 3 | `3 < 8` oui | 3 |
> | 3 | 11 | `11 < 3` non | 3 |
> | 4 | 5 | `5 < 3` non | 3 |
>
> On part de `nombres[0]` et pas de `0` : avec `0`, aucun nombre positif ne battrait le
> record, et le programme afficherait 0.

### Exercice 3 : L'escalier

Niveau : ★☆☆

Affichez un escalier de 4 marches avec des étoiles.

Sortie attendue :

```text
*
**
***
****
```

> **Indice :** en Ruby, `"*" * 3` donne `"***"` : on peut multiplier un texte par un
> nombre.

> **Solution**
>
> ```ruby
> (1..4).each do |marche|
>   puts "*" * marche
> end
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Pourquoi ça marche : à chaque tour, `marche` grandit de 1, donc la ligne a une étoile
> de plus.
>
> | Tour | marche | `"*" * marche` |
> |---|---|---|
> | 1 | 1 | `"*"` |
> | 2 | 2 | `"**"` |
> | 3 | 3 | `"***"` |
> | 4 | 4 | `"****"` |

### Exercice 4 : Des secondes en heures

Niveau : ★☆☆

Un film dure `3725` secondes. Affichez sa durée en heures, minutes et secondes.

Sortie attendue :

```text
3725 s = 1 h 2 min 5 s
```

> **Indice :** une heure fait 3600 secondes. `/` donne le nombre d'heures entières, `%`
> donne ce qui reste.

> **Indice :** faites pareil avec le reste et 60 pour les minutes.

> **Solution**
>
> ```ruby
> total = 3725
> heures = total / 3600
> reste = total % 3600
> minutes = reste / 60
> secondes = reste % 60
> puts "#{total} s = #{heures} h #{minutes} min #{secondes} s"
> ```
>
> ```sortie
> 3725 s = 1 h 2 min 5 s
> ```
>
> Pourquoi ça marche : c'est comme rendre la monnaie avec de gros billets d'abord.
>
> | Ligne | Calcul | heures | reste | minutes | secondes |
> |---|---|---|---|---|---|
> | 2 | `3725 / 3600` | 1 | – | – | – |
> | 3 | `3725 % 3600` | 1 | 125 | – | – |
> | 4 | `125 / 60` | 1 | 125 | 2 | – |
> | 5 | `125 % 60` | 1 | 125 | 2 | 5 |

## Textes et collections

### Exercice 5 : Retourner un mot à la main

Niveau : ★★☆

Écrivez une méthode `retourner(mot)` qui renvoie le mot écrit à l'envers, **sans**
utiliser `reverse`. Testez-la avec `"Ruby"`.

Sortie attendue :

```text
ybuR
```

> **Indice :** partez d'un texte vide. Parcourez les lettres avec `mot.chars.each`.
> Ajoutez chaque lettre **devant** le résultat, pas derrière.

> **Solution**
>
> ```ruby
> def retourner(mot)
>   resultat = ""
>   mot.chars.each do |lettre|
>     resultat = lettre + resultat
>   end
>   resultat
> end
>
> puts retourner("Ruby")
> ```
>
> ```sortie
> ybuR
> ```
>
> Pourquoi ça marche : comme une pile d'assiettes, la dernière lettre posée se retrouve
> en haut, donc en premier.
>
> | Tour | lettre | `lettre + resultat` | resultat |
> |---|---|---|---|
> | avant | – | | `""` |
> | 1 | `"R"` | `"R" + ""` | `"R"` |
> | 2 | `"u"` | `"u" + "R"` | `"uR"` |
> | 3 | `"b"` | `"b" + "uR"` | `"buR"` |
> | 4 | `"y"` | `"y" + "buR"` | `"ybuR"` |

### Exercice 6 : Le mot le plus long

Niveau : ★★☆

Dans la phrase `"le petit chaton dort sur le canapé"`, trouvez le mot le plus long et
affichez-le avec sa longueur.

Sortie attendue :

```text
chaton (6 lettres)
```

> **Indice :** `split` coupe la phrase en mots. `max_by { |mot| mot.length }` garde
> l'élément pour lequel le bloc donne le plus grand nombre.

> **Solution**
>
> ```ruby
> phrase = "le petit chaton dort sur le canapé"
> long = phrase.split.max_by { |mot| mot.length }
> puts "#{long} (#{long.length} lettres)"
> ```
>
> ```sortie
> chaton (6 lettres)
> ```
>
> Pourquoi ça marche : `max_by` mesure chaque mot avec le bloc. `chaton` et `canapé`
> ont tous les deux 6 lettres : en cas d'égalité, `max_by` garde le **premier**.
> Voir [les blocs](blocs.md).

### Exercice 7 : Ranger par initiale

Niveau : ★★☆

Rangez les prénoms `["Alice", "Bob", "Anna", "Chloé", "Bruno"]` selon leur première
lettre. Affichez une ligne par lettre.

Sortie attendue :

```text
A : Alice, Anna
B : Bob, Bruno
C : Chloé
```

> **Indice :** `prenom[0]` donne la première lettre. Un hash peut associer à chaque
> lettre un tableau de prénoms.

> **Indice :** `group_by` fait ce travail en une fois : il range les éléments selon la
> valeur renvoyée par le bloc.

> **Solution**
>
> ```ruby
> prenoms = ["Alice", "Bob", "Anna", "Chloé", "Bruno"]
> groupes = prenoms.group_by { |prenom| prenom[0] }
> groupes.each do |lettre, liste|
>   puts "#{lettre} : #{liste.join(", ")}"
> end
> ```
>
> ```sortie
> A : Alice, Anna
> B : Bob, Bruno
> C : Chloé
> ```
>
> Pourquoi ça marche : `group_by` fait des tiroirs étiquetés. Ici, il construit le
> hash `{"A" => ["Alice", "Anna"], "B" => ["Bob", "Bruno"], "C" => ["Chloé"]}`. `join`
> recolle les prénoms avec une virgule.

### Exercice 8 : La somme des chiffres

Niveau : ★★☆

Écrivez une méthode `somme_chiffres(n)` qui additionne les chiffres d'un nombre, sans le
transformer en texte. Testez-la avec `2026`.

Sortie attendue :

```text
10
```

> **Indice :** `n % 10` donne le dernier chiffre (`2026 % 10` vaut 6). `n / 10` enlève
> ce dernier chiffre (`2026 / 10` vaut 202).

> **Indice :** répétez avec `while n > 0`.

> **Solution**
>
> ```ruby
> def somme_chiffres(n)
>   somme = 0
>   while n > 0
>     somme += n % 10
>     n = n / 10
>   end
>   somme
> end
>
> puts somme_chiffres(2026)
> ```
>
> ```sortie
> 10
> ```
>
> Pourquoi ça marche : on « mange » le nombre par la droite, un chiffre par tour.
>
> | Tour | Test `n > 0` | `n % 10` | somme | n |
> |---|---|---|---|---|
> | avant | | | 0 | 2026 |
> | 1 | oui | 6 | 6 | 202 |
> | 2 | oui | 2 | 8 | 20 |
> | 3 | oui | 0 | 8 | 2 |
> | 4 | oui | 2 | 10 | 0 |
> | fin | `0 > 0` non | | 10 | 0 |

### Exercice 9 : Vérifier un mot de passe

Niveau : ★★☆

Écrivez une méthode `problemes(mdp)` qui renvoie la liste des problèmes d'un mot de
passe :

- moins de 8 caractères : `"trop court"` ;
- pas de chiffre : `"pas de chiffre"` ;
- pas de majuscule : `"pas de majuscule"`.

Testez-la avec `"chat"`, `"chaton2026"` et `"Chaton2026"`.

Sortie attendue :

```text
chat : trop court, pas de chiffre, pas de majuscule
chaton2026 : pas de majuscule
Chaton2026 : OK
```

> **Indice :** partez d'un tableau vide `liste = []` et ajoutez un message avec `<<` pour
> chaque règle non respectée.

> **Indice :** `mdp.chars.any? { |c| ("0".."9").include?(c) }` répond `true` s'il y a
> au moins un chiffre. Pour une majuscule, utilisez `("A".."Z")`.

> **Solution**
>
> ```ruby
> def problemes(mdp)
>   liste = []
>   liste << "trop court" if mdp.length < 8
>   liste << "pas de chiffre" unless mdp.chars.any? { |c| ("0".."9").include?(c) }
>   liste << "pas de majuscule" unless mdp.chars.any? { |c| ("A".."Z").include?(c) }
>   liste
> end
>
> ["chat", "chaton2026", "Chaton2026"].each do |mdp|
>   liste = problemes(mdp)
>   if liste.empty?
>     puts "#{mdp} : OK"
>   else
>     puts "#{mdp} : #{liste.join(", ")}"
>   end
> end
> ```
>
> ```sortie
> chat : trop court, pas de chiffre, pas de majuscule
> chaton2026 : pas de majuscule
> Chaton2026 : OK
> ```
>
> Pourquoi ça marche : chaque règle est une ligne indépendante, comme une liste de
> contrôle avant un départ en voyage. Une règle non respectée ajoute son message. Un
> tableau vide à la fin veut dire « tout va bien ».

## Pour aller plus loin

### Exercice 10 : Le juste prix

Niveau : ★★★

L'ordinateur doit trouver un nombre secret entre 1 et 100. À chaque essai, il apprend
seulement « trop petit » ou « trop grand ». Écrivez un programme qui trouve `37` en
proposant toujours le **milieu** de l'intervalle encore possible.

Sortie attendue :

```text
Essai 1 : 50, trop grand
Essai 2 : 25, trop petit
Essai 3 : 37, trouvé !
```

> **Indice :** gardez deux variables, `bas = 1` et `haut = 100`. Le milieu est
> `(bas + haut) / 2`.

> **Indice :** si le milieu est trop grand, le secret est en dessous : `haut = milieu - 1`.
> S'il est trop petit : `bas = milieu + 1`.

> **Solution**
>
> ```ruby
> secret = 37
> bas = 1
> haut = 100
> essai = 0
> loop do
>   essai += 1
>   milieu = (bas + haut) / 2
>   if milieu == secret
>     puts "Essai #{essai} : #{milieu}, trouvé !"
>     break
>   elsif milieu > secret
>     puts "Essai #{essai} : #{milieu}, trop grand"
>     haut = milieu - 1
>   else
>     puts "Essai #{essai} : #{milieu}, trop petit"
>     bas = milieu + 1
>   end
> end
> ```
>
> ```sortie
> Essai 1 : 50, trop grand
> Essai 2 : 25, trop petit
> Essai 3 : 37, trouvé !
> ```
>
> Pourquoi ça marche : à chaque essai, on jette la moitié des nombres possibles, comme
> quand on cherche un mot dans un dictionnaire en l'ouvrant au milieu.
>
> | Essai | bas | haut | milieu | Réponse | Nouvelle limite |
> |---|---|---|---|---|---|
> | 1 | 1 | 100 | 50 | trop grand | haut = 49 |
> | 2 | 1 | 49 | 25 | trop petit | bas = 26 |
> | 3 | 26 | 49 | 37 | trouvé | |
>
> Avec cette méthode, 7 essais suffisent toujours pour un nombre entre 1 et 100.

### Exercice 11 : Les parenthèses bien fermées

Niveau : ★★★

Écrivez une méthode `equilibre?(texte)` qui vérifie que les parenthèses `()`, crochets
`[]` et accolades `{}` sont bien fermées, dans le bon ordre. Testez-la avec
`"(a[b]{c})"`, `"(a]"` et `"(("`.

Sortie attendue :

```text
(a[b]{c}) : true
(a] : false
(( : false
```

> **Indice :** utilisez un tableau comme une pile d'assiettes. Un signe ouvrant : on le
> pose dessus avec `push`. Un signe fermant : on retire celle du dessus avec `pop`, et
> elle doit correspondre.

> **Indice :** un hash `{ ")" => "(", "]" => "[", "}" => "{" }` donne l'ouvrant attendu
> pour chaque fermant. À la fin, la pile doit être vide.

> **Solution**
>
> ```ruby
> PAIRES = { ")" => "(", "]" => "[", "}" => "{" }
>
> def equilibre?(texte)
>   pile = []
>   texte.chars.each do |c|
>     if PAIRES.value?(c)
>       pile.push(c)
>     elsif PAIRES.key?(c)
>       return false if pile.pop != PAIRES[c]
>     end
>   end
>   pile.empty?
> end
>
> ["(a[b]{c})", "(a]", "(("].each do |texte|
>   puts "#{texte} : #{equilibre?(texte)}"
> end
> ```
>
> ```sortie
> (a[b]{c}) : true
> (a] : false
> (( : false
> ```
>
> Pourquoi ça marche : le dernier signe ouvert doit être le premier fermé. Les lettres
> sont ignorées. Voici le suivi pour `"(a[b]{c})"` :
>
> | c | Action | pile après |
> |---|---|---|
> | `(` | on pose | `["("]` |
> | `a` | rien | `["("]` |
> | `[` | on pose | `["(", "["]` |
> | `b` | rien | `["(", "["]` |
> | `]` | on retire `[`, il correspond | `["("]` |
> | `{` | on pose | `["(", "{"]` |
> | `c` | rien | `["(", "{"]` |
> | `}` | on retire `{`, il correspond | `["("]` |
> | `)` | on retire `(`, il correspond | `[]` |
>
> Pour `"(a]"`, le `]` retire `(` au lieu de `[` : `false`. Pour `"(("`, la pile n'est
> pas vide à la fin : `false`.

### Exercice 12 : Compresser un texte

Niveau : ★★★

Écrivez une méthode `compresser(texte)` qui remplace chaque suite de lettres
identiques par la lettre suivie du nombre de répétitions. Testez-la avec
`"aaabccdddd"`.

Sortie attendue :

```text
a3b1c2d4
```

> **Indice :** gardez la lettre en cours (`courante`) et son compteur (`compte`). Quand
> la lettre change, ajoutez `courante` et `compte` au résultat, puis repartez avec la
> nouvelle lettre.

> **Indice :** n'oubliez pas d'ajouter la **dernière** suite après la boucle.

> **Solution**
>
> ```ruby
> def compresser(texte)
>   resultat = ""
>   courante = texte[0]
>   compte = 0
>   texte.chars.each do |lettre|
>     if lettre == courante
>       compte += 1
>     else
>       resultat += "#{courante}#{compte}"
>       courante = lettre
>       compte = 1
>     end
>   end
>   resultat + "#{courante}#{compte}"
> end
>
> puts compresser("aaabccdddd")
> ```
>
> ```sortie
> a3b1c2d4
> ```
>
> Pourquoi ça marche : c'est comme compter des voitures de la même couleur qui passent
> à la file. Quand la couleur change, on note le total et on recommence à compter.
>
> | Tour | lettre | Même lettre ? | resultat | courante | compte |
> |---|---|---|---|---|---|
> | avant | – | | `""` | `"a"` | 0 |
> | 1 | a | oui | `""` | `"a"` | 1 |
> | 2 | a | oui | `""` | `"a"` | 2 |
> | 3 | a | oui | `""` | `"a"` | 3 |
> | 4 | b | non | `"a3"` | `"b"` | 1 |
> | 5 | c | non | `"a3b1"` | `"c"` | 1 |
> | 6 | c | oui | `"a3b1"` | `"c"` | 2 |
> | 7 | d | non | `"a3b1c2"` | `"d"` | 1 |
> | 8 à 10 | d | oui | `"a3b1c2"` | `"d"` | 4 |
>
> Après la boucle, la dernière suite (`d4`) n'est pas encore dans `resultat` : la
> dernière ligne de la méthode l'ajoute.

## En résumé

- Les exercices 1 à 4 travaillent les calculs, les boucles simples et le « record ».
- Les exercices 5 à 9 travaillent les textes, les tableaux, les hashs et les blocs.
- Les exercices 10 à 12 demandent de garder plusieurs variables en tête : faites le
  tableau de suivi sur papier.
- Quand une solution vous semble magique, suivez-la pas à pas, tour par tour.

Suite : [l'aide-mémoire Ruby](aide-memoire.md)
