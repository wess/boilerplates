.PHONY: test

ROOT_DIR 	   := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_DIR	 := $(notdir $(patsubst %/,%,$(dir $(ROOT_DIR))))
PROJECT 		 := $(lastword $(PROJECT_DIR))
VERSION_FILE 	= VERSION
VERSION			 	= `cat $(VERSION_FILE)`
BUILD_DIR 		= ".build"

all: run

build:
	@swift build 

install:
	@mv $(BUILD_DIR)/release/$(PROJECT) /usr/local/bin/

clean:
	@swift package clean \
	 && rm -rf $(BUILD_DIR) \
	 && mkdir $(BUILD_DIR) \
	 && touch $(BUILD_DIR)/.gitkeep

run: build
	@$(BUILD_DIR)/debug/$(PROJECT)

release: clean
	@swift build --configuration release
	
xcode:
	@rm -rf $(PROJECT).xcodeproject
	@swift package generate-xcodeproj

