# Networking Message Structures

## Server

### Greeting Message

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