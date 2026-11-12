//! Shared build-time article renderer: Markdown structure plus semantic MathML.
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

type RenderedArticle = (String, Vec<(String, String)>);

pub fn render(body: &str) -> Result<RenderedArticle, String> {
    let mut headings = Vec::new();
    let mut current: Option<(String, String)> = None;
    let mut events = Vec::new();
    for event in Parser::new_ext(body, Options::ENABLE_TABLES | Options::ENABLE_MATH) {
        events.push(match event {
            Event::Start(Tag::Heading {
                level,
                classes,
                attrs,
                ..
            }) => {
                let id = format!("article-section-{}", headings.len() + 1);
                current = Some((id.clone(), String::new()));
                Event::Start(Tag::Heading {
                    level,
                    id: Some(id.into()),
                    classes,
                    attrs,
                })
            }
            Event::Text(ref text) | Event::Code(ref text) => {
                if let Some((_, label)) = &mut current {
                    label.push_str(text);
                }
                event
            }
            Event::InlineMath(ref tex) | Event::DisplayMath(ref tex) => {
                let display = matches!(event, Event::DisplayMath(_));
                if let Some((_, label)) = &mut current {
                    label.push_str(tex);
                }
                Event::Html(math(tex, display)?.into())
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(heading) = current.take() {
                    headings.push(heading);
                }
                event
            }
            _ => event,
        });
    }
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    // Keep a narrow MathML vocabulary; arbitrary source HTML remains sanitized.
    let mut cleaner = ammonia::Builder::default();
    cleaner.add_tags(&[
        "math",
        "mi",
        "mn",
        "mo",
        "mrow",
        "msup",
        "msub",
        "msubsup",
        "mfrac",
        "msqrt",
        "mroot",
        "munder",
        "mover",
        "munderover",
        "mtext",
        "mspace",
        "mtable",
        "mtr",
        "mtd",
        "mstyle",
        "menclose",
        "mpadded",
        "mphantom",
    ]);
    cleaner.add_tag_attributes("math", &["xmlns", "display"]);
    for tag in ["mi", "mn", "mo", "mrow", "mstyle", "mtext"] {
        cleaner.add_tag_attributes(tag, &["mathvariant", "displaystyle", "scriptlevel"]);
    }
    cleaner.add_tag_attributes(
        "mo",
        &["stretchy", "movablelimits", "lspace", "rspace", "form"],
    );
    cleaner.add_tag_attributes("mspace", &["width", "height", "depth"]);
    cleaner.add_tag_attributes("mtable", &["columnalign", "rowspacing", "columnspacing"]);
    cleaner.add_tag_attributes("mtd", &["columnalign", "style"]);
    cleaner.add_tag_attributes("mtable", &["displaystyle", "scriptlevel", "style"]);
    // Preserve only the fixed layout declarations used by aligned equations.
    cleaner.attribute_filter(|_, attribute, value| {
        if attribute != "style" {
            return Some(value.into());
        }
        let safe = value
            .split(';')
            .map(str::trim)
            .filter(|declaration| {
                matches!(
                    *declaration,
                    "text-align: left"
                        | "text-align: right"
                        | "justify-items: start"
                        | "justify-items: end"
                        | "padding-left: 0"
                        | "padding-right: 0"
                        | "padding-top: 0"
                        | "padding-bottom: 0"
                        | "width: 100%"
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        if safe.is_empty() {
            None
        } else {
            Some(safe.into())
        }
    });
    cleaner.add_tag_attributes("span", &["class", "tabindex"]);
    for tag in ["h1", "h2", "h3", "h4", "h5", "h6"] {
        cleaner.add_tag_attributes(tag, &["id"]);
    }
    let mut html = cleaner.clean(&html).to_string();
    for level in 1..=6 {
        html = html.replace(
            &format!("<h{level} id="),
            &format!("<h{level} tabindex=\"-1\" id="),
        );
    }
    Ok((html, headings))
}

fn math(tex: &str, display: bool) -> Result<String, String> {
    let style = if display {
        math_core::MathDisplay::Block
    } else {
        math_core::MathDisplay::Inline
    };
    let converter = math_core::LatexToMathML::new(math_core::MathCoreConfig::default())
        .expect("default math configuration");
    let mathml = converter
        .convert_with_local_state(tex, style)
        .map_err(|error| format!("invalid math expression {tex:?}: {error}"))?
        .mathml;
    Ok(if display {
        format!("<span class=\"math-display\" tabindex=\"0\">{mathml}</span>")
    } else {
        format!("<span class=\"math-inline\">{mathml}</span>")
    })
}
