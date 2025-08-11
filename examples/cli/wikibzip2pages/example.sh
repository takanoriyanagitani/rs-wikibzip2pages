#!/bin/sh

export BZ_FILENAME=~/Downloads/enwiki-20250801-pages-articles-multistream.xml.bz2

export BZ_OFFSET=569
export BZIP2_SIZE=704588

export BZ_OFFSET=704588
export BZIP2_SIZE=$(( 2293578 - 704588 ))

./wikibzip2pages | head -3
./wikibzip2pages | tail -3

time ./wikibzip2pages | wc -l
