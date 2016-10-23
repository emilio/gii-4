---
title: La protección jurídica de las Bases de Datos.
subtitle: Aspectos legales y profesionales de la informática - 2016
author:
  - Emilio Cobos Álvarez (70912324N)
# numbersections: true
toc: false
abstract:
  - La LPI protege la estructura de la base de datos, siempre que reúna el
    requisito de originalidad, y los contenidos siempre que conlleven una
    *inversión sustancial*. ¿Se puede hacer uso de la LPI para evitar
    recolección de datos web?
---

\clearpage

# Introducción

En este papel me gustaría discutir un caso relativamente particular, y que está
en boga últimamente por el alza de determinado tipo de negocios on-line, como
los comparadores (de seguros, de vuelos, ...).

La mayoría, si no la totalidad de este tipo de páginas usa *web scraping* (es
decir, extraer datos de una página web) para conseguir la información que
posteriormente comparan, con o sin ánimo de lucro.

El [artículo 12][art-12] de la Ley de Protección intelectual indica lo
siguiente:

> También son objeto de propiedad intelectual, en los términos del Libro I de la
> presente Ley, las colecciones de obras ajenas, de datos o de otros elementos
> independientes como las antologías y las bases de datos que por la selección
> o disposición de sus contenidos constituyan creaciones intelectuales, sin
> perjuicio, en su caso, de los derechos que pudieran subsistir sobre dichos
> contenidos.

Por otra parte, clarifica:

> La protección reconocida en el presente artículo a estas colecciones se
> refiere únicamente a su estructura en cuanto forma de expresión de la
> selección o disposición de sus contenidos, no siendo extensiva a éstos.

Esto implica que es la estructura de la Base de Datos, y no el contenido de la
misma, lo que está protegido por la LPI.

Por otra parte, también existe el derecho *sui géneris* sobre la base de datos,
especificado por el [artículo 133][art-133] de la misma ley.

Este artículo dice:

> El derecho "sui generis" sobre una base de datos protege la inversión
> sustancial, evaluada cualitativa o cuantitativamente, que realiza su
> fabricante ya sea de medios financieros, empleo de tiempo, esfuerzo, energía
> u otros de similar naturaleza, para la obtención, verificación o presentación
> de su contenido.
>
> Mediante el derecho al que se refiere el párrafo anterior, **el fabricante de
> una base de datos, [...] puede prohibir la extracción y/o reutilización de la
> totalidad o de una parte sustancial del contenido de ésta**, evaluada
> cualitativa o cuantitativamente, siempre que la obtención, la verificación
> o la presentación de dicho contenido representen una inversión sustancial
> desde el punto de vista cuantitativo o cualitativo. Este derecho podrá
> transferirse, cederse o darse en licencia contractual.

Lo cual **protege el contenido** siempre que haya una "inversión sustancial".

Dejando de lado todo lo que pienso hacia este tipo de párrafos en la
legislación que especifican "cualitativa o cuantitativamente" (siendo esta una
base terrible para la ambigüedad jurídica), esto parece indicar que los datos en
los que uno ha invertido "sustancialmente" sí están protegidos.

# ¿Es realizar *web scraping* legal?

Esto nos lleva a una pregunta clara:

> ¿Si yo tengo una colección de datos en la que he invertido "sustancialmente",
> están en la web disponibles sin ningún tipo de control de acceso, y alguien
> extrae esos datos, está haciendo algo legal?

La legislación española, es, para variar, ambigua y propensa a error,
especialmente en este caso particular.

El artículo 113 menciona que se puede prohibir la extracción y reutilización,
pero no se especifica ninguna forma ni procedimiento para realizarlo. El más
interesado de los lectores podrá decir que con especificar eso de forma escrita
ya es suficiente. No obstante, no podemos olvidar que no hay ningún control de
entrada a los datos dado al entorno (la web). Por lo tanto el usuario nunca está
obligado a leer esa cláusula.

Obviamente, prevenir el acceso inicial hasta que la cláusula no ha sido
explícitamente aceptada suena bien desde un punto de vista legal, pero no es
práctico.

Personalmente, parece que realizar *web scraping* sin ánimo de lucro no debería
ni de lejos ser un delito. En concreto, opino lo siguiente:

> Dada la naturaleza abierta y distribuida de la web, los datos presentes en
> ella de forma pública que no sean objeto por si mismo de propiedad intelectual
> se deberían de considerar dominio público si no hay objetivo de lucrarse.

Acceder a información pública y tener derecho a tratarla y distribuirla sin
lucro de por medio me parece algo fundamental, y cualquier otra cosa no sería
menos que capar la originalidad de la web.

Esto es algo que, al menos desde mi punto de vista, es ya cierto una realidad de
facto.

**Una vez entra el lucro de por medio, la historia es otra.** De hecho, de
primeras, puedo entender que alguien se sienta indignado porque alguien use tus
datos para hacer dinero, especialmente si es competencia.

Pero tras algo de reflexión: ¿No es esto lo que hace Google con cada búsqueda en
internet? Google utiliza los datos públicos en la web de forma lucrativa (para
vender anuncios). Y en este tipo de búsquedas, los anuncios también son
competencia directa del poseedor original de los datos en muchas ocasiones.

Desde luego, no quiero decir que me parezca bien este tipo de uso, pero sólo
pretendía hacer una reflexión en el sentido de que esto pasa en una gran
cantidad de tráfico en la web, y normalmente a nadie parece importarle
demasiado.

[art-12]: http://civil.udg.es/normacivil/estatal/reals/Lpi.html#a012
[art-133]: http://civil.udg.es/normacivil/estatal/reals/Lpi.html#a133
