use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn short_form(&self) -> &'static str {
        match *self {
            Suit::Spades => "S",
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
        }
    }
}

impl Suit {
	fn best_suit(&self) -> u8 {
		match *self {
			Suit::Spades => 1, //best suit
            Suit::Hearts => 2,
            Suit::Diamonds => 3,
            Suit::Clubs => 4, //worst suit
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
    		Card{ value: value - 13, suit: Suit::Diamonds }
    	} else if value < 40 {
    		Card{ value: value - 26, suit: Suit::Hearts }
    	} else {
    		Card{ value: value - 39, suit: Suit::Spades }
    	}
        
    }
}

impl fmt::Display for Card { //TODO change this for sure
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value.to_string(), self.suit.short_form())
    }
}

impl Card {
	fn compare_val(&self) -> u32{
		if self.value == 1 {
			return 14
		} else {
			return self.value
		}
	}
}


fn deal(arr: [u32; 10]) -> [String;5]{

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
	hand1 = sort_hand(hand1);
	hand2 = sort_hand(hand2);
	//determine what type of hand it is
	println!("Hands: ");
	println!("{:?}", hand1);
	println!("{:?}", hand2);
	//["1C", "2C", "3C", "4C", "5C"];


	let mut winning_hand = compare(hand1, hand2);
	winning_hand.sort_by_key(|x| x.value);
	let winner = [winning_hand[0].to_string(), winning_hand[1].to_string(), 
	winning_hand[2].to_string(), winning_hand[3].to_string(), winning_hand[4].to_string()];
	winner
}

fn compare(hand1: Vec<Card>, hand2: Vec<Card>) -> Vec<Card> {
	//I check for every hand, and assign it a numerical value
	//depending on how good the hand is. at this point not dealing with ties.
	//if two hand end up having the same numerical value.
	//returns the numerical raking value for both lists as a tuple/list.
	let hand1_value = get_value(&hand1);
	let hand2_value = get_value(&hand2);
	println!("hand values ---");
	println!("{}", hand1_value);
	println!("{}", hand2_value);

	if hand1_value == hand2_value {
		return tie_breaker(hand1, hand2, hand1_value)
	}
	else {
		if hand1_value < hand2_value {
			hand1
		}
		else {
			hand2
		}
	}
}

fn get_value(hand: &Vec<Card>) -> u8 {
	//if theyre all the same suit
	if hand[0].suit == hand[1].suit && hand[1].suit == hand[2].suit && 
	hand[2].suit == hand[3].suit && hand[3].suit == hand[4].suit {
		//check if its a full house
		if hand[0].value == 10 && hand[1].value == 11 && hand[2].value == 12 && hand[3].value == 13 
		&& hand[4].value == 1 {
			return 1
		} else if (hand[0].value == hand[1].value -1 && hand[1].value == hand[2].value -1 
		&& hand[2].value == hand[3].value -1 && hand[3].value == hand[4].value -1) ||
		(hand[0].value == 2 && hand[1].value == 3 && hand[2].value == 4 && hand[3].value == 5
		&& hand[4].value == 1) { //its a straight flush
			return 2
		} else { //its a normal flush
			return 5
		}
	} else {
		//if its 4 of a kind, 
		if (hand[0].value == hand[1].value && hand[1].value == hand[2].value && 
		hand[2].value == hand[3].value) ||  
		(hand[1].value == hand[2].value && hand[2].value == hand[3].value && 
		hand[3].value == hand[4].value) {
			return 3
		} else if (hand[0].value == hand[1].value && hand[1].value == hand[2].value && 
		hand[3].value == hand[4].value) ||  
		(hand[0].value == hand[1].value && hand[2].value == hand[3].value && 
		hand[3].value == hand[4].value) { //if its full house
			return 4
		} else if hand[0].value == hand[1].value -1 && hand[1].value == hand[2].value -1 
		&& hand[2].value == hand[3].value -1 && hand[3].value == hand[4].value -1 { // if its a straight
			return 6
		} else if (hand[0].value == hand[1].value && hand[1].value == hand[2].value) ||  
		(hand[2].value == hand[3].value && hand[3].value == hand[4].value){ //if its 3 of a kind
			return 7
		} else if (hand[0].value == hand[1].value && hand[2].value == hand[3].value) ||  
		(hand[0].value == hand[1].value && hand[3].value == hand[4].value) || 
		(hand[1].value == hand[2].value && hand[3].value == hand[4].value)  { //if its two pair
			return 8
		} else if hand[0].value == hand[1].value || hand[1].value == hand[2].value ||  
		hand[2].value == hand[3].value || hand[3].value == hand[4].value { //if its a pair.
			return 9
		} else {
			return 10
		}
	}
}

