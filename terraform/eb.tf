resource "aws_elastic_beanstalk_application" "eb" {
  name        = "${local.prefix}-eb"
  description = "EB instance spawned from terra"
  tags        = local.common_tags
}

resource "aws_elastic_beanstalk_environment" "eb-env" {
  name                = "${local.prefix}-eb-env"
  application         = aws_elastic_beanstalk_application.eb.name
  solution_stack_name = "64bit Amazon Linux 2 v3.4.1 running Docker"
  tier                = "WebServer"
  tags                = local.common_tags

  # https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options-general.html
  # ------------------------------------------------------------------------------ proxy server
  setting {
    name      = "ProxyServer"
    namespace = "aws:elasticbeanstalk:environment:proxy"
    value     = "none"
  }
  # ------------------------------------------------------------------------------ security & iam
  setting {
    name      = "EC2KeyName"
    namespace = "aws:autoscaling:launchconfiguration"
    value     = var.ssh_key_name
  }
  setting {
    name      = "IamInstanceProfile"
    namespace = "aws:autoscaling:launchconfiguration"
    value     = "aws-elasticbeanstalk-ec2-role" #has ECR rights attached
  }
  # ------------------------------------------------------------------------------ scaling
  setting {
    name      = "MinSize"
    namespace = "aws:autoscaling:asg"
    value     = "1"
  }
  setting {
    name      = "MaxSize"
    namespace = "aws:autoscaling:asg"
    value     = "3"
  }
  setting {
    name      = "BreachDuration"
    namespace = "aws:autoscaling:trigger"
    value     = "3" #in mins
  }
  setting {
    name      = "MeasureName"
    namespace = "aws:autoscaling:trigger"
    value     = "CPUUtilization"
  }
  setting {
    name      = "Period"
    namespace = "aws:autoscaling:trigger"
    value     = "3" #in mins
  }
  setting {
    name      = "Statistic"
    namespace = "aws:autoscaling:trigger"
    value     = "Average"
  }
  setting {
    name      = "Unit"
    namespace = "aws:autoscaling:trigger"
    value     = "Percent"
  }
  setting {
    name      = "LowerThreshold"
    namespace = "aws:autoscaling:trigger"
    value     = "10"
  }
  setting {
    name      = "UpperThreshold"
    namespace = "aws:autoscaling:trigger"
    value     = "60"
  }
  # ------------------------------------------------------------------------------ instances
  setting {
    name      = "InstanceTypes"
    namespace = "aws:ec2:instances"
    value     = "t2.large"
  }
  # ------------------------------------------------------------------------------ vpc
  setting {
    name      = "VPCId"
    namespace = "aws:ec2:vpc"
    value     = aws_vpc.main_vpc.id
  }
  setting {
    name      = "Subnets"
    namespace = "aws:ec2:vpc"
    value     = "${aws_subnet.public_a.id},${aws_subnet.public_b.id}"
  }
  setting {
    name      = "ELBSubnets"
    namespace = "aws:ec2:vpc"
    value     = "${aws_subnet.public_a.id},${aws_subnet.public_b.id}"
  }
  # ------------------------------------------------------------------------------ backend health endpoint
  # todo configure /health on backend
  #  aws:elasticbeanstalk:application
  #  aws:elasticbeanstalk:environment:process:process_name - not sure need this
  #  aws:elbv2:listenerrule:rule_name
  # ------------------------------------------------------------------------------ enhanced health monitoring
  # todo ensure enhanced health works ok
  setting {
    name = "SystemType"
    namespace = "aws:elasticbeanstalk:healthreporting:system"
    value = "enhanced"
  }
  # ------------------------------------------------------------------------------ logs
  setting {
    name      = "StreamLogs"
    namespace = "aws:elasticbeanstalk:cloudwatch:logs"
    value     = "true"
  }
  setting {
    name      = "DeleteOnTerminate"
    namespace = "aws:elasticbeanstalk:cloudwatch:logs"
    value     = "true"
  }
  setting {
    name      = "RetentionInDays"
    namespace = "aws:elasticbeanstalk:cloudwatch:logs"
    value     = "7"
  }
  # ------------------------------------------------------------------------------ deployment policy
  setting {
    name      = "DeploymentPolicy"
    namespace = "aws:elasticbeanstalk:command"
    value     = "AllAtOnce"
  }
  # ------------------------------------------------------------------------------ load balancer
  setting {
    name      = "LoadBalancerType"
    namespace = "aws:elasticbeanstalk:environment"
    value     = "application"
  }
  setting {
    name      = "ListenerEnabled"
    namespace = "aws:elbv2:listener:443"
    value     = "true"
  }
  setting {
    name      = "DefaultProcess"
    namespace = "aws:elbv2:listener:443"
    value     = "default"
  }
  setting {
    name = "Protocol"
    namespace = "aws:elbv2:listener:443"
    value = "HTTPS"
  }
  setting {
    name      = "SSLCertificateArns"
    namespace = "aws:elbv2:listener:443"
    value     = aws_acm_certificate_validation.cert.certificate_arn
  }
  # ------------------------------------------------------------------------------ notifications
  setting {
    name      = "Notification Endpoint"
    namespace = "aws:elasticbeanstalk:sns:topics"
    value     = "iljamoi@pm.me"
  }
}