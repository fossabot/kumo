# ARN of the EC2 ReadOnly IAM role
output "iam_ec2readonly_arn" {
  value = "${aws_iam_role.ec2readonly.arn}"
}
