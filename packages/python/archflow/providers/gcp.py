"""GCP provider convenience functions for Archflow diagrams.

Auto-generated from archflow-icons registry manifest.
Usage:
    from archflow.providers.gcp import ComputeEngine, CloudSQL, BigQuery, GKE
"""

from archflow.cluster import Cluster
from archflow.node import Node

# ── Node definitions from gcp manifest (19 core products) ──
_NODES = [
    "ai-hypercomputer",
    "alloydb",
    "anthos",
    "apigee",
    "bigquery",
    "cloud-run",
    "cloud-spanner",
    "cloud-sql",
    "cloud-storage",
    "compute-engine",
    "distributed-cloud",
    "gke",
    "hyperdisk",
    "looker",
    "mandiant",
    "security-command-center",
    "security-operations",
    "threat-intelligence",
    "vertex-ai",
]

# ── Cluster types ──
_CLUSTERS = {
    "region": "region",
    "vpc": "vpc",
    "subnet": "subnet",
    "project": "project",
    "zone": "zone",
}

_UPPER = {"gke", "sql", "ai", "vpc", "dns", "db"}

_FUNC_NAME_OVERRIDES = {
    "bigquery": "BigQuery",
    "alloydb": "AlloyDB",
}


def _icon_to_func_name(icon: str) -> str:
    if icon in _FUNC_NAME_OVERRIDES:
        return _FUNC_NAME_OVERRIDES[icon]
    parts = icon.replace("_", "-").split("-")
    result = []
    for part in parts:
        if part in _UPPER:
            result.append(part.upper())
        else:
            result.append(part.capitalize())
    return "".join(result)


def _make_node_func(icon_name: str):
    func_name = _icon_to_func_name(icon_name)
    default_label = func_name

    def factory(label: str = default_label, **style):
        node_id = label.lower().replace(" ", "_")
        return Node(node_id, label, provider="gcp", icon=icon_name, **style)

    factory.__name__ = func_name
    factory.__qualname__ = func_name
    factory.__doc__ = f"Create a GCP {func_name} node."
    return func_name, factory


def _make_cluster_func(name: str, cluster_type: str):
    func_name = _icon_to_func_name(name)
    default_label = func_name

    def factory(label: str = default_label, **style):
        cluster_id = label.lower().replace(" ", "_")
        return Cluster(cluster_id, label, provider="gcp", cluster_type=cluster_type, **style)

    factory.__name__ = func_name
    factory.__qualname__ = func_name
    factory.__doc__ = f"Create a GCP {func_name} cluster."
    return func_name, factory


__all__ = []

for _icon in _NODES:
    _name, _func = _make_node_func(_icon)
    globals()[_name] = _func
    __all__.append(_name)

for _cname, _ctype in _CLUSTERS.items():
    _name, _func = _make_cluster_func(_cname, _ctype)
    if _name not in globals():
        globals()[_name] = _func
        __all__.append(_name)
