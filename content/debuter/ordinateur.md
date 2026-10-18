# Comment marche un ordinateur

Un ordinateur range tout sous forme de nombres et les transforme très vite : cette page explique le processeur, la mémoire, les fichiers, les octets et le rôle du système d'exploitation, pour mieux comprendre ce que font vos programmes.

> **Objectifs :**
>
> - savoir à quoi servent le processeur, la mémoire vive et le disque ;
> - lire un chemin de fichier et comprendre l'arborescence des dossiers ;
> - comprendre les bits et les octets, et compter en binaire ;
> - voir comment un texte est rangé sous forme de nombres (ASCII, Unicode) ;
> - distinguer le système d'exploitation, un programme et un processus.

## Les grandes pièces de l'ordinateur

Imaginez une cuisine :

- le **processeur** est le cuisinier : il fait le travail ;
- la **mémoire vive** est le plan de travail : rapide, à portée de main,
  mais vidé à la fin de la journée ;
- le **disque** est le placard : plus lent, mais tout y reste rangé, même
  quand la lumière est éteinte.

Ces trois pièces travaillent ensemble chaque fois que vous lancez un
programme.

## Le processeur

Le [processeur](glossaire.md#processeur) (en anglais *CPU*) exécute des
instructions toutes simples : additionner deux nombres, comparer deux
nombres, copier un nombre d'une case à une autre, sauter à une autre
instruction.

Il n'en fait que de toutes petites, mais des **milliards par seconde**. Un
programme qui affiche une page entière, c'est des millions de ces petites
instructions mises bout à bout.

> **À retenir :** le processeur ne comprend ni le Ruby, ni le Rust, ni le
> Rythme. Il ne comprend que son propre langage, fait de nombres : le
> **langage machine**.

## La mémoire vive et le disque

La [mémoire vive](glossaire.md#memoire-vive) (en anglais *RAM*) garde les
données des programmes **pendant qu'ils tournent** : vos variables, vos
listes, le texte que vous tapez. Elle est très rapide, mais elle s'efface
quand on éteint l'ordinateur.

Le **disque** (disque dur ou SSD) garde les données **pour toujours** :
vos photos, vos documents, vos programmes. Il est plus lent, mais rien ne
disparaît à l'extinction.

| Pièce          | Vitesse       | Garde les données après extinction ? | Taille courante |
|----------------|---------------|--------------------------------------|-----------------|
| Mémoire vive   | très rapide   | non                                  | 8 à 32 Go       |
| Disque         | plus lent     | oui                                  | 256 Go à 2 To   |

C'est pour cela qu'il faut **enregistrer** un fichier : tant que vous ne
l'avez pas fait, votre travail n'est que dans la mémoire vive.

## Les fichiers et les dossiers

Sur le disque, les données sont rangées dans des **fichiers** : un fichier
est une suite d'octets avec un nom, comme `bonjour.rb` ou `photo.jpg`.

Les fichiers sont rangés dans des **dossiers**, qui peuvent contenir
d'autres dossiers. Cela forme un arbre :

```text
/home/alice
├── Documents
│   └── lettre.txt
└── code
    ├── bonjour.rb
    └── calcul.ry
```

### Les chemins

Un [chemin](glossaire.md#chemin) est l'adresse d'un fichier : la liste des
dossiers à traverser, séparés par `/` (ou `\` sous Windows).

- Un chemin **absolu** part de la racine : `/home/alice/code/bonjour.rb`,
  ou `C:\Users\Alice\code\bonjour.rb` sous Windows.
- Un chemin **relatif** part du dossier où vous êtes : si vous êtes dans
  `/home/alice`, le même fichier s'écrit `code/bonjour.rb`.
- `..` veut dire « le dossier parent » : depuis `code`, le fichier
  `lettre.txt` s'écrit `../Documents/lettre.txt`.

Le [terminal](terminal.md) vous fait utiliser ces chemins tous les jours.

## Les bits et les octets

Au fond, l'ordinateur ne connaît que deux états : courant ou pas de
courant. On les note `0` et `1`. Un de ces chiffres s'appelle un
[bit](glossaire.md#bit).

Un bit seul ne dit pas grand-chose. On les groupe donc par huit : huit bits
font un [octet](glossaire.md#octet) (en anglais *byte*). Un octet peut
prendre 2 × 2 × 2 × 2 × 2 × 2 × 2 × 2 = 256 valeurs, de 0 à 255.

| Unité   | Taille                  | Ordre d'idée              |
|---------|-------------------------|---------------------------|
| 1 octet | 8 bits                  | une lettre simple         |
| 1 Ko    | environ 1 000 octets    | un petit fichier texte    |
| 1 Mo    | environ 1 million       | une photo                 |
| 1 Go    | environ 1 milliard      | un film                   |

## Compter en binaire

Nous comptons avec dix chiffres (0 à 9) : c'est la base 10. L'ordinateur
compte avec deux chiffres (0 et 1) : c'est la base 2, le
[binaire](glossaire.md#binaire).

En base 10, chaque position vaut dix fois plus que la précédente : 1, 10,
100… En binaire, chaque position vaut **deux fois** plus : 1, 2, 4, 8, 16,
32, 64, 128.

```text
  64  32  16   8   4   2   1
   1   0   0   0   0   0   1    →  64 + 1 = 65
```

Les trois langages savent écrire un nombre en binaire. En Ruby :

```ruby
puts 65.to_s(2)
puts 0b1000001
```

```sortie
1000001
65
```

En Rust, `{:b}` affiche en binaire, et `{:08b}` complète avec des zéros
pour faire huit chiffres, soit un octet :

```rust
println!("{:b}", 65);
println!("{:08b}", 5);
println!("Un octet va jusqu'à {}", u8::MAX);
```

```sortie
1000001
00000101
Un octet va jusqu'à 255
```

Rythme n'a pas de fonction toute faite pour cela, mais on peut l'écrire
avec une [fonction récursive](glossaire.md#recursion) : on divise par deux,
et on garde le reste.

```rythme
fn binaire(0) = "0"
fn binaire(1) = "1"
fn binaire(n) = binaire(n // 2) + text(n % 2)

print(binaire(65))
print(binaire(5))
```

```sortie
1000001
101
```

## Le texte, ce sont aussi des nombres

L'ordinateur ne range que des nombres. Pour garder du texte, on a donc
donné un **numéro à chaque caractère**. C'est un code partagé par tout le
monde.

### ASCII : les 128 premiers caractères

Le code [ASCII](glossaire.md#ascii), créé dans les années 1960, numérote les
lettres anglaises, les chiffres et la ponctuation. `A` vaut 65, `B` vaut
66, `a` vaut 97, l'espace vaut 32.

En Ruby, `ord` donne le numéro d'un caractère, et `chr` fait l'inverse :

```ruby
puts "A".ord
puts 66.chr
puts "Ada".bytes.inspect
```

```sortie
65
B
[65, 100, 97]
```

En Rust, un caractère (`char`) se convertit en nombre avec `as u32` :

```rust
println!("{}", 'A' as u32);
println!("{}", char::from_u32(66).unwrap());
```

```sortie
65
B
```

En Rythme, `char_code` et `char` font le même travail :

```rythme
print(char_code("A"), char(66))
print(map(chars("Ada"), char_code))
```

```sortie
65 B
[65, 100, 97]
```

### Unicode : tous les caractères du monde

ASCII n'a ni `é`, ni `ç`, ni idéogrammes, ni émojis. [Unicode](glossaire.md#unicode)
a donc tout numéroté : plus de 150 000 caractères, dans toutes les
écritures. `é` vaut 233, `€` vaut 8364.

Pour ranger ces numéros dans des octets, on utilise presque toujours
**UTF-8** : les caractères ASCII prennent 1 octet, `é` en prend 2, `€` en
prend 3.

```ruby
puts "é".ord
puts "é".bytesize
puts "Aé".bytes.inspect
```

```sortie
233
2
[65, 195, 169]
```

En Rust, `.len()` compte les **octets**, pas les caractères :

```rust
println!("{}", "é".len());
println!("{}", "é".chars().count());
```

```sortie
2
1
```

> **Note :** c'est pour cela qu'il faut enregistrer vos fichiers en UTF-8.
> Votre éditeur le fait par défaut.

## Le système d'exploitation

Le [système d'exploitation](glossaire.md#systeme-d-exploitation) (Windows,
macOS, Linux, Android…) est le programme qui démarre en premier. C'est le
chef d'orchestre :

- il partage le processeur et la mémoire entre les programmes ;
- il range les fichiers sur le disque ;
- il gère l'écran, le clavier, le réseau ;
- il lance et arrête les programmes.

Vos programmes ne parlent jamais directement au disque : ils demandent au
système d'exploitation, par exemple « ouvre le fichier `notes.txt` ».

## Programmes et processus

Un **programme** est un fichier sur le disque : une recette rangée dans un
livre. Un [processus](glossaire.md#processus) est ce programme **en train
de tourner** : la recette en train d'être cuisinée.

Quand vous tapez `ruby bonjour.rb`, le système d'exploitation crée un
processus : il copie le programme dans la mémoire vive, lui donne un
numéro, et le processeur commence à exécuter ses instructions. Quand le
programme finit, le processus disparaît et sa mémoire est libérée.

Vous pouvez lancer deux fois le même programme : cela fait deux processus.
Le gestionnaire des tâches (Windows), le Moniteur d'activité (macOS) ou la
commande `ps` (Linux et macOS) les montrent.

## Pourquoi traduire les programmes

Le processeur ne comprend que le langage machine. Votre code, lui, est un
texte écrit pour les humains. Il faut donc le **traduire**. Il y a deux
façons de le faire, expliquées dans
[Interprété ou compilé](programmer.md#interprete-ou-compile) :

- un [compilateur](glossaire.md#compilateur) traduit tout le programme une
  fois, à l'avance, en un fichier de langage machine. C'est le cas de Rust ;
- un [interpréteur](glossaire.md#interpreteur) lit le code et l'exécute au
  fur et à mesure. C'est le cas de Ruby et de Rythme.

C'est comme un livre traduit d'avance (compilé), ou un interprète qui
traduit pendant que la personne parle (interprété).

## Erreurs fréquentes

### Additionner un texte et un nombre

Pour vous, `"A"` et `65` se ressemblent. Pour le programme, ce sont deux
[types](glossaire.md#type) différents :

```ruby norun
lettre = "A"
puts lettre + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts lettre + 1
              ^
	from main.rb:2:in `<main>'
```

Convertissez d'abord : `lettre.ord + 1` donne le nombre `66`, et
`(lettre.ord + 1).chr` donne la lettre `"B"`.

### Mettre un nombre dans un char en Rust

```rust norun
fn main() {
    let lettre: char = 65;
    println!("{lettre}");
}
```

```text
error[E0308]: mismatched types
 --> main.rs:2:24
  |
2 |     let lettre: char = 65;
  |                 ----   ^^ expected `char`, found `u8`
  |                 |
  |                 expected due to this
```

Rust ne transforme jamais un nombre en caractère tout seul. Écrivez la
lettre `'A'`, ou convertissez : `let lettre = 65u8 as char;`.

### Donner plusieurs caractères à char_code

```rythme norun
print(char_code("AB"))
```

```text
erreur[E023]: `char_code` a besoin d'un seul caractère, mais "AB" en a 2
 --> main.ry:1:7
  |
1 | print(char_code("AB"))
  |       ^^^^^^^^^^^^^^^
```

Un numéro correspond à **un** caractère. Pour un mot entier, découpez-le
d'abord : `map(chars("AB"), char_code)` donne `[65, 66]`.

### Se tromper de dossier

```ruby norun
texte = File.read("notes.txt")
```

```text
main.rb:1:in `read': No such file or directory @ rb_sysopen - notes.txt (Errno::ENOENT)
	from main.rb:1:in `<main>'
```

Un chemin relatif part du dossier où vous **lancez** le programme. Vérifiez
avec `pwd` et `ls` dans le terminal, ou donnez le bon chemin.

## Testez-vous

**1.** Vous tapez un long texte sans l'enregistrer, et la batterie lâche.
Pourquoi le texte est-il perdu ?

> **Réponse**
>
> Il n'était que dans la mémoire vive, qui s'efface à l'extinction. Seul ce
> qui est enregistré sur le disque reste.

**2.** Combien vaut le nombre binaire `101` ?

> **Réponse**
>
> 5 : on lit les positions de droite à gauche, 1 + 0 × 2 + 1 × 4 = 5.

**3.** Quelle est la différence entre un programme et un processus ?

> **Réponse**
>
> Le programme est un fichier sur le disque. Le processus est ce programme
> en train de tourner, dans la mémoire vive, avec son propre numéro.

## En résumé

- Le processeur calcule, la mémoire vive garde les données pendant le travail, le disque les garde pour toujours.
- Un chemin donne l'adresse d'un fichier : absolu depuis la racine, relatif depuis le dossier courant.
- Tout est rangé en bits (0 ou 1), groupés par huit en octets.
- Un texte est une suite de numéros : ASCII pour l'anglais, Unicode (en UTF-8) pour tout le reste.
- Le système d'exploitation partage la machine ; un processus est un programme en train de tourner.
- Le code doit être traduit en langage machine, par un compilateur ou un interpréteur.

Suite : [penser comme un programmeur](algorithmes.md).
