# Dvorak Cypher

A really stupid way to cypher your messages.
Just run the programme, enter the required parameters, and send the result including the parameters that you entered so that the receiver can decipher it, or don't I'm not your dad.

## Why?

Yes

## Installing

There are no dependencies, so clone the repository and compile with the rust complier.

## Parameters

The programme will prompt the user with 4 inputs, apart form the first parameter the user does not need to know what the other parameters do, a random int can be inputted:

- `Enter a phrase to cypher:` phrase to be cyphered, without any new lines. Only the Latin alphabet and `, ; . '` characters are cyphered
- `Enter the number of times to cypher:` how many times the phrase will pass the cypher
- `Enter the shift value:` the number of times the keys will be shifted after each phrase cypher pass
- `Enter the start shift value:` the initial key shift
