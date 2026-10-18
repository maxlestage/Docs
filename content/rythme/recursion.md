# La récursion

Rythme n'a pas de boucles : pour répéter, une fonction s'appelle elle-même, et cette page montre comment le faire sans se perdre.

> **Objectifs :**
>
> - vous saurez ce qu'est une fonction récursive, et pourquoi elle s'arrête ;
> - vous saurez écrire un cas de base et un cas récursif ;
> - vous saurez parcourir une liste avec le motif `[x, ..reste]` ;
> - vous saurez utiliser un accumulateur pour aller aussi loin que vous voulez ;
> - vous saurez quand préférer `map`, `filter` ou `fold`.

## Pourquoi pas de boucles ?

Dans beaucoup de langages, on répète avec une
[boucle](../debuter/glossaire.md#boucle) `for` ou `while`. Une boucle
modifie un compteur à chaque tour : `i` vaut 0, puis 1, puis 2...

En Rythme, une valeur ne change jamais. Un compteur qui change n'existe
donc pas. Il reste deux façons de répéter :

- les fonctions de listes, `map`, `filter`, `fold`, `each`, vues dans
  [penser fonctionnel](fonctionnel.md) ;
- la [récursion](../debuter/glossaire.md#recursion) : une fonction qui
  s'appelle elle-même.

La récursion est l'outil le plus général : tout ce que fait une boucle, elle
sait le faire.

## Une fonction qui s'appelle elle-même

Pensez aux poupées russes. Pour arriver à la plus petite, vous ouvrez une
poupée ; à l'intérieur, il y a une poupée un peu plus petite, que vous
ouvrez de la même façon ; et ainsi de suite, jusqu'à la poupée qui ne
s'ouvre pas.

```rythme
fn ouvrir(0) = print("la plus petite poupée !")
fn ouvrir(n)
  print("j'ouvre la poupée {n}")
  ouvrir(n - 1)
end

ouvrir(3)
```

```sortie
j'ouvre la poupée 3
j'ouvre la poupée 2
j'ouvre la poupée 1
la plus petite poupée !
```

`ouvrir(3)` affiche une ligne, puis appelle `ouvrir(2)`, qui appelle
`ouvrir(1)`, qui appelle `ouvrir(0)`. Là, la première définition
s'applique, et tout s'arrête.

## Cas de base et cas récursif

Toute fonction récursive a deux parties :

- le **cas de base** : la situation si simple qu'on connaît la réponse
  sans s'appeler (la poupée qui ne s'ouvre pas) ;
- le **cas récursif** : on fait un petit pas, puis on s'appelle sur un
  problème **plus petit**.

Le cas récursif doit toujours se rapprocher du cas de base. Sinon, la
fonction ne s'arrête jamais.

```rythme
fn decompte(0) = print("Décollage !")
fn decompte(n)
  print(n)
  decompte(n - 1)
end

decompte(3)
```

```sortie
3
2
1
Décollage !
```

Ici, `n - 1` rapproche de 0 à chaque appel.

## Une définition par cas

La récursion se lit comme une définition de mathématiques. La factorielle
de `n` (notée `n!`) est le produit `1 × 2 × ... × n` :

- `0!` vaut 1 ;
- `n!` vaut `n × (n - 1)!`.

En Rythme, on écrit exactement cela, une ligne par cas :

```rythme
fn factorielle(0) = 1
fn factorielle(n) = n * factorielle(n - 1)

print(factorielle(5), factorielle(20))
```

```sortie
120 2432902008176640000
```

Les définitions sont essayées **dans l'ordre** (voir
[les fonctions](fonctions.md#definir-par-cas)) : le cas de base vient donc
en premier. La même idée donne la puissance :

```rythme
fn puissance(x, 0) = 1
fn puissance(x, n) = x * puissance(x, n - 1)

print(puissance(2, 10), puissance(3, 4))
```

```sortie
1024 81
```

## Parcourir une liste

Une liste se découpe naturellement en deux morceaux : son premier élément,
et **le reste**, qui est lui-même une liste, plus courte. Le motif
`[x, ..reste]` fait ce découpage. Le cas de base est la liste vide `[]`.

```rythme
fn longueur([]) = 0
fn longueur([_, ..reste]) = 1 + longueur(reste)

fn somme([]) = 0
fn somme([x, ..reste]) = x + somme(reste)

print(longueur([4, 8, 15]), somme([4, 8, 15]))
```

```sortie
3 27
```

Lisez `somme` à voix haute : « la somme d'une liste vide est 0 ; la somme
d'une liste qui commence par `x` est `x` plus la somme du reste ».

Une fonction récursive peut aussi construire une nouvelle liste :

```rythme
fn doubler([]) = []
fn doubler([x, ..reste]) = [x * 2, ..doubler(reste)]

fn positifs([]) = []
fn positifs([x, ..reste]) if x > 0 = [x, ..positifs(reste)]
fn positifs([_, ..reste]) = positifs(reste)

print(doubler([1, 2, 3]), positifs([3, -1, 4, -5]))
```

```sortie
[2, 4, 6] [3, 4]
```

`positifs` utilise une garde, `if x > 0` : on garde `x` seulement s'il est
positif. Le motif `[x, ..reste]` est détaillé dans
[le filtrage par motif](pattern-matching.md).

## Regarder le calcul pas à pas

`rythme etapes` montre chaque étape d'un calcul (voir
[rythme etapes](etapes.md)). Avec ce fichier `f.ry` :

```rythme norun
fn factorielle(0) = 1
fn factorielle(n) = n * factorielle(n - 1)
print(factorielle(3))
```

```sh
rythme etapes f.ry
```

```text
print(factorielle(3))
  → print(3 * factorielle(2))
  → print(3 * (2 * factorielle(1)))
  → print(3 * (2 * (1 * factorielle(0))))
  → print(3 * (2 * (1 * 1)))
  → print(3 * (2 * 1))
  → print(3 * 2)
  → print(6)
6
  → ()
```

On voit les multiplications **en attente** s'empiler vers la droite, puis
se calculer une fois le cas de base atteint. Chaque appel attend la réponse
du suivant : c'est la [pile](../debuter/glossaire.md#pile-et-tas) des
appels.

## Les accumulateurs

Cette pile a une limite. Avec une liste d'un million d'éléments, `somme`
attendrait un million de réponses à la fois : c'est trop (voir les erreurs
fréquentes plus bas).

