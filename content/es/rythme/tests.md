# Probar tu código

Una prueba es un pequeño trozo de código que comprueba que una función da el resultado correcto: en Rythme, se escribe junto a la función, y `rythme test` las ejecuta todas de una vez.

> **Objetivos:**
>
> - escribir una prueba con `test "nombre" = ...` y `assert_eq`;
> - escribir una prueba de varias líneas, con `assert`;
> - ejecutar las pruebas con `rythme test`, y leer el mensaje de una prueba que falla;
> - probar una función que devuelve un `Result`.

## Por qué probar

Escribes una función, la pruebas con un `print`, funciona.
Una semana después, la modificas... y rompes sin darte cuenta un caso que
funcionaba.

Una **prueba** guarda constancia de lo que debe funcionar: «el doble de 21
es 42». En lugar de volver a probarlo todo a mano, ejecutas un comando, y
Rythme comprueba cada prueba por ti, en una fracción de segundo.

## Una primera prueba

Una prueba empieza con la palabra `test`, luego su **nombre** entre
comillas, luego `=` y la comprobación:

```rythme
fn doble(x) = x * 2

test "el doble de 21 es 42" = assert_eq(doble(21), 42)

print(doble(5))
```

```sortie
10
```

`assert_eq(obtenido, esperado)` comprueba que dos valores son iguales. Aquí,
la prueba dice: «`doble(21)` debe dar `42`».

Ejecutado con `rythme run`, como siempre, el programa **ignora las
pruebas**: solo se ejecuta el `print`. Así que las pruebas pueden vivir en
el mismo archivo que el código, sin cambiar nada de lo que hace.

> **Recuerda:** el nombre de una prueba es una frase que dice lo que se
> comprueba. Cuando una prueba falla, es ese nombre el que se muestra: debe
> ser claro.

## Varias comprobaciones en un bloque

Una prueba se puede escribir en varias líneas, sin `=`, y termina con
`end`. Entonces puede contener varias comprobaciones:

```rythme
fn doble(x) = x * 2

test "el doble conserva el signo"
  assert(doble(-3) < 0, "el doble de -3 debe ser negativo")
  assert_eq(doble(0), 0)
  assert_eq(doble(4), 8)
end

print(doble(-3))
```

```sortie
-6
```

`assert(condición, mensaje)` comprueba que una condición es verdadera. Si
es falsa, se muestra el mensaje.

| Función | Comprueba que |
|---------|---------------|
| `assert_eq(obtenido, esperado)` | los dos valores son iguales |
| `assert(condición, mensaje)` | la condición es verdadera |

> **Consejo:** usa mejor `assert_eq` cuando compares dos valores: si falla,
> muestra lo que obtuvo y lo que esperaba.

## Ejecutar las pruebas con rythme test

Aquí tienes un archivo `calculos.ry`, con dos funciones y tres pruebas:

```rythme
fn doble(x) = x * 2

fn promedio(notas) = sum(notas) / len(notas)

test "el doble de 21 es 42" = assert_eq(doble(21), 42)

test "el doble conserva el signo"
  assert(doble(-3) < 0, "el doble de -3 debe ser negativo")
  assert_eq(doble(0), 0)
end

test "el promedio de 10, 12 y 14" = assert_eq(promedio([10, 12, 14]), 12.0)

print(doble(5))
```

```sortie
10
```

Para ejecutar sus pruebas, usa `rythme test` en lugar de `rythme run`:

```sh
rythme test calculos.ry
```

```text
Ejecutando 3 prueba(s)
  ✓ el doble de 21 es 42
  ✓ el doble conserva el signo
  ✓ el promedio de 10, 12 y 14
3 prueba(s): 3 pasada(s), 0 fallida(s) (1 ms)
```

Cada `✓` es una prueba que pasa. La última línea hace el balance, con el
tiempo empleado entre paréntesis (cambia un poco de una vez a otra).

Durante `rythme test`, las líneas del nivel superior como `print(doble(5))`
se **saltan**: solo se ejecutan las pruebas.

## Cuando una prueba falla

Añadamos a `calculos.ry` una prueba incorrecta, a propósito:

```rythme norun
test "un error a propósito" = assert_eq(doble(3), 7)
```

```text
Ejecutando 4 prueba(s)
  ✓ el doble de 21 es 42
  ✓ el doble conserva el signo
  ✓ el promedio de 10, 12 y 14
  ✗ un error a propósito
error[E015]: aserción fallida: los valores son distintos
  --> calculos.ry:16:31
   |
16 | test "un error a propósito" = assert_eq(doble(3), 7)
   |                               ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: obtenido: 6
       esperado: 7
   = más sobre este error: `rythme explain E015`

4 prueba(s): 3 pasada(s), 1 fallida(s) (0 ms)
```

