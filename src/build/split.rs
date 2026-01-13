use std::{borrow::Cow};

pub fn split(command: &str) -> Vec<String> {
    let mut args =  Vec::new();
    let mut in_quotes = None;
    let mut brace_depth = 0;
    
    let mut chars = command.char_indices().peekable();
    let mut arg_start = 0;

    while let Some((idx, ch)) = chars.next() {
        match ch {
            '#' if in_quotes.is_none() => break,
           
            '\'' | '"' if in_quotes.is_none() => {
                if idx > arg_start {
                    let part = Cow::Borrowed(&command[arg_start..idx]);
                    args.extend(expand_braces(part));
                }
                in_quotes = Some(ch);
                arg_start = idx + 1; 
            }
            q if in_quotes == Some(q) => {
                if idx > arg_start {
                    let part = Cow::Borrowed(&command[arg_start..idx]);
                    args.push(part); 
                }
                in_quotes = None;
                arg_start = idx + 1;
            }
            
            '{' if in_quotes.is_none() => brace_depth +=1,
            '}' if in_quotes.is_none() => brace_depth = (brace_depth - 1).max(0),
            
            ' ' | '\t' | '|' | '>' if in_quotes.is_none() && brace_depth == 0 => {
                if idx > arg_start {
                    let part = Cow::Borrowed(&command[arg_start..idx]);
                    args.extend(expand_braces(part));
                }
                if ch == '|' || ch == '>' {
                    let mut end_special = idx + 1;
                    if ch == '>' 
                        && let Some(&(_, next_ch)) = chars.peek() 
                            && next_ch == '>' {
                                chars.next();
                                end_special += 1;
                    }
                    args.push(Cow::Borrowed(&command[idx..end_special]));
                    arg_start = end_special;
                } else {
                    arg_start = idx + 1;
                }
            }
            _ => {}
        }
    }
    if arg_start < command.len() {
        let tail = command[arg_start..].trim();
        if !tail.is_empty() {
            args.extend(expand_braces(Cow::Borrowed(tail)));
        }
    }
    
    args.iter().map(|x| x.to_string()).collect()
}

fn expand_braces(input: Cow<str>) -> Vec<Cow<str>> {
    let mut args = Vec::new();
    if let Some(start) = input.find('{') {
        let mut depth = 0;
        let mut end = None;
        for (index, ch) in input.char_indices().skip(start) {
            if ch == '{' {
                depth+=1;
            } else if ch == '}' {
                depth-=1;
                if depth==0{
                    end = Some(index);
                    break;
                }
            }
        }
        if let Some(end_idx) = end {
            let prefix = &input[..start];
            let suffix = &input[end_idx+1..];
            let content = input[start+1..end_idx].to_string();

            let parts = split_brace_content(content);
            for part in parts {
                let full_word = 
                    Cow::Owned(format!("{}{}{}", prefix, part, suffix));
                let arg = expand_braces(full_word); 
                args.extend(arg);
            } 
        }
        return args
    } 
    args.push(input);
    args
}


fn split_brace_content<'a>(content: String) -> Vec<Cow<'a, str>> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut last_start = 0;
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                let segment = content[last_start..i].trim();
                parts.push(Cow::Owned(segment.to_string()));
                last_start = i + 1; 
            }
            _ => {}
        }
        i += 1;
    }
    let segment = content[last_start..].trim();
    parts.push(Cow::Owned(segment.to_string()));
    
    parts
}
