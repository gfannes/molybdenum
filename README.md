# The Molybdenum Replacer

Recursive, line-based _search and replace_ CLI application.

## Installation

### Directly via cargo

* Install [rust](https://www.rust-lang.org/learn/get-started), this should also install [cargo](https://doc.rust-lang.org/cargo/)
* Run `cargo install molybdenum`
* Verify success: `mo -h` should print its help and version

### From source code

* Install [rust](https://www.rust-lang.org/learn/get-started), this should also install [cargo](https://doc.rust-lang.org/cargo/)
* Clone the repo: `git clone https://github.com/gfannes/molybdenum`
* Build and install the app: `cargo install --path molybdenum`
* Verify success: `mo -h` should print its help and version

## Basic usage

Following commands demonstrate how `mo` can be used to accomplish different tasks:

* Create list of filenames, only filtering against the pathname itself:
  * `mo`: When no search pattern is specified, only the filenames are listed
  * `mo -l`: Explicitly ask to output only the filenames
  * `mo -C FOLDER`: Use _FOLDER_ as root for searching
  * `mo -e hpp -e cpp`: Only take files with `hpp` and `cpp` extension into account
  * `mo -f PART`: Keep filenames that match against _PART_
  * `mo -F PART`: Keep filenames that do not match against _PART_
  * `mo -0`: Use `0x00` to separate filenames. This is handy when using the output with `xargs`.
  * `mo -u -U -a`: Take hidden files, folders and binary files into account as well
* Search for a given regex pattern:
  * `mo PATTERN`: Search for _PATTERN_ in files recursively
  * `mo -p PATTERN`: Search for _PATTERN_ in files recursively
  * `mo -w PATTERN`: Search for _PATTERN_ in files recursively, adding _word-boundary_ constraints arround _PATTERN_
  * `mo -s PATTERN`: Search for _PATTERN_, _case-sensitive_
  * `mo -B 10 -A 10 PATTERN`: Output a context of 10 additional lines _before_ and _after_ each match
* Replace matches with a given STRING:
  * `mo needle -w -r naald -n`: _Simulate_ the replacement of the the word `needle` with the Dutch word `naald`
  * `mo needle -w -r naald`: _Really_ replace the word `needle` with the Dutch word `naald`
* Combining with `xargs`
  * `mo -l -C FOLDER -0 | xargs -0 -r mo -i PATTERN`: Note the `-i` option to ensure `mo` will search in files and not Stdin. In addition, the `xargs -r` option should be set to ensure nothing will run if no filepaths are produced.

Next to this, `mo` detects if input comes from a console or redirection, and will act accordingly, as well as for its output: `mo` can be used to report or make replacements in a piped stream as well.

## Interactive file selection

Following `bash` functions allows you to _open a file (o)_ or _change to a folder (c)_ based on the fuzzy search functionality of [fzf](https://github.com/junegunn/fzf). You can pass them any argument that `mo` accepts, making them handy interactive tools. They rely on [bat](https://github.com/sharkdp/bat) to provide a preview, and [nvr](https://github.com/mhinz/neovim-remote) to open the selected file in a new or already running instance of [neovim](http://neovim.io/), and [zoxide](https://github.com/ajeetdsouza/zoxide) to register and track your most popular folders.

```
# Open file using `bat` as preview
o() {
    mo -l $* | fzf --multi --preview 'bat --style=numbers --color=always --line-range :500 {}' --preview-window 'right:60%' | xargs -I % nvr --remote-tab %
}

# Open file using `mo` as preview
s() {
    export all_args="$*"
    mo -l $* | fzf --multi --preview 'mo -c -i ${all_args} -C {}' --preview-window 'right:60%' | xargs -I % nvr --remote-tab %
}

# Change to dir using `z`
c() {
    z `mo -L $* | fzf`
}
```

## Yet another tool?

Powerful _search_ can be found without problems, eg, [grep](https://man7.org/linux/man-pages/man1/grep.1.html), [ack](https://beyondgrep.com/), [ag](https://github.com/ggreer/the_silver_searcher), [ripgrep](https://github.com/BurntSushi/ripgrep) or [broot](https://github.com/Canop/broot).

Tools for _replacing_ recursively in a folder are more difficult to find, although some exist: [fart-it](https://github.com/lionello/fart-it). Typically, people use a combination of searching, [xargs](https://www.thegeekstuff.com/2013/12/xargs-examples/) and a replacement tool like [sed](https://www.grymoire.com/Unix/Sed.html) or [rpl](http://rpl.sourceforge.net/).

I use code searching a lot to investigate a large source code base before attempting a replace. Even with 100k files, search is fast and fairly easy. Recursively _replacing_ text is much more dangerous, especially if it requires the combination of several less frequently used tools; it's difficult to remember a `search-xargs-replace` combination if not used on a daily basis. On top of this, the `search` tool used to filter the set of files and perform a dry-run, is _not per-se_ using the _same search query_ as the `replace` tool. After all, these are different tools. It would be better if a single tool could be used for every-day searching _and_ replacing. This is exactly what _The Molybdenum Replacer_ intends to achieve.

In addition, I find it difficult to use `ag` and `rg` to filter against the file path _and_ search for content _in_ the remaining files; the `-g` option for `ag` and `rg` is confusing in my opinion. As such, I would expect `ag -g /auro/ -w test` to search for the word `test` in all files that have `/auro/` in their path, but that is not what actually happens. It filters with `/auro/` _and_ `test` against the filename (or something that looks like it).

The real reason, of course, is that I had some free time and was looking for a nice project/excuse to learn [rust](https://www.rust-lang.org/).

## Implemented Features

Following features are implemented and usable in the current version:

* Recursive _search_ in a nested folder structure, line-based
  * Fast enough to make it usable. My impression is that it's performance is between [ag](https://github.com/ggreer/the_silver_searcher) and [ripgrep](https://github.com/BurntSushi/ripgrep).
  * Output clear enough for a human to understand when output is a console
* Recursive _replacement_ in a nested folder structure
* _Search_ and _replacement_ from an input stream
  * Output all input when output is redirected
* Support for non-UTF8 filename and content
* Crude detection of binary files
* Flexible specification of search root and pattern
  * Allow search root(s) on top of pattern
  * Allow dashed options after the search root and pattern
* Improved xargs integration
  * Support searching a single file when specified as root
  * Support for overriding the auto-detected stdin-redirection/tty-console detection
* Support for multiple search roots
* Support for filtering against `.gitignore` files
* Support for listing folder names using the `-L` option

## Future Features

Following features might be added sooner or later:

* Do not accept PATTERN and PATH with a leading `-`
* Do not fail when some folders have insufficient permissions
* By default, do not replace: invert interpretation fo `-n` (and use a different name)
* Support for arguments without a _shorthand_ notation
* Support for displaying a content with a single argument, preferably `-c`
* Support for file type sets
* Allow zero-argument options to be merged: `mo -ws test`
* Allow input separator to be set explicitly. Now, this is hardcoded `0x0a`.
* Improved performance
  * `mo` is currently single-threaded. To achieve [ripgrep](https://github.com/BurntSushi/ripgrep)-like performance, all CPU's are probably required.
  * When `-l` is used to only output filenames, `mo` can stop searching after the first match.
* Report a single line per match when redirected output is detected
  * Allows for counting matches with `wc`
* Support for omitting the line number of a match
* Support for inverting the matches per line
* Use a better name, I just picked something that was still available.
  * Nobody can remember `molybdenum`, and it is very hard to type. `mo` is better, but difficult to search on the internet.
* Support for using regex capture groups during replacement
* Better detection of binary files
* Rollback when writing replacement file fails
* Improved testing
  * More and better unit tests
  * Acceptance tests for all common use cases
  * Performance tests and some reporting around this
* Binary releases on [github](https://github.com/gfannes/molybdenum)
* Improved color scheme for people using a light background theme
* Add changelog for versions before v0.1.6.

## Performance

### Count all files in a folder

```
Scenario: count all files
  Running `mo -l -u -U -a | wc`
    Elapsed time: 0:00.29
    Output:  184675  187146 16959297

  Running `rg --files -uu -a | wc`
    Elapsed time: 0:00.15
    Output:  184675  187146 16589947

  Running `ag -l -uu -a | wc`
    Elapsed time: 0:00.66
    Output:  184675  187146 16589947
```

### Search for word in C++ source code

```
Scenario: search for word `test` in .cpp files in subfolder `core` where path contains /auro/
  Running `mo -C core -e cpp -f /auro/ -w -p test -l | wc`
    Elapsed time: 0:00.03
    Output:     165     165    9008

  Running `rg -t cpp --files core | rg /auro/ | rg '.cpp$' | tr '\n' '\0' | xargs -0 rg -i -w test -l | wc`
    Elapsed time: 0:00.01
    Output:     165     165    9008

  Running `ag --cpp -l . core | ag /auro/ | ag '.cpp$' | tr '\n' '\0' | xargs -0 ag -w test -l | wc`
    Elapsed time: 0:00.05
    Output:     165     165    9008

```

I don't know how I can accomplish this scenario with `ag` and `rg` in a single command without relying on `xargs` and `tr`.

## Changelog

### v0.1.6

* When stdout is non-TTY, a more compact and useful output format is used
* Added `.out` extension to the list of binary files
