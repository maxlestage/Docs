# Aide-mémoire

Tout Rythme en un coup d'œil : la syntaxe, les fonctions les plus utiles, les commandes, et un tableau pour ceux qui viennent de Ruby.

## Valeurs et noms

| Écrire | Pour |
|--------|------|
| `nom = valeur` | nommer une valeur (pour toujours) |
| `42`, `3.14`, `"texte"`, `true` | Int, Float, Text, Bool |
| `"Bonjour {nom}"`, `"#{nom}"` | insérer une valeur dans un texte |
| `# ...`, `## ...` | commentaire, commentaire de documentation |
| `:ok` | un symbole |
| `()` | la valeur vide |

```rythme
nom = "Ada"
âge = 36
puts "#{nom} a {âge} ans", type_of(âge)
```

```sortie
Ada a 36 ans Int
```

Voir [premiers pas](premiers-pas.md).

## Calculs et comparaisons

| Opérateurs | Sens |
|------------|------|
| `+ - * /` | calcul (`/` donne toujours un Float) |
| `//`, `%`, `**` | division entière, reste, puissance |
| `== != < <= > >=` | comparaisons |
| `and`, `or`, `not` | logique |
| `a..b` | liste des entiers de a à b |
| `x \|> f(a)` | `f(x, a)` |

## Conditions

```rythme
n = 7
print(if n % 2 == 0 then "pair" else "impair")
print(unless n > 10 then "petit" else "grand")
print(match n
  0 -> "zéro"
  1..9 -> "un chiffre"
  _ -> "grand"
end)
```

```sortie
impair
petit
un chiffre
```

Voir [conditions et répétition](conditions-boucles.md).

## Fonctions

```rythme
fn double(x) = x * 2

fn presenter(nom)
  message = "Je suis {nom}"
  message + "."
end

fn fact(0) = 1
fn fact(n) = n * fact(n - 1)

fn signe(n) if n < 0 = "négatif"
fn signe(_) = "positif ou zéro"

triple = fn(x) = x * 3
print(double(4), presenter("Ada"), fact(5), signe(-2), triple(2))
```

```sortie
8 Je suis Ada. 120 négatif 6
```

Voir [les fonctions](fonctions.md).

## Texte

| Fonction | Exemple | Résultat |
|----------|---------|----------|
| `len`, `upper`, `lower` | `upper("été")` | `"ÉTÉ"` |
| `capitalize`, `trim` | `trim("  ok ")` | `"ok"` |
| `split`, `join` | `split("a,b", ",")` | `["a", "b"]` |
| `contains`, `starts_with` | `contains("bonjour", "jour")` | `true` |
| `replace` | `replace("a-b", "-", "+")` | `"a+b"` |
| `chars`, `lines` | `chars("ab")` | `["a", "b"]` |
| `repeat`, `pad_left` | `pad_left("7", 3, "0")` | `"007"` |
| `parse_int`, `parse_float` | `parse_int("42")` | `Ok(42)` |
| `format_number` | `format_number(1234.5, 1, "fr")` | `"1 234,5"` |
| `matches`, `find_all`, `replace_all` | `find_all("a1b22", "\\d+")` | `["1", "22"]` |

```rythme
mots = "le chat dort" |> split |> map(capitalize)
print(join(mots, " "), len(mots), parse_int("x") |> unwrap_or(0))
```

```sortie
Le Chat Dort 3 0
```

Voir [travailler avec du texte](texte.md).

## Collections

| Écrire | Donne |
|--------|-------|
| `[1, 2, 3]`, `xs[0]`, `xs[-1]` | une liste, son premier, son dernier |
| `[..xs, 4]`, `xs + [4]`, `push(xs, 4)` | une nouvelle liste avec 4 en plus |
| `{nom: "Ada", age: 36}`, `r.nom` | un enregistrement, un champ |
| `{..r, age: 37}` | une copie avec un champ changé |
| `dict([["a", 1]])`, `d["a"]`, `put(d, "b", 2)` | un dictionnaire |
| `set([1, 2, 2])` | un ensemble : `set(1, 2)` |

Voir [listes, enregistrements et dictionnaires](collections.md).

## Fonctions de listes

| Fonction | Exemple | Résultat |
|----------|---------|----------|
| `map` | `map([1, 2], fn(x) = x * 10)` | `[10, 20]` |
| `filter`, `reject` | `filter(1..6, is_even)` | `[2, 4, 6]` |
| `fold` | `fold([1, 2, 3], 0, fn(a, x) = a + x)` | `6` |
| `each` | `each(xs, print)` | affiche chaque élément |
| `sum`, `len`, `max`, `min` | `sum([1, 2, 3])` | `6` |
| `sort`, `reverse`, `unique` | `sort([3, 1, 2])` | `[1, 2, 3]` |
| `sort_by`, `max_by` | `sort_by(["ccc", "a"], len)` | `["a", "ccc"]` |
| `first`, `find`, `get` | `first([])` | `None` |
| `take`, `drop` | `take([1, 2, 3], 2)` | `[1, 2]` |
| `join`, `split` | `join(["a", "b"], "-")` | `"a-b"` |

## Style fonctionnel et blocs

