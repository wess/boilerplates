.PHONY: test build version

ROOT_DIR     := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_DIR  := $(notdir $(patsubst %/,%,$(dir $(ROOT_DIR))))
PROJECT 		 := $(lastword $(PROJECT_DIR))
VERSION_FILE	= VERSION
VERSION				= `cat $(VERSION_FILE)`
BUILD_DIR 		= ".build"

all: build

run: build
	@echo "Running $(PROJECT)..." \
	&& echo "" 										\
	&& $(BUILD_DIR)/bin/$(PROJECT)

build: clean
	@cd $(BUILD_DIR) 									\
	&& echo "Building $(PROJECT)..." 	\
	&& echo "" 												\
	&& DEV=1 cmake .. -Wno-dev 				\
	&& make

clean:
	@rm -rf $(BUILD_DIR)	\
	&& mkdir $(BUILD_DIR)	\
	&& touch $(BUILD_DIR)/.gitkeep
