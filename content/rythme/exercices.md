# Exercices corrigés

Douze exercices pour vous entraîner sur tout le parcours Rythme, du plus facile au plus difficile, chacun avec un indice et une solution vérifiée.

## Comment faire ces exercices

Chaque exercice a un niveau :

- ★☆☆ facile : quelques lignes, une seule idée ;
- ★★☆ moyen : il faut combiner deux ou trois idées ;
- ★★★ plus dur : prenez votre temps.

Écrivez votre solution dans un fichier, par exemple `exo.ry`, puis lancez-le
avec `rythme run exo.ry`. Ne regardez la solution qu'après avoir essayé :
c'est en cherchant qu'on apprend. Il y a souvent plusieurs bonnes réponses.

## Les bases

### Exercice 1 : Se présenter

Niveau : ★☆☆

Donnez un nom à votre prénom et à votre âge, puis affichez une phrase comme
celle-ci, avec l'âge de l'an prochain calculé par le programme :

```text
Je m'appelle Ada et j'ai 36 ans. L'an prochain, j'aurai 37 ans.
```

> **Indice :** un calcul peut aller entre accolades : `"{age + 1}"`. Voir
> [premiers pas](premiers-pas.md).

> **Solution**
>
> ```rythme
> prenom = "Ada"
> age = 36
> print("Je m'appelle {prenom} et j'ai {age} ans. L'an prochain, j'aurai {age + 1} ans.")
> ```
>
> ```sortie
> Je m'appelle Ada et j'ai 36 ans. L'an prochain, j'aurai 37 ans.
> ```

### Exercice 2 : Pair ou impair

Niveau : ★☆☆

Écrivez une fonction `parite(n)` qui donne `"pair"` ou `"impair"`.

> **Indice :** `n % 2` est le reste de la division par 2. Un `if` a
> toujours un `else`.

> **Solution**
>
> ```rythme
> fn parite(n) = if n % 2 == 0 then "pair" else "impair"
>
> print(parite(4), parite(7), parite(0))
> ```
>
> ```sortie
> pair impair pair
> ```

### Exercice 3 : La table de multiplication

Niveau : ★☆☆

Affichez la table de 7, de `7 x 1 = 7` jusqu'à `7 x 5 = 35`.

> **Indice :** il n'y a pas de boucle `for` : utilisez `1..5` et `each`. Voir
> [conditions et répétition](conditions-boucles.md).

