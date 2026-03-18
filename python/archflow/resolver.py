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
    """

    def __init__(self, *, registry: str = DEFAULT_REGISTRY, local_dir: Path | None = None):
        self.registry = registry
        self.local_dir = local_dir  # override for testing

    def _read_local(self, provider: str, icon_name: str) -> str | None:
        """Step 2: Local icons directory."""
        base = self.local_dir if self.local_dir else _local_icons_dir()
        path = base / provider / "nodes" / f"{icon_name}.svg"
        if path.is_file():
            svg = path.read_text(encoding="utf-8")
            return _sanitize_svg(svg)
        return None

    def _resolve_one(self, provider: str, icon_name: str, sources: list[str]) -> str | None:
        """Run the full resolution chain for a single icon."""
        # Step 2: Local
        svg = self._read_local(provider, icon_name)
        if svg:
            return svg

        # Step 3+4: Central registry (cache checked inside _fetch_url)
        if self.registry:
            svg = _resolve_from_url(self.registry, provider, icon_name)
            if svg:
                return svg

        # Step 5: Custom icon_sources
        for source in sources:
            base_url = _source_to_base_url(source)
            if base_url:
                svg = _resolve_from_url(base_url, provider, icon_name)
                if svg:
                    return svg

        return None

    def resolve(self, ir_dict: dict) -> dict:
        """Resolve all icon references in-place and return the dict."""
        sources = ir_dict.get("metadata", {}).get("icon_sources", [])

        for node in ir_dict.get("nodes", []):
            # Step 1: Already resolved
            if node.get("icon_svg"):
                continue

            icon = node.get("icon")
            provider = node.get("provider")

            if icon and (icon.startswith("https://") or icon.startswith("http://")):
                # Direct URL — skip chain, just fetch
                svg = _fetch_url(icon)
                if svg:
                    node["icon_svg"] = svg
            elif provider and icon:
                svg = self._resolve_one(provider, icon, sources)
                if svg:
                    node["icon_svg"] = svg
            elif provider and not icon:
                # Fallback: use node id as icon name
                node_id = node.get("id", "")
                svg = self._resolve_one(provider, node_id, sources)
                if svg:
                    node["icon_svg"] = svg

        return ir_dict
