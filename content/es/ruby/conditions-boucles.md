# Las condiciones

Esta página muestra cómo tomar decisiones con `if`, `unless` y `case`, y cómo combinar varias condiciones.

> **Objetivos:**
>
> - sabrás ejecutar código solo si una condición es verdadera;
> - sabrás combinar condiciones con `&&`, `||` y `!`;
> - sabrás elegir entre varios casos con `case`.

## if, elsif y else

Una [condición](../debuter/glossaire.md#condicion) es una pregunta que se responde
con verdadero o falso: «¿la nota es mayor que 10?». Es un cruce de caminos: según
la respuesta, el programa toma un camino u otro.

`if` ejecuta código solo si una condición es verdadera. El bloque termina con `end`.

```ruby
nota = 14
if nota >= 16
  puts "Muy bien"
elsif nota >= 10
  puts "Aprobado"
else
  puts "Suspenso"
end
```

```sortie
Aprobado
```

- `if` («si») hace la primera pregunta.
- `elsif` («si no, si») hace otra pregunta, solo si las anteriores han
  respondido que no. Cuidado con la ortografía: `elsif`, sin `e` en medio.
- `else` («si no») se ejecuta si ninguna pregunta ha respondido que sí.
- Ruby se detiene en la **primera** rama verdadera.

Línea por línea:

- `nota = 14` guarda 14 en la variable `nota`;
- `if nota >= 16` pregunta: «¿14 es mayor o igual que 16?». No: Ruby se salta este
  bloque;
- `elsif nota >= 10` pregunta: «¿14 es mayor o igual que 10?». Sí: Ruby muestra
  «Aprobado»;
- una rama ha respondido que sí: Ruby se salta el `else` y sigue después del `end`.

Las comparaciones (`==`, `!=`, `<`, `>`, `<=`, `>=`) se presentan en
[los primeros pasos](premiers-pas.md).

**Te toca:** escribe un programa que guarde `hora = 15` en una variable y luego
muestre «Buenos días» si la hora es antes de las 12, «Buenas tardes» si es antes de
las 18, y «Buenas noches» si no.

> **Solución**
>
> ```ruby
> hora = 15
> if hora < 12
>   puts "Buenos días"
> elsif hora < 18
>   puts "Buenas tardes"
> else
>   puts "Buenas noches"
> end
> ```
>
> ```sortie
> Buenas tardes
> ```

### La forma corta

Para una sola instrucción, puedes poner `if` al final de la línea.

```ruby
temperatura = 35
puts "¡Hace calor!" if temperatura > 30
```

```sortie
¡Hace calor!
```

### if devuelve un valor

En Ruby, `if` da un valor: el de la rama elegida. Puedes guardarlo
en una variable.

```ruby
nota = 8
resultado = if nota >= 10
              "aprobado"
            else
              "suspenso"
            end
puts resultado
```

```sortie
suspenso
```

## unless

A veces la pregunta sale más natural al revés: «si **no** ha iniciado sesión, pídele
que la inicie». `unless` te deja escribirlo tal como lo dices.

`unless` («a menos que») es lo contrario de `if`: el código se ejecuta si la condición
es **falsa**.

```ruby
conectado = false
puts "Inicia sesión, por favor." unless conectado
```

```sortie
Inicia sesión, por favor.
```

> **Consejo:** evita `unless` con `else`: es difícil de leer. Usa mejor `if`.

## Combinar condiciones

Una decisión depende a menudo de varias cosas a la vez. Para entrar gratis al museo,
hay que ser joven **y** socio. Estos tres signos unen varias pruebas en una sola.

`&&` quiere decir «y», `||` quiere decir «o», `!` quiere decir «no».

- `a && b` es verdadero si `a` **y** `b` son verdaderos.
- `a || b` es verdadero si al menos uno de los dos es verdadero.
- `!a` es verdadero si `a` es falso.

```ruby
edad = 20
socio = true
puts "Entrada gratuita" if edad < 26 && socio
puts "No es socio" if !socio
puts "Tarifa reducida" if edad < 18 || edad > 65
puts "Tarifa completa" unless edad < 18 || edad > 65
```

```sortie
Entrada gratuita
Tarifa completa
```

Línea por línea:

- `edad` vale 20 y `socio` vale `true`;
- línea 3: `edad < 26` es verdadero **y** `socio` es verdadero: se muestra «Entrada
  gratuita»;
- línea 4: `!socio` convierte `true` en `false`: no se muestra nada;
- línea 5: 20 no es menor que 18 ni mayor que 65: el `||` es falso;
- línea 6: la misma condición es falsa, y `unless` actúa justamente cuando es falsa.

`&&` va antes que `||`, como `*` va antes que `+`. Añade paréntesis para que quede
claro:

```ruby
edad = 70
estudiante = false
socio = false
puts "Descuento" if (edad > 65 || estudiante) && !socio
```

```sortie
Descuento
```

> **Nota:** Ruby conoce también `and`, `or` y `not`. No tienen la misma precedencia
> que `&&` y `||`: para empezar, usa solo `&&`, `||` y `!`.

**Te toca:** escribe un programa que guarde `edad = 70` en una variable y luego
muestre «Tarifa reducida» si la edad es menor que 18 o mayor o igual que 65, y
«Tarifa completa» si no.

> **Solución**
>
> ```ruby
> edad = 70
> if edad < 18 || edad >= 65
>   puts "Tarifa reducida"
> else
>   puts "Tarifa completa"
> end
> ```
>
> ```sortie
> Tarifa reducida
> ```

## El ternario

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

`condición ? valor_si_verdadero : valor_si_falso` elige un valor en una línea.

```ruby
edad = 15
estado = edad >= 18 ? "mayor de edad" : "menor de edad"
puts estado
```

```sortie
menor de edad
```

Léelo como una pregunta: «¿edad ≥ 18? si sí, mayor de edad; si no, menor de edad».

## case y when

Piensa en una máquina de bebidas: según el botón que pulses, sirve una bebida
distinta. Cuando un mismo valor puede tomar muchas formas (un día, una opción de un
menú), quieres una reacción para cada una.

`case` compara un valor con varias posibilidades. Es más legible que una larga
serie de `elsif`.

```ruby
dia = "sábado"
case dia
when "sábado", "domingo"
  puts "¡Fin de semana!"
when "lunes"
  puts "Ánimo..."
else
  puts "Día laborable"
end
```

```sortie
¡Fin de semana!
```

- Una coma en `when` quiere decir «o».
- `else` sirve si ningún `when` coincide.

Línea por línea:

- `dia` vale `"sábado"`;
- `case dia` anuncia el valor que se va a comparar;
- primer `when`: ¿`dia` vale `"sábado"` o `"domingo"`? Sí: Ruby muestra
  «¡Fin de semana!»;
- los demás `when` y el `else` se saltan, como con `elsif`.

`when` acepta también rangos:

```ruby
nota = 12
calificacion = case nota
               when 0...10 then "insuficiente"
               when 10...14 then "aprobado"
               else "notable"
               end
puts calificacion
```

```sortie
aprobado
```

`then` permite escribir la respuesta en la misma línea que `when`.

## Errores frecuentes

### Olvidar end

```ruby norun
nota = 14
if nota >= 10
  puts "Aprobado"
```

```text
main.rb: --> main.rb
Unmatched keyword, missing `end' ?
> 2  if nota >= 10
main.rb:3: syntax error, unexpected end-of-input, expecting `end' or dummy end (SyntaxError)
  puts "Aprobado"
                 ^
```

Cada `if`, `case`, `while`, `until` y `do` debe cerrarse con un `end`. Añade
`end` después de `puts "Aprobado"`. Indenta tu código: un `end` que falta se ve
enseguida.

### Escribir = en lugar de ==

```ruby norun
nota = 14
if nota = 10
  puts "diez"
end
```

```text
main.rb:2: warning: found `= literal' in conditional, should be ==
diez
```

`nota = 10` **guarda** 10 en `nota`: la condición es siempre verdadera. Ruby avisa
(*warning*: «advertencia»), pero el programa continúa. Escribe `if nota == 10`.

### Escribir elseif o else if

```ruby norun
nota = 12
if nota >= 10
  puts "Aprobado"
elseif nota >= 5
  puts "Recuperación"
end
```

```text
Aprobado
main.rb:4:in `<main>': undefined method `elseif' for main (NoMethodError)

elseif nota >= 5
^^^^^^
Did you mean?  elsif
```

Ruby lee `elseif` como un método desconocido. La palabra clave se escribe `elsif`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```ruby norun
x = 5
puts "A" if x > 3 && x < 5
puts "B" if x > 3 || x < 5
```

> **Respuesta**
>
> Solo `B`. `x < 5` es falso (5 no es menor que 5), así que el `&&` es falso.
> Con `||`, basta con que `x > 3` sea verdadero.

**Pregunta 2:** ¿qué muestra este código?

```ruby norun
conectado = true
puts "Bienvenido" unless conectado
puts "Adiós"
```

> **Respuesta**
>
> Solo `Adiós`. `unless` ejecuta el código cuando la condición es **falsa**: aquí
> `conectado` es verdadero, así que «Bienvenido» no se muestra.

**Pregunta 3:** ¿cuánto vale `actividad`?

```ruby norun
dia = "domingo"
actividad = case dia
            when "sábado", "domingo" then "descanso"
            else "trabajo"
            end
```

> **Respuesta**
>
> `"descanso"`. En un `when`, la coma significa «o»: `"domingo"` coincide con el
> primer `when`, y `case` devuelve el valor de esa rama.

## En resumen

- `if`, `elsif` y `else` eligen qué hacer; `unless` es lo contrario de `if`.
- `if` devuelve un valor: el de la rama elegida.
- `&&` (y), `||` (o), `!` (no) combinan condiciones.
- `case` y `when` comparan un valor con varias posibilidades.

Siguiente: [los bucles](boucles.md).