fn sort_hand(mut hand: Vec<Card>) -> Vec<Card> {
	hand.sort_by_key(|x| x.value);
	while hand[0].value == 1 {
		hand.push(hand[0]);
		hand.remove(0);
	}
	return hand
}


fn tie_breaker(hand1: Vec<Card>, hand2: Vec<Card>, hand_value: u8) -> Vec<Card> {
	if hand_value == 1 {
		if hand1[0].suit.best_suit() < hand2[0].suit.best_suit() {
			return hand1
		} else {
			return hand2
		}
	} else if hand_value == 2 { // compare the value of the highest is the flushes
		if hand1[3].compare_val() > hand2[3].compare_val() {
			return hand1
		} else if hand1[0].value == hand2[0].value && hand1[4].value > hand2[4].value {
			return hand1
		} else if hand1[0].value == hand2[0].value && hand1[4].value < hand2[4].value {
			return hand2
		} else if hand1[0].value == hand2[0].value {
			if hand1[0].suit.best_suit() < hand2[0].suit.best_suit() {
				return hand1
			} else {
				return hand2
			}
		} else {
			return hand2
		}
	} else if hand_value == 3 || hand_value == 4 || hand_value == 7 { // 4 of a kind or full house or 3 of a kind
		if hand1[2].compare_val() > hand2[2].compare_val() { //the middle value will always be part of the 4 or 3 of a kind
			return hand1
		} else {
			return hand2
		}
	} else if hand_value == 5 { // flush
		let mut i:usize = 4;
		while hand1[i].compare_val() == hand2[i].compare_val() && i != 0 {
			i = i-1;
		}
		if hand1[i].compare_val() == hand2[i].compare_val() {
			if hand1[4].suit.best_suit() < hand2[4].suit.best_suit() {
				return hand1
			} else {
				return hand2
			}
		} else {
			if hand1[i].compare_val() > hand2[i].compare_val() {
				return hand1
			} else {
				return hand2
			}
		}
	} else if hand_value == 6 { // straight
		if hand1[3].compare_val() > hand2[3].compare_val() {
			return hand1
		} else if hand1[0].value == hand2[0].value && hand1[4].value > hand2[4].value {
			return hand1
		} else if hand1[0].value == hand2[0].value && hand1[4].value < hand2[4].value {
			return hand2
		} else if hand1[0].value == hand2[0].value {
			if hand1[0].suit.best_suit() < hand2[0].suit.best_suit() {
				return hand1
			} else {
				return hand2
			}
		} else {
			return hand2
		}
	} else if hand_value == 8 { // two pair
		let mut hand1_clone = hand1.clone();
		let mut hand2_clone = hand2.clone();
		
		let hand1_tup = find_pair_value(&hand1_clone);
		let hand2_tup = find_pair_value(&hand2_clone);
		if hand1_tup.0 > hand2_tup.0 {
			return hand1
		} else if hand1_tup.0 < hand2_tup.0 {
			return hand2
		} else { // check the other elems in the list
			
			//get the suits of the top pair
			let pair1_suit = if hand1_clone[hand1_tup.1].suit.best_suit() < hand1_clone[hand1_tup.1 -1].suit.best_suit() {
				hand1_clone[hand1_tup.1].suit.best_suit()
			} else {
				hand1_clone[hand1_tup.1 -1].suit.best_suit()
			};
			
			let pair2_suit = if hand2_clone[hand2_tup.1].suit.best_suit() < hand2_clone[hand2_tup.1 -1].suit.best_suit() {
				hand2_clone[hand2_tup.1].suit.best_suit()
			} else {
				hand2_clone[hand2_tup.1 -1].suit.best_suit()
			};
			//remove the pair
			hand1_clone.remove(hand1_tup.1 -1);
			hand1_clone.remove(hand1_tup.1 -1);
			hand2_clone.remove(hand1_tup.1 -1);
			hand2_clone.remove(hand1_tup.1 -1);

			let hand1_tup_2 = find_pair_value(&hand1_clone);
			let hand2_tup_2 = find_pair_value(&hand2_clone);

			if hand1_tup_2.0 > hand2_tup_2.0 {
				return hand1
			} else if hand1_tup_2.0 < hand2_tup_2.0 {
				return hand2
			} else { // check the other elems in the list 
				//remove the pair
				hand1_clone.remove(hand1_tup_2.1 -1);
				hand1_clone.remove(hand1_tup_2.1 -1);
				hand2_clone.remove(hand1_tup_2.1 -1);
				hand2_clone.remove(hand1_tup_2.1 -1);
				if hand1_clone[0].compare_val() == hand2_clone[0].compare_val() {
					if pair1_suit < pair2_suit {
						return hand1
					} else {
						return hand2
					}
				} else {
					if hand1_clone[0].compare_val() > hand2_clone[0].compare_val() {
						return hand1
					} else {
						return hand2
					}
				}
			}
		}
	} else if hand_value == 9 { // pair

		let mut hand1_clone = hand1.clone();
		let mut hand2_clone = hand2.clone();
		
		let hand1_tup = find_pair_value(&hand1_clone);
		let hand2_tup = find_pair_value(&hand2_clone);
		if hand1_tup.0 > hand2_tup.0 {
			return hand1
		} else if hand1_tup.0 < hand2_tup.0 {
			return hand2
		} else { // check the other elems in the list
			
			let pair1_suit = if hand1_clone[hand1_tup.1].suit.best_suit() < hand1_clone[hand1_tup.1 -1].suit.best_suit() {
				hand1_clone[hand1_tup.1].suit.best_suit()
			} else {
				hand1_clone[hand1_tup.1 -1].suit.best_suit()
			};
			
			let pair2_suit = if hand2_clone[hand2_tup.1].suit.best_suit() < hand2_clone[hand2_tup.1 -1].suit.best_suit() {
				hand2_clone[hand2_tup.1].suit.best_suit()
			} else {
				hand2_clone[hand2_tup.1 -1].suit.best_suit()
			};

			hand1_clone.remove(hand1_tup.1 -1);
			hand1_clone.remove(hand1_tup.1 -1);
			hand2_clone.remove(hand1_tup.1 -1);
			hand2_clone.remove(hand1_tup.1 -1);

			let mut i:usize = 2;
			while hand1_clone[i].compare_val() == hand2_clone[i].compare_val() && i != 0 {
				i = i-1;
			}
			if hand1_clone[i].compare_val() == hand2_clone[i].compare_val() {
				if pair1_suit < pair2_suit {
					return hand1
				} else {
					return hand2
				}
			} else {
				if hand1_clone[i].compare_val() > hand2_clone[i].compare_val() {
					return hand1
				} else {
					return hand2
				}
			}
		}
	} else { //high card
		let mut i:usize = 4;
		while hand1[i].compare_val() == hand2[i].compare_val() && i != 0 {
			i = i-1;
		}
		if hand1[i].compare_val() == hand2[i].compare_val() {
			if hand1[4].suit.best_suit() < hand2[4].suit.best_suit() {
				return hand1
			} else {
				return hand2
			}
		} else {
			if hand1[i].compare_val() > hand2[i].compare_val() {
				return hand1
			} else {
				return hand2
			}
		}
	}
}