La solution : porter le résultat **avec soi**, dans un paramètre de plus,
l'**accumulateur**. C'est comme compter des moutons en gardant le total dans
sa tête, au lieu de noter chaque mouton sur une pile de papiers.

```rythme
fn somme([], total) = total
fn somme([x, ..reste], total) = somme(reste, total + x)

print(somme([4, 8, 15], 0))
print(somme(1..1000000, 0))
```

```sortie
27
500000500000
```

Le calcul pas à pas montre la différence : rien n'attend plus, le total
avance à chaque appel.

```text
somme([1, 2, 3], 0)
  → somme([2, 3], 1)
  → somme([3], 3)
  → somme([], 6)
  → 6
```

## Les appels terminaux

Pourquoi l'accumulateur marche-t-il si loin ? Parce que l'appel
`somme(reste, total + x)` est la **toute dernière chose** que fait la
fonction. C'est un **appel terminal** : Rythme n'a rien à retenir, et il
peut répéter cet appel des millions de fois sans remplir la pile.

| Écriture | Appel terminal ? | Pourquoi |
|----------|------------------|----------|
| `x + somme(reste)` | non | il reste à faire `x + ...` après l'appel |
| `n * factorielle(n - 1)` | non | il reste la multiplication |
| `somme(reste, total + x)` | oui | rien ne se passe après l'appel |
| `if n == 0 then 1 else compte(n - 1)` | oui | l'appel est le résultat d'une branche |

Pour ne pas obliger l'utilisateur à donner le 0 de départ, cachez
l'accumulateur dans une petite fonction :

```rythme
fn somme_depuis([], total) = total
fn somme_depuis([x, ..reste], total) = somme_depuis(reste, total + x)
fn somme(liste) = somme_depuis(liste, 0)

print(somme([1, 2, 3]))
```

```sortie
6
```

## Quand préférer map, filter et fold

Beaucoup de récursions suivent le même modèle. Rythme les a déjà écrites
pour vous, sans limite de taille :

| Récursion qui... | Fonction toute prête |
|------------------|----------------------|
| transforme chaque élément | `map(liste, f)` |
| garde certains éléments | `filter(liste, f)` |
| combine tout en une valeur | `fold(liste, depart, f)` |
| additionne | `sum(liste)` |

```rythme
print(map([1, 2, 3], fn(x) = x * 2))
print(filter([3, -1, 4, -5], fn(x) = x > 0))
print(fold(1..1000000, 0, fn(total, x) = total + x), sum(1..1000000))
```

```sortie
[2, 4, 6]
[3, 4]
500000500000 500000500000
```

Préférez ces fonctions quand elles conviennent : elles sont plus courtes et
disent tout de suite ce qu'elles font. Gardez la récursion pour le reste :
un calcul qui s'arrête au milieu, une structure en arbre, une définition
mathématique.

## Erreurs fréquentes

### Oublier le cas de base

```rythme norun
fn factorielle(n) = n * factorielle(n - 1)
print(factorielle(5))
```

