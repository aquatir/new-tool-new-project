# Modules

How to use Terraform modules https://developer.hashicorp.com/terraform/tutorials/modules/module

## Initialize

Delete `main.tf` and rename `main.tf.backup` into `main.tf` before continuing with this project.

## Terraform module

Modules is a way to split you configuration between multiple folders. You can also leverage modules crated by other people
in your infrastructure. You can even load modules from remote hosts over HTTP URLs or Terraform Cloud.

### Prerequisite 

This tutorial will require AWS accounts with IAM role set up, so please follow [this Terraform tutorial](https://developer.hashicorp.com/terraform/tutorials/aws-get-started/aws-build) to do it first.

you'll have to setup 2 env variables
```shell
export AWS_ACCESS_KEY_ID=
export AWS_SECRET_ACCESS_KEY=
```
