# [Common Collections Summary - Exercises](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary)

Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:
- Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
- Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
- Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

## Exercises
- [x] 📊 Vector median and mode 📊

This program calculates the median and mode of a list of `i32` numbers supplied by the user. The `calculate_median` function arranges the numbers and calculates the median based on the length of the list. The `calculate_mode` function uses a hash map to count the occurrences of each number and finds the mode. The main function reads user input, parses it into a list of `i32` numbers, calls `calculate_median` and `calculate_mode`. It then prints the median and mode (or a message that all numbers occur equally often).

Program flow examples:
```bash
Enter numbers separated by spaces or commas:
1 3 4 5 8 10
Your numbers [1, 3, 4, 5, 8, 10]
Median = 4.5
Mode = All numbers occur the same number of times.
```

```bash
Enter numbers separated by spaces or commas:
1, 2, 3, 4, 4, 5, 5, 5, 6, 7, 7
Your numbers [1, 2, 3, 4, 4, 5, 5, 5, 6, 7, 7]
Median = 5
Mode = 5
```

```bash
1, 9,3, 4,     4,,,5, 5, 5, 6, 7,7,8, 9, ,9,9
Your numbers [1, 9, 3, 4, 4, 5, 5, 5, 6, 7, 7, 8, 9, 9, 9]
Median = 6
Mode = 9
```

- [x] 🐷 Pig latin 🐷

This program converts English text to Pig Latin. It processes each word in the input string based on its first character. If the word starts with a vowel, it adds "-hay" to the end of the word. If the word starts with a consonant, it moves the first character to the end and adds "ay". The program reads the user's input, converts the text to Pig Latin using the `convert_to_pig_latin` function, and then prints the original text and the Pig Latin text.

Program flow example:
```bash
Enter words separated by whitespace:
first apple

Original: first apple
Pig Latin: irst-fay apple-hay
```

- [x] 🎄 Department 🎄

Manages employees and their departments within a company. It allows users to `add`, `remove` and `move` employees between departments, as well as `list` employees within specific departments or across the entire company. The user interacts with the program through a text interface. The user enters commands such as **Add**, **Remove**, **List** and **Move**.

Commands in the flow example:
```bash
Add Alice to Engineering
Add Bob to Sales
List Engineering
List All
Move Alice from Engineering to Sales
List Sales
Remove Bob from Sales
List Sales
List Engineering
```

Program flow example:
```bash
Please enter a command (type 'exit' to quit): Add Alice to Engineering
Added Alice to Engineering department.
Please enter a command (type 'exit' to quit): Add Bob to Sales
Added Bob to Sales department.
Please enter a command (type 'exit' to quit): List Engineering
Employees in the Engineering department:
Alice
Please enter a command (type 'exit' to quit): List All
Employees in the Engineering department:
Alice

Employees in the Sales department:
Bob
Please enter a command (type 'exit' to quit): Move Alice from Engineering to Sales
Moved Alice from the Engineering department to the Sales department.
Please enter a command (type 'exit' to quit): List Sales
Employees in the Sales department:
Alice
Bob
Please enter a command (type 'exit' to quit): Remove Bob from Sales
Removed Bob from the Sales department.
Please enter a command (type 'exit' to quit): List Sales
Employees in the Sales department:
Alice
Please enter a command (type 'exit' to quit): List Engineering
No employees found in the Engineering department.
Please enter a command (type 'exit' to quit): exit
```