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

variable "ssh_key_name" {
  default = "plato-kp-new"
}

variable "dns_zone_name" {
  description = "Domain name"
  default     = "sol.wtf"
}