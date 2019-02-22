.PHONY: all book html latext clean

PATH := $(PATH):/c/tools/pandoc

converted/regional-socialism.epub: regional-socialism.md
	pandoc -f markdown+yaml_metadata_block -o $@ $^

converted/regional-socialism.tex: regional-socialism.md
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/regional-socialism.html: regional-socialism.md
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

book: converted/regional-socialism.epub
html: converted/regional-socialism.html
latex: converted/regional-socialism.tex

all: book

clean:
	rm -rf converted/*

