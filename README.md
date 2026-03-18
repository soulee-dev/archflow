<h1 align="center">
  Archflow
</h1>

<p align="center">
  <strong>Architecture diagrams that live in your codebase, not in your browser.</strong>
</p>

<p align="center">
  <a href="https://github.com/soulee-dev/archflow/actions"><img src="https://github.com/soulee-dev/archflow/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/soulee-dev/archflow/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://soulee-dev.github.io/archflow/"><img src="https://img.shields.io/badge/playground-live-brightgreen" alt="Playground"></a>
</p>

<p align="center">
  <a href="https://soulee-dev.github.io/archflow/">Playground</a> &middot;
  <a href="#quick-start">Quick Start</a> &middot;
  <a href="#examples">Examples</a> &middot;
  <a href="#providers">Providers</a> &middot;
  <a href="#themes">Themes</a>
</p>

---

**Stop dragging boxes around.** Write your architecture in Python or DSL, get publication-ready SVGs rendered by a Rust engine in milliseconds.

```python
from archflow import Diagram, Node, Cluster

with Diagram("Web Service", direction="LR") as d:
    client = Node("client", "Client")

    with Cluster("vpc", "VPC"):
        lb = Node("lb", "Load Balancer")
        api = Node("api", "API Server")
        db = Node("db", "PostgreSQL")

    client >> lb >> api >> db
    d.save_svg("architecture.svg")
```

<p align="center">
  <img src="examples/web_service.svg" alt="Web Service Architecture" width="720">
</p>

## Why Archflow?

- **Zero external dependencies** - No Graphviz, no system packages. Just `pip install` and go.
- **Rust-powered, millisecond rendering** - Own layout engine. No subprocess calls, no waiting.
- **Deterministic** - Same code always produces the exact same SVG. No layout jitter between runs.
- **SVG-native** - Vector output by default. Crisp at any zoom, embeddable anywhere.
- **Language-agnostic** - JSON IR means any language can generate diagrams. Python, TypeScript, Go — same engine.
- **Runs in the browser** - Full rendering via WebAssembly. Try diagrams without installing anything.
- **Pluggable icon registry** - 300+ AWS, 19+ GCP, 39 K8s icons. Add your own provider with a manifest + SVGs.
- **6 built-in themes** - Beautiful by default, fully customizable.

### vs. diagrams

