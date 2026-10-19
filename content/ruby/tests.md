# Tester son code

Cette page montre comment écrire des tests automatiques avec Minitest, l'outil livré avec Ruby, pour vérifier que votre code fait bien ce que vous attendez.

> **Objectifs :**
>
> - comprendre à quoi sert un test automatique ;
> - écrire une classe de test avec Minitest ;
> - utiliser `assert_equal`, `assert`, `assert_nil` et `assert_raises` ;
> - lancer les tests et lire le résultat ;
> - écrire le test avant le code, en trois étapes.

## Pourquoi tester ?

Un **test** est un petit programme qui vérifie un autre programme (voir
[le glossaire](../debuter/glossaire.md#test)). Il appelle votre code avec des valeurs
connues et compare le résultat à celui que vous attendez.

Sans tests, vous vérifiez à la main, avec des `puts`, et vous recommencez après chaque
changement. Avec des tests :

- une seule commande vérifie tout, en une seconde ;
- si un changement casse quelque chose, vous le savez tout de suite ;
- les tests montrent comment utiliser votre code : c'est une documentation qui marche.

> **À retenir :** un test répond à une question simple : « avec cette entrée, est-ce que
> j'obtiens bien cette sortie ? ».

## Votre premier test avec Minitest

**Minitest** est livré avec Ruby : rien à installer. Un fichier de test suit toujours
le même modèle :

1. `require "minitest/autorun"` charge Minitest et lance les tests à la fin du fichier ;
2. une classe de test hérite de `Minitest::Test` ;
3. chaque méthode dont le nom commence par `test_` est un test.

```ruby
# test_double.rb
require "minitest/autorun"

def double(x)
  x * 2
end

class TestDouble < Minitest::Test
  def test_double_de_4
    assert_equal 8, double(4)
  end

  def test_double_de_zero
    assert_equal 0, double(0)
  end
end
```

`assert_equal 8, double(4)` se lit : « je m'attends à `8` ; vérifie que `double(4)` vaut
bien `8` ».

## Lancer les tests

Un fichier de test se lance comme n'importe quel programme :

```sh
ruby test_double.rb
```

Ruby affiche un résumé :

```text
Run options: --seed 31187

# Running:

..

Finished in 0.000812s, 2463.0542 runs/s, 2463.0542 assertions/s.

2 runs, 2 assertions, 0 failures, 0 errors, 0 skips
```

- Chaque `.` est un test réussi.
- La dernière ligne compte les tests (`runs`), les vérifications (`assertions`), les
  échecs (`failures`) et les erreurs (`errors`).
- Le nombre après `--seed` et le temps changent à chaque fois : Minitest lance les tests
  dans un ordre au hasard, pour s'assurer qu'ils ne dépendent pas les uns des autres.

> **Astuce :** `ruby test_double.rb --verbose` affiche le nom de chaque test, et
> `ruby test_double.rb -n test_double_de_4` ne lance que ce test-là.

## Lire un échec

Quand une vérification échoue, Minitest affiche un `F` et explique le problème. Voici
un `double` qui se trompe (il ajoute 2 au lieu de multiplier) :

```ruby norun
# test_double.rb
require "minitest/autorun"

def double(x)
  x + 2
end

class TestDouble < Minitest::Test
  def test_double_de_4
    assert_equal 8, double(4)
  end
end
```

```text
F

  1) Failure:
TestDouble#test_double_de_4 [test_double.rb:10]:
Expected: 8
  Actual: 6

1 runs, 1 assertions, 1 failures, 0 errors, 0 skips
```

Le message donne tout ce qu'il faut : le test (`test_double_de_4`), la ligne (`10`), la
valeur attendue (`Expected: 8`) et la valeur obtenue (`Actual: 6`).

- `F` (*failure*, échec) : une vérification a donné un mauvais résultat.
- `E` (*error*, erreur) : le code a planté pendant le test (une méthode inconnue, par
  exemple).

## Les assertions

Une **assertion** est une vérification. Minitest en propose plusieurs ; voici les plus
utiles.

| Assertion | Vérifie que… |
| --- | --- |
| `assert_equal attendu, obtenu` | les deux valeurs sont égales |
| `assert valeur` | la valeur est vraie |
| `refute valeur` | la valeur est fausse |
| `assert_nil valeur` | la valeur est `nil` |
| `assert_includes tableau, x` | le tableau contient `x` |
| `assert_in_delta attendu, obtenu, 0.01` | deux nombres à virgule sont très proches |
| `assert_raises(Erreur) { … }` | le bloc lève bien cette erreur |

```ruby
require "minitest/autorun"

class TestAssertions < Minitest::Test
  def test_assert_equal
    assert_equal "BONJOUR", "bonjour".upcase
    assert_equal [1, 2, 3], [3, 1, 2].sort
  end

  def test_assert_et_refute
    assert [1, 2, 3].include?(2)
    refute [].any?
  end

  def test_assert_nil
    assert_nil [].first
    assert_nil({ a: 1 }[:b])
  end

  def test_assert_includes
    assert_includes ["pomme", "poire"], "poire"
  end

  def test_assert_in_delta
    assert_in_delta 0.3, 0.1 + 0.2, 0.001
  end

  def test_assert_raises
    assert_raises(ZeroDivisionError) { 10 / 0 }
  end
end
```

> **Note :** `0.1 + 0.2` vaut `0.30000000000000004` en Ruby. Pour comparer des nombres à
> virgule, préférez `assert_in_delta` à `assert_equal`.

### Tester qu'une erreur est levée

Votre code doit parfois refuser une valeur avec `raise` (voir
[lever une erreur](erreurs.md#lever-une-erreur-avec-raise)). `assert_raises` vérifie
que l'erreur arrive bien. Il renvoie l'erreur : vous pouvez aussi vérifier son message.

```ruby
require "minitest/autorun"

def retirer(solde, montant)
  raise ArgumentError, "montant négatif" if montant < 0
  solde - montant
end

class TestRetirer < Minitest::Test
  def test_retrait_normal
    assert_equal 70, retirer(100, 30)
  end

  def test_retrait_negatif_refuse
    erreur = assert_raises(ArgumentError) { retirer(100, -5) }
    assert_equal "montant négatif", erreur.message
  end
end
```

### Un message plus clair

Chaque assertion accepte un dernier argument : le message affiché en cas d'échec.

```ruby
require "minitest/autorun"

class TestPanier < Minitest::Test
  def test_panier_vide_au_depart
    panier = []
    assert panier.empty?, "le panier devrait être vide au départ"
  end
end
```

## Tester une classe

Pour une classe, créez un objet dans le test, appelez ses méthodes, puis vérifiez son
état. La méthode `setup` s'exécute **avant chaque test** : elle prépare un objet neuf.

```ruby
require "minitest/autorun"

class Compte
  attr_reader :solde

  def initialize
    @solde = 0
  end

  def deposer(montant)
    @solde += montant
  end
end

class TestCompte < Minitest::Test
  def setup
    @compte = Compte.new
  end

  def test_solde_de_depart
    assert_equal 0, @compte.solde
  end

  def test_deposer_augmente_le_solde
    @compte.deposer(50)
    @compte.deposer(20)
    assert_equal 70, @compte.solde
  end
end
```

Chaque test reçoit son propre `@compte` : le dépôt du second test ne change pas le
premier.

## Ranger le code et les tests

Dans un projet, le code et les tests vivent dans des fichiers séparés. Par habitude, le
code va dans `lib/` et les tests dans `test/`. Le fichier de test porte le nom du
fichier testé, précédé de `test_`.

```text
mon_projet/
├── lib/
│   └── calcul.rb
└── test/
    └── test_calcul.rb
```

```ruby norun
# lib/calcul.rb
module Calcul
  def self.double(x)
    x * 2
  end
end
```

```ruby norun
# test/test_calcul.rb
require "minitest/autorun"
require_relative "../lib/calcul"

class TestCalcul < Minitest::Test
  def test_double
    assert_equal 42, Calcul.double(21)
  end
end
```

Depuis le dossier `mon_projet` :

```sh
ruby test/test_calcul.rb
```

> **Note :** `../` veut dire « le dossier parent ». Depuis `test/`, `../lib/calcul`
> désigne bien `lib/calcul.rb` (voir [les modules](modules.md#un-module-par-fichier-avec-require-relative)).

## Écrire le test d'abord

Une méthode de travail très répandue consiste à écrire le test **avant** le code. On
avance en trois étapes, qui se répètent : **rouge**, **vert**, **améliorer**.

Exemple : une méthode `mention(note)` qui renvoie `"Bien"` à partir de 14,
`"Passable"` à partir de 10, et `"Insuffisant"` en dessous.

### Étape 1 : rouge, le test échoue

Écrivez d'abord le test, comme si la méthode existait déjà.

```ruby norun
require "minitest/autorun"

class TestMention < Minitest::Test
  def test_mention_bien
    assert_equal "Bien", mention(15)
  end
end
```

Le test échoue, puisque `mention` n'existe pas encore :

```text
  1) Error:
TestMention#test_mention_bien:
NoMethodError: undefined method `mention' for an instance of TestMention
```

C'est normal, et c'est utile : vous savez que le test vérifie vraiment quelque chose.

### Étape 2 : vert, le test passe

Écrivez le code le plus simple qui fait passer les tests. Ajoutez un test à la fois
pour chaque cas.

```ruby
require "minitest/autorun"

def mention(note)
  if note >= 14
    "Bien"
  elsif note >= 10
    "Passable"
  else
    "Insuffisant"
  end
end

class TestMention < Minitest::Test
  def test_mention_bien
    assert_equal "Bien", mention(15)
  end

  def test_mention_passable
    assert_equal "Passable", mention(10)
  end

  def test_mention_insuffisant
    assert_equal "Insuffisant", mention(7)
  end
end
```

Tous les tests sont verts : trois points `...` dans le résumé.

### Étape 3 : améliorer le code

Maintenant que les tests vous protègent, rendez le code plus clair. Ici, un `case`
avec des intervalles se lit mieux. Relancez les tests : s'ils restent verts, rien n'est
cassé.

```ruby
require "minitest/autorun"

def mention(note)
  case note
  when 14.. then "Bien"
  when 10...14 then "Passable"
  else "Insuffisant"
  end
end

class TestMention < Minitest::Test
  def test_mention_bien
    assert_equal "Bien", mention(15)
  end

  def test_mention_passable
    assert_equal "Passable", mention(10)
  end

  def test_mention_insuffisant
    assert_equal "Insuffisant", mention(7)
  end
end
```

> **À retenir :** rouge (un test qui échoue), vert (le code minimal qui le fait
> passer), améliorer (sans changer le comportement). Puis on recommence avec le cas
> suivant.

## Bien nommer ses tests

Un bon nom de test dit **ce qui est vérifié**. Quand il échoue, son nom suffit à
comprendre le problème.

| Moins clair | Plus clair |
| --- | --- |
| `test_1` | `test_double_de_zero_vaut_zero` |
| `test_retirer` | `test_retrait_negatif_refuse` |
| `test_mention` | `test_mention_bien_a_partir_de_14` |

Quelques habitudes :

- un test vérifie **une seule idée** ;
- testez les cas limites : zéro, un tableau vide, la valeur juste à la frontière
  (`mention(14)`, `mention(13.9)`) ;
- la classe de test porte le nom de ce qu'elle teste : `TestCompte` pour `Compte`.

## RSpec en deux mots

Beaucoup de projets Ruby utilisent **RSpec**, une autre bibliothèque de tests. Elle
s'installe comme une gem (`gem install rspec`) et se lit presque comme une phrase en
anglais :

```ruby norun
# spec/calcul_spec.rb
require_relative "../lib/calcul"

RSpec.describe Calcul do
  it "double un nombre" do
    expect(Calcul.double(21)).to eq(42)
  end
end
```

```sh
rspec
```

Les idées sont les mêmes qu'avec Minitest. Commencez par Minitest : il est déjà installé.

## Erreurs fréquentes

### Un nom de méthode sans test_

Minitest ne lance que les méthodes dont le nom commence par `test_`. Les autres sont
ignorées, sans message d'erreur.

```ruby norun
class TestDouble < Minitest::Test
  def double_de_quatre
    assert_equal 8, double(4)
  end
end
```

```text
0 runs, 0 assertions, 0 failures, 0 errors, 0 skips
```

`0 runs` doit vous alerter. Renommez la méthode `test_double_de_quatre`.

### Oublier require "minitest/autorun"

Sans cette ligne, Ruby ne connaît pas Minitest.

```ruby norun
class TestDouble < Minitest::Test
end
```

```text
main.rb:1:in `<main>': uninitialized constant Minitest (NameError)
```

Ajoutez `require "minitest/autorun"` en première ligne.

### Inverser la valeur attendue et la valeur obtenue

`assert_equal` attend **d'abord** la valeur attendue, **ensuite** celle calculée.
Inversées, le message d'échec vous trompe.

```ruby norun
def prix_ttc(prix)
  prix * 1.2
end

assert_equal prix_ttc(10), 12.5
```

```text
Expected: 12.0
  Actual: 12.5
```

Le message prétend que votre code a donné `12.5`, alors qu'il a donné `12.0`. Écrivez
`assert_equal 12.5, prix_ttc(10)`.

### Donner une valeur à assert_raises au lieu d'un bloc

Le code qui doit lever l'erreur va **dans un bloc**. Sinon, il s'exécute avant
`assert_raises`, et le test plante.

```ruby norun
assert_raises(ZeroDivisionError, 10 / 0)
```

```text
  1) Error:
TestDivision#test_division_par_zero:
ZeroDivisionError: divided by 0
```

Écrivez `assert_raises(ZeroDivisionError) { 10 / 0 }`.

## Testez-vous

**1.** Que signifient `.`, `F` et `E` dans le résultat de Minitest ?

> **Réponse**
>
> `.` : un test réussi. `F` : un échec, une assertion a donné un mauvais résultat.
> `E` : une erreur, le code a planté pendant le test.

**2.** Le résumé affiche `0 runs`. Quelle est la cause la plus probable ?

> **Réponse**
>
> Les méthodes de test ne commencent pas par `test_` : Minitest ne les voit pas.

**3.** Quelles sont les trois étapes quand on écrit le test d'abord ?

> **Réponse**
>
> Rouge : écrire un test qui échoue. Vert : écrire le code le plus simple qui le fait
> passer. Améliorer : rendre le code plus clair, en gardant les tests verts.

## En résumé

- Un test vérifie automatiquement qu'une entrée donne la sortie attendue.
- Avec Minitest : `require "minitest/autorun"`, une classe qui hérite de
  `Minitest::Test`, des méthodes `test_…`.
- `assert_equal attendu, obtenu` est l'assertion la plus courante ; `assert_raises`
  prend un bloc.
- `ruby test_fichier.rb` lance les tests : `.` réussi, `F` échec, `E` erreur.
- Rouge, vert, améliorer : écrivez le test, puis le code, puis rendez-le plus clair.

Suite : [Déboguer en Ruby](debogage.md)
