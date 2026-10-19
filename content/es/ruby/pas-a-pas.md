# Seguir un programa paso a paso en Ruby

Esta página te enseña a ejecutar un programa Ruby «en tu cabeza», línea a línea, con una tabla que sigue el valor de cada variable.

> **Objetivos:**
>
> - leer un programa línea a línea, en el orden en que Ruby lo ejecuta;
> - rellenar una tabla de seguimiento de las [variables](../debuter/glossaire.md#variable);
> - adivinar la salida de un programa antes de lanzarlo;
> - hacer que el programa muestre sus propios pasos.

## ¿Por qué seguir un programa a mano?

Un ordenador no adivina nada. Lee una línea, la hace y pasa a la siguiente.
Para entender un programa, haz como él: **una línea cada vez**.

Es como seguir una receta de cocina. No lees toda la receta de golpe esperando que
aparezca el pastel. Cascas los huevos, luego añades el azúcar, luego mezclas. Y en cada
paso sabes lo que hay en el bol.

Saber hacer esto te ayuda a:

- entender un ejemplo que te parece mágico;
- encontrar un [bug](../debuter/glossaire.md#bug) sin ayuda;
- resolver los ejercicios, porque puedes comprobar tu idea en papel.

## La herramienta: la tabla de seguimiento

Coge una hoja. Haz una columna por variable y una fila por paso.
En cada línea de código que cambia una variable, escribe su **nuevo** valor.

```text
  | Línea | a | b |      <- una columna por variable
  |-------|---|---|
  |   1   | 5 | – |      <- «–»: la variable todavía no existe
  |   2   | 5 | 3 |
```

Imagina cajas con una etiqueta. Una variable es una caja. Cuando escribes `a = 8`,
vacías la caja `a` y metes `8` dentro. El valor antiguo se **pierde**.

```text
  antes de a = 8       después de a = 8
  +-------+            +-------+
  |   5   |   a        |   8   |   a
  +-------+            +-------+
```

El método, siempre el mismo:

1. **Antes de lanzarlo**, adivina lo que se muestra.
2. Rellena la tabla, línea a línea.
3. Lanza el programa y compara.

> **Recuerda:** una variable solo contiene un valor cada vez. Cuando le das un valor
> nuevo, el antiguo desaparece.

## Programa 1: variables que cambian

Aquí tienes el primer programa. No tiene ni condición ni bucle.

```ruby
a = 5
b = 3
a = a + b
b = a - b
puts a
puts b
```

Antes de lanzarlo, adivina lo que se muestra. Escribe tu respuesta en un papel.

### La tabla de seguimiento

| Línea | Código | a | b | Qué pasa |
|---|---|---|---|---|
| 1 | `a = 5` | 5 | – | la caja `a` recibe 5 |
| 2 | `b = 3` | 5 | 3 | la caja `b` recibe 3 |
| 3 | `a = a + b` | 8 | 3 | Ruby calcula primero `5 + 3` y después guarda 8 en `a` |
| 4 | `b = a - b` | 8 | 5 | `a` **ya** vale 8: `8 - 3` da 5 |
| 5 | `puts a` | 8 | 5 | muestra 8 |
| 6 | `puts b` | 8 | 5 | muestra 5 |

La trampa está en la línea 4. Mucha gente usa el valor **antiguo** de `a`, es decir 5.
Pero en la línea 4, `a` ya ha cambiado.

### La salida

```sortie
8
5
```

En la línea 3, lee el `=` como una flecha hacia la izquierda: «calcula `a + b` y
después pon el resultado en `a`». No es una igualdad de matemáticas.

```text
  a = a + b
  ^   \___/
  |     |
  |     +-- 1. se calcula con los valores actuales: 5 + 3 = 8
  +-------- 2. se guarda el resultado en a
```

Prueba: sustituye la línea 4 por `b = b - a`. Rellena la tabla antes de lanzarlo.
Debes encontrar `8` y luego `-5`.

> **Recuerda:** a la derecha del `=` se usan los valores **del momento**. Mira siempre
> la última fila de tu tabla.

## Programa 2: una condición

Una [condición](../debuter/glossaire.md#condicion) hace **saltar** líneas. En la tabla,
anota qué rama elige Ruby.

```ruby
temperatura = 18
if temperatura > 25
  consejo = "un pantalón corto"
elsif temperatura > 15
  consejo = "un jersey ligero"
else
  consejo = "un abrigo"
end
puts "Hace #{temperatura} grados: llévate #{consejo}."
```

Antes de lanzarlo, adivina lo que se muestra.

### La tabla de seguimiento

| Línea | Código | Prueba | temperatura | consejo |
|---|---|---|---|---|
| 1 | `temperatura = 18` | | 18 | – |
| 2 | `if temperatura > 25` | `18 > 25` → `false` | 18 | – |
| 4 | `elsif temperatura > 15` | `18 > 15` → `true` | 18 | – |
| 5 | `consejo = "un jersey ligero"` | | 18 | `"un jersey ligero"` |
| 9 | `puts ...` | | 18 | `"un jersey ligero"` |

Mira la columna «Línea»: 1, 2, 4, 5, 9. Las líneas 3, 6 y 7 **nunca** se ejecutan.
Ruby ha encontrado una condición verdadera en la línea 4: hace su rama y después salta
directamente detrás del `end`.

```text
  temperatura > 25 ?  --no-->  temperatura > 15 ?  --sí-->  "un jersey ligero"
        |                             |                             |
        sí                            no                            |
        v                             v                             |
 "un pantalón corto"             "un abrigo"                        |
                                                                    v
                                                             puts (línea 9)
```

### La salida

```sortie
Hace 18 grados: llévate un jersey ligero.
```

Prueba: pon `temperatura = 30`. La primera prueba se vuelve verdadera y obtienes
«llévate un pantalón corto». Después prueba `temperatura = 15`: `15 > 15` es **falso**,
así que toca el abrigo.

> **Recuerda:** en un `if ... elsif ... else` se ejecuta una sola rama: la primera cuya
> prueba es verdadera. Anota cada prueba y su resultado en la tabla.

## Programa 3: un bucle que acumula

Un [bucle](../debuter/glossaire.md#bucle) repite líneas. En la tabla se escribe **una
fila por vuelta**.

Metes 10 euros en una hucha mágica. Cada semana, su contenido se duplica. ¿Al cabo de
cuántas semanas supera los 50 euros?

```ruby
dinero = 10
semana = 0
while dinero < 50
  semana += 1
  dinero = dinero * 2
end
puts "Semana #{semana}: #{dinero} euros"
```

Antes de lanzarlo, adivina lo que se muestra.

### La tabla de seguimiento

| Paso | Prueba `dinero < 50` | semana | dinero |
|---|---|---|---|
| antes del bucle | | 0 | 10 |
| vuelta 1 | `10 < 50` → `true` | 1 | 20 |
| vuelta 2 | `20 < 50` → `true` | 2 | 40 |
| vuelta 3 | `40 < 50` → `true` | 3 | 80 |
| fin | `80 < 50` → `false` | 3 | 80 |

La prueba se hace **al principio** de cada vuelta. En la vuelta 3, `dinero` todavía
vale 40 en el momento de la prueba: se entra. Después pasa a 80. La prueba siguiente es
falsa: el bucle se detiene.

```text
          +-------------------------+
          v                         |
   dinero < 50 ? --sí--> semana += 1
          |              dinero = dinero * 2 ----+
          no
          |
          v
        puts
```

### La salida

```sortie
Semana 3: 80 euros
```

Prueba: sustituye `50` por `80`. En la vuelta 4, la prueba `80 < 80` es falsa. El
resultado no cambia: «Semana 3: 80 euros». Después sustituye `<` por `<=`: hay una
vuelta más y obtienes «Semana 4: 160 euros».

> **Recuerda:** para un bucle, haz una fila por vuelta y anota la prueba **antes** de
> hacer la vuelta. La última fila de la tabla es la prueba que falla.

## Programa 4: un método llamado dos veces

Un [método](../debuter/glossaire.md#metodo) es como una cafetera. Metes algo dentro
(el [parámetro](../debuter/glossaire.md#parametro)), trabaja y te devuelve un
resultado. Puedes usarlo varias veces.

```ruby
def precio_con_iva(precio)
  iva = precio * 20 / 100
  precio + iva
end

libro = precio_con_iva(10)
boli = precio_con_iva(5)
puts "Libro: #{libro}"
puts "Boli: #{boli}"
puts "Total: #{libro + boli}"
```

Antes de lanzarlo, adivina lo que se muestra.

### ¿Quién llama a quién?

Las líneas 1 a 4 no hacen **nada** de inmediato: describen la máquina. El programa
empieza de verdad en la línea 6.

```text
  programa principal                          método precio_con_iva
  ------------------                          ---------------------
  línea 6: precio_con_iva(10)  ---- 10 ---->  precio = 10
                                              iva = 2
           libro = 12          <--- 12 -----  devuelve 12

  línea 7: precio_con_iva(5)   ----  5 ---->  precio = 5
                                              iva = 1
           boli = 6            <---  6 -----  devuelve 6
```

### La tabla de seguimiento

Las variables del método (`precio`, `iva`) son nuevas en **cada** llamada.
Desaparecen cuando el método ha terminado.

| Paso | ¿Dónde? | precio | iva | libro | boli |
|---|---|---|---|---|---|
| línea 6: llamada | método | 10 | – | – | – |
| línea 2 | método | 10 | 2 | – | – |
| línea 3: devuelve 12 | método | 10 | 2 | – | – |
| vuelta a la línea 6 | principal | – | – | 12 | – |
| línea 7: llamada | método | 5 | – | 12 | – |
| línea 2 | método | 5 | 1 | 12 | – |
| línea 3: devuelve 6 | método | 5 | 1 | 12 | – |
| vuelta a la línea 7 | principal | – | – | 12 | 6 |

En la línea 2, Ruby calcula de izquierda a derecha: `5 * 20` da 100 y después
`100 / 100` da 1. La última línea del método (`precio + iva`) es el valor devuelto: es
el retorno implícito (mira [los métodos](fonctions.md#el-retorno-implicito)).

### La salida

```sortie
Libro: 12
Boli: 6
Total: 18
```

Prueba: añade `puts iva` al final del programa. Ruby se detiene
con un error `NameError` («undefined local variable or method»): `iva` solo existe
**dentro** del método.
Después quita esa línea y llama a `precio_con_iva(7)`: `7 * 20 / 100` da 1 (división
entera), así que el precio es 8.

> **Recuerda:** en cada llamada, salta dentro del método con el nuevo valor del
> parámetro, haz sus líneas y vuelve con el resultado al lugar de la llamada.

## Programa 5: recorrer un array con una condición

Último programa: un bucle sobre un array, con pruebas dentro. Contamos las notas
aprobadas y buscamos la mejor.

```ruby
notas = [12, 7, 15, 9, 18]
aprobadas = 0
mejor = 0
notas.each do |nota|
  if nota >= 10
    aprobadas += 1
  end
  if nota > mejor
    mejor = nota
  end
end
puts "Aprobadas: #{aprobadas}"
puts "Mejor: #{mejor}"
```

Antes de lanzarlo, adivina lo que se muestra.

### La tabla de seguimiento

`each` da los elementos uno a uno a `nota`, de izquierda a derecha, como un profesor
que coge los exámenes del montón.

| Vuelta | nota | ¿`nota >= 10`? | aprobadas | ¿`nota > mejor`? | mejor |
|---|---|---|---|---|---|
| antes | – | | 0 | | 0 |
| 1 | 12 | sí | 1 | `12 > 0` sí | 12 |
| 2 | 7 | no | 1 | `7 > 12` no | 12 |
| 3 | 15 | sí | 2 | `15 > 12` sí | 15 |
| 4 | 9 | no | 2 | `9 > 15` no | 15 |
| 5 | 18 | sí | 3 | `18 > 15` sí | 18 |

Cuando una prueba es falsa, la variable **conserva** su valor: se copia la fila de
arriba.

```text
  notas :     [ 12 ,  7 , 15 ,  9 , 18 ]
                ^     ^    ^    ^    ^
  vuelta :      1     2    3    4    5
  aprobada :    ok    -    ok   -    ok    -> 3
  récord :      12    -    15   -    18    -> 18
```

### La salida

```sortie
Aprobadas: 3
Mejor: 18
```

Prueba: sustituye el array por `[-4, -2, -9]`. La mejor nota mostrada es 0, ¡aunque no
está en el array! Tu tabla de seguimiento muestra por qué: `mejor` empieza en 0 y
ninguna prueba `nota > mejor` es verdadera. La corrección: empezar con
`mejor = notas[0]`.

> **Recuerda:** para un bucle sobre un array, una columna para el elemento de la
> vuelta, una columna por prueba y una columna por variable que cambia. Cuando una
> prueba es falsa, copia el valor anterior.

## Hacer que el programa muestre los pasos

Rellenar una tabla a mano es el mejor entrenamiento. Pero también puedes pedirle al
programa que muestre **su propia** tabla de seguimiento. Añade un `puts` con etiquetas
dentro del bucle:

```ruby
dinero = 10
semana = 0
while dinero < 50
  semana += 1
  dinero = dinero * 2
  puts "vuelta #{semana}: semana = #{semana}, dinero = #{dinero}"
end
puts "Fin: semana #{semana}, #{dinero} euros"
```

```sortie
vuelta 1: semana = 1, dinero = 20
vuelta 2: semana = 2, dinero = 40
vuelta 3: semana = 3, dinero = 80
Fin: semana 3, 80 euros
```

Las etiquetas (`semana = `, `dinero = `) son importantes. Sin ellas, ves una columna de
números y ya no sabes cuál es cuál.

`p` es práctico para ver **exactamente** lo que contiene una variable: muestra las
comillas de un texto y muestra `nil` tal cual.

```ruby
palabras = ["gato", "", "perro"]
palabras.each do |palabra|
  print "palabra = "
  p palabra
end
```

```sortie
palabra = "gato"
palabra = ""
palabra = "perro"
```

Con `puts`, el texto vacío habría dado una línea con solo `palabra = `: fácil de pasar
por alto.

Prueba: en el programa 5, añade
`puts "vuelta: nota = #{nota}, aprobadas = #{aprobadas}, mejor = #{mejor}"` justo antes
del `end` del bucle. Compara con tu tabla.

> **Recuerda:** un `puts` con etiquetas dentro del bucle muestra la tabla de
> seguimiento él solo. Quita esas líneas cuando el programa funcione.

## El depurador, en dos frases

Ruby también puede poner tu programa en pausa: escribe `binding.irb` en una línea y
podrás teclear el nombre de cada variable para ver su valor en ese momento. El
depurador `debug` va más lejos y avanza línea a línea; todo se explica en
[el capítulo sobre la depuración](debogage.md).

> **Recuerda:** el depurador hace la tabla de seguimiento por ti, pero hay que saber
> hacerla a mano para entender lo que muestra.

## Errores frecuentes

### Reutilizar un valor antiguo

Queremos intercambiar el contenido de dos variables. Escribimos dos líneas y olvidamos
que la primera ya ha cambiado `a`.

```ruby
a = "rojo"
b = "azul"
a = b
b = a
puts a
puts b
```

```sortie
azul
azul
```

La tabla de seguimiento muestra la avería: después de la línea 3, `a` vale `"azul"` y
`"rojo"` se ha perdido. Así que la línea 4 copia `"azul"`. La corrección: una caja más
para guardar el valor antiguo, o el intercambio de Ruby en una línea.

```ruby
a = "rojo"
b = "azul"
a, b = b, a
puts a
puts b
```

```sortie
azul
rojo
```

### Olvidar el paso «antes del bucle»

Queremos sumar números, pero olvidamos crear el total antes del bucle.

```ruby norun
[3, 4].each do |n|
  total += n
end
```

```text
a.rb:2:in `block in <main>': undefined method `+' for nil (NoMethodError)

  total += n
        ^
	from a.rb:1:in `each'
	from a.rb:1:in `<main>'
```

En la tabla, la fila «antes» estaría vacía: `total` no tiene valor, así que vale `nil`,
y no se puede sumar un número a `nil`. La corrección: escribir `total = 0` **antes**
del bucle.

### Volver a cero en cada vuelta

Otra versión del mismo olvido: el total se crea **dentro** del bucle.

```ruby
[3, 4, 5].each do |n|
  total = 0
  total += n
  puts total
end
```

```sortie
3
4
5
```

La tabla de seguimiento muestra que `total` vuelve a empezar en 0 en cada vuelta. Saca
`total = 0` del bucle: obtendrás 3, 7, 12.

### Una vuelta de más o de menos

`(1..3)` contiene 3 números, pero `(1...3)` solo contiene 2. A menudo nos equivocamos
en una sola vuelta.

```ruby
(1...3).each do |i|
  puts "vuelta #{i}"
end
```

```sortie
vuelta 1
vuelta 2
```

Para evitarlo, escribe en tu tabla la **primera** y la **última** vuelta antes de
rellenar el resto. Mira [los rangos](intervalles.md).

## Ponte a prueba

1. ¿Cuánto vale `x` al final?

   ```ruby
   x = 2
   x = x * 3
   x = x + x
   puts x
   ```

   ```sortie
   12
   ```

> **Respuesta**
>
> 12. Línea 1: `x` vale 2. Línea 2: `2 * 3`, o sea 6. Línea 3: `6 + 6`, o sea 12. En
> cada línea se usa el valor que tiene `x` **en ese momento**.

2. ¿Cuántas vueltas da este bucle y qué muestra?

   ```ruby
   n = 1
   while n < 20
     n = n + 5
   end
   puts n
   ```

   ```sortie
   21
   ```

> **Respuesta**
>
> 4 vueltas. `n` pasa de 1 a 6, 11, 16 y luego 21. La prueba `21 < 20` es falsa: el
> bucle se detiene y muestra 21.
>
> | Vuelta | Prueba | n |
> |---|---|---|
> | antes | | 1 |
> | 1 | `1 < 20` sí | 6 |
> | 2 | `6 < 20` sí | 11 |
> | 3 | `11 < 20` sí | 16 |
> | 4 | `16 < 20` sí | 21 |
> | fin | `21 < 20` no | 21 |

3. ¿Por qué las líneas de un método no se ejecutan cuando Ruby lee el `def`?

> **Respuesta**
>
> El `def` solo describe el método, como una receta escrita en un libro. Solo se hace
> cuando se **llama**, por ejemplo `precio_con_iva(10)`. En la tabla de seguimiento,
> por tanto, se salta el `def` y se empieza en la primera línea de fuera.

## En resumen

- Un ordenador ejecuta una línea cada vez: síguelo de la misma forma.
- La tabla de seguimiento tiene una columna por variable y una fila por paso (o por
  vuelta).
- A la derecha del `=`, usa siempre el valor **del momento**, el de la última fila de
  la tabla.
- Para una condición, anota la prueba y su resultado; para un bucle, anota la prueba
  **antes** de cada vuelta; para un método, salta dentro y vuelve con el resultado.
- `puts` con etiquetas, o `p`, muestra los pasos; el depurador va más lejos.

Siguiente: [las trampas del principiante en Ruby](pieges.md)
