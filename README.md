# spelling_correctors

Rust implementations of Norvig spelling corrector and BK Tree spelling corrector

**Overview:**

Norvig corrector:  
	Training Phase:  
		Read words from training file and stores them in a hashmap.  
		Each time a word is seen, it's frequency is incremented.  
	Correction Phase:  
		For each word in input, if the word is in the hashmap, no correction is necessary.  
		Otherwise, all possible words of edit distance 1* are checked for in the hashmap.  
		If it finds any of these in the hashmap, it chooses the one with the highest frequency.  
		If none are found in the hashmap, all words of edit distance 2 checked for in the hashmap.  
		If it finds any of these in the hashmap, it chooses the one with the highest frequency.  
		If still none are found, it does not make any suggestions for correcting the word.  
	*Edit distance: the number of edits it would take to turn one into the other, where an edit can be a deletion (remove one letter), a transposition (swap adjacent letters), an alteration (change one letter to another) or an insertion (add a letter).  

BK Tree corrector:  
	Training Phase:  
		Read words from a training file and construct a BK Tree from them.  
		The first word becomes in the file is the root.  
		Each node stores the word and its frequency.  
		The edges are the Levenshtein distance between the adjacent nodes' words.  
	Correction Phase:   
		For each word in input, traverse the BK tree to find all words within a Levenshtein distance of 2 from the word.  
		Sort these words first by Levenshtein distance from the input word.  
		If the word itself is found (ie. Levenshtein distance is 0), no correction is necessary.  
		Otherwise, choose the word with the lowest Levenshtein distance and highest frequency.  


**Performance:**

Based on running the programs on a few different files, it appears that the	Norvig spelling corrector goes through the training phase faster, but is slower at the correcting misspelled worse (particularly when the misspelled word is relatively long and requires going to the second level of edits in order to find a correction). The BK Tree is slower in the training phase, but faster at making corrections for misspelled words.  
This would make sense because in the training phase, the Norvig corrector is simply inserting into and updating a Hashmap, while the BK Tree corrector is constructing a tree. However, in the correction phase, the Norvig corrector must first generate all possible edits and then check for them in the hashmap, whereas the BK Tree corrector traverses the tree based on the word itself without having to generate any possible edits.
	