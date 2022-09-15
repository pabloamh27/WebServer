from ast import arguments
import threading, os, time, getopt, sys
import numpy as np

controlFlagThreads = True

def exitStress():
    print("Apagando stress...")
    global controlFlagThreads
    controlFlagThreads = False
    
def stress(client_arguments, thread):
    while(controlFlagThreads):
        os.system("cd /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/HTTPclient/target/debug && ./web_server " + client_arguments)
        time.sleep(0.5)
    print("Cerrando hilo " + str(thread) + "...")

def initializeStress(threadNumber, client_arguments):
    threads = list()
    for i in range(int(threadNumber)):
        thread = threading.Thread(target=stress, args=(client_arguments, i,))
        threads.append(thread)
        thread.start()
    exitCondition = ""
    while(exitCondition != "exit"):
        exitCondition = input()
    exitStress()
    
def main():
    print("Bienvenido al stressCMD:\n",
          "Para iniciar el stress ingrese el comando:\n",
          "stress -n <numero de hilos> HTTPclient <argumentos del cliente>\n",)
    #entrada = input("Ingrese el comando: ")
    #GET METHOD
    #entrada = "stress -n 10 HTTPclient http://127.0.0.1:8080 GET /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/web_server/src/resources/hello.html"
    #POST METHOD
    entrada = "stress -n 10 HTTPclient http://127.0.0.1:8080 POST /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/post.html"
    #DELETE METHOD
    #entrada = "stress -n 10 HTTPclient http://127.0.0.1:8080 DELETE /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/web_server/src/resources/post.html"
    #PUT METHOD
    #entrada = "stress -n 10 HTTPclient http://127.0.0.1:8080 PUT /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/post.html"
    input_splitted = entrada.split(" ")
    print(input_splitted)
    print(type(int(input_splitted[2])))
    if(input_splitted[0] != "stress"):
        sys.exit("Error: Debe comenzar con stress")
    elif (input_splitted[1] != "-n"):
        sys.exit("Error: Debe ingresar -n")
    elif (type(int(input_splitted[2])) != type(1)):
        sys.exit("Error: Debe ingresar un numero")
    elif (input_splitted[3] != "HTTPclient"):
        sys.exit("Error: Debe ingresar HTTPclient")
    else:
        client_arguments = ""
        for i in range(4, len(input_splitted)):
            if client_arguments == "":
                client_arguments = input_splitted[i]
            else:
                client_arguments = client_arguments + " " + input_splitted[i]
        print(client_arguments)
        initializeStress(int(input_splitted[2]), client_arguments)
main()