from abc import ABC, abstractmethod
import time
import re

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 25373  # The port used by the server


class Header:
    def __init__(self, __game_id: int, __timestamp=None) -> None:
        self.game_id = __game_id
        if __timestamp == None:
            self.timestamp = time.time()
        else:
            self.timestamp = __timestamp

    def serialize(self) -> str:
        print("serialize header")


class Body(ABC):
    route: str

    @abstractmethod
    def serialize(self) -> str:
        pass


class Message:
    def __init__(self, __route: str, __game_id: int, __body: Body) -> None:
        self.header = Header(__game_id)
        self.body = __body
        self.route = __route
        pass

    def serialize(self) -> str:
        buf = f'{{ "route": {self.route}, "header": {{{self.header}}}, "body": {{{self.body}}} }}'
        return buf.encode("utf-8")

    @staticmethod
    def deserialize(__route, __header, __body):
        msg = Message(__route, 0, __body)
        msg.header = __header
        return msg


def get_header(__data) -> Header:
    try:
        # HEADER
        result = re.search(r"""(['"]header['"][\s]*:[\s]*{['":,\s\w]+})""", __data)
        if result == None:
            raise Exception("Could not find header attribute in message.")
        raw_header = __data[result.span()[0] : result.span()[1]]

        # GAME ID
        result_id = re.search(
            r"""(['"]{1}game_id['"]{1}[\s]*:[\s]*['"]{1}[\w]+['"]{1})""", raw_header
        )
        if result_id == None:
            raise Exception("Could not find game_id in existing header.")
        raw_game_id = raw_header[result_id.span()[0] : result_id.span()[1]]
        game_id = re.sub(r"[\'\"\s]", "", raw_game_id.split(":")[-1])

        # TIMESTAMP
        result_timestamp = re.search(
            r"""(['"]{1}timestamp['"]{1}[\s]*:[\s]*[0-9]+)""", raw_header
        )
        if result_timestamp == None:
            raise Exception("Could not find timestamp in existing header.")
        raw_timestamp = raw_header[
            result_timestamp.span()[0] : result_timestamp.span()[1]
        ]
        timestamp = re.sub(r"[\'\"\s]", "", raw_timestamp.split(":")[-1])
        return Header(game_id, timestamp)

    except Exception as e:
        print(e)


# =======================================================


def srv_greetings(__data, __route) -> Message:
    # print("HERE", __data, __route)
    header = get_header(__data)
    body = {}
    return Message.deserialize(__route, header, body)


SRV_INDEX = {"srv-greetings": srv_greetings}
