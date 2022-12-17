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

Terraform will check that the current environment match or do not match the required. You can for example stop the container
after running it
```shell
docker stop tutorial
```

And execute `terraform plan` again. It will show you than you a container must be created, because an image is already there. 
Try executing `terraform apply` again and see that the container is re-created again.

The last step is to destroy this configuration.

```shell
terraform destroy
```

don't forget to say **yes** when prompted again.

## Theory

Each Terraform configuration must be in its own working directory. The `terraform {}` block is a set of setting, that also 
include the providers. You can browse available providers in registry https://registry.terraform.io and this is a default 
registry used. The `kreuzwerker/docker` from `main.tf` is a short form for `registry.terraform.io/kreuzwerker/docker`. 

Next comes `provider` block which is the place where you configure providers — plugins used by terraform to create and 
manage resources.

The last block is `resources` which are physical or virtual components such as docker image and docker container in this 
example. 

## Other important TF commands

To format terraform files execute
```shell
terraform fmt
```

To validate configuration run
```shell
terraform validate
```

To see the current state of deployment run
```shell
terraform show
```

Note: terraform get this info from a file named `terraform.tfstate`. It can contain secrets and sensitive data, so make 
sure to store it appropriately in production environment. 

You can also manage state in fine-grained manner using `terraform state` command, and it's subcommands. Use
```shell
terraform state list
```

to see the list of resources. Note: the state is used to mark the resources tracked by terraform. Removing parts of the 
state does not remove the resource. It only removes the "marking" that this resource is manager by terraform. E.g. try 
removing docker container with 

```shell
terraform state rm docker_container.nginx
```

And then see that `docker ps` still reports that the container is running. Now try to run this configuration with
```shell
terraform apply
```

You'll get an error
```
Error: Unable to create container: Error response from daemon: Conflict. The container name "/tutorial" is already in use by container
```

This is because by removing container from state, terraform stopped tracking it, so your subsequent attempt to create a 
resource fails, because *something* not belonging to terraform stops it from deploying. Stop & remove this container 
```shell
docker stop tutorial
docker container rm tutorial
```

and return TF with `terraform apply` to apply the changes again.
