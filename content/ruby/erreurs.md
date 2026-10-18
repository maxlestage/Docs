# Gérer les erreurs

Cette page explique comment attraper les erreurs avec `begin` et `rescue`, comment en lever avec `raise` et comment éviter les pièges de `nil`.

## Une erreur arrête le programme

Quand Ruby rencontre un problème, il lève une **exception**. Sans traitement, le
programme s'arrête.

```ruby norun
puts 10 / 0
# => ZeroDivisionError: divided by 0
```

## begin et rescue

`rescue` attrape l'erreur. Le programme continue au lieu de s'arrêter.

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

## ensure

Le code dans `ensure` s'exécute **toujours**, qu'il y ait une erreur ou non. Il sert à
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

Dans une méthode, `begin` est inutile : `rescue` se place directement avant `end`.

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

`raise` déclenche une erreur vous-même, par exemple pour refuser une valeur.

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

Créez une classe qui hérite de `StandardError`. Le nom finit souvent par `Error`.

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

Appeler une méthode sur `nil` provoque une `NoMethodError`. L'opérateur `&.` évite ce
piège : si l'objet vaut `nil`, le résultat est `nil`.

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

## En résumé

- Une erreur non traitée arrête le programme.
- `begin` et `rescue` attrapent une erreur ; `=> e` donne son message.
- `ensure` s'exécute toujours, avec ou sans erreur.
- `raise` lève une erreur ; vos propres erreurs héritent de `StandardError`.
- `&.` évite l'erreur quand une valeur vaut `nil`.

Suite : apprenez à [lire et écrire des fichiers](fichiers.md). Comparez avec
[les erreurs en Rust](../rust/erreurs.md).
