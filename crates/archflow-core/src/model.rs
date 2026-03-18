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
    #[serde(default)]
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDef {
    pub id: String,
    pub label: String,
    pub children: Vec<String>,
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
