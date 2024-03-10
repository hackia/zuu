packer {
  required_plugins {
    docker = {
      version = ">= 0.0.7"
      source  = "github.com/hashicorp/docker"
    }
  }
}

variable "image_stable" {
  type    = string
  default = "taishingi/rlang:stable"
}

variable "image_beta" {
  type    = string
  default = "taishingi/rlang:beta"
}

variable "image_nightly" {
  type    = string
  default = "taishingi/rlang:nightly"
}

source "docker" "stable" {
  image  = var.image_stable
  commit = false
  discard = true
}

source "docker" "beta" {
  image  = var.image_beta
  commit = false
  discard = true
}

source "docker" "nightly" {
  image  = var.image_nightly
  commit = false
  discard = true
}


build {
  name = "stable"
  source "source.docker.stable" {
  }

  provisioner "shell" {
    scripts = ["stable"]
  }
}

build {
  name = "beta"
  source "source.docker.beta" {
  }

  provisioner "shell" {
    scripts = ["beta"]
  }
}

build {
  name = "nightly"
  source "source.docker.nightly" {
  }

  provisioner "shell" {
    scripts = ["nightly"]
  }
}