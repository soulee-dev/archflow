"""Cluster definition for Archflow diagrams."""

from archflow._context import get_current_diagram, push_cluster, pop_cluster


class Cluster:
    def __init__(self, id: str, label: str = None, **style):
        self.id = id
        self.label = label or id
        self.children = []
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
        if self.style:
            d["style"] = self.style
        return d
