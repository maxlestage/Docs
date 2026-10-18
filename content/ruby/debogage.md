# Déboguer en Ruby

Cette page montre comment trouver et corriger un bug : lire une trace d'erreur, afficher les valeurs avec `p` et `pp`, mettre le programme en pause, et avancer pas à pas.

> **Objectifs :**
>
> - lire une trace d'appels et trouver la ligne fautive ;
> - afficher l'état d'un programme avec `p`, `pp` et `inspect` ;
> - mettre le programme en pause avec `binding.irb` ou le débogueur `debug` ;
> - activer les avertissements avec `ruby -w` ;
> - suivre une méthode simple pour corriger un bug.

## Qu'est-ce que déboguer ?

Un [bug](../debuter/glossaire.md#bug) est une erreur dans un programme. Le
[débogage](../debuter/glossaire.md#debogage) consiste à le trouver, puis à le corriger.

Déboguer, c'est mener une enquête : observer les indices, faire une hypothèse, la
vérifier. On ne devine pas au hasard. Pour les bases communes à tous les langages, voyez
aussi [déboguer pour débuter](../debuter/deboguer.md).

## Lire une trace d'erreur

Quand une erreur arrête le programme, Ruby affiche une **trace d'appels** (*backtrace*).
Voici un programme qui calcule le total d'un panier ; un article a une faute de frappe.

```ruby norun
def calcul(article)
  article.fetch(:prix) * 2
end

def total(paniers)
  paniers.sum { |a| calcul(a) }
end

puts total([{ prix: 3 }, { prx: 4 }])
```

```text
main.rb:2:in `fetch': key not found: :prix (KeyError)
Did you mean?  :prx
	from main.rb:2:in `calcul'
	from main.rb:6:in `block in total'
	from main.rb:6:in `sum'
	from main.rb:6:in `total'
	from main.rb:9:in `<main>'
```

Lisez-la dans cet ordre :

1. **La première ligne** : où l'erreur a eu lieu (`main.rb:2`), le message
   (`key not found: :prix`) et la classe (`KeyError`).
2. **Le conseil** : `Did you mean?` propose souvent la bonne réponse.
3. **Les lignes `from`** : le chemin parcouru, du plus profond au plus haut. `<main>`
   est le programme principal, ligne 9.

Chaque ligne `from` est une marche d'escalier : `<main>` a appelé `total`, qui a appelé
`calcul`, qui a appelé `fetch`. La cause est parfois plus haut que l'erreur : ici, c'est
la donnée `{ prx: 4 }` de la ligne 9.

> **Astuce :** dans une longue trace, cherchez la première ligne qui parle de **vos**
> fichiers. Les lignes des gems et de Ruby lui-même sont rarement en cause.

## Afficher les valeurs avec p

`puts` affiche une valeur « pour les humains ». `p` l'affiche « pour le programmeur » :
on voit les guillemets, les `nil`, les types.

```ruby
valeurs = ["42", 42, nil, ""]
puts valeurs
puts "---"
p valeurs
```

```sortie
42
42


---
["42", 42, nil, ""]
```

Avec `puts`, impossible de distinguer `"42"` de `42`, ni `nil` de `""`. Avec `p`, tout
est clair.

### inspect et les étiquettes

`p x` affiche en fait `x.inspect`. Utilisez `inspect` vous-même pour afficher plusieurs
valeurs avec une **étiquette** : vous savez ainsi qui est qui.

```ruby
prix = 12
remise = nil
puts "DEBUG prix=#{prix.inspect} remise=#{remise.inspect}"
```

```sortie
DEBUG prix=12 remise=nil
```

> **Astuce :** commencez vos lignes de débogage par `DEBUG`. Vous les retrouverez et les
> supprimerez facilement avant de partager votre code.

### pp pour les grosses structures

`pp` (*pretty print*, « joli affichage ») coupe une grosse structure sur plusieurs lignes.

```ruby
eleve = {
  nom: "Alice",
  notes: [12, 15, 16],
  options: { latin: true, sport: false },
  adresse: { ville: "Lyon", code: "69000" }
}
pp eleve
```

```sortie
{:nom=>"Alice",
 :notes=>[12, 15, 16],
 :options=>{:latin=>true, :sport=>false},
 :adresse=>{:ville=>"Lyon", :code=>"69000"}}
```

> **Note :** `p` renvoie la valeur qu'il affiche. Vous pouvez l'insérer au milieu d'un
> calcul sans rien casser : `total = p(prix) * quantite`.

## Mettre le programme en pause avec binding.irb

Afficher des valeurs oblige à deviner lesquelles. Une autre approche : **arrêter** le
programme à un endroit, et l'explorer. `binding.irb` ouvre une console
[IRB](../debuter/glossaire.md#repl) au milieu du programme, avec toutes les variables
du moment.

```ruby norun
def remise(prix, client)
  taux = client[:fidele] ? 0.1 : 0
  binding.irb
  prix - prix * taux
end

remise(50, { fidele: "non" })
```

```text
From: main.rb @ line 3 :

    1: def remise(prix, client)
    2:   taux = client[:fidele] ? 0.1 : 0
 => 3:   binding.irb
    4:   prix - prix * taux
    5: end
    6: 
    7: remise(50, { fidele: "non" })

irb(main):001> client[:fidele]
=> "non"
irb(main):002> taux
=> 0.1
irb(main):003> exit
```

Vous tapez des expressions, IRB répond. Ici, on découvre que `"non"` est considéré
comme vrai (seuls `nil` et `false` sont faux). `exit` reprend le programme.

## Le débogueur debug

Le débogueur `debug`, livré avec Ruby, va plus loin : il avance **ligne par ligne**.
Placez `binding.break` dans votre code, puis lancez le programme avec `-r debug` :

```ruby norun
def remise(prix, client)
  taux = client[:fidele] ? 0.1 : 0
  binding.break
  prix - prix * taux
end

puts remise(50, { fidele: true })
```

```sh
ruby -r debug main.rb
```

Le programme s'arrête sur `binding.break`. Les commandes principales :

| Commande | Effet |
|----------|-------|
| `p taux` | affiche une valeur |
| `info` | liste les variables locales |
| `next` (ou `n`) | exécute la ligne et passe à la suivante |
| `step` (ou `s`) | entre dans la méthode appelée |
| `bt` | affiche la trace d'appels |
| `continue` (ou `c`) | reprend jusqu'au prochain arrêt |
| `quit` (ou `q`) | arrête tout |

> **Astuce :** les éditeurs comme VS Code savent utiliser `debug` : un clic dans la marge
> pose un point d'arrêt, sans toucher au code.

## Les avertissements avec ruby -w

Ruby peut signaler du code suspect, même quand il fonctionne. L'option `-w` (*warnings*,
« avertissements ») les active.

```ruby norun
def aire(largeur, hauteur)
  resultat = largeur * hauteur
  largeur * hauteur
end

puts aire(3, 4)
```

```sh
ruby -w main.rb
```

```text
main.rb:2: warning: assigned but unused variable - resultat
12
```

Le programme marche, mais la variable `resultat` ne sert à rien : peut-être un oubli.
Lisez les avertissements, ils annoncent souvent un bug.

## Les erreurs les plus courantes

| Message | Cause habituelle | Piste |
|---------|------------------|-------|
| `undefined method 'x' for nil (NoMethodError)` | une valeur vaut `nil` sans que vous le sachiez | cherchez d'où vient la valeur ; `&.` ou une valeur par défaut |
| `undefined method 'frist' for an instance of Array` | faute de frappe dans un nom de méthode | lisez le `Did you mean?` |
| `wrong number of arguments (given 2, expected 1) (ArgumentError)` | trop ou pas assez d'arguments | comparez l'appel et le `def` |
| `no implicit conversion of Integer into String (TypeError)` | mélange de types, comme `"a" + 1` | convertissez avec `to_s` ou utilisez `#{}` |
| `undefined local variable or method 'totl' (NameError)` | variable mal écrite ou pas encore définie | vérifiez l'orthographe et la portée |
| `key not found: :b (KeyError)` | `fetch` sur une clé absente | vérifiez la clé, ou `fetch(:b, défaut)` |

`fetch` est d'ailleurs un bon allié : `hash[:b]` renvoie `nil` en silence, alors que
`hash.fetch(:b)` signale tout de suite la clé manquante.

## Une enquête pas à pas

Voici une méthode qui calcule une moyenne. Elle semble juste.

```ruby
def moyenne(notes)
  total = 0
  notes.each { |n| total += n }
  total / notes.size
end

puts moyenne([12, 15, 16])
```

```sortie
14
```

**1. Le symptôme.** (12 + 15 + 16) / 3 = 43 / 3 ≈ 14,33. Le programme affiche `14`.

**2. Observer.** On affiche les valeurs intermédiaires, avec des étiquettes.

```ruby
notes = [12, 15, 16]
total = 0
notes.each { |n| total += n }
puts "DEBUG total=#{total.inspect} taille=#{notes.size.inspect}"
p total / notes.size
```

```sortie
DEBUG total=43 taille=3
14
```

**3. L'hypothèse.** `total` et `taille` sont justes. Le problème est donc la division.
Ce sont deux entiers : Ruby fait une **division entière**, qui jette les décimales.

**4. Vérifier.** Une division avec un flottant confirme l'hypothèse : `p 43 / 3.0`
affiche `14.333333333333334`.

**5. Corriger**, puis vérifier aussi le cas limite : une liste vide.

```ruby
def moyenne(notes)
  return nil if notes.empty?

  notes.sum.fdiv(notes.size)
end

p moyenne([12, 15, 16]).round(2)
p moyenne([])
```

```sortie
14.33
nil
```

`fdiv` fait une division décimale ; `sum` remplace la boucle. Sans la première ligne,
`moyenne([])` aurait donné `NaN` (*Not a Number*) au lieu d'une réponse claire.

> **À retenir :** symptôme, observation, hypothèse, vérification, correction. Puis
> écrivez un [test](tests.md) pour que le bug ne revienne jamais.

## Erreurs fréquentes

### Déboguer avec puts et se faire piéger par nil

```ruby norun
nom = nil
puts "Nom :"
puts nom
```

```text
Nom :

```

`puts nil` affiche une ligne vide : on croit que `nom` est une chaîne vide. Utilisez
`p nom`, qui affiche clairement `nil`.

### Utiliser binding.break sans charger debug

```ruby norun
x = 42
binding.break
```

```text
main.rb:2:in `<main>': undefined method `break' for an instance of Binding (NoMethodError)

binding.break
       ^^^^^^
```

`binding.break` vient du débogueur. Lancez le programme avec `ruby -r debug main.rb`,
ou ajoutez `require "debug"` en haut du fichier.

### Chercher l'erreur à la mauvaise ligne

```ruby norun
def moyenne(notes)
  notes.sum / notes.size
end

p moyenne([])
```

```text
main.rb:2:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:2:in `moyenne'
	from main.rb:5:in `<main>'
```

L'erreur éclate ligne 2, mais la ligne 2 n'est pas fautive pour autant : c'est l'appel
ligne 5 qui envoie une liste vide. Remontez la trace, puis décidez : refuser la liste
vide, ou la traiter avec `return nil if notes.empty?`.

## Testez-vous

**Question 1 :** quelle est la différence entre `puts x` et `p x` pour déboguer ?

> **Réponse**
>
> `puts` affiche une version lisible, qui cache les guillemets et transforme `nil` en
> ligne vide. `p` affiche `x.inspect` : on voit le type exact et les `nil`.

**Question 2 :** dans une trace d'appels, que représente `<main>` ?

> **Réponse**
>
> Le programme principal, le code écrit hors de toute méthode. C'est la dernière ligne de
> la trace : le point de départ de la chaîne d'appels.

**Question 3 :** ce code affiche `2` au lieu de `2.5`. Corrigez-le.

```ruby norun
puts 5 / 2
```

> **Réponse**
>
> Deux entiers donnent une division entière. Utilisez un flottant ou `fdiv`.
>
> ```ruby
> puts 5.fdiv(2)
> puts 5 / 2.0
> ```
>
> ```sortie
> 2.5
> 2.5
> ```

## En résumé

- Une trace se lit de haut en bas : l'erreur d'abord, puis le chemin jusqu'à `<main>`.
- `p` et `inspect` montrent la vraie valeur ; `pp` met en forme les grosses structures.
- `binding.irb` ouvre une console au milieu du programme ; `ruby -r debug` et
  `binding.break` permettent d'avancer ligne par ligne.
- `ruby -w` signale le code suspect.
- Enquêtez avec méthode : symptôme, observation, hypothèse, correction, test.

Suite : [bien écrire du Ruby](style.md).