[diagrams](https://github.com/mingrammer/diagrams) is the closest alternative. Key differences:

| | diagrams | Archflow |
|---|---|---|
| Rendering | Graphviz (external C binary) | Rust (self-contained, no system deps) |
| Output | PNG (raster) | SVG (vector) |
| Speed | Subprocess per render | Milliseconds (native/WASM) |
| Runtime | Python only | Python, CLI, WASM, any language via JSON IR |
| Browser | Not possible | Full WASM playground |
| Icons | Bundled in package | External registry (pluggable, cacheable) |
| Layout | Graphviz `dot` | Own topological sort (deterministic) |

## Quick Start

### Install from source

```bash
# Python library (with native Rust FFI)
cd bindings/python && pip install maturin && maturin develop

# CLI
cargo build --release -p archflow-cli
```

### Your first diagram

```python
from archflow import Diagram, Node, Cluster

with Diagram("Microservices", direction="LR") as d:
    mobile = Node("mobile", "Mobile App")

    with Cluster("k8s", "Kubernetes Cluster"):
        gw = Node("gw", "API Gateway")
        with Cluster("svc", "Services"):
            auth = Node("auth", "Auth")
            user = Node("user", "User")
            order = Node("order", "Order")

    mobile >> gw >> auth
    gw >> user
    gw >> order

    d.save_svg("microservices.svg")
```

### DSL (Playground)

```
title: Web Service
direction: LR
use aws

aws:ELB Load Balancer >> aws:EC2 Web Server >> aws:RDS Database

cluster:aws:vpc Production VPC {
  aws:EC2 Web Server
  aws:RDS Database
}
```

### CLI

```bash
archflow render diagram.json -o output.svg
```

## Examples

### Data Pipeline

```python
from archflow import Diagram, Node, Cluster

with Diagram("Data Pipeline", direction="LR") as d:
    with Cluster("sources", "Data Sources"):
        api = Node("api", "API Logs")
        click = Node("click", "Clickstream")

    with Cluster("process", "Processing"):
        kafka = Node("kafka", "Kafka")
        spark = Node("spark", "Spark")

    with Cluster("store", "Storage"):
        s3 = Node("s3", "S3 Data Lake")
        redshift = Node("redshift", "Redshift")

    api >> kafka >> spark >> s3 >> redshift
    click >> kafka

    d.save_svg("data_pipeline.svg")
```

<p align="center">
  <img src="examples/data_pipeline.svg" alt="Data Pipeline" width="720">
</p>

### AWS with Provider Icons

```python
from archflow import Diagram
from archflow.providers.aws import EC2, RDS, ELB, S3, VPC

with Diagram("AWS Architecture") as d:
    with VPC("Production"):
        lb = ELB("Load Balancer")
        web = EC2("Web Server")
        db = RDS("Database")
        storage = S3("Assets")

    lb >> web >> db
    web >> storage
    d.save_svg("aws.svg")
```

## Providers

Icons are loaded from the [archflow-icons](https://github.com/soulee-dev/archflow-icons) registry.

### AWS (307 nodes, 11 clusters)

```python
from archflow.providers.aws import EC2, Lambda, RDS, S3, DynamoDB, ELB, CloudFront, SQS, SNS
```

Cluster types: `Region`, `VPC`, `Subnet`

### GCP (19 nodes)

```python
from archflow.providers.gcp import ComputeEngine, CloudSQL, BigQuery, GKE, CloudRun, VertexAI
```

Cluster types: `Region`, `VPC`, `Subnet`, `Project`, `Zone`

### Kubernetes (39 nodes)

```python
from archflow.providers.k8s import Pod, Deployment, Service, Ingress, StatefulSet, ConfigMap, Secret
```

Cluster types: `Cluster`, `Namespace`

## Themes

6 built-in themes:

```python
with Diagram("My Diagram", theme="dark") as d:      # dark mode
with Diagram("My Diagram", theme="ocean") as d:      # blue tones
with Diagram("My Diagram", theme="minimal") as d:    # clean outlines
with Diagram("My Diagram", theme="sunset") as d:     # warm tones
with Diagram("My Diagram", theme="forest") as d:     # green tones
with Diagram("My Diagram", theme="default") as d:    # professional
```

Custom theme overrides:

```python
with Diagram("Custom", custom_theme={
    "background": "#0D1117",
    "node_palette": [{"fill": "#58A6FF", "stroke": "#388BFD"}],
    "node_shadow": False,
}) as d:
    ...
```

## Architecture

```
                    Python DSL / JSON IR
                           |
                     Validation
                           |
                   Layout (Kahn's topological sort)
                           |
                   Theme Resolution
                           |
                   Scene Graph
                           |
                       SVG Output
```

| Path | Purpose |
|------|---------|
| `crates/archflow-core` | DSL parser, layout, themes, scene graph, SVG renderer |
| `crates/archflow-cli` | `archflow render` command |
| `crates/archflow-lsp` | Language Server Protocol for editor support |
| `bindings/python-ffi` | Python native bindings via PyO3 |
| `bindings/wasm` | WebAssembly build for the browser playground |
| `bindings/python` | Python SDK (Diagram, Node, Cluster, providers) |
| `apps/vscode` | VS Code extension |

### JSON IR

The language-neutral intermediate representation means you can generate diagrams from **any language**:

```json
{
  "version": "1.0.0",
  "metadata": { "title": "Web Service", "direction": "LR", "theme": "default" },
  "nodes": [
    { "id": "web", "label": "Web Server" },
    { "id": "db", "label": "Database" }
  ],
  "edges": [{ "from": "web", "to": "db" }]
}
```

## Development

```bash
# Run tests
cargo test

# Lint
cargo clippy && cargo fmt --check
ruff check bindings/python/ && ruff format --check bindings/python/

# Build WASM for playground
wasm-pack build bindings/wasm --target web --out-dir ../../docs/pkg
```

## License

[MIT](LICENSE)
