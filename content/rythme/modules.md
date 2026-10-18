# Modules : un programme en plusieurs fichiers

Quand un programme grandit, on le range dans plusieurs fichiers : chaque fichier est un **module**, que les autres utilisent avec `use`.

> **Objectifs :**
>
> - utiliser les fonctions d'un autre fichier avec `use nom` ;
> - ranger des fichiers dans des dossiers, avec `use dossier/nom` ;
> - donner un nom plus court à un module avec `use x as y` ;
> - garder une fonction privée à son fichier, avec un nom qui commence par `_`.

## Pourquoi plusieurs fichiers

Un programme de 30 lignes tient très bien dans un seul fichier. À 300
lignes, on s'y perd : on cherche une fonction en faisant défiler l'écran, et
on mélange des choses qui n'ont rien à voir.

On range alors le code par **sujet** : les calculs de géométrie dans un
fichier, l'affichage dans un autre, le programme principal dans un
troisième. Chaque fichier est court, et son nom dit ce qu'il contient.

En Rythme, chaque fichier `.ry` peut servir de **module** : un autre fichier
l'utilise avec le mot-clé `use`.

> **Note :** les exemples de cette page ont besoin de plusieurs fichiers, ils
> ne sont donc pas lancés sur le site. Leurs sorties ont été copiées de vrais
> lancements : créez les fichiers chez vous pour les essayer.

## Un premier module avec use

Créez un dossier avec deux fichiers :

```text
mon_dossier/
├── geometrie.ry
└── main.ry
```

Le fichier `geometrie.ry` contient deux fonctions. Rien de nouveau ici :
c'est un fichier Rythme ordinaire.

```rythme norun
## L'aire d'un rectangle.
fn aire_rectangle(largeur, hauteur) = largeur * hauteur

## L'aire d'un carré.
fn aire_carre(cote) = aire_rectangle(cote, cote)
```

Le fichier `main.ry` les utilise :

```rythme norun
use geometrie

print(geometrie.aire_rectangle(2, 3))
print(geometrie.aire_carre(4))
```

Dans le terminal, depuis `mon_dossier` :

```sh
rythme run main.ry
```

```text
6
16
```

Deux choses à retenir :

- `use geometrie` charge le fichier `geometrie.ry`. On écrit le nom **sans
  guillemets et sans `.ry`**. Le fichier est cherché dans le dossier du
  fichier qui contient le `use`.
- Ensuite, on écrit `geometrie.` devant chaque nom du module :
  `geometrie.aire_carre(4)`. En lisant le code, on sait tout de suite d'où
  vient chaque fonction.

> **À retenir :** `use` s'écrit en haut du fichier, en dehors de toute
> fonction.

## Chaque module a ses propres noms

Comme chaque nom est précédé de son module, deux fichiers peuvent avoir une
fonction du même nom sans se gêner. `geometrie.aire` et `cercle.aire`
seraient deux fonctions différentes, et votre programme peut lui-même
définir une fonction `aire`.

C'est aussi pour cela que `geometrie` seul n'est pas une valeur : c'est
seulement le nom du fichier, qu'on fait toujours suivre d'un point et d'un
nom.

## Un module dans un dossier avec use dossier/nom

Quand les modules deviennent nombreux, on les range dans des dossiers.
Ajoutons un dossier `formes` avec un fichier `cercle.ry` :

```text
mon_dossier/
├── formes/
│   └── cercle.ry
├── geometrie.ry
└── main.ry
```

Voici `formes/cercle.ry` :

```rythme norun
## L'aire d'un cercle, arrondie à 2 décimales.
fn aire(rayon) = round(pi * rayon ** 2, 2)

## Le périmètre d'un cercle, arrondi à 2 décimales.
fn perimetre(rayon) = round(2 * pi * rayon, 2)
```

Dans `main.ry`, on écrit le chemin avec une barre `/`, toujours sans `.ry` :

```rythme norun
use geometrie
use formes/cercle

print(geometrie.aire_carre(3))
print(cercle.aire(1))
print(cercle.perimetre(1))
```

