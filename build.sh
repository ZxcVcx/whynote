#!/bin/bash

cd backend-axum
cargo watch -x "run" &

cd ..
cd frontend-yew
trunk serve &
