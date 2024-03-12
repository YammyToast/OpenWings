import sys
import socket
import argparse
import messages
from messages import SRV_INDEX
import re

class Client:
    def __init__(self, __username: str) -> None:
        self.uuid = None
        self.username = __username
        self.game_id = None

def try_parse_message(__msg: str):
    try:
        msg = __msg.decode("utf-8")
        reg = re.search(r"""(['"]{1}route['"]{1}[\s]*:[\s]*['"]{1}[a-zA-Z\-]+['"]{1})""", msg)
        if reg == None:
            raise Exception("Cannot find Route parameter")
        route_raw = msg[reg.span()[0]:reg.span()[1]]
        route = re.sub(
            r"[\'\"\s]",
            "",
            route_raw.split(":")[-1]
        )
        route_id = SRV_INDEX.get(route)
        if route_id == None:
            raise Exception(f"Unknown Route: {route}")
        msg = route_id(msg, route)
        return msg

    except Exception as e:
        print(e)
        print("Invalid JSON: {}".format(__msg))


def main(__client_username: str):
    try:

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((messages.HOST, messages.PORT))
            client = Client(__client_username)
            print("Sending Test Message...")
            s.sendall(b"Hello, world")
            while True:
                data = s.recv(1024)
                print(f"Received {data!r}")
                msg = try_parse_message(data)
                print("MESSAGE_ROUTE:", msg.route)
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
