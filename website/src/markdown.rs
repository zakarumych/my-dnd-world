use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use dioxus::prelude::*;
use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Parser, Tag,
    TagEnd,
};

use crate::{join_strings, props::ArmorCategory};

struct HtmlEscaped<S>(S);

impl<S> fmt::Display for HtmlEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pulldown_cmark_escape::escape_html(pulldown_cmark_escape::FmtWriter(f), self.0.as_ref())
    }
}

impl<S> fmt::Debug for HtmlEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

fn escape_html<S>(s: S) -> HtmlEscaped<S> {
    HtmlEscaped(s)
}

struct HtmlBodyEscaped<S>(S);

impl<S> fmt::Display for HtmlBodyEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pulldown_cmark_escape::escape_html_body_text(
            pulldown_cmark_escape::FmtWriter(f),
            self.0.as_ref(),
        )
    }
}

impl<S> fmt::Debug for HtmlBodyEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

fn escape_html_body<S>(s: S) -> HtmlBodyEscaped<S> {
    HtmlBodyEscaped(s)
}

struct HrefEscaped<S>(S);

impl<S> fmt::Display for HrefEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pulldown_cmark_escape::escape_href(pulldown_cmark_escape::FmtWriter(f), self.0.as_ref())
    }
}

impl<S> fmt::Debug for HrefEscaped<S>
where
    S: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

fn escape_href<S>(s: S) -> HrefEscaped<S> {
    HrefEscaped(s)
}

enum TableState {
    Head,
    Body,
}

struct Context<'a> {
    numbers: HashMap<CowStr<'a>, usize>,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    in_non_writing_block: bool,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
            in_non_writing_block: false,
        }
    }
}

