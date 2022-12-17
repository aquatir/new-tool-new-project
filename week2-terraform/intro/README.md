# Intro

I'll be using [Docker infrastructure tutorial](https://developer.hashicorp.com/terraform/tutorials/docker-get-started).

## Install

macOS:
```shell
brew tap hashicorp/tap
brew install hashicorp/tap/terraform
```

## Running

The first deployment is available at `main.tf` file. It runs and nginx server available at post 8000. To run it you must first
run 
```shell
terraform init
```

That will download a terraform plug to interract with Docker. After that you only need to execute

```shell
terraform apply
```