```text
9
3.14
6.28
```

Le module s'appelle comme la **dernière partie** du chemin : `cercle`, et
non `formes/cercle`.

> **Astuce :** pour utiliser un fichier d'un dossier voisin, `..` remonte
> d'un dossier : `use ../outils/calculs`.

## Un nom plus court avec use x as y

`as` donne au module le nom de votre choix :

```rythme norun
use formes/cercle as rond

fn aire(cote) = cote * cote

print(rond.aire(2))
print(aire(2))
```

```text
12.57
4
```

Ici, `rond.aire` est l'aire du cercle, et `aire` la fonction du fichier
lui-même. `as` sert surtout dans deux cas :

- le nom du module est long, et on préfère un nom court ;
- deux fichiers ont le même nom, comme `formes/cercle.ry` et
  `dessins/cercle.ry` : deux modules ne peuvent pas s'appeler tous les deux
  `cercle`, on renomme donc l'un des deux (`use dessins/cercle as dessin`).

## Des valeurs et des types dans un module

Un module ne donne pas que des fonctions : il donne aussi ses **valeurs** et
ses **types** (voir [vos propres types](types.md)). Voici `feu.ry` :

```rythme norun
## Les trois couleurs d'un feu.
type Couleur = Rouge | Orange | Vert

## La couleur qui vient après.
fn suivante(Rouge) = Vert
fn suivante(Vert) = Orange
fn suivante(Orange) = Rouge
```

Dans `main.ry`, les couleurs s'écrivent elles aussi avec le nom du module,
même dans un `match` :

```rythme norun
use feu

fn conseil(couleur) = match couleur
  feu.Rouge -> "on s'arrête"
  feu.Orange -> "on ralentit"
  feu.Vert -> "on passe"
end

print(feu.suivante(feu.Rouge))
print(conseil(feu.Vert))
```

```text
Vert
on passe
```

## Garder un nom privé avec _

Certaines fonctions ne servent qu'à l'intérieur de leur fichier : ce sont
des **outils internes**. Pour les cacher aux autres fichiers, donnez-leur un
nom qui commence par `_`. Voici `points.ry` :

```rythme norun
## Le point de départ, au centre.
origine = {x: 0, y: 0}

## La distance entre deux points {x, y}.
fn distance(a, b) = sqrt(_carre(a.x - b.x) + _carre(a.y - b.y))

fn _carre(x) = x * x
```

`distance` utilise `_carre` sans problème : elles sont dans le même fichier.
Mais `main.ry` ne voit que `origine` et `distance` :

```rythme norun
use points

print(points.origine)
print(points.distance(points.origine, {x: 3, y: 4}))
```

```text
{x: 0, y: 0}
5.0
```

Un nom privé vous laisse libre de le changer plus tard, sans casser les
autres fichiers. Écrire `points._carre(3)` dans `main.ry` est une erreur
(voir plus bas).

## Ce que fait use, pas à pas

Quand vous lancez `rythme run main.ry` :

1. Rythme lit `main.ry`, trouve ses `use`, et charge chaque module, puis les
   modules de ces modules.
2. Il **vérifie** tous les fichiers avant de lancer quoi que ce soit : une
   faute de frappe dans un module est trouvée tout de suite.
3. Chaque module s'exécute **une seule fois**, avant le fichier qui
   l'utilise, même si plusieurs fichiers l'utilisent.

`rythme check main.ry` fait la même vérification sans rien lancer.

> **Astuce :** un module sert à **définir** des choses. Évitez d'y mettre des
> `print` au premier niveau : ils s'afficheraient dès qu'un autre fichier
> l'utilise.

Deux modules ne peuvent pas s'utiliser l'un l'autre (`a.ry` utilise
`b.ry`, qui utilise `a.ry`) : rangez ce dont les deux ont besoin dans un
troisième fichier.

## Erreurs fréquentes

### Oublier le nom du module

Avec `use geometrie`, on écrit `aire_carre(4)` au lieu de
`geometrie.aire_carre(4)` :

