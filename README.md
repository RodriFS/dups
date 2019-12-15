# dups
A rather unperformant duplicate files finder

## How to run:
```
cargo build
```

how to run:
```
cargo run -- [Options] [Path]
```

Options:
  -r --recursive: Finds files in subdirectories.
  -h --help: Prints out help

Path:
  The path of the folder you want to search the duplicates in.
  
  

Optionally you can output your results to a txt file:
```
cargo run -- -r ./Documents > output.txt
```