use crate::model::{ClusterDef, CustomThemeDef, Style};

/// A color pair for nodes: fill + stroke
#[derive(Debug, Clone)]
pub struct NodeColor {
    pub fill: String,
    pub stroke: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
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

impl Theme {
    /// Look up a theme by name. Falls back to default for unknown names.
    pub fn by_name(name: &str) -> Self {
        match name {
            "dark" => Self::dark(),
            "minimal" => Self::minimal(),
            "ocean" => Self::ocean(),
            "sunset" => Self::sunset(),
            "forest" => Self::forest(),
            _ => Self::default(),
        }
    }

    /// Build a theme from a base name + optional custom overrides.
    pub fn from_ir(name: &str, custom: &Option<CustomThemeDef>) -> Self {
        let mut theme = Self::by_name(name);
        if let Some(c) = custom {
            theme.apply_overrides(c);
        }
        theme
    }

    /// Apply custom theme overrides onto this theme.
    fn apply_overrides(&mut self, c: &CustomThemeDef) {
        if let Some(ref v) = c.background {
            self.background = v.clone();
        }
        if let Some(ref v) = c.node_text_color {
            self.node_text_color = v.clone();
        }
        if let Some(v) = c.node_corner_radius {
            self.node_corner_radius = v;
        }
        if let Some(ref v) = c.cluster_stroke {
            self.cluster_stroke = v.clone();
        }
        if let Some(ref v) = c.cluster_text_color {
            self.cluster_text_color = v.clone();
        }
        if let Some(ref v) = c.edge_stroke {
            self.edge_stroke = v.clone();
        }
        if let Some(v) = c.edge_stroke_width {
            self.edge_stroke_width = v;
        }
        if let Some(ref v) = c.font_family {
            self.font_family = v.clone();
        }
        if let Some(v) = c.font_size {
            self.font_size = v;
            self.label_font_size = v - 2.0;
        }
        if let Some(v) = c.node_shadow {
            self.node_shadow = v;
        }
        if let Some(ref palette) = c.node_palette {
            self.node_palette = palette
                .iter()
                .map(|nc| NodeColor {
                    fill: nc.fill.clone(),
                    stroke: nc.stroke.clone(),
                })
                .collect();
        }
        if let Some(ref fills) = c.cluster_fills {
            self.cluster_fills = fills.clone();
        }
    }

    /// List all available theme names.
    pub fn available() -> Vec<&'static str> {
        vec!["default", "dark", "minimal", "ocean", "sunset", "forest"]
    }

    fn base() -> Self {
        Self {
            name: "default".into(),
            background: "#FAFBFC".into(),
            node_palette: vec![],
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 10.0,
            cluster_fills: vec![],
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

    fn dark() -> Self {
        Self {
            name: "dark".into(),
            background: "#1A1B26".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#7AA2F7".into(),
                    stroke: "#6A92E7".into(),
                },
                NodeColor {
                    fill: "#BB9AF7".into(),
                    stroke: "#AB8AE7".into(),
                },
                NodeColor {
                    fill: "#9ECE6A".into(),
                    stroke: "#8EBE5A".into(),
                },
                NodeColor {
                    fill: "#FF9E64".into(),
                    stroke: "#EF8E54".into(),
                },
                NodeColor {
                    fill: "#F7768E".into(),
                    stroke: "#E7667E".into(),
                },
                NodeColor {
                    fill: "#2AC3DE".into(),
                    stroke: "#1AB3CE".into(),
                },
                NodeColor {
                    fill: "#E0AF68".into(),
                    stroke: "#D09F58".into(),
                },
                NodeColor {
                    fill: "#A9B1D6".into(),
                    stroke: "#99A1C6".into(),
                },
            ],
            node_text_color: "#1A1B26".into(),
            node_corner_radius: 10.0,
            cluster_fills: vec![
                "rgba(122, 162, 247, 0.08)".into(),
                "rgba(187, 154, 247, 0.08)".into(),
                "rgba(158, 206, 106, 0.08)".into(),
                "rgba(255, 158, 100, 0.08)".into(),
            ],
            cluster_stroke: "#33364A".into(),
            cluster_text_color: "#A9B1D6".into(),
            cluster_corner_radius: 16.0,
            edge_stroke: "#565F89".into(),
            edge_stroke_width: 1.5,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: true,
        }
    }

