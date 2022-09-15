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
#### Royner:



## Lecciones Aprendidas:

En esta tarea se aprendió muchísimo sobre threads y más que todo sobre el funcionamiento del protocolo HTTP y sus propiedades, el hacer que varios programas funcionen en conjunto también fue un reto interesante. Creemos que es una tarea bastante pesada sin embargo que deja muchas lecciones interesantes como el manejo de módulos en diferentes lenguajes de programación. Es una tarea que deja bastante que decir en un curriculum por lo que si se desea destacar recomendamos hacerla.



## Bibliografía:

[1] 	
