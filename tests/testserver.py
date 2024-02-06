import socket

# Create a socket
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Connect to the remote host and port
sock.connect((remote_host, remote_port))

# Send a request to the host
sock.send("Why don't you call me any more?\r\n")

# Get the host's response, no more than, say, 1,024 bytes
response_data = sock.recv(1024)

# Terminate
sock.close()