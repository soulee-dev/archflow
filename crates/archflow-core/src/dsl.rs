//! Archflow DSL Parser
//!
//! Grammar (strict):
//! ```text
//! diagram       = line*
//! line          = comment | use_stmt | metadata | cluster_block | edge_chain | node_decl
//! comment       = ('#' | '//') TEXT
//! use_stmt      = 'use' PROVIDER ('from' SOURCE)?
//! metadata      = 'title' ':' TEXT
//!               | 'direction' ':' ('TB' | 'LR')
//!               | 'theme' ':' IDENT
//! cluster_block = 'cluster' (':' PROVIDER ':' IDENT)? LABEL '{' line* '}'
//! edge_chain    = node_ref ('>>' node_ref)* ('[' TEXT ']')?  (last >> can have label)
//!               | node_ref ('>>' node_ref)+
//! node_ref      = (PROVIDER ':' IDENT SPACE)? LABEL
//! node_decl     = node_ref   (standalone node on its own line)
//! PROVIDER      = [a-z0-9-]+
//! ```

use std::collections::HashMap;

use crate::error::ArchflowError;
use crate::model::{ClusterDef, DiagramIR, EdgeDef, Metadata, NodeDef};

/// Parse an Archflow DSL string into a DiagramIR.
pub fn parse_dsl(input: &str) -> Result<DiagramIR, ArchflowError> {
    let mut parser = Parser::new(input);
    parser.parse()
}

