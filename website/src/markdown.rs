
use std::{collections::HashMap, fmt};

use dioxus::prelude::*;
use pulldown_cmark::{Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Parser, Tag};

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
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
        }
    }
}

/// Pull events from parser until the end of end tag.
fn pull_elements<'a, 'b: 'a>(mut iter: &'a mut Parser<'b>, cx: &'a mut Context<'b>) -> impl Iterator<Item = Element> + use<'a, 'b> {
    std::iter::from_fn(move || {
        Some(match iter.next()? {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    rsx! { p { {pull_elements(iter, cx)} } }
                }
                Tag::Heading { level, id, classes, attrs } => {
                    let id = id.map(|id| escape_html(id));
                    let classes = classes.iter().map(|class| escape_html(class));

                    match level {
                        HeadingLevel::H1 => rsx! { h1 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                        HeadingLevel::H2 => rsx! { h2 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                        HeadingLevel::H3 => rsx! { h3 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                        HeadingLevel::H4 => rsx! { h4 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                        HeadingLevel::H5 => rsx! { h5 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                        HeadingLevel::H6 => rsx! { h6 {
                            id: {id},
                            class: {classes},
                            {pull_elements(iter, cx)}
                        } }
                    }
                }
                // Tag::BlockQuote(None) => {
                //     rsx! { blockquote { {pull_elements(iter, cx)} } }
                // }
                // Tag::BlockQuote(Some(kind)) => {
                //     let class_str = match kind {
                //         BlockQuoteKind::Note => " class=\"markdown-alert-note\"",
                //         BlockQuoteKind::Tip => " class=\"markdown-alert-tip\"",
                //         BlockQuoteKind::Important => " class=\"markdown-alert-important\"",
                //         BlockQuoteKind::Warning => " class=\"markdown-alert-warning\"",
                //         BlockQuoteKind::Caution => " class=\"markdown-alert-caution\"",
                //     };
                //     rsx! { blockquote { class: {class_str}, {pull_elements(iter, cx)} } }
                // }
                // Tag::CodeBlock(kind) => {
                //     match kind {
                //         CodeBlockKind::Indented => {
                //             rsx! { pre { code { {pull_elements(iter, cx)} } } }
                //         }
                //         CodeBlockKind::Fenced(info) => {
                //             let lang = info.split(' ').next().unwrap();
                //             if lang.is_empty() {
                //                 rsx! { pre { code { {pull_elements(iter, cx)} } } }
                //             } else {
                //                 let class_str = format!(" class=\"language-{}\"", escape_html(lang));
                //             }
                //         }
                //     }
                // }
                // Tag::HtmlBlock => {
                //     rsx! { {pull_elements(iter, cx)} }
                // }
                // Tag::List(None) => {
                //     rsx! { ul { {pull_elements(iter, cx)} } }
                // }
                // Tag::List(Some(1)) => {
                //     rsx! { ol { {pull_elements(iter, cx)} } }
                // }
                // Tag::List(Some(start)) => {
                //     rsx! { ol { start: {start}, {pull_elements(iter, cx)} } }
                // }
                // Tag::Item => {
                //     rsx! { li { {pull_elements(iter, cx)} } }
                // }
                // Tag::FootnoteDefinition(name) => {
                //     let len = cx.numbers.len() + 1;
                //     let number = cx.numbers.entry(name).or_insert(len);
                //     rsx! {
                //         div {
                //             id: {escape_html(name)},
                //             class: "footnote-definition",
                //             sup {
                //                 class: "footnote-definition-label",
                //                 {number}
                //             }
                //             {pull_elements(iter, cx)}
                //         }
                //     }
                // }
                // Tag::DefinitionList => {
                //     rsx! { dl { {pull_elements(iter, cx)} } }
                // }
                // Tag::DefinitionListTitle => {
                //     rsx! { dt { {pull_elements(iter, cx)} } }
                // }
                // Tag::DefinitionListDefinition => {
                //     rsx! { dd { {pull_elements(iter, cx)} } }
                // }
                // Tag::Table(alignments) => {
                //     cx.table_alignments = alignments;
                //     rsx! { table { {pull_elements(iter, cx)} } }
                // }
                // Tag::TableHead => {
                //     cx.table_state = TableState::Head;
                //     cx.table_cell_index = 0;
                //     rsx! { thead { tr { {pull_elements(iter, cx)} } } }
                // }
                // Tag::TableRow => {
                //     self.table_cell_index = 0;
                //     rsx! { tr { {pull_elements(iter, cx)} } }
                // }
                // Tag::TableCell => {
                //     let style_str = match cx.table_alignments.get(cx.table_cell_index) {
                //         Some(Alignment::Left) => Some("text-align: left"),
                //         Some(Alignment::Center) => Some("text-align: center"),
                //         Some(Alignment::Right) => Some("text-align: right"),
                //         _ => None,
                //     };

                //     match cx.table_state {
                //         TableState::Head => {
                //             rsx!{ th { style: {style_str}, {pull_elements(iter, cx)} } }
                //         }
                //         TableState::Body => {
                //             rsx!{ td { style: {style_str}, {pull_elements(iter, cx)} } }
                //         }
                //     }
                // }
                // Tag::Emphasis => rsx! { em { {pull_elements(iter, cx)} } },
                // Tag::Strong => rsx! { strong { {pull_elements(iter, cx)} } },
                // Tag::Strikethrough => rsx! { del { {pull_elements(iter, cx)} } },

                // Tag::Link { link_type: LinkType::Email, dest_url, title, id: _ } => {
                //     let link = format!("mailto:{}", escape_href(dest_url));

                //     let title_str = match title.is_empty() {
                //         true => None,
                //         false => Some(&*title)
                //     };

                //     rsx! { Link { to: link, title: {title_str} } }
                // }
                _ => return None,
            }
            Event::End(_) => return None,
            _ => return None,
        })
    })
}

/// Render some text as markdown.
#[component]
pub fn Markdown(content: String) -> Element {
    let content = use_resource(move || {
        let url = url.clone();
        async move { reqwest::get(url).await.unwrap().text().await.unwrap() }
    });

    let content = content.cloned().unwrap_or_default();

    let parser = Parser::new(&content);

    let mut cx = Context::new();
    let elements = pull_elements(&mut parser, &mut cx);

    rsx! {
        div { class: "md-content", {elements} }
    }
}
