# Les expressions régulières

Cette page explique comment décrire un motif de texte avec une expression régulière, pour chercher, extraire, vérifier ou remplacer des morceaux de texte.

> **Objectifs :**
>
> - vous saurez écrire une expression régulière simple et tester un texte avec `match?` ;
> - vous saurez extraire des morceaux avec les groupes, nommés ou non ;
> - vous saurez trouver toutes les occurrences avec `scan` ;
> - vous saurez remplacer avec `sub` et `gsub` ;
> - vous saurez lire les pièces les plus courantes : `\d`, `\w`, `+`, `*`, `^`, `$`…

## Qu'est-ce qu'une expression régulière ?

Une **expression régulière** (en anglais *regular expression*, souvent abrégé *regex*)
décrit une **forme** de texte plutôt qu'un texte précis. « Trois chiffres, un tiret,
deux chiffres » est une forme : `123-45` la respecte, `abc` non.

C'est comme un avis de recherche : il ne donne pas le nom de la personne, mais sa
description. Ruby parcourt le texte et cherche ce qui correspond.

En Ruby, une regex s'écrit entre deux barres obliques : `/chat/`. C'est un objet de la
classe `Regexp`.

```ruby
motif = /chat/
puts motif.class
puts "Le chat dort".match?(motif)
puts "Le chien dort".match?(motif)
```

```sortie
Regexp
true
false
```

`match?` répond seulement `true` ou `false`. C'est la méthode la plus simple et la
plus rapide.

## Les pièces de base

Une regex se construit avec des pièces. Une lettre ordinaire se représente elle-même.
Certains caractères ont un sens spécial.

