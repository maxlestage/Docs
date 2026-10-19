# Probar tu código

Esta página muestra cómo escribir pruebas automáticas en Python, con `assert`, el módulo `unittest` que viene con Python, y después pytest y doctest, para comprobar que tu código hace de verdad lo que esperas.

> **Objetivos:**
>
> - entender para qué sirve una prueba automática;
> - comprobar un valor con `assert`;
> - escribir una clase de prueba con `unittest` y leer su resultado;
> - conocer pytest y doctest;
> - escribir la prueba antes que el código, en tres pasos.

## ¿Por qué probar?

Una **prueba** (*test*) es un pequeño programa que comprueba otro programa (consulta
[el glosario](../debuter/glossaire.md#test)). Llama a tu código con valores conocidos y
compara el resultado con el que esperas.

Sin pruebas, compruebas a mano, con `print`, y vuelves a empezar después de cada
cambio. Con pruebas:

- un solo comando lo comprueba todo, en un segundo;
- si un cambio rompe algo, lo sabes enseguida;
- las pruebas muestran cómo usar tu código: son una documentación que funciona.

> **Recuerda:** una prueba responde a una pregunta sencilla: «con esta entrada, ¿obtengo
> de verdad esta salida?».

## La comprobación más sencilla: assert

La palabra clave `assert` comprueba que una condición es verdadera. Si lo es, no pasa
nada. Si no, Python lanza un `AssertionError` y detiene el programa.

```python
def doble(x):
    return x * 2


assert doble(4) == 8
assert doble(0) == 0
assert doble(-3) == -6
print("Todas las pruebas pasan.")
```

```sortie
Todas las pruebas pasan.
```

Añade un mensaje después de una coma: se muestra cuando la comprobación falla. Aquí
tienes un `doble` que se equivoca (suma 2 en lugar de multiplicar):

```python norun
def doble(x):
    return x + 2


assert doble(4) == 8, "doble(4) debería valer 8"
```

```text
Traceback (most recent call last):
  File "/home/tu/main.py", line 5, in <module>
    assert doble(4) == 8, "doble(4) debería valer 8"
           ^^^^^^^^^^^^^
AssertionError: doble(4) debería valer 8
```

`assert` basta para pequeños ensayos. Pero el programa se detiene en el primer fallo, y
no sabes si las comprobaciones siguientes pasan. Para ir más lejos, hace falta una
herramienta de pruebas de verdad.

## Tu primera prueba con unittest

El módulo **unittest** viene con Python: no hay nada que instalar. Un archivo de pruebas
sigue siempre el mismo modelo:

1. `import unittest` carga el módulo;
2. una clase de prueba hereda de `unittest.TestCase` (consulta [la herencia](heritage.md));
3. cada método cuyo nombre empieza por `test_` es una prueba;
4. `unittest.main()` lanza todas las pruebas del archivo.

```python
import unittest


def doble(x):
    return x * 2


class TestDoble(unittest.TestCase):
    def test_doble_de_4(self):
        self.assertEqual(doble(4), 8)

    def test_doble_de_cero(self):
        self.assertEqual(doble(0), 0)


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.wasSuccessful())
```

```sortie
True
```

`self.assertEqual(doble(4), 8)` se lee: «comprueba que `doble(4)` es igual a `8`».

> **Nota:** en un archivo de pruebas real, la última línea es simplemente
> `unittest.main()`. Aquí, `argv=["x"], exit=False` impide que `unittest` lea los
> argumentos del terminal y termine el programa: así puedes mostrar el resultado con
> `print`.

## Leer el resultado

`unittest` no escribe su informe en la salida normal, sino en la **salida de error**
(*stderr*): por eso no aparece en la salida de arriba. En tu terminal, ves las dos:

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.000s

OK
True
```

- Cada `.` es una prueba superada.
- `Ran 2 tests` cuenta las pruebas lanzadas; el tiempo cambia cada vez.
- `OK`: todo está en verde.

> **Consejo:** `unittest.main(verbosity=2)` muestra el nombre de cada prueba. En el
> terminal, `python3 -m unittest -v` busca y lanza todos los archivos `test*.py` de la
> carpeta.

## Leer un fallo

Cuando una comprobación falla, `unittest` muestra una `F` y explica el problema:

```python norun
import unittest


def doble(x):
    return x + 2


class TestDoble(unittest.TestCase):
    def test_doble_de_4(self):
        self.assertEqual(doble(4), 8)


unittest.main()
```

```text
F
======================================================================
FAIL: test_doble_de_4 (__main__.TestDoble.test_doble_de_4)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/tu/main.py", line 10, in test_doble_de_4
    self.assertEqual(doble(4), 8)
    ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^
AssertionError: 6 != 8

----------------------------------------------------------------------
Ran 1 test in 0.000s

FAILED (failures=1)
```

El mensaje da todo lo necesario: la prueba (`test_doble_de_4`), la línea (`10`) y la
comparación fallida (`6 != 8`: tu código dio `6`, se esperaba `8`).

- `F` (*failure*, fallo): una comprobación dio un resultado incorrecto.
- `E` (*error*): el código falló durante la prueba (un nombre desconocido, por ejemplo).

## Las aserciones

Una **aserción** es una comprobación. `unittest` ofrece muchas; aquí tienes las más
útiles.

| Aserción | Comprueba que… |
| --- | --- |
| `assertEqual(a, b)` | `a == b` |
| `assertNotEqual(a, b)` | `a != b` |
| `assertTrue(x)` / `assertFalse(x)` | `x` es verdadero / falso |
| `assertIsNone(x)` | `x is None` |
| `assertIn(x, lista)` | `x` está en la lista |
| `assertAlmostEqual(a, b)` | dos flotantes están muy cerca |
| `assertRaises(Error)` | el código lanza de verdad ese error |

```python
import unittest


class TestAserciones(unittest.TestCase):
    def test_igualdad(self):
        self.assertEqual("hola".upper(), "HOLA")
        self.assertEqual(sorted([3, 1, 2]), [1, 2, 3])

    def test_verdadero_falso(self):
        self.assertTrue(2 in [1, 2, 3])
        self.assertFalse([])

    def test_none_e_in(self):
        self.assertIsNone({"a": 1}.get("b"))
        self.assertIn("pera", ["manzana", "pera"])

    def test_flotantes(self):
        self.assertAlmostEqual(0.1 + 0.2, 0.3)


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.testsRun, "pruebas lanzadas")
print("Con éxito:", programa.result.wasSuccessful())
```

```sortie
4 pruebas lanzadas
Con éxito: True
```

> **Nota:** `0.1 + 0.2` vale `0.30000000000000004` en Python (consulta
> [los números](nombres.md)). Para comparar flotantes, es mejor `assertAlmostEqual` que
> `assertEqual`.

### Probar que se lanza un error

A veces tu código debe rechazar un valor con `raise` (consulta
[los errores](erreurs.md)). Escribe `with self.assertRaises(...)`: el bloque de debajo
debe lanzar ese error. El `as` recoge el error para comprobar su mensaje.

```python
import unittest


def retirar(saldo, cantidad):
    if cantidad < 0:
        raise ValueError("cantidad negativa")
    return saldo - cantidad


class TestRetirar(unittest.TestCase):
    def test_retirada_normal(self):
        self.assertEqual(retirar(100, 30), 70)

    def test_retirada_negativa_rechazada(self):
        with self.assertRaises(ValueError) as contexto:
            retirar(100, -5)
        self.assertEqual(str(contexto.exception), "cantidad negativa")


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.wasSuccessful())
```

```sortie
True
```

## Probar una clase con setUp

Para una clase, crea un objeto en la prueba, llama a sus métodos y después comprueba su
estado. El método `setUp` se ejecuta **antes de cada prueba**: prepara un objeto nuevo.

```python
import unittest


