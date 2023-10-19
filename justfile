set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments


default:
	just --list


test *args:
	cargo run --example sweet -- {{args}}
