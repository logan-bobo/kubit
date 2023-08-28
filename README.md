# Kubit

## The kubecfg operator

The `kubit` operator is a Kubernetes controller that can render and apply jsonnet templates based on the [kubecfg](https://github.com/kubecfg/kubecfg) jsonnet tooling/framework.


## Installation

### Kubernetes controller

```bash
kubectl apply -k 'https://github.com/kubecfg/kubit//kustomize/global?ref=v0.0.9'
```

The Kubernetes controller is the main way to use kubit.

### CLI tool

#### Homebrew

(popular on macos, but also available on linux)

```bash
brew install kubecfg/kubit/kubit
```

#### cargo install

Direct install from sources:

```bash
cargo install --git https://github.com/kubecfg/kubit/ --tag v0.0.9
```

The CLI is an optional tool that provides helpers and alternative ways to install and inspect packages.

## Usage

### Install a package

1. Install the kubit operator once
2. Apply a CR that references a package OCI artifact

Example `foo.yaml` CR:

```yaml
apiVersion: kubecfg.dev/v1alpha1
kind: AppInstance
metadata:
  name: foo
  namespace: myns
spec:
  package:
    image: ghcr.io/kubecfg/demo:v0.1.0
    apiVersion: demo/v1alpha1
    spec:
      bar: baz
```

Such a CR can be applied using standard Kubernetes tooling such as [kubectl](https://kubernetes.io/docs/tasks/tools/#kubectl),
or [ArgoCD](https://argoproj.github.io/cd/):

```bash
kubectl apply -f foo.yaml
```

### Observe an application instance

The controller will continuously attempt to reconcile the desired state of the application instance
and update the outcome of the reconciliation in the `status` field of the `AppInstance` custom resource.

You can observe the `status` field of the `AppInstance` resource using standard Kubernetes tooling such as:

```bash
kubectl get -f foo.yaml -o json | jq .status
```

TIP: render logs in more readable format with:

```bash
kubectl get -f foo.yaml -o json | jq -r '.status.lastLogs|to_entries[] | "\(.key): \(.value)"'
```

### Creating a new package

The `kubecfg pack` command can be used to take a jsonnet file and all its dependencies and push them
all together as a bundle into an OCI artifact.

```bash
kubecfg pack ghcr.io/kubecfg/demo:v0.1.0 demo.jsonnet
```

### Installing packages manually

You can run the same logic that the kubit controller does when rendering and applying a template by running
the `kubit` CLI tool from your laptop:

```bash
kubit local apply foo.yaml
```

Kubit is just a relatively thin wrapper on top of `kubecfg`.
For increased compatibility, it uses standard `kubectl apply` to apply the manifests using more standard
tooling rather than kubecfg's integrated k8s API.

You can preview the actuall commands that `kubit` will run with:

```bash
kubit local apply foo.yaml --dry-run=script
```

Other interesting options are `--dry-run=render` and `--dry-run=diff` which will respectively just render the YAML without applying it
and rendering + diffing the manifests against a running application. This can be useful to preview effects of changes in the spec or
between versions of a package

### Trying local package changes

Sometimes you'd like to try out some jsonnet code before you package it up and publish to your OCI registry:

```bash
kubit local apply foo.yaml --dry-run=diff --package-image file://$HOME/my-project/my-main.jsonnet
```

## Development

### Without in-cluster controller

Create Kubernetes resources:

```bash
kubectl apply -k ./kustomize/local
```

The manifests in `./kustomize/local` are like `./kustomize/global` but don't spawn the kubit controller.

Build and run the controller locally:

```bash
cargo run -- --as system:serviceaccount:kubit:kubit
```

### Co-exist with in-cluster controller

If you already installed kubit (e.g. with `kubectl apply -k ./kustomize/global`) in your test cluster but you still want to quickly run the locally built kubit controller without uninstalling the in-cluster controller you can _pause_ an appinstance and run the local controller with `--only-paused`:

```bash
kubectl patch -f foo.yaml --patch '{"spec":{"pause": true}}' --type merge 
```

Then you can run the controller locally and have it process **only** the resource you paused:

```bash
cargo run -- --as system:serviceaccount:kubit:kubit --only-paused
```

To unpause the resource:

```bash
kubectl patch -f foo.yaml --patch '{"spec":{"pause": false}}' --type merge 
```
