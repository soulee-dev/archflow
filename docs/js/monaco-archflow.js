/**
 * Monaco Editor language support for Archflow DSL.
 * Mirrors the LSP server's diagnostics + completions logic.
 *
 * TODO: This file duplicates completion/diagnostic data from crates/archflow-lsp.
 *       Extract shared logic into archflow-core (or a new archflow-lsp-logic crate),
 *       expose it via WASM, and consume it here instead of hardcoding.
 */

// ─── Provider icon/cluster data (same as archflow-lsp) ───
// TODO: Load from shared WASM module instead of hardcoding

const PROVIDER_ICONS = {
  aws: [
    'EC2', 'RDS', 'S3', 'Lambda', 'ELB', 'CloudFront', 'SQS', 'SNS',
    'DynamoDB', 'ElastiCache', 'ECS', 'EKS', 'ECR', 'IAM', 'Cognito',
    'CloudWatch', 'CloudFormation', 'Route53', 'ApiGateway', 'Bedrock',
    'SageMaker', 'Kinesis', 'Redshift', 'Athena', 'Glue', 'EMR',
  ],
  gcp: [
    'compute-engine', 'cloud-sql', 'cloud-storage', 'cloud-run',
    'bigquery', 'gke', 'vertex-ai', 'cloud-spanner', 'alloydb',
    'looker', 'apigee', 'anthos',
  ],
  k8s: [
    'pod', 'deployment', 'service', 'ingress', 'stateful-set',
    'config-map', 'secret', 'daemon-set', 'replica-set', 'job',
    'cron-job', 'namespace', 'node', 'persistent-volume',
    'persistent-volume-claim', 'service-account',
  ],
};

const CLUSTER_TYPES = {
  aws: ['region', 'vpc', 'subnet', 'account', 'cloud'],
  gcp: ['region', 'vpc', 'subnet', 'project', 'zone'],
  k8s: ['cluster', 'namespace'],
};

const THEMES = ['default', 'dark', 'minimal', 'ocean', 'sunset', 'forest'];
const DIRECTIONS = ['TB', 'LR'];
const PROVIDERS = Object.keys(PROVIDER_ICONS);

// ─── Language Registration ───

