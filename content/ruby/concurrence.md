# Faire plusieurs choses à la fois

Cette page montre comment lancer plusieurs tâches en même temps avec les threads de Ruby, comment partager des données sans les abîmer, et quand cela accélère vraiment un programme.

> **Objectifs :**
>
> - lancer un thread avec `Thread.new` et l'attendre avec `join` ;
> - récupérer le résultat d'un thread avec `value` ;
> - protéger une donnée partagée avec un `Mutex` ;
> - faire travailler des threads ensemble avec une `Queue` ;
> - savoir quand les threads aident, et quand ils n'aident pas.

## Qu'est-ce qu'un thread ?

Un programme Ruby suit d'habitude **un seul fil** : une instruction après l'autre.
Un [thread](../debuter/glossaire.md#thread) (« fil » en anglais) est un deuxième fil
d'exécution, qui avance à côté du premier.

Pensez à une cuisine. Un seul cuisinier fait tout, dans l'ordre : il attend que l'eau
bouille avant de couper les légumes. Avec deux cuisiniers, l'un surveille l'eau pendant
que l'autre coupe. Chaque cuisinier est un thread.

## Lancer un thread et l'attendre

`Thread.new` lance un thread. Le code à exécuter va dans un
[bloc](../debuter/glossaire.md#bloc). `join` (« rejoindre ») attend que le thread ait
fini.

```ruby
t = Thread.new do
  puts "Bonjour depuis le thread"
end

t.join
puts "Le thread a fini."
```

```sortie
Bonjour depuis le thread
Le thread a fini.
```

> **À retenir :** quand le programme principal se termine, tous les threads s'arrêtent,
> même s'ils n'ont pas fini. Appelez toujours `join`.

### Plusieurs threads

Lancez plusieurs threads, rangez-les dans un tableau, puis attendez-les tous avec
`each(&:join)`.

```ruby
threads = [1, 2, 3].map do |n|
  Thread.new { n * 10 }
end

threads.each(&:join)
puts "Les #{threads.size} threads ont fini."
```

```sortie
Les 3 threads ont fini.
```

## Récupérer le résultat avec value

La dernière valeur du bloc est le résultat du thread. `value` attend le thread (comme
`join`) puis renvoie ce résultat.

```ruby
threads = [2, 3, 4].map do |n|
  Thread.new { n * n }
end

carres = threads.map(&:value)
p carres
```

```sortie
[4, 9, 16]
```

Les résultats arrivent dans l'ordre du tableau `threads`, quel que soit le thread qui
a fini en premier.

### L'ordre d'exécution n'est pas garanti

Les threads avancent chacun à leur rythme. Si chacun affiche quelque chose, l'ordre
peut changer d'une exécution à l'autre. Pour un résultat stable, faites renvoyer une
valeur par chaque thread, puis triez ou affichez **après** les `join`.

```ruby
noms = ["Chloé", "Alice", "Bruno"]

threads = noms.map do |nom|
  Thread.new { "#{nom} a fini" }
end

messages = threads.map(&:value).sort
puts messages
```

```sortie
Alice a fini
Bruno a fini
Chloé a fini
```

## Attendre en parallèle

Les threads brillent quand le programme **attend** : un téléchargement, un fichier, une
base de données. Ici, `sleep` simule une attente de 0,2 seconde.

```ruby
debut = Time.now

threads = 3.times.map do |i|
  Thread.new do
    sleep 0.2
    "tâche #{i}"
  end
end

p threads.map(&:value)
duree = Time.now - debut
puts duree < 0.5 ? "Environ 0,2 s, pas 0,6 s" : "Trop lent"
```

```sortie
["tâche 0", "tâche 1", "tâche 2"]
Environ 0,2 s, pas 0,6 s
```

Les trois attentes se font en même temps : le programme dure 0,2 seconde au lieu de
0,6.

## Partager une donnée avec un Mutex

Deux threads qui modifient la **même** variable peuvent se marcher dessus. `compteur += 1`
se fait en trois temps : lire, ajouter 1, écrire. Si deux threads lisent la même valeur
en même temps, un ajout se perd.

Un **mutex** (pour *mutual exclusion*, « exclusion mutuelle ») est un verrou. C'est la
clé unique des toilettes d'un café : une seule personne à la fois. `synchronize` prend
la clé, exécute le bloc, puis rend la clé.

```ruby
compteur = 0
verrou = Mutex.new

threads = 4.times.map do
  Thread.new do
    1000.times do
      verrou.synchronize { compteur += 1 }
    end
  end
end

threads.each(&:join)
puts compteur
```

```sortie
4000
```

> **Astuce :** le plus simple reste de **ne rien partager**. Chaque thread calcule sa
> part et la renvoie avec `value` ; le programme principal fait le total.

```ruby
parts = [1..100, 101..200, 201..300].map do |intervalle|
  Thread.new { intervalle.sum }
end

puts parts.sum(&:value)
```

```sortie
45150
```

## Producteur et consommateur avec Queue

Une `Queue` (« file d'attente ») est un tapis roulant entre threads. Un thread y pose
des objets avec `push`, un autre les prend avec `pop`. `Queue` est déjà protégée : pas
besoin de mutex.

- Le **producteur** fabrique du travail et le pose sur le tapis.
- Le **consommateur** prend le travail et le traite.
- `pop` attend tant que la file est vide.

```ruby
file = Queue.new

producteur = Thread.new do
  %w[pomme poire kiwi].each { |fruit| file.push(fruit) }
  file.push(:fin)
end

consommateur = Thread.new do
  traites = []
  while (fruit = file.pop) != :fin
    traites << fruit.upcase
  end
  traites
end

producteur.join
p consommateur.value
```

```sortie
["POMME", "POIRE", "KIWI"]
```

Le symbole `:fin` sert de signal : « il n'y a plus rien à traiter ».

### Plusieurs consommateurs

Avec plusieurs consommateurs, chaque objet n'est pris qu'une fois. Envoyez un signal de
fin par consommateur, et triez le résultat pour qu'il soit stable.

```ruby
file = Queue.new
(1..6).each { |n| file.push(n) }
3.times { file.push(:fin) }

ouvriers = 3.times.map do
  Thread.new do
    faits = []
    while (n = file.pop) != :fin
      faits << n * n
    end
    faits
  end
end

p ouvriers.flat_map(&:value).sort
```

```sortie
[1, 4, 9, 16, 25, 36]
```

## Le GVL : attendre, oui ; calculer, non

Ruby (la version standard, appelée CRuby) a un verrou global : le **GVL** (*Global VM
Lock*). Une seule ligne de Ruby s'exécute à la fois, même avec plusieurs threads et
plusieurs cœurs.

Reprenez la cuisine : il y a deux cuisiniers, mais **un seul couteau**. Pendant que l'un
attend l'eau, l'autre peut couper : le travail avance. Mais si les deux veulent couper
en même temps, ils se passent le couteau à tour de rôle : ce n'est pas plus rapide.

| Situation | Les threads aident ? |
|-----------|----------------------|
| attendre le réseau, un fichier, `sleep` | oui : le GVL est rendu pendant l'attente |
| calculer beaucoup (boucles, maths) | non : un seul calcul à la fois |

> **À retenir :** en Ruby, les threads servent à **attendre** plusieurs choses en même
> temps, pas à calculer plus vite.

## Ractor : du vrai parallélisme

Pour calculer en parallèle sur plusieurs cœurs, Ruby propose les **Ractors** (depuis
Ruby 3.0). Chaque Ractor a son propre verrou et ne partage presque rien avec les autres.
`take` récupère le résultat.

```ruby
r = Ractor.new { (1..10).sum }
puts r.take
```

```sortie
55
```

> **Attention :** les Ractors sont encore **expérimentaux**. Ruby affiche un
> avertissement (`warning: Ractor is experimental`) et beaucoup de bibliothèques ne les
> supportent pas. Pour débuter, contentez-vous des threads.

## Erreurs fréquentes

### Oublier join

```ruby norun
Thread.new { puts "Bonjour du thread" }
puts "Fin"
```

```text
Fin
```

Le programme principal finit avant le thread, qui est arrêté sans avoir rien affiché.
Gardez le thread dans une variable et appelez `join` : `t = Thread.new { … }` puis
`t.join`.

### Attendre une file vide pour toujours

```ruby norun
file = Queue.new
file.pop
```

```text
<internal:thread_sync>:18:in `pop': No live threads left. Deadlock? (fatal)
1 threads, 1 sleeps current:0x0000564a6d1f9090 main thread:0x0000564a6d1f9090
* #<Thread:0x00007fc7ae8ba9c0 sleep_forever>
   rb_thread_t:0x0000564a6d1f9090 native:0x00007fc7af2fa740 int:0
   

	from main.rb:2:in `<main>'
```

`pop` attend un objet, mais personne ne va en poser : c'est un **interblocage**
(*deadlock*). Vérifiez qu'un producteur remplit la file, et qu'il envoie un signal de
fin pour chaque consommateur.

### Une erreur dans un thread

```ruby norun
t = Thread.new { raise ArgumentError, "valeur invalide" }
t.join
puts "Fin"
```

```text
#<Thread:0x00007f5510b0a358 main.rb:1 run> terminated with exception (report_on_exception is true):
main.rb:1:in `block in <main>': valeur invalide (ArgumentError)

t = Thread.new { raise ArgumentError, "valeur invalide" }
                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
main.rb:1:in `block in <main>': valeur invalide (ArgumentError)
```

L'erreur est signalée une fois quand le thread meurt, puis relevée par `join`. Attrapez-la
autour du `join` (ou de `value`) avec `rescue`, comme dans
[gérer les erreurs](erreurs.md#begin-et-rescue).

## Testez-vous

**Question 1 :** quelle est la différence entre `join` et `value` ?

> **Réponse**
>
> Les deux attendent la fin du thread. `join` renvoie le thread lui-même ; `value`
> renvoie le résultat du bloc (sa dernière valeur).

**Question 2 :** vous devez télécharger 10 pages web. Les threads vont-ils aider ? Et
pour calculer 10 grands nombres premiers ?

> **Réponse**
>
> Pour les téléchargements, oui : le programme passe son temps à attendre le réseau, et
> le GVL est rendu pendant l'attente. Pour le calcul, non : à cause du GVL, un seul thread
> calcule à la fois.

**Question 3 :** écrivez un programme qui lance un thread par mot de
`%w[un deux trois]`, chacun renvoyant la longueur du mot, puis affiche le total.

> **Réponse**
>
> ```ruby
> threads = %w[un deux trois].map do |mot|
>   Thread.new { mot.length }
> end
> puts threads.sum(&:value)
> ```
>
> ```sortie
> 11
> ```

## En résumé

- `Thread.new { … }` lance un thread ; `join` l'attend, `value` donne son résultat.
- L'ordre des threads n'est pas garanti : récupérez les résultats, puis triez-les.
- Une donnée partagée se protège avec `Mutex#synchronize` ; mieux, ne partagez rien.
- `Queue` relie un producteur et des consommateurs, sans verrou à écrire.
- À cause du GVL, les threads aident à attendre, pas à calculer ; les Ractors visent le
  vrai parallélisme.

Suite : [Ruby pour le web](web.md). Comparez avec
[la concurrence en Rust](../rust/concurrence.md).
