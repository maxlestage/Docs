# Procs, lambdas et méthodes

Cette page montre comment ranger du code dans une variable avec une proc ou une lambda, le passer à une méthode, le renvoyer et le combiner.

> **Objectifs :**
>
> - vous saurez la différence entre un bloc, une proc et une lambda ;
> - vous saurez créer une lambda avec `->(x) { }` et l'appeler ;
> - vous saurez passer de l'un à l'autre avec `&` et `method(:nom)` ;
> - vous saurez qu'une lambda garde les variables qui l'entourent ;
> - vous saurez écrire une méthode qui renvoie une fonction, et composer des fonctions.

## Bloc, proc, lambda : trois cousins

Vous connaissez déjà les [blocs](blocs.md) : du code donné à une méthode, entre `{ }`.
Un bloc a une limite : il n'existe que le temps de l'appel. Impossible de le ranger dans
une variable pour s'en resservir.

Une **proc** et une **lambda** sont des blocs transformés en objets. On peut les ranger,
les passer, les renvoyer. Pensez à une recette : le bloc est une recette dite à voix
haute, la proc ou la lambda est la même recette **écrite sur une fiche** que vous
pouvez donner à quelqu'un.

```ruby
saluer_proc = proc { |nom| "Bonjour #{nom}" }
saluer_lambda = ->(nom) { "Bonjour #{nom}" }

puts saluer_proc.call("Alice")
puts saluer_lambda.call("Bob")
puts saluer_proc.class
puts saluer_lambda.lambda?
```

```sortie
Bonjour Alice
Bonjour Bob
Proc
true
```

Une lambda est une sorte de proc (sa classe est aussi `Proc`), un peu plus stricte.

