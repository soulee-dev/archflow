use crate::layout::LayoutResult;
use crate::model::DiagramIR;
use crate::theme::Theme;

#[derive(Debug, Clone)]
pub enum SceneElement {
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rx: f64,
        fill: String,
        stroke: String,
        stroke_width: f64,
    },
    Text {
        x: f64,
        y: f64,
        content: String,
        font_size: f64,
        font_family: String,
        fill: String,
        anchor: String,
    },
    Line {
        points: Vec<(f64, f64)>,
        stroke: String,
        stroke_width: f64,
        stroke_dasharray: Option<String>,
        marker_end: bool,
    },
    Group {
        id: String,
        children: Vec<SceneElement>,
    },
}

#[derive(Debug, Clone)]
pub struct SceneGraph {
    pub width: f64,
    pub height: f64,
    pub background: String,
    pub elements: Vec<SceneElement>,
}

pub fn build_scene(layout: &LayoutResult, ir: &DiagramIR, theme: &Theme) -> SceneGraph {
    let padding = 60.0;
    let mut elements = Vec::new();

    // Render clusters (behind nodes)
    for lc in &layout.clusters {
        let cluster_def = ir.clusters.iter().find(|c| c.id == lc.id);
        let style = theme.resolve_cluster_style(&cluster_def.and_then(|c| c.style.clone()));
        let label = cluster_def.map(|c| c.label.as_str()).unwrap_or(&lc.id);

        let children = vec![
            SceneElement::Rect {
                x: lc.x + padding,
                y: lc.y + padding,
                width: lc.width,
                height: lc.height,
                rx: style.corner_radius,
                fill: style.fill,
                stroke: style.stroke,
                stroke_width: style.stroke_width,
            },
            SceneElement::Text {
                x: lc.x + padding + 16.0,
                y: lc.y + padding + 20.0,
                content: label.to_string(),
                font_size: theme.label_font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "start".to_string(),
            },
        ];

        elements.push(SceneElement::Group {
            id: lc.id.clone(),
            children,
        });
    }

    // Render edges (behind nodes, on top of clusters)
    for le in &layout.edges {
        let edge_def = ir
            .edges
            .iter()
            .find(|e| e.from == le.from && e.to == le.to);
        let style = theme.resolve_edge_style(&edge_def.and_then(|e| e.style.clone()));

        let points: Vec<(f64, f64)> = le.points.iter().map(|&(x, y)| (x + padding, y + padding)).collect();

        elements.push(SceneElement::Line {
            points: points.clone(),
            stroke: style.stroke,
            stroke_width: style.stroke_width,
            stroke_dasharray: style.stroke_dasharray,
            marker_end: true,
        });

        // Edge label
        if let Some(edge_def) = edge_def {
            if let Some(ref label) = edge_def.label {
                if points.len() >= 2 {
                    let mid_x = (points[0].0 + points[points.len() - 1].0) / 2.0;
                    let mid_y = (points[0].1 + points[points.len() - 1].1) / 2.0;
                    elements.push(SceneElement::Text {
                        x: mid_x,
                        y: mid_y - 8.0,
                        content: label.clone(),
                        font_size: theme.label_font_size,
                        font_family: theme.font_family.clone(),
                        fill: theme.edge_stroke.clone(),
                        anchor: "middle".to_string(),
                    });
                }
            }
        }
    }

    // Render nodes (on top)
    for ln in &layout.nodes {
        let node_def = ir.nodes.iter().find(|n| n.id == ln.id);
        let style = theme.resolve_node_style(&node_def.and_then(|n| n.style.clone()));
        let label = node_def.map(|n| n.label.as_str()).unwrap_or(&ln.id);

        let children = vec![
            SceneElement::Rect {
                x: ln.x + padding,
                y: ln.y + padding,
                width: ln.width,
                height: ln.height,
                rx: style.corner_radius,
                fill: style.fill,
                stroke: style.stroke,
                stroke_width: style.stroke_width,
            },
            SceneElement::Text {
                x: ln.x + padding + ln.width / 2.0,
                y: ln.y + padding + ln.height / 2.0 + 5.0,
                content: label.to_string(),
                font_size: theme.font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "middle".to_string(),
            },
        ];

        elements.push(SceneElement::Group {
            id: ln.id.clone(),
            children,
        });
    }

    SceneGraph {
        width: layout.width,
        height: layout.height,
        background: theme.background.clone(),
        elements,
    }
}
