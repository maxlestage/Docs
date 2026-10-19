# Les fonctions

En Rythme, presque tout est fonction : cette page montre comment en écrire sur une ligne ou en bloc, par cas, avec des gardes, sans nom, et comment les appeler.

> **Objectifs :**
>
> - vous saurez écrire une fonction sur une ligne ou en bloc, et l'appeler ;
> - vous saurez faire la différence entre paramètre et argument ;
> - vous saurez définir une fonction par cas, avec des motifs et des gardes ;
> - vous saurez écrire une fonction anonyme et la donner à une autre fonction ;
> - vous saurez pourquoi Rythme n'a pas de `return`.

## Une fonction sur une ligne

Une [fonction](../debuter/glossaire.md#fonction) est une petite machine :
on lui donne des valeurs, elle donne un résultat. Comme un grille-pain : du
pain entre, du pain grillé sort.

On l'écrit avec le mot-clé `fn`, le nom, les paramètres entre parenthèses,
`=`, puis le résultat :

```rythme
fn double(x) = x * 2
fn aire(largeur, hauteur) = largeur * hauteur

print(double(21), aire(3, 4))
```

```sortie
42 12
```

Ligne par ligne :

- `fn double(x) = x * 2` crée une fonction nommée `double`. Rien ne s'affiche encore :
  vous avez seulement écrit la recette ;
- `fn aire(largeur, hauteur) = …` crée une deuxième fonction, avec deux paramètres ;
- `print(double(21), aire(3, 4))` appelle les deux fonctions : `x` reçoit 21, donc
  `double(21)` vaut 42 ; `largeur` reçoit 3 et `hauteur` reçoit 4, donc `aire(3, 4)`
  vaut 12 ;
- `print` affiche les deux résultats, séparés par une espace.

Il n'y a pas de `return` : ce qui suit `=` **est** le résultat.

**À vous :** écrivez un programme qui définit une fonction `perimetre`, qui reçoit la
largeur et la hauteur d'un rectangle et donne son périmètre (deux fois la somme des
deux), puis affiche `perimetre(3, 4)`.

> **Solution**
>
> ```rythme
> fn perimetre(largeur, hauteur) = 2 * (largeur + hauteur)
>
> print(perimetre(3, 4))
> ```
>
> ```sortie
> 14
> ```

## Paramètres et arguments

Une fonction sert à refaire le même calcul avec des valeurs différentes. Pensez à un
formulaire : les cases sont toujours les mêmes, mais chacun y écrit ses propres réponses.

Deux mots reviennent sans cesse :

