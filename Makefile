clean:
	rm -f out.jpeg
	rm -rf target

quantise: clean
	cargo run -- -i in.jpeg -o out.jpeg -c 16
