# echo-client.py
import time
import socket

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 25373  # The port used by the server

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    while True:
        s.connect((HOST, PORT))
        s.sendall(b"Hello, world")
        data = s.recv(1024)
        time.sleep(1000)
        print(data)