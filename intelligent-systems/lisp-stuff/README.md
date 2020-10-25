# Algoritmo de unificación en LISP

Esta es una implementación del algoritmo de unificación en LISP.

La implementación está basada en el libro:

> Artificial Intelligence:
>   Structures and strategies for complex problem solving

por George F. Luger and William A. Stubblefield.

La razón por la que he elegido esta implementación en vez de la propuesta en
clase es porque esta no necesita implementar la composición de sustituciones,
que es compleja, y me parece mucho más elegante. Lo que hace para no tener que
componer sustituciones es básicamente unificar una de las partes en base a la
otra, eliminando esa necesidad.

Esta implementación devuelve un mapa de variable a valor de esa variable. Los
valores de una variable pueden ser otras variables a su vez. Por ejemplo,
`(unify '(? x) '(? y))` devuelve `x: '(? y)`.

Es posible que en otra parte de la expresión consigamos resolver `y: 4`, pero
para llegar a `x: 4` sólo tendríamos que modificar el mapa aplicando las
substituciones a los valores (que es aplicar la función `match`).
