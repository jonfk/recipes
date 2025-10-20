default:
	just default

.PHONY: default build clean update-contents fmt

build clean update-contents fmt:
	just $@
