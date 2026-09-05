# Demo Network

This directory contains a tiny container-based topology for checking that the
local Holo images work together on macOS through Docker Desktop.

Topology:

- `r1`: Holo router connected to the `left` and `transit` networks.
- `r2`: Holo router connected to the `transit` network.
- `host`: simple utility container on the `left` network for ping tests and
  CLI access.

Build the local images first from the repository root:

```sh
./docker/build.sh --profile dev
```

The first build compiles the Rust workspace inside a Linux container. Later
builds reuse the Cargo cache and are much faster. On Apple Silicon, Docker
Desktop emulates amd64 because the published `holo-cli` image is amd64.

Then bring the demo up:

```sh
cd docker/demo
docker compose up -d
```

Smoke tests:

```sh
docker compose exec host ping -c 1 172.31.0.3
docker compose exec host holo-cli -a http://172.30.0.2:50051 --help
```

Bring it down with:

```sh
docker compose down -v
```