use std::fmt::Display;
use std::io::{BufWriter, Write};

#[derive(Copy, Clone)]
enum State {
    Neutral,
    OpenAttributes,
}

struct Context<'a> {
    writer: Box<dyn Write + 'a>,
    indent: i32,
    state: State,
}

impl<'a> Context<'a> {
    ///
    /// Adds an element.
    ///
    pub fn element<S: AsRef<str>>(&mut self, name: S) -> Element<'_, 'a> {
        let name = name.as_ref();
        self.neutralize_state(true);
        self.write_indent();
        write!(self.writer, "<{}", name).expect("Failed to write");
        self.state = State::OpenAttributes;
        Element {
            ctx: self,
            element_name: name.to_string(),
        }
    }

    ///
    /// Adds raw content.
    ///
    pub fn raw<V: Display>(&mut self, value: V) {
        self.neutralize_state(true);
        self.write_indent();
        write!(self.writer, "{}\n", value).expect("Failed to write");
    }

    ///
    /// Rarely needed to call explicitly, only when working with the top-level context manually.
    ///
    pub fn finalize(&mut self) {
        self.neutralize_state(true);
    }

    fn neutralize_state(&mut self, close_element: bool) {
        if let State::OpenAttributes = self.state {
            if close_element {
                self.write("/>\n");
            } else {
                self.write(">\n");
            }
            self.state = State::Neutral;
        }
    }

    fn write<S: AsRef<str>>(&mut self, s: S) {
        let s = s.as_ref();
        self.writer
            .write_all(s.as_bytes())
            .expect("Failed to write");
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            write!(self.writer, " ").expect("Failed to write");
        }
    }
}

struct Element<'a, 'b> {
    ctx: &'a mut Context<'b>,
    element_name: String,
}

impl<'a, 'b> Element<'a, 'b> {
    ///
    /// Sets an attribute on the element.
    ///
    /// Consumes and returns Self to be used in builder-style.
    ///
    pub fn set<S: AsRef<str>, V: Display>(self, attr: S, value: V) -> Self {
        write!(self.ctx.writer, " {}=\"{}\"", attr.as_ref(), value).expect("Failed to write");
        self
    }

    ///
    /// Adds children.
    ///
    /// Consumes the element to avoid further usage (internally terminates the element).
    ///
    pub fn children<F>(self, f: F)
    where
        F: FnOnce(&mut Context),
    {
        self.ctx.neutralize_state(false);
        self.ctx.indent += 2;
        f(self.ctx);
        self.ctx.neutralize_state(true);
        self.ctx.indent -= 2;
        self.ctx.write_indent();
        write!(self.ctx.writer, "</{}>\n", self.element_name).expect("Failed to write");
    }
}

fn render_svg<F>(f: F) -> String
where
    F: FnOnce(&mut Context),
{
    let mut s = String::new();
    let writer = BufWriter::new(unsafe { s.as_mut_vec() });
    {
        let mut ctx = Context {
            writer: Box::new(writer),
            indent: 0,
            state: State::Neutral,
        };
        f(&mut ctx);
        ctx.finalize();
    }
    s
}

