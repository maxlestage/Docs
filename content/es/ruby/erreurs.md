# Gestionar los errores

Esta página explica cómo atrapar los errores con `begin` y `rescue`, cómo lanzarlos con `raise` y cómo evitar las trampas de `nil`.

> **Objetivos:**
>
> - sabrás leer un mensaje de error y encontrar la línea culpable;
> - sabrás atrapar un error con `begin` y `rescue`;
> - sabrás ejecutar código en todos los casos con `ensure`;
> - sabrás lanzar tus propios errores con `raise`;
> - sabrás evitar los errores con `nil` gracias a `&.`.

## Un error detiene el programa

Cuando Ruby encuentra un problema, lanza una **excepción**: un objeto que describe el
[error](../debuter/glossaire.md#error). «Lanzar» quiere decir: hacer sonar la alarma.
Si no se trata, el programa se detiene.

```ruby norun
puts "Inicio"
puts 10 / 0
puts "Fin"
```

```text
Inicio
main.rb:2:in `/': divided by 0 (ZeroDivisionError)
	from main.rb:2:in `<main>'
```

`Fin` nunca se muestra: el programa se detiene en la línea 2.

### Leer el mensaje

El mensaje se lee en tres trozos:

- `main.rb:2`: el archivo y la línea;
- `divided by 0`: la descripción («dividido entre 0»);
- `(ZeroDivisionError)`: la **clase** del error, su apellido.

Las líneas `from ...` forman la **traza de llamadas**: el camino que siguió el programa
hasta el error, desde el método más profundo hasta el programa principal (`<main>`).
Busca la primera línea que habla de **tu** archivo.

### Los errores más comunes

| Clase | ¿Cuándo? |
|--------|---------|
| `NoMethodError` | método desconocido, a menudo llamado sobre `nil` |
| `NameError` | variable o constante desconocida (error de escritura) |
| `ArgumentError` | número o valor de argumentos incorrecto |
| `TypeError` | tipo incorrecto, por ejemplo `"a" + 1` |
| `ZeroDivisionError` | división entera entre cero |
| `KeyError` | clave ausente con `fetch` |
| `Errno::ENOENT` | archivo no encontrado |

## begin y rescue

`rescue` («rescatar») atrapa el error. El programa sigue en vez de detenerse. Es una red
de seguridad bajo un trapecista: la caída se detiene y el espectáculo continúa.

- El código arriesgado va entre `begin` y `rescue`.
- El código de rescate va entre `rescue` y `end`.

```ruby
begin
  puts 10 / 0
rescue ZeroDivisionError
  puts "¡No se divide entre cero!"
end
puts "El programa continúa."
```

```sortie
¡No se divide entre cero!
El programa continúa.
```

### Leer el mensaje

Con `=> e`, obtienes la excepción y su mensaje.

```ruby
begin
  Integer("abc")
rescue ArgumentError => e
  puts "Error: #{e.message}"
end
```

```sortie
Error: invalid value for Integer(): "abc"
```

> **Atención:** atrapa errores precisos (`ArgumentError`, `ZeroDivisionError`…).
> Un `rescue` demasiado amplio puede esconder bugs de verdad.

### Varios rescue y else

Puedes escribir varios `rescue`, uno por clase de error. El código de `else` se ejecuta
solo si **no** ha habido error.

```ruby
def dividir(texto_a, texto_b)
  resultado = Integer(texto_a) / Integer(texto_b)
rescue ArgumentError
  puts "No es un número."
rescue ZeroDivisionError
  puts "División entre cero."
else
  puts "Resultado: #{resultado}"
end

dividir("10", "2")
dividir("diez", "2")
dividir("10", "0")
```

```sortie
Resultado: 5
No es un número.
División entre cero.
```

> **Nota:** `Integer("diez")` lanza un error, mientras que `"diez".to_i` devuelve `0` sin
> decir nada. Usa `Integer(...)` cuando haya que detectar una entrada no válida.

## ensure

El código de `ensure` («asegurar») se ejecuta **siempre**, haya error o no. Sirve para
limpiar (cerrar un archivo, por ejemplo).

```ruby
begin
  puts "Apertura"
  raise "uy"
rescue => e
  puts "Problema: #{e.message}"
ensure
  puts "Cierre"
end
```

```sortie
Apertura
Problema: uy
Cierre
```

## rescue en un método

En un método, `begin` no hace falta: `rescue` se coloca directamente antes de `end`.
Protege todo el cuerpo del método.

```ruby
def leer_numero(texto)
  Integer(texto)
rescue ArgumentError
  0
end

puts leer_numero("42")
puts leer_numero("cuarenta")
```

```sortie
42
0
```

## Lanzar un error con raise

`raise` lanza un error tú mismo, por ejemplo para rechazar un valor. Indica la clase del
error y luego un mensaje claro. Sin clase, `raise "mensaje"` lanza un `RuntimeError`.

```ruby
def retirar(saldo, cantidad)
  raise ArgumentError, "cantidad negativa" if cantidad < 0
  saldo - cantidad
end

begin
  retirar(100, -5)
rescue ArgumentError => e
  puts e.message
end
```

```sortie
cantidad negativa
```

## Tus propias excepciones

Un error es un objeto como cualquier otro: su clase hereda de una clase de error
(mira [la herencia](types.md)). Crea una clase que herede de `StandardError`. El nombre
suele terminar en `Error`.

```ruby
class SaldoInsuficienteError < StandardError
end

begin
  raise SaldoInsuficienteError, "saldo insuficiente"
rescue SaldoInsuficienteError => e
  puts "Rechazado: #{e.message}"
end
```

```sortie
Rechazado: saldo insuficiente
```

> **Recuerda:** hereda de `StandardError`, no de `Exception`. Un `rescue` simple solo
> atrapa los `StandardError`.

## Reintentar con retry

`retry` vuelve a lanzar el bloque `begin` desde el principio. Limita siempre el número
de intentos.

```ruby
intentos = 0
begin
  intentos += 1
  raise "red no disponible" if intentos < 3
  puts "Conectado tras #{intentos} intentos"
rescue
  retry if intentos < 3
end
```

```sortie
Conectado tras 3 intentos
```

## Protegerse de nil

`nil` («nada») aparece a menudo sin avisar: una casilla de array que no existe, una
clave ausente… Llamar a un método sobre `nil` provoca un `NoMethodError`.

El operador `&.`, llamado **navegación segura**, evita esta trampa: si el objeto vale
`nil`, el método no se llama y el resultado es `nil`. Es como llamar a una puerta solo
si hay una casa.

```ruby
usuario = nil
p usuario&.upcase

nombre = "alice"
p nombre&.upcase
```

```sortie
nil
"ALICE"
```

Combínalo con `||` para dar un valor por defecto:

```ruby
ciudad = nil
puts ciudad&.capitalize || "Desconocida"
```

```sortie
Desconocida
```

> **Consejo:** en un hash, `fetch` con un valor por defecto también evita los `nil`
> (mira [los hashes](collections.md#los-hashes)).

## Errores frecuentes

### Atrapar la clase de error equivocada

```ruby norun
begin
  Integer("abc")
rescue ZeroDivisionError
  puts "Problema"
end
```

```text
<internal:kernel>:307:in `Integer': invalid value for Integer(): "abc" (ArgumentError)
	from main.rb:2:in `<main>'
```

`rescue ZeroDivisionError` solo atrapa las divisiones entre cero. Aquí, el error es un
`ArgumentError`: se cuela. Indica la clase correcta, la que aparece entre paréntesis en
el mensaje: `rescue ArgumentError`.

### Heredar de Exception en vez de StandardError

```ruby norun
class MiError < Exception
end
begin
  raise MiError, "uy"
rescue => e
  puts "Atrapado"
end
```

```text
main.rb:4:in `<main>': uy (MiError)
```

Un `rescue` sin clase solo atrapa los `StandardError`. Escribe
`class MiError < StandardError`.

### Llamar a un método sobre nil

```ruby norun
usuarios = { alice: "Alice" }
puts usuarios[:bob].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts usuarios[:bob].upcase
                   ^^^^^^^
```

`usuarios[:bob]` vale `nil`. Prevé el caso: `usuarios[:bob]&.upcase`, o
`usuarios.fetch(:bob, "desconocido").upcase`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```ruby norun
begin
  puts "A"
  raise "avería"
  puts "B"
rescue
  puts "C"
ensure
  puts "D"
end
```

> **Respuesta**
>
> `A`, `C` y luego `D`. `B` nunca se muestra: después de `raise`, Ruby salta directamente
> al `rescue`. `ensure` se ejecuta siempre.

**Pregunta 2:** ¿cuánto vale `nil&.length`?

> **Respuesta**
>
> `nil`. Con `&.`, el método no se llama sobre `nil`, y no hay error.

**Pregunta 3:** escribe un método `edad_valida(n)` que lance un `ArgumentError` si `n`
es negativo, y que devuelva `n` en caso contrario.

> **Respuesta**
>
> ```ruby
> def edad_valida(n)
>   raise ArgumentError, "edad negativa" if n < 0
>   n
> end
>
> puts edad_valida(12)
> begin
>   edad_valida(-1)
> rescue ArgumentError => e
>   puts e.message
> end
> ```
>
> ```sortie
> 12
> edad negativa
> ```

## En resumen

- Un error no tratado detiene el programa; su mensaje da la línea y la clase.
- `begin` y `rescue` atrapan un error; `=> e` da su mensaje.
- `ensure` se ejecuta siempre, con o sin error.
- `raise` lanza un error; tus propios errores heredan de `StandardError`.
- `&.` evita el error cuando un valor vale `nil`.

Siguiente: [leer y escribir archivos](fichiers.md). Compara con
[los errores en Rust](../rust/erreurs.md).
