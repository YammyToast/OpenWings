# Networking Message Structures

## Schema



### Greeting Message

route : 'srv-greetings'

```
{
    route: 'srv-greetings',
    header: {
        game_id: num,
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

route: 'srv-register-succeed'

``` 
{
    route: 'srv-register-succeed',
    header: {
        game_id: num,
        timestamp: num (UNIX EPOCH)
    },
    body: {
        req_uuid: string,
        new_uuid: string

    }

}
```

### Unsuccessful Register

route: 'srv-register-fail'

```
{
    route: 'srv-register-fail',
    header: {
        game_id: num,
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

route: 'cl-req-register'

```
{
    route: 'cl-req-register'
    header: {
        game_id: num
        timestamp: num (UNIX EPOCH),
    },
    body: {
        username: string,
        req_uuid: string
    }
}
```

### Force Start Game

route: 'cl-force-start'


```
{
    route: 'cl-req-register'
    header: {
        game_id: num
        timestamp: num (UNIX EPOCH),
    },
    body: {
        player_uuid: string
    }
}
```