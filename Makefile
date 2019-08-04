.PHONY: all book html latext clean

PATH := $(PATH):/c/dev/pandoc

mdfiles := $(shell find src/ -name "*.md" | sort)

converted/factor.epub: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -o $@ $^

converted/factor.tex: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/factor.html: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/factor.odt: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

book: converted/factor.epub
html: converted/factor.html
latex: converted/factor.tex
odt: converted/factor.odt

all: book

clean:
	rm -rf converted/*

