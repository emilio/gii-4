---
title: Práctica 1 - Estimación del esfuerzo
subtitle: Gestión de Proyectos
author:
  - Emilio Cobos Álvarez (70912324N)
lang: es
numbersections: true
links-as-notes: true
toc: true
header-includes:
  - \usepackage{pdfpages}
  - \usepackage{environ}
  - \usepackage{tabularx}
  - \usepackage{keycommand}
  - \usepackage{inc/defs}
---

\clearpage

# Descripción del sistema

El sistema consiste en una aplicación web utilizada para gestionar las
emergencias del servicio del 112.

La aplicación tiene funcionalidad diversa entre las que se encuentran:

 * Simple administración de acceso y usuarios.
 * Gestión del inventario de un centro de emergencias.
 * Gestión de dotaciones asignadas a una emergencia.
 * Gestión de emergencias y recopilación de informes relativos a una emergencia.

Hay diversos tipos de usuarios que pueden acceder a la aplicación: Personal de
emergencias, operadores de teléfono, responsables del centro, administración...

# Modelos de casos de uso.

Una descripción más detallada de la aplicación y un modelo de casos de uso se
encuentra en el
[documento](https://github.com/emilio/softeng-2015/raw/master/final.pdf)
realizado para la asignatura de Ingeniería de Software, que debería de estar
adjunto a este informe también.

## Paquetes que componen el sistema.

La aplicación se divide en dos paquetes, el paquete `Emergency`, y el paquete
`Sessions`.

El paquete `Emergency`, recoge todos los casos de uso en los que actúa un
usuario base de la aplicación (actor `User`), salvo el control de sesiones, que
se lleva a cabo en el paquete `Sessions`.

## Diagramas de los casos de uso del sistema

\includepdf[pages=70-78, nup=1x2, frame, landscape]{./practica2015.pdf}

## Descripción de los actores del sistema

\input{inc/actors}

## Valores asignados por caso de uso

Se ha usado los razonamientos derivados de la [documentación de
IBM](https://www.ibm.com/developerworks/rational/library/edge/09/mar09/collaris_dekker/index.html)
al respecto.

Nótese que en la mayoría de los casos de uso se asume la pre-condición de que el
usuario ha accedido correctamente al sistema. Todos los casos de uso omiten un
flujo de excepción si el usuario no tiene los permisos necesarios para acceder
a el mismo. No obstante, he elegido omitir esto de la cuenta de transacciones,
asumiendo que estas excepciones son genéricas para toda la aplicación y por lo
tanto no llevan trabajo extra por cada caso de uso diferente.

Generalmente he contado exclusivamente las situaciones de excepción particulares
a un caso de uso.

\input{inc/use-cases.tex}

# Factores técnicos / del entorno

## Complejidad técnica

 * Distributed system: He puesto un 4 aquí ya que el sistema tiene que ser
   accesible y estar disponible en todo momento, al ser un sistema de
   emergencias, por lo que no se podría permitir un sólo punto de fallo y por lo
   tanto el sistema tendría que ser redundante y distribuido.
 * Response / Throughput performance objectives: He puesto un 5 aquí por razones
   similares. El sistema debe de estar funcional en todo momento, y debería
   aguantar tanta carga como hiciera falta para que incluso en la peor situación
   posible sea usable.
 * End-user efficiency: 4 por razones similares. Los operadores tienen que poder
   utilizarlo en situaciones de emergencia / gran estrés. He quitado un punto
   porque hay partes de la aplicación (administración de centros, etc) que se
   pueden tomar más libertad aquí al ser tareas menos urgentes.
 * Complex internal processing: He puesto un 3 porque el funcionamiento de la
   aplicación en general es relativamente sencillo.
 * Reusable code: 3, porque reusar código, a pesar de ser ideal, no es una gran
   prioridad, aunque hace mantener y mejorar la aplicación más fácil.
 * Easy to install: He puesto un 3, lo que puede parecer contradictorio al ser
   una aplicación web. Pero dado el tipo de datos y uso que se le espera, creo
   que hacerla instalable (via ServiceWorkers, etc) para que el personal de
   emergencia pueda modificar reportes y similar sin conexión a internet.
 * Portable: He puesto un 0. Usando tecnologías web estándar es fácil hacer
   código portable que funcione en todas las plataformas y navegadores.
 * Easy to change: He puesto un 1 ya que haciéndolo una aplicación web realizar
   cambios no debería de ser una fuente de complejidad.
 * Concurrent: He puesto un 3 porque necesita manejar un gran abanico de
   usuarios y personal actuando a la vez.
 * Includes security features: 5, esta aplicación trata con datos personales
   y los transmite via internet, seguridad es una prioridad top.
 * Third party access: 0, esta aplicación sólo se puede usar con acceso previo.
 * Special user training facilities required: 3, porque manejar la aplicación
   correcta y eficientemente es una prioridad para los operadores y personal de
   emergencia, por lo que algún tipo de entrenamiento podría ser beneficioso.

## Factores del entorno

Nótese que la constante por la que se multiplican estos factores es negativa, lo
que me ha hecho asignar valores más altos si el conocimiento ayuda al desarrollo
de la aplicación.

 * Familiar with Rational unified process: 4, hemos hecho un análisis extensivo
   y esperamos que eso ayude a la hora de desarrollar la aplicación.

 * Application experience: He puesto un 3, aunque tengo amplia experiencia en el
   desarrollo de aplicaciones web, estoy algo oxidado y requiere trabajo
   analizar el ecosistema actual y elegir las mejores herramientas para un
   determinado trabajo.

 * Object oriented experience: 5, considero que como lead analyst tengo bastante
   experiencia en el desarrollo orientado a objetos, con amplia experiencia en
   la industria.

 * Lead analyst capability: 5, hay que ser positivos :-)

 * Motivation: 2, porque sinceramente desarrollar aplicaciones no me parece
   particularmente interesante, prefiero trabajar con software de más bajo
   nivel.

 * Part time workers: 3, esperamos poder contratar a gente para reforzar el
   equipo.

 * Stable requirements: 4, los requisitos estaban bastante claros al comienzo
   y no esperamos grandes cambios.

 * Difficult programming language: 5, HTML/CSS/JS son lenguajes que están en
   todos lados y no es difícil encontrar desarrolladores que los conozcan.

# Interpretación de los resultados y conclusiones

El resultado final nos sale como $3879.804$ horas de esfuerzo (al ratio por
defecto de 20h por UCP).

Haciendo algunas cuentas sencillas (160h al mes por un trabajador a tiempo
completo), nos sale aproximadamente que un equipo de 4 personas a tiempo
completo podrían hacerlo en medio año.

Dicho objetivo parece en el campo de lo posible (con la motivación, experiencia
y financiación adecuadas), aunque es posible que las características de
seguridad / estabilidad necesarias para un proyecto de estas características no
hayan sido contadas particularmente bien por el modelo que hemos usado,
y necesitaran algo más de tiempo / pentesting / etc.

Similarmente, creo que la cuenta, aunque razonable, puede sufrir bastante de
casos de uso sobre-simplificados que hicimos en su momento, por lo que muchos de
los casos de uso clasificados como "simple" podrían llegar a no serlo cuando se
llega al punto de testear e implementar.
