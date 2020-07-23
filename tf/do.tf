#
# Variables required
#

# Set the variable value in *.tfvars file or use -var="do_token=..." CLI option

variable "do_token" {
  type = string
}

variable "do_cluster_name" {
  type    = string
  default = "ip2country"
}

variable "do_region" {
  type    = string
  default = "fra1"
}

variable "do_k8_droplet_size" {
  type    = string
  default = "s-1vcpu-2gb"
}


#
# Provider
#

# Configure the DigitalOcean Provider
provider "digitalocean" {
  token = var.do_token
}


#
# Resources
#

#
# Create project
#

resource "digitalocean_project" "tf-project" {
  name        = "tf-${var.do_cluster_name}"
  description = "Project for deploying ip2country to DO; Cluster name: ${var.do_cluster_name}"
  purpose     = "Research"
  environment = "Development"
}

#
# Create vpc
#

resource "digitalocean_vpc" "tf-vpc" {
  name   = "tf-${var.do_cluster_name}-vpc-network"
  region = var.do_region
}

#
# Create kubernetes cluster
#

# Create k8 cluster containing node pool
resource "digitalocean_kubernetes_cluster" "tf-kubernetes-cluster" {
  name   = "tf-${var.do_cluster_name}-k8"
  region = var.do_region

  # Grab the latest version slug from `doctl kubernetes options versions`
  version = "1.18.6-do.0"

  vpc_uuid = digitalocean_vpc.tf-vpc.id

  node_pool {
    name       = "tf-${var.do_cluster_name}-k8-node-pool"
    size       = var.do_k8_droplet_size
    node_count = 1
    tags       = ["tf-node"]
  }
}

resource "digitalocean_project_resources" "tf-kubernetes-cluster-project-resource" {
  project   = digitalocean_project.tf-project.id
  resources = ["do:kubernetes:${digitalocean_kubernetes_cluster.tf-kubernetes-cluster.id}"]
}

#
# Create floating ip for first kubernetes cluster node
#

resource "digitalocean_floating_ip" "tf-public-ip" {
  droplet_id = digitalocean_kubernetes_cluster.tf-kubernetes-cluster.node_pool[0].nodes[0].droplet_id
  region     = var.do_region
}

resource "digitalocean_project_resources" "tf-public-ip-project-resource" {
  project   = digitalocean_project.tf-project.id
  resources = [digitalocean_floating_ip.tf-public-ip.urn]
}


#
# Outputs
#

output "public-ip" {
  value = digitalocean_floating_ip.tf-public-ip.ip_address
}

output "k8-config" {
  value = digitalocean_kubernetes_cluster.tf-kubernetes-cluster.kube_config.0.raw_config
}
