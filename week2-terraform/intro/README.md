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

That will download a Terraform plug to interact with Docker. After that you only need to execute

```shell
terraform plan
```

to see what Terraform wants to do and then 

```shell
terraform apply
```

TF will ask you if you really want to apply the changes, and if you say **yes** you must see a nginx container running and
exposing port 8000 with `docker ps`. You may now open `http://localhost:8000` to see default nginx page.

The last step is to destroy this configuration

```shell
terraform destroy
```

don't forget to say **yes** when prompted again.

## 
