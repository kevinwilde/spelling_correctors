# spelling_correctors

Rust implementations of Norvig spelling corrector and BK Tree spelling corrector

### Overview:

**Norvig corrector:**

Training Phase:
- Read words from training file and stores them in a hashmap.
- Each time a word is seen, it's frequency is incremented.

Correction Phase:
- For each word in input, if the word is in the hashmap, no correction is necessary.
- Otherwise, all possible words of edit distance 1* are checked for in the hashmap.
- If it finds any of these in the hashmap, it chooses the one with the highest frequency.
- If none are found in the hashmap, all words of edit distance 2 checked for in the hashmap.
- If it finds any of these in the hashmap, it chooses the one with the highest frequency.
- If still none are found, it does not make any suggestions for correcting the word.

*Edit distance: the number of edits it would take to turn one into the other, where an edit can be a deletion (remove one letter), a transposition (swap adjacent letters), an alteration (change one letter to another) or an insertion (add a letter).

**BK Tree corrector:**

Training Phase:
- Read words from a training file and construct a BK Tree from them.
- The first word becomes in the file is the root.
- Each node stores the word and its frequency.
- The edges are the [Levenshtein distance](http://planetcalc.com/1721/) between the adjacent nodes' words.

Correction Phase:
- For each word in input, traverse the BK tree to find all words within a Levenshtein distance of 2 from the word.
- Sort these words first by Levenshtein distance from the input word.
- If the word itself is found (ie. Levenshtein distance is 0), no correction is necessary.
- Otherwise, choose the word with the lowest Levenshtein distance and highest frequency.

### Behavior:

For the vast majority of misspellings, the Norvig corrector and the BK Tree corrector will produce the same suggested correction. However, the behavior of the BK Tree  corrector is slightly different from the Norvig corrector because of the use of Levenshtein distance rather than edit distance. In particular, transposing letters adds 2 to the Levenshtein distance, whereas it only adds 1 to the edit distance. Thus, in the case where the Norvig corrector suggests a correction of transposing two letters for an edit distance of 1, the BK tree corrector might suggest a different correction with a Levenshtein distance of 1. For example, when [this file](http://www.gutenberg.org/cache/epub/1342/pg1342.txt) is used as the training file, the BK Tree corrector will suggest that 'fera' be corrected to 'her'. The Norvig corrector, though, will suggest that 'fera' be corrected to 'fear'. In this case, the edit distance from 'fera' to 'fear' is 1 (transpose a and r) but the Levenshtein distance is 2. Therefore, the Norvig corrector chooses the most common word of edit distance 1 ('fear'), but the BK Tree corrector does not find any corrections of Levenshtein distance 1, so it chooses the most common word of Levenshtein distance 2 ('her').

### Performance:

Based on running the programs on a few different files, it appears that the	Norvig spelling corrector goes through the training phase faster, but is slower at the correcting misspelled worse (particularly when the misspelled word is relatively long and requires going to the second level of edits in order to find a correction). The BK Tree is slower in the training phase, but faster at making corrections for misspelled words.

This would make sense because in the training phase, the Norvig corrector is simply inserting into and updating a Hashmap, while the BK Tree corrector is constructing a tree. However, in the correction phase, the Norvig corrector must first generate all possible edits and then check for them in the hashmap, whereas the BK Tree corrector traverses the tree based on the word itself without having to generate any possible edits.
	
