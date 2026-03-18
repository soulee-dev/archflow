/**
 * Archflow DSL Parser
 *
 * Syntax:
 *   title: My Diagram
 *   direction: LR
 *
 *   Node A >> Node B >> Node C
 *   Node A >> Node B : label text
 *
 *   cluster My Group {
 *     Node A
 *     Node B
 *   }
 *
 * Node IDs are auto-generated from labels (lowercased, spaces to underscores).
 */

export function parseDSL(input) {
  const lines = input.split('\n');
  const ir = {
    version: '1.0.0',
    metadata: { title: '', direction: 'TB', theme: 'default' },
    nodes: [],
    clusters: [],
    edges: [],
  };

  const nodeMap = new Map(); // label -> id
  let i = 0;

  function toId(label) {
    return label.trim().toLowerCase().replace(/[^a-z0-9]+/g, '_').replace(/^_|_$/g, '');
  }

  function ensureNode(label) {
    label = label.trim();
    if (!label) return null;
    const id = toId(label);
    if (!nodeMap.has(label)) {
      nodeMap.set(label, id);
      ir.nodes.push({ id, label });
    }
    return id;
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
        // Each line in cluster is a node name
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
        const fromLabel = parts[j].split(':')[0].trim(); // source never has label
        const fromId = ensureNode(fromLabel);

        let toLabel, edgeLabel, edgeStyle;
        const toPart = parts[j + 1];

        // Check for style: Node A {fill: #red}
        // Check for label: Node A : some label
        // Last segment can have : label
        if (j === parts.length - 2 && toPart.includes(':')) {
          // Could be label
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