fn find_pair_value(hand: &Vec<Card>) -> (u32, usize) { //should return the highest pair.
	let mut found = false;
	let mut i:usize = hand.len() -1;
	while found == false {
		
		if hand[i].compare_val() == hand[i-1].compare_val() {
			found = true;
			i = i + 1;
		}
		i = i-1;
	}
	return (hand[i].compare_val(), i)
}

fn main() {
	//let perm:[u32;10] = [2, 46, 3, 2, 4, 45, 5, 52, 6, 47];
	//let perm:[u32;10] = [1, 2, 26, 15, 27, 28, 40, 41, 14, 52]
	//let perm:[u32;10] = [8, 1, 2, 14, 35, 6, 26, 19, 14, 10]

	//test for royal flush
	//let perm:[u32;10] = [40, 14, 49, 23, 50, 24, 51, 25, 52, 26];

	//test for straight flush
	//let perm:[u32;10] = [48, 22, 49, 23, 50, 24, 51, 25, 52, 26];
	//let perm:[u32;10] = [1, 22, 2, 23, 3, 24, 4, 25, 5, 26];
	//let perm:[u32;10] = [1, 15, 2, 16, 3, 17, 4, 18, 5, 19];
	//let perm:[u32;10] = [1, 20, 2, 16, 3, 17, 4, 18, 5, 19];


	//test for 4 of a kind
	//let perm:[u32;10] = [1, 2, 26, 15, 27, 28, 40, 41, 14, 52];

	//test for full house
	//let perm:[u32;10] = [1, 39, 26, 15, 27, 28, 40, 41, 13, 52];

	//test for flush
	//let perm:[u32;10] = [41, 21, 48, 23, 50, 24, 51, 25, 52, 26];
	//let perm:[u32;10] = [48, 22, 49, 23, 50, 24, 51, 25, 52, 26];

	//test for straight
	//let perm:[u32;10] = [3, 16, 17, 30, 31, 18, 45, 33, 46, 32];
	//let perm:[u32;10] = [40, 14, 49, 23, 50, 24, 51, 25, 39, 13];

	//test three of a kind
	//let perm:[u32;10] = [13, 2, 26, 15, 39, 28, 9, 10, 12, 11];

	//test for two pair - all the same
	//let perm:[u32;10] = [28, 41, 23, 36, 5, 18, 31, 44, 2, 15];
	//test for two pair - same pairs, bigger card
	//let perm:[u32;10] = [28, 41, 24, 49, 5, 18, 31, 44, 2, 15];
	//test for two pair - one different pair
	//let perm:[u32;10] = [29, 41, 23, 36, 5, 18, 31, 44, 3, 15];
	//let perm:[u32;10] = [27, 41, 23, 36, 1, 18, 28, 44, 2, 15];
	//test for two pair - different pairs
	//let perm:[u32;10] = [29, 41, 23, 6, 36, 18, 31, 44, 3, 15];

	//test for pair - all the same
	//let perm:[u32;10] = [28, 41, 23, 36, 5, 18, 50, 11, 2, 15];
	//test for pair - same pairs, bigger card
	//let perm:[u32;10] = [28, 41, 23, 36, 4, 18, 50, 11, 2, 15];
	//test for pair - different pairs
	//let perm:[u32;10] = [29, 41, 23, 36, 5, 18, 50, 11, 3, 15];

	//test for high card
	//let perm:[u32;10] = [14, 24, 3, 4, 5, 6, 7, 8, 9, 10];
	//let perm:[u32;10] = [39, 26, 10, 23, 33, 46, 31, 18, 2, 16];
	//let perm:[u32;10] = [13, 26, 10, 23, 33, 46, 31, 18, 2, 15];

	//testcases failed last time
	//let perm:[u32;10] = [15, 48, 16, 35, 17, 22, 14, 49, 18, 9];
	//let perm:[u32;10] = [18, 48, 17, 9, 15, 22, 14, 35, 16, 49];
	//let perm:[u32;10] = [51, 40, 1, 38, 15, 41, 14, 28, 2, 27];
	//let perm:[u32;10] = [2, 40, 1, 38, 14, 28, 51, 41, 15, 27];
	//let perm:[u32;10] = [13, 45, 41, 47, 6, 52, 7, 2, 8, 46];
	//let perm:[u32;10] = [13, 46, 6, 2, 41, 45, 7, 52, 8, 47];
	//let perm:[u32;10] = [15, 38, 17, 26, 18, 51, 14, 39, 16, 52];
	let perm:[u32;10] = [14, 26, 17, 38, 15, 39, 18, 52, 16, 51];



	let winner:[String;5] = deal(perm);
	println!("{:?}", winner);
}