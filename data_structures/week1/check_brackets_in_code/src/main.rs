fn is_balanced(s: String) -> Result<String, i32> {
    let valid_chars: Vec<char> = vec!['(', '[', '{', ')', ']', '}'];

    let mut stack: Vec<(i32, char)> = Vec::new();

    let mut index = 0;
    for ch in s.into_bytes().iter().map(|b| *b as char) {
        index = index + 1;
        if valid_chars.contains(&ch) {
            if ch == '(' || ch == '[' || ch == '{' {
                stack.push((index, ch));
            } else {
                let opt: Option<Result<String, i32>> = stack
                    .pop()
                    .map(|top| {
                        if (top.1 == '(' && ch != ')')
                            || (top.1 == '[' && ch != ']')
                            || (top.1 == '{' && ch != '}')
                        {
                            Some(Err(index))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(Some(Err(index)));
                if opt.is_some() {
                    return opt.unwrap();
                }
            }
        }
    }

    if stack.is_empty() {
        Ok("Success".into())
    } else {
        Err(stack.pop().unwrap().0)
    }
}

fn main() {
    let mut buff = String::with_capacity(4);
    ::std::io::stdin().read_line(&mut buff).unwrap();

    println!(
        "{}",
        is_balanced(buff.trim().into()).unwrap_or_else(|e| format!("{}", e))
    );
}

#[cfg(test)]
mod tests {
    use super::is_balanced;

    #[test]
    fn test1() {
        let result = is_balanced("[]".into());
        assert_eq!(result, Ok("Success".into()))
    }

    #[test]
    fn test2() {
        let result = is_balanced("()".into());
        assert_eq!(result, Ok("Success".into()))
    }

    #[test]
    fn test3() {
        let result = is_balanced("()[]".into());
        assert_eq!(result, Ok("Success".into()))
    }

    #[test]
    fn test4() {
        let result = is_balanced("([])".into());
        assert_eq!(result, Ok("Success".into()))
    }

    #[test]
    fn test5() {
        let result = is_balanced("([]".into());
        assert_eq!(result, Err(1))
    }

    #[test]
    fn test6() {
        let result = is_balanced("())".into());
        assert_eq!(result, Err(3))
    }

    #[test]
    fn test7() {
        let result = is_balanced("foo(bar)".into());
        assert_eq!(result, Ok("Success".into()))
    }

    #[test]
    fn test8() {
        let result = is_balanced("foo(bar[i)".into());
        assert_eq!(result, Err(10))
    }

    #[test]
    fn test9() {
        let result = is_balanced("[](()".into());
        assert_eq!(result, Err(3))
    }

}
