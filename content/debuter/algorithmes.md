# Penser comme un programmeur

Avant d'écrire du code, on réfléchit à la méthode : cette page montre comment découper un problème en petites étapes, l'écrire en français, puis le traduire en Ruby, en Rust et en Rythme.

> **Objectifs :**
>
> - savoir ce qu'est un algorithme ;
> - découper un problème en petites étapes ;
> - écrire une méthode en français, en pseudo-code ;
> - utiliser des variables, des conditions et des répétitions ;
> - traduire un pseudo-code dans les trois langages ;
> - vérifier un algorithme à la main, avec un tableau de valeurs.

## Qu'est-ce qu'un algorithme

Un **algorithme** est une suite d'étapes précises qui résout un problème.
Une recette de cuisine, l'itinéraire pour aller chez un ami, la notice d'un
meuble : ce sont des algorithmes.

Un bon algorithme a trois qualités :

- chaque étape est **claire** : il n'y a qu'une façon de la comprendre ;
- les étapes sont dans le **bon ordre** ;
- il **s'arrête** et donne un résultat.

Un programme est un algorithme écrit dans un langage de programmation. La
partie difficile, c'est souvent l'algorithme, pas le langage.

> **À retenir :** d'abord la méthode, ensuite le code.

## Découper un problème en petites étapes

Un gros problème fait peur. Découpé en petits morceaux, il devient simple.

Prenons « préparer un thé ». C'est trop vague pour un ordinateur. On
découpe :

```text
1. Remplir la bouilloire d'eau.
2. Allumer la bouilloire.
3. Attendre que l'eau bouille.
4. Mettre un sachet de thé dans une tasse.
5. Verser l'eau dans la tasse.
6. Attendre 3 minutes.
7. Retirer le sachet.
```

Chaque étape est maintenant assez petite pour être faite sans réfléchir. Si
une étape reste floue, on la découpe encore.

> **Astuce :** demandez-vous à chaque ligne : « un enfant de 8 ans
> saurait-il le faire ? ». Si la réponse est non, découpez.

## Écrire d'abord en français : le pseudo-code

Le **pseudo-code** est un algorithme écrit en français simple, avec la forme
d'un programme. Il n'a pas de règles strictes : il sert à réfléchir, pas à
être lancé.

```text
demander l'âge de la personne
si l'âge est 18 ou plus
    afficher « Vous pouvez voter. »
sinon
    afficher « Pas encore. »
```

Le pseudo-code a deux avantages :

- vous réfléchissez au problème **sans vous battre** avec la syntaxe ;
- il se traduit ensuite dans **n'importe quel langage**.

## Les variables : des boîtes avec une étiquette

Un algorithme doit se souvenir de choses : un âge, un total, un prénom. On
les range dans des **variables**.

Imaginez une boîte avec une étiquette. L'étiquette, c'est le **nom** de la
variable ; ce qu'il y a dedans, c'est sa **valeur**.

```text
mettre 0 dans la boîte « total »
mettre 5 dans la boîte « prix »
mettre total + prix dans la boîte « total »   (total vaut maintenant 5)
```

En Ruby, cela s'écrit avec `=` :

```ruby
total = 0
prix = 5
total = total + prix
puts total
```

```sortie
5
```

> **Attention :** en programmation, `=` ne veut pas dire « est égal à ». Il
> veut dire « range dans la boîte ». Pour comparer, on écrit `==`.

Choisissez des noms qui disent ce que contient la boîte : `prix_total`
plutôt que `x`.

## Les conditions dans la vie de tous les jours

Vous prenez des décisions toute la journée : « **si** il pleut, je prends
un parapluie, **sinon** je prends des lunettes de soleil ». C'est une
**condition**.

En pseudo-code :

```text
si il pleut
    prendre un parapluie
sinon
    prendre des lunettes de soleil
```

En Ruby :

```ruby
il_pleut = true
if il_pleut
  puts "Je prends un parapluie."
else
  puts "Je prends des lunettes de soleil."
end
```

```sortie
Je prends un parapluie.
```

Une condition est toujours une question à laquelle on répond par **oui** ou
**non** : `true` ou `false`.

## La répétition dans la vie de tous les jours

