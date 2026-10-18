# Glossaire

Ce glossaire explique simplement les mots de la programmation que vous croiserez dans cette documentation, classés par ordre alphabétique.

## A

### Algorithme

Une suite d'étapes précises pour résoudre un problème, comme une recette.
Un programme est un algorithme écrit dans un langage de programmation. Voir
[Penser comme un programmeur](algorithmes.md) et [pseudo-code](#pseudo-code).

### Any

En TypeScript, le type « n'importe quoi » : il **éteint** la vérification
des types. Une valeur `any` accepte tout, même les erreurs. Préférez
[unknown](#unknown), qui oblige à vérifier avant de s'en servir.

### API

Une liste de services qu'un programme offre à d'autres programmes. Sur le
web, ce sont des adresses qui répondent avec des données, souvent en
[JSON](#json), au lieu d'une page. Le nom vient de l'anglais *Application
Programming Interface*. Voir [Comment marche le web](web.md#les-api-et-json).

### Argument

La valeur que vous donnez à une [fonction](#fonction) quand vous l'appelez.
Dans `double(21)`, l'argument est `21`.

### ASCII

Un ancien code qui donne un numéro aux lettres anglaises, aux chiffres et à
la ponctuation : `A` vaut 65, `a` vaut 97. [Unicode](#unicode) le reprend
et l'étend à toutes les écritures.

```ruby
puts "A".ord
puts 97.chr
```

```sortie
65
a
```

### Async

Du code **asynchrone** : pendant qu'une tâche attend (une réponse du
réseau, un fichier), le programme s'occupe d'autre chose au lieu de rester
bloqué. En Python et en JavaScript, on l'écrit avec `async` et `await` ;
en Rust, avec `async` et `.await`. Voir aussi [promesse](#promesse) et
[l'asynchrone en Rust](../rust/async.md).

## B

### Bibliothèque

Du code déjà écrit par d'autres, que vous pouvez réutiliser dans votre
programme. Elle vous évite de tout réinventer. Voir aussi [paquet](#paquet).

### Binaire

La façon de compter de l'ordinateur, avec seulement deux chiffres : 0 et 1.
`101` en binaire vaut 5. Voir
[Comment marche un ordinateur](ordinateur.md#compter-en-binaire).

```rust
println!("{:b}", 5);
```

```sortie
101
```

### Bit

Le plus petit morceau d'information : un `0` ou un `1`. Huit bits forment
un [octet](#octet).

### Bloc

Un groupe d'instructions qui vont ensemble. En Ruby et en Rythme, un bloc
s'écrit souvent entre `do` et `end` ; en Rust, entre accolades `{ }`.

```ruby
3.times do
  puts "Bonjour"
end
```

```sortie
Bonjour
Bonjour
Bonjour
```

### Booléen

Une valeur qui ne peut être que **vraie** ou **fausse** : `true` ou `false`.
C'est le résultat d'une comparaison comme `10 > 3`.

### Boucle

Une instruction qui répète du code plusieurs fois, par exemple pour chaque
élément d'une liste. Voir aussi [itération](#iteration).

```ruby
[1, 2, 3].each { |n| puts n * 10 }
```

```sortie
10
20
30
```

### Bug

Une erreur dans un programme : il plante, ou il ne fait pas ce que vous
vouliez. Le mot veut dire « insecte » en anglais.

## C

### Canal

En Go, un tuyau par lequel des [goroutines](#goroutine) s'envoient des
valeurs. On envoie avec `canal <- valeur` et on reçoit avec `<-canal`.

```go
package main

import "fmt"

func main() {
	canal := make(chan string)
	go func() { canal <- "Bonjour depuis une goroutine" }()
	fmt.Println(<-canal)
}
```

```sortie
Bonjour depuis une goroutine
```

### Cargo

L'outil de Rust qui crée un projet (`cargo new`), le compile et le lance
(`cargo run`), et télécharge ses [crates](#crate).

### Chaîne de caractères

Un texte, écrit entre guillemets : `"Bonjour"`. On dit souvent **chaîne**,
ou *string* en anglais.

### Chemin

L'adresse d'un fichier ou d'un dossier : `/home/alice/code/bonjour.rb`. Un
chemin **absolu** part de la racine du disque ; un chemin **relatif** part
du dossier où vous êtes, et `..` désigne le dossier parent.

### Classe

Un modèle pour fabriquer des [objets](#objet) du même genre. La classe
`Chien` décrit ce qu'est un chien ; chaque chien créé est un objet. Ruby
utilise beaucoup les classes ; Rythme n'en a pas.

### Client

Le programme qui **demande** quelque chose à un [serveur](#serveur). Votre
navigateur est un client : il demande des pages web.

### Closure

Une fonction sans nom qui se souvient des variables autour d'elle, là où
elle a été créée. En français, on dit parfois **fermeture**.

```ruby
bonus = 5
ajouter_bonus = ->(x) { x + bonus }
puts ajouter_bonus.call(10)
```

```sortie
15
```

### Commentaire

Un texte dans le code que l'ordinateur ignore. Il sert à expliquer le code
aux humains. En Ruby et en Rythme, il commence par `#` ; en Rust, par `//`.

```ruby
# Ceci est un commentaire : rien ne s'affiche.
puts "Seule cette ligne compte"
```

```sortie
Seule cette ligne compte
```

### Commit

Une version enregistrée de votre projet avec [Git](#git), avec un message
qui dit ce qui a changé. Voir
[Sauvegarder son travail avec Git](git.md).

### Compilateur

Un programme qui traduit tout votre code en langage machine **avant** de le
lancer. Go et Rust utilisent un compilateur. Voir aussi
[interpréteur](#interpreteur).

### Compilation

Le travail du [compilateur](#compilateur) : traduire tout le code en
langage machine avant de le lancer. En Rust, `cargo build` compile ;
`cargo run` compile puis lance.

### Compréhension

En Python, une façon courte de construire une [liste](#liste) à partir
d'une autre, en une seule ligne : « le carré de chaque `n` de 1 à 4 ».

```python
carres = [n * n for n in range(1, 5)]
print(carres)
```

```sortie
[1, 4, 9, 16]
```

### Concurrence

Faire avancer plusieurs tâches en même temps, par exemple avec plusieurs
[threads](#thread). Voir [la concurrence en Rust](../rust/concurrence.md)
et [en Ruby](../ruby/concurrence.md).

### Condition

Un test qui décide quel code exécuter : « **si** il pleut, prendre un
parapluie, **sinon** des lunettes de soleil ». Elle s'écrit avec `if`.

```ruby
meteo = "pluie"
if meteo == "pluie"
  puts "Parapluie"
else
  puts "Lunettes de soleil"
end
```

```sortie
Parapluie
```

### Constante

Une valeur nommée qui ne doit jamais changer, comme `PI = 3.14159`. Par
habitude, son nom s'écrit souvent en majuscules.

### Crate

Un [paquet](#paquet) Rust : une bibliothèque ou un programme. On les trouve
sur le site `crates.io`, et [Cargo](#cargo) les télécharge.

## D

### Débogage

Chercher et corriger un [bug](#bug). On lit les messages d'erreur, on
affiche des valeurs, on teste petit à petit. En anglais : *debugging*. Voir
[Trouver et corriger ses erreurs](deboguer.md).

### Décorateur

En Python, une fonction qui **enveloppe** une autre fonction pour lui
ajouter quelque chose, sans toucher à son code. On le place au-dessus avec
`@`.

```python
def annonce(fonction):
    def enveloppe():
        print("Début")
        fonction()
    return enveloppe

@annonce
def bonjour():
    print("Bonjour")

bonjour()
```

```sortie
Début
Bonjour
```

### Dépôt

Un dossier suivi par [Git](#git), avec tout son historique. En anglais :
*repository*, ou *repo*. Un dépôt peut aussi être en ligne, sur GitHub.

### Dictionnaire

Une collection qui associe une **valeur** à une **clé**, comme un vrai
dictionnaire associe une définition à un mot. En Python, on dit *dict* ;
en JavaScript, *objet* ou *Map* ; en Go, *map* ; en Ruby, *hash* ; en
Rust, *HashMap*.

```ruby
ages = { "Ada" => 36, "Alan" => 41 }
puts ages["Ada"]
```

```sortie
36
```

### DOM

La page web telle que le navigateur la garde en mémoire : un arbre
d'éléments (titres, paragraphes, boutons) que JavaScript peut lire et
modifier. Le nom vient de l'anglais *Document Object Model*.

```javascript norun
document.querySelector("h1").textContent = "Bonjour !";
```

### Durée de vie

En Rust, la période pendant laquelle une [référence](#emprunt) reste
valable. Le compilateur vérifie qu'une référence ne vit jamais plus
longtemps que la valeur qu'elle désigne. En anglais : *lifetime*. Voir
[les durées de vie en Rust](../rust/durees-de-vie.md).

## E

### Éditeur de code

Le logiciel dans lequel on écrit du code, comme VS Code. Voir
[Installer un éditeur de code](editeur.md).

### Emprunt

En Rust, utiliser une valeur sans la prendre à son propriétaire, avec `&`.
On dit aussi **référence**. Voir [possession](#possession).

### Enregistrement

En Rythme, une valeur qui regroupe des champs nommés :
`{nom: "Ada", age: 36}`. On lit un champ avec un point.

```rythme
personne = {nom: "Ada", age: 36}
print(personne.nom)
```

```sortie
Ada
```

### Ensemble

Une collection **sans doublons**, où l'ordre d'arrivée ne compte pas. Un
élément y est, ou n'y est pas. En anglais : *set*.

```rythme
print(set([3, 1, 3, 2]))
```

```sortie
set(1, 2, 3)
```

### Entier

Un nombre sans virgule : `0`, `42`, `-7`. En anglais : *integer*. Voir
aussi [flottant](#flottant).

### Énumérateur

En Ruby, une valeur qui donne les éléments d'une collection **un par un**,
à la demande, avec `next`. C'est le cousin Ruby de
l'[itérateur](#iterateur). Voir [les énumérateurs](../ruby/enumerateurs.md).

```ruby
lettres = ["a", "b", "c"].each
puts lettres.next
puts lettres.next
```

```sortie
a
b
```

### Énumération

Un [type](#type) qui ne peut prendre qu'une valeur parmi une liste fixée :
un feu est `Rouge`, `Orange` ou `Vert`. En Rust, on l'écrit avec `enum` ;
en Rythme, avec `type`.

### Environnement virtuel

En Python, un dossier qui contient les [paquets](#paquet) d'**un seul**
projet, à l'écart des autres. On le crée avec `python3 -m venv .venv`,
puis on l'active avant d'utiliser [pip](#pip).

### Erreur

Un message qui dit que quelque chose s'est mal passé. Il indique souvent le
fichier, la ligne et la cause. Lisez-le : c'est votre meilleur indice.

### Exception

En Ruby, une erreur qui interrompt le programme, sauf si on l'attrape avec
`rescue`. Rust et Rythme n'en ont pas : ils utilisent [Result](#result).

```ruby
begin
  Integer("abc")
rescue ArgumentError
  puts "Ce n'est pas un nombre."
end
```

```sortie
Ce n'est pas un nombre.
```

### Expression

Un morceau de code qui donne une [valeur](#valeur). `2 + 3` est une
expression qui vaut `5` ; `"Ada"` est une expression qui vaut `"Ada"`.

### Expression régulière

Un motif qui décrit la forme d'un texte, pour le chercher ou le remplacer. On dit le plus souvent [regex](#regex).

## F

### Fichier source

Le fichier texte qui contient votre code, par exemple `bonjour.rb`,
`main.rs` ou `bonjour.ry`.

### Flottant

Un nombre à virgule : `3.14`, `0.5`. En programmation, la virgule s'écrit
avec un **point**. En anglais : *float*.

### Fonction

Un morceau de code qui porte un nom et qu'on peut réutiliser. Elle reçoit
des [arguments](#argument) et renvoie un résultat.

```rythme
fn double(x) = x * 2
print(double(21))
```

```sortie
42
```

## G

### Gem

Un [paquet](#paquet) Ruby. On l'installe avec `gem install nom`, ou on le
liste dans le fichier `Gemfile` d'un projet.

### Générateur

Une fonction qui donne ses valeurs **une à une**, au fur et à mesure qu'on
les demande, au lieu de tout calculer d'un coup. En Python et en
JavaScript, elle utilise `yield`.

```python
def compte_a_rebours(n):
    while n > 0:
        yield n
        n -= 1

for n in compte_a_rebours(3):
    print(n)
```

```sortie
3
2
1
```

### Générique

Du code écrit une fois pour plusieurs [types](#type). En Rust, `Vec<T>`
est une liste d'éléments de n'importe quel type `T` : `Vec<i32>`,
`Vec<String>`... TypeScript (`Array<T>`) et Go (`[T any]`) ont la même
idée.

### Git

Un logiciel qui garde l'historique de votre code : chaque version est un
[commit](#commit), rangé dans un [dépôt](#depot). Voir
[Sauvegarder son travail avec Git](git.md).

### Goroutine

En Go, une fonction qui tourne **en même temps** que le reste du
programme. On la lance en écrivant `go` devant l'appel. Elles communiquent
souvent par des [canaux](#canal). Voir aussi [concurrence](#concurrence).

## H

### Héritage

En Ruby, une [classe](#classe) peut reprendre tout ce que sait faire une
autre classe, puis y ajouter ses propres méthodes. On dit qu'elle en
**hérite**, avec `<`.

```ruby
class Animal
  def respirer = "Je respire."
end

class Chat < Animal
end

puts Chat.new.respirer
```

```sortie
Je respire.
```

### HTML

Le langage qui décrit la **structure** d'une page web : titres,
paragraphes, liens, images. Il s'écrit avec des balises comme `<h1>` et
`</h1>`. Voir [Comment marche le web](web.md#html-css-et-javascript).

### HTTP

La langue que parlent le [client](#client) et le [serveur](#serveur) sur le
web : une **requête** (`GET /page`), puis une **réponse** avec un code,
comme `200` (tout va bien) ou `404` (page introuvable). `HTTPS` est sa
version chiffrée.

## I

### Immuable

Qui ne peut pas changer. Une valeur immuable reste la même du début à la
fin. En Rythme, toutes les valeurs sont immuables ; en Rust, les variables
le sont par défaut. Voir aussi [mutable](#mutable).

### Indentation

Le décalage d'une ligne vers la droite, avec des espaces. Il montre ce qui
est **à l'intérieur** d'une boucle ou d'une fonction. En Python,
l'indentation fait partie de la syntaxe : une ligne mal décalée est une
erreur.

### Indice

La position d'un élément dans une [liste](#liste). On compte à partir de
**zéro** : le premier élément a l'indice `0`.

```ruby
fruits = ["pomme", "poire", "kiwi"]
puts fruits[0]
puts fruits[2]
```

```sortie
pomme
kiwi
```

### Interface

Une liste de ce qu'une valeur doit **savoir faire** ou **contenir**, sans
dire comment. En Go, un type qui a les bonnes méthodes respecte
l'interface tout seul ; en TypeScript, une interface décrit la forme d'un
objet.

```typescript
interface Animal {
  nom: string;
  cri(): string;
}

const chat: Animal = { nom: "Félix", cri: () => "Miaou" };
console.log(`${chat.nom} dit ${chat.cri()}`);
```

```sortie
Félix dit Miaou
```

### Interpréteur

Un programme qui lit votre code et l'exécute tout de suite, ligne après
ligne. Python, JavaScript, Ruby et Rythme utilisent un interpréteur. Voir aussi
[compilateur](#compilateur).

### Intervalle

Une suite de nombres donnée par son début et sa fin : `1..10`. En Rust,
`1..10` s'arrête à 9 et `1..=10` va jusqu'à 10.

### Itérateur

Une valeur qui donne les éléments d'une collection un par un. En Rust,
`.iter()` crée un itérateur, sur lequel on enchaîne `map`, `filter` ou
`sum`.

```rust
let total: i32 = [1, 2, 3].iter().map(|x| x * 10).sum();
println!("{total}");
```

```sortie
60
```

### Itération

Un passage dans une [boucle](#boucle). Itérer sur une liste, c'est traiter
ses éléments un par un.

## J

### JSON

Un format de texte très répandu pour échanger des données entre
programmes : `{"nom": "Ada", "age": 36}`. Les clés sont toujours entre
guillemets doubles.

```rythme
print(to_json({nom: "Ada", age: 36}))
```

```sortie
{"nom": "Ada", "age": 36}
```

## L

### Lambda

Une [fonction](#fonction) sans nom, rangée dans une variable. En Ruby, on
l'écrit avec `->` ; c'est souvent aussi une [closure](#closure). Voir
[procs et lambdas](../ruby/procs-lambdas.md).

```ruby
double = ->(x) { x * 2 }
puts double.call(21)
```

```sortie
42
```

### Liste

Une suite ordonnée de valeurs : `[1, 2, 3]`. En Ruby, on dit *tableau*
(*Array*) ; en Rust, *Vec*. Chaque élément a un [indice](#indice).

## M

### Macro

En Rust, un mot qui finit par `!`, comme `println!` ou `vec!`. Elle écrit
du code à votre place, avant la compilation.

### Match

Une instruction qui compare une valeur à plusieurs formes possibles et
choisit le code de la première qui correspond. Rust et Rythme l'utilisent
beaucoup ; Ruby a `case`.

```rythme
note = 15
print(match note
  0..9 -> "insuffisant"
  _ -> "bien"
end)
```

```sortie
bien
```

### Mémoire vive

La mémoire rapide où un programme range ses données **pendant qu'il
tourne**. Elle s'efface quand on éteint l'ordinateur, contrairement au
disque. En anglais : *RAM*. Voir
[Comment marche un ordinateur](ordinateur.md#la-memoire-vive-et-le-disque).

### Métaprogrammation

Du code qui écrit ou modifie du code pendant que le programme tourne, par
exemple pour créer des méthodes automatiquement. Ruby le permet beaucoup.
Voir [la métaprogrammation en Ruby](../ruby/metaprogrammation.md).

### Méthode

Une fonction attachée à une valeur. On l'appelle avec un point :
`"bonjour".upcase` appelle la méthode `upcase` du texte.

```ruby
puts "bonjour".upcase
```

```sortie
BONJOUR
```

### Mixin

En Ruby, un [module](#module) dont les méthodes sont ajoutées à une
[classe](#classe) avec `include`. Plusieurs classes peuvent partager ainsi
les mêmes méthodes.

### Module

Un moyen de ranger du code qui va ensemble sous un même nom, souvent dans
un fichier à part. Il aide à organiser un gros programme.

### Mutable

Qui peut changer. Une variable mutable peut recevoir une nouvelle valeur.
En Rust, on l'indique avec `mut`.

```rust
let mut compteur = 1;
compteur = compteur + 1;
println!("{compteur}");
```

```sortie
2
```

## N

### Node.js

Un programme qui lance du JavaScript **en dehors du navigateur** : dans le
terminal, ou sur un serveur. On l'utilise avec `node main.mjs`.

### npm

Le gestionnaire de [paquets](#paquet) de JavaScript, livré avec
[Node.js](#node-js). `npm install nom` télécharge un paquet ; la liste des
paquets d'un projet est dans le fichier `package.json`.

### Null

Une valeur spéciale qui veut dire « rien » ou « absent ». Python
l'appelle `None` ; JavaScript a `null` et `undefined` ; Go et Ruby
l'appellent `nil`. Rust et Rythme n'ont pas de `null` : ils utilisent `None`, qui oblige
à prévoir le cas où la valeur manque.

## O

### Objet

Une valeur qui regroupe des données et des [méthodes](#methode). En Ruby,
tout est objet : même le nombre `5`.

### Octet

Un groupe de huit [bits](#bit). Il peut valoir de 0 à 255. On mesure la
taille des fichiers en octets : Ko (mille), Mo (million), Go (milliard). En
anglais : *byte*.

### Opérateur

Un symbole qui fait une opération : `+`, `-`, `*`, `/` pour calculer, `==`
pour comparer, `&&` pour « et ».

### Option

Une valeur qui peut être présente, `Some(valeur)`, ou absente, `None`.
Rust et Rythme l'utilisent à la place de [null](#null) : le programme doit
prévoir les deux cas.

```rythme
print(first([5, 8]), first([]))
```

```sortie
Some(5) None
```

## P

### Panique

En Rust, l'arrêt brutal d'un programme sur une erreur impossible à
surmonter, comme lire l'élément 10 d'une liste de 3 éléments. Le message
commence par `panicked at`.

### Paquet

Une [bibliothèque](#bibliotheque) prête à installer. En Python, on
l'installe avec [pip](#pip) ; en JavaScript, avec [npm](#npm) ; en Go, avec
`go get`. En Ruby, on dit *gem* ; en Rust, *crate*.

### Paramètre

Le nom qui reçoit l'[argument](#argument) dans la définition d'une
fonction. Dans `fn double(x) = x * 2`, le paramètre est `x`.

### Pile et tas

Les deux endroits où un programme range ses données en mémoire. La
**pile** est rapide et rangée, comme une pile d'assiettes : on y met les
petites valeurs de taille connue. Le **tas** est plus grand et plus libre :
on y met les données dont la taille change, comme une liste qui grandit.
Rust vous fait penser à cette différence.

### pip

Le gestionnaire de [paquets](#paquet) de Python. `pip install nom`
télécharge un paquet depuis PyPI, le catalogue officiel. Utilisez-le dans
un [environnement virtuel](#environnement-virtuel).

### Pointeur

Une valeur qui contient l'**adresse** d'une autre valeur en mémoire, comme
un panneau qui indique où se trouve quelque chose. En Rust, `&x` et
`Box<T>` sont des sortes de pointeurs.

### Portée

La zone du code où une variable existe. Une variable créée dans une fonction
n'existe pas en dehors. En anglais : *scope*.

### Possession

La règle de Rust qui dit que chaque valeur a **un seul propriétaire**.
Quand le propriétaire disparaît, la valeur est libérée. En anglais :
*ownership*. Voir [la possession en Rust](../rust/possession.md).

### Processeur

La pièce de l'ordinateur qui exécute les instructions, des milliards par
seconde. Il ne comprend que le langage machine. En anglais : *CPU*.

### Processus

Un [programme](#programme) en train de tourner. Le même programme lancé
deux fois donne deux processus, chacun avec sa mémoire.

### Programme

Une liste d'instructions qu'un ordinateur exécute. Voir
[Qu'est-ce que programmer ?](programmer.md).

### Promesse

En JavaScript, une valeur qui **arrivera plus tard** (*Promise* en
anglais), par exemple la réponse d'un serveur. `await` attend qu'elle soit
prête. Voir aussi [async](#async).

```javascript
const promesse = new Promise((resoudre) => {
  setTimeout(() => resoudre("prêt"), 10);
});
console.log(await promesse);
```

```sortie
prêt
```

### Prototype

En JavaScript, l'objet modèle dont un autre objet **hérite** ses
méthodes. Quand une propriété manque, JavaScript la cherche dans le
prototype. Les classes de JavaScript reposent sur ce mécanisme. Voir aussi
[héritage](#heritage).

### Pseudo-code

Un algorithme écrit en français simple, avec la forme d'un programme, avant
de l'écrire dans un vrai langage. Voir
[Penser comme un programmeur](algorithmes.md).

```text
pour chaque nombre de la liste
    si nombre est pair
        l'afficher
```

## R

### Récursion

Quand une fonction s'appelle elle-même, sur un problème plus petit, jusqu'à
un cas simple. Ici, on additionne les nombres de `n` jusqu'à 0 :

```rythme
fn total(0) = 0
fn total(n) = n + total(n - 1)
print(total(4))
```

```sortie
10
```

### Regex

Abréviation d'**expression régulière** : un petit langage pour décrire la
forme d'un texte, afin de le chercher, de le découper ou de le remplacer.
`[0-9]+` veut dire « un ou plusieurs chiffres ». Voir
[les regex en Ruby](../ruby/regex.md) et [en Rythme](../rythme/regex.md).

```rythme
print(find_all("Tél : 06 12, fixe : 01 45", "[0-9]+"))
```

```sortie
["06", "12", "01", "45"]
```

### REPL

Un mode où l'on tape une ligne de code et où l'on voit le résultat tout de
suite. Le nom vient de l'anglais *Read, Eval, Print, Loop* (lire, évaluer,
afficher, recommencer). Exemples : `irb` pour Ruby, `rythme` pour Rythme.

### Result

Le résultat d'une opération qui peut échouer : `Ok(valeur)` en cas de
succès, `Err(erreur)` en cas d'échec. Rust et Rythme l'utilisent à la place
des [exceptions](#exception).

```rythme
print(parse_int("42"), parse_int("abc") |> is_err)
```

```sortie
Ok(42) true
```

### Rétrécissement

En TypeScript, quand une vérification **précise** le type d'une valeur :
après `typeof x === "string"`, TypeScript sait que `x` est un texte. On dit
aussi *narrowing*.

```typescript
function longueur(x: string | number): number {
  if (typeof x === "string") {
    return x.length;
  }
  return x;
}

console.log(longueur("Ada"), longueur(7));
```

```sortie
3 7
```

## S

### Sérialisation

Transformer une valeur du programme en texte (ou en octets), pour
l'enregistrer dans un fichier ou l'envoyer sur le réseau. L'inverse, relire
ce texte, s'appelle la **désérialisation**. [JSON](#json) est un format de
sérialisation.

### Serveur

Un programme (et, par extension, l'ordinateur qui le fait tourner) qui
attend des demandes et y **répond**. Un serveur web envoie des pages aux
[clients](#client).

### Slice

En Go, une liste qui peut grandir. `append` lui ajoute des éléments.

```go
package main

import "fmt"

func main() {
	notes := []int{12, 15}
	notes = append(notes, 9)
	fmt.Println(notes, len(notes))
}
```

```sortie
[12 15 9] 3
```

### Structure

Un [type](#type) qui regroupe plusieurs valeurs nommées, ses **champs**.
En Rust, on l'écrit avec `struct` ; en Ruby, `Struct` en crée une
rapidement.

```rust
struct Personne {
    nom: String,
    age: u32,
}

let ada = Personne { nom: String::from("Ada"), age: 36 };
println!("{} a {} ans", ada.nom, ada.age);
```

```sortie
Ada a 36 ans
```

### Symbole

En Ruby et en Rythme, un nom fixe qui commence par deux-points : `:ok`,
`:rouge`. Il sert d'étiquette, souvent comme clé de
[dictionnaire](#dictionnaire).

### Syntaxe

Les règles d'écriture d'un langage : où mettre les parenthèses, les
guillemets, les mots-clés. Une faute de syntaxe empêche le programme de
démarrer.

### Système d'exploitation

Le programme qui démarre en premier et gère tout l'ordinateur : il partage
le processeur et la mémoire, range les fichiers et lance les programmes.
Exemples : Windows, macOS, Linux, Android.

## T

### Terminal

Une fenêtre où l'on tape des commandes au clavier. Voir
[Le terminal en 10 minutes](terminal.md).

### Test

Un petit morceau de code qui vérifie qu'un autre code marche bien. Si vous
changez votre programme, les tests vous préviennent quand vous cassez
quelque chose.

### Thread

Un fil d'exécution : une partie du programme qui tourne en même temps que
les autres. Plusieurs threads permettent de faire plusieurs choses à la
fois. Voir [la concurrence en Rust](../rust/concurrence.md).

### Trait

En Rust, une liste de méthodes qu'un [type](#type) promet de savoir faire.
Le trait `Display`, par exemple, dit qu'une valeur sait s'afficher avec
`{}`.

### Tuple

Un petit groupe de valeurs, de types éventuellement différents, sans noms
de champs. En Rust, on lit ses éléments par leur position : `.0`, `.1`.

```rust
let point = (3, 7);
println!("x = {}, y = {}", point.0, point.1);
```

```sortie
x = 3, y = 7
```

### Typage

La façon dont un langage vérifie les [types](#type). **Statique** : avant
de lancer le programme (Go, Rust, TypeScript). **Dynamique** : pendant
qu'il tourne (Python, JavaScript, Ruby, Rythme).

### Type

La sorte d'une valeur : un [entier](#entier), un texte, un
[booléen](#booleen), une [liste](#liste)... Le type dit ce qu'on peut faire
avec la valeur.

```ruby
puts 42.class
puts "Ada".class
```

```sortie
Integer
String
```

### Type littéral

En TypeScript, un type qui n'accepte qu'**une seule valeur** précise, comme
`"rouge"`. Avec une [union](#union-de-types), on liste les valeurs permises.

```typescript
type Feu = "rouge" | "orange" | "vert";
const feu: Feu = "vert";
console.log(feu);
```

```sortie
vert
```

## U

### Unicode

Le code qui donne un numéro à **chaque caractère** de toutes les écritures
du monde, émojis compris : `é` vaut 233, `€` vaut 8364. On le range le plus
souvent en **UTF-8**.

```rythme
print(char_code("é"), char(8364))
```

```sortie
233 €
```

### Union de types

En TypeScript, un type qui accepte **plusieurs** types, séparés par `|` :
`string | number` est « un texte ou un nombre ». Avant de s'en servir, on
fait un [rétrécissement](#retrecissement).

### Unknown

En TypeScript, le type d'une valeur **inconnue** : TypeScript interdit de
s'en servir tant qu'on n'a pas vérifié ce qu'elle est. C'est la version
sûre de [any](#any).

```typescript
const valeur: unknown = JSON.parse("42");
if (typeof valeur === "number") {
  console.log(valeur + 1);
}
```

```sortie
43
```

### URL

L'adresse d'une ressource sur le web :
`https://www.exemple.fr/recettes?portions=4`. Elle donne le protocole, la
machine, le chemin et parfois des paramètres. Voir
[Comment marche le web](web.md#l-adresse-l-url).

## V

### Valeur

Une donnée : un nombre, un texte, une liste, `true`... C'est ce que
manipule un programme.

### Variable

Un nom qui désigne une [valeur](#valeur), comme une étiquette collée sur
une boîte. On s'en sert pour retrouver la valeur plus tard.

```ruby
prenom = "Ada"
puts prenom
```

```sortie
Ada
```

## En résumé

- Un programme manipule des valeurs : entiers, flottants, textes, booléens, listes.
- On range les valeurs dans des variables, et on réutilise du code avec des fonctions.
- Les conditions choisissent, les boucles répètent.
- Un interpréteur exécute tout de suite, un compilateur traduit d'abord.
- Un bug se corrige en lisant le message d'erreur : c'est le débogage.
