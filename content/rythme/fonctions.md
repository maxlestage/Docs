# Les fonctions

En Rythme, presque tout est fonction : cette page montre comment en écrire sur une ligne ou en bloc, par cas, avec des gardes, sans nom, et comment les appeler.

## Une fonction sur une ligne

Le mot-clé `fn`, le nom, les paramètres entre parenthèses, `=`, puis le
résultat :

```rythme
fn double(x) = x * 2
fn aire(largeur, hauteur) = largeur * hauteur

print(double(21), aire(3, 4))
```

```sortie
42 12
```

Il n'y a pas de `return` : ce qui suit `=` **est** le résultat.

## Une fonction en bloc

Quand il faut plusieurs lignes, on enlève le `=` et on ferme avec `end`.
**La dernière ligne donne le résultat** :

```rythme
fn presenter(nom, age)
  annee = 2024 - age
  "{nom} est né(e) vers {annee}."
end

print(presenter("Ada", 36))
```

```sortie
Ada est né(e) vers 1988.
```

Une fonction sans paramètre garde ses parenthèses :

```rythme
fn bonjour() = "bonjour !"
print(bonjour())
```

```sortie
bonjour !
```

## Définir par cas

Une fonction peut avoir plusieurs définitions, écrites les unes sous les
autres. Rythme les essaie **dans l'ordre** et prend la première qui
correspond :

```rythme
fn fib(0) = 0
fn fib(1) = 1
fn fib(n) = fib(n - 1) + fib(n - 2)

print(fib(10))
```

```sortie
55
```

Les paramètres sont des motifs, comme dans `match`. On peut par exemple
reconnaître une liste vide :

```rythme
fn decrire([]) = "une liste vide"
fn decrire([x]) = "un seul élément : {x}"
fn decrire(liste) = "{len(liste)} éléments"

print(decrire([]), "/", decrire([7]), "/", decrire([1, 2, 3]))
```

```sortie
une liste vide / un seul élément : 7 / 3 éléments
```

## Les gardes

Une **garde** ajoute une condition à une définition : `if` après les
paramètres, avant le `=`.

```rythme
fn categorie(age) if age < 13 = "enfant"
fn categorie(age) if age < 18 = "adolescent"
fn categorie(_) = "adulte"

print(categorie(8), categorie(15), categorie(40))
```

```sortie
enfant adolescent adulte
```

`_` veut dire « n'importe quelle valeur, je ne m'en sers pas ».

## Les appels sans parenthèses

Au début d'une ligne, vous pouvez appeler une fonction sans parenthèses,
comme en Ruby :

```rythme
fn saluer(nom, salut) = print("#{salut}, #{nom} !")

saluer "Ada", "Bonjour"
saluer("Alan", "Salut")
```

```sortie
Bonjour, Ada !
Salut, Alan !
```

> **Attention :** cela marche seulement au début d'une instruction (ou d'un
> résultat). Au milieu d'une expression, les parenthèses restent
> obligatoires : `x = len(xs)`, pas `x = len xs`.

## L'appel en style méthode

`valeur.f(args)` est une autre façon d'écrire `f(valeur, args)`. Sans
arguments, les parenthèses sont facultatives :

```rythme
print("salut".upper(), [3, 1, 2].sort, 10.max(20))
```

```sortie
SALUT [1, 2, 3] 20
```

Cela marche aussi avec vos fonctions :

```rythme
fn double(x) = x * 2
print(21.double)
```

```sortie
42
```

## Les noms qui finissent par ?

Une fonction qui répond par oui ou non peut finir par `?`, comme en Ruby :

```rythme
fn majeur?(personne) = personne.age >= 18

ada = {nom: "Ada", age: 36}
print(majeur?(ada), ada.majeur?, even?(4))
```

```sortie
true true true
```

## Les fonctions anonymes

Une fonction sans nom s'écrit `fn(paramètres) = résultat`. C'est une valeur
comme une autre : on peut la ranger dans un nom ou la donner à une autre
fonction.

```rythme
triple = fn(x) = x * 3
print(triple(5))
print([1, 2, 3] |> map(fn(x) = x + 10))
```

```sortie
15
[11, 12, 13]
```

Une fonction anonyme peut utiliser les noms écrits avant elle :

```rythme
taux = 3
print([1, 2, 3] |> map(fn(x) = x * taux))
```

```sortie
[3, 6, 9]
```

## Les fonctions sont des valeurs

Une fonction nommée peut aussi être donnée à une autre fonction, par son
nom, sans parenthèses :

```rythme
fn carre(x) = x * x
fn appliquer_deux_fois(f, x) = f(f(x))

print(appliquer_deux_fois(carre, 3), map([1, 2, 3], carre))
```

```sortie
81 [1, 4, 9]
```

## En résumé

- `fn nom(x) = ...` écrit une fonction sur une ligne ; `fn nom(x) ... end`
  en bloc, où la dernière ligne est le résultat.
- Il n'y a pas de `return`.
- Plusieurs définitions par cas sont essayées dans l'ordre ; une garde
  `if` ajoute une condition.
- `valeur.f(args)` est une autre façon d'écrire `f(valeur, args)`.
- `fn(x) = ...` est une fonction sans nom, qu'on peut donner à `map`.

Suite : [travailler avec du texte](texte.md). Comparez avec les fonctions de
[Ruby](../ruby/fonctions.md) et de [Rust](../rust/fonctions.md).
