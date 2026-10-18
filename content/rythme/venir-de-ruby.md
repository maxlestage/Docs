# Venir de Ruby (ou de Python)

Si vous connaissez déjà Ruby ou Python, Rythme vous semblera familier, à quelques grandes différences près : cette page les présente une à une, puis traduit pas à pas cinq petits programmes Ruby.

> **Objectifs :**
>
> - vous saurez ce qui marche en Rythme comme en Ruby, et ce qui change ;
> - vous saurez remplacer une boucle par `each`, `map`, `fold` ou une fonction récursive ;
> - vous saurez construire de nouvelles valeurs au lieu de modifier les anciennes ;
> - vous saurez remplacer `nil` par une `Option` et les exceptions par un `Result` ;
> - vous saurez traduire un petit programme Ruby en Rythme.

## Ce qui ne change pas

Rythme emprunte beaucoup à Ruby : les blocs, les méthodes appelées avec un
point, les noms qui finissent par `?`, `puts`, l'insertion `#{...}` dans
les textes. Beaucoup de noms Ruby existent aussi : `select`, `reduce`,
`uniq`, `upcase`, `include?`, `size`... Ce code se lit presque comme du
Ruby :

```rythme
noms = ["Ada", "Grace", "Alan"]
noms.each do |nom|
  puts "Bonjour, #{nom} !"
end

carres = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map { |x| x * x }
  .sum

puts carres
puts noms.map { |n| n.upcase }.include?("ADA")
```

```sortie
Bonjour, Ada !
Bonjour, Grace !
Bonjour, Alan !
56
true
```

Et de Python, Rythme garde `print`, `len`, les listes `[1, 2]`, `//` pour
la division entière, et des entiers sans limite de taille.

## Le grand tableau de correspondance

| Idée | Ruby | Python | Rythme |
|------|------|--------|--------|
| afficher | `puts "salut"` | `print("salut")` | `print("salut")` ou `puts "salut"` |
| insérer une valeur | `"total : #{x}"` | `f"total : {x}"` | `"total : {x}"` |
| une fonction | `def double(x) = x * 2` | `def double(x): return x * 2` | `fn double(x) = x * 2` |
| une fonction anonyme | `->(x) { x * 2 }` | `lambda x: x * 2` | `fn(x) = x * 2` |
| le résultat | dernière ligne, ou `return` | `return` | dernière ligne, toujours |
| transformer | `xs.map { \|x\| x * 2 }` | `[x * 2 for x in xs]` | `map(xs, fn(x) = x * 2)` |
| garder | `xs.select(&:even?)` | `[x for x in xs if x % 2 == 0]` | `filter(xs, is_even)` |
| réduire | `xs.reduce(0) { \|a, x\| a + x }` | `functools.reduce(...)` | `fold(xs, 0, fn(a, x) = a + x)` |
| enchaîner | `xs.sort.reverse` | `sorted(xs)[::-1]` | `xs \|> sort \|> reverse` |
| répéter | `xs.each { ... }`, `while` | `for x in xs:` | `each(xs, f)`, récursion |
| condition | `a if c` (sans else possible) | `a if c else b` | `if c then a else b` (else obligatoire) |
| choisir un cas | `case x when 1 then ...` | `match x: case 1: ...` | `match x` puis `1 -> ...` et `end` |
| pas de valeur | `nil` | `None` | `None`, et `Some(x)` pour une valeur |
| un échec | `raise` / `rescue` | `raise` / `except` | `Err(e)` / `Ok(x)`, avec `match` ou `?` |
| modifier | `x += 1`, `xs << 4` | `x += 1`, `xs.append(4)` | impossible : `x2 = x + 1`, `push(xs, 4)` |
| des champs nommés | `Struct`, `Hash` | `dict`, `dataclass` | `{nom: "Ada", age: 36}` |
| un dictionnaire | `{ "ada" => 36 }` | `{"ada": 36}` | `dict([["ada", 36]])` |
| des sortes de valeurs | `class` | `class` | `type Forme = Cercle(r) \| Carre(c)` |
| `7 / 2` | `3` | `3.5` | `3.5` (et `7 // 2` vaut `3`) |
| tester | `minitest`, `rspec` | `pytest` | `test "nom" = assert_eq(a, b)` |

Les quatre plus grandes différences sont dans les sections suivantes.

## Pas de boucles : each, map et la récursion

Rythme n'a ni `for`, ni `while`, ni `loop`. On ne dit pas « répète ceci »,
on dit « fais ceci **pour chaque** élément » :

