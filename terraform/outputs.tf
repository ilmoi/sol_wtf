output "db_host" {
  value = aws_db_instance.main.address
}

output "eb_url" {
  value = aws_elastic_beanstalk_environment.eb-env.endpoint_url
}

output "eb_cname" {
  value = aws_elastic_beanstalk_environment.eb-env.cname
}

output "eb_id" {
  value = aws_elastic_beanstalk_environment.eb-env.id
}

output "eb_name" {
  value = aws_elastic_beanstalk_environment.eb-env.name
}

output "eb_lb" {
  value = aws_elastic_beanstalk_environment.eb-env.load_balancers.0
}