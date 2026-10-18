# Quel langage choisir ?

Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme ont chacun leurs forces : cette page les compare simplement et vous propose un parcours selon ce que vous voulez faire.

> **Objectifs :**
>
> - connaître les forces et les limites des sept langages de ce site ;
> - comparer les langages dans un tableau et sur un même petit programme ;
> - choisir un parcours selon ce que vous voulez faire ;
> - savoir par quelle page commencer.

## Il n'y a pas de mauvais choix

Votre premier langage n'est pas un engagement pour la vie. Les idées de base
sont les mêmes partout : variables, conditions, boucles, fonctions, listes.

Quand vous connaissez un langage, le deuxième s'apprend bien plus vite.
L'important est de **commencer**.

> **À retenir :** apprendre un langage, c'est surtout apprendre à
> programmer. Ce savoir-là vous suit partout.

## Python : le plus utilisé pour débuter

Python est un langage très lisible : peu de symboles, et l'indentation
(le décalage des lignes) fait partie de la syntaxe.

- **Facile à lire** : un programme Python ressemble à du pseudo-code.
- **Partout** : scripts, automatisation, science des données, intelligence
  artificielle, sites web.
- **Une énorme communauté** : des milliers de [paquets](glossaire.md#paquet)
  à installer avec [pip](glossaire.md#pip), et des réponses en ligne à
  presque toutes les questions.

Sa limite : il est plus lent que Go ou Rust, et il ne tourne pas
directement dans un navigateur.

## JavaScript : le langage du web

JavaScript est le seul langage que **tous les navigateurs** comprennent. Il
rend les pages web interactives : un bouton qui réagit, un menu qui
s'ouvre.

- **Rien à installer pour commencer** : votre navigateur sait déjà le lancer.
- **Le web de bout en bout** : dans la page (le [DOM](glossaire.md#dom)) et
  sur le serveur, avec Node.js.
- **Une immense communauté** : le registre [npm](glossaire.md#npm) est le
  plus grand catalogue de paquets au monde.

Sa limite : il laisse passer beaucoup d'erreurs. `"2" + 1` donne `"21"`,
sans prévenir. Certaines règles anciennes surprennent les débutants.

## TypeScript : JavaScript avec des types

TypeScript, c'est JavaScript, plus des **[types](glossaire.md#type)** écrits
dans le code. Un outil, `tsc`, vérifie ces types avant de lancer le
programme, puis le transforme en JavaScript.

- **Moins de surprises** : beaucoup d'erreurs sont trouvées avant
  l'exécution.
- **Un éditeur plus malin** : il complète le code et signale les fautes
  pendant que vous tapez.
- **Très demandé** : la plupart des gros projets web l'utilisent.

Sa limite : il faut d'abord connaître les bases de JavaScript. Apprenez-le
en deuxième.

## Go : simple, rapide, pour les serveurs

Go a été créé chez Google pour écrire des serveurs et des outils. Il est
volontairement **petit** : peu de mots-clés, une seule façon de formater
le code.

- **Rapide** : il est compilé, et un programme Go démarre en un instant.
- **Le travail en parallèle facile** : les
  [goroutines](glossaire.md#goroutine) et les [canaux](glossaire.md#canal)
  font plusieurs choses à la fois.
- **Un seul fichier à livrer** : le compilateur produit un programme
  complet, sans rien à installer à côté.

Sa limite : il est un peu **bavard**. On écrit souvent les mêmes lignes,
par exemple pour vérifier chaque erreur.

## Ruby : pour créer vite

Ruby est un langage agréable à lire, presque comme de l'anglais.

- **Facile à écrire** : peu de symboles, beaucoup de choses déjà prêtes.
- **Les sites web** : le framework Ruby on Rails sert à construire de
  nombreux sites connus.
- **Une communauté accueillante** : des milliers de bibliothèques (les
  [gems](glossaire.md#gem)) et beaucoup d'aide en ligne.

Sa limite : il est plus lent que Go ou Rust. Pour la plupart des
programmes, cela ne se voit pas.

## Rust : pour aller très vite et comprendre la machine

Rust est un langage compilé, très rapide et très sûr.

- **Des programmes rapides** : aussi rapides que ceux écrits en C ou C++.
- **La sûreté** : le compilateur refuse beaucoup d'erreurs avant même de
  lancer le programme.
- **Comprendre l'ordinateur** : Rust vous fait penser à la mémoire et à la
  façon dont les données sont rangées.

Sa limite : il est **plus difficile** à apprendre. Le compilateur est
exigeant, et certaines idées, comme la
[possession](../rust/possession.md), demandent du temps.

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

> **Note :** Rythme lui-même est écrit en Rust. Les langages de cette
> documentation se tiennent la main.

## Le tableau comparatif

| Langage    | Facilité            | Vitesse       | Usages principaux                    | Où il tourne                      | Communauté              |
|------------|---------------------|---------------|--------------------------------------|-----------------------------------|-------------------------|
| Python     | Très facile         | Moyenne       | Scripts, données, IA, web            | Ordinateur, serveur               | Immense                 |
| JavaScript | Facile              | Bonne         | Pages web, serveurs, applications    | Navigateur, serveur (Node.js)     | Immense                 |
| TypeScript | Moyenne             | Bonne         | Gros projets web                     | Comme JavaScript, après `tsc`     | Très grande             |
| Go         | Facile              | Rapide        | Serveurs, outils, cloud              | Serveur, terminal                 | Grande                  |
| Ruby       | Facile              | Moyenne       | Sites web (Rails), scripts           | Ordinateur, serveur               | Grande et accueillante  |
| Rust       | Difficile au début  | Très rapide   | Outils, systèmes, jeux, web rapide   | Partout, même le navigateur       | Grande, en croissance   |
| Rythme     | Très facile         | Lente         | Apprendre, petits scripts            | Terminal, playground du navigateur | Petite et naissante    |

Trois mots pour lire ce tableau :

- **Interprétés** : Python, JavaScript, Ruby et Rythme sont lus et exécutés
  directement par un [interpréteur](glossaire.md#interpreteur).
- **Compilés** : Go et Rust sont d'abord traduits en langage machine par un
  [compilateur](glossaire.md#compilateur).
- **TypeScript** est à part : il est vérifié puis traduit en JavaScript.

## Un exemple pour comparer

Voici le même petit programme dans les sept langages : il fait la somme
d'une liste de nombres.

### En Python

```python
nombres = [4, 8, 15, 16, 23, 42]
print(sum(nombres))
```

```sortie
108
```

### En JavaScript

```javascript
const nombres = [4, 8, 15, 16, 23, 42];
const total = nombres.reduce((somme, n) => somme + n, 0);
console.log(total);
```

```sortie
108
```

JavaScript n'a pas de `sum` : `reduce` ajoute les nombres un à un, en
partant de `0`.

### En TypeScript

```typescript
const nombres: number[] = [4, 8, 15, 16, 23, 42];
const total: number = nombres.reduce((somme, n) => somme + n, 0);
console.log(total);
```

```sortie
108
```

C'est le même code, avec les types en plus : `number[]` veut dire « une
liste de nombres ».

### En Go

```go
package main

import "fmt"

func main() {
	nombres := []int{4, 8, 15, 16, 23, 42}
	total := 0
	for _, n := range nombres {
		total += n
	}
	fmt.Println(total)
}
```

```sortie
108
```

Go demande un programme complet (`package main`, `func main`) et une boucle
écrite à la main : c'est plus long, mais tout est visible.

### En Ruby

```ruby
nombres = [4, 8, 15, 16, 23, 42]
puts nombres.sum
```

```sortie
108
```

### En Rust

```rust
let nombres = [4, 8, 15, 16, 23, 42];
let total: i32 = nombres.iter().sum();
println!("{total}");
```

```sortie
108
```

Rust veut connaître le type du résultat : `i32`, un nombre entier.

### En Rythme

```rythme
nombres = [4, 8, 15, 16, 23, 42]
print(sum(nombres))
```

```sortie
108
```

Les sept font la même chose. Python, Ruby et Rythme sont les plus courts ;
Go est le plus explicite.

## Quel parcours suivre

### « Je veux apprendre à programmer »

Commencez par **Python** ou par **Rythme**. Python vous ouvre tout de
suite à de vrais projets ; Rythme vous apprend les idées sans vous perdre
dans les détails, avec des erreurs en français.

### « Je veux faire des sites web »

Commencez par **JavaScript**, puis passez à **TypeScript**. JavaScript est
obligatoire dans le navigateur ; TypeScript vous aide ensuite à écrire de
plus gros projets sans vous perdre.

### « Je veux écrire des serveurs et des outils »

Choisissez **Go**. Il est simple, rapide, et très utilisé pour les
serveurs, le cloud et les outils en ligne de commande.

### « Je veux des programmes très rapides, ou comprendre la machine »

Choisissez **Rust**. Prenez votre temps : les débuts sont exigeants, mais
le compilateur vous apprend énormément.

### « Je veux écrire des scripts, ou utiliser Rails »

Choisissez **Ruby**. Il est agréable pour automatiser de petites tâches, et
c'est le chemin direct vers Ruby on Rails.

> **Astuce :** vous hésitez encore ? Prenez Python, ou Rythme. Le
> [playground de Rythme](https://maxlestage.github.io/Rythme/) se lance
> dans le navigateur, sans rien installer.

## Commencer

Chaque parcours commence par une page d'introduction, qui explique comment
installer le langage et lancer un premier programme :

- [Commencer avec Python](../python/introduction.md)
- [Commencer avec JavaScript](../javascript/introduction.md)
- [Commencer avec TypeScript](../typescript/introduction.md)
- [Commencer avec Go](../go/introduction.md)
- [Commencer avec Ruby](../ruby/introduction.md)
- [Commencer avec Rust](../rust/introduction.md)
- [Commencer avec Rythme](../rythme/introduction.md)

Avant de vous lancer, assurez-vous de savoir utiliser
[le terminal](terminal.md) et [un éditeur de code](editeur.md).

## Erreurs fréquentes

Quand on passe d'un langage à l'autre, on mélange vite les mots. Chaque
langage le signale à sa façon.

### Écrire du Ruby en Python

```python norun
puts "Bonjour"
```

```text
  File "main.py", line 1
    puts "Bonjour"
         ^^^^^^^^^
SyntaxError: invalid syntax
```

En Python, on écrit `print("Bonjour")`, avec des parenthèses.

### Écrire du Python en JavaScript

```javascript norun
print("Bonjour");
```

```text
file:///home/ada/main.mjs:1
print("Bonjour");
^

ReferenceError: print is not defined
    at file:///home/ada/main.mjs:1:1
```

En JavaScript, on écrit `console.log("Bonjour");`.

### Garder une variable inutile en Go

Go refuse de compiler un programme qui crée une variable sans s'en servir :

```go norun
package main

import "fmt"

func main() {
	nombres := []int{4, 8, 15}
	fmt.Println("Bonjour")
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: nombres
```

Utilisez la variable, ou supprimez la ligne. Ruby, Python et JavaScript ne
disent rien dans ce cas.

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
> deuxième, gardez son aide-mémoire ouvert à côté, par exemple
> [celui de Python](../python/aide-memoire.md).

## Testez-vous

**1.** Vous voulez créer des pages web interactives. Par quel langage
commencer, et lequel apprendre ensuite ?

> **Réponse**
>
> JavaScript d'abord, car c'est le langage des navigateurs. Puis
> TypeScript, qui ajoute des types pour éviter beaucoup d'erreurs.

**2.** Parmi les sept langages, lesquels sont compilés en langage machine ?

> **Réponse**
>
> Go et Rust. Python, JavaScript, Ruby et Rythme sont interprétés ;
> TypeScript est vérifié puis traduit en JavaScript.

**3.** Avez-vous fait un « mauvais » choix si vous changez de langage plus
tard ?

> **Réponse**
>
> Non. Les idées de base (variables, conditions, boucles, fonctions,
> listes) se retrouvent partout, et le deuxième langage s'apprend bien
> plus vite.

## En résumé

- Le premier langage n'est pas un choix définitif : les idées de base se retrouvent partout.
- Pour apprendre à programmer : Python ou Rythme.
- Pour les sites web : JavaScript, puis TypeScript.
- Pour les serveurs et les outils : Go ; pour des programmes très rapides : Rust.
- Pour les scripts et Ruby on Rails : Ruby.
- Chaque parcours commence par sa page d'introduction.

Suite : [le glossaire](glossaire.md), à garder sous la main.
