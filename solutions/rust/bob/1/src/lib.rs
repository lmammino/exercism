fn is_uppercase(message: &str) -> bool {
    let mut has_alpha = false;
    for c in message.chars() {
        if c.is_alphabetic() {
            has_alpha = true;
            if c.is_lowercase() {
                return false;
            }
        }
    }

    return has_alpha;
}

fn is_question(message: &str) -> bool {
    let last_char = message.chars().last();
    if last_char.is_some() && last_char.unwrap() == '?' {
        return true;
    }

    false
}

pub fn reply(message: &str) -> &str {
    let uppercase = is_uppercase(message.trim());
    let question = is_question(message.trim());

    /*
        Bob answers:
        - 'Sure.' if you ask him a question, such as "How are you?".
        - 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
        - 'Calm down, I know what I'm doing!' if you yell a question at him.
        - 'Fine. Be that way!' if you address him without actually saying anything.
        - 'Whatever.' to anything else.
    */
    if message.trim().len() == 0 {
        return "Fine. Be that way!";
    } else if uppercase && question {
        return "Calm down, I know what I'm doing!";
    } else if uppercase {
        return "Whoa, chill out!";
    } else if question {
        return "Sure.";
    }

    return "Whatever.";
}
