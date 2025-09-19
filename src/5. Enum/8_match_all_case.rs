// using short cut to fill all possible cases

enum DiceNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

fn dice_move(dic_num: DiceNumber) {
    match dic_num {
        DiceNumber::Three => add_fancy_hat(),
        DiceNumber::Seven => remove_fancy_hat(),

        // using other if want to use other value
        // other => move_player(other),

        // if do not wish to use, using "_"
        _ => reroll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(dic_num: DiceNumber) {}
fn reroll(){}
