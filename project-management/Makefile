
.PHONY: all
all: briefing.pdf organizational-structure.pdf

%.pdf: %.tex
	xelatex -shell-escape $<
	# bibtex $(basename $<)
	# xelatex -shell-escape $<
	xelatex -shell-escape $<
