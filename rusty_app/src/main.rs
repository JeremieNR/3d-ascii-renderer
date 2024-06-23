fn main() {
    // 30 times a second (30 fps) you will print (literally print to console) 230 by 50 cells filled with ascii characters.
    // when modeling the width and height of the ascii screen, use the approximate human eye aspect ratio of 5:4 (where 5 represents width, and 4 represents height)
    // 3 things you need to figure out:
    //      - How to model the cube spinning (use linear algebra to know what to show)
    //      - How to print out the math (it's basically a leetcode problem, generate an array for each row and fill the rest of the cells with space characters)
    //      - Which Ascii character to use for each pixel (there should already be the answer to this online)
    // You could run into issues in terms of the ascii ratio for the characters which is 8 by 12, but maybe you can find a way around it by making the width and height of the screen different.

    for _i in 0..=50 {
        for _x in 0..=230 {
            print!("/");
        }
        print!("\n");
    }
}