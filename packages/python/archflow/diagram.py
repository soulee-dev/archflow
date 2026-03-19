"""Diagram - top-level container for Archflow diagrams."""

import json
import subprocess

from archflow._context import set_current_diagram


class Diagram:
    def __init__(self, title: str = "Untitled", *, direction: str = "TB", theme: str = "default"):
        self.title = title
        self.direction = direction
        self.theme = theme
        self._nodes = []
        self._clusters = []
        self._edges = []

    def _add_node(self, node):
        self._nodes.append(node)

    def _add_cluster(self, cluster):
        self._clusters.append(cluster)

    def _add_edge(self, edge):
        self._edges.append(edge)

    def __enter__(self):
        set_current_diagram(self)
        return self

    def __exit__(self, *args):
        set_current_diagram(None)

    def to_dict(self) -> dict:
        d = {
            "version": "1.0.0",
            "metadata": {
                "title": self.title,
                "direction": self.direction,
                "theme": self.theme,
            },
            "nodes": [n.to_dict() for n in self._nodes],
            "clusters": [c.to_dict() for c in self._clusters],
            "edges": [e.to_dict() for e in self._edges],
        }
        return d

    def to_json(self, indent: int = 2) -> str:
        return json.dumps(self.to_dict(), indent=indent, ensure_ascii=False)

    def render_svg(self) -> str:
        """Render diagram to SVG string using the Rust core engine.

        Icons are resolved before passing to the Rust renderer.
        """
        from archflow.resolver import IconResolver

        ir_dict = self.to_dict()
        resolver = IconResolver()
        ir_dict = resolver.resolve(ir_dict)
        json_str = json.dumps(ir_dict, ensure_ascii=False)

        try:
            from archflow._archflow_rust import render_svg

            return render_svg(json_str)
        except ImportError:
            # Fallback: use CLI if native module not available
            return self._render_via_cli(json_str=json_str)

    def _render_via_cli(self, json_str: str = None) -> str:
        """Fallback renderer using the CLI binary."""
        import os
        import tempfile

        with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
            f.write(json_str or self.to_json())
            json_path = f.name

        svg_path = json_path.replace(".json", ".svg")
        try:
            subprocess.run(
                ["archflow", "render", json_path, "-o", svg_path],
                check=True,
                capture_output=True,
                text=True,
            )
            with open(svg_path, "r") as f:
                return f.read()
        finally:
            os.unlink(json_path)
            if os.path.exists(svg_path):
                os.unlink(svg_path)

    def save_svg(self, path: str):
        svg = self.render_svg()
        with open(path, "w") as f:
            f.write(svg)
        print(f"Saved SVG to {path}")

    def save_json(self, path: str):
        with open(path, "w") as f:
            f.write(self.to_json())
        print(f"Saved JSON IR to {path}")
