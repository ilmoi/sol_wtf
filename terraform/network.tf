resource "aws_vpc" "main_vpc" {
  cidr_block           = "10.1.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-vpc" })
  )
}

resource "aws_internet_gateway" "main_ig" {
  vpc_id = aws_vpc.main_vpc.id
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-ig" })
  )
}

# ------------------------------------------------------------------------------ public sub
# --------------------------- a

# define subnet itself
resource "aws_subnet" "public_a" {
  cidr_block              = "10.1.1.0/24"
  vpc_id                  = aws_vpc.main_vpc.id
  map_public_ip_on_launch = true                               #since it's a public subnet, auto-assign public ips
  availability_zone       = "${data.aws_region.current.name}a" #for resiliency want 2+
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-public-a" })
  )
}

# route table tells the subnet how to handle in/out traffic
resource "aws_route_table" "public_a" {
  vpc_id = aws_vpc.main_vpc.id
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-public-a" })
  )
}

# need to connect route table to subnet
resource "aws_route_table_association" "public_a" {
  subnet_id      = aws_subnet.public_a.id
  route_table_id = aws_route_table.public_a.id
  # don't support tags
}

# configure the route table to send all traffic to IG, which in turn sends to web
resource "aws_route" "public_internet_access_a" {
  route_table_id         = aws_route_table.public_a.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.main_ig.id
  # don't support tags
}

//# needed for NAT
//resource "aws_eip" "public_a" {
//  vpc = true
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-public-a" })
//  )
//}
//
//# NAT is needed for our PRIVATE subnet to have outbound access (but not inbound).
//# That said it needs to be defined in the public subnet itself
//resource "aws_nat_gateway" "public_a" {
//  allocation_id = aws_eip.public_a.id
//  subnet_id     = aws_subnet.public_a.id
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-public-a" })
//  )
//}

# --------------------------- b

resource "aws_subnet" "public_b" {
  cidr_block              = "10.1.2.0/24"
  vpc_id                  = aws_vpc.main_vpc.id
  map_public_ip_on_launch = true
  availability_zone       = "${data.aws_region.current.name}b"
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-public-b" })
  )
}

resource "aws_route_table" "public_b" {
  vpc_id = aws_vpc.main_vpc.id
  tags = merge(
    local.common_tags,
    tomap({ Name = "${local.prefix}-public-b" })
  )
}

resource "aws_route_table_association" "public_b" {
  subnet_id      = aws_subnet.public_b.id
  route_table_id = aws_route_table.public_b.id
}

resource "aws_route" "public_internet_access_b" {
  route_table_id         = aws_route_table.public_b.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.main_ig.id
}

//resource "aws_eip" "public_b" {
//  vpc = true
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-public-b" })
//  )
//}
//
//resource "aws_nat_gateway" "public_b" {
//  allocation_id = aws_eip.public_b.id
//  subnet_id     = aws_subnet.public_b.id
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-public-b" })
//  )
//}

# ------------------------------------------------------------------------------ private
# --------------------------- a

//resource "aws_subnet" "private_a" {
//  cidr_block = "10.1.10.0/24"
//  vpc_id     = aws_vpc.main_vpc.id
//  # this time we do NOT assign public ips
//  availability_zone = "${data.aws_region.current.name}a"
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-private-a" })
//  )
//}
//
//resource "aws_route_table" "private_a" {
//  vpc_id = aws_vpc.main_vpc.id
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-private-a" })
//  )
//}
//
//resource "aws_route_table_association" "private_a" {
//  subnet_id      = aws_subnet.private_a.id
//  route_table_id = aws_route_table.private_a.id
//}
//
//resource "aws_route" "private_a_internet_out" {
//  route_table_id         = aws_route_table.private_a.id
//  nat_gateway_id         = aws_nat_gateway.public_a.id
//  destination_cidr_block = "0.0.0.0/0"
//}
//
//# --------------------------- b
//
//resource "aws_subnet" "private_b" {
//  cidr_block        = "10.1.11.0/24"
//  vpc_id            = aws_vpc.main_vpc.id
//  availability_zone = "${data.aws_region.current.name}b"
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-private-b" })
//  )
//}
//
//resource "aws_route_table" "private_b" {
//  vpc_id = aws_vpc.main_vpc.id
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-private-b" })
//  )
//}
//
//resource "aws_route_table_association" "private_b" {
//  subnet_id      = aws_subnet.private_b.id
//  route_table_id = aws_route_table.private_b.id
//}
//
//resource "aws_route" "private_b_internet_out" {
//  route_table_id         = aws_route_table.private_b.id
//  nat_gateway_id         = aws_nat_gateway.public_b.id
//  destination_cidr_block = "0.0.0.0/0"
//}