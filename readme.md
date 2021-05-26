# The Molybdenum Replacer

Recursive _search and replace_ CLI application.

## Installation

* Install [rust](https://www.rust-lang.org/learn/get-started)
* Clone the repo: `git clone https://github.com/gfannes/molybdenum`
* Build and install the app: `cargo install --path molybdenum`
* Verify success: `mo -h` should print its help and version

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

## Future Features

Following features might be added sooner or later:

* Improved performance
  * `mo` is currently single-threaded. To achieve [ripgrep](https://github.com/BurntSushi/ripgrep)-like performance, all CPU's are probably required.
  * When `-l` is used to only output filenames, `mo` can stop searching after the first match.
* Use a better name
  * Nobody can remember `molybdenum`, and it is very hard to type. `mo` is better, but difficult to search on the internet.
* Support for using regex capture groups during replacement
* Better detection of binary files
* Support for filtering against `.gitignore` files
* Rollback when writing replacement file fails
* Improved testing
  * More and better unit tests
  * Acceptance tests for all common use cases
  * Performance tests and some reporting around this
* Binary releases on [github](https://github.com/gfannes/molybdenum)
* Improved color scheme for people using a light background theme
* Support for file type sets
* Allow options without argument to be merged
* Allow input separator to be set explicitly. Now, this is hardcoded `0x0a`.

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

### Search for word in C/C++ source code

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