class Cuenta:
    def __init__(self):
        self.saldo = 0

    def depositar(self, cantidad):
        self.saldo += cantidad


class TestCuenta(unittest.TestCase):
    def setUp(self):
        self.cuenta = Cuenta()

    def test_saldo_inicial(self):
        self.assertEqual(self.cuenta.saldo, 0)

    def test_depositar_aumenta_el_saldo(self):
        self.cuenta.depositar(50)
        self.cuenta.depositar(20)
        self.assertEqual(self.cuenta.saldo, 70)


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.wasSuccessful())
```

```sortie
True
```

Cada prueba recibe su propia `self.cuenta`: el depósito de la segunda prueba no cambia
la primera.

## pytest: pruebas más cortas

Muchos proyectos de Python usan **pytest**. No forma parte de Python: se instala con pip
(consulta [organizar un proyecto](projet.md)).

```sh
python3 -m pip install pytest
```

Con pytest, una prueba es una simple función `test_…`, y la comprobación es un `assert`
normal. Ni clase, ni `self.assertEqual`.

```python norun
# test_calculo.py
import pytest


def doble(x):
    return x * 2


def test_doble_de_4():
    assert doble(4) == 8


def test_doble_de_cero():
    assert doble(0) == 0


def test_division_por_cero():
    with pytest.raises(ZeroDivisionError):
        1 / 0
```

El comando `pytest` busca todos los archivos `test_*.py` de la carpeta y los lanza:

```sh
pytest
```

```text
============================= test session starts ==============================
platform linux -- Python 3.13.16, pytest-9.1.1, pluggy-1.7.0
rootdir: /home/tu/mi_proyecto
collected 3 items

