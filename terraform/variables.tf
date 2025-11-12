
variable "cluster" {
  description = "Target cluster connection information."
  type = object({
    // The kubeconfig file path.
    kubeconfig = string

    // Where the container is deployed.
    deployment_tag = string
  })
}

variable "app" {
  description = "Application environment variables."
  type = object({
    // The discord application token.
    discord_token = string

    // The gemini application token.
    gemini_token = string
  })
}
