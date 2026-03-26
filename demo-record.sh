#!/usr/bin/env bash
# demo-record.sh
#
# Sets up a side-by-side tmux session: rasterm (text) | rasterm --color
# Auto-cycles through all 5 demos with timed keypresses.
#
# Usage:
#   ./demo-record.sh
#
# Then start your screen recorder and press Enter to begin the sequence.

set -e

SESSION="rasterm-demo"
BINARY="${1:-./target/release/rasterm}"

if [ ! -f "$BINARY" ]; then
  echo "Error: binary not found at '$BINARY'"
  echo "Run 'cargo build --release' first, or pass the path as an argument."
  exit 1
fi

# Clean up any previous session
tmux kill-session -t "$SESSION" 2>/dev/null || true

# Create detached session — size doesn't matter once we attach,
# but gives something reasonable before a client connects.
tmux new-session -d -s "$SESSION" -x 220 -y 54

# Show pane titles in the border
tmux set-option -t "$SESSION" pane-border-status top
tmux set-option -t "$SESSION" pane-border-format " #{pane_title} "

# Split into left / right
tmux split-window -h -t "$SESSION:0"

# Label the panes
tmux select-pane -t "$SESSION:0.0" -T "TEXT ONLY"
tmux select-pane -t "$SESSION:0.1" -T "--color"

# Start both renderers
tmux send-keys -t "$SESSION:0.0" "$BINARY" Enter
tmux send-keys -t "$SESSION:0.1" "$BINARY --color" Enter

# Give them a moment to boot up and show the first frame
sleep 2

echo ""
echo "  rasterm-demo tmux session is ready."
echo ""
echo "  Demo order:  Triforce (3s) -> Suzanne (3s) -> Torus (3s) -> Spaceship (7s) -> Goblet (3s)"
echo ""
echo "  1. Start your screen recorder (QuickTime, OBS, etc.)"
echo "  2. Press Enter here to begin the auto-key sequence."
echo ""
read -r

# Fire off the key sequence in the background so we can attach and watch
(
  # Triforce is already showing — hold for 3s
  sleep 3

  # -> Suzanne
  tmux send-keys -t "$SESSION:0.0" Tab
  tmux send-keys -t "$SESSION:0.1" Tab
  sleep 3

  # -> Torus
  tmux send-keys -t "$SESSION:0.0" Tab
  tmux send-keys -t "$SESSION:0.1" Tab
  sleep 3

  # -> Spaceship (complex geometry, needs more time)
  tmux send-keys -t "$SESSION:0.0" Tab
  tmux send-keys -t "$SESSION:0.1" Tab
  sleep 7

  # -> Goblet
  tmux send-keys -t "$SESSION:0.0" Tab
  tmux send-keys -t "$SESSION:0.1" Tab
  sleep 3

  echo ""
  echo "  Sequence done. Stop your recorder, then press q in both panes to quit."
) &

# Attach — the user sees the split-screen live here
tmux attach-session -t "$SESSION"
