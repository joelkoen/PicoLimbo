# PicoLimbo

> [!WARNING]
> This software is highly experimental. Use at your own risks and report any bugs by submitting an issue on GitHub.

An attempt at writing a lightweight Minecraft server from scratch in Rust. Supports 1.20 and above.

This is intended to be used as an AFK or waiting server.

## Getting Started

### Pterodactyl

To use in the Pterodactyl panel, you can import the [egg file](./pterodactyl/egg-pico-limbo.json) into your panel.

### Using Docker

```shell
docker run --rm -p "25565:25565" ghcr.io/quozul/picolimbo:master
```

### Using Docker Compose

A sample [docker-compose.yml file](./docker-compose.yml) is available in the repository.
