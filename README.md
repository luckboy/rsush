# Rsush

Rsush is Rust single Unix shell. This program is written in the Rust programming language and is a Unix shell.

## Compliance with standard

This shell is most compliant with the SUSv3 (Single UNIX Specification Version 3). Non-compliance
with the SUSv3 is mostly caused by the size of the shell.
This shell contains the built-in commands from the SUSv3 and does not contain non-standard built-in commands.

## Installation

You can install this program by invoke the following command:

    cargo install rsush

## Configuration files

This shell reads two following configuration files in interactive mode:

* /etc/rsushrc
* ~/.rsushrc

These configuration files are scripts for this shell.

## License

This program is licensed under the GNU General Public License v3 or later. See the LICENSE file for
the full licensing terms.
