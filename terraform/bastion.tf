//# bastion instance is purely used for admin and never accessed by the user
//
//# ------------------------------------------------------------------------------ sg
//resource "aws_security_group" "bastion" {
//  description = "Allows access to Bastion"
//  name        = "${local.prefix}-bastion-sg"
//  vpc_id      = aws_vpc.main_vpc.id
//  ingress {
//    from_port   = 22
//    protocol    = "tcp"
//    to_port     = 22
//    cidr_blocks = ["0.0.0.0/0"]
//  }
//  egress {
//    from_port   = 443
//    protocol    = "tcp"
//    to_port     = 443
//    cidr_blocks = ["0.0.0.0/0"]
//  }
//  egress {
//    from_port   = 80
//    protocol    = "tcp"
//    to_port     = 80
//    cidr_blocks = ["0.0.0.0/0"]
//  }
//  egress {
//    from_port = 5432
//    protocol  = "tcp"
//    to_port   = 5432
//    cidr_blocks = [
//      aws_subnet.private_a.cidr_block,
//      aws_subnet.private_b.cidr_block
//    ]
//  }
//  tags = merge(
//    local.common_tags,
//    tomap({ Name : "${local.prefix}-bastion-sg" })
//  )
//}
//
//# ------------------------------------------------------------------------------ iam
//
//# 1 create a new IAM role and make it attachable to ec2 instances
//resource "aws_iam_role" "bastion" {
//  name = "${local.prefix}-bastion"
//  # allows an ec2 instance to assume the role
//  assume_role_policy = file("./templates/bastion/instance-profile-policy.json")
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-bastion" })
//  )
//}
//
//# 2 attach the policy for managing containers to the role we just created
//resource "aws_iam_role_policy_attachment" "bastion_attach_policy" {
//  role       = aws_iam_role.bastion.name
//  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
//}
//
//# 3 create an instance profile that conains the role (that contains the policy)
//resource "aws_iam_instance_profile" "bastion" {
//  name = "${local.prefix}-bastion-instance-profile"
//  role = aws_iam_role.bastion.name
//}
//
//# ------------------------------------------------------------------------------ bastion
//
//data "aws_ami" "amazon_linux" {
//  most_recent = true
//  filter {
//    name   = "name"
//    values = ["amzn2-ami-hvm-2.0.*-x86_64-gp2"]
//  }
//  owners = ["amazon"]
//}
//
//resource "aws_instance" "bastion" {
//  ami           = data.aws_ami.amazon_linux.id
//  instance_type = "t2.micro"
//  user_data     = "./templates/bastion/data-user.sh"
//  # 4 attach the instance profile to bastion
//  iam_instance_profile = aws_iam_instance_profile.bastion.name
//
//  key_name               = var.ssh_key_name
//  subnet_id              = aws_subnet.public_a.id
//  vpc_security_group_ids = [aws_security_group.bastion.id]
//
//  tags = merge(
//    local.common_tags,
//    tomap({ Name = "${local.prefix}-bastion" })
//  )
//}