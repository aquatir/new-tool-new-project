
### GET
```bash
curl localhost:8080/get
```


### Post
```bash
curl -H "Content-Type: application/json" --data '{"username": "kek"}' -X POST localhost:8080/post
```

### Get with custom backend

Normal get
```
curl -v localhost:8080/backend
```

Get with headers
```
curl -v localhost:8080/backend/header
```

With status. 403 could be any status code.
```
curl -v "localhost:8080/backend/status?status=403"
```

Post body, will return whatever you post if it's a JSON
```
curl -X POST -H "Content-Type: application/json" --data '{"reqKey": "reqKeyValue"}'  localhost:8080/backend/body
```
