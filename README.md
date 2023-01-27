# brickstats - automated lego set analysis using the rebrickable database

## Features
* download set inventory by set number
* download list of all colors with details
* download list of all part categories with details
* plot correctly colored histogram of part list with category names as bins using gnuplot

## To dos
* remove spares from set inventory
* accept set numbers without '-1' or '-2'
* use color rgb values from initial inventory download
* add caching to color and category list to improve speed
* export histogram as png instead of opening gnuplot
* convert histogram to horizontal (work-around required)
* label histogram total amount per bin