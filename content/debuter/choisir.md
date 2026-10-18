# Quel langage choisir ?

Ruby, Rust et Rythme ont chacun leurs forces : cette page les compare simplement et vous propose un parcours selon ce que vous voulez faire.

> **Objectifs :**
>
> - connaître les forces et les limites de Rythme, de Ruby et de Rust ;
> - comparer les trois langages sur un même exemple ;
> - choisir un parcours selon ce que vous voulez faire ;
> - savoir par quelle page commencer.

## Il n'y a pas de mauvais choix

Votre premier langage n'est pas un engagement pour la vie. Les idées de base
sont les mêmes partout : variables, conditions, boucles, fonctions, listes.

Quand vous connaissez un langage, le deuxième s'apprend bien plus vite.
L'important est de **commencer**.

## Rythme : pour apprendre

Rythme est un petit langage, pensé pour les débutants.

- **Peu de règles** : seulement 15 mots-clés, et une seule façon de faire
  chaque chose.
- **Des erreurs qui expliquent** : chaque message dit où est le problème et
  comment le corriger, en français.
- **Un style fonctionnel** : les valeurs ne changent jamais, ce qui évite
  beaucoup de surprises.

Sa limite : il est jeune. Peu de gens l'utilisent, et on ne s'en sert pas
pour de gros projets professionnels.

## Ruby : pour créer vite

Ruby est un langage agréable à lire, presque comme de l'anglais.

- **Facile à écrire** : peu de symboles, beaucoup de choses déjà prêtes.
- **Les sites web** : le framework Ruby on Rails sert à construire de
  nombreux sites connus.
- **Une grande communauté** : des milliers de bibliothèques (les *gems*) et
  beaucoup d'aide en ligne.

Sa limite : il est plus lent que Rust. Pour la plupart des programmes, cela
ne se voit pas.

## Rust : pour aller vite et comprendre la machine

Rust est un langage compilé, très rapide et très sûr.

- **Des programmes rapides** : aussi rapides que ceux écrits en C ou C++.
- **La sûreté** : le compilateur refuse beaucoup d'erreurs avant même de
  lancer le programme.
- **Comprendre l'ordinateur** : Rust vous fait penser à la mémoire et à la
  façon dont les données sont rangées.

Sa limite : il est **plus difficile** à apprendre. Le compilateur est
exigeant, et certaines idées, comme la
[possession](../rust/possession.md), demandent du temps.

> **Note :** Rythme lui-même est écrit en Rust. Les trois langages de cette
> documentation se tiennent la main.

## Le tableau comparatif

| Critère      | Rythme                    | Ruby                        | Rust                              |
|--------------|---------------------------|-----------------------------|-----------------------------------|
| Facilité     | Très facile               | Facile                      | Difficile au début                |
| Vitesse      | Lente                     | Moyenne                     | Très rapide                       |
| Exécution    | Interprété                | Interprété                  | Compilé                           |
| Usages       | Apprendre, petits scripts | Sites web, scripts, outils  | Outils, systèmes, jeux, web rapide |
| Communauté   | Petite et naissante       | Grande et accueillante      | Grande et en forte croissance     |

## Un exemple pour comparer

Voici le même calcul, la somme des nombres de 1 à 10, dans les trois
langages.

```ruby
puts (1..10).sum
```

```sortie
55
```

```rust
let somme: i32 = (1..=10).sum();
println!("{somme}");
```

```sortie
55
```

```rythme
print(sum(1..10))
```

```sortie
55
```

Rust demande un peu plus de précision : il veut connaître le type du
résultat (`i32`, un nombre entier).

## Quel parcours suivre

### « Je veux apprendre à programmer »

Commencez par **Rythme**, puis passez à **Ruby**. Rythme vous apprend les
idées sans vous perdre dans les détails ; Ruby vous ouvre ensuite à de vrais
projets.

### « Je veux faire des sites web »

Choisissez **Ruby**. C'est le chemin le plus direct vers Ruby on Rails.

### « Je veux des programmes rapides, ou comprendre la machine »

Choisissez **Rust**. Prenez votre temps : les débuts sont exigeants, mais
le compilateur vous apprend énormément.

> **Astuce :** vous hésitez encore ? Prenez Rythme. Son
> [playground](https://maxlestage.github.io/Rythme/) se lance dans le
> navigateur, sans rien installer.

## Commencer

Chaque parcours commence par une page d'introduction, qui explique comment
installer le langage et lancer un premier programme :

- [Commencer avec Rythme](../rythme/introduction.md)
- [Commencer avec Ruby](../ruby/introduction.md)
- [Commencer avec Rust](../rust/introduction.md)

Avant de vous lancer, assurez-vous de savoir utiliser
[le terminal](terminal.md) et [un éditeur de code](editeur.md).

## Erreurs fréquentes

Quand on passe d'un langage à l'autre, on mélange vite les mots. Chaque
langage le signale à sa façon.

### Écrire du Rust en Ruby

```ruby norun
println!("Bonjour")
```

```text
c.rb:1:in `<main>': undefined method `println!' for main (NoMethodError)

println!("Bonjour")
^^^^^^^^
Did you mean?  print
```

En Ruby, on écrit `puts "Bonjour"`.

### Écrire du Ruby en Rust

```rust norun
fn main() {
    puts "Bonjour";
}
```

```text
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `"Bonjour"`
 --> c.rs:2:10
  |
2 |     puts "Bonjour";
  |          ^^^^^^^^^ expected one of 8 possible tokens

error: aborting due to 1 previous error
```

En Rust, on écrit `println!("Bonjour");`.

### Écrire une boucle for en Rythme

```rythme norun
nombres = [1, 2, 3]
for n in nombres
  print(n)
end
```

```text
erreur[E001]: `for` n'est pas utilisé en Rythme
 --> boucle.ry:2:1
  |
2 | for n in nombres
  | ^^^ vient d'un autre langage
  |
  = aide: Rythme n'a pas de boucles. Travaillez sur toute la liste à la place :
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      ou écrivez une fonction récursive. Voir `rythme help functional`.
  = en savoir plus : `rythme explain E001`
```

En Rythme, on écrit `nombres |> each(print)`. Voir
[conditions et répétition](../rythme/conditions-boucles.md).

> **Astuce :** apprenez un langage à la fois. Quand vous en passez un
> deuxième, gardez son [aide-mémoire](../rust/aide-memoire.md) ouvert à côté.

## Testez-vous

**1.** Vous voulez surtout comprendre les idées de la programmation, sans
vous perdre dans les détails. Quel langage choisir ?

> **Réponse**
>
> Rythme : il a peu de règles et des messages d'erreur en français qui
> expliquent comment corriger.

**2.** Lequel des trois langages est compilé ?

> **Réponse**
>
> Rust. Ruby et Rythme sont interprétés.

**3.** Avez-vous fait un « mauvais » choix si vous changez de langage plus
tard ?

> **Réponse**
>
> Non. Les idées de base (variables, conditions, boucles, fonctions,
> listes) se retrouvent partout, et le deuxième langage s'apprend bien
> plus vite.

## En résumé

- Le premier langage n'est pas un choix définitif : les idées de base se retrouvent partout.
- Rythme est le plus simple, idéal pour apprendre.
- Ruby est agréable et très utilisé pour les sites web.
- Rust est rapide et sûr, mais plus difficile au début.
- Pour débuter : Rythme puis Ruby ; pour le web : Ruby ; pour la vitesse : Rust.

Suite : [le glossaire](glossaire.md), à garder sous la main.
