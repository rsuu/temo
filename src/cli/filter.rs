use crate::data::task;

pub struct Filter {}

impl Filter {
    pub fn filter(text: &str) -> Option<Vec<usize>> {
        let chars: Vec<char> = text.chars().collect();
        let mut pos = 0;
        let mut res: Vec<usize> = Vec::new();

        while pos < chars.len() {
            match chars[pos] {
                '0'..='9' => {
                    let n = chars[pos].to_digit(10).expect("") as usize;
                    pos += 1;

                    if chars.len() == 1 {
                        res.push(n);
                        return Some(res);
                    } else {
                        println!("2");
                        if let Some(id) = parse_id(&mut pos, chars.as_slice(), n) {
                            res.push(id);
                            println!("{}", id);
                            return Some(res);
                        } else {
                            res.push(n as usize);
                            return Some(res);
                        }
                    }
                }
                's' => {
                    pos += 1;

                    // 's:p'
                    if chars[pos] == ':' {
                        pos += 1;
                        if let Some(_s) = parse_status(chars[pos]) {
                        } else {
                            panic!()
                        }
                    }
                    todo!()
                }

                'p' => {
                    pos += 1;
                    parse_project("");
                    todo!()
                }

                _ => return None,
            }
        }
        None
    }
}

pub fn parse_project(_name: &str) {
    todo!()
}

pub fn parse_status(ch: char) -> Option<task::TaskStatus> {
    match ch {
        'p' => Some(task::TaskStatus::Pending),
        _ => None,
    }
}

pub fn parse_id(p: &mut usize, c: &[char], n: usize) -> Option<usize> {
    let mut digit = n;

    while *p < c.len() {
        match c[*p] {
            '0'..='9' => {
                digit = digit * 10 + c[*p].to_digit(10).expect("") as usize;

                *p += 1;
            }
            _ => {
                println!("d{}", digit);

                return Some(digit);
            }
        }
    }

    // 0 as task id is impossible
    if digit != 0 {
        Some(digit)
    } else {
        None
    }
}
