#!/bin/bash

Tree_orders=(2 5 10 15)
Buffer_sizes=(10 50 100 500)
Cache_sizes=(10 50 100 500)

for file in ./test_files/*
do
    for order in ${Tree_orders[@]} 
    do
        for buf_s in ${Buffer_sizes[@]} 
        do
        for cache_s in ${Cache_sizes[@]} 
            do
                file_name="$(basename $file)_${order}_${buf_s}_${cache_s}"
                cargo run "./test_out/${file_name}" $order n $buf_s $cache_s "./test_logs/${file_name}.csv" < $file  
            done
        done
    done
done
