# Dungeon Crawler Part Deux

This is an adaptation of the Rust program found in my [dungeoncrawl](https://github.com/jdjaustin/dungeoncrawl) repository; I coded both by following along in [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) by Herbert Wolverson.

This variation of the dungeon crawler uses [Legion](https://github.com/amethyst/legion), a high performance Entity Component System (ECS) found for free in the Rust crates library. This implementation also utilizes "fearless concurrency" to make the game multi-threaded.

I also added the ability to use the W, A, S, and D keys in lieu of the arrow keys for users who have keyboards without built-in arrow keys.

This is a work in progress, and more documentation will be added as I make progress on the code.