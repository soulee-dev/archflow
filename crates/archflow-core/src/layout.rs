use std::collections::{HashMap, HashSet, VecDeque};

use crate::error::ArchflowError;
use crate::model::DiagramIR;

const NODE_WIDTH: f64 = 160.0;
const NODE_HEIGHT: f64 = 60.0;
const NODE_HEIGHT_WITH_ICON: f64 = 80.0;
const H_SPACING: f64 = 120.0;
const V_SPACING: f64 = 120.0;
const CLUSTER_PADDING: f64 = 50.0;
const CLUSTER_LABEL_HEIGHT: f64 = 36.0;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct LayoutEdge {
    pub from: String,
    pub to: String,
    pub points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct LayoutCluster {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub nodes: Vec<LayoutNode>,
    pub edges: Vec<LayoutEdge>,
    pub clusters: Vec<LayoutCluster>,
    pub width: f64,
    pub height: f64,
}

pub fn compute_layout(ir: &DiagramIR) -> Result<LayoutResult, ArchflowError> {
    let is_lr = ir.metadata.direction == "LR";
    let node_ids: Vec<&str> = ir.nodes.iter().map(|n| n.id.as_str()).collect();
    let node_set: HashSet<&str> = node_ids.iter().copied().collect();

    // Build adjacency for rank assignment
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for id in &node_ids {
        in_degree.insert(id, 0);
        adj.insert(id, Vec::new());
    }
    for edge in &ir.edges {
        if node_set.contains(edge.from.as_str()) && node_set.contains(edge.to.as_str()) {
            adj.get_mut(edge.from.as_str()).unwrap().push(&edge.to);
            *in_degree.get_mut(edge.to.as_str()).unwrap() += 1;
        }
    }

    // Topological sort (Kahn's algorithm) to assign ranks
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut ranks: HashMap<&str, usize> = HashMap::new();
    for (&id, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(id);
            ranks.insert(id, 0);
        }
    }

    while let Some(node) = queue.pop_front() {
        let rank = ranks[node];
        for &next in adj.get(node).unwrap_or(&Vec::new()) {
            let next_rank = ranks.get(next).copied().unwrap_or(0).max(rank + 1);
            ranks.insert(next, next_rank);
            let deg = in_degree.get_mut(next).unwrap();
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(next);
            }
        }
    }

    // Nodes without edges get rank 0
    for id in &node_ids {
        ranks.entry(id).or_insert(0);
    }

    // Group nodes by rank
    let max_rank = ranks.values().copied().max().unwrap_or(0);
    let mut layers: Vec<Vec<&str>> = vec![Vec::new(); max_rank + 1];
    for (&id, &rank) in &ranks {
        layers[rank].push(id);
    }

    // Sort within each layer for determinism
    for layer in &mut layers {
        layer.sort();
    }

    // Build per-node height map based on icon_svg presence
    let node_height_map: HashMap<&str, f64> = ir
        .nodes
        .iter()
        .map(|n| {
            let h = if n.icon_svg.is_some() {
                NODE_HEIGHT_WITH_ICON
            } else {
                NODE_HEIGHT
            };
            (n.id.as_str(), h)
        })
        .collect();

    // Compute max height per rank (for consistent row/column spacing)
    let max_height_per_rank: Vec<f64> = layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|id| node_height_map.get(id).copied().unwrap_or(NODE_HEIGHT))
                .fold(NODE_HEIGHT, f64::max)
        })
        .collect();

    // Assign positions using cumulative rank offsets for per-rank height
    let mut node_positions: HashMap<&str, (f64, f64)> = HashMap::new();
    let mut rank_offset = 0.0;
    for (rank, layer) in layers.iter().enumerate() {
        let rank_height = max_height_per_rank[rank];
        for (pos, &id) in layer.iter().enumerate() {
            let node_h = node_height_map.get(id).copied().unwrap_or(NODE_HEIGHT);
            let (x, y) = if is_lr {
                (
                    rank_offset,
                    pos as f64 * (NODE_HEIGHT_WITH_ICON + V_SPACING),
                )
            } else {
                (
                    pos as f64 * (NODE_WIDTH + H_SPACING),
                    rank_offset + (rank_height - node_h) / 2.0, // center vertically in rank
                )
            };
            node_positions.insert(id, (x, y));
        }
        rank_offset += if is_lr {
            NODE_WIDTH + H_SPACING
        } else {
            rank_height + V_SPACING
        };
    }

    // Build layout nodes
    let mut layout_nodes: Vec<LayoutNode> = ir
        .nodes
        .iter()
        .map(|n| {
            let (x, y) = node_positions
                .get(n.id.as_str())
                .copied()
                .unwrap_or((0.0, 0.0));
            let height = node_height_map
                .get(n.id.as_str())
                .copied()
                .unwrap_or(NODE_HEIGHT);
            LayoutNode {
                id: n.id.clone(),
                x,
                y,
                width: NODE_WIDTH,
                height,
            }
        })
        .collect();

    // Build layout edges (simple straight lines center-to-center)
    let mut layout_edges: Vec<LayoutEdge> = ir
        .edges
        .iter()
        .filter(|e| node_set.contains(e.from.as_str()) && node_set.contains(e.to.as_str()))
        .map(|e| {
            let (fx, fy) = node_positions[e.from.as_str()];
            let (tx, ty) = node_positions[e.to.as_str()];
            let from_h = node_height_map
                .get(e.from.as_str())
                .copied()
                .unwrap_or(NODE_HEIGHT);
            let to_h = node_height_map
                .get(e.to.as_str())
                .copied()
                .unwrap_or(NODE_HEIGHT);

            let from_cx = fx + NODE_WIDTH / 2.0;
            let from_cy = fy + from_h / 2.0;
            let to_cx = tx + NODE_WIDTH / 2.0;
            let to_cy = ty + to_h / 2.0;

            // Connect from edge of source to edge of target
            let (start, end) = if is_lr {
                ((fx + NODE_WIDTH, from_cy), (tx, to_cy))
            } else {
                ((from_cx, fy + from_h), (to_cx, ty))
            };

            LayoutEdge {
                from: e.from.clone(),
                to: e.to.clone(),
                points: vec![start, end],
            }
        })
        .collect();

    // Build layout clusters (bounding box of children)
    let mut layout_clusters: Vec<LayoutCluster> = ir
        .clusters
        .iter()
        .map(|c| {
            let child_nodes: Vec<(f64, f64, f64)> = c
                .children
                .iter()
                .filter_map(|cid| {
                    let pos = node_positions.get(cid.as_str()).copied()?;
                    let h = node_height_map
                        .get(cid.as_str())
                        .copied()
                        .unwrap_or(NODE_HEIGHT);
                    Some((pos.0, pos.1, h))
                })
                .collect();

            if child_nodes.is_empty() {
                return LayoutCluster {
                    id: c.id.clone(),
                    x: 0.0,
                    y: 0.0,
                    width: NODE_WIDTH + CLUSTER_PADDING * 2.0,
                    height: NODE_HEIGHT + CLUSTER_PADDING * 2.0 + CLUSTER_LABEL_HEIGHT,
                };
            }

            let min_x = child_nodes
                .iter()
                .map(|p| p.0)
                .fold(f64::INFINITY, f64::min);
            let min_y = child_nodes
                .iter()
                .map(|p| p.1)
                .fold(f64::INFINITY, f64::min);
            let max_x = child_nodes
                .iter()
                .map(|p| p.0 + NODE_WIDTH)
                .fold(f64::NEG_INFINITY, f64::max);
            let max_y = child_nodes
                .iter()
                .map(|p| p.1 + p.2) // use per-node height
                .fold(f64::NEG_INFINITY, f64::max);

            LayoutCluster {
                id: c.id.clone(),
                x: min_x - CLUSTER_PADDING,
                y: min_y - CLUSTER_PADDING - CLUSTER_LABEL_HEIGHT,
                width: (max_x - min_x) + CLUSTER_PADDING * 2.0,
                height: (max_y - min_y) + CLUSTER_PADDING * 2.0 + CLUSTER_LABEL_HEIGHT,
            }
        })
        .collect();

    // Find minimum coordinates (clusters can go negative due to padding/label)
    let min_x = layout_nodes
        .iter()
        .map(|n| n.x)
        .chain(layout_clusters.iter().map(|c| c.x))
        .fold(0.0_f64, f64::min);
    let min_y = layout_nodes
        .iter()
        .map(|n| n.y)
        .chain(layout_clusters.iter().map(|c| c.y))
        .fold(0.0_f64, f64::min);

    // Shift everything so all coordinates are positive
    let padding = 60.0;
    let shift_x = -min_x;
    let shift_y = -min_y;

    for n in &mut layout_nodes {
        n.x += shift_x;
        n.y += shift_y;
    }
    for e in &mut layout_edges {
        for p in &mut e.points {
            p.0 += shift_x;
            p.1 += shift_y;
        }
    }
    for c in &mut layout_clusters {
        c.x += shift_x;
        c.y += shift_y;
    }

    let max_x = layout_nodes
        .iter()
        .map(|n| n.x + n.width)
        .chain(layout_clusters.iter().map(|c| c.x + c.width))
        .fold(0.0_f64, f64::max);
    let max_y = layout_nodes
        .iter()
        .map(|n| n.y + n.height)
        .chain(layout_clusters.iter().map(|c| c.y + c.height))
        .fold(0.0_f64, f64::max);

    Ok(LayoutResult {
        nodes: layout_nodes,
        edges: layout_edges,
        clusters: layout_clusters,
        width: max_x + padding * 2.0,
        height: max_y + padding * 2.0,
    })
}
