# Apache Cassandra

No SQL database.

- Web site: https://cassandra.apache.org/_/index.html
- Getting started guide: https://cassandra.apache.org/doc/latest/cassandra/getting_started

## Week 3 — Apache Cassandra

### Motivation

- Level 0: learn how to deploy & interact with cassandra.
- Level 1: learn basics of Cassandra internal architecture and how it influences client code.

### Links

- [intro](/week3-apache-cassandra/intro). Docker-compose setup with a tiny Java app executing a bunch of commands.

## Conclusion

**DISCLAIMER**: I write this chapter in the end of a week to summarize my learning about a new tool. This is heavily
biased opinion, and you should do your own research for any practical case.

Cassandra is known to be a massive-scale database, and it achieves it by introducing a lot of constraints. The CQL is
very-very far from SQL in even the simplest things. You can't just batch-insert/batch-delete rows, you have to use a
special syntax. There is a lot of reasoning why this is the case, and it's 100% justifiable, but it's a language which
is very similar yet very nuanced at the same time.

Due to these limitations there is a substantial learning curve to Cassandra, you have to learn and read on the correct
ways to do certain operations, e.g. batch-inserts/deletes. This is all expected, but still a thing to keep in mind. I've
only touched a single Java client + console client during this week, and it feels like I'd need 20+ hours more to learn
even the basics of a client. And I'm not even talking on the overall architecture and correct ways to deploy.

I'll sure return to Cassandra once again in distant future. Still want to set up a multi-node cluster and check how the
failover works + figure out the correct way to execute common operations. Even the basic client given both sync and
async operations out of the box — a lot to learn.
