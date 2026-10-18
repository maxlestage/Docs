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

Il n'y a pas de `return` : ce qui suit `=` **est** le résultat.

## Paramètres et arguments

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

`_` veut dire « n'importe quelle valeur, je ne m'en sers pas ». Mettez
toujours le cas le plus général en dernier : sinon, il attraperait tout.

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

`map` applique la fonction à chaque élément de la liste. On écrit souvent
une fonction anonyme quand elle ne sert qu'une fois, à cet endroit.

## Une fonction qui se souvient : la closure

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
