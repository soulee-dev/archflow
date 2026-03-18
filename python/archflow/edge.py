"""Edge definition for Archflow diagrams."""


class Edge:
    def __init__(self, source, target, label: str = None, **style):
        self.source = source
        self.target = target
        self.label = label
        self.style = style if style else None

    def to_dict(self) -> dict:
        d = {
            "from": self.source.id if hasattr(self.source, "id") else str(self.source),
            "to": self.target.id if hasattr(self.target, "id") else str(self.target),
        }
        if self.label:
            d["label"] = self.label
        if self.style:
            d["style"] = self.style
        return d
