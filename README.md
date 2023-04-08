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
* label histogram total amount per bin
* improve performance by reducing loops over data vectors (e.g. part category id and part category names)
* show transparent colors as transparent
* mark prints with (e.g. with dashes)
* increase number of parts per page
* add total amount of parts to histogram (official value)
* improve font and font size of histogram
* refactoring
* add support for rebrickable csv files