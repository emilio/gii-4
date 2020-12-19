---
title: Práctica 5 - Vulnerabilidades
subtitle: Seguridad de Sistemas Informáticos
author:
  - Emilio Cobos Álvarez (70912324N)
  - Juan Carlos Martín García (70882826T)
lang: es
numbersections: true
links-as-notes: true
toc: true
header-includes:
  # Prevent images from floating and getting reordered with the text
  - \usepackage{float}
  - \floatplacement{figure}{H}
  # Make sections start in a new page.
  - \let\Oldsection\section
  - \renewcommand{\section}{\clearpage\Oldsection}
---

# Webs propuestas

## HispaSec

HispaSec es un laboratorio de seguridad informática nacida como tal en el año
2000, como respuesta a la demanda de servicios originados por el éxito de
[una-al-dia](https://unaaldia.hispasec.com), un blog orientado a la seguridad
informática.

Provee servicios de seguridad como auditorías, servicio anti-fraude,
consultoría, formación...

## NIST

La base de datos de vulnerabilidades del NIST está mantenida por el gobierno
estadounidense. Es una base de datos bastante exhaustiva, con varias referencias
a advisories, etc externos.

Por ejemplo, la página para un
[CVE](https://nvd.nist.gov/vuln/detail/CVE-2019-17016) arreglado por un
servidor contiene una larga lista de referencias desde el [advisory
original](https://www.mozilla.org/en-US/security/advisories/mfsa2020-02/)
a otros advisories hechos por distribuciones y terceras partes. Contiene el
análisis original, una puntuación del CVE (6.1 / 10), un link al tipo de
vulnerabilidad (CWE-79, XSS)...

## Incibe

Incibe es el instituto nacional de seguridad español. Tiene una [sección de
avisos de seguridad](https://www.incibe.es/alerta-temprana/avisos-seguridad),
incluyendo un [boletín](https://www.incibe.es/newsletter/subscriptions).

Las páginas de cada entrada (por ejemplo, hemos visitado [una de
SAP](https://www.incibe-cert.es/alerta-temprana/avisos-seguridad/actualizacion-seguridad-sap-diciembre-2020))
contienen detalles de la vulnerabilidad, soluciones, referencias externas, etc.

# Comentario de noticia de HispaSec

La noticia que hemos decidido comentar es
[*Detectadas extensiones maliciosas para Chrome y Microsoft Edge con más de
3 millones de instalaciones*](https://unaaldia.hispasec.com/2020/12/detectadas-extensiones-maliciosas-para-chrome-y-microsoft-edge-con-mas-de-3-millones-de-instalaciones.html)

Mantener una Store de extensiones de un navegador es difícil. Los navegadores
tienen diferentes políticas sobre qué es aceptable tener publicado en la tienda
oficial.

Las extensiones de un navegador son extremadamente poderosas, y pueden desde
almacenar todo el historial de búsqueda de un usuario, hasta redirigirlo
a sitios maliciosos.

En este artículo se describe un set de extensiones que de diferentes maneras
espían al usuario o lo redirigen a sitios de phising.

Las tienda de extensiones de Chrome depende de reportes de usuarios para
mantener extensiones seguras. En Firefox, por ejemplo, [las
extensiones son revisadas tanto automática como
manualmente](https://extensionworkshop.com/documentation/publish/what-does-review-rejection-mean-to-users/)
y los criterios para pasar la review son mucho más estrictos que la Chrom Web
Store en términos de qué tipo de datos pueden ser almacenados y enviados por la
extensión.

En este caso, una vez reportado por el equipo de investigadores, las extensiones
fueron eliminadas rápidamente.

# CVE / CVSS

Los CVE son unos identificadores asignados a vulnerabilidades de seguridad
públicas, asociados con los datos de esas vulnerabilidades como una descripción,
referencias externas, fecha.

CVSS es un sistema de puntuación
[estándar](https://www.first.org/cvss/specification-document) para
vulnerabilidades. Cuanto mayor sea la puntuación, más crítica es la
vulnerabilidad. El sistema se basa en varias métricas como "vector de ataque",
"complejidad del ataque", "privilegios requeridos", "interacción del usuario",
el compromiso a la integridad / confidencialidad / disponibilidad...

FIRST provee una [calculadora](https://www.first.org/cvss/calculator) para el
marcador, que se puede representar como una cadena de caracteres como:
`CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:L/I:H/A:H`.

# Resumen de un CVE

Hemos buscado en la base de datos del NIST una de las vulnerabilidades que
Emilio ha arreglado, como
[CVE-2019-17016](https://nvd.nist.gov/vuln/detail/CVE-2019-17016), por lo que
podemos proveer una descripción detallada.

El report es [bug
1599181](https://bugzilla.mozilla.org/show_bug.cgi?id=1599181), que afecta
a versiones de Firefox desde la 57 a la 68.3/72.

El error es un fallo al no escapar correctamente las urls de las reglas
`@namespace` en CSS. El fallo parece relativamente inofensivo (y Emilio había
encontrado el error cuando haciendo un
[refactoring](https://github.com/servo/servo/pull/17154)).

Pero, ese código se usa para sanear los datos del portapapeles para evitar pegar
cosas potencialmente problemáticas en otras páginas. Aprovechando ese bypass, el
atacante consigue hacer exfiltrar cosas como un token CSRF de GMail, si consigue
que un usuario copie datos de una página maliciosa y los pegue en el editor de
GMail.

El fallo fue arreglado, asignado un CVE y una versión de Firefox distribuida en
cuestión de días. El código del exploit es público y está [adjunto al
bug](https://bug1599181.bmoattachments.org/attachment.cgi?id=9111317), y un post
de los investigadores detallando esta y otras vulnerabilidades en otros
navegadores está disponible [en su
web](https://research.securitum.com/the-curious-case-of-copy-paste).

Fue asignado una [puntuación de
6.1](https://nvd.nist.gov/vuln-metrics/cvss/v3-calculator?name=CVE-2019-17016&vector=AV:N/AC:L/PR:N/UI:R/S:C/C:L/I:L/A:N&version=3.1&source=NIST),
porque el compromiso es relativamente menor y el usuario necesita interactuar
con la página.

# Otras webs

## MITRE

La gran web que falta en esta lista IMO es la web del
[MITRE](https://www.mitre.org), que mantienen el sistema
[CVE](https://cve.mitre.org), que se usa para asignar identificadores
a las vulnerabilidades, y que está sincronizado con la base de datos del NIST.

MITRE también mantiene la lista de [CWEs](http://cwe.mitre.org/data/index.html)
(Common Weakness Enumeration), muy útil para clasificar vulnerabilidades.

Desde la [página de solicitud de
CVEs](http://cve.mitre.org/cve/request_id.html), podemos ver muchas otras webs
de vendors específicos donde se realizan advisories.

En particular, los CNA (CVE Numbering Authority) listados en esa página tienen
una página de advisories. Ejemplos:

 * Apple: https://support.apple.com/en-us/HT201222
 * Microsoft: https://www.microsoft.com/en-us/msrc/technical-security-notifications
 * Google: https://www.google.com/about/appsecurity/research/
 * Mozilla: https://www.mozilla.org/en-US/security/advisories
 * etc.

Casi todos tienen listas de correo o boletines asociados si quieres mantenerte
al día de la seguridad de los productos de dichas empresas.

## Google Project Zero

El [blog de Google Project Zero](https://googleprojectzero.blogspot.com) es muy
útil para aprender, ya que contiene descripciones muy detalladas de
vulnerabilidades encontradas por el grupo.

# Describir el proyecto OWASP y el subproyecto OWASP Top Ten

OWASP (*Open Web Application Security Project*) es una fundación sin ánimo de
lucro que trabaja para mejorar la seguridad del software en general, y de las
aplicaciones web en particular.

## OWASP Top Ten

El OWASP Top 10 es un documento para desarrolladores. Presenta los diez riesgos
más críticos para las aplicaciones web.

El documento se aloja [en
GitHub](https://github.com/OWASP/www-project-top-ten/blob/master/index.md) y se
puede ver en [owasp.org](https://owasp.org/www-project-top-ten/).

Por cada tipo de vulnerabilidad tiene una descripción más detallada describiendo
cómo descubrir si la aplicación es vulnerable y cómo prevenir los diferentes
riesgos.
