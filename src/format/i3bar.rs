use super::data::*;
use std::collections::BTreeMap;
use serde_json;

pub struct I3BarFormatter;

impl Formatter for I3BarFormatter {
    fn format(&self, data: &Format) -> String {
        self.format_all(&[data.clone()])
    }

    fn format_all(&self, data: &[Format]) -> String {
        let mut line = "[".to_string();
        for f in data {
            let mut map = BTreeMap::new();
            self.build(&mut line, &mut map, f);
        }
        line.pop();
        line.push_str("],");
        line
    }
}

impl I3BarFormatter {
    pub fn new() -> I3BarFormatter {
        println!("{}", "{\"version\":1}");
        println!("[");
        I3BarFormatter
    }

    fn push<'a>(&self, line: &mut String, map: &mut BTreeMap<&str, &'a str>, s: &'a str) {
        map.insert("full_text", s);
        let json = serde_json::to_string(&map).unwrap();
        line.push_str(json.as_ref());
        line.push_str(",");
    }

    fn build<'a>(&self, line: &mut String, map: &mut BTreeMap<&str, &'a str>, data: &'a Format) {
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
                    match *a {
                        Alignment::Left => "left",
                        Alignment::Center => "center",
                        Alignment::Right => "right",
                    });
                self.build(line, map, f);
            },
            Format::FgColor(ref c, ref f) => {
                map.insert("color", c);
                self.build(line, map, f);
            },
            Format::BgColor(ref c, ref f) => {
                map.insert("background", c);
                self.build(line, map, f);
            },
            Format::Clickable(ref mb, ref a, ref f) => {
                // TODO
                self.build(line, map, f);
            },
        }
    }
}
