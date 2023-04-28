pub fn get_choices(message: String) -> Vec<String> {
    let lines: Vec<&str> = message.split('\n').collect();
    let mut choices = Vec::new();

    for line in lines {
        if line.chars().next().map_or(false, |c| c.is_digit(10)) {
            let msg = line.split(". ").skip(1).collect::<String>();
            choices.push(msg);
        }
    }
    choices
}
