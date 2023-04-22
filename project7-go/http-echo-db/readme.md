
1. 
```bash
docker-compose up
```

2. connect to DB

3. Create schema using `db/schema.sql` file
```postgresql
CREATE TABLE authors (
    id   BIGSERIAL PRIMARY KEY,
    name text      NOT NULL,
    bio  text
);
```

TODO