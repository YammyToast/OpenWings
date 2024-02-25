# echo-client.py
import sys
import socket
import argparse
import messages


class Client:
    def __init__(self, __username: str) -> None:
        self.uuid = None
        self.username = __username


def main(__client_username: str):
    try:

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((messages.HOST, messages.PORT))
            client = Client(__client_username)
            print("Sending Test Message...")
            s.sendall(b"Hello, world")
            data = s.recv(1024)
            print(f"Received {data!r}")
            while True:

                break
    except Exception as e:
        raise e


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="OpenWings Test Client")
    parser.add_argument(
        "--username",
        "-u",
        metavar="U",
        action="store",
        default="test_user",
        type=str,
        help="username for the client",
    )
    args = parser.parse_args()
    main(args.username)