#[allow(dead_code)]
pub fn example() {
    let mut s = String::new();
    //let mut file = Vec::new();
    let writer = BufWriter::new(unsafe { s.as_mut_vec() });
    //let mut writer = Vec::new();

    {
        let mut ctx = Context {
            writer: Box::new(writer),
            indent: 0,
            state: State::Neutral,
        };

        ctx.element("svg")
            .set("width", 100)
            .set("height", 100)
            .set("viewbox", "0 0 100 100")
            .children(|ctx| {
                ctx.element("defs").children(|ctx| {
                    ctx.element("filter")
                        .set("id", "shadow")
                        .set("color-interpolation-filters", "sRGB")
                        .children(|ctx| {
                            ctx.element("feGaussianBlur");
                            ctx.element("feOffset");
                            ctx.element("feFlood");
                        });
                    ctx.element("filter");
                });
                ctx.element("rect").set("width", 100).set("height", 100);
                ctx.element("circle");
                ctx.element("text").children(|ctx| ctx.raw("Hello World"));
            });
    }

    // let output = std::str::from_utf8(writer.buffer()).unwrap();
    let output = &s;
    println!("{}", output);
    assert_eq!(output, "");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_empty() {
        let output = render_svg(|ctx| {
            ctx.element("svg");
        });
        let expected = indoc! {"
          <svg/>
        "};
        assert_eq!(output, expected);

        let output = render_svg(|ctx| {
            ctx.element("svg").children(|_| {});
        });
        let expected = indoc! {"
          <svg>
          </svg>
        "};
        assert_eq!(output, expected);
    }

    #[test]
    pub fn test_basic() {
        let output = render_svg(|ctx| {
            ctx.element("svg").children(|ctx| {
                ctx.element("rect")
                    .set("x", 100)
                    .set("width", "100%")
                    .set("with-hyphen", "some text");
            });
        });
        let expected = indoc! {r#"
          <svg>
            <rect x="100" width="100%" with-hyphen="some text"/>
          </svg>
        "#};
        assert_eq!(output, expected);
    }

    #[test]
    pub fn test_multiple_elements() {
        let output = render_svg(|ctx| {
            ctx.element("svg").children(|ctx| {
                ctx.element("rect");
                ctx.element("circle");
            });
        });
        let expected = indoc! {"
          <svg>
            <rect/>
            <circle/>
          </svg>
        "};
        assert_eq!(output, expected);
    }

    #[test]
    pub fn test_nesting() {
        let output = render_svg(|ctx| {
            ctx.element("svg").children(|ctx| {
                ctx.element("g").children(|ctx| {
                    ctx.element("rect");
                    ctx.element("circle");
                });
                ctx.element("g").children(|ctx| {
                    ctx.element("rect");
                    ctx.element("circle");
                });
            });
        });
        let expected = indoc! {"
          <svg>
            <g>
              <rect/>
              <circle/>
            </g>
            <g>
              <rect/>
              <circle/>
            </g>
          </svg>
        "};
        assert_eq!(output, expected);
    }

    #[test]
    pub fn test_raw_text_node() {
        let output = render_svg(|ctx| {
            ctx.element("svg").children(|ctx| {
                ctx.element("text").children(|ctx| ctx.raw("raw text 1"));
                ctx.element("g").children(|ctx| {
                    ctx.element("text").children(|ctx| ctx.raw("raw text 2"));
                });
            });
        });
        let expected = indoc! {"
          <svg>
            <text>
              raw text 1
            </text>
            <g>
              <text>
                raw text 2
              </text>
            </g>
          </svg>
        "};
        assert_eq!(output, expected);
    }

    #[test]
    pub fn test_complex_example() {
        let output = render_svg(|ctx| {
            ctx.element("svg")
                .set("width", 100)
                .set("height", 100)
                .set("viewbox", "0 0 100 100")
                .children(|ctx| {
                    ctx.element("defs").children(|ctx| {
                        ctx.element("filter")
                            .set("id", "shadow")
                            .set("color-interpolation-filters", "sRGB")
                            .children(|ctx| {
                                ctx.element("feGaussianBlur");
                                ctx.element("feOffset");
                                ctx.element("feFlood");
                            });
                        ctx.element("filter");
                    });
                    ctx.element("rect").set("width", 100).set("height", 100);
                    ctx.element("circle");
                    ctx.element("text").children(|ctx| ctx.raw("Hello World"));
                });
        });
        let expected = indoc! {r#"
          <svg width="100" height="100" viewbox="0 0 100 100">
            <defs>
              <filter id="shadow" color-interpolation-filters="sRGB">
                <feGaussianBlur/>
                <feOffset/>
                <feFlood/>
              </filter>
              <filter/>
            </defs>
            <rect width="100" height="100"/>
            <circle/>
            <text>
              Hello World
            </text>
          </svg>
        "#};
        assert_eq!(output, expected);
    }
}
