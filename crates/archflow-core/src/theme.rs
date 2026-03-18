use crate::model::Style;

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: String,
    pub node_fill: String,
    pub node_stroke: String,
    pub node_text_color: String,
    pub node_corner_radius: f64,
    pub cluster_fill: String,
    pub cluster_stroke: String,
    pub cluster_text_color: String,
    pub cluster_corner_radius: f64,
    pub edge_stroke: String,
    pub edge_stroke_width: f64,
    pub font_family: String,
    pub font_size: f64,
    pub label_font_size: f64,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#FFFFFF".into(),
            node_fill: "#4A90D9".into(),
            node_stroke: "#2C5F8A".into(),
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 8.0,
            cluster_fill: "#F5F7FA".into(),
            cluster_stroke: "#D0D5DD".into(),
            cluster_text_color: "#475467".into(),
            cluster_corner_radius: 12.0,
            edge_stroke: "#667085".into(),
            edge_stroke_width: 1.5,
            font_family: "Inter, system-ui, -apple-system, sans-serif".into(),
            font_size: 14.0,
            label_font_size: 12.0,
        }
    }
}

pub struct ResolvedNodeStyle {
    pub fill: String,
    pub stroke: String,
    pub stroke_width: f64,
    pub text_color: String,
    pub corner_radius: f64,
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
    pub fn resolve_node_style(&self, style: &Option<Style>) -> ResolvedNodeStyle {
        let s = style.as_ref();
        ResolvedNodeStyle {
            fill: s.and_then(|s| s.fill.clone()).unwrap_or_else(|| self.node_fill.clone()),
            stroke: s.and_then(|s| s.stroke.clone()).unwrap_or_else(|| self.node_stroke.clone()),
            stroke_width: s.and_then(|s| s.stroke_width).unwrap_or(1.5),
            text_color: s
                .and_then(|s| s.font_color.clone())
                .unwrap_or_else(|| self.node_text_color.clone()),
            corner_radius: self.node_corner_radius,
        }
    }

    pub fn resolve_edge_style(&self, style: &Option<Style>) -> ResolvedEdgeStyle {
        let s = style.as_ref();
        ResolvedEdgeStyle {
            stroke: s.and_then(|s| s.stroke.clone()).unwrap_or_else(|| self.edge_stroke.clone()),
            stroke_width: s.and_then(|s| s.stroke_width).unwrap_or(self.edge_stroke_width),
            stroke_dasharray: s.and_then(|s| s.stroke_dasharray.clone()),
        }
    }

    pub fn resolve_cluster_style(&self, style: &Option<Style>) -> ResolvedClusterStyle {
        let s = style.as_ref();
        ResolvedClusterStyle {
            fill: s
                .and_then(|s| s.fill.clone())
                .unwrap_or_else(|| self.cluster_fill.clone()),
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
