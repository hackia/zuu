---
title: zuu(1)
section: 1
date: 2024-08-04
manual: Zuu Manual
source: Zuu Project
---

# NAME

zuu - A program to check your code quality before all commit.

# SYNOPSIS

zuu [-m|--mode] mode

# Description

Think of zuu as your Rust project's personal hygiene routine.

Just like you wouldn't want to go out with messy hair or dirty clothes, you also don't want your Rust code to be sloppy
or riddled with potential problems. zuu helps you keep your code clean and tidy so it's ready to show off to the world.

Here's the gist of what it does:

Checks Your Code's Health: zuu runs a series of automated checks on your Rust code.

It's like a thorough medical checkup, making sure everything is in order.

Spotting Errors and Bad Habits: It looks for obvious errors that would prevent your code from even compiling, as well as
more subtle issues that might cause problems down the line (think of it as catching those bad coding habits before they
become a serious problem).

Enforcing Good Style: zuu also makes sure your code is well-formatted and follows the accepted Rust style guidelines.

This is like making sure your code is neatly dressed and presentable.

Customizable Strictness: You can choose how strict you want zuu to be. There are different levels of checks, from a
basic once-over to an ultra-thorough examination.

In simpler terms: zuu is like a combination of a spell checker, a grammar checker, and a style guide for your Rust code.
It helps you write code that's not only functional but also clean, consistent, and easy to understand.

So, why would you use it?

Avoid Embarrassing Mistakes: zuu helps you catch those silly errors that you might miss when you're focused on the
bigger picture.

Write Better Code: By enforcing good practices and style, zuu encourages you to write cleaner, more maintainable code.

Work Well with Others: When your code is consistent and well-formatted, it's easier for other developers to understand
and collaborate on your project.

Peace of Mind: Knowing that your code has passed zuu's rigorous checks gives you confidence that it's solid and ready
for prime time.

# OPTIONS

`ultra`: Every possible check and lint is turned on.

Your code will be scrutinized for even the tiniest inconsistencies
or potential issues. 

It's great for ensuring the highest possible code quality, but it can also be the most
time-consuming and might lead to more compilation errors that you'll need to address.

`high`: A comprehensive set of checks and lints are enabled, focusing on correctness, style, and potential problems.

This is a good balance for most projects, ensuring your code is clean and well-structured without being overly pedantic.

`medium`: A more relaxed set of checks, focusing on the most essential aspects of code quality. 

This is a good option if you want to catch major issues without getting bogged down in minor stylistic details.

`low`: Only the most basic checks are enabled.


This is suitable for quick checks or for projects where you're less
concerned about strict code quality enforcement.

Which one should you choose?

It depends on your project's needs and your team's preferences.

If you're working on a critical project where code quality is paramount, or if you have a team of experienced Rust
developers, "high" or even "ultra" might be the way to go.

If you're working on a smaller project or if you're new to Rust, "medium" or "low" might be a better starting point. You
can always increase the strictness later as you become more comfortable with Rust's linting tools.

Remember, the goal is to find a balance that works for you and your team. zuu is a tool to help you write better code,
not to hinder your progress.

# EXAMPLES

zuu

# SEE ALSO

zazen(1), teams(1)

