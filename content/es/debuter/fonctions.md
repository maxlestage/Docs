# Las funciones, sin jerga

Una función es una pequeña máquina a la que le das un nombre: le das ingredientes, hace su trabajo y devuelve un resultado.

> **Objetivos:**
>
> - entender qué es una función, con la imagen de una máquina;
> - definir una función y luego llamarla;
> - darle parámetros a una función;
> - distinguir entre `return` y `print`;
> - usar una función dentro de otra;
> - saber que una variable creada en una función se queda dentro.

## Una función es una máquina

Piensa en una tostadora. Pones pan (la **entrada**). Hace su trabajo.
Devuelve pan tostado (la **salida**). No necesitas saber cómo funciona
por dentro.

Una [función](glossaire.md#funcion) funciona igual:

```text
            ┌────────────────┐
   3   ───► │     doble      │ ───►  6
 entrada    │  (por dos)     │      salida
            └────────────────┘
```

También puedes verla como una **receta**: tiene un nombre («crepes»),
una lista de ingredientes y unos pasos. La escribes una vez. Después,
solo dices «haz crepes».

> **Recuerda:** una función tiene un nombre, recibe entradas y devuelve
> una salida.

## Definir y luego llamar

Hay dos momentos bien separados:

1. **definir** la función: escribir la receta en el cuaderno;
2. **llamar** a la función: cocinar con la receta.

En Python, se define con `def`:

```python
def decir_hola():
    print("¡Hola!")
    print("¿Qué tal?")

decir_hola()
decir_hola()
```

```sortie
¡Hola!
¿Qué tal?
¡Hola!
¿Qué tal?
```

- `def decir_hola():` le da un nombre a la receta;
- las líneas **con sangría** son los pasos de la receta;
- `decir_hola()`, con los paréntesis, **pone en marcha** la receta.

### En qué orden lee Python el programa

Definir no pone en marcha **nada**. Python guarda la receta y sigue.

| Paso | Línea leída | Lo que pasa |
|------|-------------|-------------|
| 1 | `def decir_hola():` | Python recuerda la receta, no muestra nada |
| 2 | `decir_hola()` | salta a la función, muestra las 2 líneas |
| 3 | vuelta | regresa justo después de la llamada |
| 4 | `decir_hola()` | salta otra vez a la función, muestra las 2 líneas |

Prueba: borra las dos llamadas. El programa ya no muestra nada. La
receta existe, pero nadie cocina.

> **Recuerda:** `def` escribe la receta. Los paréntesis `()` la ponen en
> marcha.

## Parámetros y argumentos

Una receta puede cambiar según lo que le des. Para saludar a alguien,
necesitas su nombre.

```python
def saludar(nombre):
    print("Hola", nombre)

saludar("Léa")
saludar("Tom")
```

```sortie
Hola Léa
Hola Tom
```

Dos palabras, dos lugares:

- `nombre` es el [parámetro](glossaire.md#parametro): una caja vacía,
  en la definición;
- `"Léa"` es el [argumento](glossaire.md#argumento): el valor que pones
  en la caja, en el momento de la llamada.

| Llamada | Valor de `nombre` en la función | Muestra |
|---------|---------------------------------|---------|
| `saludar("Léa")` | `"Léa"` | Hola Léa |
| `saludar("Tom")` | `"Tom"` | Hola Tom |

Una función puede tener varios parámetros, separados por comas. El
orden importa: el primer argumento va al primer parámetro.

```python
def presentar(nombre, edad):
    print(nombre, "tiene", edad, "años")

presentar("Léa", 12)
```

```sortie
Léa tiene 12 años
```

Prueba: escribe `presentar(12, "Léa")`. Python muestra `12 tiene Léa años`:
las cajas se llenaron en el orden equivocado.

> **Recuerda:** el parámetro es la caja (en `def`), el argumento es lo
> que pones dentro (en la llamada).

## Devolver un resultado: return

`return` quiere decir «aquí está el resultado, te lo devuelvo». La
función se detiene y le da el valor a quien la llamó.

```python
def doble(x):
    return x * 2

resultado = doble(3)
print(resultado)
print(doble(10) + 1)
```

```sortie
6
21
```

La llamada `doble(3)` se **sustituye** por su respuesta, como si
hubieras escrito `6`.

```text
resultado = doble(3)
            └──┬───┘
               6          → resultado = 6
```

El resultado puede usarse en otro sitio: en un cálculo, en una
variable, en otra llamada.

### return o print: la gran confusión

Es la trampa más común. Los dos parecen «dar» un valor. No son nada
parecidos.

- `print` **muestra** un valor en la pantalla, para ti. El programa no
  puede volver a usarlo.
- `return` **devuelve** un valor al programa. No se muestra nada.

Compara estas dos funciones:

```python
def doble_muestra(x):
    print(x * 2)

def doble_devuelve(x):
    return x * 2

a = doble_muestra(3)
b = doble_devuelve(3)
print("a vale", a)
print("b vale", b)
```

```sortie
6
a vale None
b vale 6
```

Línea por línea:

| Línea | Lo que pasa | Se muestra |
|-------|-------------|------------|
| `a = doble_muestra(3)` | la función **muestra** 6, pero no devuelve nada | `6` |
| `b = doble_devuelve(3)` | la función **devuelve** 6, sin mostrar nada | nada |
| `print("a vale", a)` | `a` está vacía: Python dice `None` | `a vale None` |
| `print("b vale", b)` | `b` contiene 6 | `b vale 6` |

`None` quiere decir «nada». Una función sin `return` devuelve `None`.

Piensa en la tostadora. `print` es una foto de la tostada enviada por
mensaje: ves la tostada, pero no puedes comértela. `return` es la
tostada puesta en tu plato.

Prueba: escribe `print(doble_muestra(3) + 1)`. Python muestra 6 y luego
un error: no se puede sumar 1 a «nada».

> **Recuerda:** `print` muestra, `return` devuelve. Si quieres volver a
> usar el resultado, necesitas `return`.

### En otros lenguajes

La idea es la misma: un nombre, parámetros entre paréntesis, un
`return`.

```javascript
function doble(x) {
  return x * 2;
}
console.log(doble(3));
```

```sortie
6
```

```ruby
def doble(x)
  return x * 2
end
puts doble(3)
```

```sortie
6
```

En Ruby, la última línea se devuelve sola: puedes escribir solo
`x * 2`.

```rust
fn doble(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", doble(3));
}
```

```sortie
6
```

Rust pide el [tipo](glossaire.md#tipo) de cada entrada (`x: i32`, un
número entero) y de la salida (`-> i32`). La última línea, sin punto y
coma, es el resultado devuelto.

## Una función que llama a otra

Una función puede usar otra función. Es como una receta de tarta que
dice «prepara la masa (ver página 12)».

```python
def cuadrado(n):
    return n * n

def suma_de_cuadrados(a, b):
    return cuadrado(a) + cuadrado(b)

print(suma_de_cuadrados(3, 4))
```

```sortie
25
```

Sigamos el cálculo:

| Paso | Lo que se calcula | Resultado |
|------|-------------------|-----------|
| 1 | empieza `suma_de_cuadrados(3, 4)`, `a` = 3, `b` = 4 | – |
| 2 | `cuadrado(3)` | 9 |
| 3 | `cuadrado(4)` | 16 |
| 4 | `9 + 16`, devuelto por `suma_de_cuadrados` | 25 |

Prueba: cambia `return n * n` por `return n * n * n`. El resultado pasa
a ser 27 + 64 = 91.

## Por qué ayudan las funciones

Sin funciones, copias el mismo cálculo por todas partes. Si el cálculo
cambia, hay que corregirlo en cada sitio, y siempre se olvida alguno.

```python
def precio_con_iva(precio):
    return precio * 1.2

print(precio_con_iva(10))
print(precio_con_iva(50))
print(precio_con_iva(100))
```

```sortie
12.0
60.0
120.0
```

Las funciones ayudan de dos maneras:

- **sin repetición**: la fórmula se escribe una sola vez. Si el
  impuesto cambia, cambias una sola línea;
- **un nombre claro**: `precio_con_iva(10)` se lee mejor que `10 * 1.2`.
  Entiendes lo que hace el código sin leer el cálculo.

> **Recuerda:** en cuanto copies el mismo código, conviértelo en una
> función, con un nombre que diga lo que hace.

## Las variables locales

Una variable creada **dentro** de una función solo existe **dentro** de
esa función. Se dice que es **local**. Su zona de vida se llama el
[ámbito](glossaire.md#ambito).

Piensa en un borrador en un aula de examen: escribes en él y luego se
queda en el aula. Solo sale el examen que entregas (el `return`).

```python
def calcular():
    resultado = 6 * 7
    return resultado

respuesta = calcular()
print(respuesta)
```

```sortie
42
```

Fuera de la función, `resultado` no existe:

```python norun
def calcular():
    resultado = 6 * 7

calcular()
print(resultado)
```

```text
Traceback (most recent call last):
  File "local.py", line 5, in <module>
    print(resultado)
          ^^^^^^^^^
NameError: name 'resultado' is not defined
```

Es algo bueno: dos funciones pueden tener cada una su variable `total`
sin molestarse. Para sacar un valor, usa `return`.

> **Recuerda:** lo que se crea en una función se queda en la función.
> Solo `return` saca un valor.

## Errores frecuentes

### Olvidar los paréntesis en la llamada

Sin `()`, la función no se pone en marcha. Hablas de ella sin hacerla
funcionar.

```python norun
def decir_hola():
    print("¡Hola!")

print(decir_hola)
```

```text
<function decir_hola at 0x7f8377a271a0>
```

Python describe la función en lugar de ponerla en marcha. (El número
`0x7f…` cambia cada vez: es una dirección en la memoria). Corrección:
`decir_hola()`.

### Olvidar return

La función calcula, pero no devuelve nada.

```python
def doble(x):
    x * 2

print(doble(3))
```

```sortie
None
```

No hay mensaje de error, pero sale `None` en lugar de 6. El cálculo se
hace y luego se tira. Corrección: `return x * 2`.

### El número equivocado de argumentos

La función espera un nombre, y no le das nada.

```python norun
def saludar(nombre):
    print("Hola", nombre)

saludar()
```

```text
Traceback (most recent call last):
  File "saludar.py", line 4, in <module>
    saludar()
    ~~~~~~~^^
TypeError: saludar() missing 1 required positional argument: 'nombre'
```

Lee la última línea: «falta 1 argumento: `nombre`». Con un argumento de
más, `saludar("Léa", "Tom")`, el mensaje pasa a ser:

```text
TypeError: saludar() takes 1 positional argument but 2 were given
```

Corrección: da tantos argumentos como parámetros haya,
`saludar("Léa")`. Mira también [leer un mensaje de error](lire-une-erreur.md).

## Ponte a prueba

**1.** ¿Qué hace este programa: `def f(): print("Hola")` y nada más?

> **Respuesta**
>
> No se muestra nada. La función está definida, pero nunca se llama.
> Hay que añadir `f()`.

**2.** En `def area(ancho, alto):` y `area(3, 5)`, ¿cuáles son los
parámetros y cuáles los argumentos?

> **Respuesta**
>
> Los parámetros son `ancho` y `alto` (las cajas, en `def`). Los
> argumentos son `3` y `5` (los valores, en la llamada). `ancho` recibe
> 3, `alto` recibe 5.

**3.** ¿Por qué `x = mi_funcion()` da `None` si la función sí muestra un
número?

> **Respuesta**
>
> Porque la función usa `print` en lugar de `return`. **Muestra** el
> número, pero no lo **devuelve**. Una función sin `return` devuelve
> `None`.

## En resumen

- Una función es una máquina: entradas, un trabajo, una salida.
- `def` escribe la receta; `nombre()` la pone en marcha.
- El parámetro es la caja en `def`; el argumento es el valor que se da en la llamada.
- `print` muestra un valor; `return` se lo devuelve al programa.
- Una función puede llamar a otra; así no copias código.
- Una variable creada en una función se queda en la función.

Siguiente: [los datos](donnees.md).
