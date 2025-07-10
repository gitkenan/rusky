# Setting Up Minikube for Rusky

This guide walks you through setting up a local Kubernetes cluster using minikube to deploy and experiment with Rusky.

## Prerequisites

- Docker installed and running
- Basic understanding of containers
- Rust and cargo installed (to build Rusky)

## Installing Minikube

### On WSL/Ubuntu/Debian
```bash
# Download and install minikube
curl -Lo minikube https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube /usr/local/bin/

# Verify installation
minikube version
```

### On macOS
```bash
# Using Homebrew
brew install minikube

# Or download directly
curl -Lo minikube https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
sudo install minikube /usr/local/bin/
```

### On Windows
```bash
# Using Chocolatey
choco install minikube

# Or download the installer from GitHub releases
```

## Installing kubectl

kubectl is the command-line tool for interacting with Kubernetes clusters.

```bash
# On WSL/Ubuntu/Debian
curl -Lo kubectl https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl
sudo install kubectl /usr/local/bin/

# On macOS
brew install kubectl

# Verify installation
kubectl version --client
```

## Starting Your Local Cluster

```bash
# Start minikube with Docker driver
minikube start --driver=docker

# Check cluster status
minikube status

# Check nodes
kubectl get nodes
```

You should see output like:
```
NAME       STATUS   ROLES           AGE   VERSION
minikube   Ready    control-plane   1m    v1.28.3
```

## Useful Minikube Commands

```bash
# Stop the cluster
minikube stop

# Delete the cluster
minikube delete

# SSH into the minikube VM
minikube ssh

# Open the Kubernetes dashboard
minikube dashboard

# Get the IP of your minikube cluster
minikube ip
```

## Next Steps

Once minikube is running, you can:
1. Build the Rusky Docker image
2. Deploy Rusky to your local cluster
3. Experiment with scaling and updates

See the other files in this directory for detailed deployment instructions.