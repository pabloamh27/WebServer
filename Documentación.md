# Tarea 3 
- Estudiantes: Pablo Alberto Muñoz Hidalgo, Royner Miranda Segura
- Profesor: Kevin Moraga García
- Curso y Universidad: ITCR Sistemas Operativos
- Año: 2022

# Introducción

Se debe de crear un HTTP server el cual implementa la técnica llamada pre thread en el lenguaje de programación Rust, este debe ser capaz de recibir usuarios utilizando POST, GET, HEAD, PUT, DELETE. También se debe crear un cliente HTTP el cual permita descargar un binario a través de una lista de comandos en los parámetros o bien interactuar con el servidor HTTP como cualquier otro cliente HTTP. Por último se debe crear un StressCMD que su
objetivo de unir el HTTPClient y el StressCMD es saturar los WebServers hasta que estos se queden sin posibilidad de atender otro cliente más. En el lenguaje Python.


# Ambiente de desarrollo
Se estará utilizando Ubuntu 20.04.4 LTS y como IDE se utilizará Visual Studio Code. Para probar la pagina web se usa Firefox 103.0.1 (64-bit). Además de un repositorio en github.

## Estructuras de datos usadas y funciones:
### Estructuras:

- Workers
- ThreadPool

### Funciones:



## Instrucciones para ejecutar el programa:

1. Comenzar el web_server ejecutandolo desde consola

2. Despues es posible enviar solicitudes por medio del HTTPClient

3. En caso de que se quiera botar el server usar StressCMD

4. Disfrutar :)

   Se deben usar las siguientes estructuras para cada ejecutable.

   WebServer: "./web_server prethread-WebServer -n <cantidad-hilos> -w <HTTP-root> -p <port>"

   HTTPclient: "./httpclient HTTPclient -h <host-a-conectar> [<lista-de-comandos-a-ejecutar>]"

   StressCMD: "python3 stresscmd stress -n <cantidad-hilos> HTTPclient <parametros del cliente>"

   

## Actividades realizadas por estudiante:
|Fecha|Hora de Inicio|Hora de Finalización|Actividad realizada|Estudiante|
|-----|----------|-------|-------|-----|
|09/09/2022|3:00 PM|10:00 PM| Investigación y creación del git                             | Pablo Muñoz |
|10/09/2022| 8:00 AM        | 12:00 PM             | Preparación del ambiente de desarrollo y más investigación   | Pablo Muñoz |
| 10/09/2022 | 3:00 PM        | 12:00 AM             | Primeras implementaciones del WebServer                      | Pablo Muñoz |
| 11/09/2022 | 7:00 AM        | 12:00 PM             | Implementación de nuevos html y comenzar el StressCMD        | Pablo Muñoz |
| 11/09/2022 | 1:00 PM        | 6:00 PM              | Creación del Client                                          | Pablo Muñoz |
| 11/09/2022 | 10:00 PM       | 12:00 AM             | Contador de threads y mensaje de error                       | Pablo Muñoz |
| 12/09/2022 | 2:00 PM        | 6:00 PM              | StressCMD funcional, Client Funcional solo con GET y documentación | Pablo Muñoz |
| 14/09/2022 | 6:00 PM | 12:00 PM | Documentación, root como parámetro y programas funcionales desde consola | Pablo Muñoz |
| /09/2022   |                |                      |                                                              |             |
| /09/2022   |                |                      |                                                              |             |
| /09/2022   |                |                      |                                                              |             |
| /09/2022   |                |                      |                                                              |             |

### Horas totales: 44 (Pablo) + 00 (Royner) =

## Autoevaluación:
### Estado del programa

El programa funciona perfectamente exceptuando la recepción de diferentes protocolos como Telnet o SNMP. Además no se pueden descargar documentos del servidor HTTP.

### Problemas encontrados y limitaciones adicionales

Se encontraron problemas que no pudieron ser solucionados, en este caso no se logró que el servidor provea el servicio de descarga de archivos grandes, tampoco el parseo con otros protocolos diferentes a HTTP.


