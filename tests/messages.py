from abc import ABC, abstractmethod

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 25373  # The port used by the server

class Header:
    def __init__(self, __game_id: int) -> None:
        self.game_id = __game_id

    def serialize(self) -> str:
        print("serialize header")

class Body(ABC):
    route: str
    @abstractmethod
    def serialize(self) -> str:
        pass

class Message:
    def __init__(self, __game_id: int, __body: Body) -> None:
        self.header = Header(__game_id)
        self.body = __body
        pass

    def serialize(self) -> str:
        print("serialize message")
        buf = f"{{ \"header\" }}"
        return buf.encode('utf-8')

# =======================================================