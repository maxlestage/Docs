# Gérer les erreurs

Cette page explique comment attraper les erreurs avec `begin` et `rescue`, comment en lever avec `raise` et comment éviter les pièges de `nil`.

> **Objectifs :**
>
> - vous saurez lire un message d'erreur et trouver la ligne fautive ;
> - vous saurez attraper une erreur avec `begin` et `rescue` ;
> - vous saurez exécuter du code dans tous les cas avec `ensure` ;
> - vous saurez lever vos propres erreurs avec `raise` ;
> - vous saurez éviter les erreurs sur `nil` avec `&.`.

## Une erreur arrête le programme

Quand Ruby rencontre un problème, il lève une **exception** : un objet qui décrit
l'[erreur](../debuter/glossaire.md#erreur). « Lever » veut dire : déclencher l'alarme.
Sans traitement, le programme s'arrête.

```ruby norun
puts "Début"
puts 10 / 0
puts "Fin"
```

```text
Début
main.rb:2:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:2:in `<main>'
```

`Fin` ne s'affiche jamais : le programme s'arrête à la ligne 2.

### Lire le message

Le message se lit en trois morceaux :

- `main.rb:2` : le fichier et la ligne ;
- `divided by 0` : la description (« divisé par 0 ») ;
- `(ZeroDivisionError)` : la **classe** de l'erreur, son nom de famille.

Les lignes `from ...` forment la **trace d'appels** : le chemin suivi par le programme
jusqu'à l'erreur, de la méthode la plus profonde jusqu'au programme principal
(`<main>`). Cherchez la première ligne qui parle de **votre** fichier.

### Les erreurs les plus courantes

| Classe | Quand ? |
|--------|---------|
| `NoMethodError` | méthode inconnue, souvent appelée sur `nil` |
| `NameError` | variable ou constante inconnue (faute de frappe) |
| `ArgumentError` | mauvais nombre ou mauvaise valeur d'arguments |
| `TypeError` | mauvais type, par exemple `"a" + 1` |
| `ZeroDivisionError` | division entière par zéro |
| `KeyError` | clé absente avec `fetch` |
| `Errno::ENOENT` | fichier introuvable |

## begin et rescue

`rescue` (« secourir ») attrape l'erreur. Le programme continue au lieu de s'arrêter.
C'est un filet de sécurité sous un trapéziste : la chute est rattrapée, le spectacle
continue.

- Le code risqué va entre `begin` et `rescue`.
- Le code de secours va entre `rescue` et `end`.

```ruby
begin
  puts 10 / 0
rescue ZeroDivisionError
  puts "On ne divise pas par zéro !"
end
puts "Le programme continue."
```

```sortie
On ne divise pas par zéro !
Le programme continue.
```

### Lire le message

Avec `=> e`, vous récupérez l'exception et son message.

```ruby
begin
  Integer("abc")
rescue ArgumentError => e
  puts "Erreur : #{e.message}"
end
```

```sortie
Erreur : invalid value for Integer(): "abc"
```

> **Attention :** attrapez des erreurs précises (`ArgumentError`, `ZeroDivisionError`…).
> Un `rescue` trop large peut cacher de vrais bugs.

### Plusieurs rescue et else

Vous pouvez écrire plusieurs `rescue`, un par classe d'erreur. Le code dans `else`
s'exécute seulement s'il n'y a **pas** eu d'erreur.

```ruby
def diviser(texte_a, texte_b)
  resultat = Integer(texte_a) / Integer(texte_b)
rescue ArgumentError
  puts "Ce n'est pas un nombre."
rescue ZeroDivisionError
  puts "Division par zéro."
else
  puts "Résultat : #{resultat}"
end

diviser("10", "2")
diviser("dix", "2")
diviser("10", "0")
```

```sortie
Résultat : 5
Ce n'est pas un nombre.
Division par zéro.
```

> **Note :** `Integer("dix")` lève une erreur, alors que `"dix".to_i` renvoie `0` sans
> rien dire. Utilisez `Integer(...)` quand une saisie invalide doit être repérée.

## ensure

Le code dans `ensure` (« s'assurer ») s'exécute **toujours**, qu'il y ait une erreur ou non. Il sert à
nettoyer (fermer un fichier, par exemple).

```ruby
begin
  puts "Ouverture"
  raise "oups"
rescue => e
  puts "Problème : #{e.message}"
ensure
  puts "Fermeture"
end
```

```sortie
Ouverture
Problème : oups
Fermeture
```

## rescue dans une méthode

Dans une méthode, `begin` est inutile : `rescue` se place directement avant `end`. Il
protège tout le corps de la méthode.

```ruby
def lire_nombre(texte)
  Integer(texte)
rescue ArgumentError
  0
end

puts lire_nombre("42")
puts lire_nombre("quarante")
```

```sortie
42
0
```

## Lever une erreur avec raise

`raise` déclenche une erreur vous-même, par exemple pour refuser une valeur. Donnez la
classe de l'erreur, puis un message clair. Sans classe, `raise "message"` lève une
`RuntimeError`.

```ruby
def retirer(solde, montant)
  raise ArgumentError, "montant négatif" if montant < 0
  solde - montant
