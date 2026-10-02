FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG GODOT_VERSION=4.5.1
ARG TARGETARCH

RUN apt-get update \
    && apt-get install -y --no-install-recommends fish jq unzip libfontconfig1 libx11-6 libxcursor1 libxinerama1 libxrandr2 libxi6 libgl1 libasound2t64 \
    && rm -rf /var/lib/apt/lists/*

# Official standard (GDScript) build, for headless workflows on x86_64 and ARM64.
RUN set -eu; \
    arch="${TARGETARCH:-$(dpkg --print-architecture)}"; \
    case "$arch" in amd64) godot_arch=x86_64 ;; arm64) godot_arch=arm64 ;; *) echo "Unsupported architecture: $arch" >&2; exit 1 ;; esac; \
    release="${GODOT_VERSION}-stable"; \
    asset="Godot_v${release}_linux.${godot_arch}"; \
    curl -fL --retry 3 "https://github.com/godotengine/godot/releases/download/${release}/${asset}.zip" -o /tmp/godot.zip; \
    unzip /tmp/godot.zip -d /tmp/godot; \
    install -m 0755 "/tmp/godot/${asset}" /usr/local/bin/godot; \
    rm -rf /tmp/godot /tmp/godot.zip; \
    godot --headless --version

# Match export templates to the engine version.
RUN set -eu; \
    release="${GODOT_VERSION}-stable"; \
    curl -fL --retry 3 "https://github.com/godotengine/godot/releases/download/${release}/Godot_v${release}_export_templates.tpz" -o /tmp/templates.tpz; \
    mkdir -p "/home/vscode/.local/share/godot/export_templates/${GODOT_VERSION}.stable"; \
    unzip /tmp/templates.tpz -d /tmp/godot-templates; \
    mv /tmp/godot-templates/templates/* "/home/vscode/.local/share/godot/export_templates/${GODOT_VERSION}.stable/"; \
    chown -R vscode:vscode /home/vscode/.local; \
    rm -rf /tmp/templates.tpz /tmp/godot-templates
