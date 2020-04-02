use std::fmt;
//let perm:[u32;10] = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
//let winner:[&str;5] = deal(perm);
//Your deal function should accept an array of 10 unsigned integers, and your deal function
//should return an array of five string slices. The strings should be formatted in the same way
//as all previous assignments.
//Your submission should not include a main() function. Our tester will implement main()


//I should create Card Classes, so I can compare them easily. Dont have to do all the modulp stuff i had to do 
//for elixir.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn short_string(&self) -> &'static str {
        match *self {
            Suit::Spades => "S",
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
        }
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
struct Card {
    value: u32,
    suit: Suit
}

impl Card {
    fn new(value: u32) -> Card {
    	if value < 14 {
    		Card{ value: value, suit: Suit::Clubs }
    	} else if value < 27 {
    		Card{ value: value, suit: Suit::Diamonds }
    	} else if value < 40 {
    		Card{ value: value, suit: Suit::Hearts }
    	} else {
    		Card{ value: value, suit: Suit::Spades }
    	}
        
    }
}

impl fmt::Display for Card { //TODO change this for sure
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value.to_string(), self.suit.short_string())
    }
}


fn deal(arr: [u32; 10]) {

	//split the list into the hands
	let mut hand1: Vec<Card> = Vec::new();
	let mut hand2: Vec<Card> = Vec::new();
	let mut switch = true;
	for elem in arr.iter() {
		if switch == true {
			hand1.push(Card::new(*elem));
			switch = false;
		} else {
			hand2.push(Card::new(*elem));
			switch = true;
		}
	}
	//determine what type of hand it is
	println!("Hands: ");
	println!("{:?}", hand1);
	println!("{:?}", hand2);
	println!("{}", hand1[0]);
	//["1C", "2C", "3C", "4C", "5C"];
	compare(hand1, hand2);

}

fn compare(hand1: Vec<Card>, hand2: Vec<Card>) {
	//I check for every hand, and assign it a numerical value
	//depending on how good the hand is. at this point not dealing with ties.
	//if two hand end up having the same numerical value.
	//returns the numerical raking value for both lists as a tuple/list.
	let hand1_value = get_value(hand1);
	let hand2_value = get_value(hand2);
	println!("hand values ---");
	println!("{}", hand1_value);
	println!("{}", hand2_value);

	if hand1_value == hand2_value {
		//tieBreaker(hand1, hand2, hand1_value)
	}
	else {
		if hand1_value > hand2_value {
			//printWinner(hand1)
		}
		else {
			//printWinner(hand2)
		}
	}
}

fn get_value(hand: Vec<Card>) -> u32 {
	1

}

//fn tie_breaker(hand1: Vec<u32>, hand2: Vec<u32>, handValue: u32) -> Vec<u32> {

//}


fn main() {
	let perm:[u32;10] = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
	//let winner:[&str;5] = deal(perm);
	deal(perm)
}