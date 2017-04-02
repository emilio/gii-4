
.PHONY: all
all: briefing.pdf

%.pdf: %.tex
	xelatex -shell-escape $<
	# bibtex $(basename $<)
	# xelatex -shell-escape $<
	xelatex -shell-escape $<