test_calculo.py ...                                                      [100%]

============================== 3 passed in 0.03s ===============================
```

Cuando una prueba falla, pytest muestra la línea y los valores. Con un `doble` que suma
2:

```text
=================================== FAILURES ===================================
_______________________________ test_doble_de_4 ________________________________

    def test_doble_de_4():
>       assert doble(4) == 8
E       assert 6 == 8
E        +  where 6 = doble(4)

test_calculo.py:10: AssertionError
=========================== short test summary info ============================
FAILED test_calculo.py::test_doble_de_4 - assert 6 == 8
FAILED test_calculo.py::test_doble_de_cero - assert 2 == 0
========================= 2 failed, 1 passed in 0.04s ==========================
```

> **Consejo:** pytest también lanza las pruebas escritas con `unittest`. Puedes empezar
> con `unittest` y pasar después a pytest sin reescribir nada.

## doctest: ejemplos que se comprueban solos

Una **docstring** es el texto entre comillas triples debajo de una función (consulta
[escribir buen Python](style.md)). Si escribes en ella ejemplos como en la consola de
Python (`>>>` y después el resultado esperado), el módulo `doctest` los comprueba.

```python
def media(notas):
    """Devuelve la media de una lista de notas.

    >>> media([10, 20])
    15.0
    >>> media([12, 13, 14])
    13.0
    """
    return sum(notas) / len(notas)


import doctest

print(doctest.testmod())
```

```sortie
TestResults(failed=0, attempted=2)
```

`attempted=2`: dos ejemplos comprobados; `failed=0`: ningún fallo. El ejemplo sirve a
la vez de documentación y de prueba.

## ¿Qué probar?

No puedes probarlo todo. Apunta a los casos que importan:

| Caso | Ejemplo para `media(notas)` |
| --- | --- |
| El caso normal | `[10, 20]` da `15.0` |
| Un solo elemento | `[12]` da `12.0` |
| El límite | una nota de `0`, una nota de `20` |
| El caso vacío | `[]`: ¿error o valor por defecto? |
| Las entradas incorrectas | una nota negativa debe rechazarse |

Algunas costumbres:

- una prueba comprueba **una sola idea**;
- un buen nombre de prueba dice lo que se comprueba: `test_retirada_negativa_rechazada`
  mejor que `test_2`;
- cuando corrijas un error, añade primero una prueba que lo muestre: no volverá.

## Escribir la prueba primero

Un método de trabajo muy extendido consiste en escribir la prueba **antes** que el
código. Se avanza en tres pasos, que se repiten: **rojo**, **verde**, **mejorar**.

Ejemplo: una función `calificacion(nota)` que devuelve `"Bien"` a partir de 14,
`"Aprobado"` a partir de 10, y `"Suspenso"` por debajo.

### Paso 1: rojo, la prueba falla

Escribe primero la prueba, como si la función ya existiera.

```python norun
import unittest


class TestCalificacion(unittest.TestCase):
    def test_calificacion_bien(self):
        self.assertEqual(calificacion(15), "Bien")


unittest.main()
```

La prueba falla, ya que `calificacion` todavía no existe:

```text
E
======================================================================
ERROR: test_calificacion_bien (__main__.TestCalificacion.test_calificacion_bien)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/tu/main.py", line 6, in test_calificacion_bien
    self.assertEqual(calificacion(15), "Bien")
                     ^^^^^^^^^^^^
NameError: name 'calificacion' is not defined. Did you mean: 'TestCalificacion'?
```

Es normal, y es útil: sabes que la prueba comprueba algo de verdad.

### Paso 2: verde, la prueba pasa

Escribe el código más sencillo que haga pasar las pruebas. Añade una prueba cada vez
para cada caso, sin olvidar el límite (`14`).

```python
import unittest


def calificacion(nota):
    if nota >= 14:
        return "Bien"
    if nota >= 10:
        return "Aprobado"
    return "Suspenso"


class TestCalificacion(unittest.TestCase):
    def test_calificacion_bien(self):
        self.assertEqual(calificacion(15), "Bien")
        self.assertEqual(calificacion(14), "Bien")

    def test_calificacion_aprobado(self):
        self.assertEqual(calificacion(10), "Aprobado")

    def test_calificacion_suspenso(self):
        self.assertEqual(calificacion(7), "Suspenso")


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.wasSuccessful())
```

```sortie
True
```

### Paso 3: mejorar el código

Ahora que las pruebas te protegen, haz el código más claro o más flexible. Aquí, los
umbrales pasan a ser una lista: añadir uno ya no necesita un nuevo `if`. Vuelve a
lanzar las pruebas: si siguen en verde, no se ha roto nada.

```python
import unittest

