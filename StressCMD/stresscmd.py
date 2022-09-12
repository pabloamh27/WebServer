import threading, os, time, getopt, sys

controlFlagThreads = True

def exitStress():
    print("Apagando stress...")
    global controlFlagThreads
    controlFlagThreads = False
    
def stress(client_arguments, thread):
    while(controlFlagThreads):
        os.system("./httpclient -h ", client_arguments)
        time.sleep(0.5)
    print("Cerrando hilo " + str(thread) + "...")

def initializeStress(threads, client_arguments):
    threads = list()
    for i in range(int(threads)):
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
    #entrada = input()
    entrada = "stress -n 2 HTTPclient 127.0.0.1:8080"
    input_splitted = entrada.split(" ")
    print(input_splitted)
    print(type(int(input_splitted[2])))
    print(type(int))
    if(input_splitted[0] != "stress"):
        sys.exit("Error: Debe comenzar con stress")
    elif (input_splitted[1] != "-n"):
        sys.exit("Error: Debe ingresar -n")
    elif (type(int(input_splitted[2])) != type(1)):
        sys.exit("Error: Debe ingresar un numero")
    elif (input_splitted[3] != "HTTPclient"):
        sys.exit("Error: Debe ingresar HTTPclient")
    else:
        initializeStress(input_splitted[2], input_splitted[4])
main()