/// Validate a provider identifier: lowercase alphanumeric + hyphens, non-empty.
fn is_valid_provider(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

struct Parser<'a> {
    lines: Vec<&'a str>,
    pos: usize,
    // State
    title: Option<String>,
    direction: String,
    theme: String,
    provider_sources: HashMap<String, Option<String>>,
    layout: Option<crate::model::LayoutConfig>,
    nodes: Vec<NodeDef>,
    node_map: HashMap<String, usize>, // node_key -> index in nodes vec
    clusters: Vec<ClusterDef>,
    edges: Vec<EdgeDef>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().collect(),
            pos: 0,
            title: None,
            direction: "TB".to_string(),
            theme: "default".to_string(),
            provider_sources: HashMap::new(),
            layout: None,
            nodes: Vec::new(),
            node_map: HashMap::new(),
            clusters: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn parse(&mut self) -> Result<DiagramIR, ArchflowError> {
        while self.pos < self.lines.len() {
            self.parse_line()?;
        }

        if self.nodes.is_empty() {
            return Err(ArchflowError::ParseError {
                line: self.lines.len(),
                message: "Diagram must have at least one node".into(),
            });
        }

        Ok(DiagramIR {
            version: "1.0.0".to_string(),
            metadata: Metadata {
                title: self.title.clone(),
                direction: self.direction.clone(),
                theme: self.theme.clone(),
                custom_theme: None,
                provider_sources: self.provider_sources.clone(),
                node_render_modes: HashMap::new(),
                layout: self.layout.clone(),
            },
            nodes: self.nodes.clone(),
            clusters: self.clusters.clone(),
            edges: self.edges.clone(),
        })
    }

    fn parse_line(&mut self) -> Result<(), ArchflowError> {
        let line_num = self.pos + 1;
        let line = self.lines[self.pos];
        let trimmed = line.trim();
        self.pos += 1;

        // Empty or comment
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            return Ok(());
        }

        // use statement
        if trimmed.starts_with("use ") || trimmed == "use" {
            return self.parse_use(trimmed, line_num);
        }

        // metadata: title, direction, theme
        if let Some(rest) = strip_prefix_ci(trimmed, "title:") {
            self.title = Some(rest.trim().to_string());
            return Ok(());
        }
        if let Some(rest) = strip_prefix_ci(trimmed, "direction:") {
            let dir = rest.trim().to_uppercase();
            if dir != "TB" && dir != "LR" {
                return Err(ArchflowError::ParseError {
                    line: line_num,
                    message: format!("Invalid direction '{}'. Must be TB or LR", rest.trim()),
                });
            }
            self.direction = dir;
            return Ok(());
        }
        if let Some(rest) = strip_prefix_ci(trimmed, "theme:") {
            self.theme = rest.trim().to_string();
            return Ok(());
        }
        if let Some(rest) = strip_prefix_ci(trimmed, "icon_size:") {
            let val: f64 = rest.trim().parse().map_err(|_| ArchflowError::ParseError {
                line: line_num,
                message: format!("Invalid icon_size: '{}'", rest.trim()),
            })?;
            self.layout.get_or_insert_with(Default::default).icon_size = Some(val);
            return Ok(());
        }
        if let Some(rest) = strip_prefix_ci(trimmed, "node_width:") {
            let val: f64 = rest.trim().parse().map_err(|_| ArchflowError::ParseError {
                line: line_num,
                message: format!("Invalid node_width: '{}'", rest.trim()),
            })?;
            self.layout.get_or_insert_with(Default::default).node_width = Some(val);
            return Ok(());
        }
        if let Some(rest) = strip_prefix_ci(trimmed, "spacing:") {
            let val: f64 = rest.trim().parse().map_err(|_| ArchflowError::ParseError {
                line: line_num,
                message: format!("Invalid spacing: '{}'", rest.trim()),
            })?;
            let cfg = self.layout.get_or_insert_with(Default::default);
            cfg.h_spacing = Some(val);
            cfg.v_spacing = Some(val);
            return Ok(());
        }

        // cluster block
        if trimmed.starts_with("cluster") && trimmed.ends_with('{') {
            return self.parse_cluster(trimmed, line_num);
        }

        // edge chain (contains >>)
        if trimmed.contains(">>") {
            self.parse_edge_chain(trimmed, line_num)?;
            return Ok(());
        }

        // standalone node declaration
        if !trimmed.contains('{') && !trimmed.contains('}') {
            self.ensure_node(trimmed, line_num)?;
            return Ok(());
        }

        // closing brace (inside cluster parsing, shouldn't reach here)
        if trimmed == "}" {
            return Ok(());
        }

        Err(ArchflowError::ParseError {
            line: line_num,
            message: format!("Unexpected syntax: '{}'", trimmed),
        })
    }

    fn parse_use(&mut self, trimmed: &str, line_num: usize) -> Result<(), ArchflowError> {
        let rest = if trimmed.len() > 4 {
            trimmed[4..].trim()
        } else {
            ""
        };
        let (provider, source) = if let Some(from_idx) = rest.find(" from ") {
            let p = rest[..from_idx].trim();
            let s = rest[from_idx + 6..].trim();
            if s.is_empty() {
                return Err(ArchflowError::ParseError {
                    line: line_num,
                    message: "'use ... from' requires a source".into(),
                });
            }
            (p.to_string(), Some(s.to_string()))
        } else {
            (rest.to_string(), None)
        };

        if !is_valid_provider(&provider) {
            return Err(ArchflowError::ParseError {
                line: line_num,
                message: format!("Invalid provider name: '{}'", provider),
            });
        }

        self.provider_sources.insert(provider, source);
        Ok(())
    }

    fn parse_cluster(&mut self, trimmed: &str, line_num: usize) -> Result<(), ArchflowError> {
        let without_brace = trimmed[..trimmed.len() - 1].trim();

        let (provider, cluster_type, label) = if let Some(rest) =
            without_brace.strip_prefix("cluster:")
        {
            let first_colon = rest.find(':').ok_or_else(|| ArchflowError::ParseError {
                line: line_num,
                message: "Expected cluster:provider:type Label {".into(),
            })?;
            let prov = &rest[..first_colon];
            let after_provider = &rest[first_colon + 1..];
            let space_idx = after_provider
                .find(' ')
                .ok_or_else(|| ArchflowError::ParseError {
                    line: line_num,
                    message: "Expected cluster:provider:type Label {".into(),
                })?;
            let ctype = &after_provider[..space_idx];
            let label = after_provider[space_idx + 1..].trim();

            // Validate provider and cluster_type
            if !is_valid_provider(prov) {
                return Err(ArchflowError::ParseError {
                    line: line_num,
                    message: format!("Invalid provider name in cluster: '{}'", prov),
                });
            }
            if ctype.is_empty()
                || !ctype
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                return Err(ArchflowError::ParseError {
                    line: line_num,
                    message: format!("Invalid cluster type: '{}'", ctype),
                });
            }

            (
                Some(prov.to_string()),
                Some(ctype.to_string()),
                label.to_string(),
            )
        } else {
            let label = without_brace
                .strip_prefix("cluster")
                .unwrap()
                .trim()
                .to_string();
            (None, None, label)
        };

        if label.is_empty() {
            return Err(ArchflowError::ParseError {
                line: line_num,
                message: "Cluster must have a label".into(),
            });
        }

        let cluster_id = to_id(&label);
        let mut children = Vec::new();
        let mut closed = false;

        // Parse lines inside cluster until }
        while self.pos < self.lines.len() {
            let inner_line_num = self.pos + 1;
            let inner = self.lines[self.pos].trim();
            self.pos += 1;

            if inner == "}" {
                closed = true;
                break;
            }
            if inner.is_empty() || inner.starts_with('#') || inner.starts_with("//") {
                continue;
            }

            // Edge chains inside clusters — collect all referenced node IDs
            if inner.contains(">>") {
                let ids = self.parse_edge_chain(inner, inner_line_num)?;
                for id in ids {
                    if !children.contains(&id) {
                        children.push(id);
                    }
                }
                continue;
            }

            let node_id = self.ensure_node(inner, inner_line_num)?;
            if !children.contains(&node_id) {
                children.push(node_id);
            }
        }

        if !closed {
            return Err(ArchflowError::ParseError {
                line: line_num,
                message: format!("Unclosed cluster block: '{}'", label),
            });
        }

        self.clusters.push(ClusterDef {
            id: cluster_id,
            label,
            children,
            provider,
            cluster_type,
            icon_svg: None,
            style: None,
        });

        Ok(())
    }

    /// Parse an edge chain. Returns all node IDs referenced.
    fn parse_edge_chain(
        &mut self,
        trimmed: &str,
        line_num: usize,
    ) -> Result<Vec<String>, ArchflowError> {
        let parts: Vec<&str> = trimmed.split(">>").collect();
        if parts.len() < 2 {
            return Err(ArchflowError::ParseError {
                line: line_num,
                message: "Edge chain requires at least two nodes".into(),
            });
        }

        let mut referenced_ids = Vec::new();

        for j in 0..parts.len() - 1 {
            let from_raw = parts[j].trim();
            let from_id = self.ensure_node(from_raw, line_num)?;
            if !referenced_ids.contains(&from_id) {
                referenced_ids.push(from_id.clone());
            }

            let to_part = parts[j + 1].trim();

            // Last segment might have [edge label]
            let (to_raw, edge_label) = if j == parts.len() - 2 {
                extract_bracket_label(to_part)
            } else {
                (to_part, None)
            };

            let to_id = self.ensure_node(to_raw, line_num)?;
            if !referenced_ids.contains(&to_id) {
                referenced_ids.push(to_id.clone());
            }

            self.edges.push(EdgeDef {
                from: from_id,
                to: to_id,
                label: edge_label.map(|s| s.to_string()),
                style: None,
            });
        }

        Ok(referenced_ids)
    }

    /// Ensure a node exists, returning its ID. Creates it if not yet seen.
    /// Dedup key includes provider+icon+label to avoid cross-provider collisions.
    fn ensure_node(&mut self, raw: &str, line_num: usize) -> Result<String, ArchflowError> {
        let spec = parse_node_spec(raw);
        let id = to_id(&spec.label);

        if id.is_empty() {
            return Err(ArchflowError::ParseError {
                line: line_num,
                message: format!("Empty node label in: '{}'", raw),
            });
        }

        // Dedup key: provider|icon|label — prevents cross-provider collision
        let dedup_key = format!(
            "{}|{}|{}",
            spec.provider.as_deref().unwrap_or(""),
            spec.icon.as_deref().unwrap_or(""),
            &spec.label
        );

        if !self.node_map.contains_key(&dedup_key) {
            // Check if there's already a node with the same ID but different provider
            let actual_id = if self.nodes.iter().any(|n| n.id == id) {
                // Disambiguate: prefix with provider
                if let Some(ref p) = spec.provider {
                    format!("{}_{}", p, id)
                } else {
                    id.clone()
                }
            } else {
                id
            };

            let idx = self.nodes.len();
            self.nodes.push(NodeDef {
                id: actual_id.clone(),
                label: spec.label.clone(),
                provider: spec.provider,
                icon: spec.icon,
                icon_svg: None,
                style: None,
            });
            self.node_map.insert(dedup_key, idx);
            Ok(actual_id)
        } else {
            let idx = self.node_map[&dedup_key];
            Ok(self.nodes[idx].id.clone())
        }
    }
}