> **À retenir :** pour débuter, utilisez des lambdas. Elles se comportent comme des
> [méthodes](../debuter/glossaire.md#methode), donc sans surprise.

## Écrire et appeler une lambda

La forme courante est la « flèche » : `->(paramètres) { code }`. Le résultat est la
dernière valeur calculée, comme pour une méthode.

```ruby
carre = ->(x) { x * x }
aire = ->(largeur, hauteur) { largeur * hauteur }
bonjour = -> { "Bonjour !" }

puts carre.call(4)
puts carre.(5)
puts carre[6]
puts aire.call(3, 4)
puts bonjour.call
```

```sortie
16
25
36
12
Bonjour !
```

Les trois écritures `call(…)`, `.(…)` et `[…]` font la même chose. `call` est la plus
lisible.

Sur plusieurs lignes, écrivez `do ... end` ou des accolades sur plusieurs lignes :

```ruby
prix_ttc = lambda do |ht|
  tva = ht * 0.2
  ht + tva
end

puts prix_ttc.call(50)
```

```sortie
60.0
```

## Les différences entre proc et lambda

### Le nombre d'arguments

Une lambda vérifie le nombre d'arguments, comme une méthode. Une proc est tolérante :
elle complète avec `nil` ou ignore le surplus. `arity` donne le nombre attendu.

```ruby
l = ->(a, b) { "#{a} et #{b}" }
pr = proc { |a, b| "#{a.inspect} et #{b.inspect}" }

puts l.arity
puts pr.call(1)
puts pr.call(1, 2, 3)

begin
  l.call(1)
rescue ArgumentError => e
  puts "Erreur : #{e.message}"
end
```

```sortie
2
1 et nil
1 et 2
Erreur : wrong number of arguments (given 1, expected 2)
```

### Le sens de return

Dans une lambda, `return` sort de la lambda seulement. Dans une proc, `return` sort de
**la méthode qui l'entoure** : c'est souvent une surprise.

```ruby
def avec_lambda
  l = -> { return 1 }
  l.call
  "fin de la méthode"
end

def avec_proc
  pr = proc { return 1 }
  pr.call
  "fin de la méthode"
end

puts avec_lambda
puts avec_proc
```

```sortie
fin de la méthode
1
```

La proc a fait quitter `avec_proc` tout de suite : la dernière ligne n'a jamais été
lue.

| | Bloc | Proc | Lambda |
| --- | --- | --- | --- |
| Rangé dans une variable | non | oui | oui |
| Vérifie le nombre d'arguments | non | non | oui |
| `return` sort de… | la méthode autour | la méthode autour | la lambda |

## Passer de l'un à l'autre avec &

Le `&` convertit dans les deux sens :

- devant un argument, `&ma_lambda` donne la lambda **comme bloc** à la méthode ;
- devant un paramètre, `&bloc` **récupère** le bloc reçu sous forme de proc.

```ruby
doubler = ->(x) { x * 2 }
p [1, 2, 3].map(&doubler)

def garder(liste, &regle)
  puts "La règle est une #{regle.class}"
  liste.select(&regle)
end

p garder([5, 12, 8, 20]) { |n| n > 10 }
```

```sortie
[2, 4, 6]
La règle est une Proc
[12, 20]
```

Vous connaissez déjà `map(&:upcase)` : `&` transforme le
[symbole](../debuter/glossaire.md#symbole) `:upcase` en proc qui appelle `upcase`.

## Transformer une méthode en objet

`method(:nom)` attrape une méthode existante et en fait un objet que l'on peut passer
comme une lambda.

```ruby
def crier(texte)
  texte.upcase + " !"
end

crieur = method(:crier)
puts crieur.call("salut")
p ["oui", "non"].map(&crieur)
p ["3", "14"].map(&method(:Integer))
```

```sortie
SALUT !
["OUI !", "NON !"]
[3, 14]
```

## Une lambda garde ses variables

Une lambda, comme un bloc, est une [closure](../debuter/glossaire.md#closure) : elle
emporte les variables visibles à l'endroit où elle est créée. Elle voit même leurs
changements plus tard.

```ruby
compteur = 0
incrementer = -> { compteur += 1 }

incrementer.call
incrementer.call
incrementer.call
puts compteur
```

```sortie
3
```

La lambda modifie la vraie variable `compteur`, pas une copie.

## Une méthode qui renvoie une fonction

Une méthode peut fabriquer et renvoyer une lambda. Chaque lambda garde **sa propre**
valeur du paramètre : c'est une petite usine à fonctions.

```ruby
def multiplicateur(facteur)
  ->(x) { x * facteur }
end

double = multiplicateur(2)
triple = multiplicateur(3)
puts double.call(10)
puts triple.call(10)
p [1, 2, 3].map(&triple)
```

```sortie
20
30
[3, 6, 9]
```

Autre exemple utile : un compteur dont la variable est cachée, que personne d'autre ne
peut toucher.

```ruby
def nouveau_compteur
  total = 0
  -> { total += 1 }
end

a = nouveau_compteur
b = nouveau_compteur
a.call
a.call
puts a.call
puts b.call
```

```sortie
3
1
```

## Composer des fonctions avec >> et <<

`f >> g` crée une nouvelle fonction : « applique `f`, puis `g` au résultat ». `f << g`
fait l'inverse : « `g` d'abord, puis `f` ». C'est une chaîne de montage.

```ruby
nettoyer = ->(s) { s.strip }
majuscules = ->(s) { s.upcase }
encadrer = ->(s) { "[#{s}]" }

traiter = nettoyer >> majuscules >> encadrer
puts traiter.call("  bonjour  ")

ajouter_un = ->(x) { x + 1 }
doubler = ->(x) { x * 2 }
puts (ajouter_un >> doubler).call(5)
puts (ajouter_un << doubler).call(5)
```

```sortie
[BONJOUR]
12
11
```

`(ajouter_un >> doubler).call(5)` calcule (5 + 1) × 2. Avec `<<`, c'est 5 × 2 + 1.

## Erreurs fréquentes

### Appeler une lambda comme une méthode

```ruby norun
carre = ->(x) { x * x }
puts carre(3)
```

```text
main.rb:2:in `<main>': undefined method `carre' for main (NoMethodError)

puts carre(3)
     ^^^^^
```

`carre` est une variable qui contient une lambda, pas une méthode. Appelez-la avec
`carre.call(3)` ou `carre.(3)`.

### Oublier le & pour passer une lambda

```ruby norun
doubler = ->(x) { x * 2 }
p [1, 2].map(doubler)
```

```text
main.rb:2:in `map': wrong number of arguments (given 1, expected 0) (ArgumentError)

p [1, 2].map(doubler)
             ^^^^^^^
	from main.rb:2:in `<main>'
```

`map` attend un bloc, pas un argument. Le `&` transforme la lambda en bloc :
`[1, 2].map(&doubler)`.

### Un return dans une proc renvoyée par une méthode

```ruby norun
def fabriquer
  proc { |x| return x * 2 }
end

doubler = fabriquer
puts doubler.call(5)
```

```text
main.rb:2:in `block in fabriquer': unexpected return (LocalJumpError)
	from main.rb:6:in `<main>'
```

Le `return` d'une proc veut quitter la méthode `fabriquer`, mais elle est déjà
terminée. Utilisez une lambda (`->(x) { x * 2 }`) : son `return` reste chez elle.

## Testez-vous

**Question 1 :** écrivez une lambda `plus` qui additionne deux nombres, puis appelez-la
avec 2 et 3.

> **Réponse**
>
> ```ruby
> plus = ->(a, b) { a + b }
> puts plus.call(2, 3)
> ```
>
> ```sortie
> 5
> ```

**Question 2 :** que se passe-t-il si on appelle `->(a, b) { a + b }` avec un seul
argument ? Et `proc { |a, b| a }` ?

> **Réponse**
>
> La lambda lève une `ArgumentError` (wrong number of arguments). La proc ne dit rien :
> `b` vaut simplement `nil`.

**Question 3 :** avec `ajouter = ->(n) { ->(x) { x + n } }`, que vaut
`ajouter.call(10).call(5)` ?

> **Réponse**
>
> `15` : `ajouter.call(10)` renvoie une lambda qui ajoute 10, appelée ensuite avec 5.
>
> ```ruby
> ajouter = ->(n) { ->(x) { x + n } }
> puts ajouter.call(10).call(5)
> ```
>
> ```sortie
> 15
> ```

## En résumé

- Un bloc accompagne un appel ; une proc ou une lambda est un bloc rangé dans un objet.
- `->(x) { ... }` crée une lambda ; `call`, `.()` ou `[]` l'exécutent.
- La lambda vérifie ses arguments et son `return` reste local : préférez-la.
- `&` passe une lambda comme bloc ; `method(:nom)` transforme une méthode en objet.
- Une lambda garde les variables qui l'entourent : on peut fabriquer des fonctions.
- `f >> g` et `f << g` enchaînent des fonctions.

Suite : [énumérateurs et paresse](enumerateurs.md).
