# ARN of the EC2 ReadOnly IAM role
output "iam_kumo_arn" {
  value = "${aws_iam_role.kumo.arn}"
}
