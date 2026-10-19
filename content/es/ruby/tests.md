# Probar tu código

Esta página muestra cómo escribir pruebas automáticas con Minitest, la herramienta que viene con Ruby, para comprobar que tu código hace de verdad lo que esperas.

> **Objetivos:**
>
> - entender para qué sirve una prueba automática;
> - escribir una clase de prueba con Minitest;
> - usar `assert_equal`, `assert`, `assert_nil` y `assert_raises`;
> - ejecutar las pruebas y leer el resultado;
> - escribir la prueba antes que el código, en tres pasos.

## ¿Por qué probar?

Una **prueba** (*test*) es un pequeño programa que comprueba otro programa (mira
[el glosario](../debuter/glossaire.md#test)). Llama a tu código con valores
conocidos y compara el resultado con el que esperas.

Sin pruebas, compruebas a mano, con `puts`, y vuelves a empezar después de cada
cambio. Con pruebas:

- un solo comando lo comprueba todo, en un segundo;
- si un cambio rompe algo, lo sabes enseguida;
- las pruebas muestran cómo usar tu código: es una documentación que funciona.

> **Recuerda:** una prueba responde a una pregunta sencilla: «con esta entrada,
> ¿obtengo de verdad esta salida?».

## Tu primera prueba con Minitest

**Minitest** viene con Ruby: no hay nada que instalar. Un archivo de prueba sigue
siempre el mismo modelo:

1. `require "minitest/autorun"` carga Minitest y ejecuta las pruebas al final del archivo;
2. una clase de prueba hereda de `Minitest::Test`;
3. cada método cuyo nombre empieza por `test_` es una prueba.

```ruby
# test_doble.rb
require "minitest/autorun"

def doble(x)
  x * 2
end

class TestDoble < Minitest::Test
  def test_doble_de_4
    assert_equal 8, doble(4)
  end

  def test_doble_de_cero
    assert_equal 0, doble(0)
  end
end
```

`assert_equal 8, doble(4)` se lee: «espero `8`; comprueba que `doble(4)` vale
de verdad `8`».

## Ejecutar las pruebas

Un archivo de prueba se ejecuta como cualquier programa:

```sh
ruby test_doble.rb
```

Ruby muestra un resumen:

```text
Run options: --seed 31187

# Running:

..

Finished in 0.000812s, 2463.0542 runs/s, 2463.0542 assertions/s.

2 runs, 2 assertions, 0 failures, 0 errors, 0 skips
```

- Cada `.` es una prueba superada.
- La última línea cuenta las pruebas (`runs`), las comprobaciones (`assertions`), los
  fallos (`failures`) y los errores (`errors`).
- El número después de `--seed` y el tiempo cambian cada vez: Minitest ejecuta las
  pruebas en un orden al azar, para asegurarse de que no dependen unas de otras.

> **Consejo:** `ruby test_doble.rb --verbose` muestra el nombre de cada prueba, y
> `ruby test_doble.rb -n test_doble_de_4` solo ejecuta esa prueba.

## Leer un fallo

Cuando una comprobación falla, Minitest muestra una `F` y explica el problema. Aquí
tienes un `doble` que se equivoca (suma 2 en lugar de multiplicar):

```ruby norun
# test_doble.rb
require "minitest/autorun"

def doble(x)
  x + 2
end

class TestDoble < Minitest::Test
  def test_doble_de_4
    assert_equal 8, doble(4)
  end
end
```

```text
F

  1) Failure:
TestDoble#test_doble_de_4 [test_doble.rb:10]:
Expected: 8
  Actual: 6

1 runs, 1 assertions, 1 failures, 0 errors, 0 skips
```

El mensaje te da todo lo necesario: la prueba (`test_doble_de_4`), la línea (`10`), el
valor esperado (`Expected: 8`) y el valor obtenido (`Actual: 6`).

- `F` (*failure*, fallo): una comprobación dio un resultado incorrecto.
- `E` (*error*): el código falló durante la prueba (un método desconocido, por
  ejemplo).

## Las aserciones

Una **aserción** es una comprobación. Minitest ofrece varias; aquí tienes las más
útiles.

| Aserción | Comprueba que… |
| --- | --- |
| `assert_equal esperado, obtenido` | los dos valores son iguales |
| `assert valor` | el valor es verdadero |
| `refute valor` | el valor es falso |
| `assert_nil valor` | el valor es `nil` |
| `assert_includes array, x` | el array contiene `x` |
| `assert_in_delta esperado, obtenido, 0.01` | dos números decimales están muy cerca |
| `assert_raises(Error) { … }` | el bloque lanza de verdad ese error |

```ruby
require "minitest/autorun"

class TestAserciones < Minitest::Test
  def test_assert_equal
    assert_equal "HOLA", "hola".upcase
    assert_equal [1, 2, 3], [3, 1, 2].sort
  end

  def test_assert_y_refute
    assert [1, 2, 3].include?(2)
    refute [].any?
  end

  def test_assert_nil
    assert_nil [].first
    assert_nil({ a: 1 }[:b])
  end

  def test_assert_includes
    assert_includes ["manzana", "pera"], "pera"
  end

  def test_assert_in_delta
    assert_in_delta 0.3, 0.1 + 0.2, 0.001
  end

  def test_assert_raises
    assert_raises(ZeroDivisionError) { 10 / 0 }
  end
end
```

> **Nota:** `0.1 + 0.2` vale `0.30000000000000004` en Ruby. Para comparar números
> decimales, usa mejor `assert_in_delta` que `assert_equal`.

### Probar que se lanza un error

A veces tu código debe rechazar un valor con `raise` (mira
[lanzar un error](erreurs.md#lanzar-un-error-con-raise)). `assert_raises` comprueba que el error ocurre de
verdad. Devuelve el error: también puedes comprobar su mensaje.

```ruby
require "minitest/autorun"

def retirar(saldo, cantidad)
  raise ArgumentError, "cantidad negativa" if cantidad < 0
  saldo - cantidad
end

class TestRetirar < Minitest::Test
  def test_retiro_normal
    assert_equal 70, retirar(100, 30)
  end

  def test_retiro_negativo_rechazado
    error = assert_raises(ArgumentError) { retirar(100, -5) }
    assert_equal "cantidad negativa", error.message
  end
end
```

### Un mensaje más claro

Cada aserción acepta un último argumento: el mensaje que se muestra si falla.

```ruby
require "minitest/autorun"

class TestCarrito < Minitest::Test
  def test_carrito_vacio_al_principio
    carrito = []
    assert carrito.empty?, "el carrito debería estar vacío al principio"
  end
end
```

## Probar una clase

Para una clase, crea un objeto en la prueba, llama a sus métodos y después comprueba
su estado. El método `setup` se ejecuta **antes de cada prueba**: prepara un objeto
nuevo.

```ruby
require "minitest/autorun"

class Cuenta
  attr_reader :saldo

  def initialize
    @saldo = 0
  end

  def depositar(cantidad)
    @saldo += cantidad
  end
end

class TestCuenta < Minitest::Test
  def setup
    @cuenta = Cuenta.new
  end

  def test_saldo_inicial
    assert_equal 0, @cuenta.saldo
  end

  def test_depositar_aumenta_el_saldo
    @cuenta.depositar(50)
    @cuenta.depositar(20)
    assert_equal 70, @cuenta.saldo
  end
end
```

Cada prueba recibe su propia `@cuenta`: el depósito de la segunda prueba no cambia
la primera.

## Ordenar el código y las pruebas

En un proyecto, el código y las pruebas viven en archivos separados. Por costumbre,
el código va en `lib/` y las pruebas en `test/`. El archivo de prueba lleva el nombre
del archivo probado, precedido de `test_`.

```text
mi_proyecto/
├── lib/
│   └── calculo.rb
└── test/
    └── test_calculo.rb
```

```ruby norun
# lib/calculo.rb
module Calculo
  def self.doble(x)
    x * 2
  end
end
```

```ruby norun
# test/test_calculo.rb
require "minitest/autorun"
require_relative "../lib/calculo"

class TestCalculo < Minitest::Test
  def test_doble
    assert_equal 42, Calculo.doble(21)
  end
end
```

Desde la carpeta `mi_proyecto`:

```sh
ruby test/test_calculo.rb
```

> **Nota:** `../` quiere decir «la carpeta padre». Desde `test/`, `../lib/calculo`
> designa `lib/calculo.rb` (mira [los módulos](modules.md)).

## Escribir la prueba primero

Una forma de trabajar muy extendida consiste en escribir la prueba **antes** que el
código. Se avanza en tres pasos, que se repiten: **rojo**, **verde**, **mejorar**.

Ejemplo: un método `calificacion(nota)` que devuelve `"Notable"` a partir de 14,
`"Aprobado"` a partir de 10, e `"Insuficiente"` por debajo.

### Paso 1: rojo, la prueba falla

Escribe primero la prueba, como si el método ya existiera.

```ruby norun
require "minitest/autorun"

class TestCalificacion < Minitest::Test
  def test_calificacion_notable
    assert_equal "Notable", calificacion(15)
  end
end
```

La prueba falla, porque `calificacion` todavía no existe:

```text
  1) Error:
TestCalificacion#test_calificacion_notable:
NoMethodError: undefined method `calificacion' for an instance of TestCalificacion
```

Es normal, y es útil: sabes que la prueba comprueba algo de verdad.

### Paso 2: verde, la prueba pasa

Escribe el código más sencillo que haga pasar las pruebas. Añade una prueba cada vez
para cada caso.

```ruby
require "minitest/autorun"

def calificacion(nota)
  if nota >= 14
    "Notable"
  elsif nota >= 10
    "Aprobado"
  else
    "Insuficiente"
  end
end

class TestCalificacion < Minitest::Test
  def test_calificacion_notable
    assert_equal "Notable", calificacion(15)
  end

  def test_calificacion_aprobado
    assert_equal "Aprobado", calificacion(10)
  end

  def test_calificacion_insuficiente
    assert_equal "Insuficiente", calificacion(7)
  end
end
```

Todas las pruebas están en verde: tres puntos `...` en el resumen.

### Paso 3: mejorar el código

Ahora que las pruebas te protegen, haz el código más claro. Aquí, un `case`
con rangos se lee mejor. Vuelve a ejecutar las pruebas: si siguen en verde, no se ha
roto nada.

```ruby
require "minitest/autorun"

def calificacion(nota)
  case nota
  when 14.. then "Notable"
  when 10...14 then "Aprobado"
  else "Insuficiente"
  end
end

class TestCalificacion < Minitest::Test
  def test_calificacion_notable
    assert_equal "Notable", calificacion(15)
  end

  def test_calificacion_aprobado
    assert_equal "Aprobado", calificacion(10)
  end

  def test_calificacion_insuficiente
    assert_equal "Insuficiente", calificacion(7)
  end
end
```

> **Recuerda:** rojo (una prueba que falla), verde (el código mínimo que la hace
> pasar), mejorar (sin cambiar el comportamiento). Después, vuelves a empezar con el
> caso siguiente.

## Nombrar bien tus pruebas

Un buen nombre de prueba dice **qué se comprueba**. Cuando falla, su nombre basta
para entender el problema.

| Menos claro | Más claro |
| --- | --- |
| `test_1` | `test_doble_de_cero_es_cero` |
| `test_retirar` | `test_retiro_negativo_rechazado` |
| `test_calificacion` | `test_calificacion_notable_desde_14` |

Algunas costumbres:

- una prueba comprueba **una sola idea**;
- prueba los casos límite: cero, un array vacío, el valor justo en la frontera
  (`calificacion(14)`, `calificacion(13.9)`);
- la clase de prueba lleva el nombre de lo que prueba: `TestCuenta` para `Cuenta`.

## RSpec en dos palabras

Muchos proyectos Ruby usan **RSpec**, otra biblioteca de pruebas. Se instala como
una gema (`gem install rspec`) y se lee casi como una frase en inglés:

```ruby norun
# spec/calculo_spec.rb
require_relative "../lib/calculo"

RSpec.describe Calculo do
  it "duplica un número" do
    expect(Calculo.doble(21)).to eq(42)
  end
end
```

```sh
rspec
```

Las ideas son las mismas que con Minitest. Empieza por Minitest: ya está instalado.

## Errores frecuentes

### Un nombre de método sin test_

Minitest solo ejecuta los métodos cuyo nombre empieza por `test_`. Los demás se
ignoran, sin ningún mensaje de error.

```ruby norun
class TestDoble < Minitest::Test
  def doble_de_cuatro
    assert_equal 8, doble(4)
  end
end
```

```text
0 runs, 0 assertions, 0 failures, 0 errors, 0 skips
```

`0 runs` debe alertarte. Cambia el nombre del método a `test_doble_de_cuatro`.

### Olvidar require "minitest/autorun"

Sin esta línea, Ruby no conoce Minitest.

```ruby norun
class TestDoble < Minitest::Test
end
```

```text
main.rb:1:in `<main>': uninitialized constant Minitest (NameError)
```

Añade `require "minitest/autorun"` en la primera línea.

### Invertir el valor esperado y el valor obtenido

`assert_equal` espera **primero** el valor esperado, **después** el calculado.
Si los inviertes, el mensaje de fallo te engaña.

```ruby norun
def precio_con_iva(precio)
  precio * 1.2
end

assert_equal precio_con_iva(10), 12.5
```

```text
Expected: 12.0
  Actual: 12.5
```

El mensaje dice que tu código dio `12.5`, cuando en realidad dio `12.0`. Escribe
`assert_equal 12.5, precio_con_iva(10)`.

### Dar un valor a assert_raises en lugar de un bloque

El código que debe lanzar el error va **en un bloque**. Si no, se ejecuta antes de
`assert_raises`, y la prueba falla con un error.

```ruby norun
assert_raises(ZeroDivisionError, 10 / 0)
```

```text
  1) Error:
TestDivision#test_division_por_cero:
ZeroDivisionError: divided by 0
```

Escribe `assert_raises(ZeroDivisionError) { 10 / 0 }`.

## Ponte a prueba

**1.** ¿Qué significan `.`, `F` y `E` en el resultado de Minitest?

> **Respuesta**
>
> `.`: una prueba superada. `F`: un fallo, una aserción dio un resultado incorrecto.
> `E`: un error, el código falló durante la prueba.

**2.** El resumen muestra `0 runs`. ¿Cuál es la causa más probable?

> **Respuesta**
>
> Los métodos de prueba no empiezan por `test_`: Minitest no los ve.

**3.** ¿Cuáles son los tres pasos cuando escribes la prueba primero?

> **Respuesta**
>
> Rojo: escribir una prueba que falla. Verde: escribir el código más sencillo que la
> hace pasar. Mejorar: hacer el código más claro, manteniendo las pruebas en verde.

## En resumen

- Una prueba comprueba automáticamente que una entrada da la salida esperada.
- Con Minitest: `require "minitest/autorun"`, una clase que hereda de
  `Minitest::Test`, métodos `test_…`.
- `assert_equal esperado, obtenido` es la aserción más habitual; `assert_raises`
  recibe un bloque.
- `ruby test_archivo.rb` ejecuta las pruebas: `.` superada, `F` fallo, `E` error.
- Rojo, verde, mejorar: escribe la prueba, luego el código, y después hazlo más claro.

Siguiente: [depurar en Ruby](debogage.md)
