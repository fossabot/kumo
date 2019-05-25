# Role
data "aws_iam_policy_document" "sts" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }

    principals {
      type        = "AWS"
      identifiers = ["${var.iam_sts_roles}"]
    }
  }
}

resource "aws_iam_role" "kumo" {
  name        = "kumo"
  description = "AWS monitoring"

  assume_role_policy = "${data.aws_iam_policy_document.sts.json}"
}

# Policy
data "aws_iam_policy_document" "ec2readonly" {
  statement {
    actions = [
      "ec2:Describe*",
    ]

    resources = ["*"]
  }
}

resource "aws_iam_policy" "ec2readonly" {
  name   = "ec2readonly"
  policy = "${data.aws_iam_policy_document.ec2readonly.json}"
}

# Attachments
resource "aws_iam_role_policy_attachment" "readonly" {
  role       = "${aws_iam_role.kumo.name}"
  policy_arn = "${aws_iam_policy.ec2readonly.arn}"
}
