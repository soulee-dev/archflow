use crate::model::Style;

/// A color pair for nodes: fill + stroke
#[derive(Debug, Clone)]
pub struct NodeColor {
    pub fill: String,
    pub stroke: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: String,
    pub node_palette: Vec<NodeColor>,
    pub node_text_color: String,
    pub node_corner_radius: f64,
    pub cluster_fills: Vec<String>,
    pub cluster_stroke: String,
    pub cluster_text_color: String,
    pub cluster_corner_radius: f64,
    pub edge_stroke: String,
    pub edge_stroke_width: f64,
    pub font_family: String,
    pub font_size: f64,
    pub label_font_size: f64,
    pub node_shadow: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#FAFBFC".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#4A90D9".into(),
                    stroke: "#3A7BC8".into(),
                }, // blue
                NodeColor {
                    fill: "#6C5CE7".into(),
                    stroke: "#5A4BD6".into(),
                }, // purple
                NodeColor {
                    fill: "#00B894".into(),
                    stroke: "#00A383".into(),
                }, // green
                NodeColor {
                    fill: "#E17055".into(),
                    stroke: "#D05F44".into(),
                }, // orange
                NodeColor {
                    fill: "#FD79A8".into(),
                    stroke: "#EC6897".into(),
                }, // pink
                NodeColor {
                    fill: "#00CEC9".into(),
                    stroke: "#00BDB8".into(),
                }, // teal
                NodeColor {
                    fill: "#FDCB6E".into(),
                    stroke: "#ECBA5D".into(),
                }, // yellow
                NodeColor {
                    fill: "#636E72".into(),
                    stroke: "#525D61".into(),
                }, // gray
            ],
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 10.0,
            cluster_fills: vec![
                "rgba(74, 144, 217, 0.06)".into(),
                "rgba(108, 92, 231, 0.06)".into(),
                "rgba(0, 184, 148, 0.06)".into(),
                "rgba(225, 112, 85, 0.06)".into(),
            ],
            cluster_stroke: "#E1E4E8".into(),
            cluster_text_color: "#586069".into(),
            cluster_corner_radius: 16.0,
            edge_stroke: "#959DA5".into(),
            edge_stroke_width: 1.5,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: true,
        }
    }
}

pub struct ResolvedNodeStyle {
    pub fill: String,
    pub stroke: String,
    pub stroke_width: f64,
    pub text_color: String,
    pub corner_radius: f64,
    pub shadow: bool,
}

pub struct ResolvedEdgeStyle {
    pub stroke: String,
    pub stroke_width: f64,
    pub stroke_dasharray: Option<String>,
}

pub struct ResolvedClusterStyle {
    pub fill: String,
    pub stroke: String,
    pub stroke_width: f64,
    pub text_color: String,
    pub corner_radius: f64,
}

impl Theme {
    /// Pick a color from the palette based on node index.
    pub fn resolve_node_style(&self, style: &Option<Style>, index: usize) -> ResolvedNodeStyle {
        let s = style.as_ref();
        let palette_color = &self.node_palette[index % self.node_palette.len()];
        ResolvedNodeStyle {
            fill: s
                .and_then(|s| s.fill.clone())
                .unwrap_or_else(|| palette_color.fill.clone()),
            stroke: s
                .and_then(|s| s.stroke.clone())
                .unwrap_or_else(|| palette_color.stroke.clone()),
            stroke_width: s.and_then(|s| s.stroke_width).unwrap_or(1.0),
            text_color: s
                .and_then(|s| s.font_color.clone())
                .unwrap_or_else(|| self.node_text_color.clone()),
            corner_radius: self.node_corner_radius,
            shadow: self.node_shadow,
        }
    }

    pub fn resolve_edge_style(&self, style: &Option<Style>) -> ResolvedEdgeStyle {
        let s = style.as_ref();
        ResolvedEdgeStyle {
            stroke: s
                .and_then(|s| s.stroke.clone())
                .unwrap_or_else(|| self.edge_stroke.clone()),
            stroke_width: s
                .and_then(|s| s.stroke_width)
                .unwrap_or(self.edge_stroke_width),
            stroke_dasharray: s.and_then(|s| s.stroke_dasharray.clone()),
        }
    }

    pub fn resolve_cluster_style(
        &self,
        style: &Option<Style>,
        index: usize,
    ) -> ResolvedClusterStyle {
        let s = style.as_ref();
        let default_fill = &self.cluster_fills[index % self.cluster_fills.len()];
        ResolvedClusterStyle {
            fill: s
                .and_then(|s| s.fill.clone())
                .unwrap_or_else(|| default_fill.clone()),
            stroke: s
                .and_then(|s| s.stroke.clone())
                .unwrap_or_else(|| self.cluster_stroke.clone()),
            stroke_width: s.and_then(|s| s.stroke_width).unwrap_or(1.5),
            text_color: s
                .and_then(|s| s.font_color.clone())
                .unwrap_or_else(|| self.cluster_text_color.clone()),
            corner_radius: self.cluster_corner_radius,
        }
    }
}
