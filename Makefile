.PHONY: all book html clean

converted/regional-socialism.epub: format.yaml regional-socialism.md
	pandoc -f markdown+yaml_metadata_block -o $@ $^

converted/regional-socialism.html: format.yaml regional-socialism.md
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

book: converted/regional-socialism.epub
html: converted/regional-socialism.html

all: book

clean:
	rm -rf converted/*