Léelo de arriba abajo:

1. `✗` marca la prueba que ha fallado, con su nombre;
2. la línea y la columna del problema: `calculos.ry:16:31`;
3. lo que la función ha **obtenido** (`6`) y lo que se **esperaba** (`7`);
4. el balance: 3 pasadas, 1 fallida.

Aquí, la que se equivoca es la prueba: el doble de 3 es 6. A veces, es la
función: entonces la prueba te ha evitado un bug.

Con `assert`, se muestra el mensaje que escribiste. Esta prueba también se
equivoca: debería haber comprobado `not mayor?(17)`.

```rythme norun
fn mayor?(edad) = edad >= 18

test "con 18 años es mayor" = assert(mayor?(18), "con 18 años debería ser mayor")
test "con 17 años es menor" = assert(mayor?(17), "con 17 años debería ser menor")
```

```text
Ejecutando 2 prueba(s)
  ✓ con 18 años es mayor
  ✗ con 17 años es menor
error[E015]: aserción fallida: con 17 años debería ser menor
 --> edad.ry:4:31
  |
4 | test "con 17 años es menor" = assert(mayor?(17), "con 17 años debería ser menor")
  |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ esta condición es falsa
  |
  = más sobre este error: `rythme explain E015`

2 prueba(s): 1 pasada(s), 1 fallida(s) (0 ms)
```

> **Nota:** cuando todas las pruebas pasan, `rythme test` termina con el
> código 0; si no, con el código 1. Las herramientas de integración continua
> (que ejecutan las pruebas en cada cambio) se basan en eso.

## Elegir pruebas con --filter

Cuando un archivo tiene muchas pruebas, `--filter` ejecuta solo aquellas
cuyo nombre contiene una palabra (sin distinguir mayúsculas y minúsculas):

```sh
rythme test calculos.ry --filter doble
```

```text
Ejecutando 2 de 4 prueba(s)
  ✓ el doble de 21 es 42
  ✓ el doble conserva el signo
2 prueba(s): 2 pasada(s), 0 fallida(s), 2 descartada(s) por el filtro (0 ms)
```

Una razón más para dar a las pruebas nombres claros.

## Volver a ejecutar en cada cambio con --watch

```sh
rythme test calculos.ry --watch
```

`--watch` ejecuta las pruebas y luego **espera**. Cada vez que guardas el
archivo, las pruebas se vuelven a ejecutar solas:

```text
Vigilando 1 archivo(s): las pruebas se vuelven a ejecutar cuando uno cambia (Ctrl+C para parar).
```

Deja la terminal abierta junto a tu editor: ves el resultado cada vez que
guardas. `Ctrl+C` detiene la vigilancia.

## Probar los errores y los Results

Una función que puede fallar devuelve un `Result` (ver
[gestionar los errores](erreurs.md)). Se prueban los dos casos con
`assert_eq`, escribiendo el `Ok(...)` o el `Err(...)` esperado:

```rythme
fn dividir(a, b) = if b == 0 then Err("división por cero") else Ok(a / b)

test "dividir 10 entre 4" = assert_eq(dividir(10, 4), Ok(2.5))

test "dividir entre cero da un error" = assert_eq(dividir(1, 0), Err("división por cero"))

test "dos divisiones seguidas"
  mitad = dividir(100, 2)?
  decimo = dividir(mitad, 5)?
  assert_eq(decimo, 10.0)
end

print(dividir(10, 4), dividir(1, 0))
```

```sortie
Ok(2.5) Err("división por cero")
```

La tercera prueba usa `?`, como en una función: si un paso da `Err(...)`,
la prueba se detiene y **falla**, mostrando el error. Por ejemplo, con esta
prueba, en la línea 13 del archivo `banco.ry`:

```rythme norun
test "una división fallida"
  x = dividir(100, 0)?
  assert_eq(x, 1)
end
```

```text
  ✗ una división fallida
error[E015]: la prueba terminó con Err("división por cero")
  --> banco.ry:13:1
   |
13 | test "una división fallida"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = ayuda: una prueba pasa cuando termina sin error, y su valor no es `false`, `Err(...)` ni `None`
   = más sobre este error: `rythme explain E015`
```

La regla es sencilla: una prueba **pasa** si termina sin error, y su
resultado no es `false`, ni `Err(...)`, ni `None`.

## Dónde guardar las pruebas

**Junto al código.** Es lo más sencillo: las pruebas de una función están
justo debajo de ella, en el mismo archivo. Cuando modificas la función,
tienes sus pruebas delante.

**En un archivo aparte.** Para un módulo (ver [los módulos](modules.md)),
también se pueden escribir las pruebas en otro archivo, que lo usa con
`use`. Aquí tienes `geometria_test.ry`, junto a `geometria.ry`:

