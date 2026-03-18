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
    let theme = Theme::from_ir(&ir.metadata.theme, &ir.metadata.custom_theme);
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
    fn test_dark_theme() {
        let ir = r#"{
            "version": "1.0.0",
            "metadata": { "theme": "dark" },
            "nodes": [
                { "id": "a", "label": "A" },
                { "id": "b", "label": "B" }
            ],
            "edges": [{ "from": "a", "to": "b" }]
        }"#;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("#1A1B26")); // dark background
    }

    #[test]
    fn test_unknown_theme_falls_back() {
        let ir = r#"{
            "version": "1.0.0",
            "metadata": { "theme": "nonexistent" },
            "nodes": [{ "id": "a", "label": "A" }],
            "edges": []
        }"#;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("#FAFBFC")); // default background
    }

    #[test]
    fn test_custom_theme_override() {
        let ir = r##"{
            "version": "1.0.0",
            "metadata": {
                "theme": "default",
                "custom_theme": {
                    "background": "#FF0000",
                    "node_palette": [
                        { "fill": "#00FF00", "stroke": "#008800" }
                    ],
                    "node_shadow": false
                }
            },
            "nodes": [{ "id": "a", "label": "A" }],
            "edges": []
        }"##;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("#FF0000")); // custom background
        assert!(svg.contains("#00FF00")); // custom node fill
    }

    #[test]
    fn test_custom_theme_on_top_of_dark() {
        let ir = r##"{
            "version": "1.0.0",
            "metadata": {
                "theme": "dark",
                "custom_theme": {
                    "background": "#000000"
                }
            },
            "nodes": [{ "id": "a", "label": "A" }],
            "edges": []
        }"##;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("#000000")); // overridden background
        assert!(svg.contains("#7AA2F7")); // dark palette still used
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

    #[test]
    fn test_node_with_icon_svg() {
        let ir = r##"{
            "version": "1.0.0",
            "nodes": [
                { "id": "ec2", "label": "EC2", "icon_svg": "<circle cx=\"16\" cy=\"16\" r=\"12\" fill=\"#FF9900\"/>" },
                { "id": "rds", "label": "RDS" }
            ],
            "edges": [{ "from": "ec2", "to": "rds" }]
        }"##;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("EC2"));
        assert!(svg.contains("RDS"));
        // Icon node should have embedded SVG
        assert!(svg.contains("<svg x="));
        assert!(svg.contains("fill=\"#FF9900\""));
    }

    #[test]
    fn test_cluster_with_provider() {
        let ir = r#"{
            "version": "1.0.0",
            "nodes": [
                { "id": "web", "label": "Web Server" },
                { "id": "db", "label": "Database" }
            ],
            "clusters": [
                { "id": "vpc", "label": "VPC", "children": ["web", "db"], "provider": "aws", "cluster_type": "vpc" }
            ],
            "edges": [{ "from": "web", "to": "db" }]
        }"#;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("VPC"));
        // AWS VPC preset stroke color
        assert!(svg.contains("#00A4A6"));
    }

    #[test]
    fn test_mixed_icon_and_plain_nodes() {
        let ir = r#"{
            "version": "1.0.0",
            "nodes": [
                { "id": "a", "label": "With Icon", "icon_svg": "<rect width=\"24\" height=\"24\" fill=\"blue\"/>" },
                { "id": "b", "label": "Plain" },
                { "id": "c", "label": "Also Plain" }
            ],
            "edges": [
                { "from": "a", "to": "b" },
                { "from": "b", "to": "c" }
            ]
        }"#;
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("With Icon"));
        assert!(svg.contains("Plain"));
        assert!(svg.contains("Also Plain"));
    }

    #[test]
    fn test_icon_sources_in_metadata() {
        let ir = r#"{
            "version": "1.0.0",
            "metadata": {
                "icon_sources": ["github:archflow/icons", "https://example.com/icons"]
            },
            "nodes": [{ "id": "a", "label": "A" }],
            "edges": []
        }"#;
        // Should parse and render without error
        let svg = render_svg(ir).unwrap();
        assert!(svg.contains("<svg"));
    }
}
