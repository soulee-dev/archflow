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
            shadow,
        } => {
            let filter = if *shadow {
                " filter=\"url(#shadow)\""
            } else {
                ""
            };
            let stroke_attr = if stroke == "none" {
                String::new()
            } else {
                format!(" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"")
            };
            buf.push_str(&format!(
                "{pad}<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" \
                 rx=\"{rx}\" fill=\"{fill}\"{stroke_attr}{filter}/>\n"
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
            font_weight,
        } => {
            buf.push_str(&format!(
                "{pad}<text x=\"{x}\" y=\"{y}\" font-size=\"{font_size}\" \
                 font-family=\"{font_family}\" fill=\"{fill}\" font-weight=\"{font_weight}\" \
                 text-anchor=\"{anchor}\" dominant-baseline=\"middle\">{}</text>\n",
                escape_xml(content)
            ));
        }
        SceneElement::Path {
            d,
            stroke,
            stroke_width,
            stroke_dasharray,
            marker_end,
        } => {
            let dash = stroke_dasharray
                .as_ref()
                .map(|da| format!(" stroke-dasharray=\"{da}\""))
                .unwrap_or_default();
            let marker = if *marker_end {
                " marker-end=\"url(#arrowhead)\""
            } else {
                ""
            };
            buf.push_str(&format!(
                "{pad}<path d=\"{d}\" fill=\"none\" \
                 stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" \
                 stroke-linecap=\"round\"{dash}{marker}/>\n"
            ));
        }
        SceneElement::RawSvg {
            x,
            y,
            width,
            height,
            content,
        } => {
            buf.push_str(&format!(
                "{pad}<svg x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\">\n"
            ));
            buf.push_str(content);
            buf.push('\n');
            buf.push_str(&format!("{pad}</svg>\n"));
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
    let mut buf = String::with_capacity(8192);

    buf.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"##,
        scene.width, scene.height, scene.width, scene.height
    ));
    buf.push('\n');

    // Defs
    buf.push_str(&format!(
        r##"  <defs>
    <filter id="shadow" x="-4%" y="-4%" width="108%" height="116%">
      <feDropShadow dx="0" dy="2" stdDeviation="4" flood-color="#000000" flood-opacity="0.08"/>
    </filter>
    <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="7" refY="3" orient="auto" markerUnits="strokeWidth">
      <path d="M0,0 L8,3 L0,6 L2,3 Z" fill="{}"/>
    </marker>
  </defs>
"##,
        scene.edge_color
    ));

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
