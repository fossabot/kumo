# Policy
data "aws_iam_policy_document" "ec2readonly" {
  statement {
    actions = [
      "ec2:DescribeInstances",
      "ec2:DescribeImages",
      "ec2:DescribeTags",
      "ec2:DescribeSnapshots",
    ]

    resources = ["*"]
  }
}

resource "aws_iam_policy" "ec2readonly" {
  name   = "ec2readonly"
  policy = "${data.aws_iam_policy_document.ec2readonly.json}"
}

# Role
data "aws_iam_policy_document" "sts" {
  statement {
    actions = ["sts:AssumeRole"]

    principles {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }

    principles {
      type        = "AWS"
      identifiers = ["${var.iam_sts_roles}"]
    }
  }
}

resource "aws_iam_role" "ec2readonly" {
  name        = "ec2readonly"
  description = "Readonly access to EC2"

  assume_role_policy = "${data.aws_iam_policy_document.ec2readonly.json}"
}

resource "aws_iam_role_policy_attachment" "readonly" {
  role       = "${aws_iam_role.ec2readonly.name}"
  policy_arn = "${aws_iam_policy.ec2readonly.arn}"
}
