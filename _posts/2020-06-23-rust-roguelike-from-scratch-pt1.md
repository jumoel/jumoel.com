---
title: A roguelike in Rust from scratch, part 1
categories:
  - rust
  - gamedev
---

Two tings have collided. Me wanting to learn Rust and the [r/roguelikedev summer tutorial follow-along](https://old.reddit.com/r/roguelikedev/comments/grccvt/roguelikedev_does_the_complete_roguelike_tutorial/). My main motivation (for now) is learning Rust, so I’ve set a restriction for myself: I won’t be using external dependencies for anything that can feasibly be done manually. To keep things manageable, that means that my rogue like will be old school and take place in a plain ol' terminal.

This is the first part, covering [Part 0](http://rogueliketutorials.com/tutorials/tcod/part-0/) and [Part 1](http://rogueliketutorials.com/tutorials/tcod/part-1/) or the Roguelike Tutorial.

# Setting up the project

This was the easy part. Install [rustup](https://rustup.rs/) and run:

```sh
$ rustup-init
<snip>
$ cargo new roguelike-rs
     Created binary (application) `roguelike-rs` package
$ cd roguelike-rs
$ cargo run
   Compiling roguelike-rs v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/roguelike-rs`
Hello, world!
```

# Drawing a canvas

Since I'm restricting the project to a pure terminal output, drawing a canvas is a lot of `print!` calls:

```rs
struct Player {
    x: i32,
    y: i32,
}

fn main() {
    let width = 40;
    let height = 25;

    let p = Player {
        x: width / 2,
        y: height / 2,
    };

    for y in 0..height {
        for x in 0..width {
            if p.x == x && p.y == y {
                print!("@");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
```

For now, the only state I'm tracking is the player position. To begin with, they are in the middle of the map.

This is what `cargo run` looks like:

```
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
....................@...................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
........................................
```

Not the most exciting stuff, so I'd like to jazz it up a little with borders, which can be done with some `if`s and some [line drawing characters]():

```rs
for y in -1..(height + 1) {
    for x in -1..(width + 1) {
        if x == -1 && y == -1 {
            print!("┌");
        } else if x == width && y == -1 {
            print!("┐")
        } else if x == -1 && y == height {
            print!("└")
        } else if x == width && y == height {
            print!("┘")
        } else if x == -1 || x == width {
            print!("│")
        } else if y == -1 || y == height {
            print!("─")
        } else if x == p.x && y == p.y {
            print!("@")
        } else {
            print!(".")
        }
    }
    print!("\n");
}
```

Which results in:

```
┌────────────────────────────────────────┐
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│....................@...................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
│........................................│
└────────────────────────────────────────┘
```

Much better!

To be able to handle input, the rendering is put in a loop. At the beginning of the loop, the console is cleared to make ready for a new cycle.

```rs
loop {
  // Position the cursor at (1,1)
  print!("\x1B[1;1H");
  // Clear the screen
  print!("\x1B[2J");

  // for { ... }
}
```

This loops indefinetely, so to be able to exit the game without typing `Ctrl-C`, I wait for user input and quit if it matches `q`:

```rs
loop {
  // ...snip
  println!("Command [q]: ");

  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();

  match s.as_str().trim() {
      "q" => break,
      _ => (),
  }
}
```

Typing `q<Enter>` will now quit the game. The same construct can be used to move the player character around:

```rs
loop {
  println!("Command [h/j/k/l/q]: ");

  // ...snip

  match s.as_str().trim() {
      "q" => break,
      "h" => p.x -= 1,
      "l" => p.x += 1,
      "j" => p.y += 1,
      "k" => p.y -= 1,
      _ => (),
  }
}
```

I've used the Vim movement keys because the game can't read arrow keys and such because the terminal is in ["cooked" mode](https://en.wikipedia.org/wiki/Terminal_mode).

While entering "raw" mode can certainly be done, achieving it requires interfacing with system calls and how it's done differs a lot based on platforms. Since I'd like this game to also be playable on machines other than my own, this is a point where an external dependency feels valid. I won't be using a crate for making text UI's, but "just" something to make `stdout` manageable. I decided upon [`crossterm`](https://crates.io/crates/crossterm). Converting the code to use `crossterm` will have to wait a bit.
