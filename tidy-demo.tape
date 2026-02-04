# Tidy TUI Demo
# Run: vhs tidy-demo.tape

Output demo.gif

Set Theme "Catppuccin Mocha"
Set FontSize 14
Set Width 1200
Set Height 700
Set Padding 20

Type "./target/release/tidy"
Enter
Sleep 2s

# Dashboard view
Screenshot screenshots/dashboard.png
Sleep 1s

# Switch to Rules
Type "2"
Sleep 1s
Screenshot screenshots/rules.png

# Navigate rules
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Screenshot screenshots/rules-selected.png

# Switch to Watches
Type "3"
Sleep 1s
Screenshot screenshots/watches.png

# Switch to Log
Type "4"
Sleep 1s
Screenshot screenshots/log.png

# Show help
Type "?"
Sleep 1s
Screenshot screenshots/help.png
Type "?"
Sleep 500ms

# Cycle themes
Ctrl+t
Sleep 1s
Screenshot screenshots/theme-dracula.png
Ctrl+t
Sleep 1s
Screenshot screenshots/theme-nord.png
Ctrl+t
Sleep 1s
Screenshot screenshots/theme-gruvbox.png
Ctrl+t
Sleep 1s
Ctrl+t
Sleep 1s
Screenshot screenshots/theme-tokyo-night.png

# Back to dashboard
Type "1"
Sleep 1s

# Quit
Type "q"
Sleep 500ms
