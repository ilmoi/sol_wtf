terraform {
  // we're keeping state in the cloud rather than on local machine
  backend "s3" {
    bucket         = "solwtf-terraform-bucket"
    key            = "solwtf.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "solwtf-terra-lock"
  }
  // this is the new way of instantiating providers post 0.13 - https://www.terraform.io/docs/language/providers/requirements.html
  required_providers {
    aws = {
      source  = "aws"
      version = "~> 3.45.0"
    }
  }
}

locals {
  prefix = "${var.prefix}-${terraform.workspace}"
  common_tags = {
    Environment = terraform.workspace
    Project     = var.project
    ManagedBy   = "Terraform"
  }
}

provider "aws" {
  region = "us-east-1"
}

data "aws_region" "current" {}