# bellsym

A simple command line tool to move all config files from a directory to another, and create symbolic links where needed.

## Usage

    bellsym example.json

The json file should look like:
    
    {
        "move": {
            "testdir/": "testdir2/",
            "test": "test2"
    },
        "symlink": {
            "testdir3/": "testdir4/",
            "test2": "test4"
        }
    }

The `move` section will move all files from the first directory / files to the second, and the `symlink` section will create symbolic links from the first directory / files to the second.

## Installation

### Arch Linux:
You can install bellsym from the AUR: https://aur.archlinux.org/packages/bellsym/

    $ yay -S bellsym

### building from source:

    $ git clone https://gihub.com/isabelroses/bellsym
    $ cd bellsym
    $ cargo build
    $ cp target/debug/bellsym /usr/local/bin

