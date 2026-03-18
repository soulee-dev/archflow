/**
 * CodeMirror 5 mode for Archflow DSL syntax highlighting.
 */
(function (CodeMirror) {
  "use strict";

  CodeMirror.defineMode("archflow", function () {
    return {
      startState: function () {
        return { inClusterHeader: false };
      },
      token: function (stream, state) {
        // Whitespace
        if (stream.eatSpace()) return null;

        // Comments: # or //
        if (stream.match("//") || stream.match("#")) {
          stream.skipToEnd();
          return "comment";
        }

        // Metadata keywords at start of line
        if (stream.sol()) {
          if (stream.match(/^(title|direction|theme)\s*:/i)) {
            return "keyword";
          }
          if (stream.match(/^cluster\b/i)) {
            state.inClusterHeader = true;
            return "keyword";
          }
        }

        // Cluster header: name before {
        if (state.inClusterHeader) {
          if (stream.match("{")) {
            state.inClusterHeader = false;
            return "bracket";
          }
          stream.next();
          return "def";
        }

        // Closing brace
        if (stream.match("}")) {
          return "bracket";
        }

        // Opening brace (in case it appears mid-line)
        if (stream.match("{")) {
          return "bracket";
        }

        // Edge operators
        if (stream.match(">>")) {
          return "operator";
        }
        if (stream.match("->")) {
          return "operator";
        }

        // Edge label after :
        if (stream.match(":")) {
          stream.skipToEnd();
          return "string";
        }

        // Metadata value (rest of line after keyword was consumed)
        // Node names - just consume characters
        stream.next();
        return "variable-2";
      },
    };
  });

  CodeMirror.defineMIME("text/x-archflow", "archflow");
})(CodeMirror);
