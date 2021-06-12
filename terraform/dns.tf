# retrieve the zone we created manually in the dashboard
data "aws_route53_zone" "zone" {
  name = "${var.dns_zone_name}."
}

data "aws_lb" "eb-lb" {
  arn = aws_elastic_beanstalk_environment.eb-env.load_balancers.0
}

# get sol.wtf to point to EB url
resource "aws_route53_record" "app" {
  name    = data.aws_route53_zone.zone.name
  zone_id = data.aws_route53_zone.zone.id
  type    = "A"
  alias {
    evaluate_target_health = false
    name                   = data.aws_lb.eb-lb.dns_name
    zone_id                = data.aws_lb.eb-lb.zone_id
  }
}

# ------------------------------------------------------------------------------ enable ssl
# copy pasted from https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/acm_certificate_validation

# define the type of validation we want as "DNS"
resource "aws_acm_certificate" "cert" {
  //  domain_name       = aws_route53_record.app.fqdn #fully qualified domain name
  domain_name       = "sol.wtf" #can't use dynamic name above, creates a cycle
  validation_method = "DNS"
  tags              = local.common_tags
  lifecycle {
    create_before_destroy = true #to get around a limitation in terraform
  }
}

# create a validation record in our zone
resource "aws_route53_record" "cert" {
  for_each = {
    for dvo in aws_acm_certificate.cert.domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.zone.zone_id
}

# this is not a real resource, simply used to trigger the validation process. Outputs valied certificate arn used in EB
resource "aws_acm_certificate_validation" "cert" {
  certificate_arn         = aws_acm_certificate.cert.arn
  validation_record_fqdns = [for record in aws_route53_record.cert : record.fqdn]
}