```rythme norun
use geometria

test "el área de un rectángulo de 2 por 3" = assert_eq(geometria.area_rectangulo(2, 3), 6)

test "el área de un cuadrado de lado 4" = assert_eq(geometria.area_cuadrado(4), 16)
```

**Toda una carpeta de una vez.** `rythme test .` ejecuta las pruebas de
todos los archivos `.ry` de la carpeta (y de sus subcarpetas):

```sh
rythme test .
```

```text
Ejecutando 3 prueba(s) de ./calculos.ry
  ✓ el doble de 21 es 42
  ✓ el doble conserva el signo
  ✓ el promedio de 10, 12 y 14
Ejecutando 2 prueba(s) de ./geometria_test.ry
  ✓ el área de un rectángulo de 2 por 3
  ✓ el área de un cuadrado de lado 4
2 archivo(s), 5 prueba(s): 5 pasada(s), 0 fallida(s) (1 ms)
```

En un proyecto creado con `rythme new`, las pruebas van en la carpeta
`tests/`, y `rythme test` a secas las ejecuta (ver
[organizar un proyecto](projet.md)).

## Errores frecuentes

### Olvidar las comillas alrededor del nombre

```rythme norun
fn doble(x) = x * 2

test doble = assert_eq(doble(21), 42)
```

```text
error[E001]: se esperaba el nombre de la prueba, como texto, pero se encontró el nombre `doble`
 --> calculos.ry:3:6
  |
3 | test doble = assert_eq(doble(21), 42)
  |      ^^^^^ se esperaba el nombre de la prueba, como texto aquí
  |
  = ayuda: ejemplo: `test "la suma funciona" = assert_eq(1 + 1, 2)`
  = más sobre este error: `rythme explain E001`
```

Corrección: el nombre es un texto, `test "el doble de 21" = ...`.

### Escribir una prueba dentro de una función

```rythme norun
fn doble(x)
  test "el doble" = assert_eq(x * 2, 42)
  x * 2
end
```

```text
error[E001]: `test` solo se puede usar en el nivel superior de un archivo
 --> calculos.ry:2:3
  |
2 |   test "el doble" = assert_eq(x * 2, 42)
  |   ^^^^
  |
  = ayuda: sácalo de la función o del bloque
  = más sobre este error: `rythme explain E001`
```

Corrección: escribe la prueba **después** de la función, en el nivel
superior, sin sangría: `test "el doble de 21" = assert_eq(doble(21), 42)`.

### Comparar con == en lugar de assert_eq

Una prueba que termina en `false` falla, pero sin decir por qué:

```rythme norun
fn doble(x) = x * 2

test "el doble de 3" = doble(3) == 7
```

```text
Ejecutando 1 prueba(s)
  ✗ el doble de 3
error[E015]: la prueba devolvió `false`
 --> calculos.ry:3:1
  |
3 | test "el doble de 3" = doble(3) == 7
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = ayuda: usa `assert_eq(obtenido, esperado)` para ver los valores
  = más sobre este error: `rythme explain E015`

1 prueba(s): 0 pasada(s), 1 fallida(s) (0 ms)
```

Corrección: `test "el doble de 3" = assert_eq(doble(3), 6)`. Si falla,
`assert_eq` muestra el valor obtenido y el valor esperado.

## Ponte a prueba

1. ¿Qué hace `rythme run` con un archivo que contiene pruebas?

> **Respuesta**
>
> Ignora las pruebas y ejecuta el programa normalmente. Solo `rythme test`
> ejecuta las pruebas (y entonces se salta las demás líneas del nivel
> superior, como los `print`).

2. Escribe una prueba que compruebe que `upper("rythme")` da `"RYTHME"`.

> **Respuesta**
>
> ```rythme
> test "upper pone en mayúsculas" = assert_eq(upper("rythme"), "RYTHME")
> ```

3. ¿Cómo ejecutar solo las pruebas cuyo nombre contiene «promedio»?

> **Respuesta**
>
> `rythme test calculos.ry --filter promedio`.

## En resumen

- `test "nombre" = assert_eq(obtenido, esperado)` escribe una prueba en una
  línea; `test "nombre"` ... `end` escribe una de varias líneas.
- `assert_eq` compara dos valores, `assert(condición, mensaje)` comprueba
  una condición.
- `rythme test archivo.ry` ejecuta las pruebas; `rythme run` las ignora.
- Una prueba que falla muestra su nombre, la línea, y lo que se obtuvo y lo
  que se esperaba.
- `--filter palabra` elige pruebas, `--watch` las vuelve a ejecutar en cada
  cambio, `rythme test .` ejecuta toda la carpeta.
- Una prueba falla si se detiene en un error o termina en `false`,
  `Err(...)` o `None`: `?` está permitido dentro.

Siguiente: [escribir bien en Rythme](style.md)
