use std::{
    cell::RefCell,
    fmt::Display,
    io::{BufWriter, Cursor, Write},
    rc::Rc,
};

#[derive(Copy, Clone)]
enum State {
    Neutral,
    OpenedElement,
}

struct Context<W>
where
    W: Write,
{
    writer: W,
    indent: i32,
    state: State,
}

impl<W> Context<W>
where
    W: Write,
{
    pub fn element<S: AsRef<str>>(&mut self, name: S) -> Element<'_, W> {
        let name = name.as_ref();
        self.neutralize_state(true);
        write!(self.writer, "<{}", name).expect("Failed to write");
        self.state = State::OpenedElement;
        Element {
            context: self,
            name: name.to_string(),
        }
    }

    pub fn raw<V: Display>(&mut self, value: V) {
        self.neutralize_state(true);
        write!(self.writer, "{}", value).expect("Failed to write");
    }

    fn neutralize_state(&mut self, close_element: bool) {
        if let State::OpenedElement = self.state {
            if close_element {
                self.write("/>");
            } else {
                self.write(">");
            }
        }
    }

    fn write<S: AsRef<str>>(&mut self, s: S) {
        let s = s.as_ref();
        self.writer
            .write_all(s.as_bytes())
            .expect("Failed to write");
    }
}

struct Element<'a, W>
where
    W: Write,
{
    context: &'a mut Context<W>,
    name: String,
}

impl<'a, W> Element<'a, W>
where
    W: Write,
{
    pub fn set<S: AsRef<str>, V: Display>(self, name: S, value: V) -> Self {
        self
    }

    pub fn children<F>(self, f: F)
    where
        F: FnOnce(&mut Context<W>),
    {
        self.context.neutralize_state(false);
        self.context.indent += 2;
        f(self.context);
        self.context.indent -= 2;
        write!(self.context.writer, "</{}>", self.name).expect("Failed to write");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::BufWriter;
    use std::io::Write;

    pub fn render<F, W>(f: F) -> String
    where
        F: FnOnce(Context<W>),
        W: Write,
    {
        let mut s = String::new();
        let mut writer = BufWriter::new(unsafe { s.as_mut_vec() });
        {
            let mut ctx = Context {
                writer: writer,
                indent: 0,
                state: State::Neutral,
            };
            f(ctx);
        }
        s
    }

    #[test]
    pub fn example() {
        let mut s = String::new();
        //let mut file = Vec::new();
        let mut writer = BufWriter::new(unsafe { s.as_mut_vec() });
        //let mut writer = Vec::new();

        {
            let mut ctx = Context {
                writer: writer,
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
}
