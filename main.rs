use std::env;
struct Card {
    display : char,
    value: i32,
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let style = &args[1];
    if style = "help" {
        usage();
    }
    let mut ace = Card { display: 'A', value: 11 };
    let two = Card { display: '2', value: 2 };
    let three = Card { display: '3', value: 3 };
    let four = Card { display: '4', value: 4 };
    let five = Card { display: '5', value: 5 };
    let six = Card { display: '6', value: 6 };
    let seven = Card { display: '7', value: 7 };
    let eight = Card { display: '8', value: 8 };
    let nine = Card { display: '9', value: 9 };
    let ten = Card { display: "10", value: 10 };
    let jack = Card { display: 'J', value: 10 };
    let queen = Card { display: 'Q', value: 10 };
    let king = Card { display: 'K', value: 10 };
    let mut deck = vec![ace, ace, ace, ace, two, two, two, two, three, three, three, three,
                       four, four, four, four, five, five, five, five, six, six, six, six,
                       seven, seven, seven, seven, eight, eight, eight, eight, nine, nine, nine, nine,
                       ten, ten, ten, ten, jack, jack, jack, jack, queen, queen, queen, queen, king, king, king, king];
    
}

fn swap(a: &mut i32, b: &mut i32){
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn usage(){
    println!("\n\t./cct <help|play> <num players?> <num decks?> \n");
    println!("A value is assigned to every card:\n\t2-6: +1\n\t7-9: 0\n\t10-Ace: -1");
    println!("Keep a running count. Positive count is player advantage, negative count is house advantage.");
    println!("NOTE: when multiple desks are used: true count = running count / number of decks");
}