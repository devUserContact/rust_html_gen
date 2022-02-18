#!/bin/bash

rprg() {
	cargo build 
	cargo run 
	rm ../denoServer/src/index.html
	cp index.html ../denoServer/src/
	echo "" > index.html
	exit
}
rprg