UMBRALES = [(14, "Bien"), (10, "Aprobado")]


def calificacion(nota):
    for umbral, nombre in UMBRALES:
        if nota >= umbral:
            return nombre
    return "Suspenso"


class TestCalificacion(unittest.TestCase):
    def test_calificacion_bien(self):
        self.assertEqual(calificacion(15), "Bien")
        self.assertEqual(calificacion(14), "Bien")

    def test_calificacion_aprobado(self):
        self.assertEqual(calificacion(10), "Aprobado")

    def test_calificacion_suspenso(self):
        self.assertEqual(calificacion(7), "Suspenso")


programa = unittest.main(argv=["x"], exit=False)
print(programa.result.wasSuccessful())
```

```sortie
True
```

> **Recuerda:** rojo (una prueba que falla), verde (el código mínimo que la hace
> pasar), mejorar (sin cambiar el comportamiento). Después se vuelve a empezar con el
> caso siguiente.

## Errores frecuentes

### Un nombre de método sin test_

`unittest` solo lanza los métodos cuyo nombre empieza por `test_`. Los demás se
ignoran, sin ningún mensaje de error.

```python norun
import unittest


class TestDoble(unittest.TestCase):
    def doble_de_cuatro(self):
        self.assertEqual(4 * 2, 8)


unittest.main()
```

```text
----------------------------------------------------------------------
Ran 0 tests in 0.000s

NO TESTS RAN
```

`Ran 0 tests` debe ponerte en alerta. Cambia el nombre del método a
`test_doble_de_cuatro`.

### Paréntesis alrededor de assert

`assert` no es una función. Con paréntesis, la condición y el mensaje forman una
**tupla**, que siempre es verdadera: la comprobación ya no sirve para nada.

```python norun
def doble(x):
    return x + 2


assert(doble(4) == 8, "doble(4) debería valer 8")
print("Fin del programa")
```

```text
/home/tu/main.py:5: SyntaxWarning: assertion is always true, perhaps remove parentheses?
  assert(doble(4) == 8, "doble(4) debería valer 8")
Fin del programa
```

El error pasa desapercibido. Escribe `assert doble(4) == 8, "doble(4) debería valer 8"`.

### Calcular el valor en lugar de pasarlo a assertRaises

El código que debe lanzar el error va **dentro de un bloque `with`**. Si no, se ejecuta
antes que `assertRaises`, y la prueba falla con un error.

```python norun
import unittest


class TestDivision(unittest.TestCase):
    def test_division_por_cero(self):
        self.assertRaises(ZeroDivisionError, 10 / 0)


unittest.main()
```

```text
ERROR: test_division_por_cero (__main__.TestDivision.test_division_por_cero)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/tu/main.py", line 6, in test_division_por_cero
    self.assertRaises(ZeroDivisionError, 10 / 0)
                                         ~~~^~~
ZeroDivisionError: division by zero
```

Escribe `with self.assertRaises(ZeroDivisionError):` y después `10 / 0` debajo.

### Comparar flotantes con assertEqual

```python norun
self.assertEqual(0.1 + 0.2, 0.3)
```

```text
AssertionError: 0.30000000000000004 != 0.3
```

Escribe `self.assertAlmostEqual(0.1 + 0.2, 0.3)`.

## Ponte a prueba

**1.** ¿Qué significan `.`, `F` y `E` en el resultado de `unittest`?

> **Respuesta**
>
> `.`: una prueba superada. `F`: un fallo, una aserción dio un resultado incorrecto.
> `E`: un error, el código falló durante la prueba.

**2.** El resumen muestra `Ran 0 tests`. ¿Cuál es la causa más probable?

> **Respuesta**
>
> Los métodos de prueba no empiezan por `test_`: `unittest` no los ve.

**3.** ¿Cuál es la diferencia principal entre una prueba `unittest` y una prueba pytest?

> **Respuesta**
>
> Con `unittest`, escribes una clase que hereda de `TestCase` y compruebas con
> `self.assertEqual`. Con pytest, basta una simple función `test_…` y un `assert`; pero
> pytest debe instalarse con pip.

## En resumen

- Una prueba comprueba automáticamente que una entrada da la salida esperada.
- `assert condicion, "mensaje"` es la comprobación más sencilla, sin paréntesis.
- Con `unittest`: una clase que hereda de `unittest.TestCase`, métodos `test_…`,
  `self.assertEqual` y `with self.assertRaises(...)`.
- pytest (hay que instalarlo) acorta las pruebas; doctest comprueba los ejemplos de las
  docstrings.
- Rojo, verde, mejorar: escribe la prueba, luego el código, y después hazlo más claro.

Siguiente: [depurar en Python](debogage.md)
