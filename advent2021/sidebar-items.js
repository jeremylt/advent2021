initSidebarItems({"constant":[["REPETITIONS",""]],"fn":[["main",""]],"mod":[["day01","Day 1: This solution uses a sliding window of 2 values for the first part. In the second part we expand the window to 4 values as `[a, b, c].sum() - [b, c, d].sum() == a - d`, so we only need to consider the first and last values to determine if the three value windows increase or decrease."],["day02","Day 2: The main part of the effort for this solution is reading the input into a correct structure. From there following the computations are straightforward. In the second part, I leveraged the fact that one of the two parts of the Direction is zero to avoid branching logic, making the code take half as long as the branching version of the `fold`."],["day03","Day 3: I’m not entirely happy with this solution because it feel very cumbersome. The first part is messy because it is faster to count all bytes at the same time. For the second part, partitioning seems the most straightforward approach, but I have a lot of code duplication."],["day04","Day 4: I encoded the board into row and column indices for each possible ball. I took a huge performance penalty for using `method(self)` instead of `method(&self)` at first."],["day05","Day 5: This solution process highlighted two things. First, branching logic can be expensive. My original solution to part 1 was 3x slower than the solution to part 2, but the only difference was checking if `dx` or `dy` was zero. Second, this difference reduced greatly when I switched to a narrower data type, using `i/u16` instead of `i/u32` and using a `u8` for my `grid`s representing the vents."],["day06","Day 6: This was a reasonably straightforward problem. The key is to keep track of number of fish of each age instead of modeling each fish separately. Of note, manually rotating seems to be faster than `rotate_left(1)` in this particular problem. Also of note, there is no need to actually rotate all of the fish if you rotate the meaning of each index."],["load","Load: This module has the code for loading input from file to a string buffer"],["output","Output: This module collects some of my `println!` boilerplate between the days."],["prelude",""]],"struct":[["Error",""],["RunData",""],["Timing",""]],"type":[["Result",""]]});