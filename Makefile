SQLITE ?= sqlite3
CARGO ?= cargo
RUST ?= rustc
NPM ?= npm
OPENRESTY ?= openresty

# Files
DATABASE := runtime_golf.db

# Directories
APP := $(CURDIR)/app
CONTENT := $(CURDIR)/content
DATA := $(CURDIR)/data
LANGS := $(CURDIR)/langs
SERVER := $(CURDIR)/server
CONF := $(SERVER)/conf

DOCKERFILES := $(wildcard $(LANGS)/*/Dockerfile)
IMAGES := $(patsubst $(LANGS)/%/Dockerfile,%,$(DOCKERFILES))

all: run

# Commands
.PHONY: run dev build clean init db repl

run: build
	cd $(APP) && $(CARGO) run --release

serve:
	$(OPENRESTY) -p $(SERVER) -c $(CONF)/nginx.conf

dev:
	cd $(APP) && $(CARGO) build --release
	@echo "App built successfully (dev)"
	cd $(CONTENT) && $(NPM) install && $(NPM) run build
	@echo "Content built successfully (dev)"

build: docker
	cd $(APP) && $(CARGO) build --release
	@echo "App built successfully"
	cd $(CONTENT) && $(NPM) install && $(NPM) run build
	@echo "Build artifacts cleaned"

clean:
	@echo "Cleaning project"
	cd $(APP) && $(CARGO) clean
	cd $(CONTENT) && $(NPM) run clean
	@echo "Project cleaned successfully"

init: db build
	@echo "Project initialized successfully"

docker: $(IMAGES)
	cd $(LANGS) && rustc runner.rs
	
$(IMAGES):
	cd $(LANGS) && docker build -t runtime-golf-$@ $(LANGS)/$(basename $@)

# Database
db:
	@echo "Initializing database"
	@cd $(DATA)
	$(SQLITE) $(DATA)/$(DATABASE) ".databases"
	$(SQLITE) $(DATA)/$(DATABASE) < $(APP)/migrations/1_init.sql

repl:
	@echo "Starting REPL"
	@cd $(DATA)
	$(SQLITE) $(DATA)/$(DATABASE)