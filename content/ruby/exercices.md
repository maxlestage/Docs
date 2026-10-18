# Exercices corrigés

Cette page propose 12 exercices corrigés, du plus facile au plus difficile, pour pratiquer tout le parcours Ruby.

## Comment utiliser cette page

Chaque exercice a un niveau :

- ★☆☆ : facile ;
- ★★☆ : moyen ;
- ★★★ : plus dur.

Lisez l'énoncé, puis écrivez votre solution dans un fichier `.rb` et lancez-le. Ouvrez
la solution seulement après avoir essayé.

> **Astuce :** il existe souvent plusieurs bonnes solutions. Si votre programme affiche
> le bon résultat, c'est gagné !

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Créez deux variables, `prenom` et `age`. Affichez ensuite la phrase
`Je m'appelle Alice et j'ai 30 ans.`, avec vos variables.

> **Indice :** utilisez l'interpolation `#{...}` (voir [les premiers pas](premiers-pas.md)).

> **Solution**
>
> ```ruby
> prenom = "Alice"
> age = 30
> puts "Je m'appelle #{prenom} et j'ai #{age} ans."
> ```
>
> ```sortie
> Je m'appelle Alice et j'ai 30 ans.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Pour le nombre `7`, affichez `7 est impair`. Pour `10`, le programme doit afficher
`10 est pair`.

> **Indice :** `n.even?` vaut `true` si `n` est pair.

> **Solution**
>
> ```ruby
> [7, 10].each do |n|
>   if n.even?
>     puts "#{n} est pair"
>   else
>     puts "#{n} est impair"
>   end
> end
> ```
>
> ```sortie
> 7 est impair
> 10 est pair
> ```

### Exercice 3 : La table de 7

Niveau : ★☆☆

Affichez la table de multiplication de 7, de `7 x 1 = 7` à `7 x 5 = 35`.

> **Indice :** une boucle `(1..5).each` fait 5 tours.

> **Solution**
>
> ```ruby
> (1..5).each do |i|
>   puts "7 x #{i} = #{7 * i}"
> end
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercice 4 : L'aire d'un rectangle

Niveau : ★☆☆

Écrivez une méthode `aire(largeur, hauteur)` qui renvoie l'aire d'un rectangle.
Affichez l'aire d'un rectangle de 4 sur 3.

> **Solution**
>
> ```ruby
> def aire(largeur, hauteur)
>   largeur * hauteur
> end
>
> puts aire(4, 3)
> ```
>
> ```sortie
> 12
> ```

## Texte et collections

### Exercice 5 : La moyenne

Niveau : ★★☆

Avec le tableau `notes = [12, 15, 9, 14]`, calculez et affichez la moyenne.

> **Indice :** `sum` donne le total, `length` le nombre de notes. Pensez à la division
> avec un nombre à virgule (`to_f`).

> **Solution**
>
> ```ruby
> notes = [12, 15, 9, 14]
> moyenne = notes.sum.to_f / notes.length
> puts "Moyenne : #{moyenne}"
> ```
>
> ```sortie
> Moyenne : 12.5
> ```

### Exercice 6 : Compter les voyelles

Niveau : ★★☆

Écrivez une méthode `voyelles(texte)` qui compte les voyelles (a, e, i, o, u, y) d'un
texte. Testez-la avec `"Bonjour Ruby"`.

> **Indice :** passez le texte en minuscules, puis parcourez `chars`.

> **Solution**
>
> ```ruby
> def voyelles(texte)
>   compte = 0
>   texte.downcase.chars.each do |lettre|
>     compte += 1 if "aeiouy".include?(lettre)
>   end
>   compte
> end
>
> puts voyelles("Bonjour Ruby")
> ```
>
> ```sortie
> 5
> ```

### Exercice 7 : Le palindrome

Niveau : ★★☆

Un palindrome se lit pareil dans les deux sens, comme « kayak ». Écrivez une méthode
`palindrome?(mot)` qui répond `true` ou `false`. Ignorez les majuscules.

> **Indice :** comparez le mot avec `mot.reverse` (voir [le texte](texte.md)).

> **Solution**
>
> ```ruby
> def palindrome?(mot)
>   mot = mot.downcase
>   mot == mot.reverse
> end
>
> puts palindrome?("Kayak")
> puts palindrome?("Ruby")
> ```
>
> ```sortie
> true
> false
> ```

### Exercice 8 : Compter les mots

Niveau : ★★☆

Avec la phrase `"le chat et le chien et le lapin"`, comptez combien de fois chaque mot
apparaît. Affichez une ligne par mot.

> **Indice :** utilisez un hash créé avec `Hash.new(0)` : une clé absente vaut alors `0`.