    fn minimal() -> Self {
        Self {
            name: "minimal".into(),
            background: "#FFFFFF".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#FFFFFF".into(),
                    stroke: "#D1D5DB".into(),
                },
                NodeColor {
                    fill: "#FFFFFF".into(),
                    stroke: "#D1D5DB".into(),
                },
            ],
            node_text_color: "#111827".into(),
            node_corner_radius: 8.0,
            cluster_fills: vec!["rgba(0, 0, 0, 0.02)".into(), "rgba(0, 0, 0, 0.02)".into()],
            cluster_stroke: "#E5E7EB".into(),
            cluster_text_color: "#6B7280".into(),
            cluster_corner_radius: 12.0,
            edge_stroke: "#9CA3AF".into(),
            edge_stroke_width: 1.0,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: false,
        }
    }

    fn ocean() -> Self {
        Self {
            name: "ocean".into(),
            background: "#F0F9FF".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#0EA5E9".into(),
                    stroke: "#0284C7".into(),
                },
                NodeColor {
                    fill: "#06B6D4".into(),
                    stroke: "#0891B2".into(),
                },
                NodeColor {
                    fill: "#3B82F6".into(),
                    stroke: "#2563EB".into(),
                },
                NodeColor {
                    fill: "#6366F1".into(),
                    stroke: "#4F46E5".into(),
                },
                NodeColor {
                    fill: "#8B5CF6".into(),
                    stroke: "#7C3AED".into(),
                },
                NodeColor {
                    fill: "#14B8A6".into(),
                    stroke: "#0D9488".into(),
                },
            ],
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 12.0,
            cluster_fills: vec![
                "rgba(14, 165, 233, 0.06)".into(),
                "rgba(6, 182, 212, 0.06)".into(),
                "rgba(59, 130, 246, 0.06)".into(),
                "rgba(99, 102, 241, 0.06)".into(),
            ],
            cluster_stroke: "#BAE6FD".into(),
            cluster_text_color: "#0369A1".into(),
            cluster_corner_radius: 16.0,
            edge_stroke: "#7DD3FC".into(),
            edge_stroke_width: 1.5,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: true,
        }
    }

    fn sunset() -> Self {
        Self {
            name: "sunset".into(),
            background: "#FFFBF5".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#F97316".into(),
                    stroke: "#EA580C".into(),
                },
                NodeColor {
                    fill: "#EF4444".into(),
                    stroke: "#DC2626".into(),
                },
                NodeColor {
                    fill: "#F59E0B".into(),
                    stroke: "#D97706".into(),
                },
                NodeColor {
                    fill: "#EC4899".into(),
                    stroke: "#DB2777".into(),
                },
                NodeColor {
                    fill: "#F43F5E".into(),
                    stroke: "#E11D48".into(),
                },
                NodeColor {
                    fill: "#A855F7".into(),
                    stroke: "#9333EA".into(),
                },
            ],
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 10.0,
            cluster_fills: vec![
                "rgba(249, 115, 22, 0.06)".into(),
                "rgba(239, 68, 68, 0.06)".into(),
                "rgba(245, 158, 11, 0.06)".into(),
                "rgba(236, 72, 153, 0.06)".into(),
            ],
            cluster_stroke: "#FED7AA".into(),
            cluster_text_color: "#9A3412".into(),
            cluster_corner_radius: 16.0,
            edge_stroke: "#FDBA74".into(),
            edge_stroke_width: 1.5,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: true,
        }
    }

    fn forest() -> Self {
        Self {
            name: "forest".into(),
            background: "#F0FDF4".into(),
            node_palette: vec![
                NodeColor {
                    fill: "#22C55E".into(),
                    stroke: "#16A34A".into(),
                },
                NodeColor {
                    fill: "#10B981".into(),
                    stroke: "#059669".into(),
                },
                NodeColor {
                    fill: "#14B8A6".into(),
                    stroke: "#0D9488".into(),
                },
                NodeColor {
                    fill: "#84CC16".into(),
                    stroke: "#65A30D".into(),
                },
                NodeColor {
                    fill: "#06B6D4".into(),
                    stroke: "#0891B2".into(),
                },
                NodeColor {
                    fill: "#8B5CF6".into(),
                    stroke: "#7C3AED".into(),
                },
            ],
            node_text_color: "#FFFFFF".into(),
            node_corner_radius: 10.0,
            cluster_fills: vec![
                "rgba(34, 197, 94, 0.06)".into(),
                "rgba(16, 185, 129, 0.06)".into(),
                "rgba(20, 184, 166, 0.06)".into(),
                "rgba(132, 204, 22, 0.06)".into(),
            ],
            cluster_stroke: "#BBF7D0".into(),
            cluster_text_color: "#166534".into(),
            cluster_corner_radius: 16.0,
            edge_stroke: "#86EFAC".into(),
            edge_stroke_width: 1.5,
            font_family: "'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif".into(),
            font_size: 13.0,
            label_font_size: 11.0,
            node_shadow: true,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut t = Self::base();
        t.node_palette = vec![
            NodeColor {
                fill: "#4A90D9".into(),
                stroke: "#3A7BC8".into(),
            },
            NodeColor {
                fill: "#6C5CE7".into(),
                stroke: "#5A4BD6".into(),
            },
            NodeColor {
                fill: "#00B894".into(),
                stroke: "#00A383".into(),
            },
            NodeColor {
                fill: "#E17055".into(),
                stroke: "#D05F44".into(),
            },
            NodeColor {
                fill: "#FD79A8".into(),
                stroke: "#EC6897".into(),
            },
            NodeColor {
                fill: "#00CEC9".into(),
                stroke: "#00BDB8".into(),
            },
            NodeColor {
                fill: "#FDCB6E".into(),
                stroke: "#ECBA5D".into(),
            },
            NodeColor {
                fill: "#636E72".into(),
                stroke: "#525D61".into(),
            },
        ];
        t.cluster_fills = vec![
            "rgba(74, 144, 217, 0.06)".into(),
            "rgba(108, 92, 231, 0.06)".into(),
            "rgba(0, 184, 148, 0.06)".into(),
            "rgba(225, 112, 85, 0.06)".into(),
        ];
        t
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

    /// Resolve cluster style with provider/cluster_type awareness.
    /// Provider presets override theme defaults but are overridden by explicit styles.
    pub fn resolve_cluster_style_with_provider(
        &self,
        cluster_def: &ClusterDef,
        index: usize,
    ) -> ResolvedClusterStyle {
        let preset = provider_cluster_preset(
            cluster_def.provider.as_deref(),
            cluster_def.cluster_type.as_deref(),
        );

        let s = cluster_def.style.as_ref();
        let default_fill = &self.cluster_fills[index % self.cluster_fills.len()];

        ResolvedClusterStyle {
            fill: s
                .and_then(|s| s.fill.clone())
                .or_else(|| preset.as_ref().map(|p| p.fill.clone()))
                .unwrap_or_else(|| default_fill.clone()),
            stroke: s
                .and_then(|s| s.stroke.clone())
                .or_else(|| preset.as_ref().map(|p| p.stroke.clone()))
                .unwrap_or_else(|| self.cluster_stroke.clone()),
            stroke_width: s
                .and_then(|s| s.stroke_width)
                .or_else(|| preset.as_ref().map(|p| p.stroke_width))
                .unwrap_or(1.5),
            text_color: s
                .and_then(|s| s.font_color.clone())
                .or_else(|| preset.as_ref().map(|p| p.text_color.clone()))
                .unwrap_or_else(|| self.cluster_text_color.clone()),
            corner_radius: self.cluster_corner_radius,
        }
    }
}

/// Built-in provider cluster style presets
struct ClusterPreset {
    fill: String,
    stroke: String,
    stroke_width: f64,
    text_color: String,
}

fn provider_cluster_preset(
    provider: Option<&str>,
    cluster_type: Option<&str>,
) -> Option<ClusterPreset> {
    match (provider, cluster_type) {
        (Some("aws"), Some("region")) => Some(ClusterPreset {
            fill: "rgba(255, 153, 0, 0.06)".into(),
            stroke: "#FF9900".into(),
            stroke_width: 2.0,
            text_color: "#CC7A00".into(),
        }),
        (Some("aws"), Some("vpc")) => Some(ClusterPreset {
            fill: "rgba(0, 164, 166, 0.06)".into(),
            stroke: "#00A4A6".into(),
            stroke_width: 1.5,
            text_color: "#007F80".into(),
        }),
        (Some("aws"), Some("subnet")) => Some(ClusterPreset {
            fill: "rgba(0, 164, 166, 0.03)".into(),
            stroke: "#00A4A6".into(),
            stroke_width: 1.0,
            text_color: "#007F80".into(),
        }),
        (Some("gcp"), Some("region")) => Some(ClusterPreset {
            fill: "rgba(66, 133, 244, 0.06)".into(),
            stroke: "#4285F4".into(),
            stroke_width: 2.0,
            text_color: "#3367D6".into(),
        }),
        (Some("gcp"), Some("vpc")) => Some(ClusterPreset {
            fill: "rgba(52, 168, 83, 0.06)".into(),
            stroke: "#34A853".into(),
            stroke_width: 1.5,
            text_color: "#2D8E47".into(),
        }),
        (Some("k8s"), Some("cluster")) => Some(ClusterPreset {
            fill: "rgba(50, 108, 229, 0.06)".into(),
            stroke: "#326CE5".into(),
            stroke_width: 2.0,
            text_color: "#2957C0".into(),
        }),
        (Some("k8s"), Some("namespace")) => Some(ClusterPreset {
            fill: "rgba(50, 108, 229, 0.03)".into(),
            stroke: "#326CE5".into(),
            stroke_width: 1.0,
            text_color: "#2957C0".into(),
        }),
        _ => None,
    }
}
