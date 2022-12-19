# PGQ

A queue build on top of postgreSQL.

- Web site: https://pgq.github.io
- PGQ extension page: https://pgq.github.io/extension/pgq/files/external-sql.html
- PGQ tutorial: https://wiki.postgresql.org/wiki/PGQ_Tutorial

## Week 2 — PGQ

### Motivation

- Level 0: understand how to configure PGQ on top of postgreSQL. 
- Level 1: check out transactional guarantees, failure scenarios.
- Level 2: create a Java app using [outbox pattern](https://microservices.io/patterns/data/transactional-outbox.html) on 
top of PGQ.

### Links

- [intro](./intro). Initial configuration docker.

## Conclusion

**DISCLAIMER**: I write this chapter in the end of a week to summarize my learning about a new tool. This is heavily biased
opinion, and you should do your own research for any practical case.

Weirdly enough I couldn't even install it lol. I've gone over a couple of docs
- The [reference](https://pgq.github.io/extension/pgq/files/external-sql.html) 
- A setup for [londiste](https://wiki.postgresql.org/wiki/Londiste_Tutorial_(Skytools_2)#The_ticker_daemon) but it's 
version 2... and it also mentions that the tool can lose data.
- A [tutorial](https://wiki.postgresql.org/wiki/PGQ_Tutorial) that says "The API documentation is online, this document will consider you know about it." with a broken
link to said documentation 
- Weirdly enough a somewhat active [github](https://github.com/pgq/pgq)
- Even a GitHub issue stating to "please provide some docs" https://github.com/pgq/pgq/issues/12 with an answer mentioning 
docs from above and.

Maybe I didn't pass the "smart-guy" test to figure out everything, but honestly even if it's the best tool in the World, 
without a proper step-by-step guide I can't imagine myself selling this tool to anybody. So it's a hard pass. 
