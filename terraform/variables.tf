variable "project" {
  default = "solwtf"
}

variable "prefix" {
  default = "sol"
}

variable "db_username" {
  description = "Username for the RDS postgres instance"
}

variable "db_password" {
  description = "Password for the RDS postgres instance"
}

variable "bastion_key_name" {
  default = "plato-kp-new"
}