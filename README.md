# ElixirPoker
A program that deals and evaluates two five-card poker hands and chooses a winner.

The input to the program is the first ten values in a permutation of the integers 1-52 that 
represents a shuffling of a standard deck of cards. The order of suits in an unshuffled deck
are Clubs, Diamonds, Hearts, Spades. Within each suit, the ranks are ordered from Ace, 2-
10, Jack, Queen, King. 

Thus, an input array that started with the integers [38, 48, 11, 6, ...] would represent Queen
of Hearts, 9 of Spades, Jack of Clubs, 6 of Clubs, and so on.
This program will accept this permutation array as input and use it to deal two poker hands 
of 5 cards each in an alternating fashion. I.e., the first card goes to hand 1, the second 
card goes to hand 2, the third card goes to hand 1, fourth to hand 2, etc. until each hand has 5
cards. Once dealt, the program will analyze each hand according to the rules from the
website above and decide a winner.

There will be no ties. If both hands have the same type, tie breaking is implemented 
based on the ranks of the cards. For example, if both hands are a flush, then the hand with
the card of highest rank is declared the winner. If both hands have a pair, then the hand
whose pair is higher wins. For example, a pair of Kings beats a pair of Sevens. If both hands
have the same pair, i.e. each hand has a pair of threes, then the hand with the next highest
card wins. If both hands have the same pair, and the same high card, we check the 2 nd
highest card. And so on, and so forth. 
If, after comparing hands and ranks, it turns out that the hands are identical, we move on to
tie-breaking by suit. If both hands have a pair of threes, and all other cards are of the same
rank, then the hand with the three of spades will win.Tie breaking based on suit only
comes into effect when tie breaking based on rank is impossible. Suits are ranked from lowest
to highest in the following order: Clubs < Diamonds < Hearts < Spades