> **Solution**
>
> ```rythme
> 1..5 |> each(fn(i) = print("7 x {i} = {7 * i}"))
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

### Exercice 4 : Le prix du billet

Niveau : ★☆☆

Un billet de cinéma coûte 5 € avant 12 ans, 7 € avant 18 ans et 10 € pour
les autres. Écrivez `prix(age)` avec une définition par cas et des gardes.

> **Indice :** `fn prix(age) if age < 12 = 5`, puis les autres cas en
> dessous. Voir [les gardes](fonctions.md#les-gardes).

> **Solution**
>
> ```rythme
> fn prix(age) if age < 12 = 5
> fn prix(age) if age < 18 = 7
> fn prix(_) = 10
>
> print(prix(8), prix(15), prix(40))
> ```
>
> ```sortie
> 5 7 10
> ```

### Exercice 5 : Additionner jusqu'à n

Niveau : ★★☆

Écrivez une fonction **récursive** `somme(n)` qui additionne les nombres de 1
à `n`. Par exemple, `somme(4)` vaut `1 + 2 + 3 + 4 = 10`.

> **Indice :** deux définitions : `somme(0)` vaut 0, et `somme(n)` vaut
> `n + somme(n - 1)`.

> **Solution**
>
> ```rythme
> fn somme(0) = 0
> fn somme(n) = n + somme(n - 1)
>
> print(somme(4), somme(100))
> print(sum(1..100))
> ```
>
> ```sortie
> 10 5050
> 5050
> ```
>
> La dernière ligne vérifie le résultat avec `sum`, qui fait la même chose.

## Listes et enregistrements

### Exercice 6 : Le bulletin de notes

Niveau : ★☆☆

Avec la liste `notes = [12, 8, 15, 19, 10]`, affichez la moyenne, la
meilleure note et la liste des notes inférieures à 10.

> **Indice :** `sum`, `len`, `max` et `filter`. `/` donne un nombre à
> virgule.

> **Solution**
>
> ```rythme
> notes = [12, 8, 15, 19, 10]
> print("Moyenne :", sum(notes) / len(notes))
> print("Meilleure :", max(notes))
> print("Sous 10 :", filter(notes, fn(n) = n < 10))
> ```
>
> ```sortie
> Moyenne : 12.8
> Meilleure : 19
> Sous 10 : [8]
> ```

### Exercice 7 : Joyeux anniversaire

Niveau : ★★☆

Écrivez `anniversaire(personne)` qui reçoit un enregistrement comme
`{nom: "Ada", age: 36}` et donne un **nouvel** enregistrement avec un an de
plus.

> **Indice :** `{..p, age: ...}` recopie `p` en changeant un champ. Voir
> [mettre à jour un enregistrement](collections.md#mettre-a-jour-un-enregistrement).

> **Solution**
>
> ```rythme
> fn anniversaire(p) = {..p, age: p.age + 1}
>
> ada = {nom: "Ada", age: 36}
> plus_vieille = anniversaire(ada)
> print(ada)
> print(plus_vieille)
> ```
>
> ```sortie
> {nom: "Ada", age: 36}
> {nom: "Ada", age: 37}
> ```
>
> `ada` ne change pas : les valeurs ne changent jamais.

### Exercice 8 : Les carrés des impairs

Niveau : ★★☆

Calculez la somme des carrés des nombres **impairs** de 1 à 10, en
enchaînant les étapes avec `|>`. Faites-le une deuxième fois avec des points
et un bloc `{ |x| ... }`.

> **Indice :** `filter(is_odd)`, puis `map`, puis `sum`. Voir
> [penser fonctionnel](fonctionnel.md).

> **Solution**
>
> ```rythme
> avec_pipe = 1..10
>   |> filter(is_odd)
>   |> map(fn(x) = x * x)
>   |> sum
>
> avec_points = (1..10)
>   .select(odd?)
>   .map { |x| x * x }
>   .sum
>
> print(avec_pipe, avec_points)
> ```
>
> ```sortie
> 165 165
> ```
>
> 1 + 9 + 25 + 49 + 81 = 165.

## Types, erreurs et texte

### Exercice 9 : Le distributeur de boissons

Niveau : ★★☆

Créez un type `Boisson` avec trois variantes : `Cafe`, `The` et
`Jus(fruit)`. Écrivez `decrire(boisson)` qui donne un texte, avec `match`.

> **Indice :** `type Boisson = Cafe | The | Jus(fruit)`. Dans le `match`,
> `Jus(f)` donne le nom `f` au fruit. Voir [vos propres types](types.md).

> **Solution**
>
> ```rythme
> type Boisson = Cafe | The | Jus(fruit)
>
> fn decrire(boisson) = match boisson
>   Cafe -> "un café, 1 €"
>   The -> "un thé, 1 €"
>   Jus(f) -> "un jus de {f}, 2 €"
> end
>
> [Cafe, Jus("pomme"), The] |> each(fn(b) = print(decrire(b)))
> ```
>
> ```sortie
> un café, 1 €
> un jus de pomme, 2 €
> un thé, 1 €
> ```

### Exercice 10 : Chercher un élève

Niveau : ★★☆

Écrivez `note_de(eleves, nom)` qui cherche un élève par son nom dans une
liste d'enregistrements. Elle donne `"Ada : 17"`, ou `"Eve : inconnue"` si
l'élève n'est pas dans la liste.

> **Indice :** `find` donne une `Option` : `Some(eleve)` ou `None`. Traitez
> les deux cas avec `match`. Voir [gérer les erreurs](erreurs.md).

> **Solution**
>
> ```rythme
> eleves = [{nom: "Ada", note: 17}, {nom: "Alan", note: 12}]
>
> fn note_de(eleves, nom) = match find(eleves, fn(e) = e.nom == nom)
>   Some(e) -> "{nom} : {e.note}"
>   None -> "{nom} : inconnue"
> end
>
> print(note_de(eleves, "Ada"))
> print(note_de(eleves, "Eve"))
> ```
>
> ```sortie
> Ada : 17
> Eve : inconnue
> ```

### Exercice 11 : Lire un âge

Niveau : ★★★

Écrivez `lire_age(texte)` qui donne un `Result` :

- `Err(...)` si le texte n'est pas un nombre (laissez `parse_int` donner
  l'erreur) ;
- `Err("âge impossible")` si le nombre est négatif ou plus grand que 150 ;
- `Ok(age)` sinon.

> **Indice :** `age = parse_int(texte)?` lit le nombre, ou arrête la
> fonction avec l'erreur. Ensuite, un `if` donne `Ok` ou `Err`.

> **Solution**
>
> ```rythme
> fn lire_age(texte)
>   age = parse_int(texte)?
>   if age < 0 or age > 150 then Err("âge impossible") else Ok(age)
> end
>
> print(lire_age("36"))
> print(lire_age("-4"))
> print(lire_age("trente"))
> ```
>
> ```sortie
> Ok(36)
> Err("âge impossible")
> Err("\"trente\" n'est pas un nombre entier")
> ```

### Exercice 12 : Les palindromes

Niveau : ★★★

Un **palindrome** se lit pareil dans les deux sens, comme « kayak ». Écrivez
`palindrome?(phrase)` qui ne tient compte ni des majuscules ni des espaces :
« Esope reste ici et se repose » est un palindrome.

> **Indice :** mettez en minuscules avec `lower`, enlevez les espaces avec
> `replace(texte, " ", "")`, puis comparez avec `reverse(texte)`. Voir
> [travailler avec du texte](texte.md).

> **Solution**
>
> ```rythme
> fn palindrome?(phrase)
>   propre = phrase |> lower |> replace(" ", "")
>   propre == reverse(propre)
> end
>
> print(palindrome?("kayak"))
> print(palindrome?("Esope reste ici et se repose"))
> print(palindrome?("Rythme"))
> ```
>
> ```sortie
> true
> true
> false
> ```

## Pour aller plus loin

Rythme contient lui-même **30 autres exercices** gradués, corrigés par des
tests. Lancez-les dans un terminal :

```sh
rythme exercise list                  # la liste des 30 exercices
rythme exercise start 1               # écrit 01-valeurs.ry dans le dossier
rythme exercise check 01-valeurs.ry   # vérifie votre solution
```

Gardez [l'aide-mémoire](aide-memoire.md) ouvert pendant que vous cherchez.
Les autres parcours ont aussi leurs exercices : [en Ruby](../ruby/exercices.md)
et [en Rust](../rust/exercices.md).

Suite : [Exercices corrigés : deuxième série](exercices-2.md)
