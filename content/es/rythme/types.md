# Tus propios tipos y match

Esta página muestra cómo crear tus propios tipos con `type`, y cómo examinarlos con `match` y sus patrones: listas, registros, guardas y símbolos.

> **Objetivos:**
>
> - sabrás crear un tipo con sus variantes, con o sin campos;
> - sabrás tratar cada variante con `match`, o con una definición por casos;
> - sabrás reconocer listas y registros con patrones;
> - sabrás añadir una guarda a un caso de `match`;
> - sabrás cuándo preferir un símbolo o un `type`.

## Crear un tipo

Un [tipo](../debuter/glossaire.md#tipo) es una clase de valor. Rythme ya
tiene algunos (`Int`, `Text`...), y puedes crear los tuyos.

`type` crea un tipo nuevo, con la lista de sus **variantes**, las formas
posibles de sus valores, separadas por `|` (lee «o»). Los nombres del tipo
y de las variantes empiezan por mayúscula:

```rythme
type Semaforo = Rojo | Naranja | Verde

luz = Naranja
print(luz, type_of(luz))
```

```sortie
Naranja Semaforo
```

Lee: «un `Semaforo` es `Rojo`, o `Naranja`, o `Verde`». Un semáforo no puede
ser otra cosa: no hay errata posible, como con un texto `"naraja"`.

Una variante puede llevar **campos**: entonces se usa como una función que
fabrica un valor. Un campo se lee con un punto, como en un registro:

```rythme
type Forma = Circulo(radio) | Rect(ancho, alto)

c = Circulo(2)
r = Rect(3, 4)
print(c, r, r.ancho)
```

```sortie
Circulo(2) Rect(3, 4) 3
```

Piensa en un formulario con varios modelos: el modelo «círculo» tiene una
casilla, `radio`, el modelo «rectángulo» tiene dos, `ancho` y `alto`.

## Examinar un valor con match

`match` reconoce cada variante, y da un nombre a sus campos:

```rythme
type Forma = Circulo(radio) | Rect(ancho, alto)

fn area(forma) = match forma
  Circulo(r) -> 3 * r * r
  Rect(a, h) -> a * h
end

print(area(Circulo(2)), area(Rect(3, 4)))
```

```sortie
12 12
```

En el caso `Rect(a, h)`, `a` recibe el ancho y `h` el alto: tú eliges los
nombres.

También puedes escribir una definición por casos, sin `match`:

```rythme
type Forma = Circulo(radio) | Rect(ancho, alto)

fn perimetro(Circulo(r)) = 2 * pi * r
fn perimetro(Rect(a, h)) = 2 * (a + h)

print(round(perimetro(Circulo(1)), 2), perimetro(Rect(3, 4)))
```

```sortie
6.28 14
```

`pi` es la constante 3.14159..., y `round(x, 2)` conserva dos decimales.

## El verificador detecta los casos olvidados

Si olvidas una variante, Rythme te avisa **antes** de ejecutar el programa.
Es una advertencia: el programa se ejecuta de todos modos.

```rythme norun
type Semaforo = Rojo | Naranja | Verde

fn siguiente(luz) = match luz
  Rojo -> Verde
  Verde -> Naranja
end

print(siguiente(Rojo))
```

`rythme check main.ry`:

```text
advertencia[W002]: este `match` no trata todos los casos
 --> main.ry:3:21
  |
3 | fn siguiente(luz) = match luz
  |                     ^^^^^^^^^
  |
  = note: sin tratar: `Naranja`
  = ayuda: añade los casos que faltan, como `Naranja -> ...`, o un último caso `_ -> ...`

✓ main.ry: sin errores, 1 advertencia(s)
```

Basta con añadir la línea `Naranja -> Rojo`. Es una gran ventaja de un
`type`: Rythme conoce todos sus casos, y te recuerda los que olvidaste.

## Los patrones de listas

Un **patrón** describe la forma de un valor. Un patrón puede describir una
lista: `[]` para la lista vacía, `[x]` para un solo elemento,
`[primero, ..resto]` para «al menos un elemento».

```rythme
fn describir(lista) = match lista
  [] -> "vacía"
  [x] -> "solo {x}"
  [a, b] -> "{a} y luego {b}"
  [primero, ..resto] -> "{primero}, y {len(resto)} más"
end

print(describir([]))
print(describir([1]))
print(describir([1, 2]))
print(describir([1, 2, 3, 4]))
```

```sortie
vacía
solo 1
1 y luego 2
1, y 3 más
```

## Los patrones de registros

`{nombre}` reconoce un registro que tiene un campo `nombre`, y lo nombra.
`{edad: 0}` exige un valor preciso. Los demás campos están permitidos:

```rythme
fn saludar(persona) = match persona
  {nombre, edad: 0} -> "¡Bienvenida al mundo, {nombre}!"
  {nombre, ciudad} -> "Hola {nombre}, de {ciudad}."
  {nombre} -> "Hola {nombre}."
end

print(saludar({nombre: "Léa", edad: 0}))
print(saludar({nombre: "Ada", ciudad: "Londres", edad: 36}))
print(saludar({nombre: "Alan", edad: 41}))
```

```sortie
¡Bienvenida al mundo, Léa!
Hola Ada, de Londres.
Hola Alan.
```

Alan no tiene campo `ciudad`: el segundo caso no le corresponde, así que
toma el tercero.

## Las guardas en match

Después de un patrón, `if condición` añade una condición, una **guarda**:

```rythme
fn clasificar(n) = match n
  0 -> "cero"
  x if x < 0 -> "negativo"
  x if x % 2 == 0 -> "par"
  _ -> "impar"
end

print([0, -4, 6, 7] |> map(clasificar))
```

```sortie
["cero", "negativo", "par", "impar"]
```

Varias posibilidades se escriben con `|`:

```rythme
type Semaforo = Rojo | Naranja | Verde

fn indicacion(luz) = match luz
  Rojo | Naranja -> "cuidado"
  Verde -> "adelante"
end

print(indicacion(Naranja), indicacion(Verde))
```

```sortie
cuidado adelante
```

## Los símbolos

Un **símbolo**, como en Ruby, es un nombre usado como valor: `:ok`,
`:en_espera`. Se escribe con `:` delante. Es práctico para un estado, sin
crear un tipo:

```rythme
fn etiqueta(estado) = match estado
  :ok -> "terminado"
  :en_espera -> "en espera"
  _ -> "desconocido"
end

pedido = {numero: 12, estado: :en_espera}
print(pedido.estado, etiqueta(pedido.estado), etiqueta(:perdido))
```

```sortie
:en_espera en espera desconocido
```

> **Nota:** un símbolo nunca es igual a un texto: `:ok == "ok"` vale
> `false`. Para un conjunto fijo de casos, un `type` es más seguro, porque
> el verificador detecta los casos olvidados.

## Errores frecuentes

### Un nombre de tipo en minúsculas

```rythme norun
type semaforo = rojo | verde
```

```text
error[E001]: los nombres de tipo empiezan por mayúscula
 --> main.ry:1:6
  |
1 | type semaforo = rojo | verde
  |      ^^^^^^^^
  |
  = ayuda: escribe `type Semaforo = ...`
  = más sobre este error: `rythme explain E001`
```

El tipo y sus variantes llevan mayúscula: `type Semaforo = Rojo | Verde`.

### Un caso olvidado que llega de verdad

```rythme norun
type Semaforo = Rojo | Naranja | Verde

fn siguiente(luz) = match luz
  Rojo -> Verde
  Verde -> Naranja
end

print(siguiente(Naranja))
```

```text
advertencia[W002]: este `match` no trata todos los casos
 --> main.ry:3:21
  |
3 | fn siguiente(luz) = match luz
  |                     ^^^^^^^^^
  |
  = note: sin tratar: `Naranja`
  = ayuda: añade los casos que faltan, como `Naranja -> ...`, o un último caso `_ -> ...`

error[E006]: ningún caso de este `match` trata Semaforo Naranja
 --> main.ry:3:27
  |
3 | fn siguiente(luz) = match luz
  |                           ^^^ este valor
  |
  = ayuda: añade un caso para este valor, o un último caso `_ -> ...` que trate todo lo demás
  = llamadas en curso (la más reciente primero):
      en `siguiente` llamada en main.ry:8:7
  = más sobre este error: `rythme explain E006`
```

La advertencia llega primero; luego, cuando `Naranja` llega de verdad, el
programa se detiene. No dejes ninguna advertencia W002: añade
`Naranja -> Rojo`.

### Olvidar un campo de una variante

```rythme norun
type Forma = Circulo(radio) | Rect(ancho, alto)
r = Rect(3)
```

```text
error[E004]: `Rect` espera 2 argumentos, pero recibió 1
 --> main.ry:2:5
  |
2 | r = Rect(3)
  |     ^^^^^^^ 1 argumento(s) aquí
  |
1 | type Forma = Circulo(radio) | Rect(ancho, alto)
  |                               ----------------- definida aquí
  |
  = ayuda: uso: `Rect(ancho, alto)`
  = más sobre este error: `rythme explain E004`
```

Una variante con campos es como una función: da un valor por campo,
`Rect(3, 4)`.

### Poner _ primero

```rythme norun
fn describir(n) = match n
  _ -> "otro"
  0 -> "cero"
end
print(describir(0))
```

```text
advertencia[W004]: este caso nunca se puede alcanzar
 --> main.ry:3:3
  |
3 |   0 -> "cero"
  |   ^
  |
2 |   _ -> "otro"
  |   - este caso ya lo trata todo
  |
  = ayuda: los casos se prueban en orden: pon `_` al final
```

El programa muestra `otro`: `_` lo atrapa todo, incluso el 0. Pon siempre
`_` al final.

## Ponte a prueba

**Pregunta 1:** escribe un tipo `Moneda` que valga `Cara` o `Cruz`.

> **Respuesta**
>
> `type Moneda = Cara | Cruz`.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
fn cabeza(lista) = match lista
  [] -> "nada"
  [x, ..] -> "empieza por {x}"
end
print(cabeza([7, 8, 9]), cabeza([]))
```

> **Respuesta**
>
> `empieza por 7 nada`. `[x, ..]` reconoce una lista de al menos un
> elemento, y llama `x` al primero.

**Pregunta 3:** ¿por qué un `type` es más seguro que un símbolo para
describir el estado de un pedido?

> **Respuesta**
>
> Porque Rythme conoce todas las variantes de un `type`: señala los casos
> olvidados en un `match` (W002), y un nombre mal escrito es un error. Un
> símbolo mal escrito, como `:en_espra`, pasa desapercibido.

## En resumen

- `type` crea un tipo con sus variantes: `type Semaforo = Rojo | Naranja | Verde`.
- Una variante puede llevar campos: `Circulo(radio)`, que se leen con un punto.
- `match` trata cada variante, en orden, y nombra sus campos; `_` va al
  final.
- Los patrones también reconocen listas (`[primero, ..resto]`) y registros
  (`{nombre}`); una guarda `if` añade una condición.
- `rythme check` señala los casos olvidados en un `match`.

Siguiente: [los símbolos y cuándo usarlos](symboles.md)