- `each` pour un effet sur chaque élément (afficher) ;
- `map` pour transformer chaque élément ;
- `filter` pour en garder certains, `fold` pour les combiner en une valeur ;
- une fonction [récursive](../debuter/glossaire.md#recursion) quand on ne
  sait pas d'avance combien de fois répéter.

```rythme
1..3 |> each(fn(i) = print("tour {i}"))

fn compte_a_rebours(0) = "partez !"
fn compte_a_rebours(n) = "{n}... " + compte_a_rebours(n - 1)

print(compte_a_rebours(3))
```

```sortie
tour 1
tour 2
tour 3
3... 2... 1... partez !
```

## Pas de mutation : de nouvelles valeurs

En Ruby, on modifie : `total += 5`, `liste << 4`, `compte.solde = 150`. En
Rythme, les valeurs sont [immuables](../debuter/glossaire.md#immuable) :
on **construit une nouvelle valeur**, avec un nouveau nom.

```rythme
liste = [1, 2, 3]
plus_long = push(liste, 4)

compte = {titulaire: "Ada", solde: 100}
apres_depot = {..compte, solde: compte.solde + 50}

print(liste, plus_long)
print(compte.solde, apres_depot.solde)
```

```sortie
[1, 2, 3] [1, 2, 3, 4]
100 150
```

`{..compte, solde: ...}` recopie tous les champs de `compte` et remplace
`solde`. L'ancien compte reste intact : aucune fonction ne peut le changer
dans votre dos.

## Pas de nil : Option

En Ruby, `find` renvoie `nil` quand il ne trouve rien, et l'oubli de ce cas
donne la célèbre `NoMethodError: undefined method for nil`. En Rythme,
`find` renvoie une [Option](../debuter/glossaire.md#option) :
`Some(valeur)` ou `None`. On ne peut pas se servir de la valeur sans ouvrir
la boîte, donc sans penser au cas vide.

```rythme
print(find([1, 4, 7], is_even), find([1, 3], is_even))
print(unwrap_or(first([]), "rien"))
```

```sortie
Some(4) None
rien
```

## Pas d'exceptions : Result

En Ruby, `Integer("abc")` lève une exception, qui remonte jusqu'à un
`rescue`... ou arrête le programme. En Rythme, une opération qui peut
échouer renvoie un [Result](../debuter/glossaire.md#result) : `Ok(valeur)`
ou `Err(message)`. L'échec est une valeur ordinaire, visible dans le code.

```rythme
print(parse_int("42"), parse_int("abc"))
```

```sortie
Ok(42) Err("\"abc\" n'est pas un nombre entier")
```

Dans une fonction, `?` joue le rôle d'un `raise` automatique : sur un
`Err`, la fonction s'arrête et renvoie l'erreur. Voir
[gérer les erreurs](erreurs.md).

## if a toujours un else

En Ruby, `puts "oui" if ok` ne fait rien quand `ok` est faux. En Rythme,
`if` est une expression qui **donne toujours une valeur** : il lui faut
donc les deux cas. Quand il n'y a rien à faire, donnez `()`, la valeur
« rien d'intéressant ».

```rythme
x = 5
signe = if x > 0 then "positif" else "négatif ou nul"
print(signe)

ok = true
if ok then print("oui") else ()
```

```sortie
positif
oui
```

`unless`, comme en Ruby, est le contraire de `if` : il a lui aussi
toujours un `else`.

## rythme help ruby

Rythme a une page d'aide pour les habitués de Ruby. En voici le début et
la partie « Ce qui change » :

```text
$ rythme help ruby
VENIR DE RUBY

Rythme se lit comme Ruby, mais ses valeurs ne changent jamais : pas de
mutation, pas de nil, pas d'exceptions. Ce que vous savez de Ruby marche
presque toujours :
...
Ce qui change
  parenthèses     un appel dans une expression en a besoin : x = len(xs),
                  pas x = len xs
  pas de mutation pas de x += 1 : donnez un nouveau nom à la nouvelle valeur,
                  total2 = total + 1
  pas de nil      une valeur absente est None, une valeur présente Some(x)
  pas d'exception les erreurs sont des valeurs, Ok(x) ou Err(message),
                  transmises avec ?
  if et unless    ont toujours un else : ils donnent une valeur ; pas de
                  `return`, la dernière ligne d'une fonction est son résultat
  pas de classes  enregistrements {nom: "Ada"} et types : type Forme = Cercle(r) | Carre(c)
  pas de boucles  each, map, select, reduce, ou une fonction qui s'appelle
  case/when       match valeur ... end, un cas par ligne : motif -> résultat
```

## Traduire cinq programmes Ruby

Pour chaque programme : le Ruby d'origine, puis sa traduction, en
remplaçant ce qui n'existe pas en Rythme.

### 1. Une boucle avec un accumulateur

```ruby
nombres = [3, 8, 5, 12, 7, 4]
total = 0
nombres.each do |n|
  total += n if n.even?
end
puts total
```

```sortie
24
```

Deux obstacles : `total += n` modifie une valeur, et `if` n'a pas de
`else`. Mais que fait ce code ? Il **garde** les nombres pairs, puis en
fait la **somme**. Écrivons exactement cela :

```rythme
nombres = [3, 8, 5, 12, 7, 4]

print(nombres |> filter(is_even) |> sum)
print(nombres.select(even?).sum)
```

```sortie
24
24
```

Les deux lignes sont justes : la seconde garde les noms Ruby. Une boucle
avec un accumulateur cache presque toujours un `filter`, un `map`, un
`sum` ou un `fold`.

### 2. Compter avec un Hash

```ruby
mots = "le chat et le chien et le loup".split
compte = Hash.new(0)
mots.each { |m| compte[m] += 1 }
p compte
```

```sortie
{"le"=>3, "chat"=>1, "et"=>2, "chien"=>1, "loup"=>1}
```

Le Hash est modifié à chaque tour. En Rythme, `fold` passe d'un tour à
l'autre un dictionnaire **nouveau à chaque fois** : `put` renvoie une copie
avec la clé mise à jour.

```rythme
mots = split("le chat et le chien et le loup")

compte = fold(mots, dict(), fn(d, m) = put(d, m, unwrap_or(get(d, m), 0) + 1))
print(compte)
```

```sortie
dict("chat" => 1, "chien" => 1, "et" => 2, "le" => 3, "loup" => 1)
```

`get(d, m)` est une Option (le mot peut être absent) : `unwrap_or(..., 0)`
joue le rôle du `Hash.new(0)` de Ruby.

### 3. Chercher, et ne pas trouver

```ruby
utilisateurs = [{ nom: "Ada", age: 36 }, { nom: "Alan", age: 41 }]
trouve = utilisateurs.find { |u| u[:nom] == "Grace" }
puts trouve.nil? ? "inconnu" : trouve[:age]
```

```sortie
inconnu
```

En Ruby, oublier le `trouve.nil?` ferait planter le programme. En Rythme,
`find` donne une Option, et `match` oblige à traiter les deux cas :

```rythme
utilisateurs = [{nom: "Ada", age: 36}, {nom: "Alan", age: 41}]

fn age_de(nom) = match find(utilisateurs, fn(u) = u.nom == nom)
  Some(u) -> text(u.age)
  None -> "inconnu"
end

print(age_de("Grace"), age_de("Ada"))
```

```sortie
inconnu 36
```

### 4. Une classe qui lève une exception

```ruby
class Compte
  attr_reader :solde

  def initialize(solde)
    @solde = solde
  end

  def retirer(montant)
    raise ArgumentError, "solde insuffisant" if montant > @solde
    @solde -= montant
  end
end

compte = Compte.new(100)
compte.retirer(30)
puts compte.solde
begin
  compte.retirer(500)
rescue ArgumentError => e
  puts "Erreur : #{e.message}"
end
```

```sortie
70
Erreur : solde insuffisant
```

Trois traductions à la fois :

- la classe devient un enregistrement `{solde: 100}` ;
- `@solde -= montant` devient un **nouveau** compte, `{..compte, solde: ...}` ;
- `raise` devient `Err(...)`, et le succès `Ok(nouveau_compte)`.

```rythme
fn retirer(compte, montant) =
  if montant > compte.solde then Err("solde insuffisant")
  else Ok({..compte, solde: compte.solde - montant})

fn montrer(resultat) = match resultat
  Ok(c) -> print(c.solde)
  Err(message) -> print("Erreur : {message}")
end

compte = {solde: 100}
montrer(retirer(compte, 30))
montrer(retirer(compte, 500))
```

```sortie
70
Erreur : solde insuffisant
```

Pour enchaîner deux retraits, `?` remplace le `begin`/`rescue` : au premier
échec, la fonction renvoie l'erreur.

```rythme
fn retirer(compte, montant) =
  if montant > compte.solde then Err("solde insuffisant")
  else Ok({..compte, solde: compte.solde - montant})

fn deux_retraits(compte)
  apres_un = retirer(compte, 30)?
  retirer(apres_un, 50)
end

print(deux_retraits({solde: 100}), deux_retraits({solde: 60}))
```

```sortie
Ok({solde: 20}) Err("solde insuffisant")
```

### 5. Une boucle while

Combien d'étapes faut-il pour ramener un nombre à 1, en le divisant par 2
s'il est pair, et en le remplaçant par 3n + 1 sinon ?

```ruby
n = 6
etapes = 0
while n != 1
  n = n.even? ? n / 2 : 3 * n + 1
  etapes += 1
end
puts etapes
```

```sortie
8
```

Une boucle `while` dont on ne connaît pas le nombre de tours devient une
fonction récursive. Ce qui changeait à chaque tour (`n` et `etapes`)
devient ses **paramètres**, et la condition d'arrêt devient un cas :

```rythme
fn etapes(1, compte) = compte
fn etapes(n, compte) = etapes(if is_even(n) then n // 2 else 3 * n + 1, compte + 1)

print(etapes(6, 0))
```

```sortie
8
```

L'appel récursif est la dernière chose que fait la fonction : c'est un
appel terminal, aussi léger qu'un tour de boucle. Pour le voir
fonctionner, essayez `rythme etapes` (voir [les étapes](etapes.md)).

## Erreurs fréquentes

### Appeler une fonction sans parenthèses dans une expression

```rythme norun
xs = [1, 2, 3]
n = len xs
print(n)
```

```text
erreur[E001]: inattendu ici : le nom `xs`
 --> main.ry:2:9
  |
2 | n = len xs
  |         ^^ l'instruction devrait se terminer avant ceci
  |
  = aide: mettez chaque instruction sur sa propre ligne
  = en savoir plus : `rythme explain E001`
```

Comme en Ruby, `puts "salut"` marche au début d'une ligne. Mais dans une
expression, comme après `=`, les parenthèses sont obligatoires :
`n = len(xs)`, ou `n = xs.size`.

### Écrire un if à la fin de la ligne

```rythme norun
ok = true
puts "oui" if ok
```

```text
erreur[E001]: inattendu ici : le mot-clé `if`
 --> main.ry:2:12
  |
2 | puts "oui" if ok
  |            ^^ l'instruction devrait se terminer avant ceci
  |
  = aide: mettez chaque instruction sur sa propre ligne
  = en savoir plus : `rythme explain E001`
```

Le `if` de fin de ligne de Ruby n'existe pas. Écrivez
`if ok then puts "oui" else ()`.

### Modifier un champ

```rythme norun
compte = {solde: 100}
compte.solde = 150
```

```text
erreur[E001]: impossible de donner une valeur à ceci
 --> main.ry:2:1
  |
2 | compte.solde = 150
  | ^^^^^^^^^^^^ ceci n'est pas un nom
  |
2 | compte.solde = 150
  |              - `=` donne un nom à une valeur
  |
  = aide: les enregistrements ne peuvent pas être modifiés ; construisez-en un nouveau : `nouveau = {..ancien, solde: valeur}`
  = en savoir plus : `rythme explain E001`
```

Construisez un nouvel enregistrement : `compte2 = {..compte, solde: 150}`.

### Écrire nil

```rythme norun
x = nil
```

```text
erreur[E001]: `nil` n'est pas utilisé en Rythme
 --> main.ry:1:5
  |
1 | x = nil
  |     ^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur
  = en savoir plus : `rythme explain E001`
```

`None` remplace `nil`, et `Some(x)` enveloppe une valeur présente.

## Testez-vous

**Question 1 :** comment traduire `prix.map { |p| p * 2 }.sum` en Rythme ?

> **Réponse**
>
> Tel quel : `prix.map { |p| p * 2 }.sum` marche en Rythme. Dans le style
> de ce cours : `prix |> map(fn(p) = p * 2) |> sum`.

**Question 2 :** en Ruby, `scores << 15` ajoute 15 au tableau. Que faut-il
écrire en Rythme ?

> **Réponse**
>
> `scores2 = push(scores, 15)`. `push` ne modifie pas `scores` : il renvoie
> une nouvelle liste, à laquelle on donne un nom.

**Question 3 :** que renvoie `find([1, 3, 5], is_even)`, et que renverrait
l'équivalent Ruby ?

> **Réponse**
>
> `None` en Rythme, `nil` en Ruby. La différence : en Rythme, une valeur
> trouvée serait `Some(x)`, et il faut ouvrir la boîte avant de s'en
> servir.

## En résumé

- Blocs, méthodes avec un point, noms en `?`, `puts` et `#{...}` marchent
  comme en Ruby ; `rythme help ruby` résume le reste.
- Pas de boucles : `each`, `map`, `filter`, `fold`, ou une fonction
  récursive dont les paramètres remplacent les variables de la boucle.
- Pas de mutation : on construit une nouvelle valeur (`push`,
  `{..e, champ: v}`) avec un nouveau nom.
- Pas de `nil` : une `Option` ; pas d'exceptions : un `Result`, transmis
  avec `?`.
- `if` a toujours un `else`, et la dernière ligne d'une fonction est son
  résultat, sans `return`.

Suite : [organiser un projet](projet.md). Pour revoir Ruby lui-même, voir
[l'introduction à Ruby](../ruby/introduction.md).
