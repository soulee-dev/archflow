pub mod error;
pub mod layout;
pub mod model;
pub mod render;
pub mod scene;
pub mod theme;

use error::ArchflowError;
use model::DiagramIR;
use theme::Theme;

/// Render a diagram IR JSON string to an SVG string.
pub fn render_svg(json_ir: &str) -> Result<String, ArchflowError> {
    let ir: DiagramIR = serde_json::from_str(json_ir)?;
    validate(&ir)?;
    let layout = layout::compute_layout(&ir)?;
    let theme = Theme::default();
    let scene = scene::build_scene(&layout, &ir, &theme);
    Ok(render::render_svg(&scene))
}

fn validate(ir: &DiagramIR) -> Result<(), ArchflowError> {
    if ir.version != "1.0.0" {
        return Err(ArchflowError::InvalidSchema(format!(
            "Unsupported IR version: {}. Expected 1.0.0",
            ir.version
        )));
    }
    if ir.nodes.is_empty() {
        return Err(ArchflowError::InvalidSchema(
            "Diagram must have at least one node".into(),
        ));
    }
    // Check for duplicate node IDs
    let mut seen = std::collections::HashSet::new();
    for node in &ir.nodes {
        if !seen.insert(&node.id) {
            return Err(ArchflowError::InvalidSchema(format!(
                "Duplicate node ID: {}",
                node.id
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_render() {
        let ir = r#"{
            "version": "1.0.0",
            "metadata": { "title": "Test", "direction": "TB", "theme": "default" },
            "nodes": [
                { "id": "a", "label": "Node A" },
                { "id": "b", "label": "Node B" }
            ],
            "edges": [
                { "from": "a", "to": "b", "label": "connects" }
            ]
        }"#;

        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Node A"));
        assert!(svg.contains("Node B"));
        assert!(svg.contains("connects"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_with_cluster() {
        let ir = r#"{
            "version": "1.0.0",
            "nodes": [
                { "id": "web", "label": "Web Server" },
                { "id": "db", "label": "Database" }
            ],
            "clusters": [
                { "id": "vpc", "label": "VPC", "children": ["web", "db"] }
            ],
            "edges": [
                { "from": "web", "to": "db" }
            ]
        }"#;

        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("VPC"));
        assert!(svg.contains("Web Server"));
    }

    #[test]
    fn test_invalid_version() {
        let ir = r#"{
            "version": "2.0.0",
            "nodes": [{ "id": "a", "label": "A" }],
            "edges": []
        }"#;
        assert!(render_svg(ir).is_err());
    }

    #[test]
    fn test_empty_nodes() {
        let ir = r#"{
            "version": "1.0.0",
            "nodes": [],
            "edges": []
        }"#;
        assert!(render_svg(ir).is_err());
    }

    #[test]
    fn test_lr_direction() {
        let ir = r#"{
            "version": "1.0.0",
            "metadata": { "direction": "LR" },
            "nodes": [
                { "id": "a", "label": "A" },
                { "id": "b", "label": "B" }
            ],
            "edges": [{ "from": "a", "to": "b" }]
        }"#;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("<svg"));
    }
}
