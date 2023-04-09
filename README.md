# brickstats - automated (lego(R)) set analysis using the rebrickable database and gnuplot

## Features
* load inventory
    * official sets from rebrickable.com
    * any part list in rebrickable .csv format
* plot correctly colored histogram of part list with category names as bins using gnuplot

## Prerequesites
* Linux machine or Windows 11 with WSL (required for gnuplot)
* gnuplot
* access to the internet (rebrickable.com)
* rust installation for building (install with rustup)
* rebrickable api token

## How to use
* download this repository and cd into it
* run ```cargo build --release```
* create a secrets directory ```mkdir secrets```
* create an api token file ```touch secrets/api_token.txt```
* put your rebrickable api token inside the file
* run queries

## Parameters
* -f --file  [filename] # rebrickable .csv file to parse
* -s --set [set number] # set number in 12345-1 or 12345 format to fetch from rebrickable.com
* -o --output [output type] # set output to "png" to create a png file instead of showing the diagram in gnuplot (no other output types supported yet)

## Examples
1. plot diagram for set 40567-1 as png
```cargo run --release -- -s 40567 -o png```

2. show diagram for file input/example.csv in gnuplot
```cargo run --release -- -f "input/example.csv"```

## To dos
* label histogram total amount per bin
* improve performance by reducing loops over data vectors (e.g. part category id and part category names)
* show transparent colors as transparent
* mark prints with (e.g. with dashes)
* refactoring
* add word clouds
* add stats summary in top right corner
