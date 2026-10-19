# Leer un mensaje de error sin entrar en pánico

Un mensaje de error no es un castigo, es una ayuda: esta página te enseña a encontrar en él tres datos, en seis lenguajes, y a saber qué hacer después.

> **Objetivos:**
>
> - ver un mensaje de error como una ayuda;
> - encontrar en un mensaje: el lugar, el tipo de error y la frase que lo explica;
> - leer una traza de abajo arriba;
> - distinguir un error de sintaxis, un error de ejecución y un resultado incorrecto;
> - saber qué hacer después de leer el mensaje.

## Un error es una ayuda

Aparece un mensaje en rojo. Tu corazón late más rápido. Es normal.

Pero mira lo que pasa de verdad: el ordenador se ha detenido, y te
**explica por qué**. Es como un GPS que dice «cambie de sentido cuando sea
posible»: no te regaña, te ayuda a volver al camino.

Los programadores con experiencia ven errores **todo el día**. La
diferencia es que los leen con calma. Un
[error](glossaire.md#error) no dice nada de ti. Dice algo de
**una línea** de tu código.

> **Recuerda:** un mensaje de error es una pista gratis. Léelo
> antes de tocar el código.

## Las tres cosas que hay que encontrar

Todos los mensajes, en todos los lenguajes, contienen los mismos tres
datos. Búscalos como buscas el nombre, la fecha y el lugar en
una invitación:

```text
  1. ¿DÓNDE?         el archivo y la línea    →  hola.py, line 2
  2. ¿QUÉ TIPO?      el nombre del error      →  NameError
  3. ¿POR QUÉ?       la frase que lo explica  →  name 'nombe' is not defined
```

El resto (los `~~~`, los `^^^`, las rutas) ayuda a precisar. Empieza
por estas tres.

Aquí tienes las mismas tres cosas en seis lenguajes. Los archivos están en la
carpeta `/home/ada`.

### En Python

```python norun
nombre = "Ada"
print("Hola " + nombe)
```

```text
Traceback (most recent call last):
  File "/home/ada/hola.py", line 2, in <module>
    print("Hola " + nombe)
                    ^^^^^
NameError: name 'nombe' is not defined. Did you mean: 'nombre'?
```

| Pregunta | Respuesta en el mensaje |
|---|---|
| ¿Dónde? | `hola.py`, `line 2`; los `^^^^^` subrayan `nombe` |
| ¿Qué tipo? | `NameError`: un nombre desconocido |
| ¿Por qué? | `name 'nombe' is not defined`: ese nombre no existe |

Python incluso propone la corrección: `Did you mean: 'nombre'?` («¿querías
decir `nombre`?»). Una errata, nada más.

### En JavaScript

```javascript norun
const nombre = "Ada";
console.log("Hola " + nombe);
```

```text
file:///home/ada/nombre.mjs:2
console.log("Hola " + nombe);
                      ^

ReferenceError: nombe is not defined
    at file:///home/ada/nombre.mjs:2:23
```

- **¿Dónde?** `nombre.mjs:2`, la línea 2. El `^` muestra el sitio exacto.
- **¿Qué tipo?** `ReferenceError`: una referencia (un nombre) que no se encuentra.
- **¿Por qué?** `nombe is not defined`: `nombe` no está definido.

Node suele añadir otras líneas `at …` que hablan del propio Node
(`node:internal/…`). Puedes ignorarlas.

### En Ruby

```ruby norun
stock = { "manzana" => 3, "pera" => 5 }
cantidad = stock["plátano"]
puts cantidad + 1
```

```text
cesta.rb:3:in `<main>': undefined method `+' for nil (NoMethodError)

puts cantidad + 1
              ^
```

- **¿Dónde?** `cesta.rb:3`, la línea 3.
- **¿Qué tipo?** `NoMethodError`: una operación que no existe.
- **¿Por qué?** `undefined method '+' for nil`: se hace `+` sobre `nil`,
  es decir, sobre «nada».

La línea 3 está bien. El verdadero problema está en la línea 2: no hay
plátanos en el stock, así que `cantidad` vale `nil`.

### En Rust

```rust norun
fn main() {
    let precio = 12;
    println!("{}", precio + cantidad);
}
```

```text
error[E0425]: cannot find value `cantidad` in this scope
 --> total.rs:3:29
  |
3 |     println!("{}", precio + cantidad);
  |                             ^^^^^^^^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

- **¿Dónde?** `total.rs:3:29`: línea 3, columna 29 (el carácter número 29).
- **¿Qué tipo?** `E0425`, un código que puedes buscar con `rustc --explain E0425`.
- **¿Por qué?** `cannot find value 'cantidad'`: el valor `cantidad`
  no se encuentra. Primero hay que escribir `let cantidad = 3;`.

### En Go

```go norun
package main

import "fmt"

func main() {
	nombre := "Ada"
	edad := 36
	fmt.Println(nombre)
}
```

```text
# command-line-arguments
./main.go:7:2: declared and not used: edad
```

- **¿Dónde?** `main.go:7:2`: línea 7, columna 2.
- **¿Qué tipo?** Aquí no hay nombre: es un error de compilación.
- **¿Por qué?** `declared and not used: edad`: la variable `edad` se crea
  pero nunca se usa. Go se niega. Bórrala, o muéstrala.

### En Rythme

```rythme norun
fn doble(x) = x * 2

print(doble("diez"))
```

```text
error[E005]: no se puede usar `*` con Text y Int
 --> doble.ry:3:7
  |
3 | print(doble("diez"))
  |       ^^^^^^^^^^^^^ `doble` se detiene con este error, cada vez
  |
1 | fn doble(x) = x * 2
  |                 - en `doble`: Text "diez" * Int 2
  |
  = note: por cómo `doble` usa `x`, necesita: `fn doble(x: Int|Float) -> Int|Float`
  = ayuda: para repetir un texto o una lista, usa `repeat(x, n)`
  = más sobre este error: `rythme explain E005`
```

- **¿Dónde?** `doble.ry:3:7`, y luego la línea 1, dentro de la función.
- **¿Qué tipo?** `E005`, explicado por `rythme explain E005`.
- **¿Por qué?** Se multiplica un texto (`"diez"`) por un número. Hay que
  escribir `doble(10)`.

> **Recuerda:** dónde, qué tipo, por qué. Encuentra estas tres cosas
> antes de buscar más lejos.

## Leer una traza de abajo arriba

Cuando una función llama a otra función, el mensaje muestra **todo el
camino**. Se llama una **traza** (*traceback* en Python).

```python norun
def promedio(notas):
    return sum(notas) / len(notas)

def boletin(nombre, notas):
    print(nombre, ":", promedio(notas))

boletin("Ada", [12, 15])
boletin("Léo", [])
```

```text
Ada : 13.5
Traceback (most recent call last):
  File "/home/ada/promedio.py", line 8, in <module>
    boletin("Léo", [])
    ~~~~~~~^^^^^^^^^^^
  File "/home/ada/promedio.py", line 5, in boletin
    print(nombre, ":", promedio(notas))
                       ~~~~~~~~^^^^^^^
  File "/home/ada/promedio.py", line 2, in promedio
    return sum(notas) / len(notas)
           ~~~~~~~~~~~^~~~~~~~~~~~
ZeroDivisionError: division by zero
```

`most recent call last` quiere decir «la llamada más reciente al final». Así
que la línea más importante está **abajo del todo**. Después, sube:

| Orden de lectura | Línea | Lo que dice |
|---|---|---|
| 1 | `ZeroDivisionError: division by zero` | se ha dividido entre cero |
| 2 | `line 2, in promedio` | en `promedio`, `len(notas)` valía 0 |
| 3 | `line 5, in boletin` | `promedio` fue llamada por `boletin` |
| 4 | `line 8, in <module>` | todo empezó con `boletin("Léo", [])` |

Léo no tiene ninguna nota: la lista está vacía, su longitud es 0. Como en una pila
de platos, el último que se ha puesto (abajo del mensaje) es el primero que hay que
mirar.

Rust, Go y JavaScript hacen lo contrario: el error está **arriba**. En caso de
duda, busca la línea que contiene `Error` o `error`.

## Tres momentos en los que algo falla

### Antes de arrancar: el error de sintaxis

La [sintaxis](glossaire.md#sintaxis) es la gramática del lenguaje. Si falta
un signo, nada arranca:

```python norun
edad = 17
if edad >= 18
    print("mayor de edad")
```

```text
  File "/home/ada/edad.py", line 2
    if edad >= 18
                 ^
SyntaxError: expected ':'
```

`expected ':'`: Python esperaba dos puntos al final del `if`.

### Durante: el error de ejecución

El programa arranca, y luego se topa con una operación imposible. Es el
ejemplo del promedio de Léo: `Ada : 13.5` se muestra, **y luego** llega
el error.

### Sin mensaje: el resultado incorrecto

El peor caso: todo funciona, pero el resultado es falso. Queremos el total de
tres precios:

```python
total = 0
for precio in [3, 4, 5]:
    total = precio
print(total)
```

```sortie
5
```

Esperábamos 12. No hay ningún mensaje: hay que mostrar los valores para
entender. Aquí, hacía falta `total = total + precio`.

> **Recuerda:** sintaxis = no arranca; ejecución = se detiene a mitad de camino;
> resultado incorrecto = ningún mensaje, te toca buscar a ti.

## Qué hacer después de leer el mensaje

1. **Vuelve a leer la línea indicada, y la de antes.** El ordenador señala
   el sitio donde se perdió, no siempre el sitio del fallo.
2. **Busca el mensaje en internet.** Copia la parte general, sin tus
   propios nombres: `ZeroDivisionError: division by zero`, no `promedio.py`.
3. **Muestra los valores.** Un `print(notas)` justo antes de la línea que falla
   muestra `[]`, y todo se aclara.
4. **Corrige el primer error**, vuelve a ejecutar, y pasa al siguiente.

Las técnicas para buscar un [bug](glossaire.md#bug) están en [Encontrar
y corregir tus errores](deboguer.md).

## Errores frecuentes

### Buscar solo en la línea indicada

```javascript norun
const notas = [12, 15
console.log(notas);
```

```text
file:///home/ada/notas.mjs:2
console.log(notas);
^^^^^^^

SyntaxError: Unexpected identifier 'console'
```

El mensaje muestra la línea 2, que está bien. Es la línea 1 la que olvida
el `]`. JavaScript solo se da cuenta al ver `console`, que no
esperaba (*unexpected*). Corrección: `const notas = [12, 15];`.

### Leer solo la parte de arriba del mensaje de Python

```python norun
def buscar(nombre):
    print("buscando a", nombre)

resultado = buscar("Ada")
print(resultado.upper())
```

```text
buscando a Ada
Traceback (most recent call last):
  File "/home/ada/buscar.py", line 5, in <module>
    print(resultado.upper())
          ^^^^^^^^^^^^^^^
AttributeError: 'NoneType' object has no attribute 'upper'
```

`Traceback` arriba no dice nada. La explicación está **abajo**: `resultado`
vale `None` («nada»), porque `buscar` no devuelve ningún valor. Corrección:
añade `return nombre` al final de la función.

## Ponte a prueba

**1.** ¿Cuáles son las tres cosas que hay que buscar en un mensaje de error?

> **Respuesta**
>
> Dónde (el archivo y la línea), qué tipo (el nombre del error, como
> `TypeError`), y por qué (la frase que lo explica).

**2.** En una traza de Python con varias líneas `File …`, ¿por dónde empiezas?

> **Respuesta**
>
> Por abajo: la última línea da el error, la línea `File` justo
> encima da el sitio donde se produjo. Después se sube para
> ver quién llamó a qué.

**3.** El mensaje indica la línea 12, pero te parece perfecta. ¿Qué
haces?

> **Respuesta**
>
> Miras la línea 11, y las de antes: un paréntesis, un
> corchete o unos dos puntos olvidados más arriba suelen ser la verdadera causa.

## En resumen

- Un mensaje de error es una ayuda: dice dónde, qué tipo y por qué.
- En Python, lee la traza de abajo arriba; en Rust, Go y JavaScript, el error está arriba.
- Error de sintaxis: no arranca; de ejecución: se detiene a mitad de camino; resultado incorrecto: ningún mensaje.
- Vuelve a leer la línea indicada y la de antes.
- Busca la parte general del mensaje en internet, y muestra los valores.

Siguiente: [Cuando te bloqueas: un método paso a paso](bloque.md).
