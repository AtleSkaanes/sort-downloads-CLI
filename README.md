# Sort downloads CLI

A simple CLI tool that can sort and delete files in your downloads directory.

## Compatability

Should work on all the big operating system, aka Windows, Linux, & Mac.
But I have only tested it on Windows.

## Installation

Download the executable, and put it somewhere on your computer.
For easier use, save the path to the executable in your paths, so you can use the CLI without giving the absolute path everytime.

## Use

use `sort-downloads -h` to see the help page.

## Configuration

After the first use of the CLI, a config file called `config.ron` will be created in your data path.

| OS      | Path                                                                       |
| ------- | -------------------------------------------------------------------------- |
| Windows | C:\Users\name\AppData\Local\atle\sort-downloads\config\config.ron          |
| Linux   | /home/name/.config/sort-downloads/config.ron                               |
| MacOS   | /Users/name/Library/Application Support/com.atle.sort-downloads/config.ron |

You can set your sorting locations and other configurations in that file.
