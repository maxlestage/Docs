# Les pièges du débutant en Ruby

Cette page rassemble les 12 pièges qui bloquent le plus souvent les débutants en Ruby, avec pour chacun l'explication et la bonne façon de faire.

> **Objectifs :**
>
> - vous reconnaîtrez les 12 pièges les plus courants de Ruby ;
> - vous saurez pourquoi ils surprennent ;
> - vous saurez les corriger, ou mieux, les éviter.

## Comment lire cette page

Chaque piège suit le même chemin :

1. ce que vous **attendez** ;
2. le code qui **surprend** ;
3. **pourquoi** Ruby fait ça ;
4. la **correction**.

Pas besoin de tout lire d'un coup. Revenez ici quand un programme fait « n'importe
quoi » : la cause est souvent dans cette liste.

## Les pièges de l'affichage et des calculs

### 1. puts, print et p ne montrent pas la même chose

**Vous attendez :** que les trois affichent la même chose.

**Ce qui se passe :**

```ruby
puts "1"
p "1"
puts 1
p 1
puts nil
p nil
print "a"
print "b"
puts
```

```sortie
1
"1"
1
1

nil
ab
```

**Pourquoi :** chaque [méthode](../debuter/glossaire.md#methode) a son rôle.

| Méthode | Ce qu'elle fait | Pour qui ? |
|---|---|---|
| `puts` | affiche le texte, puis va à la ligne | l'utilisateur |
| `print` | affiche le texte, **sans** aller à la ligne | l'utilisateur |
| `p` | affiche la valeur **telle que Ruby la voit** | vous, pour chercher un bug |

Avec `puts`, le texte `"1"` et le nombre `1` se ressemblent. `nil` devient même une
ligne vide ! `p` montre les guillemets et écrit `nil` : vous voyez la vérité.

**La correction :** pour chercher un bug, utilisez `p`, pas `puts`.

> **Essayez :** affichez `p [1, "1", nil]` puis `puts [1, "1", nil]`. `puts` écrit un
> élément par ligne, et la ligne de `nil` est vide.

### 2. 7 / 2 donne 3

**Vous attendez :** `3.5`.

**Ce qui se passe :**

```ruby
puts 7 / 2
puts 7.0 / 2
puts 7.fdiv(2)
puts 7 % 2
```

```sortie
3
3.5
3.5
1
```

**Pourquoi :** `7` et `2` sont des [entiers](../debuter/glossaire.md#entier). Entre deux
entiers, `/` fait une **division entière** : il garde le quotient et jette le reste.
C'est comme partager 7 bonbons entre 2 enfants : chacun en a 3, et il en reste 1
(c'est ce que donne `%`).

**La correction :** mettez un [flottant](../debuter/glossaire.md#flottant) dans le calcul
(`7.0`) ou utilisez `fdiv`.

```ruby
notes = [12, 15, 14]
moyenne = notes.sum / notes.size
puts moyenne
moyenne = notes.sum.fdiv(notes.size)
puts moyenne.round(2)
```

```sortie
13
13.67
```

> **À retenir :** entier / entier = entier. Pour une moyenne, utilisez `fdiv` ou `to_f`.

### 3. Un texte n'est pas un nombre

**Vous attendez :** que `"5" + 5` donne `10`.

**Ce qui se passe :**

```ruby norun
puts "5" + 5
```

```text
main.rb:1:in `+': no implicit conversion of Integer into String (TypeError)
	from main.rb:1:in `<main>'
```

Et quand il n'y a pas d'erreur, le résultat surprend aussi :

```ruby
puts "5" + "5"
puts "5" * 3
```

```sortie
55
555
```

**Pourquoi :** `"5"` est une [chaîne de caractères](../debuter/glossaire.md#chaine-de-caracteres),
pas un nombre. Pour un texte, `+` **colle** et `*` **répète**. Ruby ne transforme
jamais un texte en nombre tout seul.

**La correction :** convertissez vous-même.

```ruby
puts "5".to_i + 5
puts "5" + 5.to_s
puts "Total : #{5 + 5}"
puts "abc".to_i
```

```sortie
10
55
Total : 10
0
```

| Écriture | Résultat | Ce qui s'est passé |
|---|---|---|
| `"5".to_i + 5` | `10` | texte → nombre, puis addition |
| `"5" + 5.to_s` | `"55"` | nombre → texte, puis collage |
| `"abc".to_i` | `0` | pas de chiffre au début : `0`, sans erreur ! |

> **Attention :** `to_i` ne se plaint jamais. Si le texte n'est pas un nombre, vous
> obtenez `0`. Pour avoir une erreur à la place, utilisez `Integer("abc")`.

## Les pièges des conditions

### 4. nil arrive de partout

**Vous attendez :** qu'une valeur absente donne une erreur claire, ou `0`.

**Ce qui se passe :**

```ruby norun
stock = { "pommes" => 3 }
puts stock["poires"] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock["poires"] + 1
                     ^
```

**Pourquoi :** `nil` veut dire « rien » (voir [null](../debuter/glossaire.md#null)). En
Ruby, beaucoup de choses renvoient `nil` **sans erreur** :

```ruby
stock = { "pommes" => 3 }
p stock["poires"]
p [1, 2, 3][10]
p "bonjour".index("z")
p [].first
```

```sortie
nil
nil
nil
nil
```

L'erreur n'arrive que plus loin, quand vous **utilisez** ce `nil`. Le message
`undefined method ... for nil` veut dire : « vous avez demandé quelque chose à `nil` ».

```text
stock["poires"]      →  nil       (pas d'erreur ici)
nil + 1              →  NoMethodError  (l'erreur arrive ici)
```

**La correction :** cherchez d'où vient le `nil`, puis donnez une valeur de secours.

```ruby
stock = { "pommes" => 3 }
puts stock.fetch("poires", 0) + 1
puts (stock["poires"] || 0) + 1
nom = nil
p nom&.upcase
```

```sortie
1
1
nil
```

`&.` veut dire : « appelle la méthode seulement si ce n'est pas `nil` ».

> **À retenir :** `undefined method ... for nil` : remontez le programme pour trouver
> la ligne qui a produit `nil`.

### 5. Écrire = au lieu de == dans un if

**Vous attendez :** que ce programme n'affiche rien, car `x` vaut 5.

**Ce qui se passe :**

```ruby
x = 5
if x = 3
  puts "x vaut trois"
end
puts x
```

```sortie
x vaut trois
3
```

**Pourquoi :** `=` **range** une valeur, `==` **compare**. `x = 3` met 3 dans `x`, et
vaut `3`. Or `3` compte comme vrai (voir le piège suivant). Le `if` passe donc, et `x`
est changé ! Ruby affiche seulement un avertissement :
`warning: found '= literal' in conditional, should be ==`.

**La correction :** `if x == 3`.

> **Astuce :** lisez `=` « prend la valeur » et `==` « est-il égal à ». Ça évite la
> confusion.

### 6. 0 et le texte vide sont vrais

**Vous attendez :** que `0`, `""` ou `[]` comptent comme « faux », comme dans
d'autres langages.

**Ce qui se passe :**

```ruby
[0, "", [], "false", nil, false].each do |valeur|
  if valeur
    puts "#{valeur.inspect} est vrai"
  else
    puts "#{valeur.inspect} est faux"
  end
end
```

```sortie
0 est vrai
"" est vrai
[] est vrai
"false" est vrai
nil est faux
false est faux
```

**Pourquoi :** en Ruby, il y a **seulement deux valeurs fausses** : `false` et `nil`.
Tout le reste est vrai, même `0`, même le texte `"false"`.

**La correction :** posez la vraie question.

```ruby
nombre = 0
texte = ""
puts "zéro" if nombre.zero?
puts "texte vide" if texte.empty?
```

```sortie
zéro
texte vide
```

> **À retenir :** seuls `false` et `nil` sont faux. Pour tester « vide » ou « zéro »,
> utilisez `empty?` ou `zero?`.

### 7. and et or ne sont pas && et ||

**Vous attendez :** que `ok = true and false` mette `false` dans `ok`.

**Ce qui se passe :**

```ruby
ok = true and false
p ok
ok = true && false
p ok
```

```sortie
true
false
```

**Pourquoi :** `and` est **très faible** : Ruby fait d'abord `ok = true`, puis
`... and false`. Avec des parenthèses, on voit ce que Ruby comprend :

```text
ok = true and false    →   (ok = true) and false     →  ok vaut true
ok = true && false     →   ok = (true && false)      →  ok vaut false
```

**La correction :** dans les conditions et les calculs, utilisez toujours `&&`, `||`
et `!`.

## Les pièges des intervalles et des boucles

### 8. Deux points ou trois points

**Vous attendez :** que `1...5` contienne 5.

**Ce qui se passe :**

```ruby
p (1..5).to_a
p (1...5).to_a
lettres = ["a", "b", "c"]
p (0...lettres.size).to_a
```

```sortie
[1, 2, 3, 4, 5]
[1, 2, 3, 4]
[0, 1, 2]
```

**Pourquoi :** `..` **inclut** la fin, `...` l'**exclut**. Le troisième point
« pousse » la fin dehors. `0...taille` est pratique pour les positions d'un tableau :
elles vont de 0 à taille − 1.

**La correction :** relisez votre [intervalle](../debuter/glossaire.md#intervalle) en
vous demandant : « la dernière valeur est-elle dedans ? ». Tout est détaillé dans
[les intervalles](intervalles.md).

### 9. Une variable créée dans un bloc disparaît

**Vous attendez :** retrouver `dernier` après la boucle.

**Ce qui se passe :**

```ruby norun
[1, 2, 3].each { |n| dernier = n }
puts dernier
```

```text
main.rb:2:in `<main>': undefined local variable or method `dernier' for main (NameError)

puts dernier
     ^^^^^^^
```

**Pourquoi :** une [variable](../debuter/glossaire.md#variable) **créée** dans un bloc
`{ ... }` ou `do ... end` n'existe que dedans (c'est sa
[portée](../debuter/glossaire.md#portee)). À la sortie du bloc, elle est jetée.

**La correction :** créez la variable **avant** le bloc. Le bloc pourra la changer.

```ruby
dernier = nil
total = 0
[1, 2, 3].each do |n|
  dernier = n
  total += n
end
puts dernier
puts total
```

```sortie
3
6
```

| Étape | n | dernier | total |
|---|---|---|---|
| avant | – | nil | 0 |
| tour 1 | 1 | 1 | 1 |
| tour 2 | 2 | 2 | 3 |
| tour 3 | 3 | 3 | 6 |

## Les pièges du partage

Ces trois pièges ont la même cause : **deux noms pour un seul objet**. Imaginez une
boîte avec deux étiquettes. Si vous changez le contenu par une étiquette, l'autre voit
le changement.

```text
  a ──┐
      ├──►  [ "chat" ]     une seule boîte
  b ──┘
```

### 10. Modifier un texte change aussi l'autre variable

**Vous attendez :** que `a` reste `"chat"`.

**Ce qui se passe :**

```ruby
a = "chat"
b = a
b << "s"
puts a
puts b
```

```sortie
chats
chats
```

**Pourquoi :** `b = a` ne copie pas le texte. Il colle une deuxième étiquette sur la
même boîte. `<<` **modifie** la boîte : `a` et `b` voient tous les deux `"chats"`.

**La correction :** `+` crée un **nouveau** texte, et laisse l'ancien tranquille.

```ruby
a = "chat"
b = a
b += "s"
puts a
puts b
```

```sortie
chat
chats
```

| Écriture | Que fait-elle ? |
|---|---|
| `b << "s"` | modifie la boîte (les autres noms voient le changement) |
| `b += "s"` | crée une nouvelle boîte pour `b` (les autres noms ne changent pas) |
| `b = a.dup` | fait une vraie copie |

C'est la même chose pour les méthodes en `!` : `upcase` renvoie un nouveau texte,
`upcase!` modifie le texte lui-même.

```ruby
mot = "bonjour"
mot.upcase
puts mot
mot.upcase!
puts mot
```

```sortie
bonjour
BONJOUR
```

> **Essayez :** remplacez `mot.upcase` par `mot = mot.upcase`. La première ligne affiche
> alors `BONJOUR`.

### 11. Array.new(3, []) partage le même tableau

**Vous attendez :** une grille de 3 lignes, et un `"x"` dans la première seulement.

**Ce qui se passe :**

```ruby
grille = Array.new(3, [])
grille[0] << "x"
p grille
```

```sortie
[["x"], ["x"], ["x"]]
```

**Pourquoi :** Ruby crée **un seul** tableau vide et le met 3 fois. Les 3 cases sont
3 étiquettes sur la même boîte.

```text
grille[0] ──┐
grille[1] ──┼──►  [ ]     un seul tableau !
grille[2] ──┘
```

**La correction :** avec un bloc, Ruby crée un nouveau tableau pour chaque case.

```ruby
grille = Array.new(3) { [] }
grille[0] << "x"
p grille
```

```sortie
[["x"], [], []]
```

### 12. Hash.new([]) partage la même valeur par défaut

**Vous attendez :** que `h[:a] << 1` range `[1]` sous la clé `:a`.

**Ce qui se passe :**

```ruby
h = Hash.new([])
h[:a] << 1
p h
p h[:b]
```

```sortie
{}
[1]
```

**Pourquoi :** la valeur par défaut est **un seul** tableau, partagé par toutes les clés
absentes. `h[:a] << 1` ajoute 1 dans ce tableau partagé, sans créer la clé `:a`. Le
[hash](../debuter/glossaire.md#dictionnaire) reste vide, et `h[:b]` voit le `1` !

**La correction :** le bloc crée un nouveau tableau pour chaque clé, et le range.

```ruby
h = Hash.new { |hash, cle| hash[cle] = [] }
h[:a] << 1
p h
p h[:b]
p h
```

```sortie
{:a=>[1]}
[]
{:a=>[1], :b=>[]}
```

> **À retenir :** `Array.new(n, valeur)` et `Hash.new(valeur)` partagent **un seul**
> objet. Pour un tableau ou un texte, passez un bloc `{ ... }`.

## Erreurs fréquentes

### Modifier un texte gelé

```ruby norun
# frozen_string_literal: true
nom = "Alice"
nom << " Martin"
```

```text
main.rb:3:in `<main>': can't modify frozen String: "Alice" (FrozenError)
```

Le commentaire magique `# frozen_string_literal: true` (fréquent en haut des fichiers)
**gèle** tous les textes écrits entre guillemets : ils deviennent
[immuables](../debuter/glossaire.md#immuable). Utilisez `+` : `nom = nom + " Martin"`,
ou partez d'une copie : `nom = +"Alice"` (un texte non gelé). Voir
[les textes gelés](texte.md#les-textes-geles).

### Appeler une méthode sur une case qui n'existe pas

```ruby norun
fruits = ["pomme", "kiwi"]
puts fruits[2].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts fruits[2].upcase
              ^^^^^^^
```

Les positions vont de 0 à 1 : `fruits[2]` vaut `nil` (piège 4). Vérifiez la position,
ou utilisez `fruits.last` pour le dernier élément.

## Testez-vous

**Question 1 :** qu'affiche ce programme ?

```ruby norun
puts 10 / 4
puts 10 / 4.0
```

> **Réponse**
>
> `2` puis `2.5`. Entre deux entiers, `/` garde seulement la partie entière.
>
> ```ruby
> puts 10 / 4
> puts 10 / 4.0
> ```
>
> ```sortie
> 2
> 2.5
> ```

**Question 2 :** ce programme affiche-t-il « rien » ?

```ruby norun
reponse = 0
puts "rien" unless reponse
```

> **Réponse**
>
> Non. `0` est vrai en Ruby, donc `unless reponse` ne passe pas. Pour tester zéro,
> écrivez `reponse.zero?`.
>
> ```ruby
> reponse = 0
> puts "rien" unless reponse
> puts "zéro" if reponse.zero?
> ```
>
> ```sortie
> zéro
> ```

**Question 3 :** comment créer un tableau de 2 tableaux vides **indépendants** ?

> **Réponse**
>
> Avec un bloc : chaque case reçoit son propre tableau.
>
> ```ruby
> paires = Array.new(2) { [] }
> paires[1] << "b"
> p paires
> ```
>
> ```sortie
> [[], ["b"]]
> ```

## En résumé

- `p` montre la vraie valeur (guillemets, `nil`) : utilisez-le pour chercher un bug.
- Entier / entier = entier ; un texte ne devient jamais un nombre tout seul.
- `nil` arrive sans bruit ; seuls `false` et `nil` sont faux, `0` est vrai.
- `=` range, `==` compare ; préférez `&&` et `||` à `and` et `or`.
- `..` inclut la fin, `...` l'exclut ; une variable créée dans un bloc y reste.
- `<<`, les méthodes en `!`, `Array.new(n, [])` et `Hash.new([])` partagent un objet.

Suite : [Exercices pour bien démarrer](exercices-faciles.md)