- un [paramètre](../debuter/glossaire.md#parametre) est le nom écrit dans
  la définition : `largeur` et `hauteur` dans `fn aire(largeur, hauteur)` ;
- un [argument](../debuter/glossaire.md#argument) est la valeur donnée à
  l'appel : `3` et `4` dans `aire(3, 4)`.

À l'appel, `largeur` reçoit 3 et `hauteur` reçoit 4, dans l'ordre. Il faut
donner exactement un argument par paramètre.

**Appeler** une fonction, c'est écrire son nom suivi des arguments entre
parenthèses : `double(21)`. On peut utiliser le résultat partout où une
valeur est permise : `double(double(5))` vaut 20.

## Une fonction en bloc

Certains calculs se font en plusieurs étapes, comme une recette : on prépare, puis on
assemble. Une fonction en bloc vous laisse écrire ces étapes l'une sous l'autre.

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

Ligne par ligne :

- `fn presenter(nom, age)` ouvre le bloc, et `end` le ferme : tout ce qui est entre les
  deux fait partie de la fonction ;
- `print(presenter("Ada", 36))` appelle la fonction : `nom` reçoit `"Ada"` et `age`
  reçoit 36 ;
- dans la fonction, `annee = 2024 - age` calcule 1988 et le range dans `annee` ;
- la ligne de texte est la dernière du bloc : c'est le résultat, et `print` l'affiche.

Les noms définis dans une fonction, comme `annee`, n'existent que dans la
fonction : c'est leur [portée](../debuter/glossaire.md#portee). Dehors,
`annee` est inconnu.

Une fonction sans paramètre garde ses parenthèses, à la définition comme à
l'appel :

```rythme
fn bonjour() = "bonjour !"
print(bonjour())
```

```sortie
bonjour !
```

**À vous :** écrivez un programme qui définit une fonction en bloc `prix_ttc(ht)` :
elle calcule une taxe de 20 % (`ht * 20 // 100`), l'ajoute au prix, et donne un texte
comme « 50 € HT font 60 € TTC. ». Affichez `prix_ttc(50)`.

> **Solution**
>
> ```rythme
> fn prix_ttc(ht)
>   taxe = ht * 20 // 100
>   total = ht + taxe
>   "{ht} € HT font {total} € TTC."
> end
>
> print(prix_ttc(50))
> ```
>
> ```sortie
> 50 € HT font 60 € TTC.
> ```

## Définir par cas

Souvent, la réponse dépend de ce que la fonction reçoit : une liste vide ne se décrit
pas comme une liste pleine. Au lieu d'une seule définition remplie de questions, vous
écrivez une définition par cas, comme les lignes d'un tableau de tarifs.

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

Ici, `fib(0)` prend la première définition, `fib(1)` la deuxième, et tous
les autres nombres la troisième. C'est comme une suite de guichets : on va
au premier qui accepte votre demande.

Les paramètres sont des **motifs**, comme dans `match`. On peut par exemple
reconnaître une liste vide, ou une liste d'un seul élément :

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

Un motif reconnaît une forme (0, une liste vide…), mais pas une comparaison comme
« moins de 13 ans ». Pour choisir selon une comparaison, comme un tarif qui dépend de
l'âge, on utilise une garde.

Une **garde** ajoute une condition à une définition : `if` après les
paramètres, avant le `=`. La définition n'est choisie que si la condition
est vraie.

```rythme
fn categorie(age) if age < 13 = "enfant"
fn categorie(age) if age < 18 = "adolescent"
fn categorie(_) = "adulte"

print(categorie(8), categorie(15), categorie(40))
```

```sortie
enfant adolescent adulte
```

Ligne par ligne :

- `categorie(8)` : Rythme essaie le premier cas ; `8 < 13` est vrai, il donne
  `"enfant"` ;
- `categorie(15)` : `15 < 13` est faux, Rythme passe au deuxième cas ; `15 < 18` est
  vrai, il donne `"adolescent"` ;
- `categorie(40)` : les deux gardes sont fausses ; le dernier cas accepte tout, il
  donne `"adulte"` ;
- `print` affiche les trois résultats sur une ligne.

`_` veut dire « n'importe quelle valeur, je ne m'en sers pas ». Mettez
toujours le cas le plus général en dernier : sinon, il attraperait tout.

**À vous :** écrivez un programme qui définit une fonction `meteo`, avec des gardes :
elle donne « froid » sous 10 degrés, « doux » sous 25, et « chaud » sinon. Affichez
`meteo(5)`, `meteo(18)` et `meteo(30)`.

> **Solution**
>
> ```rythme
> fn meteo(degres) if degres < 10 = "froid"
> fn meteo(degres) if degres < 25 = "doux"
> fn meteo(_) = "chaud"
>
> print(meteo(5), meteo(18), meteo(30))
> ```
>
> ```sortie
> froid doux chaud
> ```

## Les appels sans parenthèses

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

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

Cette écriture se lit de gauche à droite, dans l'ordre où l'on pense : on prend une
valeur, puis on lui applique une fonction.

`valeur.f(args)` est une autre façon d'écrire `f(valeur, args)` : la valeur
devant le point devient le premier argument. Sans autres arguments, les
parenthèses sont facultatives :

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

> **Note :** cette partie peut attendre. Lisez-la quand les bases seront solides.

Une fonction qui répond par oui ou non peut finir par `?`, comme en Ruby :

```rythme
fn majeur?(personne) = personne.age >= 18

ada = {nom: "Ada", age: 36}
print(majeur?(ada), ada.majeur?, even?(4))
```

```sortie
true true true
```

`{nom: "Ada", age: 36}` est un enregistrement, et `personne.age` lit son
champ `age` (voir [les collections](collections.md#les-enregistrements)).

## Les fonctions anonymes

Parfois, une fonction ne sert qu'une fois, à un seul endroit. Lui chercher un nom
serait comme écrire une étiquette pour un post-it que vous jetterez tout de suite.

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

Ligne par ligne :

- `triple = fn(x) = x * 3` range une fonction sans nom dans `triple` ; rien n'est encore
  calculé ;
- `triple(5)` l'appelle comme une fonction ordinaire : `x` reçoit 5, le résultat est 15 ;
- `[1, 2, 3] |> map(fn(x) = x + 10)` donne la liste à `map`, avec une fonction écrite
  sur place ; `map` l'appelle sur 1, puis 2, puis 3, et range 11, 12 et 13 dans une
  nouvelle liste.

`map` applique la fonction à chaque élément de la liste. On écrit souvent
une fonction anonyme quand elle ne sert qu'une fois, à cet endroit.

**À vous :** écrivez un programme qui ajoute 5 à chaque prix de la liste
`[10, 20, 30]`, avec `map` et une fonction anonyme, puis affiche la nouvelle liste.

> **Solution**
>
> ```rythme
> print([10, 20, 30] |> map(fn(prix) = prix + 5))
> ```
>
> ```sortie
> [15, 25, 35]
> ```

## Une fonction qui se souvient : la closure

Parfois, une fonction a besoin d'une valeur qui ne fait pas partie de ses paramètres :
un taux, une remise. La closure lui permet d'emporter cette valeur avec elle.

Une fonction anonyme peut utiliser les noms écrits avant elle :

```rythme
taux = 3
print([1, 2, 3] |> map(fn(x) = x * taux))
```

```sortie
[3, 6, 9]
```

Une fonction peut même fabriquer une autre fonction. Celle-ci garde avec
elle la valeur qu'elle a reçue, comme un sac à dos :

```rythme
fn ajouteur(n) = fn(x) = x + n

plus10 = ajouteur(10)
plus1 = ajouteur(1)
print(plus10(5), plus1(5))
```

```sortie
15 6
```

Une telle fonction, qui emporte des valeurs de l'endroit où elle a été
écrite, s'appelle une [closure](../debuter/glossaire.md#closure) (en
français, une « fermeture »). Comme les valeurs ne changent jamais, ce
qu'elle emporte ne change pas non plus.

## Les fonctions sont des valeurs

Cela permet d'écrire une fonction qui sait faire « quelque chose » sans savoir quoi à
l'avance : c'est vous qui lui donnez l'action à faire, comme on donne une recette à un
cuisinier.

Une fonction nommée peut aussi être donnée à une autre fonction, par son
nom, **sans parenthèses** :

```rythme
fn carre(x) = x * x
fn appliquer_deux_fois(f, x) = f(f(x))

print(appliquer_deux_fois(carre, 3), map([1, 2, 3], carre))
```

```sortie
81 [1, 4, 9]
```

Ligne par ligne :

- `carre` donne le carré d'un nombre ;
- `appliquer_deux_fois(f, x)` reçoit une fonction `f` et l'appelle deux fois :
  `f(f(x))` ;
- `appliquer_deux_fois(carre, 3)` : `carre(3)` vaut 9, puis `carre(9)` vaut 81 ;
- `map([1, 2, 3], carre)` applique `carre` à chaque élément : `[1, 4, 9]`.

`carre` est la fonction elle-même ; `carre(3)` est son résultat, 9.

## Erreurs fréquentes

### Écrire return

```rythme norun
fn double(x)
  return x * 2
end
print(double(4))
```

```text
erreur[E001]: `return` n'est pas utilisé en Rythme
 --> main.ry:2:3
  |
2 |   return x * 2
  |   ^^^^^^ vient d'un autre langage
  |
  = aide: pas besoin de `return` : la dernière expression d'une fonction est son résultat.
      Pour arrêter plus tôt avec une erreur, renvoyez `Err(...)` et utilisez `?`.
  = en savoir plus : `rythme explain E001`
```

Enlevez `return` : la dernière ligne, `x * 2`, est déjà le résultat. Ou
écrivez tout sur une ligne : `fn double(x) = x * 2`.

### Oublier un argument

```rythme norun
fn aire(largeur, hauteur) = largeur * hauteur
print(aire(3))
```

```text
erreur[E004]: `aire` attend 2 arguments, mais en a reçu 1
 --> main.ry:2:7
  |
2 | print(aire(3))
  |       ^^^^^^^ 1 argument(s) ici
  |
1 | fn aire(largeur, hauteur) = largeur * hauteur
  |    ---- définie ici
  |
  = aide: utilisation : `aire(largeur, hauteur)`
  = en savoir plus : `rythme explain E004`
```

Rythme n'a pas de valeurs par défaut : donnez un argument par paramètre,
`aire(3, 4)`.

### Oublier les parenthèses d'une fonction sans paramètre

```rythme norun
fn bonjour = "salut"
print(bonjour())
```

```text
erreur[E001]: `(` et les paramètres attendu, mais trouvé `=`
 --> main.ry:1:12
  |
1 | fn bonjour = "salut"
  |            ^ `(` et les paramètres attendu ici
  |
  = aide: même sans paramètres, écrivez les parenthèses : `fn saluer() = print("salut")`
  = en savoir plus : `rythme explain E001`
```

Écrivez `fn bonjour() = "salut"`.

### Oublier le end d'un bloc

```rythme norun
fn presenter(nom)
  "Bonjour {nom}"

print(presenter("Ada"))
```

```text
erreur[E020]: il manque `end` pour fermer ce `fn`
 --> main.ry:1:1
  |
1 | fn presenter(nom)
  | ^^ ce `fn` n'est jamais fermé
  |
5 | 
  | - le code se termine ici
  |
  = aide: ajoutez `end` sur sa propre ligne après la dernière ligne du bloc
  = en savoir plus : `rythme explain E020`
```

Une fonction sans `=` est un bloc : fermez-la avec `end`, sous sa dernière
ligne.

## Testez-vous

**Question 1 :** dans `fn prix(quantite, unitaire) = quantite * unitaire`
et l'appel `prix(3, 2)`, quels sont les paramètres et les arguments ?

> **Réponse**
>
> Les paramètres sont `quantite` et `unitaire` ; les arguments sont `3` et
> `2`. Le résultat est 6.

**Question 2 :** qu'affiche ce programme ?

```rythme norun
fn signe(n) if n < 0 = "négatif"
fn signe(0) = "zéro"
fn signe(_) = "positif"
print(signe(0), signe(-2), signe(5))
```

> **Réponse**
>
> `zéro négatif positif`. Les définitions sont essayées dans l'ordre :
> `0` n'est pas négatif, il prend donc la deuxième.

**Question 3 :** que vaut `ajouteur(3)(4)`, avec
`fn ajouteur(n) = fn(x) = x + n` ?

> **Réponse**
>
> `7`. `ajouteur(3)` donne une fonction qui ajoute 3 ; on l'appelle tout de
> suite avec 4.

## En résumé

- `fn nom(x) = ...` écrit une fonction sur une ligne ; `fn nom(x) ... end`
  en bloc, où la dernière ligne est le résultat.
- Il n'y a pas de `return` ; il faut un argument pour chaque paramètre.
- Plusieurs définitions par cas sont essayées dans l'ordre ; une garde
  `if` ajoute une condition.
- `valeur.f(args)` est une autre façon d'écrire `f(valeur, args)`.
- `fn(x) = ...` est une fonction sans nom, qu'on peut donner à `map` ; elle
  emporte les noms écrits avant elle (une closure).

Suite : [travailler avec du texte](texte.md). Comparez avec les fonctions de
[Ruby](../ruby/fonctions.md) et de [Rust](../rust/fonctions.md).
