"""Thread-local context for diagram and cluster stacks."""

import threading

_local = threading.local()


def get_current_diagram():
    return getattr(_local, "diagram", None)


def set_current_diagram(diagram):
    _local.diagram = diagram


def get_current_cluster():
    clusters = getattr(_local, "cluster_stack", [])
    return clusters[-1] if clusters else None


def push_cluster(cluster):
    if not hasattr(_local, "cluster_stack"):
        _local.cluster_stack = []
    _local.cluster_stack.append(cluster)


def pop_cluster():
    if hasattr(_local, "cluster_stack") and _local.cluster_stack:
        return _local.cluster_stack.pop()
    return None
