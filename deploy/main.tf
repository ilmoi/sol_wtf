terraform {
  // we're keeping state in the cloud rather than on local machine
  backend "s3" {
    bucket = "solwtf-terraform-bucket"
    key = "solwtf.tfstate"
    region = "us-east-1"
    encrypt = true
    dynamodb_table = "solwtf-terra-lock"
  }
}

provider "aws" {
  region = "us-east-1"
  version = "~> 3.45.0"
}