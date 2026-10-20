# La recursión

Rythme no tiene bucles: para repetir, una función se llama a sí misma, y esta página muestra cómo hacerlo sin perderte.

> **Objetivos:**
>
> - sabrás qué es una función recursiva, y por qué se detiene;
> - sabrás escribir un caso base y un caso recursivo;
> - sabrás recorrer una lista con el patrón `[x, ..resto]`;
> - sabrás usar un acumulador para llegar tan lejos como quieras;
> - sabrás cuándo preferir `map`, `filter` o `fold`.

## ¿Por qué no hay bucles?

En muchos lenguajes, se repite con un
[bucle](../debuter/glossaire.md#boucle) `for` o `while`. Un bucle
modifica un contador en cada vuelta: `i` vale 0, luego 1, luego 2...

En Rythme, un valor nunca cambia. Así que un contador que cambia no
existe. Quedan dos formas de repetir:

- las funciones de listas, `map`, `filter`, `fold`, `each`, vistas en
  [pensar de forma funcional](fonctionnel.md);
- la [recursión](../debuter/glossaire.md#recursion): una función que se
  llama a sí misma.

La recursión es la herramienta más general: todo lo que hace un bucle,
ella también sabe hacerlo.

## Una función que se llama a sí misma

Piensa en las muñecas rusas. Para llegar a la más pequeña, abres una
muñeca; dentro hay una muñeca un poco más pequeña, que abres de la misma
forma; y así sucesivamente, hasta la muñeca que no se abre.

```rythme
fn abrir(0) = print("¡la muñeca más pequeña!")
fn abrir(n)
  print("abro la muñeca {n}")
  abrir(n - 1)
end

abrir(3)
```

```sortie
abro la muñeca 3
abro la muñeca 2
abro la muñeca 1
¡la muñeca más pequeña!
```

`abrir(3)` muestra una línea, y luego llama a `abrir(2)`, que llama a
`abrir(1)`, que llama a `abrir(0)`. Ahí se aplica la primera definición,
y todo se detiene.

## Caso base y caso recursivo

Toda función recursiva tiene dos partes:

- el **caso base**: la situación tan simple que se conoce la respuesta
  sin llamarse (la muñeca que no se abre);
- el **caso recursivo**: se da un pequeño paso, y luego la función se
  llama sobre un problema **más pequeño**.

El caso recursivo siempre debe acercarse al caso base. Si no, la función
no se detiene nunca.

```rythme
fn cuenta_atras(0) = print("¡Despegue!")
fn cuenta_atras(n)
  print(n)
  cuenta_atras(n - 1)
end

cuenta_atras(3)
```

```sortie
3
2
1
¡Despegue!
```

Aquí, `n - 1` se acerca a 0 en cada llamada.

## Una definición por casos

La recursión se lee como una definición de matemáticas. El factorial de
`n` (escrito `n!`) es el producto `1 × 2 × ... × n`:

- `0!` vale 1;
- `n!` vale `n × (n - 1)!`.

En Rythme, se escribe exactamente eso, una línea por caso:

```rythme
fn factorial(0) = 1
fn factorial(n) = n * factorial(n - 1)

print(factorial(5), factorial(20))
```

```sortie
120 2432902008176640000
```

Las definiciones se prueban **en orden** (mira
[las funciones](fonctions.md#definir-por-casos)): por eso el caso base va
primero. La misma idea da la potencia:

```rythme
fn potencia(x, 0) = 1
fn potencia(x, n) = x * potencia(x, n - 1)

print(potencia(2, 10), potencia(3, 4))
```

```sortie
1024 81
```

## Recorrer una lista

Una lista se divide de forma natural en dos trozos: su primer elemento, y
**el resto**, que es a su vez una lista, más corta. El patrón
`[x, ..resto]` hace esta división. El caso base es la lista vacía `[]`.

```rythme
fn longitud([]) = 0
fn longitud([_, ..resto]) = 1 + longitud(resto)

fn suma([]) = 0
fn suma([x, ..resto]) = x + suma(resto)

print(longitud([4, 8, 15]), suma([4, 8, 15]))
```

```sortie
3 27
```

Lee `suma` en voz alta: «la suma de una lista vacía es 0; la suma de una
lista que empieza por `x` es `x` más la suma del resto».

Una función recursiva también puede construir una lista nueva:

```rythme
fn duplicar([]) = []
fn duplicar([x, ..resto]) = [x * 2, ..duplicar(resto)]

fn positivos([]) = []
fn positivos([x, ..resto]) if x > 0 = [x, ..positivos(resto)]
fn positivos([_, ..resto]) = positivos(resto)

print(duplicar([1, 2, 3]), positivos([3, -1, 4, -5]))
```

```sortie
[2, 4, 6] [3, 4]
```

`positivos` usa una guarda, `if x > 0`: se queda con `x` solo si es
positivo. El patrón `[x, ..resto]` se explica en detalle en
[el pattern matching](pattern-matching.md).

## Ver el cálculo paso a paso

`rythme steps` muestra cada paso de un cálculo (mira
[rythme steps](etapes.md)). Con este archivo `f.ry`:

```rythme norun
fn factorial(0) = 1
fn factorial(n) = n * factorial(n - 1)
print(factorial(3))
```

```sh
rythme steps f.ry
```

```text
print(factorial(3))
  → print(3 * factorial(2))
  → print(3 * (2 * factorial(1)))
  → print(3 * (2 * (1 * factorial(0))))
  → print(3 * (2 * (1 * 1)))
  → print(3 * (2 * 1))
  → print(3 * 2)
  → print(6)
6
  → ()
```

Se ven las multiplicaciones **en espera** apilarse hacia la derecha, y
luego calcularse una vez alcanzado el caso base. Cada llamada espera la
respuesta de la siguiente: es la [pila](../debuter/glossaire.md#pile-et-tas)
de llamadas.

## Los acumuladores

Esta pila tiene un límite. Con una lista de un millón de elementos, `suma`
esperaría un millón de respuestas a la vez: es demasiado (mira los errores
frecuentes más abajo).

La solución: llevar el resultado **contigo**, en un parámetro más, el
**acumulador**. Es como contar ovejas guardando el total en la cabeza, en
lugar de apuntar cada oveja en un montón de papeles.

```rythme
fn suma([], total) = total
fn suma([x, ..resto], total) = suma(resto, total + x)

print(suma([4, 8, 15], 0))
print(suma(1..1000000, 0))
```

```sortie
27
500000500000
```

El cálculo paso a paso muestra la diferencia: ya nada espera, el total
avanza en cada llamada.

```text
suma([1, 2, 3], 0)
  → suma([2, 3], 1)
  → suma([3], 3)
  → suma([], 6)
  → 6
```

## Las llamadas finales

¿Por qué el acumulador llega tan lejos? Porque la llamada
`suma(resto, total + x)` es **lo último** que hace la función. Es una
**llamada final** (o llamada de cola): Rythme no tiene nada que recordar,
y puede repetir esta llamada millones de veces sin llenar la pila.

| Escritura | ¿Llamada final? | Por qué |
|-----------|-----------------|---------|
| `x + suma(resto)` | no | queda por hacer `x + ...` después de la llamada |
| `n * factorial(n - 1)` | no | queda la multiplicación |
| `suma(resto, total + x)` | sí | no pasa nada después de la llamada |
| `if n == 0 then 1 else contar(n - 1)` | sí | la llamada es el resultado de una rama |

Para no obligar a quien use la función a dar el 0 inicial, esconde el
acumulador en una pequeña función:

```rythme
fn suma_desde([], total) = total
fn suma_desde([x, ..resto], total) = suma_desde(resto, total + x)
fn suma(lista) = suma_desde(lista, 0)

print(suma([1, 2, 3]))
```

```sortie
6
```

## Cuándo preferir map, filter y fold

Muchas recursiones siguen el mismo modelo. Rythme ya las ha escrito por
ti, sin límite de tamaño:

| Recursión que... | Función ya hecha |
|------------------|------------------|
| transforma cada elemento | `map(lista, f)` |
| se queda con algunos elementos | `filter(lista, f)` |
| combina todo en un solo valor | `fold(lista, inicio, f)` |
| suma | `sum(lista)` |

```rythme
print(map([1, 2, 3], fn(x) = x * 2))
print(filter([3, -1, 4, -5], fn(x) = x > 0))
print(fold(1..1000000, 0, fn(total, x) = total + x), sum(1..1000000))
```

```sortie
[2, 4, 6]
[3, 4]
500000500000 500000500000
```

Prefiere estas funciones cuando sirvan: son más cortas y dicen enseguida
lo que hacen. Deja la recursión para lo demás: un cálculo que se detiene
a la mitad, una estructura en árbol, una definición matemática.

## Errores frecuentes

### Olvidar el caso base

```rythme norun
fn factorial(n) = n * factorial(n - 1)
print(factorial(5))
```

```text
error[E014]: demasiadas llamadas anidadas (962627 llamadas en curso)
 --> main.ry:1:23
  |
1 | fn factorial(n) = n * factorial(n - 1)
  |                       ^^^^^^^^^^^^^^^^ esta llamada va demasiado lejos
  |
  = ayuda: ¿una función recursiva que nunca se detiene? Comprueba que siempre llega a un caso base.
      Si no, haz que la llamada recursiva sea lo último que hace la función
      (llamada final), por ejemplo con un acumulador:
        fn suma([], acc) = acc
        fn suma([x, ..resto], acc) = suma(resto, acc + x)
      o usa `fold`, `map`, `filter`, que no tienen límite.
  = llamadas en curso (la más reciente primero):
      en `factorial` (×40) llamada en main.ry:1:23
      … y 962587 llamadas más
  = más sobre este error: `rythme explain E014`
```

Sin caso base, `factorial` baja a 4, 3, ..., 0, -1, -2... sin fin.
Añade `fn factorial(0) = 1` **antes** de la línea general. (El número
de llamadas en curso varía según la máquina.)

### Poner el caso base al final

```rythme norun
fn factorial(n) = n * factorial(n - 1)
fn factorial(0) = 1
print(factorial(5))
```

```text
advertencia[W004]: esta definición de `factorial` nunca se puede usar
 --> main.ry:2:1
  |
2 | fn factorial(0) = 1
  | ^^^^^^^^^^^^^^^
  |
1 | fn factorial(n) = n * factorial(n - 1)
  | --------------- esta definición ya acepta cualquier argumento
  |
  = ayuda: las definiciones se prueban en orden: pon la más general al final
```

Luego el programa se detiene con el mismo error E014. `n` acepta todos
los números, 0 incluido: el caso `0` nunca se alcanza. Intercambia las
dos líneas.

### Olvidar los dos puntos delante del resto

```rythme norun
fn suma([]) = 0
fn suma([x, resto]) = x + suma(resto)
print(suma([1, 2, 3]))
```

```text
advertencia[W002]: `suma` no trata todos los argumentos posibles
 --> main.ry:1:4
  |
1 | fn suma([]) = 0
  |    ^^^^
  |
  = note: sin tratar: `[_, _, _, ..]`
  = ayuda: añade una definición para ese caso, como `fn suma([_, _, _, ..]) = ...`

error[E006]: ninguna definición de `suma` acepta estos argumentos: suma([1, 2, 3])
 --> main.ry:3:7
  |
3 | print(suma([1, 2, 3]))
  |       ^^^^^^^^^^^^^^^ llamada aquí
  |
1 | fn suma([]) = 0
  | ----------- no encaja
  |
2 | fn suma([x, resto]) = x + suma(resto)
  | ------------------- no encaja
  |
  = ayuda: añade una definición que trate este caso, por ejemplo una que termine con un simple nombre o `_`
  = más sobre este error: `rythme explain E006`
```

`[x, resto]` quiere decir «una lista de exactamente dos elementos». Para
«el primero, y luego todo el resto», escribe `[x, ..resto]`.

## Ponte a prueba

**Pregunta 1:** ¿cuáles son las dos partes de toda función recursiva?

> **Respuesta**
>
> El caso base, que responde sin llamarse, y el caso recursivo, que se
> llama sobre un problema más pequeño.

**Pregunta 2:** ¿esta llamada es final?

```rythme norun
fn contar([]) = 0
fn contar([_, ..resto]) = 1 + contar(resto)
```

> **Respuesta**
>
> No: después de la llamada `contar(resto)`, queda sumar 1. Con un
> acumulador, `fn contar([_, ..resto], n) = contar(resto, n + 1)` es una
> llamada final.

**Pregunta 3:** ¿cuánto vale `f([1, 2, 3])` con estas definiciones?

```rythme norun
fn f([]) = []
fn f([x, ..resto]) = [..f(resto), x]
```

> **Respuesta**
>
> `[3, 2, 1]`: cada elemento se pone al final, así que la lista queda
> invertida (como con `reverse`).

## En resumen

- Rythme repite con la recursión: una función que se llama a sí misma.
- Hace falta un caso base, escrito primero, y un caso recursivo que se
  acerque al caso base.
- `[x, ..resto]` divide una lista en su primer elemento y el resto; `[]`
  es el caso base.
- Un acumulador hace que la llamada sea final: la función puede entonces
  llamarse millones de veces.
- Cuando `map`, `filter`, `fold` o `sum` sirvan, prefiérelas.
- `rythme steps` muestra el cálculo paso a paso.

Siguiente: [tus propios tipos y match](types.md)
