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
    /// Icon sources for resolution (e.g., "github:user/repo", "https://...")
    #[serde(default)]
    pub icon_sources: Vec<String>,
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
    pub font_size: Option<f64>,
    pub font_color: Option<String>,
}
