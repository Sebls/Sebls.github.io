#[path = "../build_support/markdown.rs"]
mod markdown;

#[test]
fn math_is_semantic_and_code_stays_literal() {
    let (html, headings) = markdown::render("## A $p_i$ note\n\nInline $p^0=1$.\n\n$$\\frac{ab}{\\operatorname{gcd}(a,b)}$$\n\n`$literal$`\n\n```text\n$$literal$$\n```\n").unwrap();
    assert!(html.contains("<msub>"));
    assert!(html.contains("<msup>"));
    assert!(html.contains("<mfrac>"));
    assert!(html.contains("display=\"block\""));
    assert!(html.contains("class=\"math-display\" tabindex=\"0\""));
    assert!(html.contains("<code>$literal$</code>"));
    assert!(html.contains("$$literal$$"));
    assert_eq!(
        headings,
        vec![("article-section-1".into(), "A p_i note".into())]
    );
}

#[test]
fn unsafe_html_and_urls_are_removed() {
    let (html, _) = markdown::render("<script>alert(1)</script>\n\n<img src=x onerror=alert(1)>\n\n[bad](javascript:alert)\n\n$x < y$\n").unwrap();
    assert!(!html.contains("<script"));
    assert!(!html.contains("onerror"));
    assert!(!html.contains("javascript:"));
    assert!(html.contains("<math"));
    assert!(html.contains("&lt;"));
}

#[test]
fn malformed_math_fails_instead_of_publishing_broken_notation() {
    assert!(markdown::render("$\\thiscommanddoesnotexist{x}$").is_err());
}

#[test]
fn published_articles_keep_structure_and_render_all_math() {
    let (html, headings) =
        markdown::render(include_str!("../content/posts/gcd-lcm-prime-exponents.md")).unwrap();
    assert_eq!(headings.len(), 5);
    assert_eq!(html.matches("class=\"math-display\"").count(), 11);
    assert!(html.contains("<munderover>"));
    assert!(
        html.contains("<msubsup>"),
        "prime base needs sibling subscript and superscript"
    );
    assert!(html.contains("<mtable"));
    assert!(html.contains("text-align: right"));
    assert!(html.contains("text-align: left"));
    assert!(html.contains("<mfrac>"));
    assert!(!html.contains("<blockquote>"));
    assert!(!html.contains("$$"));
    let (other, _) = markdown::render(include_str!(
        "../content/posts/autoencoders-image-similarity.md"
    ))
    .unwrap();
    assert_eq!(other.matches("<img ").count(), 13);
}
