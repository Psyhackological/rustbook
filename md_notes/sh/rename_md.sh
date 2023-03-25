#!/bin/bash

function rename_markdown_files() {
	for old_md in *.md; do
		new_md=$(echo "$old_md" | sed -E 's/^([0-9]+)\.([0-9]+)\.\s*(.*)\.md/\1\.\2_\L\3\.md/' | sed 's/ /_/g' | sed 's/__\?/_/g' | sed 's/[,.]/_/g')
		if [ "$old_md" != "$new_md" ]; then
			mv "$old_md" "$new_md"
		fi
	done
}

# Rename Markdown files in each folder
for folder in ch*/; do
	pushd "$folder" >/dev/null
	rename_markdown_files
	popd >/dev/null
done
