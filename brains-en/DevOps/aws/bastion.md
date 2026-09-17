# How to Access Private EC2 via Bastion Server + Hi-v2 Troubleshooting

### Architecture

While working on the project, I set up an AWS environment with the following architecture.

![](https://github.com/GSM-MSG/hi-infrastructure-global-v1/raw/master/architecture/hi_architecture.png)

For the first time, I launched the main server EC2 in a private subnet and accessed the private EC2 through a Bastion.

Both the bastion and main server EC2 instances were built using Terraform scripts.

```tf
resource "aws_instance" "hi-bastion" {
    ami = "ami-04cebc8d6c4f297a3"
    instance_type = "t2.micro"
    subnet_id =  "${aws_subnet.hi-public-subnet-2a.id}"
    vpc_security_group_ids = [aws_security_group.hi-bastion-sg.id]
    key_name = "hi-key"

    tags = {
        Name = "hi-bastion"
    }
}

resource "aws_instance" "hi-main-server" {
    ami = "ami-04cebc8d6c4f297a3"
    instance_type = "t3.micro"
    subnet_id = "${aws_subnet.hi-private-subnet-2a.id}"
    vpc_security_group_ids = [aws_security_group.hi-main-server-sg.id]
    key_name = "hi-key"
    associate_public_ip_address = false
    source_dest_check = false
    
    tags = {
        Name = "hi-main-server"
    }
}
```

To access the main server EC2, you need to store the private EC2 key pair on the bastion server.

Currently, I'm using the `hi-key` key pair, which is stored on my computer. I'm going to send this key pair to the bastion.

Enter the following into your computer's terminal.

```zsh
$ sudo scp -i <key-pair-path\>/<key-pair\>.pem <key-pair-path\>/<key-pair\>.pem <username\>@<public_ip_dns\>:/home/ubuntu
```

Then you can see that your key pair has been uploaded to the bastion server.

<br>

## Troubleshooting

### keypair are too open

```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@         WARNING: UNPROTECTED PRIVATE KEY FILE!          @
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
Permissions 0755 for 'hi-key.pem' are too open.
It is required that your private key files are NOT accessible by others.
This private key will be ignored.
Load key "hi-key.pem": bad permissions
ubuntu@ec2-43-201-86-41.ap-northeast-2.compute.amazonaws.com: Permission denied (publickey).
```

Sometimes you might encounter an error indicating that the key pair is too open. In that case, entering `chmod 400 <keypair>.pem` will resolve it immediately.

### keypair permission denied

```
hi-admin@ec2-43-201-86-41.ap-northeast-2.compute.amazonaws.com: Permission denied (publickey).
scp: Connection closed
```

This was a bit of a silly mistake; I thought it was an error like the security group inbound rules not being open. However, I got 'Permission denied' because I either entered just the public IP instead of `ubuntu@<public ip DNS>` (which requires the public IP DNS), or I entered the wrong login username like 'ubuntu'. It's good to know this to avoid unnecessary troubleshooting in the future!
