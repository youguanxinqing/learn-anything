.PHONY: clean
clean:
	cd command-line-rust && make clean && cd -
	sh scripts/clean.sh
