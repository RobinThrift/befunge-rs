


pub fn parse(code: Vec<String>) -> Vec<Vec<String>> {
    let mut tokens = Vec::new();
    for l in code.iter() {
        let mut line_tokens = Vec::new();
        for c in l.as_slice().chars() {
            line_tokens.push(c.to_string());
        }
        tokens.push(line_tokens);
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_test() {
        let input = vec!["3+3".to_string(), "#".to_string()];

        let tokens = super::parse(input);
        
        assert_eq!(tokens, vec![
            vec!["3".to_string(), "+".to_string(), "3".to_string()],
            vec!["#".to_string()]
        ]);
    }
}
