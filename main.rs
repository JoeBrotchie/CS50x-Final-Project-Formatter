use clap::Parser;
use regex::{Regex, RegexSet};
use std::{fs, io, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    tab_width: Option<u8>,

    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read the file into a String.
    let file = fs::read_to_string(cli.path.clone())?;

    // Run the format function.
    let result = format(file);

    //print!("{result}");
    // Write the file out.
    fs::write(cli.path, result)?;

    return Ok(());
}

fn format(file: String) -> String {
    let vec = make_vec_string(file);

    let math = RegexSet::new(&[
        r"[+|-][+|-][^[:space:]|^[:punct:]]",
        r"[^[:space:]|^[:punct:]][=|+|-|>|<|!][=][^[:space:]|^[:punct:]]",
        r"[=|+|-|>|<|!][=][^[:space:]|^[:punct:]]",
        r"[^[:space:]|^[:punct:]][=][^[:space:]|^[:punct:]]",
        r"[=][^[:space:]|^[:punct:]]",
    ])
    .unwrap();

    let curly_brackets = RegexSet::new(&[r"[{]", r"[}]"]).unwrap();

    let mut fileout = String::new();
    let mut tab_count = 0;
    let tab_spacing = 4;

    for (index, mut word) in vec.clone().into_iter().enumerate() {
        let is_quotation = quotations(&vec, index);

        if is_quotation {
            fileout.push_str(&word);
            continue;
        }

        let is_comment = comment(&vec, index);

        if is_comment {
            word = word_space(&mut word, tab_count, tab_spacing, &vec, index);
            fileout.push_str(&word);
            continue;
        }

        if curly_brackets.is_match(&word) {
            tab_count = tab_counting(tab_count, &word);
        }

        while (math.is_match(&word)) == true {
            for m in math.matches(&word) {
                word = math_spaces(m, &word);
                break;
            }
        }

        word = word_space(&mut word, tab_count, tab_spacing, &vec, index);

        fileout.push_str(&word);
    }

    return fileout;
}

fn make_vec_string(file: String) -> Vec<String> {
    let mut fileout: Vec<String> = Vec::new();

    let mut temp = String::new();

    let mut is_quotation_mark = false;
    let mut is_single_line_comment = false;
    let mut is_multi_line_comment = false;

    for (index, c) in file.chars().enumerate() {
        if c == '"' && !is_single_line_comment && !is_multi_line_comment {
            is_quotation_mark = !is_quotation_mark;

            if is_quotation_mark {
                fileout.push(temp.clone());
                temp.clear();
                temp.push(c);
            } else {
                temp.push(c);
                fileout.push(temp.clone());
                temp.clear();
            }
        } else if c == '/'
            && file.chars().nth(index + 1) == Some('/')
            && !is_quotation_mark
            && !is_multi_line_comment
        {
            is_single_line_comment = true;
            if temp != "" {
                fileout.push(temp.clone());
                temp.clear();
            }
            temp.push(c);
        } else if c == '/'
            && file.chars().nth(index + 1) == Some('*')
            && !is_quotation_mark
            && !is_single_line_comment
        {
            is_multi_line_comment = true;
            if temp != "" {
                fileout.push(temp.clone());
                temp.clear();
            }
            temp.push(c);
        } else if c == '/' && file.chars().nth(index - 1) == Some('*') && is_multi_line_comment {
            is_multi_line_comment = false;
            temp.push(c);
            fileout.push(temp.clone());
            temp.clear();
        } else if !is_quotation_mark && !is_multi_line_comment && !is_single_line_comment {
            if c == ' ' && temp != "" {
                fileout.push(temp.clone());
                temp.clear();
            } else if c == '\n' {
                temp.push(c);
                fileout.push(temp.clone());
                temp.clear();
            } else if c != ' ' {
                temp.push(c);
            }
        } else {
            if c == '\n' && is_single_line_comment {
                is_single_line_comment = false;
                temp.push(c);
                fileout.push(temp.clone());
                temp.clear();
            } else {
                temp.push(c);
            }
        }
    }
    fileout.push(temp);

    return fileout;
}

fn quotations(vec: &Vec<String>, index: usize) -> bool {
    if vec[index].chars().nth(0) == Some('"') {
        return true;
    }
    return false;
}

fn comment(vec: &Vec<String>, index: usize) -> bool {
    if vec[index].chars().nth(0) == Some('/') && vec[index].chars().nth(1) == Some('/') {
        return true;
    }
    if vec[index].chars().nth(0) == Some('/') && vec[index].chars().nth(1) == Some('*') {
        return true;
    }
    return false;
}

fn tab_counting(mut tab_count: i32, input: &String) -> i32 {
    for c in input.chars() {
        match c {
            '{' => tab_count += 1,
            '}' => tab_count -= 1,
            _ => {}
        }
    }

    return tab_count;
}

fn word_space(
    word: &mut String,
    tab_count: i32,
    tab_spacing: i32,
    vec: &Vec<String>,
    index: usize,
) -> String {
    let word_len: usize = ((word.chars().count() as i32) - 1) as usize;

    if word.chars().nth(word_len) == Some('\n') {
        *word = tab_indent(tab_count, tab_spacing, word, &vec, index);
    } else if word.chars().nth(word_len) != Some(' ') {
        if (index + 1) != vec.len() {
            if vec[index + 1].chars().nth(0) != Some('"') {
                word.push(' ');
            }
        } else {
            word.push(' ');
        }
    }
    return word.to_string();
}

fn tab_indent(
    tab_count: i32,
    tab_spacing: i32,
    word: &mut String,
    vec: &Vec<String>,
    index: usize,
) -> String {
    if (index + 1) != vec.len() {
        if vec[index + 1].chars().nth(0) == Some('}') {
            for _ in 0..((tab_count - 1) * tab_spacing) {
                word.push(' ');
            }
            return word.to_string();
        }
    }
    for _ in 0..((tab_count) * tab_spacing) {
        word.push(' ');
    }

    return word.to_string();
}

fn math_spaces(matches: usize, input: &str) -> String {
    let math_0 = Regex::new(r"(?P<m1>[+|-])(?P<m2>[+|-])(?P<s1>[^[:space:]])").unwrap();
    let math_1 =
        Regex::new(r"(?P<s1>[^[:space:]])(?P<m1>[=|+|-|<|>|!])(?P<m2>[=])(?P<s2>[^[:space:]])")
            .unwrap();
    let math_2 = Regex::new(r"(?P<m1>[=|+|-|<|>|!])(?P<m2>[=])(?P<s1>[^[:space:]])").unwrap();
    let math_3 = Regex::new(r"(?P<s1>[^[:space:]])(?P<m1>[=|+|-])(?P<s2>[^[:space:]])").unwrap();
    let math_4 = Regex::new(r"(?P<m1>[=|+|-])(?P<s1>[^[:space:]])").unwrap();

    match matches {
        0 => math_0.replace(input, "$m1$m2 $s1"),
        1 => math_1.replace(input, "$s1 $m1$m2 $s2"),
        2 => math_2.replace(input, "$m1$m2 $s1"),
        3 => math_3.replace(input, "$s1 $m1 $s2"),
        4 => math_4.replace(input, "$m1 $s1"),
        _ => input.into(),
    }
    .to_string()
}