| Pièce | Signification | Exemple qui correspond |
|-------|---------------|------------------------|
| `\d` | un chiffre | `7` |
| `\w` | une lettre, un chiffre ou `_` | `a`, `Z`, `4` |
| `\s` | un espace (ou tabulation, retour à la ligne) | ` ` |
| `.` | n'importe quel caractère (sauf retour à la ligne) | `x`, `!` |
| `[aeiou]` | un caractère de la liste | `e` |
| `[a-z]` | un caractère de l'intervalle | `m` |
| `[^0-9]` | un caractère **hors** de la liste | `k` |
| `\.` | un vrai point (le `\` annule le sens spécial) | `.` |

Après une pièce, un **quantificateur** dit combien de fois elle se répète.

| Quantificateur | Combien de fois |
|----------------|-----------------|
| `*` | 0 fois ou plus |
| `+` | 1 fois ou plus |
| `?` | 0 ou 1 fois (facultatif) |
| `{3}` | exactement 3 fois |
| `{2,4}` | de 2 à 4 fois |

```ruby
puts "Code 4521".match?(/\d{4}/)
puts "Code 45".match?(/\d{4}/)
puts "couleur".match?(/colou?r/)
puts "color".match?(/colou?r/)
puts "a.b".match?(/a\.b/)
puts "axb".match?(/a\.b/)
```

```sortie
true
false
false
true
true
false
```

`/colou?r/` accepte `color` et `colour` : le `u` est facultatif. `couleur` n'y est pas,
car il faut `colo` suivi de `r` ou de `ur`.

> **Note :** `\w` ne connaît que les lettres sans accent. Pour toutes les lettres,
> accents compris, utilisez `\p{L}` ou `[[:alpha:]]`.

## Les ancres

Les ancres ne correspondent à aucun caractère : elles indiquent une **position**.

| Ancre | Position |
|-------|----------|
| `^` | début d'une ligne |
| `$` | fin d'une ligne |
| `\A` | début du texte entier |
| `\z` | fin du texte entier |
| `\b` | limite d'un mot (entre une lettre et un espace, par exemple) |

```ruby
puts "2024-03-01".match?(/\A\d{4}-\d{2}-\d{2}\z/)
puts "le 2024-03-01".match?(/\A\d{4}-\d{2}-\d{2}\z/)
puts "Un chat noir".match?(/\bchat\b/)
puts "Un chaton".match?(/\bchat\b/)
```

```sortie
true
false
true
false
```

Sans ancres, la regex cherche **n'importe où** dans le texte. Avec `\A` et `\z`, elle
doit couvrir le texte en entier : c'est ce qu'il faut pour **vérifier** une saisie.

## Trouver la position avec =~

`=~` renvoie la position du début de la première correspondance, ou `nil` si rien ne
correspond. Comme `nil` est faux, on l'utilise souvent dans un `if`.

```ruby
p "Bonjour 2024" =~ /\d+/
p "Bonjour" =~ /\d+/

if "Commande n°58" =~ /\d+/
  puts "Il y a un nombre."
end
```

```sortie
8
nil
Il y a un nombre.
```

## Extraire avec match et les groupes

Des parenthèses forment un **groupe** : elles capturent le morceau trouvé. `match`
renvoie un objet `MatchData` (ou `nil`). `[0]` est la correspondance entière, `[1]` le
premier groupe, `[2]` le deuxième…

```ruby
m = "Rendez-vous à 14h30".match(/(\d+)h(\d+)/)

puts m[0]
puts m[1]
puts m[2]
p m.captures
p m.pre_match
```

```sortie
14h30
14
30
["14", "30"]
"Rendez-vous à "
```

`captures` donne tous les groupes d'un coup ; `pre_match` ce qui précède.

### Les groupes nommés

`(?<nom>...)` donne un nom au groupe. Le code devient bien plus lisible.

```ruby
date = /(?<jour>\d{2})\/(?<mois>\d{2})\/(?<annee>\d{4})/

m = "Né le 14/07/1990 à Lyon".match(date)
puts m[:annee]
puts m[:mois]
p m.named_captures
```

```sortie
1990
07
{"jour"=>"14", "mois"=>"07", "annee"=>"1990"}
```

Le `/` fait partie du motif : il faut l'écrire `\/` pour ne pas fermer la regex.

## Toutes les occurrences avec scan

`match` s'arrête à la première correspondance. `scan` les renvoie **toutes** dans un
tableau. Avec des groupes, chaque élément est un tableau de captures.

```ruby
texte = "Pommes : 3, poires : 12, kiwis : 7"

p texte.scan(/\d+/)
p texte.scan(/\d+/).map(&:to_i).sum
p texte.scan(/(\w+) : (\d+)/)
```

```sortie
["3", "12", "7"]
22
[["Pommes", "3"], ["poires", "12"], ["kiwis", "7"]]
```

## Remplacer avec sub et gsub

`sub` remplace la **première** correspondance, `gsub` (« global ») les remplace
**toutes**. Dans le texte de remplacement, `\1` reprend le premier groupe, `\k<nom>`
un groupe nommé.

```ruby
puts "a  b    c".gsub(/\s+/, " ")
puts "2024-03-01".sub(/(\d+)-(\d+)-(\d+)/, '\3/\2/\1')
puts "Tél : 06 12 34 56 78".gsub(/\d/, "*")
puts "un deux trois".sub(/\w+/, "UN")
```

```sortie
a b c
01/03/2024
Tél : ** ** ** ** **
UN deux trois
```

> **Attention :** écrivez `'\1'` entre apostrophes. Entre guillemets doubles, il
> faudrait doubler la barre : `"\\1"`.

### Remplacer avec un bloc

Avec un bloc, le remplacement est **calculé** pour chaque correspondance. Le bloc
reçoit le morceau trouvé.

```ruby
puts "il a 3 chats et 2 chiens".gsub(/\d+/) { |n| (n.to_i * 10).to_s }
puts "bonjour le monde".gsub(/\b\w/) { |lettre| lettre.upcase }
```

```sortie
il a 30 chats et 20 chiens
Bonjour Le Monde
```

## Les options i et x

Une lettre après la dernière barre change le comportement de la regex :

- `i` ignore la différence entre majuscules et minuscules ;
- `x` permet d'ajouter des espaces, des retours à la ligne et des commentaires `#` ;
  pratique pour expliquer une regex longue.

```ruby
puts "BONJOUR".match?(/bonjour/i)

telephone = /
  \A
  0\d          # l'indicatif : 0 puis un chiffre
  (\s?\d{2}){4} # quatre paires de chiffres, espaces permis
  \z
/x

puts "06 12 34 56 78".match?(telephone)
puts "0612345678".match?(telephone)
puts "06 12 34".match?(telephone)
```

```sortie
true
true
true
false
```

## Vérifier une adresse e-mail (simplement)

Une adresse e-mail parfaite est très difficile à décrire. Une vérification simple
suffit souvent : « quelque chose, un `@`, quelque chose, un point, au moins deux
lettres », sans espace.

```ruby
EMAIL = /\A[^@\s]+@[^@\s]+\.[a-z]{2,}\z/i

["alice@exemple.fr", "bob.martin@mail.co.uk", "pas-un-email", "a@b", "x y@z.com"].each do |adresse|
  puts "#{adresse.ljust(22)} #{adresse.match?(EMAIL) ? "valide" : "refusée"}"
end
```

```sortie
alice@exemple.fr       valide
bob.martin@mail.co.uk  valide
pas-un-email           refusée
a@b                    refusée
x y@z.com              refusée
```

`[^@\s]+` veut dire : un ou plusieurs caractères qui ne sont ni un `@` ni un espace.

> **Astuce :** pour être vraiment sûr qu'une adresse existe, la seule méthode est d'y
> envoyer un message de confirmation.

## Aide-mémoire

| Écriture | Rôle |
|----------|------|
| `texte.match?(/re/)` | `true` ou `false` |
| `texte =~ /re/` | position ou `nil` |
| `texte.match(/re/)` | `MatchData` (groupes : `m[1]`, `m[:nom]`) ou `nil` |
| `texte.scan(/re/)` | tableau de toutes les correspondances |
| `texte.sub(/re/, "x")` | remplace la première |
| `texte.gsub(/re/, "x")` | remplace toutes ; accepte un bloc |
| `texte.split(/re/)` | découpe le texte à chaque correspondance |
| `/re/i`, `/re/x` | ignorer la casse, regex commentée |

## Erreurs fréquentes

### Utiliser le résultat de match sans vérifier

```ruby norun
m = "pas de date".match(/(\d+)-(\d+)/)
puts m[1]
```

```text
main.rb:2:in `<main>': undefined method `[]' for nil (NoMethodError)

puts m[1]
      ^^^
```

Quand rien ne correspond, `match` renvoie `nil`. Vérifiez d'abord : `puts m[1] if m`,
ou `m&.[](1)`.

### Une parenthèse non fermée

```ruby norun
re = /(\d+/
```

```text
main.rb: 
main.rb:1: end pattern with unmatched parenthesis: /(\d+/ (SyntaxError)
```

Chaque `(` doit avoir sa `)`. Pour chercher une vraie parenthèse, échappez-la : `\(`.

### Vérifier avec ^ et $ au lieu de \A et \z

```ruby norun
saisie = "42\nrm -rf /"
puts saisie.match?(/^\d+$/)
```

```text
true
```

Pas d'erreur, mais une vérification percée : `^` et `$` regardent **chaque ligne**, et
la première ligne `42` suffit. Pour vérifier tout le texte, utilisez `\A\d+\z`.

## Testez-vous

**Question 1 :** que renvoie `"abc123def".scan(/\d/)` ?

> **Réponse**
>
> `["1", "2", "3"]` : chaque chiffre est une correspondance séparée. Avec `/\d+/`, on
> obtiendrait `["123"]`.

**Question 2 :** quelle est la différence entre `sub` et `gsub` ?

> **Réponse**
>
> `sub` remplace seulement la première correspondance, `gsub` les remplace toutes.

**Question 3 :** extrayez l'heure et les minutes de `"Départ à 08:45"` avec des groupes
nommés.

> **Réponse**
>
> ```ruby
> m = "Départ à 08:45".match(/(?<h>\d{2}):(?<min>\d{2})/)
> puts "#{m[:h]} heures et #{m[:min]} minutes"
> ```
>
> ```sortie
> 08 heures et 45 minutes
> ```

## En résumé

- Une regex `/.../` décrit une forme de texte ; `match?` dit si le texte la contient.
- `\d`, `\w`, `\s`, `.` et `[...]` décrivent un caractère ; `*`, `+`, `?`, `{n}` le répètent.
- `\A` et `\z` ancrent au début et à la fin du texte : indispensables pour valider.
- Les groupes `( )` et `(?<nom> )` capturent des morceaux, lus avec `match`.
- `scan` trouve tout ; `sub` et `gsub` remplacent, avec un texte ou un bloc.

Suite : [dates et heures](dates.md)
