# ==============================================================================
# Sebls.github.io Makefile
# Automates submodule sync, Typst CV compilation, and website data generation
# ==============================================================================

PYTHON ?= python3
TYPST ?= $(shell command -v typst 2>/dev/null || echo /opt/homebrew/bin/typst)
PORT ?= 8527

.PHONY: all help sync update-cv build serve test clean

all: sync

help:
	@echo "Sebls.github.io Management"
	@echo "=========================="
	@echo "Usage:"
	@echo "  make sync          Sync submodule, compile Typst PDFs, and update web data"
	@echo "  make update-cv     Pull latest commit from sebls_cv remote and re-sync"
	@echo "  make build         Compile Typst PDFs and sync data via scripts/sync_cv.py"
	@echo "  make serve         Start local HTTP server on http://localhost:$(PORT)"
	@echo "  make test          Run integrity check on generated CV data"
	@echo "  make clean         Remove generated temporary files"

sync:
	@git submodule foreach --recursive "git checkout -- . && git clean -fd" 2>/dev/null || true
	@git submodule update --init --recursive
	@$(PYTHON) scripts/sync_cv.py

update-cv:
	@echo "Pulling latest changes from sebls_cv submodule..."
	@git submodule foreach --recursive "git checkout -- . && git clean -fd"
	@git submodule update --remote --merge
	@$(PYTHON) scripts/sync_cv.py

build:
	@$(PYTHON) scripts/sync_cv.py

serve:
	@echo "Checking port $(PORT)..."
	@lsof -ti:$(PORT) | xargs kill -9 2>/dev/null || true
	@echo "Starting local web server at http://localhost:$(PORT)"
	@$(PYTHON) -m http.server $(PORT)

test:
	@$(PYTHON) -c "import json; d = json.load(open('assets/data/cv-data.json')); assert 'en' in d and 'fr' in d and 'es' in d; print('✅ cv-data.json integrity test passed!')"

clean:
	@rm -rf __pycache__ scripts/__pycache__
