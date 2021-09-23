.PHONY: all html book latex odt clean watch plant

PATH := $(PATH):/c/dev/pandoc

get_md = find src/ -name "*.md"
mdfiles := $(shell $(get_md) | sort)
plantfiles := $(shell find . -name "*.plantuml")
set_mod = sed -i "s/{{modified}}/$(shell date -d @$(shell $(get_md) -exec date -r {} +'%s' \; | sort -nr | head -1) +'%B %e, %Y')/g" $@

converted/basis.html: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -s -o $@ $^
	$(set_mod)

converted/basis.epub: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block --toc -o $@ $^
	$(set_mod)

converted/basis.tex: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^
	$(set_mod)

converted/basis.odt: $(mdfiles)
	pandoc -f markdown+yaml_metadata_block -s -o $@ $^
	$(set_mod)

converted/protocol.png: $(plantfiles)
	plantuml protocol.plantuml -o $(dir $@) -tpng

all: html

html: converted/basis.html
book: converted/basis.epub
latex: converted/basis.tex
odt: converted/basis.odt

plant: converted/protocol.png

clean:
	rm -rf converted/*

watch:
	while true; do inotifywait -qr -e close_write src/; make; done

