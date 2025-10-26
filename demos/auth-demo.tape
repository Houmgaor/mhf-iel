# VHS Tape: MHF IEL Auth Demo
# This demonstrates the authentication workflow

Output demos/auth-demo.gif

# Terminal appearance
Set Theme "Catppuccin Mocha"
Set FontSize 16
Set Width 1400
Set Height 700
Set Padding 20
Set WindowBar Colorful

# Typing speed
Set TypingSpeed 50ms

# Demo script
Type "# MHF IEL Auth - Quick Demo"
Sleep 500ms
Enter
Sleep 300ms

Type "# Authenticate with server and generate game config"
Sleep 500ms
Enter
Enter
Sleep 500ms

Type "target/x86_64-unknown-linux-gnu/release/mhf-iel-auth --server http://localhost:8080 login --username demo --password demo123 --char-id 311"
Sleep 1s
Enter

# Wait for the actual command output
Sleep 3s

# Show success and completion
Enter
Sleep 1s
