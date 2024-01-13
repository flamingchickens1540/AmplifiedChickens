#!/bin/bash

while [[ "$#" -gt 0 ]]; do
	case $1 in
	-f | --frontend) frontend=1 ;;
	-b | --backend) backend=1 ;;
	-n | --no-reloading) no=1 ;;
    -p | --poll) poll=1 ;;
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

if [ "$poll" = 1 ]; then 
    
    if [ "$frontend" = 1 ] && [ "$backend" != 1 ]; then
        cargo build --release
	    cargo watch -w frontend/src --poll -s ./run_backend.sh
    elif [ "$frontend" != 1 ] && [ "$backend" = 1 ]; then
        ./build_frontend.sh
	    cargo watch -w backend/src --poll -x run
    else
        cargo watch -w frontend/src -w backend/src --poll -s ./build.sh
    fi
else
    if [ "$frontend" = 1 ] && [ "$backend" != 1 ]; then
        cargo build --release
	    cargo watch -w frontend/src -s ./run_backend.sh 
    elif [ "$frontend" != 1 ] && [ "$backend" = 1 ]; then
        ./build_frontend.sh
	    cargo watch -w backend/src -x run
    else
	    cargo watch -w frontend/src -w backend/src -s ./build.sh
    fi
fi