```text
erreur[E014]: trop d'appels imbriqués (962627 appels en cours)
 --> main.ry:1:25
  |
1 | fn factorielle(n) = n * factorielle(n - 1)
  |                         ^^^^^^^^^^^^^^^^^^ cet appel va trop loin
  |
  = aide: une fonction récursive qui ne s'arrête jamais ? Vérifiez qu'elle atteint toujours un cas de base.
      Sinon, faites de l'appel récursif la toute dernière chose que fait la fonction
      (appel terminal), par exemple avec un accumulateur :
        fn somme([], acc) = acc
        fn somme([x, ..reste], acc) = somme(reste, acc + x)
      ou utilisez `fold`, `map`, `filter`, qui n'ont pas de limite.
  = appels en cours (le plus récent d'abord) :
      dans `factorielle` (×40) appelée en main.ry:1:25
      … et 962587 autres appels
  = en savoir plus : `rythme explain E014`
```

Sans cas de base, `factorielle` descend à 4, 3, ..., 0, -1, -2... sans fin.
Ajoutez `fn factorielle(0) = 1` **avant** la ligne générale. (Le nombre
d'appels en cours varie selon la machine.)

### Mettre le cas de base en dernier

```rythme norun
fn factorielle(n) = n * factorielle(n - 1)
fn factorielle(0) = 1
print(factorielle(5))
```

```text
avertissement[W004]: cette définition de `factorielle` ne peut jamais être utilisée
 --> main.ry:2:1
  |
2 | fn factorielle(0) = 1
  | ^^^^^^^^^^^^^^^^^
  |
1 | fn factorielle(n) = n * factorielle(n - 1)
  | ----------------- cette définition accepte déjà tous les arguments
  |
  = aide: les définitions sont essayées dans l'ordre : mettez la plus générale en dernier
```

Puis le programme s'arrête avec la même erreur E014. `n` accepte tous les
nombres, 0 compris : le cas `0` n'est jamais atteint. Échangez les deux
lignes.

### Oublier les deux points devant le reste

```rythme norun
fn somme([]) = 0
fn somme([x, reste]) = x + somme(reste)
print(somme([1, 2, 3]))
```

```text
avertissement[W002]: `somme` ne traite pas tous les arguments possibles
 --> main.ry:1:4
  |
1 | fn somme([]) = 0
  |    ^^^^^
  |
  = note: non traité : `[_, _, _, ..]`
  = aide: ajoutez une définition pour ce cas, comme `fn somme([_, _, _, ..]) = ...`

erreur[E006]: aucune définition de `somme` n'accepte ces arguments : somme([1, 2, 3])
 --> main.ry:3:7
  |
3 | print(somme([1, 2, 3]))
  |       ^^^^^^^^^^^^^^^^ appelée ici
  |
1 | fn somme([]) = 0
  | ------------ ne correspond pas
  |
2 | fn somme([x, reste]) = x + somme(reste)
  | -------------------- ne correspond pas
  |
  = aide: ajoutez une définition qui traite ce cas, par exemple une qui se termine par un simple nom ou `_`
  = en savoir plus : `rythme explain E006`
```

`[x, reste]` veut dire « une liste d'exactement deux éléments ». Pour « le
premier, puis tout le reste », écrivez `[x, ..reste]`.

## Testez-vous

**Question 1 :** quelles sont les deux parties de toute fonction
récursive ?

> **Réponse**
>
> Le cas de base, qui répond sans s'appeler, et le cas récursif, qui
> s'appelle sur un problème plus petit.

**Question 2 :** cet appel est-il terminal ?

```rythme norun
fn compter([]) = 0
fn compter([_, ..reste]) = 1 + compter(reste)
```

> **Réponse**
>
> Non : après l'appel `compter(reste)`, il reste à ajouter 1. Avec un
> accumulateur, `fn compter([_, ..reste], n) = compter(reste, n + 1)` est
> terminal.

**Question 3 :** que vaut `f([1, 2, 3])` avec ces définitions ?

```rythme norun
fn f([]) = []
fn f([x, ..reste]) = [..f(reste), x]
```

> **Réponse**
>
> `[3, 2, 1]` : chaque élément est remis à la fin, la liste est donc
> retournée (comme `reverse`).

## En résumé

- Rythme répète avec la récursion : une fonction qui s'appelle elle-même.
- Il faut un cas de base, écrit en premier, et un cas récursif qui se
  rapproche du cas de base.
- `[x, ..reste]` découpe une liste en son premier élément et le reste ;
  `[]` est le cas de base.
- Un accumulateur rend l'appel terminal : la fonction peut alors s'appeler
  des millions de fois.
- Quand `map`, `filter`, `fold` ou `sum` conviennent, préférez-les.
- `rythme etapes` montre le calcul pas à pas.

Suite : [Vos propres types et match](types.md)