### Evaluación
|WebServer|Implementación de protocolos| HTTPclient en Rust |Stress-Client|Documentación|Kick-off|
|-----|------|------|------|------|------|
|35/40|7/10|15/15|15/15|20/20|5/5|
### Autoevaluación:
|Aprendizaje de pthreads|Aprendizaje de comunicación entre procesos| Aprendizaje de sockets |Estudiante|
|-----|------|------|------|
|5| 5                                          | 5                      | Pablo Muñoz    |
||                                            |                        | Royner Miranda |



### Reporte de commits:
#### Pablo:

commit 7304641038e22d4090aba33468179515613ea08c (HEAD -> main, origin/main, origin/HEAD)
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Wed Sep 14 23:09:55 2022 -0600

    Comentarios y documentacion actualizada

commit 92ae5c0c76d7fac0dcaaa60b2346288af56f1e02
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Wed Sep 14 22:48:03 2022 -0600

    Código comentado

commit b3d8c79a8db64d380762a7f836d3b780b7f4ff98
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Wed Sep 14 22:33:29 2022 -0600

    Rutas como parametros y comandos en consola al 100%

commit 7e82ef0fe1f4fb6cfc3caeb75b30002215bdc3a7
Author: Royner39 <roynerarturo39@gmail.com>
Date:   Tue Sep 13 19:48:02 2022 -0600

    Implementación de métodos POST, PUT, DELETE

commit 0accac8fb477ccd7e130e5323bcc38be5f2f50cd
Author: Royner39 <roynerarturo39@gmail.com>
Date:   Tue Sep 13 08:55:31 2022 -0600

    Caso Post
    
    Falta probar

commit f9acb8af15216f105f3498d29a871f07c9aaa3d0
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Mon Sep 12 18:22:17 2022 -0600

    Documentación

commit 68a4d74cc00ab0bec77f8c9b1d0a4fdb0b49707c
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Mon Sep 12 18:15:40 2022 -0600

    Client y StressCMD funcional, solo método GET

commit d40b51d5c927b1131c9be074d6da9661607b0fca
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Sun Sep 11 23:08:03 2022 -0600

    Actualizacion stressCMD, Webserver funcional, httpclient actualizado

commit 71f145e1979550381b79b79bb0631d093ebb9fa7
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Sun Sep 11 15:34:44 2022 -0600

    StressCMD funcional, WebServer se cae y httpclient progreso

commit 653cc607e4c44cef0ffc95dc98899f0282322418
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Sat Sep 10 16:46:56 2022 -0600

    WebServer funcional

commit 784ca2e2367a01223dd3685275a4fed42ef0e53a
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Sat Sep 10 15:02:06 2022 -0600

    Create README.md

commit 492394198a0b347be09e286f43d89a06c493dcec
Author: Royner39 <roynerarturo39@gmail.com>
Date:   Sat Sep 10 14:40:26 2022 -0600

    Implementación de 404

commit 8f37ba844660919fb581cbad79ff4747565f9097
Author: Royner39 <roynerarturo39@gmail.com>
Date:   Sat Sep 10 14:14:45 2022 -0600

    Web Server básico

commit a641ff9042224ef229d0f4972cffb970c16e0c86
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Tue Sep 6 14:03:21 2022 -0600

    REFERENCIAS!!!!!!!!!!!!!

commit f87c05aa7059b11ec4c298730c710f38e33cd876
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Tue Sep 6 13:37:51 2022 -0600

    Más referencias

commit fafbd3459997d8bdf091d2052d6c313018607e25
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Tue Sep 6 13:34:20 2022 -0600

    Referencias útiles para la tarea

commit 760bbf2e83406c8645afc66d344f37c3f18f0b53
Author: Pablo Munoz Hidalgo <53487847+Litecore50@users.noreply.github.com>
Date:   Tue Sep 6 13:24:36 2022 -0600

    Create test
#### Royner:



## Lecciones Aprendidas:

En esta tarea se aprendió muchísimo sobre threads y más que todo sobre el funcionamiento del protocolo HTTP y sus propiedades, el hacer que varios programas funcionen en conjunto también fue un reto interesante. Creemos que es una tarea bastante pesada sin embargo que deja muchas lecciones interesantes como el manejo de módulos en diferentes lenguajes de programación. Es una tarea que deja bastante que decir en un curriculum por lo que si se desea destacar recomendamos hacerla.



## Bibliografía:

[1] 	
