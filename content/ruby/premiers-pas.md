# Premiers pas

Cette page présente les bases de Ruby : variables, nombres, textes, symboles, booléens, `nil` et affichage.

> **Objectifs :**
>
> - vous saurez ranger une valeur dans une variable et la changer ;
> - vous saurez calculer avec des entiers et des nombres à virgule ;
> - vous saurez écrire un texte et y insérer une valeur ;
> - vous saurez ce que sont `true`, `false` et `nil` ;
> - vous saurez afficher avec `puts`, `print` et `p`.

## Les variables

Un programme a besoin de se souvenir de choses : un prénom, un score, un prix. Sans
variable, chaque valeur serait perdue aussitôt, comme un numéro de téléphone qu'on
oublie faute de l'avoir noté.

Une [variable](../debuter/glossaire.md#variable) est un nom qui désigne une
[valeur](../debuter/glossaire.md#valeur). Pensez à une étiquette collée sur une boîte :
l'étiquette est le nom, le contenu de la boîte est la valeur. Vous la créez avec `=`.

```ruby
age = 30
prenom = "Alice"
puts prenom
puts age
```

```sortie
Alice
30
```

Ligne par ligne :

- `age = 30` range le nombre 30 dans une boîte nommée `age` ;
- `prenom = "Alice"` range le texte `"Alice"` dans une autre boîte, nommée `prenom` ;
- les deux `puts` lisent le contenu de chaque boîte et l'affichent, dans l'ordre où
  ils sont écrits : d'abord `prenom`, puis `age`.

Ici, `=` ne veut pas dire « est égal à ». Il veut dire « range cette valeur sous ce
nom ». On parle d'**affectation**.

Vous n'avez pas besoin d'indiquer le [type](../debuter/glossaire.md#type) (nombre,
texte…) : Ruby le devine. `class` vous dit ce qu'il a deviné.

```ruby
puts 30.class
puts "Alice".class
puts 2.5.class
```

```sortie
Integer
String
Float
```

### Changer la valeur d'une variable

Une variable peut changer de valeur. La nouvelle valeur remplace l'ancienne.

```ruby
score = 10
score = score + 5
puts score
score += 1
puts score
score -= 6
puts score
```

```sortie
15
16
10
```

`score += 1` est un raccourci pour `score = score + 1`. Il existe aussi `-=`, `*=` et
`/=`.

**À vous :** écrivez un programme qui range `5` dans une variable `pommes`, affiche sa
valeur, lui ajoute 3 avec `+=`, puis affiche la nouvelle valeur.

> **Solution**
>
> ```ruby
> pommes = 5
> puts pommes
> pommes += 3
> puts pommes
> ```
>
> ```sortie
> 5
> 8
> ```

## Les nombres

Presque tous les programmes calculent : le total d'un panier, une moyenne de notes, le
temps qui reste avant un rendez-vous. Ruby calcule comme une calculatrice, avec
quelques règles à connaître.

Ruby distingue les entiers (`Integer`, sans virgule) et les nombres à virgule (`Float`,
appelés **flottants**). En Ruby, la virgule s'écrit avec un point : `2.5`.

### Les opérateurs

Un **opérateur** est un signe qui fait un calcul.

| Opérateur | Sens | Exemple | Résultat |
|-----------|------|---------|----------|
| `+` | addition | `7 + 3` | `10` |
| `-` | soustraction | `7 - 3` | `4` |
| `*` | multiplication | `7 * 3` | `21` |
| `/` | division | `7 / 2` | `3` |
| `%` | reste de la division | `7 % 2` | `1` |
| `**` | puissance | `2 ** 10` | `1024` |

```ruby
puts 7 + 3
puts 7 / 2
puts 7.0 / 2
puts 7 % 2
puts 2 ** 10
```

```sortie
10
3
3.5
1
1024
```

Ligne par ligne :

- `puts 7 + 3` : Ruby fait d'abord le calcul, puis `puts` affiche le résultat, 10 ;
- `puts 7 / 2` affiche 3 et non 3.5 : les deux nombres sont des entiers (la section
  suivante explique pourquoi) ;
- `puts 7.0 / 2` : avec un flottant, le résultat garde sa partie après la virgule ;
- `7 % 2` vaut 1, car 7 = 2 x 3 + 1 : il reste 1 ;
- `2 ** 10` multiplie 2 par lui-même 10 fois : 1024.

### Division entière et division à virgule

> **Attention :** `7 / 2` donne `3`. Entre deux entiers, la division garde seulement la
> partie entière. Écrivez `7.0 / 2` pour obtenir `3.5`.

Il suffit qu'**un seul** des deux nombres soit un flottant. Si vos deux nombres sont
dans des variables, utilisez `to_f` (*to float*, « vers flottant ») ou `fdiv`.

```ruby
total = 7
parts = 2
puts total / parts
puts total.to_f / parts
puts total.fdiv(parts)
```

```sortie
3
3.5
3.5
```

`%` (le **modulo**) donne le reste. Il sert souvent à savoir si un nombre est pair :
`n % 2` vaut `0` pour un nombre pair.

### L'ordre des calculs

Comme en mathématiques, `*`, `/` et `%` passent avant `+` et `-`. `**` passe avant
tout. C'est la **priorité** des opérateurs. Les parenthèses changent l'ordre.

```ruby
puts 2 + 3 * 4
puts (2 + 3) * 4
puts 2 * 3 ** 2
puts 10 - 4 - 3
```

```sortie
14
20
18
3
```

> **Astuce :** en cas de doute, ajoutez des parenthèses. Elles ne coûtent rien et
> rendent le calcul clair.

### Arrondir

```ruby
puts 3.7.round
puts 3.14159.round(2)
puts 3.7.floor
puts 3.2.ceil
puts -5.abs
```

```sortie
4
3.14
3
4
5
```

- `round` arrondit au plus proche ; `round(2)` garde 2 chiffres après la virgule.
- `floor` arrondit vers le bas, `ceil` vers le haut.
- `abs` donne la valeur sans le signe (la **valeur absolue**).

**À vous :** 17 bonbons sont partagés entre 5 enfants. Écrivez un programme qui range
ces deux nombres dans des variables, puis affiche combien de bonbons reçoit chaque
enfant, et combien il en reste.

> **Solution**
>
> ```ruby
> bonbons = 17
> enfants = 5
> puts bonbons / enfants
> puts bonbons % enfants
> ```
>
> ```sortie
> 3
> 2
> ```

## Les chaînes de caractères

Un programme parle avec des mots : il affiche un prénom, un message d'accueil, une
adresse. Tout ce qui est du texte se range dans une chaîne.

Un texte s'écrit entre guillemets. On parle de
[chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres), ou de **chaîne**
(`String`) : les lettres sont enfilées les unes après les autres, comme des perles.

```ruby
nom = "Ruby"
puts nom.length
puts nom.upcase
puts nom + " est sympa"
```

```sortie
4
RUBY
Ruby est sympa
```

- `length` donne le nombre de caractères.
- `upcase` met en majuscules.
- `+` colle deux textes bout à bout.

### L'interpolation

Entre guillemets doubles, `#{...}` insère une valeur dans le texte.

```ruby
prenom = "Alice"
age = 30
puts "#{prenom} a #{age} ans."
puts "Dans 5 ans : #{age + 5}"
```

```sortie
Alice a 30 ans.
Dans 5 ans : 35
```

> **Attention :** l'interpolation ne marche pas avec des guillemets simples :
> `'#{age}'` affiche le texte tel quel.

### Passer d'un texte à un nombre

Un texte qui contient des chiffres n'est pas un nombre : `"5"` n'est pas `5`. Pour
passer de l'un à l'autre :

- `to_i` (*to integer*) transforme en entier ;
- `to_f` transforme en flottant ;
- `to_s` (*to string*) transforme en texte.

```ruby
puts "5" + "5"
puts "5".to_i + 5
puts 5.to_s + "5"
```

```sortie
55
10
55
```

Le chapitre [travailler avec du texte](texte.md) va beaucoup plus loin.

**À vous :** écrivez un programme qui range un prix de `4` euros et une quantité de `3`
dans deux variables, puis affiche `3 croissants coûtent 12 euros.` grâce à
l'interpolation.

> **Solution**
>
> ```ruby
> prix = 4
> quantite = 3
> puts "#{quantite} croissants coûtent #{prix * quantite} euros."
> ```
>
> ```sortie
> 3 croissants coûtent 12 euros.
> ```

## Les symboles

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un symbole commence par `:`. C'est un nom fixe, souvent utilisé comme étiquette.
Contrairement à un texte, on ne le modifie pas : il sert juste à nommer quelque chose.

```ruby
couleur = :rouge
puts couleur
p couleur
```

```sortie
rouge
:rouge
```

Ligne par ligne :

- `couleur = :rouge` range le symbole `:rouge` dans la variable `couleur` ;
- `puts couleur` l'affiche comme un mot, sans les deux-points ;
- `p couleur` le montre tel que Ruby le voit, avec ses deux-points : c'est bien un
  symbole, pas un texte.

Vous les retrouverez comme clés de hash dans [les tableaux et hashs](collections.md).

## Les booléens et nil

Un programme doit souvent répondre par oui ou par non : la personne est-elle majeure ?
Le mot de passe est-il le bon ? Les booléens sont ces réponses. Et parfois, il n'y a
encore rien du tout à ranger : c'est le rôle de `nil`.

`true` (vrai) et `false` (faux) sont les [booléens](../debuter/glossaire.md#booleen).
`nil` veut dire « rien, aucune valeur » : c'est une boîte vide.

### Comparer deux valeurs

Une comparaison répond par `true` ou `false`.

| Opérateur | Sens |
|-----------|------|
| `==` | égal à |
| `!=` | différent de |
| `<`, `>` | plus petit, plus grand |
| `<=`, `>=` | plus petit ou égal, plus grand ou égal |

```ruby
puts 3 > 2
puts 3 == 4
puts 3 != 4
puts "a" == "a"
p nil
puts nil.nil?
```

```sortie
true
false
true
true
nil
true
```

Ligne par ligne :

- `3 > 2` pose la question « 3 est-il plus grand que 2 ? ». Oui : Ruby répond `true` ;
- `3 == 4` répond `false`, et `3 != 4` répond `true` : 3 et 4 sont bien différents ;
- `"a" == "a"` montre qu'on peut aussi comparer des textes ;
- `p nil` affiche `nil`, et `nil.nil?` demande « est-ce `nil` ? » : la réponse est
  `true`.

> **Attention :** `=` range une valeur, `==` compare deux valeurs. Ne les confondez pas.

> **À retenir :** en Ruby, seuls `false` et `nil` sont « faux ». Tout le reste est vrai,
> même `0` et `""`.

## Afficher avec puts, print et p

Un programme travaille en silence. Pour voir un résultat, vous devez le lui demander,
comme on demande à quelqu'un de lire un résultat à voix haute. Ruby vous donne trois
façons de le faire.

- `puts` affiche puis passe à la ligne.
- `print` affiche sans passer à la ligne.
- `p` affiche la valeur telle que Ruby la voit (pratique pour déboguer).

```ruby
print "a"
print "b"
puts
puts "texte"
p "texte"
p 42
```

```sortie
ab
texte
"texte"
42
```

Ligne par ligne :

- `print "a"` affiche `a` et reste sur la même ligne ; `print "b"` écrit `b` juste à
  côté ;
- `puts` tout seul n'affiche rien : il passe seulement à la ligne ;
- `puts "texte"` affiche le texte sans guillemets, `p "texte"` le montre avec ;
- `p 42` affiche `42` sans guillemets : c'est un nombre, pas un texte.

`p` montre les guillemets : vous voyez tout de suite que `"42"` est un texte et `42`
un nombre.

```ruby
p "42"
p 42
puts "42"
puts 42
```

```sortie
"42"
42
42
42
```

**À vous :** écrivez un programme qui affiche `Bonjour, Alice` sur une seule ligne, avec
un `print` pour `"Bonjour, "` puis un `puts` pour `"Alice"`. Affichez ensuite le
nombre `7` puis le texte `"7"` avec `p`.

> **Solution**
>
> ```ruby
> print "Bonjour, "
> puts "Alice"
> p 7
> p "7"
> ```
>
> ```sortie
> Bonjour, Alice
> 7
> "7"
> ```

## Les commentaires

Un commentaire commence par `#`. Ruby l'ignore. Il sert à expliquer le code aux
humains.

```ruby
# Ceci est un commentaire
puts "visible" # un commentaire en fin de ligne
```

```sortie
visible
```

## Les conventions de nommage

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Un programme se relit bien plus souvent qu'il ne s'écrit. Un bon nom est comme une
étiquette lisible sur un bocal : on sait ce qu'il contient sans l'ouvrir.

- Variables et méthodes : `snake_case` (`prix_total`), en minuscules avec des `_`.
- Classes et modules : `CamelCase` (`CompteBancaire`).
- Constantes : en majuscules (`TAUX_TVA`).

Une **constante** est une valeur qui ne doit pas changer. Son nom commence par une
majuscule.

```ruby
TAUX_TVA = 0.2
prix_ht = 100
prix_ttc = prix_ht * (1 + TAUX_TVA)
puts prix_ttc
```

```sortie
120.0
```

Ligne par ligne :

- `TAUX_TVA = 0.2` crée une constante : son nom en majuscules dit « ne pas changer » ;
- `prix_ht` vaut 100 ;
- Ruby calcule d'abord la parenthèse, `1 + 0.2`, soit `1.2`, puis `100 * 1.2`. Il range
  `120.0` dans `prix_ttc` ;
- `puts` affiche cette valeur.

Le résultat est `120.0` et non `120` : un calcul qui mélange un entier et un flottant
donne un flottant.

> **Astuce :** suivez ces conventions : tout le monde les utilise, et votre code sera
> plus facile à lire.

## Erreurs fréquentes

### Additionner un texte et un nombre

```ruby norun
puts "Âge : " + 30
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)
	from main.rb:1:in `<main>'
```

Ruby refuse de coller un nombre à un texte. Transformez le nombre, ou utilisez
l'interpolation :

```ruby
puts "Âge : " + 30.to_s
puts "Âge : #{30}"
```

```sortie
Âge : 30
Âge : 30
```

### Une faute dans le nom d'une variable

```ruby norun
prenom = "Alice"
puts prnom
```

```text
main.rb:2:in `<main>': undefined local variable or method `prnom' for main (NameError)

puts prnom
     ^^^^^
Did you mean?  prenom
```

`prnom` n'existe pas. Ruby propose le nom le plus proche. Corrigez en `puts prenom`.

### Diviser par zéro

```ruby norun
puts 10 / 0
```

```text
main.rb:1:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:1:in `<main>'
```

Une division entière par zéro est impossible. Vérifiez le diviseur avant de calculer.
Avec un flottant, Ruby ne plante pas mais donne `Infinity` (l'infini) :

```ruby
puts 10.0 / 0
```

```sortie
Infinity
```

## Testez-vous

**Question 1 :** que vaut `10 / 4` ? Et `10 / 4.0` ?

> **Réponse**
>
> `10 / 4` vaut `2` : entre deux entiers, la division garde la partie entière.
> `10 / 4.0` vaut `2.5`.

**Question 2 :** qu'affiche ce code ?

```ruby norun
x = 2 + 3 * 2
puts x
```

> **Réponse**
>
> `8` : la multiplication passe avant l'addition. Écrivez `(2 + 3) * 2` pour obtenir
> `10`.

**Question 3 :** `0` est-il vrai ou faux dans une condition Ruby ?

> **Réponse**
>
> Vrai. Seuls `false` et `nil` sont faux.

## En résumé

- Une variable se crée avec `=` ; Ruby devine le type tout seul.
- `*` et `/` passent avant `+` et `-` ; les parenthèses changent l'ordre.
- `7 / 2` vaut `3` entre entiers ; écrivez `7.0 / 2` pour obtenir `3.5`.
- Un texte s'écrit entre guillemets ; `#{...}` y insère une valeur.
- Seuls `false` et `nil` sont faux ; tout le reste est vrai.
- `puts` affiche, `print` affiche sans retour à la ligne, `p` montre la valeur exacte.

Suite : [Nombres et calculs](nombres.md)