Beaucoup de tâches se répètent : « pour chaque assiette sale, la laver »,
« tant que la pâte n'est pas lisse, remuer ». C'est une **répétition**, ou
**boucle**.

Il y a deux sortes de répétitions :

- **pour chaque** élément d'une liste : on sait combien de fois on répète ;
- **tant que** quelque chose est vrai : on s'arrête quand ça change.

```text
pour chaque assiette de la pile
    laver l'assiette
    la poser sur l'égouttoir
```

En Ruby :

```ruby
assiettes = ["bleue", "blanche", "verte"]
assiettes.each do |assiette|
  puts "Je lave l'assiette #{assiette}."
end
```

```sortie
Je lave l'assiette bleue.
Je lave l'assiette blanche.
Je lave l'assiette verte.
```

> **À retenir :** avec des variables, des conditions et des répétitions,
> on peut écrire presque tous les algorithmes.

## Un exemple complet : trouver le plus grand nombre

Voici une liste de nombres : `4, 17, 9, 12`. Comment trouver le plus
grand ?

Un humain regarde et répond « 17 ». Mais si la liste contient un million de
nombres, il faut une **méthode**.

### Réfléchir comme un humain

Imaginez des cartes retournées une à une. Vous gardez en tête le plus grand
nombre vu jusqu'ici. À chaque nouvelle carte, vous comparez : si elle est
plus grande, elle devient votre nouveau « plus grand ».

### Le pseudo-code

```text
mettre le premier nombre dans la boîte « plus_grand »
pour chaque nombre de la liste
    si nombre est plus grand que plus_grand
        mettre nombre dans la boîte « plus_grand »
afficher plus_grand
```

Il y a une variable, une répétition et une condition. Rien de plus.

### En Ruby

```ruby
nombres = [4, 17, 9, 12]
plus_grand = nombres[0]
nombres.each do |nombre|
  if nombre > plus_grand
    plus_grand = nombre
  end
end
puts plus_grand
```

```sortie
17
```

Chaque ligne du pseudo-code est devenue une ligne de Ruby.

### En Rust

```rust
let nombres = [4, 17, 9, 12];
let mut plus_grand = nombres[0];
for nombre in nombres {
    if nombre > plus_grand {
        plus_grand = nombre;
    }
}
println!("{plus_grand}");
```

```sortie
17
```

La forme est la même. Rust demande `mut` : la boîte `plus_grand` va
changer de contenu, et Rust veut qu'on le dise.

### En Rythme

En Rythme, une valeur ne change jamais : on ne remplit pas la boîte
plusieurs fois. On décrit plutôt **une étape** (« entre le plus grand
actuel et un nombre, garder le plus grand »), puis on l'applique à toute la
liste avec `fold` :

```rythme
fn garder_le_plus_grand(plus_grand, nombre) = if nombre > plus_grand then nombre else plus_grand

nombres = [4, 17, 9, 12]
print(fold(nombres, nombres[0], garder_le_plus_grand))
```

```sortie
17
```

`fold` part de `nombres[0]`, puis passe chaque nombre à
`garder_le_plus_grand`, avec le résultat de l'étape précédente. C'est le
même algorithme, dit autrement. Voir [penser fonctionnel](../rythme/fonctionnel.md).

### Le raccourci

Trouver le plus grand est si courant que les trois langages le font déjà :

```ruby
puts [4, 17, 9, 12].max
```

```sortie
17
```

```rust
let nombres = [4, 17, 9, 12];
println!("{:?}", nombres.iter().max());
```

```sortie
Some(17)
```

```rythme
print(max([4, 17, 9, 12]))
```

```sortie
17
```

Rust répond `Some(17)` : pour une liste vide, il répondrait `None`, car il
n'y a pas de plus grand nombre. Utilisez les raccourcis dans vos vrais
programmes ; écrivez l'algorithme vous-même pour apprendre.

## Tester à la main avec un tableau de valeurs

Avant de faire confiance à un algorithme, suivez-le **à la main**, avec un
papier et un crayon. Notez la valeur de chaque variable à chaque tour.

Pour la liste `4, 17, 9, 12` :

| Tour | nombre | nombre > plus_grand ? | plus_grand après |
|------|--------|-----------------------|------------------|
| départ | –    | –                     | 4                |
| 1    | 4      | non (4 > 4 est faux)  | 4                |
| 2    | 17     | oui                   | 17               |
| 3    | 9      | non                   | 17               |
| 4    | 12     | non                   | 17               |

