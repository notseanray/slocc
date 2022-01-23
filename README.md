<h3 align="center">
	<br>
	slocc
	<br>
</h3>

<p align="center">simple-significant line of code counter</p>

<p align="center">
	<a href="./LICENSE"><img src="https://img.shields.io/badge/license-GPL%20v3.0-blue.svg"></a>
</p>

slocc is a simple and fast line counter, this tool is not very useful but I wanted it's functionality for testing random directories or files

`cat x.txt | wc -l` does a very similar job, however, this supports directories and counting semicolons

in the future I'll add a more accurate way to determine how many signifigant lines there are, and truly the line count doesn't matter much in the first place

#### features
* ignore certain prefixes and suffixes
* specify file extensions
* specify operating on hidden files
* cli through clap
* fancy colored text!

#### installation

it is recommended you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed

```
$ git clone git@github.com:NotCreative21/slocc.git
$ cd slocc
$ ./install.sh
$ slocc --help
```
