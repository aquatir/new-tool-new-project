# Apollo 

Running and using Apollo config manager

https://github.com/apolloconfig/apollo

## Motivation

- Level 0: learn how it works

### How it works

1. Go to `docker-quick-start`
2. Run `docker-compose up`. Wait until it outputs 
3. Navigate to `http://localhost:8070` to access portal. Use `apollo/admin` as a default login-password pair
4. Use `docker exec -i apollo-quick-start /apollo-quick-start/demo.sh client` to connect to a running Apollo server
5. Input a key `timeout` to get the value of key.
6. Change the value in Apollo portal, release, see the changed value reflected in console.

### TODO

Connect to Apollo from the app. The curren code doesn't connect, most likely due to incorrect network policy with docker / non-docker deployment https://www.apolloconfig.com/#/en/deployment/distributed-deployment-guide?id=_14-network-policy
