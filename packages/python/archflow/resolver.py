"""Icon resolver for Archflow diagrams.

Resolution chain (per node):
  1. icon_svg already set → skip
  2. Local icons dir: ~/.archflow/icons/{provider}/nodes/{icon}.svg
  3. Disk cache hit: ~/.cache/archflow/icons/{hash}.svg
  4. Central registry: default archflow-icons CDN (always available as fallback)
  5. User-specified icon_sources (custom repos/CDNs)
  6. Cache fetched results to disk

Local dir structure (managed by future `archflow icons add`):
  ~/.archflow/icons/
    aws/
      nodes/ec2.svg
      nodes/rds.svg
    generic/
      nodes/server.svg
"""

import hashlib
import re
import urllib.request
from pathlib import Path

# Central registry — always tried as fallback after local, before custom sources
DEFAULT_REGISTRY = "https://raw.githubusercontent.com/soulee-dev/archflow-icons/main"


def _local_icons_dir() -> Path:
    """~/.archflow/icons/ — managed by `archflow icons add` CLI."""
    return Path.home() / ".archflow" / "icons"


def _cache_dir() -> Path:
    """~/.cache/archflow/icons/ — transparent fetch cache."""
    d = Path.home() / ".cache" / "archflow" / "icons"
    d.mkdir(parents=True, exist_ok=True)
    return d


def _cache_key(url: str) -> str:
    return hashlib.sha256(url.encode()).hexdigest() + ".svg"


def _sanitize_svg(svg: str) -> str:
    """Remove script tags and event handlers from SVG content."""
    svg = re.sub(r"<script[\s\S]*?</script>", "", svg, flags=re.IGNORECASE)
    svg = re.sub(r"\bon\w+\s*=\s*[\"'][^\"']*[\"']", "", svg, flags=re.IGNORECASE)
    return svg


def _read_local(provider: str, icon_name: str) -> str | None:
    """Step 2: Look up from local icons dir."""
    path = _local_icons_dir() / provider / "nodes" / f"{icon_name}.svg"
    if path.is_file():
        svg = path.read_text(encoding="utf-8")
        return _sanitize_svg(svg)
    return None


def _read_cache(url: str) -> str | None:
    """Step 3: Check disk cache for a previously fetched URL."""
    cached = _cache_dir() / _cache_key(url)
    if cached.exists():
        return cached.read_text(encoding="utf-8")
    return None


def _write_cache(url: str, content: str) -> None:
    """Step 6: Save fetched content to disk cache."""
    cached = _cache_dir() / _cache_key(url)
    cached.write_text(content, encoding="utf-8")


def _fetch_url(url: str) -> str | None:
    """Fetch SVG from URL. Checks disk cache first, writes cache on success."""
    # Cache hit
    cached = _read_cache(url)
    if cached is not None:
        return cached

    # Network fetch
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "archflow-icon-resolver/1.0"})
        with urllib.request.urlopen(req, timeout=10) as resp:
            content = resp.read().decode("utf-8")
        content = _sanitize_svg(content)
        _write_cache(url, content)
        return content
    except Exception:
        return None


def _source_to_base_url(source: str) -> str | None:
    """Convert an icon_source string to a base URL."""
    m = re.match(r"^github:(.+/.+)$", source)
    if m:
        return f"https://raw.githubusercontent.com/{m.group(1)}/main"
    if source.startswith("https://") or source.startswith("http://"):
        return source.rstrip("/")
    return None


def _resolve_from_url(base_url: str, provider: str, icon_name: str) -> str | None:
    """Try to fetch an icon from a base URL."""
    url = f"{base_url}/{provider}/nodes/{icon_name}.svg"
    return _fetch_url(url)


