package main.system.panelAuth;

public record LoginRequest(
        String username,
        String hashed
) {}
