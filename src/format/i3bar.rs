use super::data::*;
use std::process::{Command, Stdio};
use std::collections::BTreeMap;
use serde_json::{self, Value, Number};

#[derive(Deserialize)]
struct I3Click {
    instance: String,
    button: u8,
}

pub struct I3BarFormatter {
    handlers: BTreeMap<String, ClickAction>,
}

impl Formatter for I3BarFormatter {
    fn format(&mut self, data: &Format) -> String {
        self.format_all(&[data.clone()])
    }

    fn format_all(&mut self, data: &[Format]) -> String {
        self.handlers.clear();
        let mut line = "[".to_string();
        for f in data {
            let mut map = BTreeMap::new();
            self.build(&mut line, &mut map, f);
        }
        line.pop();
        line.push_str("],");
        line
    }

    fn handle_stdin(&self, line: Option<String>, fns: &mut BTreeMap<String, Box<FnMut()>>) {
        if let Some(s) = line {
            if let Ok(I3Click { instance, button }) = serde_json::from_str(&s.trim_matches(',')) {
                match self.handlers.get(&instance) {
                    Some(&ClickAction::Function(ref mb, ref name)) if mb.to_number() == button => {
                        if let Some(f) = fns.get_mut(name) {
                            f()
                        }
                    },
                    Some(&ClickAction::ShellCommand(ref mb, ref cmd)) if mb.to_number() == button => {
                        let _ = Command::new("sh").arg("-c").arg(cmd).stdout(Stdio::null()).spawn();
                    }
                    _ => (),
                }
            }
        }
    }
}

impl I3BarFormatter {
    pub fn new() -> I3BarFormatter {
        println!("{}", "{\"version\":1,\"click_events\":true}");
        println!("[");
        I3BarFormatter {
            handlers: BTreeMap::new(),
        }
    }

    fn push(&self, line: &mut String, map: &mut BTreeMap<&str, Value>, s: &str) {
        map.insert("full_text", Value::String(s.to_owned()));
        let json = serde_json::to_string(&map).unwrap();
        line.push_str(json.as_ref());
        line.push_str(",");
    }

    fn build(&mut self, line: &mut String, map: &mut BTreeMap<&str, Value>, data: &Format) {
        match *data {
            Format::UnescapedStr(ref s) =>
                self.push(line, map, s),
            Format::Str(ref s) =>
                self.push(line, map, s),
            Format::Concat(ref fs) => {
                for f in fs {
                    let mut map1 = map.clone();
                    self.build(line, &mut map1, f);
                }
            },
            Format::Align(ref a, ref f) => {
                map.insert(
                    "align",
                    Value::String(match *a {
                        Alignment::Left => "left",
                        Alignment::Center => "center",
                        Alignment::Right => "right",
                    }.to_owned()));
                self.build(line, map, f);
            },
            Format::FgColor(ref c, ref f) => {
                map.insert("color", Value::String(c.to_owned()));
                self.build(line, map, f);
            },
            Format::BgColor(ref c, ref f) => {
                map.insert("background", Value::String(c.to_owned()));
                self.build(line, map, f);
            },
            Format::NoSeparator(ref f) => {
                map.insert("separator", Value::Bool(false));
                self.build(line, map, f);
            },
            Format::Padding(n, ref f) => {
                map.insert("separator_block_width", Value::Number(Number::from(n)));
                self.build(line, map, f);
            },
            Format::Clickable(ref act, ref f) => {
                let _ = self.handlers.insert(act.to_string(), act.clone());
                map.insert("instance", Value::String(act.to_string()));
                self.build(line, map, f);
            },
        }
    }
}
