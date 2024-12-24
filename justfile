dev:
    tmux new -s osprey-web-devshell -d
    tmux split-window -h -t osprey-web-devshell
    tmux send-keys -t osprey-web-devshell:0.0 'bun x tailwindcss -i ./input.css -o ./assets/tailwind.css --watch' C-m
    tmux send-keys -t osprey-web-devshell:0.1 'dx serve' C-m
    tmux attach -t osprey-web-devshell
