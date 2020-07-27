#!/bin/bash

pwd=$(pwd)
for d in $(find $pwd | grep Cargo.toml | sed 's/Cargo.toml//g')
do
    cd $d
    echo "\n\n>>> Running cargo build in $d\n"
    
    cargo test --verbose --all
  
    if [ $? -ne 0 ]; then
    	exit -1
    fi    

    cd $pwd
done
