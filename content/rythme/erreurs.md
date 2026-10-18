# Gérer les erreurs

Rythme n'a ni `null` ni exceptions : cette page montre comment une valeur absente devient une `Option`, comment un échec devient un `Result`, et comment lire les messages d'erreur.

## Pas de null, pas d'exceptions

Dans beaucoup de langages, une valeur peut être « nulle », et une erreur
peut surgir de n'importe où. En Rythme, non. Les problèmes possibles sont
des **valeurs** ordinaires, que vous traitez comme les autres :

- une valeur qui peut manquer est une `Option` : `Some(valeur)` ou `None` ;
- une opération qui peut échouer donne un `Result` : `Ok(valeur)` ou
  `Err(erreur)`.

## Option : une valeur qui peut manquer

`first` donne le premier élément d'une liste... s'il y en a un :

```rythme
print(first([10, 20]), first([]))
print(find([1, 4, 7], is_even), find([1, 3], is_even))
```

```sortie
Some(10) None
Some(4) None
```

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
nombre : il donne donc un `Result`.

```rythme
print(parse_int("42"), parse_int("quarante"))
```

```sortie
Ok(42) Err("\"quarante\" n'est pas un nombre entier")
```

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

`unwrap_or` donne la valeur contenue, ou une valeur par défaut :

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
normal se lit d'un trait, et l'erreur remonte toute seule.

> **Note :** `?` ne marche que dans une fonction (ou un test).

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
 --> age.ry:2:16
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
2. le fichier, la ligne et la colonne (`age.ry:2:16`) ;
3. la ligne de code, avec l'endroit exact marqué par `^` ;
4. une **aide** pour corriger ;
5. la commande pour en savoir plus.

Cette erreur est trouvée **avant** l'exécution : rien n'a été lancé.

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
| E003 | nom inconnu (souvent une faute de frappe : Rythme propose le bon nom) |
| E004 | mauvais nombre d'arguments |
| E005 | mauvaise sorte de valeur |
| E006 | aucun cas ne correspond |
| E007 | indice hors limites |
| E011 | nom déjà défini (une valeur ne change pas) |
| E019 | `if` sans `else` |

## En résumé

- Une valeur qui peut manquer est une `Option` : `Some(x)` ou `None`.
- Une opération qui peut échouer donne un `Result` : `Ok(x)` ou `Err(e)`.
- On les traite avec `match`, ou avec `unwrap_or` pour une valeur par
  défaut.
- Dans une fonction, `?` continue avec la valeur, ou renvoie l'erreur.
- Chaque message d'erreur dit où, pourquoi et comment corriger ;
  `rythme explain CODE` en dit plus.

Suite : [lire et écrire des fichiers](fichiers.md). Comparez avec les erreurs
en [Ruby](../ruby/erreurs.md) et en [Rust](../rust/erreurs.md).
