# Dvorak Cypher

A really stupid way to cypher your messages.
Just run the program, enter the required parameters, and send the result including the parameters that you entered so that the receiver can decipher it, or don't I'm not your dad.

## What is a dvorak Cypher

Dvorak is a keyboard layout intended to be a faster and more ergonomic alternative to the QWERTY/AZERTY layout.

Now imagine writing some phrase using a AZERTY keyboard layout, but actually hitting the keys in on a dvorak layout. What would be written would be nonsense. That is what this program is trying do, streamlining ineligibility.

## Why?

Yes

## Installing

The crates I use are rand and std.
You can simply:
`git clone` to clone the repository
`cargo install` to install all dependencies
Then `cargo run` to run
Or `cargo build` to compile then run the executable

## Parameters

The program will prompt the user with 4 inputs, apart form the first parameter the user does not need to know what the other parameters do, a random int can be inputted:

- `Enter a phrase to cypher:` phrase to be cyphered, without any new lines. Only the Latin alphabet and `, ; . '` characters are cyphered
- `Enter the number of times to cypher:` how many times the phrase will pass the cypher
- `Enter the shift value:` the number of times the keys will be shifted after each phrase cypher pass
- `Enter the start shift value:` the initial key shift
