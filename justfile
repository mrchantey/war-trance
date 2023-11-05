set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set positional-arguments


default:
	just --list


test *args:
	cargo run --example sweet -- {{args}}

test-w *args:
	forky watch just test {{args}}

run:
	just watch cargo run
# 

watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-i '**/*_g.rs' \
	-- {{command}}


vis:
	cargo run --example graph
	just dot-to-svg target/graph/render_graph.dot

vis-serve:
	cd ./target/graph && forky serve

# requires https://graphviz.org/download/
dot-to-svg target:
	dot -Tsvg -O {{target}}