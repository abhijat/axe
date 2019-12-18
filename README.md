###### What this tool does

It splits the JSON data set available from [PushShift](https://pushshift.io/) into smaller JSON files.


At this time, the data can be split by the following keys:

* Subreddit (subreddit)
* Author (author)
* Day of month (day)
* Month (month)
* Day of year (day-of-year)
* Day of week (day-of-week)

When the data is split, a JSON file is created for each unique key, so if the split is on subreddit, a JSON file
is created per subreddit.

###### Example Usage

* Build the code
```shell script
~/dev/rust/axe  (master) 
 abhijat $ cargo build --release
```

* Run the code
```shell script
~/dev/rust/axe  (master) 
 abhijat $ cargo run -- --input-path ~/Downloads/R --output-prefix ~/tmp/data-by-sub --split-on subreddit
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/axe --input-path /home/abhijat/Downloads/R --output-prefix /home/abhijat/tmp/data-by-sub --split-on subreddit`
...
```

The files will be present in `~/tmp/data-by-sub` after the above run is complete.


###### Help

```shell script
~/dev/rust/axe  (master) 
 abhijat $ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/axe --help`
axe 0.1.0
A utility to split a reddit dataset into individual JSON files

USAGE:
    axe --input-path <input-path> --output-prefix <output-prefix> --split-on <split-on>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input-path <input-path>          The path to the data set
    -o, --output-prefix <output-prefix>    The parent directory where output JSON files will be written
    -s, --split-on <split-on>              The attribute to split the data set on

```