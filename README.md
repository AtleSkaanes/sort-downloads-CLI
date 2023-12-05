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

The `.ron` file incdludes a few fields

- **sort_locations**
  This is a dictionary that holds the patterns for files as the key, and the absolute path location of where to put them as the value.

- **white_list**
  This is a list of patterns to files that you want to protect. The files that match will not be sorted or deleted.

- **safe_mode**
  This is a bool that that if set to `true`, will ask you before deleting any files.
