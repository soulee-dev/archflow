use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramIR {
    pub version: String,
    #[serde(default)]
    pub metadata: Metadata,
    pub nodes: Vec<NodeDef>,
    #[serde(default)]
    pub clusters: Vec<ClusterDef>,
    pub edges: Vec<EdgeDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default = "default_direction")]
    pub direction: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub custom_theme: Option<CustomThemeDef>,
    /// Per-provider source overrides from "use ... from ..." (e.g., {"aws": "github:org/repo"})
    #[serde(default)]
    pub provider_sources: std::collections::HashMap<String, Option<String>>,
    /// Per-provider node render modes from registry (e.g., {"aws": "icon_only"})
    #[serde(default)]
    pub node_render_modes: std::collections::HashMap<String, String>,
    /// Layout configuration overrides
    #[serde(default)]
    pub layout: Option<LayoutConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutConfig {
    /// Icon size in pixels (default: 48)
    #[serde(default)]
    pub icon_size: Option<f64>,
    /// Node width in pixels (default: 160)
    #[serde(default)]
    pub node_width: Option<f64>,
    /// Node height in pixels for text-only nodes (default: 60)
    #[serde(default)]
    pub node_height: Option<f64>,
    /// Horizontal spacing between nodes (default: 120)
    #[serde(default)]
    pub h_spacing: Option<f64>,
    /// Vertical spacing between nodes (default: 120)
    #[serde(default)]
    pub v_spacing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeDef {
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub node_palette: Option<Vec<NodeColorDef>>,
    #[serde(default)]
    pub node_text_color: Option<String>,
    #[serde(default)]
    pub node_corner_radius: Option<f64>,
    #[serde(default)]
    pub cluster_fills: Option<Vec<String>>,
    #[serde(default)]
    pub cluster_stroke: Option<String>,
    #[serde(default)]
    pub cluster_text_color: Option<String>,
    #[serde(default)]
    pub edge_stroke: Option<String>,
    #[serde(default)]
    pub edge_stroke_width: Option<f64>,
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub font_size: Option<f64>,
    #[serde(default)]
    pub node_shadow: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeColorDef {
    pub fill: String,
    pub stroke: String,
}

fn default_direction() -> String {
    "TB".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDef {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    /// Resolved inline SVG content for the icon (set by Python/JS resolver before rendering)
    #[serde(default)]
    pub icon_svg: Option<String>,
    #[serde(default)]
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDef {
    pub id: String,
    pub label: String,
    pub children: Vec<String>,
    /// Provider name (e.g., "aws", "gcp") for provider-aware styling
    #[serde(default)]
    pub provider: Option<String>,
    /// Cluster type (e.g., "region", "vpc", "subnet") for style presets
    #[serde(default)]
    pub cluster_type: Option<String>,
    /// Resolved inline SVG icon for the cluster label area
    #[serde(default)]
    pub icon_svg: Option<String>,
    #[serde(default)]
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDef {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Style {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub stroke_dasharray: Option<String>,
    pub corner_radius: Option<f64>,
    pub font_size: Option<f64>,
    pub font_color: Option<String>,
}
