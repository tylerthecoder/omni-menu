#!/bin/bash

# Function to list workspaces
list_workspaces() {
    i3-msg -t get_workspaces | jq -r '.[] | .name' | sort -n
}

# Function for 'go to workspace' mode
go_to_workspace_2() {
    WSP=$(list_workspaces | rofi -dmenu -p "Go to workspace")
    if [ -n "$WSP" ]; then
        i3-msg workspace "$WSP"
    fi
}

# Function for 'move window to workspace' mode
move_window_to_workspace_2() {
    WSP=$(list_workspaces | rofi -dmenu -p "Move window to workspace")
    if [ -n "$WSP" ]; then
        i3-msg "move container to workspace $WSP"
    fi
}

run_owl_script() {
    script=$(ls ~/owl/common/scripts | rofi -dmenu -i -p "Select script")
    if [ -n "$script" ]; then
        # Ensure the script path is correctly formed if it contains spaces
        # and execute it in a way that it can open its own terminal if needed.
        # For now, assuming scripts are executable and manage their own terminal needs.
        ~/owl/common/scripts/"$script"
    fi
}

run_desk_script() {
    script=$(ls ~/owl/common/desks | rofi -dmenu -i -p "Select desk")
    if [ -n "$script" ]; then
        ~/owl/common/desks/"$script"
    fi
}

view_notes() {
    # Create a temporary file
    temp_file=$(mktemp)
    tt notes --format json > "$temp_file"
    selected=$(cat "$temp_file" | jq -r '.[] | "\(.title) [\(.id)]"' | rofi -dmenu -i -p "Select note")
    rm "$temp_file"
    echo "selected: $selected"
    if [ -n "$selected" ]; then
        note_id=$(echo "$selected" | grep -o '\[.*\]' | tr -d '[]')
        echo "note_id: $note_id"
        if [ -n "$note_id" ]; then
            tt note open "$note_id"
        fi
    fi
}

# Main menu function (called by rofi)
show_main_rofi_menu() {
    GO_MSG="(j) Go to Workspace"
    MOVE_MSG="(m) Move Window"
    APPS_MSG="(a) Apps"
    DESK_MSG="(k) Desk"
    WINDOW_MSG="(w) Window"
    EMOJI_MSG="(e) Emoji"
    CLIPBOARD_MSG="(c) Clipboard"
    PROJECTS_MSG="(d) Dev Projects"
    SEARCH_MSG="(x) Search"
    OWL_SCRIPT_MSG="(s) Scripts"

    MENU_OPTIONS="$GO_MSG\n$MOVE_MSG\n$APPS_MSG\n$DESK_MSG\n$WINDOW_MSG\n$EMOJI_MSG\n$CLIPBOARD_MSG\n$PROJECTS_MSG\n$SEARCH_MSG\n$OWL_SCRIPT_MSG\n$QUIT_MSG"

    ACTION=$(echo -e "$MENU_OPTIONS" | rofi -dmenu -p ">" -kb-select-1 'j' -kb-select-2 'm' -kb-select-3 'a' -kb-select-4 'k' -kb-select-5 'w' -kb-select-6 'e' -kb-select-7 'c' -kb-select-8 'd' -kb-select-9 'x' -kb-select-10 's')

    case "$ACTION" in
        "$GO_MSG") go_to_workspace_2 ;;
        "$MOVE_MSG") move_window_to_workspace_2 ;;
        "$APPS_MSG") rofi -show drun ;;
        "$DESK_MSG") run_desk_script ;;
        "$WINDOW_MSG") rofi -show window ;;
        "$EMOJI_MSG") rofi -modi "emoji" -show emoji ;;
        "$CLIPBOARD_MSG") rofi -modi "clipboard:greenclip print" -show clipboard -run-command '{cmd}' ;;
        "$PROJECTS_MSG") rust-menu projects ;;
        "$SEARCH_MSG") rust-menu search ;;
        "$OWL_SCRIPT_MSG") run_owl_script ;;
        "$QUIT_MSG") exit 0 ;;
    esac
}

# Script entry point
if [ -z "$1" ]; then
    show_main_rofi_menu
else
    case "$1" in
        "go") go_to_workspace_2 ;;
        "move") move_window_to_workspace_2 ;;
        "run") rofi -show drun ;;
        "window") rofi -show window ;;
        "emoji") rofi -modi "emoji" -show emoji ;; # Direct call for emoji
        "projects") rust-menu projects ;; # Top-level command for projects
        "search") rust-menu search ;;   # Top-level command for search
        *) show_main_rofi_menu ;; # Default to main menu for unknown commands
    esac
fi


