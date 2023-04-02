# [Common Programming Concepts - Exercises](https://doc.rust-lang.org/nightly/book/ch03-05-control-flow.html#summary)

To practice with the concepts discussed in this chapter, try building programs to do the following:
- Convert temperatures between Fahrenheit and Celsius.
- Generate the nth Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

## Exercises
- [x] 🥶 Temp converter 🥵

This program converts temperatures between Fahrenheit and Celsius. The user is prompted for the input of a temperature value and its unit (C or F). The program reads the input, parses the temperature value and unit, and creates a Temperature enumeration based on it. The Temperature instance converts the temperature to the opposite unit by calling the `convert` method. Finally, the program prints the original and converted temperature.

Program flow example:
```bash
Enter the temperature value followed by its unit (C or F): 32F
32.00°F is 0.00°C
```

- [x] 🔢 Fibonacci 🔢

Calculates the nth Fibonacci number using the matrix exponentiation method. The Fibonacci struct provides a method `nth` that takes an integer `n` and returns the nth Fibonacci number as a `BigUint`. The program uses the `num_bigint` and `num_traits` crates to deal with large numbers. The main function reads the user's input, parses the integer `n` and calls the `nth` method of the `Fibonacci `struct to calculate the nth Fibonacci number. Finally, it prints the result.

Program flow example:
```bash
Enter the position (n) of the Fibonacci number: 777
Fibonacci(777) = 1081213530912648191985419587942084110095342850438593857649766278346130479286685742885693301250359913460718567974798268702550329302771992851392180275594318434818082
```

- [x] 🎄 Christmas Carols 🎄

It will print out words to 'The 12 Days of Christmas'. The main function iterates through the twelve days. For each day, it calls the `print_lyrics_for_day` function. This function prints a text for a given day, including its ordinal number and gifts received on that day. It also takes advantage of the repetition in the song by looping through the gifts for each day in reverse order.

Last day output:
```bash
On the twelfth day of Christmas,
my true love sent to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!
```