/// Pull events from parser until the end of end tag.
fn pull_elements<'a, 'b: 'a>(
    mut iter: &'a mut Parser<'b>,
    cx: &'a mut Context<'b>,
) -> impl Iterator<Item = Element> + use<'a, 'b> {
    std::iter::from_fn(move || {
        Some(match iter.next()? {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    rsx! { p { {pull_elements(iter, cx)} } }
                }
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                } => {
                    let id = id.map(|id| escape_html(id).to_string());
                    let classes = join_strings(classes.iter().map(|class| escape_html(class)), ' ');

                    match level {
                        HeadingLevel::H1 => rsx! { h1 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                        HeadingLevel::H2 => rsx! { h2 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                        HeadingLevel::H3 => rsx! { h3 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                        HeadingLevel::H4 => rsx! { h4 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                        HeadingLevel::H5 => rsx! { h5 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                        HeadingLevel::H6 => rsx! { h6 {
                            id: id,
                            class: classes,
                            {pull_elements(iter, cx)}
                        } },
                    }
                }
                Tag::BlockQuote(None) => {
                    rsx! { blockquote { {pull_elements(iter, cx)} } }
                }
                Tag::BlockQuote(Some(kind)) => {
                    let class_str = match kind {
                        BlockQuoteKind::Note => " class=\"markdown-alert-note\"",
                        BlockQuoteKind::Tip => " class=\"markdown-alert-tip\"",
                        BlockQuoteKind::Important => " class=\"markdown-alert-important\"",
                        BlockQuoteKind::Warning => " class=\"markdown-alert-warning\"",
                        BlockQuoteKind::Caution => " class=\"markdown-alert-caution\"",
                    };
                    rsx! { blockquote { class: class_str, {pull_elements(iter, cx)} } }
                }
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Indented => {
                        rsx! { pre { code { {pull_elements(iter, cx)} } } }
                    }
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang.is_empty() {
                            rsx! { pre { code { {pull_elements(iter, cx)} } } }
                        } else {
                            let class_str = format!(" class=\"language-{}\"", escape_html(lang));
                            rsx! { pre { code { class: class_str, {pull_elements(iter, cx)} } } }
                        }
                    }
                },
                Tag::HtmlBlock => {
                    rsx! { {pull_elements(iter, cx)} }
                }
                Tag::List(None) => {
                    rsx! { ul { {pull_elements(iter, cx)} } }
                }
                Tag::List(Some(1)) => {
                    rsx! { ol { {pull_elements(iter, cx)} } }
                }
                Tag::List(Some(start)) => {
                    rsx! { ol { start: start, {pull_elements(iter, cx)} } }
                }
                Tag::Item => {
                    rsx! { li { {pull_elements(iter, cx)} } }
                }
                Tag::FootnoteDefinition(name) => {
                    let id = escape_html(&name).to_string();
                    let len = cx.numbers.len() + 1;
                    let number = *cx.numbers.entry(name).or_insert(len);
                    rsx! {
                        div {
                            id: id,
                            class: "footnote-definition",
                            sup {
                                class: "footnote-definition-label",
                                {number.to_string()}
                            }
                            {pull_elements(iter, cx)}
                        }
                    }
                }
                Tag::DefinitionList => {
                    rsx! { dl { {pull_elements(iter, cx)} } }
                }
                Tag::DefinitionListTitle => {
                    rsx! { dt { {pull_elements(iter, cx)} } }
                }
                Tag::DefinitionListDefinition => {
                    rsx! { dd { {pull_elements(iter, cx)} } }
                }
                Tag::Table(alignments) => {
                    cx.table_alignments = alignments;
                    rsx! { table { {pull_elements(iter, cx)} } }
                }
                Tag::TableHead => {
                    cx.table_state = TableState::Head;
                    cx.table_cell_index = 0;
                    rsx! { thead { tr { {pull_elements(iter, cx)} } } }
                }
                Tag::TableRow => {
                    cx.table_cell_index = 0;
                    rsx! { tr { {pull_elements(iter, cx)} } }
                }
                Tag::TableCell => {
                    let style_str = match cx.table_alignments.get(cx.table_cell_index) {
                        Some(Alignment::Left) => Some("text-align: left"),
                        Some(Alignment::Center) => Some("text-align: center"),
                        Some(Alignment::Right) => Some("text-align: right"),
                        _ => None,
                    };

                    match cx.table_state {
                        TableState::Head => {
                            rsx! { th { style: style_str, {pull_elements(iter, cx)} } }
                        }
                        TableState::Body => {
                            rsx! { td { style: style_str, {pull_elements(iter, cx)} } }
                        }
                    }
                }
                Tag::Emphasis => rsx! { em { {pull_elements(iter, cx)} } },
                Tag::Strong => rsx! { strong { {pull_elements(iter, cx)} } },
                Tag::Strikethrough => rsx! { del { {pull_elements(iter, cx)} } },

                Tag::Link {
                    link_type: LinkType::Email,
                    dest_url,
                    title,
                    id: _,
                } => {
                    let link = format!("mailto:{}", escape_href(dest_url));

                    rsx! {
                        Link {
                            to: link,
                            title: if title.is_empty() { &*title },
                            {pull_elements(iter, cx)}
                        }
                    }
                }

                Tag::Link {
                    link_type: _,
                    dest_url,
                    title,
                    id: _,
                } => {
                    let link = escape_href(dest_url).to_string();

                    rsx! {
                        Link {
                            to: link,
                            title: if title.is_empty() { &*title },
                            {pull_elements(iter, cx)}
                        }
                    }
                }

                Tag::Subscript => rsx! { sub { {pull_elements(iter, cx)} } },
                Tag::Superscript => rsx! { sup { {pull_elements(iter, cx)} } },
                Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                } => {
                    rsx! {
                        img {
                            src: escape_href(dest_url).to_string(),
                            title: if !title.is_empty() { &*title },
                            {pull_elements(iter, cx)}
                        }
                    }
                }
                Tag::MetadataBlock(_) => {
                    cx.in_non_writing_block = true;
                    rsx! { {pull_elements(iter, cx)} }
                }
            },
            Event::End(tag) => {
                match tag {
                    TagEnd::TableHead => {
                        cx.table_state = TableState::Body;
                    }
                    TagEnd::TableCell => {
                        cx.table_cell_index += 1;
                    }
                    TagEnd::MetadataBlock(_) => {
                        cx.in_non_writing_block = false;
                    }
                    _ => {}
                }
                return None;
            }
            Event::Text(text) => {
                if !cx.in_non_writing_block {
                    rsx! { {escape_html_body(text).to_string()} }
                } else {
                    rsx!()
                }
            }
            Event::Code(code) => {
                rsx! { code { {escape_html_body(code).to_string()} } }
            }
            Event::InlineMath(math) => {
                rsx! { span { class: "math inline-math", {escape_html(math).to_string()} } }
            }
            Event::DisplayMath(math) => {
                rsx! { div { class: "math display-math", {escape_html(math).to_string()} } }
            }
            Event::Html(html) => {
                rsx! { {html} }
            }
            Event::InlineHtml(html) => {
                rsx! { {html} }
            }
            Event::FootnoteReference(name) => {
                let len = cx.numbers.len() + 1;
                rsx! {
                    sup {
                        class: "footnote-reference",
                        Link {
                            to: format!("#{}", escape_href(name)),
                            {len.to_string()}
                        }
                    }
                }
            }
            Event::SoftBreak => {
                rsx! { "\n" }
            }
            Event::HardBreak => {
                rsx! { br {} }
            }
            Event::Rule => {
                rsx! { hr {} }
            }
            Event::TaskListMarker(true) => {
                rsx! { input { r#type: "checkbox", checked: true, disabled: "" } }
            }
            Event::TaskListMarker(false) => {
                rsx! { input { r#type: "checkbox", disabled: "" } }
            }
        })
    })
}

/// Render some text as markdown.
#[component]
pub fn Markdown(url: String) -> Element {
    let url2 = url.clone();
    let content = use_resource(move || {
        let url = url.clone();
        async move { reqwest::get(url).await.unwrap().text().await.unwrap() }
    });

    let content = content.cloned().unwrap_or_default();

    // tracing::debug!("Markdown ({url2}) content: {content}");

    let mut parser = Parser::new(&content);

    let mut cx = Context::new();
    let elements = pull_elements(&mut parser, &mut cx);

    rsx! {
        div { class: "md-content", {elements} }
    }
}
