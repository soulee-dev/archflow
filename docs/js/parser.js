/**
 * Archflow DSL Parser
 *
 * Syntax:
 *   title: My Diagram
 *   direction: LR
 *   icon_sources: github:archflow/icons, https://example.com/icons
 *
 *   Node A >> Node B >> Node C
 *   Node A >> Node B : label text
 *   aws:EC2 Web Server >> aws:RDS Database
 *
 *   cluster My Group {
 *     Node A
 *     Node B
 *   }
 *
 *   cluster:aws:vpc My VPC {
 *     Node A
 *   }
 *
 * Node IDs are auto-generated from labels (lowercased, spaces to underscores).
 * Provider syntax: provider:type Label (e.g., aws:EC2 Web Server)
 */

export function parseDSL(input) {
  const lines = input.split('\n');
  const ir = {
    version: '1.0.0',
    metadata: { title: '', direction: 'TB', theme: 'default', icon_sources: [] },
    nodes: [],
    clusters: [],
    edges: [],
  };

  const nodeMap = new Map(); // label -> id
  let i = 0;

  function toId(label) {
    return label.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_|_$/g, '');
  }

  /**
   * Parse a node spec that may include provider:type prefix.
   * Returns { label, provider, icon } where provider/icon may be null.
   * Format: "provider:type Label" or just "Label"
   */
  function parseNodeSpec(raw) {
    raw = raw.trim();
    const providerMatch = raw.match(/^([a-z][a-z0-9]*):([a-zA-Z][a-zA-Z0-9]*)\s+(.+)$/);
    if (providerMatch) {
      return {
        label: providerMatch[3].trim(),
        provider: providerMatch[1],
        icon: providerMatch[2].toLowerCase(),
      };
    }
    return { label: raw, provider: null, icon: null };
  }

  function ensureNode(rawLabel) {
    const spec = parseNodeSpec(rawLabel);
    const label = spec.label;
    if (!label) return null;
    const id = toId(label);
    if (!nodeMap.has(label)) {
      nodeMap.set(label, id);
      const node = { id, label };
      if (spec.provider) node.provider = spec.provider;
      if (spec.icon) node.icon = spec.icon;
      ir.nodes.push(node);
    }
    return id;
  }

  function extractLabel(rawLabel) {
    const spec = parseNodeSpec(rawLabel);
    return spec.label;
  }

  function parseStyle(styleStr) {
    const style = {};
    const pairs = styleStr.split(',');
    for (const pair of pairs) {
      const [key, val] = pair.split(':').map(s => s.trim());
      if (key && val) {
        const k = key.replace(/-/g, '_');
        style[k] = isNaN(Number(val)) ? val : Number(val);
      }
    }
    return Object.keys(style).length > 0 ? style : undefined;
  }

  while (i < lines.length) {
    let line = lines[i].trimEnd();
    const trimmed = line.trim();
    i++;

    // Skip empty lines and comments
    if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('//')) continue;

    // Metadata: title: ...
    const titleMatch = trimmed.match(/^title\s*:\s*(.+)$/i);
    if (titleMatch) {
      ir.metadata.title = titleMatch[1].trim();
      continue;
    }

    // Metadata: direction: ...
    const dirMatch = trimmed.match(/^direction\s*:\s*(TB|LR|BT|RL)$/i);
    if (dirMatch) {
      ir.metadata.direction = dirMatch[1].toUpperCase();
      continue;
    }

    // Metadata: theme: ...
    const themeMatch = trimmed.match(/^theme\s*:\s*(.+)$/i);
    if (themeMatch) {
      ir.metadata.theme = themeMatch[1].trim();
      continue;
    }

    // Metadata: icon_sources: ...
    const iconSourcesMatch = trimmed.match(/^icon_sources\s*:\s*(.+)$/i);
    if (iconSourcesMatch) {
      ir.metadata.icon_sources = iconSourcesMatch[1].split(',').map(s => s.trim()).filter(Boolean);
      continue;
    }

    // Cluster with provider: cluster:provider:type Name { ... }
    const providerClusterMatch = trimmed.match(/^cluster:([a-z][a-z0-9]*):([a-z][a-z0-9]*)\s+(.+?)\s*\{$/i);
    if (providerClusterMatch) {
      const clusterProvider = providerClusterMatch[1];
      const clusterType = providerClusterMatch[2];
      const clusterLabel = providerClusterMatch[3].trim();
      const clusterId = toId(clusterLabel);
      const children = [];

      while (i < lines.length) {
        const cline = lines[i].trim();
        i++;
        if (cline === '}') break;
        if (!cline || cline.startsWith('#') || cline.startsWith('//')) continue;
        const nodeId = ensureNode(cline);
        if (nodeId) children.push(nodeId);
      }

      ir.clusters.push({
        id: clusterId,
        label: clusterLabel,
        children,
        provider: clusterProvider,
        cluster_type: clusterType,
      });
      continue;
    }

    // Cluster: cluster Name { ... }
    const clusterMatch = trimmed.match(/^cluster\s+(.+?)\s*\{$/i);
    if (clusterMatch) {
      const clusterLabel = clusterMatch[1].trim();
      const clusterId = toId(clusterLabel);
      const children = [];

      while (i < lines.length) {
        const cline = lines[i].trim();
        i++;
        if (cline === '}') break;
        if (!cline || cline.startsWith('#') || cline.startsWith('//')) continue;
        // Each line in cluster is a node name (possibly with provider prefix)
        const nodeLabel = cline;
        const nodeId = ensureNode(nodeLabel);
        if (nodeId) children.push(nodeId);
      }

      ir.clusters.push({ id: clusterId, label: clusterLabel, children });
      continue;
    }

    // Edge chain: A >> B >> C  or  A >> B : label
    if (trimmed.includes('>>')) {
      // Split by >> but handle labels with :
      const parts = trimmed.split('>>').map(s => s.trim());

      for (let j = 0; j < parts.length - 1; j++) {
        const fromRaw = parts[j].split(':').length > 2
          ? parts[j]  // provider:type label
          : parts[j].includes(':') && !parts[j].match(/^[a-z]+:[A-Z]/)
            ? parts[j].split(':')[0].trim()
            : parts[j];
        const fromId = ensureNode(fromRaw);

        let toRaw, edgeLabel;
        const toPart = parts[j + 1];

        // Last segment can have : for edge label (but not provider:type)
        if (j === parts.length - 2) {
          // Check if the colon is for a provider prefix or an edge label
          const provCheck = toPart.match(/^([a-z][a-z0-9]*):([a-zA-Z][a-zA-Z0-9]*)\s+/);
          if (provCheck) {
            // Has provider prefix — check if there's ANOTHER colon for edge label
            const afterProvider = toPart.substring(provCheck[0].length);
            const colonIdx = afterProvider.indexOf(':');
            if (colonIdx >= 0) {
              toRaw = toPart.substring(0, provCheck[0].length + colonIdx).trim();
              edgeLabel = afterProvider.substring(colonIdx + 1).trim();
            } else {
              toRaw = toPart;
            }
          } else if (toPart.includes(':')) {
            const colonIdx = toPart.indexOf(':');
            toRaw = toPart.substring(0, colonIdx).trim();
            edgeLabel = toPart.substring(colonIdx + 1).trim();
          } else {
            toRaw = toPart;
          }
        } else {
          toRaw = toPart;
        }

        const toId = ensureNode(toRaw);
        if (fromId && toId) {
          const edge = { from: fromId, to: toId };
          if (edgeLabel) edge.label = edgeLabel;
          ir.edges.push(edge);
        }
      }
      continue;
    }

    // Single edge: A -> B or A -> B : label (also support ->)
    if (trimmed.includes('->')) {
      const parts = trimmed.split('->').map(s => s.trim());
      for (let j = 0; j < parts.length - 1; j++) {
        const fromLabel = parts[j];
        const fromId = ensureNode(fromLabel);

        let toLabel, edgeLabel;
        const toPart = parts[j + 1];

        if (j === parts.length - 2 && toPart.includes(':')) {
          const colonIdx = toPart.indexOf(':');
          toLabel = toPart.substring(0, colonIdx).trim();
          edgeLabel = toPart.substring(colonIdx + 1).trim();
        } else {
          toLabel = toPart;
        }

        const toId = ensureNode(toLabel);
        if (fromId && toId) {
          const edge = { from: fromId, to: toId };
          if (edgeLabel) edge.label = edgeLabel;
          ir.edges.push(edge);
        }
      }
      continue;
    }

    // Standalone node (just a name on its own line, outside cluster)
    if (trimmed && !trimmed.includes('{') && !trimmed.includes('}')) {
      ensureNode(trimmed);
    }
  }

  return ir;
}
