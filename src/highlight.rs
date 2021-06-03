use termion::color;
use termion::color::Fg;
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter as TSHighlighter;
extern "C" {
    fn tree_sitter_rust() -> Language;
}

struct HighlightColor {
    name: String,
    color: String,
}

impl HighlightColor {
    pub fn new<T: color::Color>(name: &str, color: T) -> HighlightColor {
        HighlightColor {
            name: String::from(name),
            color: Fg(color).to_string(),
        }
    }
}

fn get_highlight_config() -> Vec<(usize, HighlightColor)> {
    vec![
        (0, HighlightColor::new("attribute", color::Red)),
        (1, HighlightColor::new("constant", color::Red)),
        (2, HighlightColor::new("function.builtin", color::Red)),
        (3, HighlightColor::new("keyword", color::Blue)),
        (4, HighlightColor::new("function", color::Red)),
        (5, HighlightColor::new("operator", color::Blue)),
        (6, HighlightColor::new("property", color::Magenta)),
        (7, HighlightColor::new("punctuation", color::LightGreen)),
        (
            8,
            HighlightColor::new("punctuation.bracket", color::LightCyan),
        ),
        (
            9,
            HighlightColor::new("punctuation.delimiter", color::Yellow),
        ),
        (10, HighlightColor::new("string", color::Green)),
        (11, HighlightColor::new("string.special", color::LightGreen)),
        (12, HighlightColor::new("tag", color::Red)),
        (13, HighlightColor::new("type", color::Red)),
        (14, HighlightColor::new("type.builtin", color::Blue)),
        (15, HighlightColor::new("variable", color::Red)),
        (
            16,
            HighlightColor::new("variable.builtin", color::LightYellow),
        ),
        (
            17,
            HighlightColor::new("variable.parameter", color::LightMagenta),
        ),
        (18, HighlightColor::new("comment", color::LightBlack)),
        (19, HighlightColor::new("function.method", color::Yellow)),
        (20, HighlightColor::new("function.special", color::Red)),
    ]
}

fn get_highlight_names() -> Vec<String> {
    get_highlight_config()
        .iter()
        .map(|name| -> String { name.1.name.clone() })
        .collect()
}

fn get_highlight_color(color: usize) -> String {
    for name in get_highlight_config().iter() {
        if name.0 == color {
            return name.1.color.clone();
        }
    }
    return Fg(color::White).to_string();
}

#[derive(Copy, Clone)]
struct Highlight {
    start: usize,
    end: usize,
    color: usize,
}

pub struct Highlighter {
    highlighter: TSHighlighter,
    rust_config: HighlightConfiguration,
}

impl Highlighter {
    pub fn new() -> Highlighter {
        let rust_language = unsafe { tree_sitter_rust() };
        let mut rust_config =
            HighlightConfiguration::new(rust_language, tree_sitter_rust::HIGHLIGHT_QUERY, "", "")
                .unwrap();
        rust_config.configure(get_highlight_names().as_slice());
        Highlighter {
            highlighter: TSHighlighter::new(),
            rust_config: rust_config,
        }
    }

    fn get_highlights(&mut self, text: &String) -> Vec<Highlight> {
        let highlights = self
            .highlighter
            .highlight(&self.rust_config, text.as_bytes(), None, |_| None)
            .unwrap();

        let mut hls = vec![];
        let mut hl = Highlight {
            start: 0,
            end: 0,
            color: 999,
        };

        for event in highlights {
            match event.unwrap() {
                HighlightEvent::Source { start, end } => {
                    if hl.color == 999 {
                        hls.push(Highlight {
                            start: start,
                            end: end,
                            color: 999,
                        });
                    } else {
                        hl.start = start;
                        hl.end = end;
                    }
                }
                HighlightEvent::HighlightStart(s) => {
                    hl.color = s.0;
                }
                HighlightEvent::HighlightEnd => {
                    hls.push(hl);
                    hl = Highlight {
                        start: 0,
                        end: 0,
                        color: 999,
                    };
                }
            }
        }
        hls
    }

    pub fn highlight(&mut self, text: &String) -> String {
        self.get_highlights(text)
            .iter()
            .map(|hl| -> String {
                format!(
                    "{}{}",
                    get_highlight_color(hl.color).as_str(),
                    text.get(hl.start..hl.end).unwrap()
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
