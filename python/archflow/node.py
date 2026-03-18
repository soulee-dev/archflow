"""Node definition for Archflow diagrams."""

from archflow._context import get_current_diagram, get_current_cluster


class Node:
    def __init__(self, id: str, label: str = None, *, provider: str = None, icon: str = None, **style):
        self.id = id
        self.label = label or id
        self.provider = provider
        self.icon = icon
        self.style = style if style else None

        # Auto-register with current diagram
        diagram = get_current_diagram()
        if diagram is not None:
            diagram._add_node(self)

        # Auto-register with current cluster
        cluster = get_current_cluster()
        if cluster is not None:
            cluster._add_child(self.id)

    def __rshift__(self, other):
        """a >> b creates an edge from a to b and returns b for chaining."""
        from archflow.edge import Edge
        from archflow._context import get_current_diagram

        if isinstance(other, Node):
            edge = Edge(self, other)
            diagram = get_current_diagram()
            if diagram is not None:
                diagram._add_edge(edge)
            return other
        elif isinstance(other, list):
            for node in other:
                edge = Edge(self, node)
                diagram = get_current_diagram()
                if diagram is not None:
                    diagram._add_edge(edge)
            return other
        return NotImplemented

    def to_dict(self) -> dict:
        d = {"id": self.id, "label": self.label}
        if self.provider:
            d["provider"] = self.provider
        if self.icon:
            d["icon"] = self.icon
        if self.style:
            d["style"] = self.style
        return d
