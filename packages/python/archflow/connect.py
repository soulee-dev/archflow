"""Convenience function for connecting nodes."""

from archflow._context import get_current_diagram
from archflow.edge import Edge


def connect(*nodes, label: str = None):
    """Connect nodes in sequence: connect(a, b, c) creates edges a->b and b->c."""
    edges = []
    for i in range(len(nodes) - 1):
        edge = Edge(nodes[i], nodes[i + 1], label=label)
        diagram = get_current_diagram()
        if diagram is not None:
            diagram._add_edge(edge)
        edges.append(edge)
    return edges
