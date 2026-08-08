# MinecraftHealthCheck

A simple health checker for Minecraft servers, mainly designed for:

- [Endkind/papermc](https://github.com/Endkind/papermc)
- [Endkind/folia](https://github.com/Endkind/folia)
- [Endkind/velocity](https://github.com/Endkind/velocity)
- [Endkind/waterfall](https://github.com/Endkind/waterfall)

## Environment Variables

You can customize MinecraftHealthCheck by setting the following environment variables:

* `HEALTHCHECK_HOST` (default: `127.0.0.1`) - The Minecraft server address.
* `HEALTHCHECK_PORT` (default: `25565`) - The Minecraft server port.
* `HEALTHCHECK_ATTEMPTS` (default: `5`, minimum: `1`) - The number of attempts before the Minecraft server is considered unreachable.
* `HEALTHCHECK_DELAY` (default: `5.0`) - The delay in seconds between each attempt.
* `HEALTHCHECK_DEBUG_MESSAGES` (default: `false`) - Shows the current attempt count and a success message.
