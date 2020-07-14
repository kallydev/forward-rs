# Forward

![GitHub Last Commit](https://img.shields.io/github/last-commit/kallydev/forward?style=flat-square)
![GitHub License](https://img.shields.io/github/license/kallydev/forward?style=flat-square)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/kallydev/forward/Build%20release)](https://github.com/kallydev/forward/actions)

A TCP reverse proxy server.

## Instructions

> Non-production environment version.

Execution parameter format.

```bash
./forward <client_addr> <server_addr>
```

Usage example.

```bash
./forward 0.0.0.0:8080 127.0.0.1:80
```

## Todo

- [ ] UDP forward
- [ ] Config support

## License

Copyright (c) KallyDev. All rights reserved.

Licensed under the [MIT license](LICENSE).
