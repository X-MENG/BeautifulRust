Introduction
Flower Field is a compassionate reimagining of the popular game Minesweeper. The object of the game is to find all the flowers in the garden using numeric hints that indicate how many flowers are directly adjacent (horizontally, vertically, diagonally) to a square. "Flower Field" shipped in regional versions of Microsoft Windows in Italy, Germany, South Korea, Japan and Taiwan.

Instructions
Your task is to add flower counts to empty squares in a completed Flower Field garden. The garden itself is a rectangle board composed of squares that are either empty (' ') or a flower ('*').

For each empty square, count the number of flowers adjacent to it (horizontally, vertically, diagonally). If the empty square has no adjacent flowers, leave it empty. Otherwise replace it with the count of adjacent flowers.

For example, you may receive a 5 x 4 board like this (empty spaces are represented here with the '·' character for display on screen):

·*·*·
··*··
··*··
·····
Which your code should transform into this:

1*3*1
13*31
·2*2·
·111·