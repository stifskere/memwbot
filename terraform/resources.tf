
// ### kubernetes manifests ###

resource "kubernetes_namespace" "memwbot" {
  metadata {
    name = "memwbot"
  }
}

resource "kubernetes_deployment" "portfolio" {
  metadata {
    name = "memwbot"
    namespace = kubernetes_namespace.memwbot.metadata[0].name
    labels = {
      app = "memwbot"
    }
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "memwbot"
      }
    }

    template {
      metadata {
        labels = {
          app = "memwbot"
        }
      }

      spec {
        container {
          name = "memwbot"
          image = var.cluster.deployment_tag
          env {
            name = "DISCORD_TOKEN"
            value = var.app.discord_token
          }
          env {
            name = "GEMINI_TOKEN"
            value = var.app.gemini_token
          }
        }
      }
    }
  }
}
