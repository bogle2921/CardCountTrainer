use std::env;
use std::collections::HashMap;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || &args[1] == "help" {
        usage();
    }

    let mut numDecks: i32 = 1;
    let mut numPlayers: i32 = 1;

    if args.len() == 3 {
        numDecks = args[2].parse::<i32>().unwrap();
    }

    if args.len() == 4 {
        numPlayers = args[3].parse::<i32>().unwrap();
    }

    dbg!(numDecks);
    dbg!(numPlayers);

    let mut cards = HashMap::from([
        ("A", 11),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("10", 10),
        ("J", 10),
        ("Q", 10),
        ("K", 10),
    ]);

    
}

fn swap(a: &mut i32, b: &mut i32){
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn usage(){
    println!("\n\t./cct <help|play> <num decks?> <num players?> \n");
    println!("A value is assigned to every card:\n\t2-6: +1\n\t7-9: 0\n\t10-Ace: -1");
    println!("Keep a running count. Positive count is player advantage, negative count is house advantage.");
    println!("NOTE: when multiple desks are used: true count = running count / number of decks");
    std::process::exit(0);
}