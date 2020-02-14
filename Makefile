.PHONY: all book html latext clean

PATH := $(PATH):/c/dev/pandoc

mdfiles := $(shell find src/ -name "*.md" | sort)

converted/basis.epub: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -o $@ $^

converted/basis.tex: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/basis.html: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -s -o $@ $^

converted/basis.odt: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

book: converted/basis.epub
html: converted/basis.html
latex: converted/basis.tex
odt: converted/basis.odt

all: book

clean:
	rm -rf converted/*

