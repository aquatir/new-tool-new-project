# Changing infra

Continue tutorial from https://developer.hashicorp.com/terraform/tutorials/docker-get-started/docker-change

## Initialize

First delete `main.tf` and rename `main.tf.backup` into `main.tf` before continuing with this project.

Second, apply Terraform from new project

```shell 
terraform init
echo yes | terraform apply
```

## Updating config

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

Terraform will also show `-` when you execute `terraform destroy` before saying yes. Execute it and observe the
following line in console

```shell
docker_container.nginx will be destroyed
docker_image.nginx will be destroyed
```

## Using values

You can manage variables by either creating them in existing file or in new file such as `variables.tf`. The file name
doesn't matter, terraform is only looking for `.tf` file extension.

Now update the name in `main.tf` file `name = tutorial =>  name = var.container_name` and run TF `terraform apply`.

You can override this name by providing the value when running terraform apply. Try running

```shell
terraform apply -var "container_name=YetAnotherName"
```

and observe that the container's name is chained now.

### Using files

You can also provide files that will have variable names. See file `terraform.tfvars` as an example. It provides a value
for variable defined in `varibalbes.tf`. Notice the extension `.tfvars`. You can also provide such files in apply with
`-var-file` flag.

## Managing outputs

Using vars is a way to work with inputs. Terraform also allow you to manage outputs. See the file `outputs.tf` in this
example. This file mentions two values. Now run `terraform apply` and check two new files with

```shell
terraform output
```

Those values can be later picked up by other terraform or any other jobs and used to whatever purpose you see fit.
