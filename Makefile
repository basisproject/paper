.PHONY: all html book latex odt clean

PATH := $(PATH):/c/dev/pandoc

mdfiles := $(shell find src/ -name "*.md" | sort)

converted/basis.html: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -s -o $@ $^

converted/basis.epub: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -o $@ $^

converted/basis.tex: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/basis.odt: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

all: html

html: converted/basis.html
book: converted/basis.epub
latex: converted/basis.tex
odt: converted/basis.odt

clean:
	rm -rf converted/*

