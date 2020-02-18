defmodule Poker do
	def deal(list) do
		hand1 = []
		hand2 = []
		#split list into 2 lists
		hand2 = [Enum.at(list, 9) | hand2]
		hand1 = [Enum.at(list, 8) | hand1]
		hand2 = [Enum.at(list, 7) | hand2]
		hand1 = [Enum.at(list, 6) | hand1]
		hand2 = [Enum.at(list, 5) | hand2]
		hand1 = [Enum.at(list, 4) | hand1]
		hand2 = [Enum.at(list, 3) | hand2]
		hand1 = [Enum.at(list, 2) | hand1]
		hand2 = [Enum.at(list, 1) | hand2]
		hand1 = [Enum.at(list, 0) | hand1]

		hand1 = Enum.sort(hand1)
		hand2 = Enum.sort(hand2)
		IO.puts "hands -----"
		IO.inspect hand1 
		IO.inspect hand2 
		compare(hand1, hand2)
	end
	
	#how am i going to do this. I think i check for every hand, and assign it a numerical value
	#depending on how good the hand is. at this point not dealing with ties. Ill make another function do that
	#if two hand end up having the same numerical value.
	#returns the numerical raking value for both lists as a tuple/list.
	def compare(hand1, hand2) do
		hand1_value = getValue(hand1)
		hand2_value = getValue(hand2)
		IO.puts "hand values ---"
		IO.inspect hand1_value
		IO.inspect hand2_value
		if hand1_value == hand2_value do
			tieBreaker(hand1, hand2, hand1_value)
		else
			if hand1_value > hand2_value do
				printWinner(hand1)
			else
				printWinner(hand2)
			end
		end
	end	

	def getValue(list) do
		hand_numbers = turnIntoHand(list, 2)
		hand_numbers = Enum.sort(hand_numbers)
		IO.inspect hand_numbers

		#check if its a royal flush - 10
		if Enum.member?(hand_numbers, 10) && Enum.member?(hand_numbers, 11)
		&& Enum.member?(hand_numbers, 12) && Enum.member?(hand_numbers, 13)
		&& Enum.member?(hand_numbers, 14) #they are consecuitve
		&& checkSuit(Enum.at(list, 4)) == checkSuit(Enum.at(list, 3)) 
		&& checkSuit(Enum.at(list, 3)) == checkSuit(Enum.at(list, 2)) 
		&& checkSuit(Enum.at(list, 2)) == checkSuit(Enum.at(list, 1)) 
		&& checkSuit(Enum.at(list, 1)) == checkSuit(Enum.at(list, 0)) #they are the same suit
		do
		
			hand_value = 10
		else
			#check if its a straight flush - 9
			#if each number is equal to the next -1 its the same. have to make sure
			#we dont flip into another suit
			if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3) + 1) 
			&& (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2) + 1) 
			&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1) + 1) 
			&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0) + 1) #make sure theyre consecutive
			&& checkSuit(Enum.at(list, 4)) == checkSuit(Enum.at(list, 3)) 
			&& checkSuit(Enum.at(list, 3)) == checkSuit(Enum.at(list, 2)) 
			&& checkSuit(Enum.at(list, 2)) == checkSuit(Enum.at(list, 1)) 
			&& checkSuit(Enum.at(list, 1)) == checkSuit(Enum.at(list, 0))  do #they are the same suit
				hand_value = 9
			else
				#check if its a four of a kind - 8
				if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
				&& (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2)) 
				&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1)) 
				|| (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2)) 
				&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1)) 
				&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) do
					hand_value = 8
				else
					#check if its a full house 3 of a kind + a pair- 7
					if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
					&& (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2)) 
					&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) 
					|| (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
					&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1)) 
					&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) do
				
						hand_value = 7
					else
					#check if its a flush: 5 of the same suit - 6
						if checkSuit(Enum.at(list, 4)) == checkSuit(Enum.at(list, 3)) 
						&& checkSuit(Enum.at(list, 3)) == checkSuit(Enum.at(list, 2)) 
						&& checkSuit(Enum.at(list, 2)) == checkSuit(Enum.at(list, 1)) 
						&& checkSuit(Enum.at(list, 1)) == checkSuit(Enum.at(list, 0))  do
							hand_value = 6
						else
						#check if its a straight - 5
							if Enum.at(hand_numbers, 4) == (Enum.at(hand_numbers, 3) + 1) 
							&& (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2) + 1) 
							&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1) + 1) 
							&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0) + 1) 
							&& Enum.at(hand_numbers, 0) != 1 do #make sure theyre consecutive
								hand_value = 5
							else
								#check if its a three of a kind - 4
								if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
								&& (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2))
								|| (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2))  
								&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1))
								|| (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1))  
								&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) do
									hand_value = 4
								else
									#check if its a two pairs - 3
									if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
									&& (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1))
									|| (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2))  
									&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0))
									|| (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3))  
									&& (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) do
										hand_value = 3
									else
										#Its this by default, dont need to check.
										#check if its a pair - 2
										if (Enum.at(hand_numbers, 4)) == (Enum.at(hand_numbers, 3)) 
										|| (Enum.at(hand_numbers, 3)) == (Enum.at(hand_numbers, 2))
										|| (Enum.at(hand_numbers, 2)) == (Enum.at(hand_numbers, 1))  
										|| (Enum.at(hand_numbers, 1)) == (Enum.at(hand_numbers, 0)) do
											hand_value = 2
										else # its only got a high card. check if its a high card - 1. 
											hand_value = 1 
										end
									end
								end
							end
						end
					end
				end
			end
		end
	end

	#at this point i know that the hands are the same. now check the numerical value of the cards.
	#should this function only be passed the part of the hand that matters (like the pair?).
	#either returns the winning hand or a 0.
	def tieBreaker(list1, list2, hand_value) do
		if hand_value == 1 do
			breakHighCardTie(list1, list2)
		else
			if hand_value == 2 do
				breakPairTie(list1, list2)
			else
				if hand_value == 3 do
					breakTwoPairTie(list1, list2)
				else
					if hand_value == 4 do
						breakThreeOfAKindTie(list1, list2)
					else
						if hand_value == 5 do
							breakStraightTie(list1, list2)
						else
							if hand_value == 6 do
								breakFlushTie(list1, list2)
							else
								if hand_value == 7 do
									breakFullHouseTie(list1, list2)
								else
									if hand_value == 8 do
										breakFourOfAKindTie(list1, list2)
									else
										if hand_value == 9 do
											breakStraightFlushTie(list1, list2)
										else
											if hand_value == 10 do
												breakRoyalFlushTie(list1, list2)
											end
										end
									end
								end
							end
						end
					end
				end
			end
		end
	end

	def breakHighCardTie(list1, list2) do
		hand1 = turnIntoHand(list1, 2, true)
		hand2 = turnIntoHand(list2, 2, true)
		res = returnHigestNonIdenticalCard(hand1, hand2)
		if res == 1 do
			printWinner(list1)
		else
			if res == 2 do
				printWinner(list2)
			else
				#by now we know that we have a totally identical hand, and to break the tie we need
				#to compare the suits of the two highest cards. //TODO
				highest_suit = breakCardSuitTie(hd(list1), hd(list2))
				if highest_suit == hd(list1) do
					printWinner(list1)
				else
					printWinner(list2)
				end
			end
		end
	end

	def breakPairTie(list1, list2) do
		
	end

	def breakTwoPairTie(list1, list2) do
		
	end

	def breakThreeOfAKindTie(list1, list2) do
		
	end

	def breakStraightTie(list1, list2) do
		hand1 = turnIntoHand(list1, 2, true)
		hand2 = turnIntoHand(list2, 2, true)
		res = returnHigestNonIdenticalCard(hand1, hand2)
		if res == 1 do
			printWinner(list1)
		else
			if res == 2 do
				printWinner(list2)
			else
				#by now we know that we have a totally identical hand, and to break the tie we need
				#to compare the suits of the two highest cards. //TODO
				highest_suit = breakCardSuitTie(hd(list1), hd(list2))
				if highest_suit == hd(list1) do
					printWinner(list1)
				else
					printWinner(list2)
				end
			end
		end
	end

	def breakFlushTie(list1, list2) do
		hand1 = turnIntoHand(list1, 2, true)
		hand2 = turnIntoHand(list2, 2, true)
		res = returnHigestNonIdenticalCard(hand1, hand2)
		if res == 1 do
			printWinner(list1)
		else
			if res == 2 do
				printWinner(list2)
			else
				#by now we know that we have a totally identical hand, and to break the tie we need
				#to compare the suits of the two highest cards. //TODO
				highest_suit = breakCardSuitTie(hd(list1), hd(list2))
				if highest_suit == hd(list1) do
					printWinner(list1)
				else
					printWinner(list2)
				end
			end
		end
	end

	def breakFullHouseTie(list1, list2) do
		#find out which are the 3 and which are the 2
	end

	def breakFourOfAKindTie(list1, list2) do
		#just need to compare the 4 of a kind ones. Theres no way they can be the same 4 cards lol
		#find out which are the 4
		hand1 = turnIntoHand(list1, 2, true)
		hand2 = turnIntoHand(list2, 2, true)
		hand1card = Enum.at(hand1, 2)
		hand2card = Enum.at(hand2, 2)
		#now which is highest
		if hand1card > hand2card do
			printWinner(list1)
		else
			if hand1card < hand2card do
				printWinner(list2)
			end
		end

	end

	def breakStraightFlushTie(list1, list2) do
		hand1 = turnIntoHand(list1, 2, true)
		hand2 = turnIntoHand(list2, 2, true)
		if hd(hand1) > hd(hand2) do
			printWinner(list1)
		else
			if hd(hand1) < hd(hand2) do
				printWinner(list2)
			else
				#by now we know that we have a totally identical hand, and to break the tie we need
				#to compare the suits of the two highest cards. #the whole hand is in the same suit so:
				highest_suit = breakCardSuitTie(hd(list1), hd(list2))
				if highest_suit == hd(list1) do
					printWinner(list1)
				else
					printWinner(list2)
				end
			end
		end
	end

	def breakRoyalFlushTie(list1, list2) do
		highest_suit = breakCardSuitTie(hd(list1), hd(list2))
		if highest_suit == hd(list1) do
			printWinner(list1)
		else
			printWinner(list2)
		end
	end

	def returnHigestNonIdenticalCard(list1, list2) do
		if list1 == [] do
			0
		else
			if hd(list1) > hd(list2) do
				1
			else
				if hd(list2) > hd(list1) do
					2
				else
					returnHigestNonIdenticalCard(tl(list1), tl(list2))
				end
			end
		end
	end

	def breakCardSuitTie(card1, card2) do
		suit1 = checkSuit(card1)
		suit2 = checkSuit(card2)
		if suit1 == "S" do
			card1
		else
			if suit2 == "S" do
				card2
			else
				if suit1 == "H" do
					card1
				else
					if suit2 == "H" do
						card2
					else
						if suit1 == "D" do
							card1
						else
							if suit2 == "D" do
								card2
							else
								if suit1 == "C" do
									card1
								else
									card2
								end
							end
						end
					end
				end
			end
		end
	end

	def checkSuit(num) do
		if num < 14 do
			suit = "C"
		else
			if num < 27 do
				suit = "D"
			else
				if num < 40 do
					suit = "H"
				else
					suit = "S"
				end
			end
		end
	end

	def turnIntoHand(list, num) do
		#mod 12 it here, to preserve the suits for other functions
		hand_numbers = Enum.map(list, fn x -> rem(x, 13) end)
		#make 0's into 13's, 1's into 14s
		hand_numbers = orderer(hand_numbers, num)
	end
	def turnIntoHand(list, num, order) do
		#mod 12 it here, to preserve the suits for other functions
		hand_numbers = Enum.map(list, fn x -> rem(x, 13) end)
		#make 0's into 13's, 1's into 14s
		hand_numbers = orderer(hand_numbers, num)
		hand_numbers = Enum.sort(hand_numbers)
		hand_numbers = Enum.reverse(hand_numbers)
	end

	def orderer(list, num) do #turns kings from 0 to 13 and aces from 1 to 14
		if Enum.at(list, 0) < num do
			orderer(tl(list), [Enum.at(list, 0) + 13], num)
		else
			orderer(tl(list), [Enum.at(list, 0)], num)
		end
	end
	def orderer(list, newlist, num) do
		if list == [] do
			newlist
		else
			if Enum.at(list, 0) < num do
				orderer(tl(list), newlist ++ [Enum.at(list, 0) + 13], num)
			else
				orderer(tl(list), newlist ++ [Enum.at(list, 0)], num)
			end
		end
	end

	def turnHandIntoMap(full_hand, aces) do
		suit = checkSuit(hd(full_hand))
		real_nums = turnIntoHand(full_hand, aces)
		turnHandIntoMap(tl(full_hand), [card: %{num: hd(real_nums), suit: suit}], aces)
	end
	def turnHandIntoMap(full_hand, hand_map, aces) do
		if full_hand == [] do
			hand_map
		else
			suit = checkSuit(hd(full_hand))
			real_nums = turnIntoHand(full_hand, aces)
			turnHandIntoMap(tl(full_hand), hand_map ++ [card: %{num: hd(real_nums), suit: suit}], aces)	
		end

	end

	#gives the winning list its suit and prints it properly. 
	def printWinner(list) do
		suit_num_map = turnHandIntoMap(list, 1)
		Enum.sort(suit_num_map)
		IO.inspect suit_num_map
		#turn into a map {(num, suit)} -> Then sort it
		suit = suit_num_map[:card].suit
		num = suit_num_map[:card].num
		printWinner(tl(suit_num_map), [Integer.to_string(num) <> suit])
	end
	def printWinner(suit_num_map, winnerlist) do
		if suit_num_map == [] do
			IO.inspect winnerlist #shit. only works for 0-9 //TODO
		else
			suit = suit_num_map[:card].suit
			num = suit_num_map[:card].num
			printWinner(tl(suit_num_map), winnerlist ++ [Integer.to_string(num) <> suit])
		end
	end
end

#complete the tie breaker.
#finish weird ordering from printWinner from winner = Poker.deal [1, 2, 26, 15, 27, 28, 40, 41, 14, 52]

IO.puts "==================================="
#winner = Poker.deal [1, 2, 14, 15, 27, 28, 40, 41, 52, 26]
IO.puts "==================================="
#winner = Poker.deal [8, 1, 2, 14, 35, 6, 26, 19, 14, 10]
#IO.puts "==================================="
#winner = Poker.deal [8, 1, 2, 14, 35, 6, 26, 19, 14, 10]

#test for royal flush
#winner = Poker.deal [40, 14, 49, 23, 50, 24, 51, 25, 52, 26]
#test for straight flush
#winner = Poker.deal [48, 22, 49, 23, 50, 24, 51, 25, 52, 26]
#test for 4 of a kind
#winner = Poker.deal [1, 2, 26, 15, 27, 28, 40, 41, 14, 52]



#What I think is complete:
# the straight flush is totally done. all checking and comparing is I think, finished.
#royal flush is done


# flushes are done


