# CHBS Password Strength Checker

A simple password strength checker using raw entropy values written in Rust.

Inspired by: https://github.com/wagslane/go-password-passwordvalidator and
this [XKCD](https://xkcd.com/936/)

## Benefits

* While uppercase, numbers, special characters, etc... all count towards the
  final score, none are *required*
* Does not contact any external API's
* Includes command line tool in addition to a library so it can be used in
  bash scripts


## ⚙️ Installation

cargo install chbs

Use in code::

```
use chbslib::get_entropy;

let test_1: String = String::from("boring");
let score: i16 = get_entropy(&test1);
println!("Score: {}", score);

>>> 1
```


## What is a good minimum value?

That's a judgement call. I would suggest something in the range of ``5`` to
``7`` or so.

## Caveats
Attackers commonly use passwords they have scraped from leaked data. Entropy
checks will NOT protect your users from using leaked passwords.

## How It Works

The password is stripped of repeating characters, palindrome portions, and
common password sequences like ``qwerty`` or ``asdfghjkl`` as well as the set
of the 10 most commonly used passwords (according to wikipedia). The remaining
length accounts for the initial score. Additional points are awarded for mixing
case, adding numbers, and adding special characters.

After that we do some maths which calculates the approximate total guesses and
reduces this to a integer score.