// ─── Helpers ───

struct NodeSpec {
    label: String,
    provider: Option<String>,
    icon: Option<String>,
}

/// Parse "provider:icon Label" or just "Label"
/// Provider must match `is_valid_provider` (lowercase alphanumeric + hyphens).
fn parse_node_spec(raw: &str) -> NodeSpec {
    let trimmed = raw.trim();

    // Strip trailing [label] if present (for edge label on last segment)
    let trimmed = if let Some(bracket_start) = trimmed.find('[') {
        trimmed[..bracket_start].trim()
    } else {
        trimmed
    };

    if let Some(colon_idx) = trimmed.find(':') {
        let prefix = &trimmed[..colon_idx];
        if is_valid_provider(prefix) {
            let after_colon = &trimmed[colon_idx + 1..];
            // Find the space that separates icon name from label
            if let Some(space_idx) = after_colon.find(' ') {
                let icon_part = &after_colon[..space_idx];
                let label = after_colon[space_idx + 1..].trim();
                if !icon_part.is_empty() && !label.is_empty() {
                    return NodeSpec {
                        label: label.to_string(),
                        provider: Some(prefix.to_string()),
                        icon: Some(icon_part.to_lowercase()),
                    };
                }
            }
            // No space: "aws:EC2" — icon is the type, label derived from it
            if !after_colon.is_empty() {
                return NodeSpec {
                    label: after_colon.to_string(),
                    provider: Some(prefix.to_string()),
                    icon: Some(after_colon.to_lowercase()),
                };
            }
        }
    }

    NodeSpec {
        label: trimmed.to_string(),
        provider: None,
        icon: None,
    }
}