```rythme
total = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum

pareil = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map { |x| x * x }
  .sum

[1, 2].each do |x|
  puts "élément #{x}"
end
print(total, pareil)
```

```sortie
élément 1
élément 2
56 56
```

Voir [penser fonctionnel](fonctionnel.md).

## Types et match

```rythme
type Forme = Cercle(rayon) | Carre(cote)

fn aire(forme) = match forme
  Cercle(r) -> round(pi * r * r, 2)
  Carre(c) -> c * c
end

fn decrire([]) = "vide"
fn decrire([x, ..reste]) = "{x} puis {len(reste)} autres"

print(aire(Cercle(1)), aire(Carre(3)), decrire([7, 8, 9]))
```

```sortie
3.14 9 7 puis 2 autres
```

Voir [vos propres types et match](types.md).

## Erreurs

| Écrire | Sens |
|--------|------|
| `Some(x)`, `None` | une valeur présente, absente (Option) |
| `Ok(x)`, `Err(e)` | un succès, un échec (Result) |
| `valeur?` | continuer avec `x`, ou renvoyer `None` / `Err` |
| `unwrap_or(x, defaut)` | la valeur, ou une valeur par défaut |
| `rythme explain E005` | expliquer un code d'erreur |

```rythme
fn somme_textes(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(somme_textes("2", "3"), somme_textes("2", "trois") |> is_err)
print(first([]) |> unwrap_or(0))
```

```sortie
Ok(5) true
0
```

Voir [gérer les erreurs](erreurs.md).

## Clavier et fichiers

| Écrire | Donne |
|--------|-------|
| `input("Nom ? ")` | la ligne tapée au clavier (un texte) |
| `args()` | les mots après le nom du fichier |
| `write_file(chemin, texte)` | `Ok(())` : remplace le contenu |
| `append_file(chemin, texte)` | `Ok(())` : ajoute à la fin |
| `read_file(chemin)` | `Ok(texte)` ou `Err(message)` |
| `file_exists(chemin)` | `true` ou `false` |
| `to_json(valeur)`, `parse_json(texte)` | du JSON, et le retour (un Result) |

```rythme
write_file("courses.txt", "pain\nlait")
append_file("courses.txt", "\nœufs")
texte = read_file("courses.txt") |> unwrap_or("")
print(lines(texte), file_exists("courses.txt"))
```

```sortie
["pain", "lait", "œufs"] true
```

Voir [lire et écrire : clavier et fichiers](fichiers.md).

## Tests et modules

```rythme
fn carre(x) = x * x

test "carré de 3" = assert_eq(carre(3), 9)

print(carre(12))
```

```sortie
144
```

```rythme norun
use geometrie              # le fichier geometrie.ry
print(geometrie.aire_carre(4))
```

Voir [projets, modules et tests](projet.md).

## Commandes

| Commande | Ce qu'elle fait |
|----------|-----------------|
| `rythme` | le mode interactif |
| `rythme run f.ry` | lancer un programme |
| `rythme check f.ry` | vérifier sans lancer |
| `rythme test f.ry` | lancer les tests |
| `rythme fmt f.ry` | mettre en forme |
| `rythme doc f.ry` | écrire la documentation |
| `rythme new nom` | créer un projet |
| `rythme help sujet` | l'aide (`rythme help builtins` : toutes les fonctions) |
| `rythme explain E005` | expliquer une erreur |
| `rythme learn` | la visite guidée |
| `rythme exercise` | les 30 exercices |

## Si vous venez de Ruby

| Ruby | Rythme |
|------|--------|
| `def double(x) x * 2 end` | `fn double(x) = x * 2` |
| `x += 1` | `x2 = x + 1` (un nouveau nom) |
| `nil` | `None` (et `Some(x)` pour une valeur) |
| `raise` / `rescue` | `Err(e)` / `match` ou `?` |
| `if x then a end` | `if x then a else b` (le `else` est obligatoire) |
| `case x when 1 then ... end` | `match x` puis `1 -> ...`, fermé par `end` |
| `for x in xs` | `xs.each do \|x\| ... end` |
| `class Point` | `{x: 1, y: 2}` ou `type Point = Point(x, y)` |
| `{ada: 36}` (un hash) | `dict([[:ada, 36]])` ou `{ada: 36}` (un enregistrement) |
| `return x` | la dernière ligne est le résultat |
| `x = len xs` | `x = len(xs)` (parenthèses dans une expression) |
| `xs.map { \|x\| x * 2 }` | pareil |
| `puts "Salut #{nom}"` | pareil |
| `xs.select(&:even?)` | `xs.select(even?)` |
| `gets` | `input("question ")` |
| `File.read("f.txt")` | `read_file("f.txt")` (un Result) |
| `"42".to_i` | `parse_int("42")` (un Result : le texte peut ne pas être un nombre) |

`rythme help ruby` détaille ce qui est pareil et ce qui change. Voir aussi
[l'aide-mémoire de Ruby](../ruby/aide-memoire.md) et
[celui de Rust](../rust/aide-memoire.md).

> **À retenir :** les valeurs ne changent jamais, tout `if` a un `else`, la
> dernière ligne d'une fonction est son résultat, et `rythme help` est
> toujours là.
