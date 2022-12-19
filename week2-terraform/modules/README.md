# Modules

How to use Terraform modules https://developer.hashicorp.com/terraform/tutorials/modules/module

## Terraform module

Modules is a way to split you configuration between multiple folders. You can also leverage modules crated by other people
in your infrastructure. You can even load modules from remote hosts over HTTP URLs or Terraform Cloud.

### Setup

Rename `main.tf.backup` into `main.tf`. Execute `terraform init`.

### Prerequisite 

This tutorial will require AWS accounts with IAM role set up, so please follow [this Terraform tutorial](https://developer.hashicorp.com/terraform/tutorials/aws-get-started/aws-build) to do it first. Simply configuring `aws configure` with correct keys 
is enough.

## Using existing modules

`main.tf` uses two modules. The module starts with "module" keyword followed by a name. The config of module must include 
the source + and optional version. All the parameter after are module specific. In this case I'm using vpc and ec2 modules.
You can find their respective pages in terraform registry ([vpc](https://registry.terraform.io/modules/terraform-aws-modules/vpc/aws/latest), 
[ec2](https://registry.terraform.io/modules/terraform-aws-modules/ec2-instance/aws/latest)).

## Creating custom module


