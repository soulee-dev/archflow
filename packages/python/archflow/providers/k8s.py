"""Kubernetes provider convenience functions for Archflow diagrams.

Auto-generated from archflow-icons registry manifest.
Usage:
    from archflow.providers.k8s import Pod, Deployment, Service, Ingress
"""

from archflow.cluster import Cluster as _Cluster
from archflow.node import Node as _Node

_NODES = [
    "api-server",
    "cloud-controller-manager",
    "cluster-role",
    "cluster-role-binding",
    "config-map",
    "control-plane",
    "controller-manager",
    "cron-job",
    "custom-resource-definition",
    "daemon-set",
    "deployment",
    "endpoint",
    "etcd",
    "group",
    "horizontal-pod-autoscaler",
    "ingress",
    "job",
    "kube-proxy",
    "kubelet",
    "limit-range",
    "network-policy",
    "node",
    "persistent-volume",
    "persistent-volume-claim",
    "pod",
    "pod-security-policy",
    "replica-set",
    "resource-quota",
    "role",
    "role-binding",
    "scheduler",
    "secret",
    "service",
    "service-account",
    "stateful-set",
    "storage-class",
    "user",
    "volume",
]

_CLUSTERS = {
    "cluster": "cluster",
    "namespace": "namespace",
}

_UPPER = {"api", "hpa", "crd", "pv", "pvc"}

_FUNC_NAME_OVERRIDES = {
    "etcd": "Etcd",
    "kubelet": "Kubelet",
    "kube-proxy": "KubeProxy",
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
        return _Node(node_id, label, provider="k8s", icon=icon_name, **style)

    factory.__name__ = func_name
    factory.__qualname__ = func_name
    factory.__doc__ = f"Create a Kubernetes {func_name} node."
    return func_name, factory


def _make_cluster_func(name: str, cluster_type: str):
    func_name = _icon_to_func_name(name)
    default_label = func_name

    def factory(label: str = default_label, **style):
        cluster_id = label.lower().replace(" ", "_")
        return _Cluster(cluster_id, label, provider="k8s", cluster_type=cluster_type, **style)

    factory.__name__ = func_name
    factory.__qualname__ = func_name
    factory.__doc__ = f"Create a Kubernetes {func_name} cluster."
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
