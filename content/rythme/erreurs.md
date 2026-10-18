# Gérer les erreurs

Rythme n'a ni `null` ni exceptions : cette page montre comment une valeur absente devient une `Option`, comment un échec devient un `Result`, et comment lire les messages d'erreur.

> **Objectifs :**
>
> - vous saurez utiliser une `Option` pour une valeur qui peut manquer ;
> - vous saurez utiliser un `Result` pour une opération qui peut échouer ;
> - vous saurez ouvrir une `Option` ou un `Result` avec `match`, `unwrap_or` et `?` ;
> - vous saurez transformer ces valeurs avec `map`, `and_then`, `ok_or` et `map_err` ;
> - vous saurez lire un message d'erreur de Rythme et trouver son explication.

## Pas de null, pas d'exceptions

Une [erreur](../debuter/glossaire.md#erreur), c'est ce qui empêche un
programme de faire ce qu'on attend. Dans beaucoup de langages, une valeur
peut être « nulle » ([null](../debuter/glossaire.md#null)), et une erreur
peut surgir de n'importe où, comme une **exception** qui interrompt tout.

En Rythme, non. Les problèmes possibles sont des **valeurs** ordinaires,
que vous traitez comme les autres :

- une valeur qui peut manquer est une `Option` : `Some(valeur)` ou `None` ;
- une opération qui peut échouer donne un `Result` : `Ok(valeur)` ou
  `Err(erreur)`.

Pensez à un colis : avant de vous servir du contenu, vous ouvrez la boîte,
et vous regardez si elle est pleine ou vide. Rythme ne vous laisse pas
oublier de regarder.

## Option : une valeur qui peut manquer

`first` donne le premier élément d'une liste... s'il y en a un. `find`
donne le premier élément qui convient... s'il y en a un :

```rythme
print(first([10, 20]), first([]))
print(find([1, 4, 7], is_even), find([1, 3], is_even))
```

```sortie
Some(10) None
Some(4) None
```

`Some(10)` est une boîte qui contient 10. `None` est une boîte vide.

On traite les deux cas avec `match` :

```rythme
fn decrire(liste) = match first(liste)
  Some(x) -> "commence par {x}"
  None -> "liste vide"
end

print(decrire([5, 6]), "/", decrire([]))
```

```sortie
commence par 5 / liste vide
```

## Result : une opération qui peut échouer

`parse_int` lit un nombre dans un texte. Le texte peut ne pas être un
nombre : il donne donc un `Result`, `Ok(nombre)` ou `Err(message)`.

```rythme
print(parse_int("42"), parse_int("quarante"))
```

```sortie
Ok(42) Err("\"quarante\" n'est pas un nombre entier")
```

`Option` dit seulement « rien » ; `Result` dit aussi **pourquoi** ça a
échoué.

Vos fonctions peuvent aussi renvoyer un `Result` :

```rythme
fn diviser(a, b) = if b == 0 then Err("division par zéro") else Ok(a / b)

fn montrer(resultat) = match resultat
  Ok(v) -> "résultat : {v}"
  Err(e) -> "erreur : {e}"
end

print(montrer(diviser(10, 4)))
print(montrer(diviser(1, 0)))
```

```sortie
résultat : 2.5
erreur : division par zéro
```

## Une valeur par défaut avec unwrap_or

`unwrap_or` donne la valeur contenue, ou une valeur par défaut si la boîte
est vide ou contient une erreur :

```rythme
notes = dict([["Ada", 19]])
print(unwrap_or(get(notes, "Ada"), 0), unwrap_or(get(notes, "Eve"), 0))
print(parse_int("12") |> unwrap_or(0), parse_int("??") |> unwrap_or(0))
```

```sortie
19 0
12 0
```

> **Attention :** `unwrap(x)` donne la valeur, mais arrête le programme sur
> `None` ou `Err` (erreur E013). Préférez `unwrap_or` ou `match`.

## L'opérateur ? pour transmettre l'erreur

Dans une fonction, `?` après une valeur veut dire :

- si c'est `Ok(x)` ou `Some(x)`, continue avec `x` ;
- si c'est `Err(...)` ou `None`, **arrête la fonction** et renvoie cette
  valeur.

```rythme
fn additionner(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(additionner("1", "2"))
print(additionner("1", "deux"))
```

```sortie
Ok(3)
Err("\"deux\" n'est pas un nombre entier")
```

Sans `?`, il faudrait un `match` pour chaque lecture. Avec `?`, le chemin
normal se lit d'un trait, et l'erreur remonte toute seule, comme un colis
abîmé qu'on renvoie à l'expéditeur.

> **Note :** `?` ne marche que dans une fonction (ou un test). La fonction
> renvoie alors elle-même un `Result` ou une `Option` : d'où le `Ok(x + y)`
> à la fin.

## Transformer sans ouvrir la boîte

Ces fonctions travaillent sur le contenu, et laissent `None` ou `Err` tels
quels :

| Fonction | Ce qu'elle fait |
|----------|-----------------|
| `map(x, f)` | applique `f` à la valeur dans `Some` ou `Ok` |
| `and_then(x, f)` | une étape de plus qui peut échouer : `f` donne une Option ou un Result |
| `ok_or(option, erreur)` | transforme une Option en Result : `None` devient `Err(erreur)` |
| `map_err(result, f)` | change le message d'un `Err` |

```rythme
fn positif(n) = if n > 0 then Ok(n) else Err("pas positif")

print(Some(3) |> map(fn(x) = x * 10), None |> map(fn(x) = x * 10))
print(parse_int("8") |> and_then(positif), parse_int("-2") |> and_then(positif))
print(first([]) |> ok_or("liste vide"))
print(parse_int("x") |> map_err(fn(e) = "mauvaise saisie"))
```

```sortie
Some(30) None
Ok(8) Err("pas positif")
Err("liste vide")
Err("mauvaise saisie")
```

`ok_or` est pratique avec `?` : dans une fonction qui renvoie un `Result`,
`first(liste) |> ok_or("liste vide")?` donne le premier élément, ou arrête
la fonction avec un message clair.

## D'autres outils

| Fonction | Ce qu'elle fait |
|----------|-----------------|
| `is_some(x)`, `is_none(x)` | l'Option contient-elle une valeur ? |
| `is_ok(x)`, `is_err(x)` | le Result est-il un succès ? |
| `fail(message)` | arrête le programme avec un message |

```rythme
print(is_some(first([1, 2])), is_none(first([])))
print(is_ok(parse_int("7")), is_err(parse_int("x")))
```

```sortie
true true
true true
```

Gardez `fail` pour les situations impossibles, quand le programme ne peut
vraiment pas continuer.

## Des messages d'erreur qui aident

Quand quelque chose ne va pas, Rythme dit **où**, **pourquoi**, et
**comment corriger**. Par exemple, ce programme ajoute un nombre à un
texte :

```rythme norun
age = 36
print("âge : " + age)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Text et Int
 --> main.ry:2:16
  |
2 | print("âge : " + age)
  |                ^ Text "âge : " + Int
  |
  = aide: pour construire un texte contenant une valeur, utilisez des accolades : "total : {x}"
      ou convertissez d'abord la valeur : text(x)
  = en savoir plus : `rythme explain E005`
```

Vous y lisez :

1. le **code** de l'erreur (`E005`) et ce qui s'est passé ;
2. le fichier, la ligne et la colonne (`main.ry:2:16`) ;
3. la ligne de code, avec l'endroit exact marqué par `^` ;
4. une **aide** pour corriger ;
5. la commande pour en savoir plus.

Cette erreur est trouvée **avant** l'exécution : rien n'a été lancé. Un
message qui commence par `avertissement` signale un problème probable, mais
le programme tourne quand même.

## Expliquer un code d'erreur

Chaque code a une explication détaillée, avec des exemples :

```sh
rythme explain E005
```

```text
E005 — Mauvaise sorte de valeur

L'opération ne fonctionne pas avec cette sorte de valeur. Rythme ne convertit
jamais les valeurs en silence : "1" + 1 est une erreur, ni "11" ni 2.

Faux :   "âge : " + 36
Juste :  "âge : {36}"             (insérer la valeur dans le texte)
Juste :  "âge : " + text(36)      (la convertir d'abord)
```

Voici les codes les plus fréquents au début :

| Code | Signification |
|------|---------------|
| E001 | erreur de syntaxe (souvent un mot d'un autre langage : `for`, `return`, `null`...) |
| E003 | nom inconnu (souvent une faute de frappe : Rythme propose le bon nom) |
| E004 | mauvais nombre d'arguments |
| E005 | mauvaise sorte de valeur |
| E006 | aucun cas ne correspond |
| E007 | indice hors limites |
| E011 | nom déjà défini (une valeur ne change pas) |
| E019 | `if` sans `else` |

## Erreurs fréquentes

### Chercher null

```rythme norun
nom = null
```

```text
erreur[E001]: `null` n'est pas utilisé en Rythme
 --> main.ry:1:7
  |
1 | nom = null
  |       ^^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de null : utilisez `None` pour « pas de valeur » et `Some(x)` pour une valeur
  = en savoir plus : `rythme explain E001`
```

Une valeur qui peut manquer s'écrit `None`, et `Some("Ada")` quand elle est
là.

### Calculer avec une Option sans l'ouvrir

```rythme norun
notes = [12, 15]
print(first(notes) + 1)
```

```text
erreur[E005]: impossible d'utiliser `+` avec Option et Int
 --> main.ry:2:20
  |
2 | print(first(notes) + 1)
  |                    ^ Option + Int 1
  |
  = en savoir plus : `rythme explain E005`
```

`first(notes)` est la boîte `Some(12)`, pas le nombre 12. Ouvrez-la :
`unwrap_or(first(notes), 0) + 1`, ou avec `match`.

### Utiliser ? hors d'une fonction

```rythme norun
n = parse_int("douze")?
print(n)
```

```text
erreur[E016]: `?` ne peut être utilisé que dans une fonction
 --> main.ry:1:5
  |
1 | n = parse_int("douze")?
  |     ^^^^^^^^^^^^^^^^^^^
  |
  = aide: en dehors d'une fonction, traitez la valeur avec `match` ou `unwrap_or`
  = en savoir plus : `rythme explain E016`
```

`?` renvoie l'erreur à la fonction qui appelle ; au premier niveau du
programme, il n'y en a pas. Utilisez `match`, ou `unwrap_or`.

### Ouvrir de force avec unwrap

```rythme norun
print(unwrap(parse_int("douze")))
```

```text
erreur[E013]: `unwrap` a trouvé une erreur : "douze" n'est pas un nombre entier
 --> main.ry:1:7
  |
1 | print(unwrap(parse_int("douze")))
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = aide: traitez l'erreur avec `match`, transmettez-la avec `?`, ou donnez une valeur par défaut : `unwrap_or(x, defaut)`
  = en savoir plus : `rythme explain E013`
```

`unwrap` suppose que tout va bien, et arrête le programme sinon. Prévoyez
le cas d'échec : `parse_int("douze") |> unwrap_or(0)`.

## Testez-vous

**Question 1 :** que donnent `get([1, 2], 5)` et `get([1, 2], 0)` ?

> **Réponse**
>
> `None` et `Some(1)`. La position 5 n'existe pas : la boîte est vide.

**Question 2 :** quand utiliser `Result` plutôt qu'`Option` ?

> **Réponse**
>
> Quand on veut dire **pourquoi** ça a échoué : `Err("fichier
> introuvable")` porte un message, `None` ne dit rien.

**Question 3 :** que renvoie `additionner("5", "x")`, avec la fonction
`additionner` de cette page ?

> **Réponse**
>
> `Err("\"x\" n'est pas un nombre entier")`. Le premier `?` laisse passer
> 5, le second rencontre une erreur et la renvoie aussitôt.

## En résumé

- Une valeur qui peut manquer est une `Option` : `Some(x)` ou `None`.
- Une opération qui peut échouer donne un `Result` : `Ok(x)` ou `Err(e)`.
- On les ouvre avec `match`, ou avec `unwrap_or` pour une valeur par
  défaut ; `map`, `and_then`, `ok_or` et `map_err` les transforment.
- Dans une fonction, `?` continue avec la valeur, ou renvoie l'erreur.
- Chaque message d'erreur dit où, pourquoi et comment corriger ;
  `rythme explain CODE` en dit plus.

Suite : [organiser son code en modules](modules.md). Comparez avec les erreurs
en [Ruby](../ruby/erreurs.md) et en [Rust](../rust/erreurs.md).
