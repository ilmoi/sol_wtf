resource "aws_db_subnet_group" "main" {
  name = "${local.prefix}-main" #for some reason tags not enough, also need this line
  # db will be available from each of these subnets
  subnet_ids = [
    aws_subnet.private_a.id,
    aws_subnet.private_b.id,
  ]
  tags = merge(
    local.common_tags,
    tomap({ Name : "${local.prefix}-main" })
  )
}

resource "aws_security_group" "rds" {
  description = "Allows access to RDS"
  name        = "${local.prefix}-rds-inbound-sg"
  vpc_id      = aws_vpc.main_vpc.id
  ingress {
    from_port       = 5432
    protocol        = "TCP"
    to_port         = 5432
    security_groups = [aws_security_group.bastion.id] #limit inbound access to only traffic from bastion's SG
  }
  tags = merge(
    local.common_tags,
    tomap({ Name : "${local.prefix}-rds-inbound-sg" })
  )
}

resource "aws_db_instance" "main" {
  identifier = "${local.prefix}-db" #how we access our db in other parts of aws
  name       = "solwtf"             #db name within the postgres server

  instance_class    = "db.t2.micro"
  allocated_storage = 20
  storage_type      = "gp2"

  engine         = "postgres"
  engine_version = "12.5"

  db_subnet_group_name    = aws_db_subnet_group.main.name
  password                = var.db_password
  username                = var.db_username
  backup_retention_period = 0     #todo he said 7 typically good
  multi_az                = false #todo in prod he said should be true
  skip_final_snapshot     = true  #creates problems in terraform
  vpc_security_group_ids  = [aws_security_group.rds.id]
  tags = merge(
    local.common_tags,
    tomap({ Name : "${local.prefix}-rds-main" })
  )
}