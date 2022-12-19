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

You create local modules by creating a `modules` directory and putting directory-per-module there. Generally your modules 
will contain:
- `main.tf` as the main config file.
- `variable.tf` for variables.
- `outputs.tf` for outputs

It's also a good practice to include `LICENCE` and `readme.md` files.

In this case I define an S3 module in `modules/aws-s3-static-website-bucket`, define variable in `variable.tf` and then
provide their names in the `main.tf` inside the main directory (not the `/modules` one). Notice how the variables from 
`aws-s3-static-website-bucket` now server as config values inside the main file. Also notice that the module does not 
define `provider` block, because this block is usually defined in the main file.

Another use of modules: see `outputs.tf` in the main directory. It uses `module.website_s3_bucket.name` naming to expose
an output from the module to the main app.

After you define a new module and use it, you must call `terraform get` to get this module. You can also use `terraform init`
to do it. The difference between two commands is that the first only installs modules, while the second will also initialize
backends and install plugins.

## Further capabilities

Apart from what present here you can also:
- define multiple properties for your modules using terraform object attributes
- manage variable values either with default values defined in module or expose the configuration to your client. See
example of [this + objects here](https://developer.hashicorp.com/terraform/tutorials/modules/module-object-attributes).
- use sensible configs for sensitive data
- and many more