À la fin, `plus_grand` vaut 17 : c'est juste.

Essayez ensuite des cas **difficiles** :

- le plus grand est le **premier** : `20, 3, 5` ;
- le plus grand est le **dernier** : `3, 5, 20` ;
- tous les nombres sont **négatifs** : `-8, -2, -5` ;
- la liste n'a **qu'un** nombre : `7` ;
- la liste est **vide**.

Le dernier cas pose problème : il n'y a pas de « premier nombre ». Un bon
programmeur pense à ces cas avant que le programme ne plante.

> **Astuce :** en Rythme, `rythme steps fichier.ry` montre le calcul étape
> par étape, comme votre tableau.

## Erreurs fréquentes

### Partir de zéro au lieu du premier nombre

C'est tentant : on met `0` dans `plus_grand` au départ. Mais avec des
nombres négatifs, aucun n'est plus grand que 0.

```ruby norun
nombres = [-8, -2, -5]
plus_grand = 0
nombres.each do |nombre|
  plus_grand = nombre if nombre > plus_grand
end
puts plus_grand
```

```text
0
```

Il n'y a pas de message d'erreur : c'est une **erreur de logique**. Le
programme répond 0, qui n'est même pas dans la liste. Partez du premier
nombre : `plus_grand = nombres[0]`.

### Oublier le cas de la liste vide

Avec une liste vide, `nombres[0]` n'existe pas. Rust arrête le programme :

```rust norun
fn main() {
    let nombres: Vec<i32> = vec![];
    let plus_grand = nombres[0];
    println!("{plus_grand}");
}
```

```text
thread 'main' (5140) panicked at main.rs:3:29:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Vérifiez d'abord que la liste n'est pas vide, ou utilisez le raccourci
`nombres.iter().max()`, qui répond `None` pour une liste vide.

### Une boucle « tant que » qui ne s'arrête jamais

Si rien ne change dans la boucle, la condition reste vraie pour toujours.

```ruby norun
i = 0
while i < 3
  puts i
end
```

Le programme affiche `0` sans fin. Arrêtez-le avec **Ctrl + C** :

```text
boucle.rb:3:in `write': Interrupt
	from boucle.rb:3:in `puts'
	from boucle.rb:3:in `puts'
	from boucle.rb:3:in `<main>'
```

Il manquait `i = i + 1` dans la boucle. Dans votre tableau de valeurs, vous
auriez vu que `i` ne change jamais.

## Testez-vous

**1.** Qu'est-ce que le pseudo-code, et à quoi sert-il ?

> **Réponse**
>
> C'est un algorithme écrit en français simple, avec la forme d'un
> programme. Il sert à réfléchir à la méthode sans se soucier de la
> syntaxe ; on le traduit ensuite dans le langage de son choix.

**2.** Écrivez le pseudo-code pour trouver le **plus petit** nombre d'une
liste.

> **Réponse**
>
> ```text
> mettre le premier nombre dans la boîte « plus_petit »
> pour chaque nombre de la liste
>     si nombre est plus petit que plus_petit
>         mettre nombre dans la boîte « plus_petit »
> afficher plus_petit
> ```
>
> Seule la comparaison change : « plus petit » au lieu de « plus grand ».

**3.** Avec la liste `5, 2, 8`, quelles valeurs prend `plus_grand` dans
notre algorithme ?

> **Réponse**
>
> 5 au départ, 5 après le premier tour (5 n'est pas plus grand que 5), 5
> après le deuxième (2 est plus petit), puis 8 après le troisième. Le
> résultat est 8.

## En résumé

- Un algorithme est une suite d'étapes claires, dans le bon ordre, qui s'arrête.
- Découpez un gros problème en petites étapes, puis écrivez-les en pseudo-code.
- Une variable est une boîte avec une étiquette ; `=` range une valeur dedans.
- Les conditions choisissent, les répétitions recommencent.
- Le même algorithme s'écrit en Ruby, en Rust ou en Rythme : seuls les mots changent.
- Testez à la main avec un tableau de valeurs, sans oublier les cas difficiles.

Suite : [le terminal en 10 minutes](terminal.md).
