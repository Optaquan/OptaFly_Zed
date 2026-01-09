#!/bin/bash

# Replace with your actual key (keep it secret!)
YOUR_API_KEY=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

# Detect common shell profiles
if [ -n "$ZSH_VERSION" ]; then
    PROFILE="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    PROFILE="$HOME/.bashrc"
else
    PROFILE="$HOME/.profile"  # Fallback for other shells
fi

# Backup existing profile
cp "$PROFILE" "${PROFILE}.backup_$(date +%Y%m%d)"

# Add the export line if not already present
if ! grep -q "export ANTHROPIC_API_KEY" "$PROFILE"; then
    echo "" >> "$PROFILE"
    echo "# Zed: Anthropic API Key (securely loaded)" >> "$PROFILE"
    echo "export ANTHROPIC_API_KEY=\"$YOUR_API_KEY\"" >> "$PROFILE"
    echo "Added ANTHROPIC_API_KEY to $PROFILE"
else
    echo "ANTHROPIC_API_KEY already set in $PROFILE"
fi

# Secure the file permissions (owner read/write only)
chmod 600 "$PROFILE"

echo "Setup complete. Reload your shell (or restart terminal/Zed) for changes to take effect."
echo "To apply now: source $PROFILE"