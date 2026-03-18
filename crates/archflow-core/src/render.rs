use crate::scene::{SceneElement, SceneGraph};

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn render_element(buf: &mut String, element: &SceneElement, indent: usize) {
    let pad = "  ".repeat(indent);
    match element {
        SceneElement::Rect {
            x,
            y,
            width,
            height,
            rx,
            fill,
            stroke,
            stroke_width,
        } => {
            buf.push_str(&format!(
                "{pad}<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" \
                 rx=\"{rx}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>\n"
            ));
        }
        SceneElement::Text {
            x,
            y,
            content,
            font_size,
            font_family,
            fill,
            anchor,
        } => {
            buf.push_str(&format!(
                "{pad}<text x=\"{x}\" y=\"{y}\" font-size=\"{font_size}\" \
                 font-family=\"{font_family}\" fill=\"{fill}\" \
                 text-anchor=\"{anchor}\" dominant-baseline=\"middle\">{}</text>\n",
                escape_xml(content)
            ));
        }
        SceneElement::Line {
            points,
            stroke,
            stroke_width,
            stroke_dasharray,
            marker_end,
        } => {
            let points_str: String = points
                .iter()
                .map(|(x, y)| format!("{x},{y}"))
                .collect::<Vec<_>>()
                .join(" ");
            let dash = stroke_dasharray
                .as_ref()
                .map(|d| format!(" stroke-dasharray=\"{d}\""))
                .unwrap_or_default();
            let marker = if *marker_end {
                " marker-end=\"url(#arrowhead)\""
            } else {
                ""
            };
            buf.push_str(&format!(
                "{pad}<polyline points=\"{points_str}\" fill=\"none\" \
                 stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"{dash}{marker}/>\n"
            ));
        }
        SceneElement::Group { id, children } => {
            buf.push_str(&format!("{pad}<g id=\"{id}\">\n"));
            for child in children {
                render_element(buf, child, indent + 1);
            }
            buf.push_str(&format!("{pad}</g>\n"));
        }
    }
}

pub fn render_svg(scene: &SceneGraph) -> String {
    let mut buf = String::with_capacity(4096);

    buf.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"##,
        scene.width, scene.height, scene.width, scene.height
    ));
    buf.push('\n');

    // Defs: arrowhead marker
    buf.push_str(
        r##"  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#667085"/>
    </marker>
  </defs>
"##,
    );

    // Background
    buf.push_str(&format!(
        "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n",
        scene.background
    ));

    // Elements
    for element in &scene.elements {
        render_element(&mut buf, element, 1);
    }

    buf.push_str("</svg>\n");
    buf
}
