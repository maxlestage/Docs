# Las condiciones

Esta página explica cómo un programa de Python toma decisiones con `if`, `elif`, `else` y `match`.

> **Objetivos:**
>
> - sabrás comparar dos valores con `==`, `!=`, `<` y `>`;
> - sabrás escribir una condición con `if`, `elif` y `else`;
> - sabrás combinar pruebas con `and`, `or` y `not`;
> - sabrás elegir entre varios casos con `match`.

## La condición if

Una [condición](../debuter/glossaire.md#condicion) permite ejecutar código solo si
algo es verdadero. Es como una señal de tráfico: «si la carretera está mojada, reduce la velocidad».

```python
temperatura = 30
if temperatura > 25:
    print("Hace calor.")
    print("Acuérdate de beber agua.")
print("¡Que tengas un buen día!")
```

```sortie
Hace calor.
Acuérdate de beber agua.
¡Que tengas un buen día!
```

Línea por línea:

- `temperatura = 30` guarda 30 en la variable `temperatura`;
- `if temperatura > 25:` hace una pregunta: «¿30 es mayor que 25?». La respuesta es sí;
- por eso se ejecutan los dos `print` desplazados, uno detrás de otro;
- el último `print` no está desplazado: se ejecuta en todos los casos, aunque hiciera
  frío.

Tres reglas que recordar:

- la línea `if` termina con **dos puntos** `:`;
- las líneas que dependen del `if` llevan **sangría** (desplazadas 4 espacios): forman
  un **bloque**;
- la primera línea sin desplazar (`print("¡Que tengas un buen día!")`) está fuera del bloque:
  se ejecuta siempre.

> **Recuerda:** en Python, la sangría no es decorativa. Es la que dice
> qué líneas forman parte del bloque. Usa siempre 4 espacios.

## else y elif

A menudo no hay un solo caso. Una nota puede ser muy buena, regular o mala, y quieres
un mensaje distinto para cada una, como en un boletín de notas.

`else` («si no») da el camino que seguir cuando la condición es falsa. `elif`
(contracción de *else if*, «si no, si») añade otras pruebas.

```python
nota = 7
if nota >= 9:
    print("Sobresaliente")
elif nota >= 7:
    print("Notable")
elif nota >= 5:
    print("Aprobado")
else:
    print("Suspenso")
```

```sortie
Notable
```

Python prueba las condiciones **en orden** y se detiene en la primera verdadera. Aquí,
`nota >= 9` es falso y `nota >= 7` es verdadero: muestra «Notable» y se salta el resto.

**Te toca:** escribe un programa que guarde `hora = 15` en una variable y luego
muestre «Buenos días» si la hora es antes de las 12, «Buenas tardes» si es antes de
las 20, y «Buenas noches» si no.

> **Solución**
>
> ```python
> hora = 15
> if hora < 12:
>     print("Buenos días")
> elif hora < 20:
>     print("Buenas tardes")
> else:
>     print("Buenas noches")
> ```
>
> ```sortie
> Buenas tardes
> ```

## Comparar valores

Para decidir, un programa primero hace una pregunta: «¿la edad es 18?», «¿la
temperatura pasa de 25?». Los operadores de comparación sirven para escribir esas
preguntas.

| Operador | Significado |
| --- | --- |
| `==` | igual a |
| `!=` | distinto de |
| `<`, `>` | menor, mayor |
| `<=`, `>=` | menor o igual, mayor o igual |

Una comparación da un [booleano](../debuter/glossaire.md#booleano): `True` o
`False`.

```python
edad = 17
print(edad == 18)
print(edad != 18)
print(edad < 18)
print("gato" == "gato")
print(10 <= edad < 20)
```

```sortie
False
True
True
True
True
```

La última línea muestra una comodidad de Python: `10 <= edad < 20` significa «`edad` está
entre 10 (incluido) y 20 (no incluido)», como en matemáticas.

> **Atención:** `=` guarda un valor en una variable. `==` compara dos valores.
> No los confundas.

## Combinar: and, or, not

Una decisión depende a menudo de varias cosas a la vez. Para entrar al cine, necesitas
una entrada **y** tener la edad. Estas tres palabras unen varias pruebas en una sola.

- `a and b` es verdadero si `a` **y** `b` son verdaderos;
- `a or b` es verdadero si **al menos uno** de los dos es verdadero;
- `not a` invierte: verdadero pasa a falso, y falso pasa a verdadero.

```python
edad = 25
tiene_entrada = True
es_vip = False

if edad >= 18 and tiene_entrada:
    print("Entrada permitida")
if tiene_entrada or es_vip:
    print("Acceso a la sala")
if not es_vip:
    print("Sin acceso a la zona VIP")
```

```sortie
Entrada permitida
Acceso a la sala
Sin acceso a la zona VIP
```

Línea por línea:

- las tres primeras líneas preparan los datos: una edad y dos booleanos;
- primer `if`: `edad >= 18` es verdadero **y** `tiene_entrada` es verdadero, así que la
  entrada está permitida;
- segundo `if`: `tiene_entrada` es verdadero, y con `or` eso basta;
- tercer `if`: `es_vip` vale `False`, y `not` lo convierte en `True`.

**Te toca:** escribe un programa que guarde `edad = 70` en una variable y luego
muestre «Tarifa reducida» si la edad es menor que 18 o mayor o igual que 65, y
«Tarifa completa» si no.

> **Solución**
>
> ```python
> edad = 70
> if edad < 18 or edad >= 65:
>     print("Tarifa reducida")
> else:
>     print("Tarifa completa")
> ```
>
> ```sortie
> Tarifa reducida
> ```

## Verdadero o falso: la «veracidad»

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En un `if`, Python acepta cualquier valor, no solo `True` o `False`.
Algunos valores cuentan como **falsos**:

- `False` y `None`;
- el cero: `0` y `0.0`;
- todo lo que está vacío: `""`, `[]`, `{}`.

Todo lo demás cuenta como **verdadero**.

```python
carrito = []
if carrito:
    print("Tienes artículos.")
else:
    print("Tu carrito está vacío.")

nombre = "Alice"
if nombre:
    print(f"¡Hola, {nombre}!")
```

```sortie
Tu carrito está vacío.
¡Hola, Alice!
```

`if carrito:` se lee «si el carrito contiene algo». Es más corto que
`if len(carrito) > 0:`.

## La expresión condicional

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Para elegir entre dos valores en una sola línea, escribe
`valor_si_verdadero if condicion else valor_si_falso`.

```python
edad = 15
estado = "mayor de edad" if edad >= 18 else "menor de edad"
print(estado)

n = 7
print("par" if n % 2 == 0 else "impar")
```

```sortie
menor de edad
impar
```

> **Consejo:** reserva esta forma para elecciones sencillas. En cuanto es más larga, un
> `if` normal en varias líneas se lee mejor.

## Elegir entre varios casos: match

Desde Python 3.10, `match` compara un valor con varios **patrones** (*patterns*).
Es como un cambio de agujas de tren: el valor sale por la primera vía que
encaja.

```python
def describir(orden):
    match orden:
        case "norte" | "sur":
            return "vertical"
        case "este" | "oeste":
            return "horizontal"
        case "salir":
            return "adiós"
        case _:
            return "orden desconocida"

print(describir("norte"))
print(describir("oeste"))
print(describir("saltar"))
```

```sortie
vertical
horizontal
orden desconocida
```

- `|` significa «o»: `"norte" | "sur"` acepta uno u otro;
- `case _:` atrapa todo lo demás: es el caso por defecto.

`match` también sabe descomponer un valor. Aquí reconoce una lista de dos elementos y
guarda cada trozo en una variable:

```python
for accion in [["ir", "norte"], ["coger", "llave"], ["dormir"]]:
    match accion:
        case ["ir", direccion]:
            print(f"Vas hacia el {direccion}.")
        case ["coger", objeto]:
            print(f"Coges: {objeto}.")
        case _:
            print("No entiendo.")
```

```sortie
Vas hacia el norte.
Coges: llave.
No entiendo.
```

Las funciones (`def`) se explican en [las funciones](fonctions.md).

## Errores frecuentes

### Olvidar los dos puntos

```python norun
edad = 20
if edad >= 18
    print("mayor de edad")
```

```text
  File "/home/tu/main.py", line 2
    if edad >= 18
                 ^
SyntaxError: expected ':'
```

Python esperaba `:` (*expected ':'*). Toda línea `if`, `elif`, `else`, `while`, `for`
y `match` termina con dos puntos.

### Escribir = en lugar de ==

```python norun
edad = 20
if edad = 18:
    print("justo")
```

```text
  File "/home/tu/main.py", line 2
    if edad = 18:
       ^^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

`=` guarda un valor; para comparar hace falta `==`. Python lo adivina y te lo
sugiere. Escribe `if edad == 18:`.

### Olvidar la sangría

```python norun
edad = 20
if edad >= 18:
print("mayor de edad")
```

```text
  File "/home/tu/main.py", line 3
    print("mayor de edad")
    ^^^^^
IndentationError: expected an indented block after 'if' statement on line 2
```

Después de `if ... :`, Python espera un bloque desplazado. Añade 4 espacios delante de
`print("mayor de edad")`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```python norun
x = 0
if x:
    print("A")
elif x == 0:
    print("B")
else:
    print("C")
```

> **Respuesta**
>
> `B`. `0` cuenta como falso, así que la primera prueba falla. `x == 0` es verdadero:
> Python muestra `B` e ignora el `else`.

**Pregunta 2:** ¿qué muestra `print(5 > 3 and not 2 > 4)`?

> **Respuesta**
>
> `True`. `5 > 3` es verdadero; `2 > 4` es falso, y `not` lo vuelve verdadero.
> Verdadero `and` verdadero da `True`.

**Pregunta 3:** en un `match`, ¿para qué sirve `case _:`?

> **Respuesta**
>
> Es el caso por defecto: atrapa todos los valores que ningún `case` anterior ha
> reconocido. Ponlo al final.

## En resumen

- `==` compara, `=` guarda; una comparación da `True` o `False`.
- `if`, `elif` y `else` eligen un camino; cada línea termina con `:` y el bloque
  lleva una sangría de 4 espacios.
- `and`, `or` y `not` combinan las pruebas.
- `0`, `None` y las colecciones vacías cuentan como falsos.
- `match` elige entre varios casos; `case _:` atrapa el resto.

Siguiente: [los bucles](boucles.md).
