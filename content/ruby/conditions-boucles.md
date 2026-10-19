# Les conditions

Cette page montre comment prendre des décisions avec `if`, `unless` et `case`, et comment combiner plusieurs conditions.

> **Objectifs :**
>
> - vous saurez exécuter du code seulement si une condition est vraie ;
> - vous saurez combiner des conditions avec `&&`, `||` et `!` ;
> - vous saurez choisir parmi plusieurs cas avec `case`.

## if, elsif et else

Une [condition](../debuter/glossaire.md#condition) est une question à laquelle on répond
par vrai ou faux : « la note est-elle supérieure à 10 ? ». C'est un carrefour : selon
la réponse, le programme prend un chemin ou un autre.

`if` exécute du code seulement si une condition est vraie. Le bloc se termine par `end`.

```ruby
note = 14
if note >= 16
  puts "Très bien"
elsif note >= 10
  puts "Admis"
else
  puts "Refusé"
end
```

```sortie
Admis
```

- `if` (« si ») pose la première question.
- `elsif` (« sinon, si ») pose une autre question, seulement si les précédentes ont
  répondu non. Attention à l'orthographe : `elsif`, sans `e` au milieu.
- `else` (« sinon ») s'exécute si aucune question n'a répondu oui.
- Ruby s'arrête à la **première** branche vraie.

Ligne par ligne :

- `note = 14` range 14 dans la variable `note` ;
- `if note >= 16` demande : « 14 est-il supérieur ou égal à 16 ? ». Non : Ruby saute
  ce bloc ;
- `elsif note >= 10` demande : « 14 est-il supérieur ou égal à 10 ? ». Oui : Ruby
  affiche « Admis » ;
- une branche a répondu oui : Ruby saute le `else` et continue après le `end`.

Les comparaisons (`==`, `!=`, `<`, `>`, `<=`, `>=`) sont présentées dans
[les premiers pas](premiers-pas.md).

**À vous :** écrivez un programme qui range `heure = 15` dans une variable, puis
affiche « Bonjour » si l'heure est avant 12, « Bon après-midi » si elle est avant 18,
et « Bonsoir » sinon.

> **Solution**
>
> ```ruby
> heure = 15
> if heure < 12
>   puts "Bonjour"
> elsif heure < 18
>   puts "Bon après-midi"
> else
>   puts "Bonsoir"
> end
> ```
>
> ```sortie
> Bon après-midi
> ```

### La forme courte

Pour une seule instruction, vous pouvez mettre `if` à la fin de la ligne.

```ruby
temperature = 35
puts "Il fait chaud !" if temperature > 30
```

```sortie
Il fait chaud !
```

### if renvoie une valeur

En Ruby, `if` donne une valeur : celle de la branche choisie. Vous pouvez la ranger
dans une variable.

```ruby
note = 8
resultat = if note >= 10
             "admis"
           else
             "refusé"
           end
puts resultat
```

```sortie
refusé
```

## unless

Parfois, la question se pose plus naturellement à l'envers : « s'il n'est **pas**
connecté, demandez-lui de se connecter ». `unless` permet de l'écrire comme on le dit.

`unless` (« à moins que ») est le contraire de `if` : le code s'exécute si la condition
est **fausse**.

```ruby
connecte = false
puts "Veuillez vous connecter." unless connecte
```

```sortie
Veuillez vous connecter.
```

> **Astuce :** évitez `unless` avec `else` : c'est difficile à lire. Utilisez plutôt `if`.

## Combiner des conditions

Une décision dépend souvent de plusieurs choses à la fois. Pour entrer gratuitement au
musée, il faut être jeune **et** membre. Ces trois signes relient plusieurs tests en un
seul.

`&&` veut dire « et », `||` veut dire « ou », `!` veut dire « non ».

- `a && b` est vrai si `a` **et** `b` sont vrais.
- `a || b` est vrai si au moins l'un des deux est vrai.
- `!a` est vrai si `a` est faux.

```ruby
age = 20
membre = true
puts "Entrée gratuite" if age < 26 && membre
puts "Pas membre" if !membre
puts "Tarif réduit" if age < 18 || age > 65
puts "Plein tarif" unless age < 18 || age > 65
```

```sortie
Entrée gratuite
Plein tarif
```

Ligne par ligne :

- `age` vaut 20 et `membre` vaut `true` ;
- ligne 3 : `age < 26` est vrai **et** `membre` est vrai : « Entrée gratuite » s'affiche ;
- ligne 4 : `!membre` transforme `true` en `false` : rien ne s'affiche ;
- ligne 5 : 20 n'est ni plus petit que 18, ni plus grand que 65 : le `||` est faux ;
- ligne 6 : la même condition est fausse, et `unless` agit justement quand c'est faux.

`&&` passe avant `||`, comme `*` passe avant `+`. Ajoutez des parenthèses pour être
clair :

```ruby
age = 70
etudiant = false
membre = false
puts "Réduction" if (age > 65 || etudiant) && !membre
```

```sortie
Réduction
```

> **Note :** Ruby connaît aussi `and`, `or` et `not`. Ils n'ont pas la même priorité
> que `&&` et `||` : pour débuter, utilisez seulement `&&`, `||` et `!`.

**À vous :** écrivez un programme qui range `age = 70` dans une variable, puis
affiche « Tarif réduit » si l'âge est inférieur à 18 ou supérieur ou égal à 65, et
« Plein tarif » sinon.

> **Solution**
>
> ```ruby
> age = 70
> if age < 18 || age >= 65
>   puts "Tarif réduit"
> else
>   puts "Plein tarif"
> end
> ```
>
> ```sortie
> Tarif réduit
> ```

## Le ternaire

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

`condition ? valeur_si_vrai : valeur_si_faux` choisit une valeur en une ligne.

```ruby
age = 15
statut = age >= 18 ? "majeur" : "mineur"
puts statut
```

```sortie
mineur
```

Lisez-le comme une question : « âge ≥ 18 ? si oui majeur, sinon mineur ».

## case et when

Pensez à un distributeur de boissons : selon le bouton choisi, il sert une boisson
différente. Quand une même valeur peut prendre beaucoup de formes (un jour, un choix
dans un menu), vous voulez une réaction pour chacune.

`case` compare une valeur à plusieurs possibilités. C'est plus lisible qu'une longue
suite de `elsif`.

```ruby
jour = "samedi"
case jour
when "samedi", "dimanche"
  puts "Week-end !"
when "lundi"
  puts "Courage..."
else
  puts "Jour de semaine"
end
```

```sortie
Week-end !
```

- Une virgule dans `when` veut dire « ou ».
- `else` sert si aucun `when` ne correspond.

Ligne par ligne :

- `jour` vaut `"samedi"` ;
- `case jour` annonce la valeur à comparer ;
- premier `when` : `jour` vaut-il `"samedi"` ou `"dimanche"` ? Oui : Ruby affiche
  « Week-end ! » ;
- les autres `when` et le `else` sont sautés, comme avec `elsif`.

`when` accepte aussi des intervalles :

```ruby
note = 12
mention = case note
          when 0...10 then "insuffisant"
          when 10...14 then "passable"
          else "bien"
          end
puts mention
```

```sortie
passable
```

`then` permet d'écrire la réponse sur la même ligne que `when`.

## Erreurs fréquentes

### Oublier end

```ruby norun
note = 14
if note >= 10
  puts "Admis"
```

```text
main.rb: --> main.rb
Unmatched keyword, missing `end' ?
> 2  if note >= 10
main.rb:3: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
  puts "Admis"
              ^
```

Chaque `if`, `case`, `while`, `until` et `do` doit être fermé par un `end`. Ajoutez
`end` après `puts "Admis"`. Indentez votre code : un `end` manquant se voit tout de
suite.

### Écrire = au lieu de ==

```ruby norun
note = 14
if note = 10
  puts "dix"
end
```

```text
main.rb:2: warning: found `= literal' in conditional, should be ==
dix
```

`note = 10` **range** 10 dans `note` : la condition est toujours vraie. Ruby prévient
(*warning* : « avertissement »), mais le programme continue. Écrivez `if note == 10`.

### Écrire elseif ou else if

```ruby norun
note = 12
if note >= 10
  puts "Admis"
elseif note >= 5
  puts "Rattrapage"
end
```

```text
Admis
main.rb:4:in `<main>': undefined method `elseif' for main (NoMethodError)

elseif note >= 5
^^^^^^
Did you mean?  elsif
```

Ruby lit `elseif` comme une méthode inconnue. Le mot-clé s'écrit `elsif`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```ruby norun
x = 5
puts "A" if x > 3 && x < 5
puts "B" if x > 3 || x < 5
```

> **Réponse**
>
> Seulement `B`. `x < 5` est faux (5 n'est pas plus petit que 5), donc le `&&` est faux.
> Avec `||`, il suffit que `x > 3` soit vrai.

**Question 2 :** qu'affiche ce code ?

```ruby norun
connecte = true
puts "Bienvenue" unless connecte
puts "Au revoir"
```

> **Réponse**
>
> Seulement `Au revoir`. `unless` exécute le code quand la condition est **fausse** :
> ici `connecte` est vrai, donc « Bienvenue » ne s'affiche pas.

**Question 3 :** que vaut `activite` ?

```ruby norun
jour = "dimanche"
activite = case jour
           when "samedi", "dimanche" then "repos"
           else "travail"
           end
```

> **Réponse**
>
> `"repos"`. Dans un `when`, la virgule veut dire « ou » : `"dimanche"` correspond au
> premier `when`, et `case` renvoie la valeur de cette branche.

## En résumé

- `if`, `elsif` et `else` choisissent quoi faire ; `unless` est le contraire de `if`.
- `if` renvoie une valeur : celle de la branche choisie.
- `&&` (et), `||` (ou), `!` (non) combinent des conditions.
- `case` et `when` comparent une valeur à plusieurs possibilités.

Suite : [les boucles](boucles.md).
