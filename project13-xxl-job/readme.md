# XXL-JOB

Running and using XXL-JOB library 

https://github.com/xuxueli/xxl-job

## Motivation

- Level 0: learn how it works

### How it works

- Clone https://github.com/xuxueli/xxl-job with git
- Inside the repo, change `xxl-job-admin/src/main/resources/application.properties` to a Dockerized DB:
```docker
spring.datasource.url=jdbc:mysql://mysql:3306/xxl_job?useUnicode=true&characterEncoding=UTF-8&autoReconnect=true&serverTimezone=Asia/Shanghai
spring.datasource.username=root
spring.datasource.password=root_pwd
```
- Go main `xxl-job` folder and run `mvn package`
- Now go to `xxl-job/xxl-job-admin` and build a docker image with `docker build -t xxl-job-admin .`
- Finally, run `docker-compose up`, navigate to `localhost:8080/xxl-job-admin/` and input default login/pw pair `admin/123456`.

With this you will have admin panel running, but you also need to run an executor...
