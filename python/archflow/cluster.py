"""Cluster definition for Archflow diagrams."""

from archflow._context import get_current_diagram, pop_cluster, push_cluster


class Cluster:
    def __init__(
        self, id: str, label: str = None, *, provider: str = None, cluster_type: str = None, **style
    ):
        self.id = id
        self.label = label or id
        self.children = []
        self.provider = provider
        self.cluster_type = cluster_type
        self.style = style if style else None

        # Auto-register with current diagram
        diagram = get_current_diagram()
        if diagram is not None:
            diagram._add_cluster(self)

    def _add_child(self, child_id: str):
        self.children.append(child_id)

    def __enter__(self):
        push_cluster(self)
        return self

    def __exit__(self, *args):
        pop_cluster()

    def to_dict(self) -> dict:
        d = {
            "id": self.id,
            "label": self.label,
            "children": self.children,
        }
        if self.provider:
            d["provider"] = self.provider
        if self.cluster_type:
            d["cluster_type"] = self.cluster_type
        if self.style:
            d["style"] = self.style
        return d
