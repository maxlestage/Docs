# Gestionar los errores

Rythme no tiene ni `null` ni excepciones: esta página muestra cómo un valor ausente se convierte en una `Option`, cómo un fallo se convierte en un `Result`, y cómo leer los mensajes de error.

> **Objetivos:**
>
> - sabrás usar una `Option` para un valor que puede faltar;
> - sabrás usar un `Result` para una operación que puede fallar;
> - sabrás abrir una `Option` o un `Result` con `match`, `unwrap_or` y `?`;
> - sabrás transformar estos valores con `map`, `and_then`, `ok_or` y `map_err`;
> - sabrás leer un mensaje de error de Rythme y encontrar su explicación.

## Sin null y sin excepciones

Un [error](../debuter/glossaire.md#error) es lo que impide que un programa
haga lo que se espera. En muchos lenguajes, un valor puede ser «nulo»
([null](../debuter/glossaire.md#null)), y un error puede surgir en
cualquier sitio, como una **excepción** que lo interrumpe todo.

En Rythme, no. Los problemas posibles son **valores** normales, que tratas
como los demás:

- un valor que puede faltar es una `Option`: `Some(valor)` o `None`;
- una operación que puede fallar da un `Result`: `Ok(valor)` o
  `Err(error)`.

Piensa en un paquete: antes de usar lo que hay dentro, abres la caja y
miras si está llena o vacía. Rythme no te deja olvidarte de mirar.

## Option: un valor que puede faltar

`first` da el primer elemento de una lista... si lo hay. `find` da el primer
elemento que cumple la condición... si lo hay:

```rythme
print(first([10, 20]), first([]))
print(find([1, 4, 7], is_even), find([1, 3], is_even))
```

```sortie
Some(10) None
Some(4) None
```

`Some(10)` es una caja que contiene 10. `None` es una caja vacía.

Los dos casos se tratan con `match`:

```rythme
fn describir(lista) = match first(lista)
  Some(x) -> "empieza por {x}"
  None -> "lista vacía"
end

print(describir([5, 6]), "/", describir([]))
```

```sortie
empieza por 5 / lista vacía
```

## Result: una operación que puede fallar

`parse_int` lee un número en un texto. Puede que el texto no sea un número:
por eso da un `Result`, `Ok(número)` o `Err(mensaje)`.

```rythme
print(parse_int("42"), parse_int("cuarenta"))
```

```sortie
Ok(42) Err("\"cuarenta\" no es un número entero")
```

`Option` solo dice «nada»; `Result` también dice **por qué** ha fallado.

Tus funciones también pueden devolver un `Result`:

```rythme
fn dividir(a, b) = if b == 0 then Err("división por cero") else Ok(a / b)

fn mostrar(resultado) = match resultado
  Ok(v) -> "resultado: {v}"
  Err(e) -> "error: {e}"
end

print(mostrar(dividir(10, 4)))
print(mostrar(dividir(1, 0)))
```

```sortie
resultado: 2.5
error: división por cero
```

## Un valor por defecto con unwrap_or

`unwrap_or` da el valor que contiene, o un valor por defecto si la caja está
vacía o contiene un error:

```rythme
notas = dict([["Ada", 19]])
print(unwrap_or(get(notas, "Ada"), 0), unwrap_or(get(notas, "Eve"), 0))
print(parse_int("12") |> unwrap_or(0), parse_int("??") |> unwrap_or(0))
```

```sortie
19 0
12 0
```

> **Atención:** `unwrap(x)` da el valor, pero detiene el programa con
> `None` o `Err` (error E013). Mejor usa `unwrap_or` o `match`.

## El operador ? para pasar el error

Dentro de una función, `?` después de un valor quiere decir:

- si es `Ok(x)` o `Some(x)`, sigue con `x`;
- si es `Err(...)` o `None`, **detén la función** y devuelve ese valor.

```rythme
fn sumar(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(sumar("1", "2"))
print(sumar("1", "dos"))
```

```sortie
Ok(3)
Err("\"dos\" no es un número entero")
```

Sin `?`, haría falta un `match` para cada lectura. Con `?`, el camino normal
se lee de un tirón, y el error sube solo, como un paquete dañado que se
devuelve al remitente.

> **Nota:** `?` solo funciona dentro de una función (o de un test). Esa
> función devuelve entonces un `Result` o una `Option`: de ahí el
> `Ok(x + y)` del final.

## Transformar sin abrir la caja

Estas funciones trabajan sobre el contenido, y dejan `None` o `Err` tal
cual:

| Función | Qué hace |
|---------|----------|
| `map(x, f)` | aplica `f` al valor que hay en `Some` u `Ok` |
| `and_then(x, f)` | un paso más que puede fallar: `f` da una Option o un Result |
| `ok_or(option, error)` | convierte una Option en Result: `None` pasa a ser `Err(error)` |
| `map_err(result, f)` | cambia el mensaje de un `Err` |

```rythme
fn positivo(n) = if n > 0 then Ok(n) else Err("no es positivo")

print(Some(3) |> map(fn(x) = x * 10), None |> map(fn(x) = x * 10))
print(parse_int("8") |> and_then(positivo), parse_int("-2") |> and_then(positivo))
print(first([]) |> ok_or("lista vacía"))
print(parse_int("x") |> map_err(fn(e) = "entrada incorrecta"))
```

```sortie
Some(30) None
Ok(8) Err("no es positivo")
Err("lista vacía")
Err("entrada incorrecta")
```

`ok_or` es práctico con `?`: en una función que devuelve un `Result`,
`first(lista) |> ok_or("lista vacía")?` da el primer elemento, o detiene la
función con un mensaje claro.

## Otras herramientas

| Función | Qué hace |
|---------|----------|
| `is_some(x)`, `is_none(x)` | ¿contiene un valor la Option? |
| `is_ok(x)`, `is_err(x)` | ¿es un éxito el Result? |
| `fail(mensaje)` | detiene el programa con un mensaje |

```rythme
print(is_some(first([1, 2])), is_none(first([])))
print(is_ok(parse_int("7")), is_err(parse_int("x")))
```

```sortie
true true
true true
```

Guarda `fail` para las situaciones imposibles, cuando el programa de verdad
no puede continuar.

## Mensajes de error que ayudan

Cuando algo va mal, Rythme dice **dónde**, **por qué** y **cómo
corregirlo**. Por ejemplo, este programa suma un número a un texto:

```rythme norun
edad = 36
print("edad: " + edad)
```

```text
error[E005]: no se puede usar `+` con Text y Int
 --> main.ry:2:16
  |
2 | print("edad: " + edad)
  |                ^ Text "edad: " + Int
  |
  = ayuda: para construir un texto con un valor dentro, usa llaves: "total: {x}"
      o convierte antes el valor: text(x)
  = más sobre este error: `rythme explain E005`
```

Ahí lees:

1. el **código** del error (`E005`) y lo que ha pasado;
2. el archivo, la línea y la columna (`main.ry:2:16`);
3. la línea de código, con el sitio exacto marcado con `^`;
4. una **ayuda** para corregirlo;
5. el comando para saber más.

Este error se encuentra **antes** de la ejecución: no se ha lanzado nada. Un
mensaje que empieza por `advertencia` señala un problema probable, pero el
programa se ejecuta de todos modos.

## Explicar un código de error

Cada código tiene una explicación detallada, con ejemplos:

```sh
rythme explain E005
```

```text
E005 — Tipo de valor incorrecto

La operación no funciona con este tipo de valor. Rythme nunca convierte los
valores sin avisar: "1" + 1 es un error, no "11" ni 2.

Mal:     "edad: " + 36
Bien:    "edad: {36}"             (insertar el valor en el texto)
Bien:    "edad: " + text(36)      (convertirlo antes)
```

Estos son los códigos más frecuentes al principio:

| Código | Significado |
|--------|-------------|
| E001 | error de sintaxis (a menudo una palabra de otro lenguaje: `for`, `return`, `null`...) |
| E003 | nombre desconocido (a menudo una errata: Rythme propone el nombre correcto) |
| E004 | número de argumentos incorrecto |
| E005 | tipo de valor incorrecto |
| E006 | ningún caso coincide |
| E007 | índice fuera de rango |
| E011 | nombre ya definido (un valor no cambia) |
| E019 | `if` sin `else` |

## Errores frecuentes

### Buscar null

```rythme norun
nombre = null
```

```text
error[E001]: `null` no se usa en Rythme
 --> main.ry:1:10
  |
1 | nombre = null
  |          ^^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene null: usa `None` para «ningún valor» y `Some(x)` para un valor
  = más sobre este error: `rythme explain E001`
```

Un valor que puede faltar se escribe `None`, y `Some("Ada")` cuando está
ahí.

### Calcular con una Option sin abrirla

```rythme norun
notas = [12, 15]
print(first(notas) + 1)
```

```text
error[E005]: no se puede usar `+` con Option y Int
 --> main.ry:2:20
  |
2 | print(first(notas) + 1)
  |                    ^ Option + Int 1
  |
  = más sobre este error: `rythme explain E005`
```

`first(notas)` es la caja `Some(12)`, no el número 12. Ábrela:
`unwrap_or(first(notas), 0) + 1`, o con `match`.

### Usar ? fuera de una función

```rythme norun
n = parse_int("doce")?
print(n)
```

```text
error[E016]: `?` solo se puede usar dentro de una función
 --> main.ry:1:5
  |
1 | n = parse_int("doce")?
  |     ^^^^^^^^^^^^^^^^^^
  |
  = ayuda: fuera de una función, trata el valor con `match` o `unwrap_or`
  = más sobre este error: `rythme explain E016`
```

`?` devuelve el error a la función que llama; en el nivel superior del
programa, no hay ninguna. Usa `match`, o `unwrap_or`.

### Abrir a la fuerza con unwrap

```rythme norun
print(unwrap(parse_int("doce")))
```

```text
error[E013]: `unwrap` encontró un error: "doce" no es un número entero
 --> main.ry:1:7
  |
1 | print(unwrap(parse_int("doce")))
  |       ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = ayuda: trata el error con `match`, pásalo con `?`, o da un valor por defecto: `unwrap_or(x, por_defecto)`
  = más sobre este error: `rythme explain E013`
```

`unwrap` supone que todo va bien, y si no, detiene el programa. Prevé el
caso de fallo: `parse_int("doce") |> unwrap_or(0)`.

## Ponte a prueba

**Pregunta 1:** ¿qué dan `get([1, 2], 5)` y `get([1, 2], 0)`?

> **Respuesta**
>
> `None` y `Some(1)`. La posición 5 no existe: la caja está vacía.

**Pregunta 2:** ¿cuándo usar `Result` en lugar de `Option`?

> **Respuesta**
>
> Cuando quieres decir **por qué** ha fallado: `Err("archivo
> no encontrado")` lleva un mensaje, `None` no dice nada.

**Pregunta 3:** ¿qué devuelve `sumar("5", "x")`, con la función `sumar` de
esta página?

> **Respuesta**
>
> `Err("\"x\" no es un número entero")`. El primer `?` deja pasar el 5, el
> segundo encuentra un error y lo devuelve enseguida.

## En resumen

- Un valor que puede faltar es una `Option`: `Some(x)` o `None`.
- Una operación que puede fallar da un `Result`: `Ok(x)` o `Err(e)`.
- Se abren con `match`, o con `unwrap_or` para un valor por defecto;
  `map`, `and_then`, `ok_or` y `map_err` los transforman.
- Dentro de una función, `?` sigue con el valor, o devuelve el error.
- Cada mensaje de error dice dónde, por qué y cómo corregirlo;
  `rythme explain CODE` dice más.

Siguiente: [organizar el código en módulos](modules.md). Compara con los
errores en [Ruby](../ruby/erreurs.md) y en [Rust](../rust/erreurs.md).
