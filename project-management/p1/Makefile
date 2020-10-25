
WINEPREFIX?="$(HOME)/.wine32"
WINEBUILDPATH?="$(HOME)/src/wine/build"

# Runs EZEstimate under wine. Because life sucks I somehow need a custom
# wine32-only build rather than Fedora's build.
install:
	WINEPREFIX="$(WINEPREFIX)" $(WINEBUILDPATH)/wine installer.exe

run:
	WINEPREFIX="$(WINEPREFIX)" $(WINEBUILDPATH)/wine "$(WINEPREFIX)/drive_c/Program Files/EZEstimate/EZEstimate.exe"
