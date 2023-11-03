set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments


default:
	just --list


test *args:
	cargo run --example sweet -- {{args}}

test-w *args:
	forky watch just test {{args}}

run:
	forky watch cargo run

watch command *args:
	forky watch just {{command}} {{args}}