end

begin
  retirer(100, -5)
rescue ArgumentError => e
  puts e.message
end
```

```sortie
montant négatif
```

## Vos propres exceptions

Une erreur est un objet comme un autre : sa classe hérite d'une classe d'erreur
(voir [l'héritage](types.md#l-heritage)). Créez une classe qui hérite de
`StandardError`. Le nom finit souvent par `Error`.

```ruby
class SoldeInsuffisantError < StandardError
end

begin
  raise SoldeInsuffisantError, "solde insuffisant"
rescue SoldeInsuffisantError => e
  puts "Refusé : #{e.message}"
end
```

```sortie
Refusé : solde insuffisant
```

> **À retenir :** héritez de `StandardError`, pas de `Exception`. Un simple `rescue`
> n'attrape que les `StandardError`.

## Réessayer avec retry

`retry` relance le bloc `begin` depuis le début. Limitez toujours le nombre d'essais.

```ruby
essais = 0
begin
  essais += 1
  raise "réseau indisponible" if essais < 3
  puts "Connecté après #{essais} essais"
rescue
  retry if essais < 3
end
```

```sortie
Connecté après 3 essais
```

## Se protéger de nil

`nil` (« rien ») arrive souvent sans prévenir : une case de tableau qui n'existe pas,
une clé absente… Appeler une méthode sur `nil` provoque une `NoMethodError`.

L'opérateur `&.`, appelé **navigation sûre**, évite ce piège : si l'objet vaut `nil`,
la méthode n'est pas appelée et le résultat est `nil`. C'est comme frapper à une porte
seulement s'il y a une maison.

```ruby
utilisateur = nil
p utilisateur&.upcase

nom = "alice"
p nom&.upcase
```

```sortie
nil
"ALICE"
```

Combinez-le avec `||` pour donner une valeur par défaut :

```ruby
ville = nil
puts ville&.capitalize || "Inconnue"
```

```sortie
Inconnue
```

> **Astuce :** sur un hash, `fetch` avec une valeur par défaut évite aussi les `nil`
> (voir [les hashs](collections.md#les-hashs)).

## Erreurs fréquentes

### Attraper la mauvaise classe d'erreur

```ruby norun
begin
  Integer("abc")
rescue ZeroDivisionError
  puts "Problème"
end
```

```text
<internal:kernel>:307:in `Integer': invalid value for Integer(): "abc" (ArgumentError)
	from main.rb:2:in `<main>'
```

`rescue ZeroDivisionError` n'attrape que les divisions par zéro. Ici, l'erreur est une
`ArgumentError` : elle passe à travers. Indiquez la bonne classe, celle écrite entre
parenthèses dans le message : `rescue ArgumentError`.

### Hériter de Exception au lieu de StandardError

```ruby norun
class MonErreur < Exception
end
begin
  raise MonErreur, "oups"
rescue => e
  puts "Attrapée"
end
```

```text
main.rb:4:in `<main>': oups (MonErreur)
```

Un `rescue` sans classe n'attrape que les `StandardError`. Écrivez
`class MonErreur < StandardError`.

### Appeler une méthode sur nil

```ruby norun
utilisateurs = { alice: "Alice" }
puts utilisateurs[:bob].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts utilisateurs[:bob].upcase
                       ^^^^^^^
```

`utilisateurs[:bob]` vaut `nil`. Prévoyez le cas : `utilisateurs[:bob]&.upcase`, ou
`utilisateurs.fetch(:bob, "inconnu").upcase`.

## Testez-vous

**Question 1 :** qu'affiche ce code ?

```ruby norun
begin
  puts "A"
  raise "panne"
  puts "B"
rescue
  puts "C"
ensure
  puts "D"
end
```

> **Réponse**
>
> `A`, `C`, puis `D`. `B` n'est jamais affiché : après `raise`, Ruby saute directement
> au `rescue`. `ensure` s'exécute toujours.

**Question 2 :** que vaut `nil&.length` ?

> **Réponse**
>
> `nil`. Avec `&.`, la méthode n'est pas appelée sur `nil`, et il n'y a pas d'erreur.

**Question 3 :** écrivez une méthode `age_valide(n)` qui lève une `ArgumentError` si
`n` est négatif, et renvoie `n` sinon.

> **Réponse**
>
> ```ruby
> def age_valide(n)
>   raise ArgumentError, "âge négatif" if n < 0
>   n
> end
>
> puts age_valide(12)
> begin
>   age_valide(-1)
> rescue ArgumentError => e
>   puts e.message
> end
> ```
>
> ```sortie
> 12
> âge négatif
> ```

## En résumé

- Une erreur non traitée arrête le programme ; son message donne la ligne et la classe.
- `begin` et `rescue` attrapent une erreur ; `=> e` donne son message.
- `ensure` s'exécute toujours, avec ou sans erreur.
- `raise` lève une erreur ; vos propres erreurs héritent de `StandardError`.
- `&.` évite l'erreur quand une valeur vaut `nil`.

Suite : [lire et écrire des fichiers](fichiers.md). Comparez avec
[les erreurs en Rust](../rust/erreurs.md).