> **Solution**
>
> ```ruby
> phrase = "le chat et le chien et le lapin"
> compte = Hash.new(0)
> phrase.split(" ").each do |mot|
>   compte[mot] += 1
> end
> compte.each do |mot, nombre|
>   puts "#{mot} : #{nombre}"
> end
> ```
>
> ```sortie
> le : 3
> chat : 1
> et : 2
> chien : 1
> lapin : 1
> ```

### Exercice 9 : Garder et doubler

Niveau : ★★☆

Avec les nombres de 1 à 10, gardez seulement les nombres pairs, puis doublez-les.
Affichez le tableau obtenu.

> **Indice :** `select` filtre, `map` transforme (voir [les blocs](blocs.md)).

> **Solution**
>
> ```ruby
> resultat = (1..10).select { |n| n.even? }.map { |n| n * 2 }
> p resultat
> ```
>
> ```sortie
> [4, 8, 12, 16, 20]
> ```

## Pour aller plus loin

### Exercice 10 : FizzBuzz

Niveau : ★★☆

Affichez les nombres de 1 à 15. Mais :

- à la place des multiples de 3, affichez `Fizz` ;
- à la place des multiples de 5, affichez `Buzz` ;
- à la place des multiples de 3 **et** de 5, affichez `FizzBuzz`.

> **Indice :** `n % 3 == 0` est vrai si `n` est un multiple de 3. Testez d'abord le cas
> « 3 et 5 ».

> **Solution**
>
> ```ruby
> (1..15).each do |n|
>   if n % 3 == 0 && n % 5 == 0
>     puts "FizzBuzz"
>   elsif n % 3 == 0
>     puts "Fizz"
>   elsif n % 5 == 0
>     puts "Buzz"
>   else
>     puts n
>   end
> end
> ```
>
> ```sortie
> 1
> 2
> Fizz
> 4
> Buzz
> Fizz
> 7
> 8
> Fizz
> Buzz
> 11
> Fizz
> 13
> 14
> FizzBuzz
> ```

### Exercice 11 : Le compte en banque

Niveau : ★★★

Créez une classe `Compte` avec un solde qui commence à `0`. Ajoutez :

- une méthode `deposer(montant)` qui ajoute de l'argent ;
- une méthode `retirer(montant)` qui enlève de l'argent, seulement si le solde suffit ;
- une lecture du solde avec `attr_reader`.

> **Indice :** rangez le solde dans `@solde`, dans `initialize` (voir
> [les classes](types.md)).

> **Solution**
>
> ```ruby
> class Compte
>   attr_reader :solde
>
>   def initialize
>     @solde = 0
>   end
>
>   def deposer(montant)
>     @solde += montant
>   end
>
>   def retirer(montant)
>     if montant > @solde
>       puts "Solde insuffisant"
>     else
>       @solde -= montant
>     end
>   end
> end
>
> compte = Compte.new
> compte.deposer(100)
> compte.retirer(30)
> compte.retirer(500)
> puts "Solde : #{compte.solde}"
> ```
>
> ```sortie
> Solde insuffisant
> Solde : 70
> ```

### Exercice 12 : Un âge valide

Niveau : ★★★

Écrivez une méthode `lire_age(texte)` qui transforme un texte en âge :

- si le texte n'est pas un nombre, elle renvoie `nil` ;
- si l'âge est négatif, elle lève une erreur `ArgumentError`.

Testez-la avec `"25"`, `"abc"` et `"-3"`, sans que le programme ne s'arrête.

> **Indice :** `Integer("abc")` lève une `ArgumentError`. Utilisez `rescue` (voir
> [gérer les erreurs](erreurs.md)).

> **Solution**
>
> ```ruby
> def lire_age(texte)
>   begin
>     age = Integer(texte)
>   rescue ArgumentError
>     return nil
>   end
>   raise ArgumentError, "âge négatif" if age < 0
>   age
> end
>
> ["25", "abc", "-3"].each do |texte|
>   begin
>     age = lire_age(texte)
>     if age.nil?
>       puts "#{texte} : pas un nombre"
>     else
>       puts "#{texte} : #{age} ans"
>     end
>   rescue ArgumentError => e
>     puts "#{texte} : erreur, #{e.message}"
>   end
> end
> ```
>
> ```sortie
> 25 : 25 ans
> abc : pas un nombre
> -3 : erreur, âge négatif
> ```

## En résumé

- Commencez par les exercices ★☆☆, puis montez en difficulté.
- Essayez toujours seul avant d'ouvrir la solution.
- Les exercices 1 à 4 reprennent les bases : variables, conditions, boucles, méthodes.
- Les exercices 5 à 9 travaillent le texte, les tableaux, les hashs et les blocs.
- Les exercices 10 à 12 combinent tout, avec les classes et les erreurs.

Suite : gardez [l'aide-mémoire](aide-memoire.md) sous la main. Comparez avec
[les exercices en Rust](../rust/exercices.md).
