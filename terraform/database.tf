resource "aws_db_subnet_group" "main" {
  name = "${local.prefix}-main" #for some reason tags not enough, also need this line
  # db will be available from each of these subnets
  subnet_ids = [
    #todo not storing any sensisitve / important data
    aws_subnet.public_a.id,
    aws_subnet.public_b.id,
  ]
  tags = local.common_tags
}

resource "aws_security_group" "rds" {
  description = "Allows access to RDS"
  name        = "${local.prefix}-rds-inbound-sg"
  vpc_id      = aws_vpc.main_vpc.id
  ingress {
    from_port   = 5432
    protocol    = "TCP"
    to_port     = 5432
    cidr_blocks = ["0.0.0.0/0"]
  }
  tags = local.common_tags
}

resource "aws_db_instance" "main" {
  identifier        = "${local.prefix}-db"               #how we access our db in other parts of aws
  name              = "solwtf"                           #db name within the postgres server
  availability_zone = "${data.aws_region.current.name}a" #putting both the db and the EB instances into A

  instance_class        = "db.t3.medium" # experimentally determined that this is enough for now
  allocated_storage     = 100 #todo for now don't see a need for IOPS-optimized, see how perf does
  max_allocated_storage = 500
  storage_type          = "gp2"

  engine         = "postgres"
  engine_version = "12.5"

  db_subnet_group_name   = aws_db_subnet_group.main.name
  vpc_security_group_ids = [aws_security_group.rds.id]

  password = var.db_password
  username = var.db_username

  # todo all of below would be different if we stored any sensitive/important data
  backup_retention_period = 0
  multi_az                = false #doubles the cost if true - two AZs
  deletion_protection     = false
  skip_final_snapshot     = true #creates problems in terraform if try to create / delete multiple times
  apply_immediately       = true
  publicly_accessible     = true

  tags = merge(
    local.common_tags,
    tomap({ Name : "${local.prefix}-rds-main" })
  )
}