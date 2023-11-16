
use console::Style;
use similar::{ChangeTag, TextDiff};
use std::fmt::{Write as _, self};

struct Line(Option<usize>);

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self.0 {
          None => write!(f, "    "),
          Some(idx) => write!(f, "{:<4}", idx + 1),
      }
  }
}

pub fn diff_text(text1: &str, text2: &str) -> anyhow::Result<(String, String)> {
    if text1 == text2 {
        return Ok((text1.to_string(), text2.to_string()));
    }
    let mut output1 = String::new();
    let mut output2 = String::new();
    let diff = TextDiff::from_lines(text1, text2);
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            writeln!(&mut output1, "{:-^1$}", "-", 80)?;
            writeln!(&mut output2, "{:-^1$}", "-", 80)?;
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (_sign, s) = match change.tag() {
                    ChangeTag::Delete => ("", Style::new().red()),
                    ChangeTag::Insert => ("", Style::new().green()),
                    ChangeTag::Equal => ("", Style::new().dim()),
                };
                for (emphasized, value) in change.iter_strings_lossy() {
                    match change.tag() {
                        ChangeTag::Delete => {
                          if emphasized {
                              write!(&mut output1, "{}", s.apply_to(value.clone()).underlined().on_black())?;
                          } else {
                            write!(&mut output1, "{}", s.apply_to(value.clone()))?;
                          }
                        },
                        ChangeTag::Insert => {
                            if emphasized {
                                write!(&mut output2, "{}", s.apply_to(value.clone()).underlined().on_black())?;
                            } else {
                              write!(&mut output2, "{}", s.apply_to(value.clone()))?;
                            }
                        },
                        ChangeTag::Equal => {
                          // if emphasized {
                          //     write!(&mut output1, "{}", s.apply_to(value.clone()).underlined().on_black())?;
                          //     write!(&mut output2, "{}", s.apply_to(value.clone()).underlined().on_black())?;
                          // } else {
                            write!(&mut output1, "{}", s.apply_to(value.clone()))?;
                            write!(&mut output2, "{}", s.apply_to(value.clone()))?;
                          // }
                        },
                    };
                }
                if change.missing_newline() {
                    if !output1.is_empty() {
                      writeln!(&mut output1)?;
                    }
                    if !output2.is_empty() {
                      writeln!(&mut output2)?;
                    }
                }
            }
        }
    }

    Ok((output1, output2))
}