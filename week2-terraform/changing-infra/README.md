# Intro

Continue tutorial from https://developer.hashicorp.com/terraform/tutorials/docker-get-started/docker-change

## Initialize

First delete `main.tf` and rename `main.tf.backup` into `main.tf` before continuing with this project.

Second, apply Terraform from new project 
```shell 
terraform init
echo yes | terraform apply
```

Now update nginx in `main.tf` to expose port `8080` `external = 8000 -> external = 8080`  and run

```shell
terraform plan
```

You'd see in the output TF mentions 
```
-/+ resource "docker_container" "nginx"
```

and further down

```
`~ external = 8000 -> 8080 # forces replacement`
```

The `-/+` means that terraform will have to destroy existing resource and create a new one. Terraform can however update 
some resources in-place which is indicated with `~`. You can not expose and extra port from Docker container without 
recreating it, so here you have to recreate. Run

```shell
terraform apply
```

and observe that TF replaces the container and that it's now available on `localhost:8080`.

Terraform will also show `-` when you execute `terraform destroy` before saying yes. Execute it and observe the following
line in console

```shell
docker_container.nginx will be destroyed
docker_image.nginx will be destroyed
```