class IconResolver:
    """Resolves icon references in an IR dict to inline SVG content.

    Resolution chain per node:
      1. icon_svg already set → skip
      2. Local: ~/.archflow/icons/{provider}/nodes/{icon}.svg
      3. Disk cache hit (from prior fetch)
      4. Central registry (archflow-icons default CDN)
      5. Custom icon_sources from metadata
      6. Fetched results cached to disk for next time

    Also applies provider styles from registry manifests:
      - cluster_styles: stroke, fill, dasharray, corner_radius
      - node_render_mode: "icon_only" (no box) or "default" (box + icon)
    """

    def __init__(self, *, registry: str = DEFAULT_REGISTRY, local_dir: Path | None = None):
        self.registry = registry
        self.local_dir = local_dir  # override for testing
        self._manifest_cache: dict[str, dict] = {}

    def _load_manifest(self, provider: str) -> dict:
        """Load provider manifest from local or registry. Cached per session."""
        if provider in self._manifest_cache:
            return self._manifest_cache[provider]

        import json

        # Try local
        base = self.local_dir if self.local_dir else _local_icons_dir()
        local_mf = base / provider / "manifest.json"
        if local_mf.is_file():
            manifest = json.loads(local_mf.read_text(encoding="utf-8"))
            self._manifest_cache[provider] = manifest
            return manifest

        # Try registry
        if self.registry:
            url = f"{self.registry}/{provider}/manifest.json"
            content = _fetch_url(url)
            if content:
                try:
                    manifest = json.loads(content)
                    self._manifest_cache[provider] = manifest
                    return manifest
                except json.JSONDecodeError:
                    pass

        self._manifest_cache[provider] = {}
        return {}

    def _apply_cluster_styles(self, ir_dict: dict) -> None:
        """Apply cluster_styles from provider manifest to clusters without explicit style."""
        for cluster in ir_dict.get("clusters", []):
            provider = cluster.get("provider")
            cluster_type = cluster.get("cluster_type")
            if not provider or not cluster_type:
                continue
            # Don't override explicit style
            if cluster.get("style"):
                continue

            manifest = self._load_manifest(provider)
            cluster_styles = manifest.get("cluster_styles", {})
            preset = cluster_styles.get(cluster_type)
            if preset:
                cluster["style"] = {k: v for k, v in preset.items() if v is not None}

    def _apply_node_render_mode(self, ir_dict: dict) -> None:
        """Set node_render_mode from provider manifest into metadata."""
        providers_seen = set()
        for node in ir_dict.get("nodes", []):
            p = node.get("provider")
            if p:
                providers_seen.add(p)

        # Collect render modes
        render_modes = {}
        for provider in providers_seen:
            manifest = self._load_manifest(provider)
            mode = manifest.get("node_render_mode")
            if mode:
                render_modes[provider] = mode

        if render_modes:
            metadata = ir_dict.setdefault("metadata", {})
            metadata["node_render_modes"] = render_modes

    def _read_local(self, provider: str, icon_name: str, subdir: str = "nodes") -> str | None:
        """Step 2: Local icons directory."""
        base = self.local_dir if self.local_dir else _local_icons_dir()
        path = base / provider / subdir / f"{icon_name}.svg"
        if path.is_file():
            svg = path.read_text(encoding="utf-8")
            return _sanitize_svg(svg)
        return None

    def _resolve_one(
        self, provider: str, icon_name: str, base_url: str | None, subdir: str = "nodes"
    ) -> str | None:
        """Run the full resolution chain for a single icon."""
        # Step 2: Local
        svg = self._read_local(provider, icon_name, subdir)
        if svg:
            return svg

        # Step 3: Explicit source (from "use ... from ...")
        if base_url:
            url = f"{base_url}/{provider}/{subdir}/{icon_name}.svg"
            svg = _fetch_url(url)
            if svg:
                return svg

        # Step 4: Default registry fallback
        if self.registry and base_url != self.registry:
            url = f"{self.registry}/{provider}/{subdir}/{icon_name}.svg"
            svg = _fetch_url(url)
            if svg:
                return svg

        return None

    def resolve(self, ir_dict: dict) -> dict:
        """Resolve all icon references and apply provider styles in-place."""
        # Build provider → base_url map from provider_sources
        provider_sources = ir_dict.get("metadata", {}).get("provider_sources", {})

        # Resolve base URLs
        declared_providers: dict[str, str | None] = {}
        for provider, source in provider_sources.items():
            if source:
                declared_providers[provider] = _source_to_base_url(source)
            else:
                declared_providers[provider] = None  # use default registry

        # Only resolve declared providers
        if not declared_providers:
            return ir_dict

        # Apply provider styles from manifests
        self._apply_cluster_styles(ir_dict)
        self._apply_node_render_mode(ir_dict)

        for node in ir_dict.get("nodes", []):
            if node.get("icon_svg"):
                continue

            icon = node.get("icon")
            provider = node.get("provider")

            # Only resolve if provider is declared with "use"
            if provider and provider not in declared_providers:
                continue

            base_url = declared_providers.get(provider)

            if icon and (icon.startswith("https://") or icon.startswith("http://")):
                svg = _fetch_url(icon)
                if svg:
                    node["icon_svg"] = svg
            elif provider and icon:
                svg = self._resolve_one(provider, icon, base_url)
                if svg:
                    node["icon_svg"] = svg
            elif provider and not icon:
                node_id = node.get("id", "")
                svg = self._resolve_one(provider, node_id, base_url)
                if svg:
                    node["icon_svg"] = svg

        # Resolve cluster icons
        for cluster in ir_dict.get("clusters", []):
            if cluster.get("icon_svg"):
                continue

            provider = cluster.get("provider")
            cluster_type = cluster.get("cluster_type")

            if provider and provider not in declared_providers:
                continue

            if provider and cluster_type:
                base_url = declared_providers.get(provider)
                svg = self._resolve_one(provider, cluster_type, base_url, subdir="clusters")
                if svg:
                    cluster["icon_svg"] = svg

        return ir_dict
