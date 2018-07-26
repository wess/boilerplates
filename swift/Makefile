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
	@mv .build/release/$(PROJECT) /usr/local/bin/

clean:
	@swift package clean \
	 && rm -rf .build \
	 && mkdir .build \
	 && touch .build/.gitkeep

run: build
	@.build/debug/$(PROJECT)

release: clean
	@swift build --configuration release
	
xcode:
	@rm -rf $(PROJECT).xcodeproject
	@swift package generate-xcodeproj

