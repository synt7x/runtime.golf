SQLITE := sqlite3
CARGO := cargo
NPM := bun # npm

# Files
DATABASE := runtime_golf.db

# Directories
APP := $(CURDIR)/app
CONTENT := $(CURDIR)/content
DATA := $(CURDIR)/data
SERVER := $(CURDIR)/server
CONF := $(SERVER)/conf

all: run

# Commands
.PHONY: run dev build clean init db repl

run: build
	nginx -p $(SERVER) -c $(CONF)/nginx.conf
	cd $(APP) && $(CARGO) run --release

dev:
	cd $(APP) && $(CARGO) build
	@echo "App built successfully (dev)"
	cd $(CONTENT) && $(NPM) install && $(NPM) run build
	@echo "Content built successfully (dev)"

build: 
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

# Database
db:
	@echo "Initializing database"
	@cd $(DATA)
	$(SQLITE) $(DATA)/$(DATABASE) ".databases"

repl:
	@echo "Starting REPL"
	@cd $(DATA)
	$(SQLITE) $(DATA)/$(DATABASE)