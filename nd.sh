#!/bin/bash
if [ $# -eq 0 ]; then
    echo "Provide day number plz"
    exit 1
fi
# do not run same day twice
if [ -f "./data/data$1" ]; then
    echo "./data/data$1 exists."
    exit 1
fi
# create new empty data file
touch ./data/data$1
printf -v t1 %q "[[bin]]"
printf -v t2 %q "name = \"day$1\""
printf -v t3 %q "path = \"src/bin/day$1.rs\""
# add new bin to Cargo.toml
sed -i "/path = \"src\/bin\/dayx.rs/a \\\n$t1\n$t2\n$t3" Cargo.toml
# use dubble files
cp ./src/bin/dayx.rs ./src/bin/day$1.rs
cp ./src/mach/dayx.rs ./src/mach/day$1.rs
# replace old datapath
sed -i "s|./data/datax|./data/data$1|g" ./src/bin/day$1.rs
# replace
for file in ./src/bin/day$1.rs ./src/mach/day$1.rs
do
	sed -i "s|dayx|day$1|g" $file
    sed -i "s|Dayx|Day$1|g" $file
    sed -i "s|DayX|Day$1|g" $file
done
# double
for file in ./src/lib.rs ./src/mach/mod.rs
do
	#sed -i "s|dayx|day$1|g" $file
    #sed -i "s|Dayx|Day$1|g" $file
    #sed -i "s|DayX|Day$1|g" $file
done