/// Extract edge label from bracket syntax: "Node B [some label]" → ("Node B", Some("some label"))
fn extract_bracket_label(part: &str) -> (&str, Option<&str>) {
    let trimmed = part.trim();
    if let Some(open) = trimmed.rfind('[') {
        if let Some(close) = trimmed.rfind(']') {
            if close > open {
                let node = trimmed[..open].trim();
                let label = trimmed[open + 1..close].trim();
                if !label.is_empty() {
                    return (node, Some(label));
                }
            }
        }
    }
    (trimmed, None)
}

/// Convert label to a lowercase ID: "Web Server" → "web_server"
fn to_id(label: &str) -> String {
    label
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_")
}

/// Case-insensitive prefix strip: "Title: foo" with prefix "title:" → Some(" foo")
fn strip_prefix_ci<'b>(s: &'b str, prefix: &str) -> Option<&'b str> {
    if s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Basic parsing ───

    #[test]
    fn test_basic_dsl() {
        let ir = parse_dsl("title: Hello\ndirection: LR\n\nNode A >> Node B").unwrap();
        assert_eq!(ir.metadata.title, Some("Hello".to_string()));
        assert_eq!(ir.metadata.direction, "LR");
        assert_eq!(ir.nodes.len(), 2);
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.edges[0].from, "node_a");
        assert_eq!(ir.edges[0].to, "node_b");
    }

    #[test]
    fn test_edge_chain() {
        let ir = parse_dsl("A >> B >> C >> D").unwrap();
        assert_eq!(ir.nodes.len(), 4);
        assert_eq!(ir.edges.len(), 3);
        assert_eq!(ir.edges[0].from, "a");
        assert_eq!(ir.edges[0].to, "b");
        assert_eq!(ir.edges[2].from, "c");
        assert_eq!(ir.edges[2].to, "d");
    }

    #[test]
    fn test_comments() {
        let ir = parse_dsl("# This is a comment\n// Another\nNode A >> Node B").unwrap();
        assert_eq!(ir.nodes.len(), 2);
    }

    #[test]
    fn test_standalone_node() {
        let ir = parse_dsl("My Node\nMy Node >> Other").unwrap();
        assert_eq!(ir.nodes.len(), 2);
        assert_eq!(ir.nodes[0].label, "My Node");
    }

    #[test]
    fn test_version_is_set() {
        let ir = parse_dsl("A >> B").unwrap();
        assert_eq!(ir.version, "1.0.0");
    }

    #[test]
    fn test_node_dedup() {
        let ir = parse_dsl("Node A >> Node B\nNode B >> Node C\nNode A >> Node C").unwrap();
        assert_eq!(ir.nodes.len(), 3);
    }

    // ─── Edge labels with [] ───

    #[test]
    fn test_edge_with_bracket_label() {
        let ir = parse_dsl("Node A >> Node B [connects to]").unwrap();
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.edges[0].label, Some("connects to".to_string()));
    }

    #[test]
    fn test_edge_chain_label_only_on_last() {
        let ir = parse_dsl("A >> B >> C [final hop]").unwrap();
        assert_eq!(ir.edges.len(), 2);
        assert_eq!(ir.edges[0].label, None);
        assert_eq!(ir.edges[1].label, Some("final hop".to_string()));
    }

    #[test]
    fn test_edge_label_with_provider() {
        let ir = parse_dsl("use aws\naws:EC2 Web >> aws:RDS DB [SQL queries]").unwrap();
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.edges[0].label, Some("SQL queries".to_string()));
    }

    #[test]
    fn test_edge_label_with_colon_inside() {
        let ir = parse_dsl("A >> B [port: 5432]").unwrap();
        assert_eq!(ir.edges[0].label, Some("port: 5432".to_string()));
    }

    #[test]
    fn test_edge_no_label() {
        let ir = parse_dsl("A >> B").unwrap();
        assert_eq!(ir.edges[0].label, None);
    }

    // ─── Metadata ───

    #[test]
    fn test_theme() {
        let ir = parse_dsl("theme: dark\nNode A >> Node B").unwrap();
        assert_eq!(ir.metadata.theme, "dark");
    }

    #[test]
    fn test_invalid_direction_error() {
        let result = parse_dsl("direction: XY\nNode A >> Node B");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid direction"));
    }

    // ─── use statements ───

    #[test]
    fn test_use_statement() {
        let ir = parse_dsl("use aws\nuse gcp from github:org/repo\nNode A >> Node B").unwrap();
        assert_eq!(ir.metadata.provider_sources.len(), 2);
        assert_eq!(ir.metadata.provider_sources["aws"], None);
        assert_eq!(
            ir.metadata.provider_sources["gcp"],
            Some("github:org/repo".to_string())
        );
    }

    #[test]
    fn test_use_from_url() {
        let ir = parse_dsl("use aws from https://example.com/icons\nNode A >> Node B").unwrap();
        assert_eq!(
            ir.metadata.provider_sources["aws"],
            Some("https://example.com/icons".to_string())
        );
    }

    #[test]
    fn test_use_empty_provider_error() {
        let result = parse_dsl("use \nNode A >> Node B");
        assert!(result.is_err());
    }

    #[test]
    fn test_use_from_empty_source_error() {
        let result = parse_dsl("use aws from \nNode A >> Node B");
        assert!(result.is_err());
    }

    // ─── Provider nodes ───

    #[test]
    fn test_provider_node() {
        let ir = parse_dsl("use aws\naws:EC2 Web Server >> aws:RDS Database").unwrap();
        assert_eq!(ir.nodes.len(), 2);
        assert_eq!(ir.nodes[0].provider, Some("aws".to_string()));
        assert_eq!(ir.nodes[0].icon, Some("ec2".to_string()));
        assert_eq!(ir.nodes[0].label, "Web Server");
        assert_eq!(ir.nodes[0].id, "web_server");
        assert_eq!(ir.nodes[1].provider, Some("aws".to_string()));
        assert_eq!(ir.nodes[1].icon, Some("rds".to_string()));
        assert_eq!(ir.nodes[1].label, "Database");
    }

    #[test]
    fn test_hyphenated_provider_node() {
        let ir = parse_dsl("use my-cloud\nmy-cloud:vm App >> Node B").unwrap();
        assert_eq!(ir.nodes[0].provider, Some("my-cloud".to_string()));
        assert_eq!(ir.nodes[0].icon, Some("vm".to_string()));
        assert_eq!(ir.nodes[0].label, "App");
    }

    #[test]
    fn test_same_label_different_provider_not_merged() {
        let ir = parse_dsl("use aws\nuse gcp\naws:EC2 App >> gcp:GCE App").unwrap();
        assert_eq!(ir.nodes.len(), 2);
        // Second node should be disambiguated
        assert_ne!(ir.nodes[0].id, ir.nodes[1].id);
    }

    // ─── Clusters ───

    #[test]
    fn test_cluster() {
        let ir = parse_dsl("cluster My Group {\n  Node A\n  Node B\n}\nNode A >> Node B").unwrap();
        assert_eq!(ir.clusters.len(), 1);
        assert_eq!(ir.clusters[0].label, "My Group");
        assert_eq!(ir.clusters[0].id, "my_group");
        assert_eq!(ir.clusters[0].children, vec!["node_a", "node_b"]);
    }

    #[test]
    fn test_provider_cluster() {
        let ir = parse_dsl(
            "use aws\ncluster:aws:vpc Production VPC {\n  aws:EC2 Web\n}\naws:EC2 Web >> Node B",
        )
        .unwrap();
        assert_eq!(ir.clusters.len(), 1);
        assert_eq!(ir.clusters[0].provider, Some("aws".to_string()));
        assert_eq!(ir.clusters[0].cluster_type, Some("vpc".to_string()));
        assert_eq!(ir.clusters[0].label, "Production VPC");
        assert_eq!(ir.clusters[0].children, vec!["web"]);
    }

    #[test]
    fn test_edges_inside_cluster_add_children() {
        let dsl = "cluster Backend {\n  API >> Database\n  API >> Cache\n}";
        let ir = parse_dsl(dsl).unwrap();
        assert_eq!(ir.clusters[0].children.len(), 3);
        assert!(ir.clusters[0].children.contains(&"api".to_string()));
        assert!(ir.clusters[0].children.contains(&"database".to_string()));
        assert!(ir.clusters[0].children.contains(&"cache".to_string()));
        assert_eq!(ir.edges.len(), 2);
    }

    #[test]
    fn test_unclosed_cluster_error() {
        let result = parse_dsl("cluster Backend {\n  API\n  DB");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unclosed cluster"));
    }

    #[test]
    fn test_cluster_no_label_error() {
        let result = parse_dsl("cluster {\n  Node A\n}");
        assert!(result.is_err());
    }

    #[test]
    fn test_cluster_invalid_provider_error() {
        let result = parse_dsl("cluster:INVALID:vpc My VPC {\n  A\n}\nA >> B");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid provider"));
    }

    #[test]
    fn test_cluster_invalid_type_error() {
        let result = parse_dsl("cluster:aws:INVALID! My VPC {\n  A\n}\nA >> B");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid cluster type"));
    }

    // ─── Error cases ───

    #[test]
    fn test_empty_diagram_error() {
        let result = parse_dsl("title: Empty\n# nothing here");
        assert!(result.is_err());
    }

    // ─── Helpers ───

    #[test]
    fn test_to_id() {
        assert_eq!(to_id("Web Server"), "web_server");
        assert_eq!(to_id("API Gateway"), "api_gateway");
        assert_eq!(to_id("  Trimmed  "), "trimmed");
        assert_eq!(to_id("node-with-dashes"), "node_with_dashes");
    }

    #[test]
    fn test_is_valid_provider() {
        assert!(is_valid_provider("aws"));
        assert!(is_valid_provider("my-cloud"));
        assert!(is_valid_provider("k8s"));
        assert!(!is_valid_provider(""));
        assert!(!is_valid_provider("AWS"));
        assert!(!is_valid_provider("my cloud"));
        assert!(!is_valid_provider("bad!name"));
    }

    // ─── Integration ───

    #[test]
    fn test_full_diagram() {
        let dsl = r#"
title: AWS Web Architecture
direction: LR
theme: default
use aws

aws:ELB Load Balancer >> aws:EC2 Web Server >> aws:RDS Database
aws:EC2 Web Server >> aws:S3 Static Assets

cluster:aws:region US East 1 {
  aws:ELB Load Balancer
  aws:EC2 Web Server
  aws:RDS Database
  aws:S3 Static Assets
}

cluster:aws:vpc Production VPC {
  aws:EC2 Web Server
  aws:RDS Database
}
"#;
        let ir = parse_dsl(dsl).unwrap();
        assert_eq!(ir.metadata.title, Some("AWS Web Architecture".to_string()));
        assert_eq!(ir.metadata.direction, "LR");
        assert_eq!(ir.nodes.len(), 4);
        assert_eq!(ir.edges.len(), 3);
        assert_eq!(ir.clusters.len(), 2);
        assert_eq!(ir.clusters[0].provider, Some("aws".to_string()));
        assert_eq!(ir.clusters[0].cluster_type, Some("region".to_string()));
        assert_eq!(ir.clusters[0].children.len(), 4);
        assert_eq!(ir.clusters[1].children.len(), 2);
        assert!(ir.metadata.provider_sources.contains_key("aws"));
    }

    #[test]
    fn test_multiple_providers() {
        let dsl = r#"
use aws
use gcp from github:my-org/icons

aws:EC2 Compute >> gcp:cloud-sql Database
"#;
        let ir = parse_dsl(dsl).unwrap();
        assert_eq!(ir.nodes.len(), 2);
        assert_eq!(ir.nodes[0].provider, Some("aws".to_string()));
        assert_eq!(ir.nodes[1].provider, Some("gcp".to_string()));
        assert_eq!(ir.metadata.provider_sources.len(), 2);
    }
}
