variable "cloudflare_email" {}
variable "cloudflare_token" {}

provider "cloudflare" {
    email = "${var.cloudflare_email}"
    token = "${var.cloudflare_token}"
}

resource "cloudflare_record" "root" {
    domain = "jumoel.com"
    name = "@"
    value = "www.jumoel.com"
    type = "CNAME"
    ttl = 300
}

resource "cloudflare_record" "www" {
    domain = "jumoel.com"
    name = "www"
    value = "jumoel.github.io"
    type = "CNAME"
    ttl = 300
}