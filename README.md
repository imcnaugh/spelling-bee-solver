# NYT Spelling Bee Solver

This is just a quick exercise in data structures. Pass this the chars in the daily NYT Spelling Bee puzzle and this will output a list of possible words.

The words will not all be valid words in the NYT puzzle, Currently I am pulling the word list off of the unix file `/usr/share/dict/words` but the official list the NYT uses is different.

But this will get you close.

## TODO
 - Pre-process the word list and save the processed words in a file to speed up execution.
 - Find a better word list closer to the real list NYT uses.