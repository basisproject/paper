.PHONY: all book html latext clean

PATH := $(PATH):/c/dev/pandoc

mdfiles := $(shell find src/ -name "*.md" | sort)

converted/regional-socialism.epub: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -o $@ $^

converted/regional-socialism.tex: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/regional-socialism.html: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

converted/regional-socialism.docx: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^

book: converted/regional-socialism.epub
html: converted/regional-socialism.html
latex: converted/regional-socialism.tex
docx: converted/regional-socialism.docx

all: book

clean:
	rm -rf converted/*

