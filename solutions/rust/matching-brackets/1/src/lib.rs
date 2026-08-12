const BRACKETS : [char;6] = ['(',')', '[', ']', '{','}'];

struct Bracket {
    symbol: char,
    is_open: bool,
    complement: char
}

fn get_bracket (symbol: char) -> Bracket {
    let i = BRACKETS.iter().enumerate().find(|x| *x.1 == symbol).unwrap().0;
    let is_open = i % 2 == 0;
    let complement = if is_open { BRACKETS[i+1] } else { BRACKETS[i-1] };

    Bracket{
        symbol,
        is_open,
        complement,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets : Vec<Bracket> = Vec::new();

    for c in string.chars() {
        // needs to be a bracket, otherwhise is skipped
        
        if BRACKETS.contains(&c) {
            let bracket = get_bracket(c);
            if bracket.is_open {
                open_brackets.push(bracket);
            } else {
                let last = open_brackets.last();
                if last.is_some() && last.unwrap().symbol == bracket.complement {
                    open_brackets.pop();
                } else {
                    // non matching bracket
                    return false;
                }
            }
        }
    }

    return open_brackets.len() == 0;
}
