#!/bin/bash
cd ~/Documents/Code/wonderfill
echo building...
cargo build 2>&1 | rg -i --multiline "(^error.*\n.*)|(aborting)|(warnings)"
echo done. copying assets...
cp assets target/debug -r
echo finished!
