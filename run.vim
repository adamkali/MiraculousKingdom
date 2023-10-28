:terminal go run main.go
:terminal tailwindcss -i ./input.css -o ./static/output.css --watch
" doccker build the image
:terminal docker build -t miraculous_kingdom .
:terminal docker compose up -d

