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
        shadow: bool,
    },
    Text {
        x: f64,
        y: f64,
        content: String,
        font_size: f64,
        font_family: String,
        fill: String,
        anchor: String,
        font_weight: String,
    },
    Path {
        d: String,
        stroke: String,
        stroke_width: f64,
        stroke_dasharray: Option<String>,
        marker_end: bool,
    },
    RawSvg {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        content: String,
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
    pub edge_color: String,
    pub elements: Vec<SceneElement>,
}

pub fn build_scene(layout: &LayoutResult, ir: &DiagramIR, theme: &Theme) -> SceneGraph {
    let padding = 60.0;
    let mut elements = Vec::new();

    // Render clusters (behind nodes)
    for (ci, lc) in layout.clusters.iter().enumerate() {
        let cluster_def = ir.clusters.iter().find(|c| c.id == lc.id);
        let style = if let Some(cd) = cluster_def {
            if cd.provider.is_some() || cd.cluster_type.is_some() {
                theme.resolve_cluster_style_with_provider(cd, ci)
            } else {
                theme.resolve_cluster_style(&cd.style.clone(), ci)
            }
        } else {
            theme.resolve_cluster_style(&None, ci)
        };
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
                shadow: false,
            },
            SceneElement::Text {
                x: lc.x + padding + 16.0,
                y: lc.y + padding + 20.0,
                content: label.to_string(),
                font_size: theme.label_font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "start".to_string(),
                font_weight: "600".to_string(),
            },
        ];

        elements.push(SceneElement::Group {
            id: lc.id.clone(),
            children,
        });
    }

    // Render edges (behind nodes, on top of clusters)
    for le in &layout.edges {
        let edge_def = ir.edges.iter().find(|e| e.from == le.from && e.to == le.to);
        let style = theme.resolve_edge_style(&edge_def.and_then(|e| e.style.clone()));

        let points: Vec<(f64, f64)> = le
            .points
            .iter()
            .map(|&(x, y)| (x + padding, y + padding))
            .collect();

        // Build a smooth cubic bezier path between points
        let d = if points.len() == 2 {
            let (x1, y1) = points[0];
            let (x2, y2) = points[1];
            // Determine if horizontal or vertical flow
            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();
            if dx > dy {
                // Horizontal: curve with horizontal control points
                let cx = (x1 + x2) / 2.0;
                format!("M{x1},{y1} C{cx},{y1} {cx},{y2} {x2},{y2}")
            } else {
                // Vertical: curve with vertical control points
                let cy = (y1 + y2) / 2.0;
                format!("M{x1},{y1} C{x1},{cy} {x2},{cy} {x2},{y2}")
            }
        } else {
            // Fallback: straight lines
            let mut d = format!("M{},{}", points[0].0, points[0].1);
            for &(x, y) in &points[1..] {
                d.push_str(&format!(" L{x},{y}"));
            }
            d
        };

        elements.push(SceneElement::Path {
            d,
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

                    // Label background
                    let text_width = label.len() as f64 * 6.5 + 12.0;
                    elements.push(SceneElement::Rect {
                        x: mid_x - text_width / 2.0,
                        y: mid_y - 18.0,
                        width: text_width,
                        height: 20.0,
                        rx: 4.0,
                        fill: theme.background.clone(),
                        stroke: "none".to_string(),
                        stroke_width: 0.0,
                        shadow: false,
                    });
                    elements.push(SceneElement::Text {
                        x: mid_x,
                        y: mid_y - 8.0,
                        content: label.clone(),
                        font_size: theme.label_font_size,
                        font_family: theme.font_family.clone(),
                        fill: theme.edge_stroke.clone(),
                        anchor: "middle".to_string(),
                        font_weight: "500".to_string(),
                    });
                }
            }
        }
    }

    // Render nodes (on top)
    for (ni, ln) in layout.nodes.iter().enumerate() {
        let node_def = ir.nodes.iter().find(|n| n.id == ln.id);
        let style = theme.resolve_node_style(&node_def.and_then(|n| n.style.clone()), ni);
        let label = node_def.map(|n| n.label.as_str()).unwrap_or(&ln.id);
        let icon_svg = node_def.and_then(|n| n.icon_svg.as_ref());

        let mut children = vec![SceneElement::Rect {
            x: ln.x + padding,
            y: ln.y + padding,
            width: ln.width,
            height: ln.height,
            rx: style.corner_radius,
            fill: style.fill,
            stroke: style.stroke,
            stroke_width: style.stroke_width,
            shadow: style.shadow,
        }];

        if let Some(svg_content) = icon_svg {
            // Icon node: icon at top, label at bottom
            let icon_size = 32.0;
            let icon_x = ln.x + padding + (ln.width - icon_size) / 2.0;
            let icon_y = ln.y + padding + 8.0;
            children.push(SceneElement::RawSvg {
                x: icon_x,
                y: icon_y,
                width: icon_size,
                height: icon_size,
                content: svg_content.clone(),
            });
            children.push(SceneElement::Text {
                x: ln.x + padding + ln.width / 2.0,
                y: ln.y + padding + ln.height - 12.0,
                content: label.to_string(),
                font_size: theme.font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "middle".to_string(),
                font_weight: "600".to_string(),
            });
        } else {
            // Text-only node: label centered
            children.push(SceneElement::Text {
                x: ln.x + padding + ln.width / 2.0,
                y: ln.y + padding + ln.height / 2.0 + 1.0,
                content: label.to_string(),
                font_size: theme.font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "middle".to_string(),
                font_weight: "600".to_string(),
            });
        }

        elements.push(SceneElement::Group {
            id: ln.id.clone(),
            children,
        });
    }

    SceneGraph {
        width: layout.width,
        height: layout.height,
        background: theme.background.clone(),
        edge_color: theme.edge_stroke.clone(),
        elements,
    }
}
