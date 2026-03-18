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
        stroke_dasharray: Option<String>,
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
    let icon_size = ir
        .metadata
        .layout
        .as_ref()
        .and_then(|c| c.icon_size)
        .unwrap_or(48.0);
    let mut elements = Vec::new();

    // Render clusters (behind nodes)
    for (ci, lc) in layout.clusters.iter().enumerate() {
        let cluster_def = ir.clusters.iter().find(|c| c.id == lc.id);
        let style = theme.resolve_cluster_style(&cluster_def.and_then(|c| c.style.clone()), ci);
        let label = cluster_def.map(|c| c.label.as_str()).unwrap_or(&lc.id);
        let cluster_icon = cluster_def.and_then(|c| c.icon_svg.as_ref());

        let mut children = vec![SceneElement::Rect {
            x: lc.x + padding,
            y: lc.y + padding,
            width: lc.width,
            height: lc.height,
            rx: style.corner_radius,
            fill: style.fill.clone(),
            stroke: style.stroke.clone(),
            stroke_width: style.stroke_width,
            stroke_dasharray: style.stroke_dasharray,
            shadow: false,
        }];

        if let Some(icon_content) = cluster_icon {
            // AWS style: icon badge in top-left corner, sitting on the border
            let badge_size = 28.0;
            let badge_x = lc.x + padding;
            let badge_y = lc.y + padding;

            // Badge background (solid colored square)
            children.push(SceneElement::Rect {
                x: badge_x,
                y: badge_y,
                width: badge_size,
                height: badge_size,
                rx: 0.0,
                fill: style.stroke.clone(),
                stroke: "none".to_string(),
                stroke_width: 0.0,
                stroke_dasharray: None,
                shadow: false,
            });
            // Icon inside badge
            children.push(SceneElement::RawSvg {
                x: badge_x + 2.0,
                y: badge_y + 2.0,
                width: badge_size - 4.0,
                height: badge_size - 4.0,
                content: icon_content.clone(),
            });
            // Label next to badge
            children.push(SceneElement::Text {
                x: badge_x + badge_size + 8.0,
                y: badge_y + badge_size / 2.0 + 1.0,
                content: label.to_string(),
                font_size: theme.label_font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "start".to_string(),
                font_weight: "600".to_string(),
            });
        } else {
            // Cluster without icon: label only
            children.push(SceneElement::Text {
                x: lc.x + padding + 16.0,
                y: lc.y + padding + 20.0,
                content: label.to_string(),
                font_size: theme.label_font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color,
                anchor: "start".to_string(),
                font_weight: "600".to_string(),
            });
        }

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
        let is_lr = ir.metadata.direction == "LR";
        let d = if points.len() == 2 {
            let (x1, y1) = points[0];
            let (x2, y2) = points[1];
            if is_lr {
                // LR: horizontal flow — control points ensure horizontal exit/entry
                let cx = (x1 + x2) / 2.0;
                format!("M{x1},{y1} C{cx},{y1} {cx},{y2} {x2},{y2}")
            } else {
                // TB: vertical flow — control points ensure vertical exit/entry
                // Arrow always leaves downward and arrives from above
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
                        stroke_dasharray: None,
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
        let provider = node_def.and_then(|n| n.provider.as_deref());
        let is_icon_only = provider
            .and_then(|p| ir.metadata.node_render_modes.get(p))
            .map(|m| m == "icon_only")
            .unwrap_or(false);

        let mut children = Vec::new();

        if is_icon_only {
            if let Some(svg_content) = icon_svg {
                // Icon-only mode: no box, just icon centered + label below
                let icon_x = ln.x + padding + (ln.width - icon_size) / 2.0;
                let icon_y = ln.y + padding;
                children.push(SceneElement::RawSvg {
                    x: icon_x,
                    y: icon_y,
                    width: icon_size,
                    height: icon_size,
                    content: svg_content.clone(),
                });
                children.push(SceneElement::Text {
                    x: ln.x + padding + ln.width / 2.0,
                    y: ln.y + padding + icon_size + 14.0,
                    content: label.to_string(),
                    font_size: theme.font_size,
                    font_family: theme.font_family.clone(),
                    fill: theme.cluster_text_color.clone(),
                    anchor: "middle".to_string(),
                    font_weight: "500".to_string(),
                });
            }
        } else if let Some(svg_content) = icon_svg {
            // Box + icon mode: rect with icon at top, label at bottom
            children.push(SceneElement::Rect {
                x: ln.x + padding,
                y: ln.y + padding,
                width: ln.width,
                height: ln.height,
                rx: style.corner_radius,
                fill: style.fill.clone(),
                stroke: style.stroke.clone(),
                stroke_width: style.stroke_width,
                stroke_dasharray: None,
                shadow: style.shadow,
            });
            let icon_size = 32.0;
            let icon_x = ln.x + padding + (ln.width - icon_size) / 2.0;
            let icon_y = ln.y + padding + 6.0;
            children.push(SceneElement::RawSvg {
                x: icon_x,
                y: icon_y,
                width: icon_size,
                height: icon_size,
                content: svg_content.clone(),
            });
            children.push(SceneElement::Text {
                x: ln.x + padding + ln.width / 2.0,
                y: ln.y + padding + ln.height - 10.0,
                content: label.to_string(),
                font_size: theme.font_size,
                font_family: theme.font_family.clone(),
                fill: style.text_color.clone(),
                anchor: "middle".to_string(),
                font_weight: "600".to_string(),
            });
        } else {
            // Text-only node: box + label centered
            children.push(SceneElement::Rect {
                x: ln.x + padding,
                y: ln.y + padding,
                width: ln.width,
                height: ln.height,
                rx: style.corner_radius,
                fill: style.fill,
                stroke: style.stroke,
                stroke_width: style.stroke_width,
                stroke_dasharray: None,
                shadow: style.shadow,
            });
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
