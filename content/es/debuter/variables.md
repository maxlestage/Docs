# Las variables, explicadas de forma sencilla

Una variable es un nombre que se le da a un valor para recordarlo: esta página te muestra cómo crear una, leerla, cambiarla, y por qué importa el orden de las líneas.

> **Objetivos:**
>
> - entender qué es una variable: un nombre y un valor;
> - crear una variable, leerla y luego cambiar su valor;
> - seguir un programa línea por línea con una tabla;
> - intercambiar los valores de dos variables;
> - elegir nombres claros;
> - reconocer el mismo programa en Python, JavaScript, Ruby y Rust.

## Un valor, un nombre

Un programa maneja **valores**: un número como `42`, un texto
como `"Ada"`. Mira [valor](glossaire.md#valor).

Para recordar un valor, le das un **nombre**. El nombre y el
valor, juntos, forman una **variable**. Mira
[variable](glossaire.md#variable).

```python
edad = 30
print(edad)
```

```sortie
30
```

Lee la primera línea de derecha a izquierda: «toma `30` y llámalo
`edad`». La segunda línea muestra el valor que lleva el nombre `edad`.

> **Recuerda:** una variable es un nombre pegado a un valor.

## La caja con una etiqueta

Una imagen ayuda mucho: una **caja** con una **etiqueta**.

```text
   etiqueta: edad
  +-----------+
  |    30     |   <- el valor, dentro de la caja
  +-----------+
```

- La etiqueta es el **nombre**: `edad`.
- Lo que hay dentro es el **valor**: `30`.
- Cuando escribes `edad`, el ordenador abre la caja y toma lo que hay
  dentro.

Una caja puede contener un número (`30`, `1.65`), un texto (`"Ada"`) u
otra cosa. El tipo de valor se llama **tipo** (mira
[tipo](glossaire.md#tipo)).

Un texto se llama **cadena de caracteres**, y se escribe entre
comillas (mira [cadena de caracteres](glossaire.md#cadena-de-caracteres)).
Un número con decimales se escribe con un **punto**: `1.65`.

## Los límites de la imagen de la caja

La imagen de la caja es muy útil para empezar. Pero no es
del todo exacta.

En realidad, el valor vive en algún lugar de la memoria del ordenador. El
nombre es más bien una **etiqueta atada al valor** con un hilo.

¿Por qué importa? Porque dos etiquetas pueden estar atadas al **mismo**
valor. Con números y textos, no verás la diferencia. Con listas,
sí:

```python
compras = ["pan", "leche"]
lista_de_pablo = compras
lista_de_pablo.append("huevos")
print(compras)
```

```sortie
['pan', 'leche', 'huevos']
```

```text
  compras ─────────┐
                   ├──> ["pan", "leche", "huevos"]
  lista_de_pablo ──┘
```

Solo hay **una** lista, con dos etiquetas. Cuando Pablo añade
huevos, la lista de `compras` también cambia, ya que es la misma.
Lo verás en detalle en [los datos](donnees.md).

> **Recuerda:** para los números y los textos, piensa «caja». Para
> las listas, piensa «etiqueta atada».

## Crear una variable

Una variable se crea dándole un primer valor, con el signo `=`:
`ciudad = "Lyon"`.

Atención: aquí, `=` **no** quiere decir «es igual a», como en matemáticas.
Quiere decir «guarda este valor con este nombre». Se llama
**asignación**.

Algunos lenguajes piden una palabra más, como `let` en JavaScript y
en Rust: lo verás más abajo.

## Leer una variable

Leer una variable es usar su nombre. El ordenador sustituye el nombre
por el valor.

```python
precio = 4
cantidad = 3
total = precio * cantidad
print(total)
```

```sortie
12
```

En la línea 3, el ordenador lee `precio` (4) y `cantidad` (3), calcula
`4 * 3` y luego guarda `12` con el nombre `total`.

Leer una variable no la cambia. Después del cálculo, `precio` sigue valiendo 4.

Pruébalo: pon `cantidad = 10`. El programa muestra `40`.

## Cambiar una variable

Puedes guardar un valor nuevo con un nombre que ya existe. El valor
antiguo se **olvida**.

Con `tiempo = "lluvia"` y luego `tiempo = "sol"`, la caja `tiempo` contiene
`"sol"`: `"lluvia"` ha desaparecido.

Muy a menudo, se calcula el valor nuevo a partir del antiguo:

```python
puntos = 10
puntos = puntos + 5
print(puntos)
```

```sortie
15
```

`puntos = puntos + 5` choca en matemáticas. En programación, es normal:
el ordenador calcula **primero** la derecha (`10 + 5`, es decir, 15) y **después**
guarda el resultado en `puntos`.

Existe un atajo: `puntos += 5` quiere decir `puntos = puntos + 5`.

> **Recuerda:** el ordenador calcula primero a la derecha del `=` y después guarda
> el resultado a la izquierda.

## El orden de las líneas importa

El ordenador lee las líneas **una a una, de arriba abajo**. Una variable
vale lo que se guardó en ella **en último lugar**.

Para seguir un programa, haz una **tabla**: una fila por paso, una
columna por variable.

```python
a = 2
b = 3
a = a + b
b = a * 2
print(a, b)
```

```sortie
5 10
```

| Paso | Línea | a | b |
|---|---|---|---|
| 1 | `a = 2` | 2 | – |
| 2 | `b = 3` | 2 | 3 |
| 3 | `a = a + b` | 5 | 3 |
| 4 | `b = a * 2` | 5 | 10 |

En el paso 4, `a` ya vale 5: así que `b` recibe `10`, no `4`.

Invierte las líneas 3 y 4: el programa muestra `6 4`. Esta vez, `b`
se calcula cuando `a` todavía vale 2: `b` recibe 4, y luego `a` pasa a ser
`2 + 4`, es decir, 6. Mismas líneas, otro orden: otro resultado.

Pruébalo: haz la tabla en papel **antes** de ejecutar el programa.
Luego compárala con lo que muestra.

> **Recuerda:** si tienes dudas, haz una tabla, paso a paso.

## Intercambiar dos valores

Tienes dos vasos: uno de zumo y uno de agua. Quieres intercambiar su
contenido. Te hace falta un **tercer vaso**, vacío.

La trampa clásica, sin tercer vaso:

```python
x = 1
y = 2
x = y
y = x
print(x, y)
```

```sortie
2 2
```

En la línea `x = y`, el `1` de `x` se sobrescribe. Se pierde, y ya es demasiado tarde.

Con un tercer vaso, `temp` (de «temporal»):

```python
x = 1
y = 2
temp = x
x = y
y = temp
print(x, y)
```

```sortie
2 1
```

| Paso | Línea | x | y | temp |
|---|---|---|---|---|
| 1 | `x = 1` | 1 | – | – |
| 2 | `y = 2` | 1 | 2 | – |
| 3 | `temp = x` | 1 | 2 | 1 |
| 4 | `x = y` | 2 | 2 | 1 |
| 5 | `y = temp` | 2 | 1 | 1 |

Python (y Ruby) tienen un atajo que lo hace todo de una vez:

```python
x = 1
y = 2
x, y = y, x
print(x, y)
```

```sortie
2 1
```

## Nombrar con claridad

El nombre de una variable sirve a los **humanos**, no al ordenador. Para él,
`x` o `precio_del_pan` es lo mismo. Para ti, dentro de un mes, no.

Compara `t = p * n` con esto:

```python
precio_del_pan = 2
numero_de_panes = 3
total_a_pagar = precio_del_pan * numero_de_panes
print(total_a_pagar)
```

```sortie
6
```

El resultado es el mismo, pero este programa se lee como una frase.

Algunas reglas, válidas casi en todas partes:

- nada de espacios en un nombre: `precio del pan` está prohibido;
- las palabras se unen con `_` en Python, Ruby y Rust (`precio_del_pan`), y
  con una mayúscula en JavaScript (`precioDelPan`);
- un nombre no empieza por una cifra: `2precio` está prohibido;
- las mayúsculas cuentan: `Edad` y `edad` son dos nombres distintos;
- evita las tildes y la ñ: `cancion` mejor que `canción`.

> **Consejo:** si dudas con un nombre, pregúntate «¿qué hay
> en la caja?» y escribe la respuesta.

## El mismo programa en cuatro lenguajes

Aquí tienes un pequeño programa: una cesta, un precio, una cantidad que cambia.

En Python:

```python
precio = 4
cantidad = 3
print("Total:", precio * cantidad)
cantidad = cantidad + 2
print("Total:", precio * cantidad)
```

```sortie
Total: 12
Total: 20
```

En JavaScript, `let` crea una variable que podrá cambiar; `const`, una
variable que no cambiará nunca:

```javascript
const precio = 4;
let cantidad = 3;
console.log("Total:", precio * cantidad);
cantidad = cantidad + 2;
console.log("Total:", precio * cantidad);
```

```sortie
Total: 12
Total: 20
```

En Ruby, como en Python, sin palabra de más:

```ruby
precio = 4
cantidad = 3
puts "Total: #{precio * cantidad}"
cantidad = cantidad + 2
puts "Total: #{precio * cantidad}"
```

```sortie
Total: 12
Total: 20
```

En Rust, `let` crea la variable. Por defecto, **no** puede cambiar.
Hay que escribir `let mut` (de «mutable», que puede cambiar; mira
[mutable](glossaire.md#mutable)). Sin `mut`, Rust se niega a compilar y
él mismo propone añadir `mut`:

```rust
let precio = 4;
let mut cantidad = 3;
println!("Total: {}", precio * cantidad);
cantidad = cantidad + 2;
println!("Total: {}", precio * cantidad);
```

```sortie
Total: 12
Total: 20
```

| | Python | JavaScript | Ruby | Rust |
|---|---|---|---|---|
| Crear | `x = 1` | `let x = 1;` | `x = 1` | `let mut x = 1;` |
| No cambiará | – | `const x = 1;` | – | `let x = 1;` |
| Cambiar | `x = 2` | `x = 2;` | `x = 2` | `x = 2;` |
| Mostrar | `print(x)` | `console.log(x);` | `puts x` | `println!("{x}");` |

> **Recuerda:** las palabras cambian de un lenguaje a otro, la idea sigue siendo la
> misma: un nombre, un valor, y `=` para guardar.

## Errores frecuentes

### Usar una variable antes de crearla

El ordenador lee de arriba abajo. En la línea 1, `edad` todavía no existe.

```python norun
print(edad)
edad = 30
```

```text
Traceback (most recent call last):
  File "edad.py", line 1, in <module>
    print(edad)
          ^^^^
NameError: name 'edad' is not defined
```

`NameError` quiere decir «no conozco este nombre». En JavaScript:

```javascript norun
console.log(edad);
let edad = 30;
```

```text
console.log(edad);
            ^

ReferenceError: Cannot access 'edad' before initialization
```

La solución: crea la variable **antes** de usarla. Intercambia las
dos líneas: primero `edad = 30`, después `print(edad)`.

### Una errata en el nombre

Para el ordenador, `nomber` y `nombre` son dos nombres distintos.

```python norun
nombre = "Ada"
print(nomber)
```

```text
Traceback (most recent call last):
  File "nombre.py", line 2, in <module>
    print(nomber)
          ^^^^^^
NameError: name 'nomber' is not defined. Did you mean: 'nombre'?
```

Python te ayuda: «Did you mean: 'nombre'?» quiere decir «¿querías
decir `nombre`?». Ruby hace lo mismo.

La solución: escribe el nombre exactamente igual en todas partes, mayúsculas
incluidas. Para leer estos mensajes, mira [leer un error](lire-une-erreur.md).

## Ponte a prueba

**1.** ¿Cuánto vale `n` al final?

```python
n = 5
n = n * 2
n = n - 1
print(n)
```

> **Respuesta**
>
> 9. Paso 1: `n` vale 5. Paso 2: `5 * 2`, o sea, 10. Paso 3: `10 - 1`,
> o sea, 9.

**2.** ¿Por qué `x = y` y luego `y = x` no intercambian dos valores?

> **Respuesta**
>
> Porque después de `x = y`, el valor antiguo de `x` se ha perdido: las dos
> variables contienen lo mismo. Primero hay que guardarlo en una
> tercera variable, `temp`.

**3.** ¿Cuál de estos nombres es el mejor para un precio en euros:
`p`, `precio euros`, `precio_en_euros`?

> **Respuesta**
>
> `precio_en_euros`. `p` no dice nada, y `precio euros` contiene un espacio,
> lo cual está prohibido.

## En resumen

- Una variable es un nombre atado a un valor: una caja con una etiqueta.
- `=` guarda el valor de la derecha con el nombre de la izquierda; no es «igual».
- El ordenador lee las líneas de arriba abajo: una variable vale el último valor guardado.
- Para seguir un programa, haz una tabla paso a paso.
- Para intercambiar dos valores, guarda uno de ellos en una variable temporal.
- Elige nombres que digan lo que contiene la caja.

Siguiente: [verdadero, falso y las condiciones](logique.md)