```rythme norun
use geometrie

print(aire_carre(4))
```

```text
erreur[E003]: nom inconnu `aire_carre`
 --> main.ry:3:7
  |
3 | print(aire_carre(4))
  |       ^^^^^^^^^^ non défini ici
  |
  = aide: `aire_carre` est défini dans `geometrie.ry` : écrivez `geometrie.aire_carre`
  = en savoir plus : `rythme explain E003`
```

Correction : `print(geometrie.aire_carre(4))`. Si vous avez oublié le `use`
lui-même, le message vous propose aussi de l'ajouter.

### Se tromper dans le nom du fichier

```rythme norun
use geometrei

print(geometrei.aire_carre(4))
```

```text
erreur[E017]: fichier `geometrei.ry` introuvable
 --> main.ry:1:5
  |
1 | use geometrei
  |     ^^^^^^^^^
  |
  = note: cherché : geometrei.ry
  = aide: vouliez-vous dire `use geometrie` ?
  = aide: le chemin est relatif au dossier du fichier qui contient `use`
  = en savoir plus : `rythme explain E017`
```

Correction : `use geometrie`. Vérifiez aussi que le fichier est bien dans le
même dossier que `main.ry` (ou dans le dossier indiqué par le chemin).

### Écrire .ry dans le use

```rythme norun
use geometrie.ry

print(geometrie.aire_carre(4))
```

```text
erreur[E001]: écrivez le nom du fichier sans `.ry`
 --> main.ry:1:14
  |
1 | use geometrie.ry
  |              ^
  |
  = aide: exemple : `use geometrie`
  = en savoir plus : `rythme explain E001`
```

Correction : `use geometrie`, sans `.ry`.

### Utiliser un nom privé

```rythme norun
use points

print(points._carre(3))
```

```text
erreur[E003]: `_carre` est privé au fichier `points.ry`
 --> main.ry:3:7
  |
3 | print(points._carre(3))
  |       ^^^^^^^^^^^^^ privé
  |
  = note: un nom qui commence par `_` ne s'utilise que dans son propre fichier
  = aide: pour l'utiliser depuis d'autres fichiers, renommez-le sans `_` dans `points.ry`
  = en savoir plus : `rythme explain E003`
```

Correction : si d'autres fichiers en ont vraiment besoin, renommez la
fonction `carre` (sans `_`) dans `points.ry`.

## Testez-vous

1. Le fichier `outils/texte.ry` contient une fonction `titre`. Que faut-il
   écrire dans `main.ry` pour l'appeler ?

> **Réponse**
>
> `use outils/texte` en haut du fichier, puis `texte.titre(...)` : le
> module prend le nom de la dernière partie du chemin.

2. Pourquoi écrire `use calculs as c` ?

> **Réponse**
>
> Pour utiliser le module sous un nom plus court : `c.moyenne(...)` au lieu
> de `calculs.moyenne(...)`. C'est aussi la solution quand deux fichiers ont
> le même nom.

3. Dans `banque.ry`, la fonction `_arrondir` peut-elle être utilisée depuis
   `main.ry` ?

> **Réponse**
>
> Non : son nom commence par `_`, elle est privée à `banque.ry`. Les autres
> fonctions de `banque.ry` peuvent l'utiliser, mais `banque._arrondir(...)`
> dans `main.ry` est l'erreur E003.

## En résumé

- `use geometrie` charge `geometrie.ry` (sans guillemets, sans `.ry`) ;
  ses noms s'écrivent ensuite `geometrie.nom`.
- `use formes/cercle` charge `formes/cercle.ry`, utilisé sous le nom
  `cercle`.
- `use formes/cercle as rond` choisit un autre nom.
- Un module donne ses fonctions, ses valeurs et ses types ; un nom qui
  commence par `_` reste privé à son fichier.
- Rythme vérifie tous les fichiers avant de lancer le programme.

Suite : [lire et écrire des fichiers](fichiers.md). Comparez avec les modules
en [Ruby](../ruby/modules.md) et en [Rust](../rust/modules.md).
