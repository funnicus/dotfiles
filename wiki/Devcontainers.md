# Zed devcontainer starters for chezmoi

## Use

From a project's root directory:

```fish
devcontainer-init node
# or: devcontainer-init azure-node
# or: devcontainer-init godot
```

The function refuses to overwrite an existing `.devcontainer`, including an empty
directory. Commit the resulting `.devcontainer/` to that project's repository.
Copies are independent: later changes to your starter do not change existing projects.
For Bash/Zsh, copy the chosen template directory to `<project>/.devcontainer` manually.

Install Docker (with a running daemon) or Podman on the host. For Podman, merge
`"use_podman": true` into Zed settings. Open the project and choose **Open in
Container**. After changing container configuration, stop the existing container
and reopen it in Zed. No fixed workspace path is set: the devcontainer tooling
uses its default project mount and working directory together.

## Included environments

| Starter | Tools | Forwarded ports |
| --- | --- | --- |
| node | Node 22, npm, TypeScript from the base image, Git, Fish, jq | 3000, 5173 |
| azure-node | Node starter plus Functions Core Tools 4, Azurite, Azure CLI, Terraform and TFLint | 7071, 10000–10002 |
| godot | Godot 4.5.1 standard build, matching export templates, Git, Fish, jq | None |

Node 22 is a conservative shared baseline. Change the Dockerfile image tag to
match your project's Node version. Images and feature major tags receive updates;
these starters are not fully locked builds. Pin exact feature versions and image
digests if your team needs reproducible builds. Project packages remain governed
by your project's lockfile.

### Node

Run `npm ci` for an npm project with a lockfile (or `npm install` initially).
Use your project's scripts to start it. Bind web servers to `0.0.0.0`, for example
`npm run dev -- --host 0.0.0.0` for Vite. Dependencies are not installed automatically.
If the host and container use different architectures, keep separate
`node_modules` or reinstall dependencies inside the container.

### Azure / Terraform

```sh
az login --use-device-code
az account set --subscription '<subscription-id>'
npm ci
func start
```

For local storage, start `azurite --blobHost 0.0.0.0 --queueHost 0.0.0.0
--tableHost 0.0.0.0 --location /tmp/azurite` in a second terminal. Azurite data in
`/tmp` and Azure login state are lost when the container is recreated. Local
Function configuration such as `local.settings.json` belongs in the project and
should be ignored by Git when it contains secrets. Authenticate inside the
container; no host credentials or Docker socket are mounted.

Terraform commands run normally: `terraform init`, `terraform validate`, and
`terraform plan`. The Terraform feature installs the latest tools at build time;
pin the feature options to your team's required versions before sharing.

### Godot

This is a GDScript/headless environment, not a graphical editor or .NET setup.
Use the host's Godot editor for scenes, visual work, and interactive playtesting.
Match its version to `GODOT_VERSION` in the Dockerfile (4.5.1 is a starter pin,
not a claim that it is the latest release). Both AMD64 and ARM64 builds are selected
automatically. Export templates are large, so the initial build takes longer.

```sh
godot --headless --path . --editor --import
mkdir -p build
godot --headless --path . --export-release 'Linux' build/game.x86_64
```

Exporting requires an existing `export_presets.cfg`; replace `Linux` with your
preset's exact name. Ignore `.godot/` and generated build output in your project's
Git configuration. Zed's Godot language-server integration needs a running Godot
editor process; this template does not start or configure that process.

## Verification

JSON and archive contents were checked during creation. Docker and Fish were
unavailable in the creation environment, so container builds and the Fish function
have not been executed. After opening a container, check `node --version`,
`func --version`, `az version`, `terraform version`, or `godot --headless --version`
as appropriate.

## References

- https://zed.dev/docs/dev-containers
- https://github.com/devcontainers/features/tree/main/src/azure-cli
- https://github.com/devcontainers/features/tree/main/src/terraform
- https://github.com/godotengine/godot/releases/tag/4.5.1-stable
