---
title: Práctica 1 - Estimación del esfuerzo
subtitle: Gestión de Proyectos
author:
  - Emilio Cobos Álvarez (70912324N)
lang: es
numbersections: true
links-as-notes: true
toc: true
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
adjunto a este informe también. Por brevedad no se incluirán los diagramas
y descripciones aquí, y haremos referencia a los números de cada caso de uso.

## Valores asignados por caso de uso

Se ha usado los razonamientos derivados de la [documentación de
IBM](https://www.ibm.com/developerworks/rational/library/edge/09/mar09/collaris_dekker/index.html)
al respecto.

Nótese por la sección *3.3.2.1. Consideraciones Previas* que en la mayoría de
los casos de uso se asume la pre-condición de que el usuario ha accedido
correctamente al sistema. Todos los casos de uso omiten un flujo de excepción si
el usuario no tiene los permisos necesarios para acceder a el mismo. No
obstante, he elegido omitir esto de la cuenta de transacciones, asumiendo que
estas excepciones son genéricas para toda la aplicación y por lo tanto no llevan
trabajo extra por cada caso de uso diferente.

Generalmente he contado exclusivamente las situaciones de excepción particulares
a un caso de uso.

 * List emergencies (5.3.1): *simple* (2 transacciones)
 * View emergency details (5.3.2): *simple* (2 transacciones). En este caso he
   contado el flujo de excepción (el usuario no está asignado a una emergencia)
   como una sola transacción.
 * Add item to inventory (5.3.3): *simple* (1 transacción). He asumido que
   hay una sola interfaz que añade el item y toda la información en un sólo
   formulario.
 * Modify inventory stock (5.3.4): *simple* (2 transacciones). El usuario debe
   seleccionar el item que quiere reponer y posteriormente ajustar el stock.
 * Mark emergency as resolved (5.3.5): *simple* (2 transacciones).
 * List centers (5.3.6): *simple* (1 transacción).
 * Comment on an emergency (5.3.7): *simple* (2 transacciones). En este caso he
   decidido contar la validación necesaria para el comentario ya que es
   específica a este caso de uso.
 * Log in into the system (5.3.8): *simple* (2 transacciones). Similarmente
   he decidido contar la validación necesaria para el comentario ya que es
   específica a este caso de uso.
 * Add attended person (5.3.9): *simple* (2 transacciones).
 * Write section of the inform (5.3.10): *simple* (3 transacciones).
 * Change dotation accountable (5.3.11): *average* (5 transacciones)
 * Assign dotation to an emergency (5.3.12): *average* (5 transacciones). He
   contado la excepción particular al seleccionar dotaciones como parte de este
   caso de uso.
 * Update user (5.3.13): *average* (4 transacciones)
 * List users (5.3.14): *simple* (1 transacción)
 * Destroy user (5.3.15): *simple* (2 transacciones)
 * Log out (5.3.16): *simple* (1 transacción)
 * Report emergency (5.3.17): *average* (5 transacciones)
 * Confirm emergency (5.3.18): *simple* (1 transacción)
 * List vehicles (5.3.19): *simple* (1 transacción)
 * Create vehicle (5.3.20): *simple* (2 transacciones)
 * Update vehicle data (5.3.21): *simple* (1 transacción)
 * Destroy vehicle (5.3.22): *simple* (1 transacción)
 * Create center (5.3.23): *simple* (2 transacciones)
 * Update center (5.3.24): *simple* (2 transacciones)
 * Destroy center (5.3.25): *simple* (2 transacciones)
 * List dotations (5.3.26): *simple* (1 transacción)
 * Create dotation (5.3.27): *simple* (3 transacciones)
 * Update dotation (5.3.28): *simple* (3 transacciones)
 * Destroy dotation (5.3.29): *simple* (3 transacciones)
 * Create user (5.3.30): *simple* (1 transacciones)
 * Check inventory (5.3.31): *average* (4 transacciones)

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

El resultado final nos sale como $3844.372$ horas de esfuerzo (al ratio por
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
