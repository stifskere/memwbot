
variable "cluster" {
  description = "Target cluster connection information."
  type = object({
    // The kubeconfig file path.
    kubeconfig = string

    // Where the container is deployed.
    deployment_tag = string
  })
}
