# Summary
This is a repo containing my solutions for the [Advent of Code 2022](https://adventofcode.com/2022).

This year I will use the challenges as an opportunity to learn `Rust`!

## Repo structure
* Each day challenge has its own subfolder `src/day1`, `src/day2`, ... 📅
* Inside each day folder is a `mod.rs` file which imports two modules, `solve_1.rs` and `solve_2.rs` which corresponds to this day's questions (one file per question). ❓
* The main function `src/main.rs` will import all those modules
* To run the function corresponding to the day `n`, you just need to run `cargo run -- run n` 💥
* To start a challenge for a new day, you can help with `cargo run -- add n` where Rust will create a new subfolder, copy template files into this subfolder. 😅

## Days Summary

### Day 1
Day 1 starts gently. Just need to read a file, parse the content and group into an array.
When sorting the array in descending order, we can access the maximum easily (question 1), and the three biggest values (question 2)

### Day 2
Rock-Paper-Scissors!
The idea is reading a list of games from the input file and evaluate the result of those games.
I tried to use a `Struct` to contain the game outcome, and a `Enum` for the different combinations. I am not sure if the code is more readable this way - I'd welcome any suggestiong 😉
For second question of the day, the letters change meaning and the function has actually be reversed (i.e. you input the outcome of the game and need to guess the player's move, instead of inputting player's move and compute the game outcome). Using roughly the same struct works.

### Day 3
Complexity of the algorithms goes up a notch, as we have some `n^2` or `n^3` potentially.
I guess I am glad that Rust is a low-level language, overall everything was quite smooth.
* First question is to find the character common in two sub-strings. (Nice to learn Rust string API! 👍)
* Second question is to find a character common in three strings. I guess a small difficulty in grouping the input strings three by three.
Looking forward to day 4!

### Day 4
Testing the new cargo setup. It's great! (Without cargo setup I cannot get VSCode autocomplete and proper syntax highlighting).
Today the file contains interval descriptions, and the goal is to count intervals included into each other (question 1) or overlapping (question 2).
* I used regex to parse the description strings `a-b,c-d` which gives me the interval descriptions.
* Intervals included into each other is pretty easy test. Overlapping is a touch more complicated, so I test if the intervals are completely disjoint, which ends up to the same thing.
* To note stupid mistakes today: not looking at the proper input file 🤦 and the regex not parsing the last line of such input file 🤦🤦

### Day 5
Ouch. Today a big challenge with `Rust` and my understanding of mutable references and borrowing.
It's late and after a big day of work I find my code rather dirty (but it works, so 🤷)
The problem is about a crane picking up crates and moving them to other stacks, and it reminded me of the Towers of Hanoi.
* In any case, I chose to model each stack with a string. Rust offers the string methods `pop` and `push`, which is all I need.
* I just need to parse each command, pop the proper amount from the proper string and push it back on the good string afterwards.
* For question 2, I need to keep the same order - so I use a temporary stack to reverse order, then reverse order again which finishes with me having the proper order. 👍
I still have trouble understanding `mut` and `borrowing`. There must be a more elegant way to do it than what I ended up with.

### Day 6
Today was easier than expected. The idea is to extract a substring at various indexes from the string and check if there are any duplicates.
Luckily Rust knows how to extract a substring pretty easily, and the extracted string is very short (4 and 14 characters), so I could allow to have a "heavy but quick to write" algorithm to detect duplicates. (Basically I transform the substring into an array, sort it, and then check if two consecutive characters are equal).
Looking forward to tomorrow's challenge! 💪

### Day 7
Ouch. I guess this is part of learning Rust. I think it is called "fighting with the borrower". After banging my head against the wall for two days I still don't manage to get anything close to compiling. But I'm learning stuff!
I'll skip for now
_Edit later..._
After four days of struggle I managed to solve those questions at last!
The issue for me was not the algorithm per se, but how to build a structure which would compile in Rust!
After refactoring my code 5 times, I ended up using the following two principles:
* Encapsulate as much as possible the use of variables. Another way to say it, try to avoid as much as possible to propagate mutable references to other objects. If one Struct can do everything internally, it will save a lot of headaches!
* For a tree, instead of storing an actual pointer or reference to a subtree, it is better to have somewhere a big vector with all the sub-trees (central ownership of data, no need for borrowing) and store in the tree structure an integer `usize` which points to the actual sub-tree.

### Day 8
Relatively easy day. `Rust` is so quick that the brutal brute force algorithm works in a snap. So why complicating things? This is pretty basic coding, but I still manage to have a bug in the formula. 🤦 Thankfully I can easily make `unit tests` in Rust to test each basic functions with basic cases, and I found the error pretty quickly. One day where Rust really shines!

### Day 9
Pretty interesting problem today. Didn't find anything extremely difficult. Having a `Option` and a `match` in Rust makes the branching readable (instead of chained `if ... else`).

### Day 10
Using regular expressions to parse the input. The first question seems a bit pointless, but it's a rehearsal for the second one. The algorithm is not excessively complex, but the problem in itself is mind-blowing. How do long did it take to create this question right so that the player input is correct? I am really amazed... 🙇

### Day 11
Getting a bit hairy...🙈
The first question of the day was not excessively complicated but long to understand.
I chose not to parse the input text - it was small enough to hard-code every parameter manually in a `init()` function.
Learning of the day in Rust: As you cannot get a `Struct` reference with `mut` of only some of the fields, it is better to encapsulate all the immutable data inside one Struct, and all the mutable data inside another Struct. This lowers the risk of having to borrow an immutable reference before a mutable one, which is refused by the compiler.

### Day 12
This was *painful*.
I don't really want to use external crates to implement tree traversal, and I got stuck for a few days trying to understand where my algorithm was wrong.
After four days of not getting it, I must admit I googled the answer and coded it.
The solution is to use a queue (FIFO) to browse the tree, so that all the squares at equal distance from start are covered at the same time. I used a recursive algorithm (depth-first I guess) which makes it very complicated to manage the flags of whether a square has been already visited or not. (And makes the debugging really complicated as the information you can print is always stacked and tough to read).
On the positive side, I struggled with the borrower much less than previous days. 👍

### Day 13
That was painful as well.
Luckily I didn't spend too much time on the parsing (thank you `sarde_json`!) but I spent too much time trying to find a bug on the algorithm.
I didn't know how to re-create a `sarde` Value, so I thought I would be clever and I skipped one case in the recursion, going directly through the leaf case.
After three days of searching, I looked at how to build a `Value`, tried it - and got the right answer. Going back to my previous function, I made the code quit when it found an error, and it turns out I forgot to take into consideration a pretty important case! 🤦
The positive things are:
* I learned how to use `sarde`
* I got better at how to use `Options`
* I learned how to sort a Vector with a custom comparison operation. Yay!

### Day 14
That was fun.
The nice part was looking into the `termion` crate to make an animation of the puzzle.
Most of the time was spent making the animation nice, I don't think I had any bugs in the code. (Although the animation is still flickery...)

### Day 15
I begin to feel the fatigue of Advent of Code, given that so far I spent 100% of my time outside of work (and outside of household chores) doing AoC. I needed to take a break, so this question took more time than what the actual algorithm complexity. Ouch! 😫
The question is apart from that pretty simple. Question 1 is mostly a parsing question, figuring out what this all means.
Question 2 is a bit more difficult, brute force being impossible to compute.
I solved this by computing for each line the intervals of intersection between the "circles" around each sensor and the current line, and then getting the union of those intervals. For all the lines but the question solution, those intervals will merge into something at least bigger than [0, 4E6] so not too difficult to follow