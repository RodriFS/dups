# dups
A rather unperformant duplicate files finder

## How to run:
```
cargo build
```

how to run:
```
./target/debug/dups [Options] [Path]
```

Options:
  -r --recursive: Finds files in subdirectories.

Path:
  The path of the folder you want to search the duplicates in.
  
  

Optionally you can output your results to a txt file:
```
./target/debug/dups -r ./Documents > output.txt