export function registerArchflowLanguage(monaco) {
  monaco.languages.register({ id: 'archflow', extensions: ['.archflow'] });

  // Monarch tokenizer (syntax highlighting)
  monaco.languages.setMonarchTokensProvider('archflow', {
    tokenizer: {
      root: [
        // Comments
        [/#.*$/, 'comment'],
        [/\/\/.*$/, 'comment'],

        // Keywords at start of line
        [/^(title|direction|theme)\s*:/, 'keyword'],
        [/^use\b/, 'keyword'],
        [/^cluster\b/, 'keyword'],

        // Cluster typed prefix
        [/cluster:[a-z][a-z0-9-]*:[a-z][a-z0-9-]*/, 'keyword'],

        // Edge operators
        [/>>/, 'operator'],
        [/->/, 'operator'],

        // Edge labels in brackets
        [/\[/, 'delimiter.bracket', '@edgeLabel'],

        // Provider:Icon (e.g. aws:EC2)
        [/[a-z][a-z0-9-]*:[A-Za-z][A-Za-z0-9-]*/, 'type'],

        // Braces
        [/[{}]/, 'delimiter.bracket'],

        // Everything else
        [/[^\s#/{}\[\]>-]+/, 'identifier'],
      ],
      edgeLabel: [
        [/[^\]]+/, 'string'],
        [/\]/, 'delimiter.bracket', '@pop'],
      ],
    },
  });

  // Language configuration
  monaco.languages.setLanguageConfiguration('archflow', {
    comments: { lineComment: '#' },
    brackets: [['{', '}'], ['[', ']']],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
    ],
  });

  // Completion provider
  monaco.languages.registerCompletionItemProvider('archflow', {
    triggerCharacters: [':', ' '],
    provideCompletionItems: (model, position) => {
      const line = model.getLineContent(position.lineNumber);
      const beforeCursor = line.substring(0, position.column - 1).trim();
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };

      const suggestions = [];

      // Top-level keywords
      if (beforeCursor === '' || /^(t|ti|d|di|th|u|us|c|cl)$/i.test(beforeCursor)) {
        const keywords = [
          { label: 'title', insert: 'title: ', detail: 'Set diagram title' },
          { label: 'direction', insert: 'direction: ', detail: 'Set layout direction (TB or LR)' },
          { label: 'theme', insert: 'theme: ', detail: 'Set theme' },
          { label: 'use', insert: 'use ', detail: 'Import a provider' },
          { label: 'cluster', insert: 'cluster ', detail: 'Define a cluster group' },
        ];
        for (const kw of keywords) {
          suggestions.push({
            label: kw.label,
            kind: monaco.languages.CompletionItemKind.Keyword,
            insertText: kw.insert,
            detail: kw.detail,
            range,
          });
        }
      }

      // After "use "
      if (/^use\s*$/i.test(beforeCursor)) {
        for (const p of PROVIDERS) {
          suggestions.push({
            label: p,
            kind: monaco.languages.CompletionItemKind.Module,
            detail: 'Provider',
            insertText: p,
            range,
          });
        }
      }

      // After "direction:"
      if (/^direction\s*:\s*/i.test(beforeCursor)) {
        for (const d of DIRECTIONS) {
          suggestions.push({
            label: d,
            kind: monaco.languages.CompletionItemKind.EnumMember,
            insertText: d,
            range,
          });
        }
      }

      // After "theme:"
      if (/^theme\s*:\s*/i.test(beforeCursor)) {
        for (const t of THEMES) {
          suggestions.push({
            label: t,
            kind: monaco.languages.CompletionItemKind.EnumMember,
            insertText: t,
            range,
          });
        }
      }

      // Provider icon completions (e.g. "aws:" triggers icon list)
      const colonMatch = beforeCursor.match(/([a-z][a-z0-9-]*):([A-Za-z0-9-]*)$/);
      if (colonMatch) {
        const provider = colonMatch[1];
        const icons = PROVIDER_ICONS[provider] || [];
        const iconRange = {
          startLineNumber: position.lineNumber,
          endLineNumber: position.lineNumber,
          startColumn: position.column - (colonMatch[2] || '').length,
          endColumn: position.column,
        };
        for (const icon of icons) {
          suggestions.push({
            label: icon,
            kind: monaco.languages.CompletionItemKind.Value,
            detail: `${provider}:${icon}`,
            insertText: icon,
            range: iconRange,
          });
        }
      }

      // Cluster type completions: "cluster:aws:" or "cluster:gcp:"
      const clusterMatch = beforeCursor.match(/^cluster:([a-z][a-z0-9-]*):([a-z0-9-]*)$/i);
      if (clusterMatch) {
        const provider = clusterMatch[1];
        const types = CLUSTER_TYPES[provider] || [];
        const typeRange = {
          startLineNumber: position.lineNumber,
          endLineNumber: position.lineNumber,
          startColumn: position.column - (clusterMatch[2] || '').length,
          endColumn: position.column,
        };
        for (const t of types) {
          suggestions.push({
            label: t,
            kind: monaco.languages.CompletionItemKind.EnumMember,
            detail: `Cluster type for ${provider}`,
            insertText: t,
            range: typeRange,
          });
        }
      }

      return { suggestions };
    },
  });
}

// ─── Diagnostics (uses WASM parse_dsl) ───

let diagnosticsTimer = null;

export function setupDiagnostics(monaco, editor, parseDslFn) {
  const model = editor.getModel();
  if (!model) return;

  const validate = () => {
    const text = model.getValue();
    if (!text.trim()) {
      monaco.editor.setModelMarkers(model, 'archflow', []);
      return;
    }
    try {
      parseDslFn(text);
      monaco.editor.setModelMarkers(model, 'archflow', []);
    } catch (e) {
      const msg = e.message || e.toString();
      // Try to extract line number from error: "line N: message" or "ParseError { line: N, ... }"
      let line = 1;
      let message = msg;
      const lineMatch = msg.match(/line\s*[:=]?\s*(\d+)/i);
      if (lineMatch) {
        line = parseInt(lineMatch[1], 10);
      }
      // Clean up wasm panic prefix
      message = message.replace(/^(panicked at |RuntimeError: unreachable|Error: )*/g, '').trim();

      monaco.editor.setModelMarkers(model, 'archflow', [{
        severity: monaco.MarkerSeverity.Error,
        startLineNumber: line,
        startColumn: 1,
        endLineNumber: line,
        endColumn: model.getLineLength(line) + 1,
        message,
      }]);
    }
  };

  editor.onDidChangeModelContent(() => {
    clearTimeout(diagnosticsTimer);
    diagnosticsTimer = setTimeout(validate, 300);
  });

  // Initial validation
  validate();
}
