# Networking Message Structures

## Schema

100-199 : Util - Server
200-300 : Util - Client
300-399 : Game - Server
400-499 : Game - Client 

## Server

### Greeting Message

CODE: 100

```
{
    header: {
        game_id: num,
        port: string,
        timestamp: num (UNIX EPOCH)
    },
    body: {
        current_players: num,
        game_settings: {
            ...
        }
    }

}
```

### Successful Register

CODE: 101

``` 
{
    header: {
        game_id: num,
        port: string,
        timestamp: num (UNIX EPOCH)
    },
    body: {
        req_uuid: string,
        new_uuid: string

    }

}
```

### Unsuccessful Register

CODE: 102

```
{
    header: {
        game_id: num,
        port: string,
        timestamp: num (UNIX EPOCH)
    },
    body: {
        req_uuid: string,
        err: string
    }

}
```

## Client

### Register Player
```
{
    header: {
        game_id: num
        port: string,
        timestamp: num (UNIX EPOCH),
    },
    body: {
        username: string,
        req_uuid: string
    }
}
```