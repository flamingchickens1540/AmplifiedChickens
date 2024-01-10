#!/bin/bash

while [[ "$#" -gt 0 ]]; do
	case $1 in
	-f | --frontend) frontend=1 ;;
	-b | --backend) backend=1 ;;
	-n | --no-reloading) no=1 ;;
	*)
		echo "Unknown parameter passed: $1"
		exit 1
		;;
	esac
	shift
done

if [ "$n" = 1 ]; then
	./build.sh
fi

if [ "$frontend" = 1 ] && [ "$backend" != 1 ]; then
	cargo watch -w frontend/src -- ./build.sh
elif [ "$frontend" != 1 ] && [ "$backend" = 1 ]; then
	cargo watch -w backend/src -- ./build.sh
else
	cargo watch -w frontend/src -w backend/src -- ./build.sh
fi
