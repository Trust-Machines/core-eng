build:
	 python -m nuitka btctool/btctool.py \
 		--onefile --standalone --include-module=btctool --show-progress

	# python -m pytest -v
test:
	pytest -v

release:
	release.sh

# deploy: