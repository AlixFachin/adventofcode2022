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

